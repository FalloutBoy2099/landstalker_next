-- Behaviours of room 647 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[357] = function(e)
  wait_flag(e, 8, true)
  pause_player()
  turn(e, 1)
  move_noclip(e, 16)
  turn(e, 2)
  move_noclip(e, 16)
  turn(e, 0)
  pause(e, 30)
  playback(33)
  tile_swap(1)
  tile_swap(3)
  pause(e, 30)
  set_flag(9)
  wait_flag(e, 10, true)
  cutscene(e, 130)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[357] = true
WAITS_FIRST[357] = true

B[358] = function(e)
  wait_flag(e, 9, true)
  move_noclip(e, 160)
  turn_wait(e, 2)
  move_noclip(e, 160)
  tile_swap(4)
  set_flag(10)
  pause(e, 0)
end
SCRIPTED[358] = true
WAITS_FIRST[358] = true

B[359] = function(e)
  wait_trigger(e, 57)
  pause_player()
  e.visible = false; e.base_visible = false
  move_to(e, 38, 61)
  rotate_player(3)
  pause(e, 2)
  cutscene(e, 131)
  wait_flag(e, 9, true)
  playback(34)
  pause(e, 30)
  turn(e, 0)
  move_noclip(e, 16)
  rotate_player(1)
  turn(e, 3)
  move_wait(e, 16)
  pause(e, 8)
  cutscene(e, 132)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[359] = true

B[360] = function(e)
  wait_flag(e, 8, true)
  tile_swap(1)
  tile_swap(3)
  move_noclip(e, 160)
  turn_wait(e, 3)
  move_noclip(e, 160)
  tile_swap(2)
  set_flag(9)
  pause(e, 0)
end
SCRIPTED[360] = true
WAITS_FIRST[360] = true

-- story actions
story(130, {
  say(120),
})

story(131, {
  say(121),
  if_yes({
      story_flag(8, true),
    }, {
      lock_controls(false),
      hide_entity(-1),
      swap_tiles(40),
    }),
})

story(132, {
  say(122),
})
