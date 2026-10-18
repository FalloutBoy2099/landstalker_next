# EmuHawk: the original game as a reference

`run.py` runs the US ROM (`D:\Git\Landstalker\roms\landstalker_us.md`) in EmuHawk (`D:\Roms\Hawk\EmuHawk.exe`, Genesis Plus GX core)
under `hawk.lua`, which presses buttons at given frames, takes screenshots, logs RAM symbols and writes savestates. The RAM symbols
come from the disassembly (`gen_ram.py` -> `ram.lua`, regenerate when ram.inc changes; ~2000 names, `Player_X`, `g_RmNum1`, ...).

```
python tools/emuhawk/run.py --out DIR --end 1500 --input "300:+Start;310:-Start" --shot 200,1500 --log g_RmNum1:w,Player_X --every 30
python tools/emuhawk/run.py --out DIR --state DIR0/state_select.State --end 700 --input "10:+C;20:-C"      # starts from a savestate
```

* frames are 60 Hz; buttons `Up Down Left Right A B C Start`; log suffix `:w` = word, `:l` = long (big endian, 68000 bus `M68K BUS`)
* a Lua error in the script opens a message box and blocks the run: kill `EmuHawk.exe`, fix. **No backslash in the Lua source** (the
  script loader refuses `\/` in strings): paths come from environment variables.
* the Lua chunk is loaded as a string: `debug.getinfo` gives no file path.

## Known good points (US ROM)
* boot: `300:+Start;310:-Start;600:+Start;610:-Start;900:+Start;910:-Start` reaches the file select (savestate at frame 1450);
  from there `C` at 10, 100, 200, 300 (10 frames each) starts a new game in file 1: room 139 from frame ~220, Nigel at (62, 24).
* original intro timeline (frames after the file select): room 139 220-1080, 140 1080-2600 (controller playback on), 141 2600-3440, 142 from 3440.
* RAM: `g_RmNum1` (word) = room, `Player_X/Y` (bytes, tiles), `Player_SubX/SubY`, `Player_Z` (word), `SpriteN_*` at +0x80 each,
  `g_Flags` ($FF1000) story flags, `g_LockPlayerActions`, `g_ControllerPlayback`, `g_IntroStringToDisplay`.

## Measured on the original (US ROM, 60 Hz)
* a frame of the console = one tick of the entities' scripts: an entity of speed 2 moves 2/16 of a cell every frame (the port used 30 ticks/s: fixed, 60);
* Nigel walks 2/16 cell per frame = 7.5 cells/s (`g_PlayerSpeed` 2); a jump lasts 24 frames and rises 28/16 of a unit (button B in the ROM);
* the screen keeps Nigel at 85 pixels from the top (not in the middle) and 160 from the left, in a picture of 320x224 (the port: scale 4 = 1280 px);
* the scenes of the intro (rooms 139 to 143) show lines 24 to 184 only (black bars);
* the fade is `g_PaletteDarkenCurrentBrightness`: 4096 black, 0 clear (65024 = clear too), about 18 frames; a caption (`g_IntroStringToDisplay`) counts its 120 frames
  down only while the screen is lit;
* entity slots: `Player` is slot 0, `SpriteN` at +0x80 each = entity N-1 of the room file; `RotationAndSize` bits 6-7: 0 NE 1 SE 2 SW 3 NW;
  slot 0 stays the camera's anchor even when it is not drawn (room 142: Nigel is not there at the start, the old man comes on the barge).

## Recorded cutscenes (`data/cutscenes/*.track`)
`make_track.py LOG.csv OUT.track FIRST LAST [HANDOVER_ROOMS [EXTRA]]` turns a log of `analyze.entcols()` (every 2 frames) into the track the port plays
(`track.rs`, `cutscene::track_update`): where every entity is, fades, captions, messages. `intro.track` was made with
`run.py --state <file select> --input <C taps> --log $(python -c "import analyze; print(analyze.entcols())") --every 2` then
`make_track.py log.csv data/cutscenes/intro.track 218 12500 142 intro_extra.txt`: rooms 139, 140, 141 are played from the recording, then the game's scripts take over in 142.

## Warping to any room of the original (enemies, objects)
The US ROM has a debug "next room" on controller 2 Start, switched off by `DebugModeEnable` (word at ROM offset 0x210 = $FFFF). Make a **copy** of the
ROM with those two bytes at 0 (`ls_debug.md`, never the user's file) and run `run.py --script warp.lua --warp ROOM --rom ls_debug.md --state STATE ...`:
warp.lua writes `g_RmNum1 = ROOM - 1` into the RAM and presses P2 Start, Nigel lands on the first free tile of the room. `enemy_batch.py` does that for a list
of rooms while Nigel walks about and `enemy_stats.py` summarises the enemies' moves.

## Enemies (measured)
Speed byte s = s/16 cell per frame = 3.75 cells/s per level. Walkers (orcs, skeletons, bubbles...) patrol one cell (16 frames) then wait 64 frames, and chase
without stopping at the same speed; the spiked balls move at 2 and 4 (7.5 and 15 cells/s). `data/scripts/ai.lua` follows this (bosses keep their old slower
speeds: not measured, they sleep until the story wakes them).
