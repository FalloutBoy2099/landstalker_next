//! lsctl: command line tool of landstalker_next.
//!   lsctl check [data_dir]                         every room loads and is consistent (warps, files, grid sizes)
//!   lsctl verify-legacy [data_dir] [engine_assets] the converted maps equal the C++ engine's files cell for cell
//!   lsctl room <id> [data_dir]                     prints a summary of a room (cells, entities, warps)
//!   lsctl lsa-pack <folder> <out.lsa>              packs a folder into an LSA archive
//!   lsctl lsa-pack-sprites [sprites_dir] [out_dir]  one LSA per sprite: sprite_NNN.lsa and entities/entity_NNN.lsa
//!   lsctl evt-render <dump.evt> <seconds> <out.wav>   plays the register writes of an engine dump through the Rust chips
//!   lsctl evt-to-vgm <dump.evt> <out.vgm> [--once]  writes a VGM file from a dump (finds the loop; --once: a jingle, no loop,
//!                                                  the idle writes of the driver after its end are cut)
//!   lsctl vgm-render <file.vgm> <out.wav>          plays a VGM file (intro + two loops)
//!   lsctl lsa-list <file.lsa>                      lists an archive
//!   lsctl lsa-unpack <file.lsa> <folder>           writes the files of an archive

use anyhow::{bail, Result};
use ls_data::{load_index, room_dir, Room};
use std::path::{Path, PathBuf};

fn read_csv(path: &Path, radix: u32) -> Result<Vec<Vec<u32>>> {
    let text = std::fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        rows.push(line.split(',').map(|v| u32::from_str_radix(v.trim(), radix)).collect::<std::result::Result<Vec<_>, _>>()?);
    }
    Ok(rows)
}

fn check(data: &Path) -> Result<()> {
    let index = load_index(data)?;
    let ids: std::collections::HashSet<i32> = index.rooms.iter().map(|r| r.id).collect();
    let mut problems = 0;
    let mut entities = 0;
    let mut warps = 0;
    for entry in &index.rooms {
        let dir = room_dir(data, entry.id);
        let room = match Room::load(&dir) {
            Ok(r) => r,
            Err(e) => {
                println!("room {}: {:#}", entry.id, e);
                problems += 1;
                continue;
            }
        };
        if room.file.id != entry.id {
            println!("room {}: the file says it is room {}", entry.id, room.file.id);
            problems += 1;
        }
        for w in &room.file.warps {
            warps += 1;
            if !ids.contains(&w.target_room) {
                println!("room {}: a warp leads to the missing room {}", entry.id, w.target_room);
                problems += 1;
            }
        }
        for name in ["blocks.png", "preview.png", "palette.yaml"] {
            if !dir.join(name).exists() {
                println!("room {}: {} is missing", entry.id, name);
                problems += 1;
            }
        }
        for e in &room.file.entities {
            for pic in e.poses.iter().flat_map(|p| p.values()).flat_map(|v| v.pictures()) {
                if !dir.join(&pic.file).exists() {
                    println!("room {}: entity {}: {} is missing", entry.id, e.id, pic.file);
                    problems += 1;
                }
            }
        }
        entities += room.file.entities.len();
    }
    problems += check_sprites(data);
    println!("{} rooms, {} entities, {} warps, {} problems", index.rooms.len(), entities, warps, problems);
    if problems > 0 {
        bail!("{} problems", problems);
    }
    Ok(())
}

/// The poses of every sprite and entity type read, and every picture they name is there (a file of the folder).
fn check_sprites(data: &Path) -> usize {
    use ls_data::sprites::SpriteDef;
    let mut problems = 0;
    let mut folders: Vec<PathBuf> = Vec::new();
    for sub in ["sprites", "sprites/entities"] {
        if let Ok(dir) = std::fs::read_dir(data.join(sub)) {
            folders.extend(dir.flatten().map(|e| e.path()).filter(|p| p.is_dir() && p.file_name().map_or(false, |n| n.to_string_lossy().starts_with("sprite_") || n.to_string_lossy().starts_with("entity_"))));
        }
    }
    let mut pictures = 0;
    for folder in &folders {
        let yaml = if folder.join("sprite.yaml").exists() { folder.join("sprite.yaml") } else { folder.join("entity.yaml") };
        let def: SpriteDef = match std::fs::read_to_string(&yaml).map_err(anyhow::Error::from).and_then(|t| serde_yaml::from_str(&t).map_err(anyhow::Error::from)) {
            Ok(d) => d,
            Err(e) => {
                println!("{}: {:#}", yaml.display(), e);
                problems += 1;
                continue;
            }
        };
        for (pose, views) in &def.poses {
            for pic in views.pictures() {
                pictures += 1;
                if !folder.join(&pic.file).exists() {
                    println!("{}: pose {}: {} is missing", folder.display(), pose, pic.file);
                    problems += 1;
                }
            }
        }
    }
    println!("{} sprite and entity folders, {} pictures", folders.len(), pictures);
    problems
}

fn verify_legacy(data: &Path, assets: &Path) -> Result<()> {
    let index = load_index(data)?;
    let mut bad = 0;
    for entry in &index.rooms {
        let room = Room::load(&room_dir(data, entry.id))?;
        let legacy = assets.join("rooms").join(format!("room_{:03}", entry.id));
        let hm = read_csv(&legacy.join("heightmap.csv"), 16)?;
        let bg = read_csv(&legacy.join("map_bg.csv"), 10)?;
        let fg = read_csv(&legacy.join("map_fg.csv"), 10)?;
        let mut mismatches = 0;
        for (iy, row) in hm.iter().enumerate() {
            for (ix, &v) in row.iter().enumerate() {
                match room.cell(ix as i32, iy as i32) {
                    Some(c) if c.raw() as u32 == v => {}
                    _ => mismatches += 1,
                }
            }
        }
        for (name, grid) in [("background", &bg), ("foreground", &fg)] {
            for (by, row) in grid.iter().enumerate() {
                for (bx, &v) in row.iter().enumerate() {
                    if room.block(name, bx as i32, by as i32) != Some(v) {
                        mismatches += 1;
                    }
                }
            }
        }
        if mismatches > 0 {
            println!("room {}: {} cells differ from the engine's files", entry.id, mismatches);
            bad += 1;
        }
    }
    println!("{} rooms compared, {} differ", index.rooms.len(), bad);
    if bad > 0 {
        bail!("{} rooms differ", bad);
    }
    Ok(())
}

fn room_summary(data: &Path, id: i32) -> Result<()> {
    let room = Room::load(&room_dir(data, id))?;
    let f = &room.file;
    println!("room {} \"{}\" ({})", f.id, f.name, f.path);
    println!("  tileset {} palette {} music {} heights {}..{}", f.tileset.id, f.tileset.palette, f.music, f.heights.z_begin, f.heights.z_end);
    println!("  grid {:?} origin {:?}, heightmap {:?}, blocks {:?} at {:?}", f.map.grid.size, f.map.grid.origin, f.map.heightmap.size, f.map.blocks.size, f.map.blocks.origin);
    for e in &f.entities {
        println!("  entity #{} {} ({}) at {:?} {} behaviour {}", e.id, e.kind, e.type_name, e.position, e.orientation, e.behaviour);
    }
    for w in &f.warps {
        println!("  warp {:?} -> room {} cell {:?} ({})", w.area, w.target_room, w.target_cell, w.kind);
    }
    Ok(())
}

/// Writes a built archive and reads it back (every file is checked against its crc32).
fn write_archive(b: &ls_data::lsa::LsaBuilder, path: &Path) -> Result<(usize, usize)> {
    let bytes = b.build()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, &bytes)?;
    let a = ls_data::lsa::Lsa::open(path)?;
    let mut raw = 0;
    for n in a.names() {
        raw += a.read(n)?.len();
    }
    Ok((raw, bytes.len()))
}

/// One archive per sprite: `sprite_NNN.lsa` (sprite.yaml and the pictures of data/sprites/sprite_NNN) and, per entity type,
/// `entities/entity_NNN.lsa` (entity.yaml and the pictures of data/sprites/entities/entity_NNN).
fn pack_sprites(dir: &Path, out: &Path) -> Result<()> {
    let (mut files, mut raw, mut packed) = (0, 0, 0);
    let (mut sprites, mut entities) = (0, 0);
    for (sub, prefix) in [("", "sprite_"), ("entities", "entity_")] {
        let base = dir.join(sub);
        if !base.exists() {
            continue;
        }
        for e in std::fs::read_dir(&base)? {
            let p = e?.path();
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            if p.is_dir() && name.starts_with(prefix) {
                let mut b = ls_data::lsa::LsaBuilder::new();
                b.add_dir(&p)?;
                let (r, k) = write_archive(&b, &out.join(sub).join(format!("{}.lsa", name)))?;
                if prefix == "sprite_" {
                    sprites += 1;
                } else {
                    entities += 1;
                }
                files += 1;
                raw += r;
                packed += k;
            }
        }
    }
    println!("{} sprite archives + {} entity archives ({} files), {} bytes of files in {} bytes", sprites, entities, files, raw, packed);
    Ok(())
}

fn write_wav(path: &Path, pcm: &[i16], rate: u32) -> Result<()> {
    let bytes = (pcm.len() * 2) as u32;
    let mut f = Vec::with_capacity(44 + bytes as usize);
    f.extend_from_slice(b"RIFF");
    f.extend_from_slice(&(36 + bytes).to_le_bytes());
    f.extend_from_slice(b"WAVEfmt ");
    f.extend_from_slice(&16u32.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&2u16.to_le_bytes());
    f.extend_from_slice(&rate.to_le_bytes());
    f.extend_from_slice(&(rate * 4).to_le_bytes());
    f.extend_from_slice(&4u16.to_le_bytes());
    f.extend_from_slice(&16u16.to_le_bytes());
    f.extend_from_slice(b"data");
    f.extend_from_slice(&bytes.to_le_bytes());
    for v in pcm {
        f.extend_from_slice(&v.to_le_bytes());
    }
    std::fs::write(path, f)?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let default_data = PathBuf::from("data");
    match args.first().map(String::as_str) {
        Some("check") => check(&args.get(1).map(PathBuf::from).unwrap_or(default_data)),
        Some("verify-legacy") => verify_legacy(
            &args.get(1).map(PathBuf::from).unwrap_or(default_data),
            &args.get(2).map(PathBuf::from).unwrap_or(PathBuf::from("../landstalker_engine/assets")),
        ),
        Some("room") => {
            let id: i32 = args.get(1).ok_or_else(|| anyhow::anyhow!("usage: lsctl room <id> [data_dir]"))?.parse()?;
            room_summary(&args.get(2).map(PathBuf::from).unwrap_or(default_data), id)
        }
        Some("lsa-pack") => {
            let (dir, out) = (args.get(1), args.get(2));
            let (Some(dir), Some(out)) = (dir, out) else { bail!("usage: lsctl lsa-pack <folder> <out.lsa>") };
            let mut b = ls_data::lsa::LsaBuilder::new();
            b.add_dir(Path::new(dir))?;
            let bytes = b.build()?;
            std::fs::write(out, &bytes)?;
            let a = ls_data::lsa::Lsa::from_bytes(bytes.clone())?;
            // every file is read back (and its crc checked) before the archive is declared good
            let raw: usize = a.names().map(|n| a.size_of(n).unwrap_or(0)).sum();
            for n in a.names() {
                a.read(n)?;
            }
            println!("{}: {} files, {} bytes of files in {} bytes", out, a.len(), raw, bytes.len());
            Ok(())
        }
        Some("evt-render") => {
            let (Some(file), Some(secs), Some(out)) = (args.get(1), args.get(2), args.get(3)) else { bail!("usage: lsctl evt-render <dump.evt> <seconds> <out.wav>") };
            let events = ls_data::vgm::parse_events(&std::fs::read(file)?)?;
            let rate = ls_data::vgm::Synth::sample_rate(ls_data::vgm::YM_CLOCK);
            let pcm = ls_data::vgm::render_events(&events, (secs.parse::<f64>()? * rate as f64) as usize);
            write_wav(Path::new(out), &pcm, rate)?;
            println!("{} events, {} frames -> {}", events.len(), pcm.len() / 2, out);
            Ok(())
        }
        Some("evt-to-vgm") => {
            let (Some(file), Some(out)) = (args.get(1), args.get(2)) else { bail!("usage: lsctl evt-to-vgm <dump.evt> <out.vgm>") };
            let mut events = ls_data::vgm::parse_events(&std::fs::read(file)?)?;
            // the song starts at its first write; the console's clock: 6 master clocks per OPN clock
            let clocks_per_sample = ls_data::vgm::YM_CLOCK as f64 / 6.0 / 44100.0;
            let sample = |t: u64| (t as f64 / clocks_per_sample).round() as u32;
            let once = args.iter().any(|a| a == "--once");
            // a song that plays samples is searched for its loop without them (they do not repeat to the clock)
            let plain = ls_data::vgm::without_dac(&events);
            let samples = plain.len() < events.len();
            let jitter = if samples { ls_data::vgm::JITTER_WITH_SAMPLES } else { ls_data::vgm::JITTER };
            let loop_found = if once { None } else { ls_data::vgm::detect_loop_with(&plain, jitter) };
            // a song without a loop ends where the driver, done, writes the same few registers over and over
            let tail = if loop_found.is_none() { ls_data::vgm::detect_repeat(&plain, 0, jitter, false).map(|(start, _)| plain[start].0) } else { None };
            let (kept, loop_sample, total) = match loop_found {
                Some((start, period)) => {
                    // the intro and one turn of the loop: the loop ends where the next turn begins
                    let (t_start, t_end) = (plain[start].0, plain[start + period].0);
                    events.retain(|e| e.0 < t_end);
                    (events, Some(sample(t_start)), Some(sample(t_end)))
                }
                None => {
                    let end = tail.unwrap_or(u64::MAX).min(ls_data::vgm::song_end(&events));
                    events.retain(|e| e.0 < end);
                    let last = events.last().map_or(0, |e| e.0);
                    (events, None, Some(sample(last)))
                }
            };
            let list: Vec<(u32, ls_data::vgm::Write)> = kept.iter().map(|&(t, w)| (sample(t), w)).collect();
            let total = total.unwrap_or(0);
            let mut bytes = ls_data::vgm::write_vgm(&list, total, loop_sample);
            if out.ends_with(".vgz") {
                bytes = ls_data::vgm::gzip(&bytes);
            }
            std::fs::write(out, &bytes)?;
            match loop_found {
                Some((start, period)) => println!("{}: loop from event {} ({} events per turn){}, {} samples, {} bytes", out, start, period, if samples { ", with samples" } else { "" }, total, bytes.len()),
                None => println!("{}: no loop{}, {} samples, {} bytes", out, if samples { ", with samples" } else { "" }, total, bytes.len()),
            }
            Ok(())
        }
        Some("vgm-render") => {
            let (Some(file), Some(out)) = (args.get(1), args.get(2)) else { bail!("usage: lsctl vgm-render <file.vgm> <out.wav>") };
            let v = std::sync::Arc::new(ls_data::vgm::Vgm::parse(&std::fs::read(file)?)?);
            let r = v.render();
            write_wav(Path::new(out), &r.pcm, r.sample_rate)?;
            println!("{} events, {} frames at {} Hz, loop from sample {:?} of {} -> {}", v.events.len(), r.pcm.len() / 2, r.sample_rate, v.loop_sample, v.total_samples, out);
            Ok(())
        }
        Some("find-floor") => {
            // rooms with cells of a floor type (hexadecimal), e.g. 10 spikes, 19 swamp, 29 lava, 2a..2d ice
            let ty = u8::from_str_radix(args.get(1).map(String::as_str).unwrap_or("10").trim_start_matches("0x"), 16)?;
            let data = args.get(2).map(PathBuf::from).unwrap_or(default_data);
            for entry in &load_index(&data)?.rooms {
                let room = Room::load(&room_dir(&data, entry.id))?;
                let (mut n, mut first) = (0, None);
                for y in -20..120 {
                    for x in -20..120 {
                        if room.cell_at_world(x, y).map_or(false, |c| c.floor_type == ty) {
                            n += 1;
                            first.get_or_insert((x, y));
                        }
                    }
                }
                if n > 0 {
                    println!("room {:>3}: {} cells of type {:#x}, first at {:?}", entry.id, n, ty, first.unwrap());
                }
            }
            Ok(())
        }
        Some("lsa-pack-sprites") => {
            let dir = PathBuf::from(args.get(1).map(String::as_str).unwrap_or("data/sprites"));
            let out = PathBuf::from(args.get(2).map(String::as_str).unwrap_or(dir.to_str().unwrap_or("data/sprites")));
            pack_sprites(&dir, &out)
        }
        Some("lsa-list") => {
            let a = ls_data::lsa::Lsa::open(Path::new(args.get(1).ok_or_else(|| anyhow::anyhow!("usage: lsctl lsa-list <file.lsa>"))?))?;
            for n in a.names() {
                println!("{:>8}  {}", a.size_of(n).unwrap_or(0), n);
            }
            println!("{} files", a.len());
            Ok(())
        }
        Some("lsa-unpack") => {
            let (Some(file), Some(dir)) = (args.get(1), args.get(2)) else { bail!("usage: lsctl lsa-unpack <file.lsa> <folder>") };
            let a = ls_data::lsa::Lsa::open(Path::new(file))?;
            for n in a.names() {
                let p = Path::new(dir).join(n);
                if let Some(parent) = p.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(p, a.read(n)?)?;
            }
            println!("{} files written to {}", a.len(), dir);
            Ok(())
        }
        _ => {
            println!("usage: lsctl check [data] | verify-legacy [data] [engine_assets] | room <id> [data] | find-floor <type> | lsa-pack <folder> <out.lsa> | lsa-pack-sprites [sprites] [out] | lsa-list <file> | lsa-unpack <file> <folder>");
            Ok(())
        }
    }
}
