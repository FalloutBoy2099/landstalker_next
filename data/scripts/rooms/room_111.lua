-- Behaviours of room 111 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[877] = function(e)
  wait_trigger(e, 1)
  tile_swap(8)
  pause_player()
  pause(e, 30)
  move_until_collision(e)
  playback(59)
  pause(e, 40)
  tile_swap(3)
  set_flag(8)
  e.visible = false; e.base_visible = false; do return end
end

B[884] = function(e)
  e.visible = false; e.base_visible = false
  wait_flag(e, 8, true)
  cutscene(e, 0x100 + 78)
  sfx(47)
  pause(e, 30)
  move_noclip(e, 32)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[884] = true

B[885] = function(e)
  cutscene(e, 0x100 + 79)
  pause(e, 0)
end
SCRIPTED[885] = true

B[886] = function(e)
  wait_trigger(e, 135)
  pause_player()
  cutscene(e, 0x100 + 86)
  pause(e, 30)
  wait_trigger(e, 138)
  cutscene(e, 0x100 + 80)
  wait_trigger(e, 136)
  pause_player()
  sfx(11)
  move_to(e, 36, 33)
  move_noclip(e, 160)
  turn(e, 3)
  move_noclip(e, 160)
  rotate_player(3)
  turn_wait(e, 1)
  pause(e, 10)
  cutscene(e, 0x100 + 81)
  set_flag(9)
  pause(e, 0)
end
SCRIPTED[886] = true

-- story actions
story(334, {
  block_cells({{29, 34}, {29, 35}, {29, 36}, {29, 35}, {29, 36}, {29, 37}, {29, 36}, {29, 37}, {29, 38}, {29, 37}, {29, 38}, {29, 39}, {29, 38}, {29, 39}, {29, 40}, {29, 39}, {29, 40}, {29, 41}, {29, 40}, {29, 41}, {29, 42}, {29, 41}, {29, 42}, {29, 43}, {29, 42}, {29, 43}, {29, 44}, {29, 43}, {29, 44}, {29, 45}, {29, 44}, {29, 45}, {29, 46}, {29, 45}, {29, 46}, {29, 47}, {29, 46}, {29, 47}, {29, 48}, {29, 47}, {29, 48}, {29, 49}, {29, 48}, {29, 49}, {29, 50}, {29, 49}, {29, 50}, {29, 51}, {29, 50}, {29, 51}, {29, 52}}),
  say(320),
  show_entity(-1, true),
  pause_frames(5),
  pause_frames(5),
})

story(335, {
  make_hostile(2, false),
})

story(336, {
  play_sound(34),
  say(321),
})

story(337, {
  say(322),
})

story(342, {
  hide_entity(2),
})
