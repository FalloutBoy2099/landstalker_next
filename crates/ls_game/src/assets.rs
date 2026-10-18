//! Where the game reads its data files from. A sprite can be loose files (data/sprites: the YAML and the PNG pictures, for
//! development) or one LSA archive per sprite (see docs/lsa_format.md and `lsctl lsa-pack-sprites`):
//!   data/sprites/sprite_NNN.lsa            sprite.yaml + the pictures of the animated sprite sprite_NNN (Nigel...)
//!   data/sprites/entities/entity_NNN.lsa   entity.yaml (the definition of the entity type NNN) + its pictures
//! Everything else is read from the data folder. landstalker.ini, [data] sprites = auto | files | lsa chooses.

use bevy::prelude::*;
use ls_data::lsa::Lsa;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

struct Inner {
    root: PathBuf,
    /// "files", "lsa" or "auto"
    mode: String,
    /// the archives already opened, by path relative to the data folder (None: there is no such archive)
    archives: Mutex<HashMap<String, Option<Arc<Lsa>>>>,
}

#[derive(Resource, Clone)]
pub struct DataFiles(Arc<Inner>);

/// Which archive holds a file of the sprites, and under which name: `sprites/sprite_000/walk/back_0.png` is `walk/back_0.png` in
/// `sprites/sprite_000.lsa`; `sprites/entities/entity_018/idle/back.png` is `idle/back.png` in `sprites/entities/entity_018.lsa`.
fn archive_of(rel: &str) -> Option<(String, String)> {
    let rest = rel.strip_prefix("sprites/")?;
    let (folder, file) = rest.split_once('/')?;
    if folder.starts_with("sprite_") {
        return Some((format!("sprites/{}.lsa", folder), file.to_string()));
    }
    if folder == "entities" {
        let (entity, file) = file.split_once('/')?;
        if entity.starts_with("entity_") {
            return Some((format!("sprites/entities/{}.lsa", entity), file.to_string()));
        }
    }
    None
}

impl DataFiles {
    /// `mode`: "files" (loose files only), "lsa" (the archives only), anything else = auto (a loose file when there is one: you can
    /// edit it, then the sprite's archive).
    pub fn open(root: PathBuf, mode: &str) -> DataFiles {
        let mode = match mode {
            "files" | "lsa" => mode.to_string(),
            _ => "auto".to_string(),
        };
        let n = std::fs::read_dir(root.join("sprites")).map(|d| d.flatten().filter(|e| e.path().extension().map_or(false, |x| x == "lsa")).count()).unwrap_or(0)
            + std::fs::read_dir(root.join("sprites/entities")).map(|d| d.flatten().filter(|e| e.path().extension().map_or(false, |x| x == "lsa")).count()).unwrap_or(0);
        println!("data: sprites: mode {} ({} sprite archives found)", mode, n);
        DataFiles(Arc::new(Inner { root, mode, archives: Mutex::new(HashMap::new()) }))
    }

    fn archive(&self, rel: &str) -> Option<Arc<Lsa>> {
        let mut cache = self.0.archives.lock().ok()?;
        if let Some(a) = cache.get(rel) {
            return a.clone();
        }
        let path = self.0.root.join(rel);
        let a = if path.exists() {
            match Lsa::open(&path) {
                Ok(a) => Some(Arc::new(a)),
                Err(e) => {
                    println!("data: {:#}", e);
                    None
                }
            }
        } else {
            None
        };
        cache.insert(rel.to_string(), a.clone());
        a
    }

    /// A file of the data folder ("sprites/entities/entity_018_default.png", "rooms/room_005/room.yaml"...).
    pub fn read(&self, rel: &str) -> Option<Vec<u8>> {
        let rel = rel.replace(std::path::MAIN_SEPARATOR, "/");
        let from_archive = || {
            let (archive, name) = archive_of(&rel)?;
            self.archive(&archive)?.read(&name).map_err(|e| println!("data: {:#}", e)).ok()
        };
        let from_file = || std::fs::read(self.0.root.join(&rel)).ok();
        match (self.0.mode.as_str(), archive_of(&rel).is_some()) {
            ("lsa", true) => from_archive(),
            ("files", _) | (_, false) => from_file(),
            _ => from_file().or_else(from_archive),
        }
    }

    pub fn read_string(&self, rel: &str) -> Option<String> {
        String::from_utf8(self.read(rel)?).ok()
    }

    pub fn image(&self, rel: &str) -> Option<image::RgbaImage> {
        image::load_from_memory(&self.read(rel)?).ok().map(|i| i.to_rgba8())
    }

    /// The definitions of the entity types: (number, text of its `entity.yaml`), from the loose folders `sprites/entities/entity_NNN/`
    /// and the archives `entity_NNN.lsa` (see `read` for which one wins).
    pub fn entity_definitions(&self) -> Vec<(u32, String)> {
        let mut ids = std::collections::BTreeSet::new();
        if let Ok(dir) = std::fs::read_dir(self.0.root.join("sprites/entities")) {
            for e in dir.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                let id = name.strip_prefix("entity_").map(|n| n.strip_suffix(".lsa").unwrap_or(n)).and_then(|n| n.parse::<u32>().ok());
                if let Some(id) = id {
                    ids.insert(id);
                }
            }
        }
        ids.into_iter().filter_map(|id| Some((id, self.read_string(&format!("sprites/entities/entity_{:03}/entity.yaml", id))?))).collect()
    }
}
