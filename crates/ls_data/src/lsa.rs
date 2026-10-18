//! LSA, Landstalker Asset archive: one binary file holding many named files (the YAML of the sprites and all their PNG
//! pictures, for instance). See docs/lsa_format.md.
//!
//! ```text
//! header (32 bytes, little endian)
//!    0  4  magic "LSA1"
//!    4  4  flags (0)
//!    8  4  number of entries
//!   12  4  reserved (0)
//!   16  8  offset of the index
//!   24  8  size of the index
//! data     the files one after the other (stored as they are, or compressed with zstd)
//! index    per entry: u16 name length, the name (UTF-8, '/' between folders), u8 method (0 stored, 1 zstd),
//!          u8 kind (0 other, 1 yaml, 2 png), u16 reserved, u64 offset, u32 stored size, u32 size, u32 crc32 of the file
//! ```

use anyhow::{anyhow, bail, Context, Result};
use std::collections::BTreeMap;
use std::path::Path;

const MAGIC: &[u8; 4] = b"LSA1";
const HEADER: usize = 32;

#[derive(Clone, Copy, Debug)]
struct Entry {
    method: u8,
    kind: u8,
    offset: u64,
    stored: u32,
    size: u32,
    crc: u32,
}

/// An opened archive (the whole file is in memory).
pub struct Lsa {
    data: Vec<u8>,
    index: BTreeMap<String, Entry>,
}

fn u16_at(d: &[u8], at: usize) -> Result<u16> {
    Ok(u16::from_le_bytes(d.get(at..at + 2).ok_or_else(|| anyhow!("LSA: file cut short"))?.try_into()?))
}
fn u32_at(d: &[u8], at: usize) -> Result<u32> {
    Ok(u32::from_le_bytes(d.get(at..at + 4).ok_or_else(|| anyhow!("LSA: file cut short"))?.try_into()?))
}
fn u64_at(d: &[u8], at: usize) -> Result<u64> {
    Ok(u64::from_le_bytes(d.get(at..at + 8).ok_or_else(|| anyhow!("LSA: file cut short"))?.try_into()?))
}

fn kind_of(name: &str) -> u8 {
    let lower = name.to_lowercase();
    if lower.ends_with(".yaml") || lower.ends_with(".yml") {
        1
    } else if lower.ends_with(".png") {
        2
    } else {
        0
    }
}

impl Lsa {
    pub fn from_bytes(data: Vec<u8>) -> Result<Lsa> {
        if data.len() < HEADER || &data[0..4] != MAGIC {
            bail!("not an LSA file (bad magic)");
        }
        let count = u32_at(&data, 8)? as usize;
        let index_offset = u64_at(&data, 16)? as usize;
        let index_size = u64_at(&data, 24)? as usize;
        let idx = data.get(index_offset..index_offset + index_size).ok_or_else(|| anyhow!("LSA: the index is outside the file"))?;
        let mut index = BTreeMap::new();
        let mut at = 0usize;
        for _ in 0..count {
            let name_len = u16_at(idx, at)? as usize;
            at += 2;
            let name = std::str::from_utf8(idx.get(at..at + name_len).ok_or_else(|| anyhow!("LSA: index cut short"))?)?.to_string();
            at += name_len;
            let method = *idx.get(at).ok_or_else(|| anyhow!("LSA: index cut short"))?;
            let kind = *idx.get(at + 1).ok_or_else(|| anyhow!("LSA: index cut short"))?;
            let offset = u64_at(idx, at + 4)?;
            let stored = u32_at(idx, at + 12)?;
            let size = u32_at(idx, at + 16)?;
            let crc = u32_at(idx, at + 20)?;
            at += 24;
            index.insert(name, Entry { method, kind, offset, stored, size, crc });
        }
        Ok(Lsa { data, index })
    }

    pub fn open(path: &Path) -> Result<Lsa> {
        let data = std::fs::read(path).with_context(|| format!("{}", path.display()))?;
        Lsa::from_bytes(data).with_context(|| format!("{}", path.display()))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.index.contains_key(name)
    }

    /// The names in the archive, sorted.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.index.keys().map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Size of a file once decompressed.
    pub fn size_of(&self, name: &str) -> Option<usize> {
        self.index.get(name).map(|e| e.size as usize)
    }

    /// The content of a file (decompressed, checked against its crc32).
    pub fn read(&self, name: &str) -> Result<Vec<u8>> {
        let e = self.index.get(name).ok_or_else(|| anyhow!("{}: not in the archive", name))?;
        let stored = self.data.get(e.offset as usize..e.offset as usize + e.stored as usize).ok_or_else(|| anyhow!("{}: outside the file", name))?;
        let bytes = match e.method {
            0 => stored.to_vec(),
            1 => zstd::bulk::decompress(stored, e.size as usize).with_context(|| format!("{}: zstd", name))?,
            m => bail!("{}: unknown method {}", name, m),
        };
        if crc32fast::hash(&bytes) != e.crc {
            bail!("{}: crc32 does not match (the archive is damaged)", name);
        }
        Ok(bytes)
    }

    /// Kind of a file: 0 other, 1 yaml, 2 png.
    pub fn kind_of(&self, name: &str) -> Option<u8> {
        self.index.get(name).map(|e| e.kind)
    }
}

/// Builds an archive from named files.
#[derive(Default)]
pub struct LsaBuilder {
    files: BTreeMap<String, Vec<u8>>,
}

impl LsaBuilder {
    pub fn new() -> LsaBuilder {
        LsaBuilder::default()
    }

    pub fn add(&mut self, name: &str, bytes: Vec<u8>) {
        self.files.insert(name.replace('\\', "/"), bytes);
    }

    /// Adds every file under `dir` (recursively), named relative to it.
    pub fn add_dir(&mut self, dir: &Path) -> Result<()> {
        fn walk(base: &Path, dir: &Path, out: &mut Vec<(String, std::path::PathBuf)>) -> Result<()> {
            for e in std::fs::read_dir(dir)? {
                let p = e?.path();
                if p.is_dir() {
                    walk(base, &p, out)?;
                } else {
                    out.push((p.strip_prefix(base)?.to_string_lossy().replace('\\', "/"), p));
                }
            }
            Ok(())
        }
        let mut list = Vec::new();
        walk(dir, dir, &mut list)?;
        for (name, path) in list {
            self.add(&name, std::fs::read(&path).with_context(|| format!("{}", path.display()))?);
        }
        Ok(())
    }

    /// The archive as bytes: zstd where it makes a file at least 5 % smaller.
    pub fn build(&self) -> Result<Vec<u8>> {
        let mut data = vec![0u8; HEADER];
        let mut index = Vec::new();
        for (name, bytes) in &self.files {
            let packed = zstd::bulk::compress(bytes, 19)?;
            let (method, blob): (u8, &[u8]) = if (packed.len() as f64) < bytes.len() as f64 * 0.95 { (1, &packed) } else { (0, bytes) };
            let offset = data.len() as u64;
            data.extend_from_slice(blob);
            index.extend_from_slice(&(name.len() as u16).to_le_bytes());
            index.extend_from_slice(name.as_bytes());
            index.push(method);
            index.push(kind_of(name));
            index.extend_from_slice(&0u16.to_le_bytes());
            index.extend_from_slice(&offset.to_le_bytes());
            index.extend_from_slice(&(blob.len() as u32).to_le_bytes());
            index.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            index.extend_from_slice(&crc32fast::hash(bytes).to_le_bytes());
        }
        let index_offset = data.len() as u64;
        data.extend_from_slice(&index);
        data[0..4].copy_from_slice(MAGIC);
        data[8..12].copy_from_slice(&(self.files.len() as u32).to_le_bytes());
        data[16..24].copy_from_slice(&index_offset.to_le_bytes());
        data[24..32].copy_from_slice(&(index.len() as u64).to_le_bytes());
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn what_goes_in_comes_out() {
        let mut b = LsaBuilder::new();
        b.add("sprites/entities.yaml", "entities:\n  1: {a: 1}\n".repeat(200).into_bytes());
        b.add("sprites/entities/entity_001_default.png", (0..=255u8).cycle().take(5000).collect());
        b.add("empty.txt", Vec::new());
        let bytes = b.build().unwrap();
        let a = Lsa::from_bytes(bytes).unwrap();
        assert_eq!(a.len(), 3);
        assert_eq!(a.read("sprites/entities.yaml").unwrap(), "entities:\n  1: {a: 1}\n".repeat(200).into_bytes());
        assert_eq!(a.read("sprites/entities/entity_001_default.png").unwrap().len(), 5000);
        assert_eq!(a.read("empty.txt").unwrap(), Vec::<u8>::new());
        assert!(a.read("nothing").is_err());
        assert_eq!(a.kind_of("sprites/entities.yaml"), Some(1));
        assert_eq!(a.kind_of("sprites/entities/entity_001_default.png"), Some(2));
    }

    #[test]
    fn a_damaged_archive_is_refused() {
        let mut b = LsaBuilder::new();
        b.add("x.yaml", b"hello hello hello hello hello hello hello hello".to_vec());
        let mut bytes = b.build().unwrap();
        assert!(Lsa::from_bytes(bytes[..10].to_vec()).is_err());
        bytes[0] = b'X';
        assert!(Lsa::from_bytes(bytes).is_err());
        let mut b = LsaBuilder::new();
        b.add("y.bin", (0..=255u8).collect());
        let mut bytes = b.build().unwrap();
        bytes[HEADER + 3] ^= 0xFF;
        let a = Lsa::from_bytes(bytes).unwrap();
        assert!(a.read("y.bin").is_err());
    }
}
