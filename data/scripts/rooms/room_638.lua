-- Behaviours of room 638 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[262] = function(e)
  wait_flag(e, 45, false)
  wait_flag(e, 8, true)
  pause_player()
  pause(e, 30)
  sfx(88)
  tile_swap(2)
  rotate_player(3)
  pause(e, 30)
  move_to(e, 17, 21)
  move_noclip(e, 32)
  turn(e, 2)
  move_noclip(e, 48)
  turn_wait(e, 0)
  cutscene(e, 59)
  turn(e, 2)
  move_noclip(e, 16)
  pause(e, 45)
  turn(e, 0)
  move_noclip(e, 16)
  cutscene(e, 60)
  turn(e, 2)
  move_noclip(e, 48)
  set_flag(9)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[262] = true
WAITS_FIRST[262] = true

B[263] = function(e)
  wait_flag(e, 45, false)
  wait_flag(e, 8, true)
  pause(e, 76)
  move_to(e, 17, 21)
  move_noclip(e, 32)
  turn(e, 2)
  move_noclip(e, 32)
  wait_flag(e, 9, true)
  turn(e, 0)
  move_noclip(e, 32)
  turn(e, 3)
  move_noclip(e, 32)
  set_flag(45)
  sfx(88)
  tile_swap(3)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[263] = true
WAITS_FIRST[263] = true

B[267] = function(e)
  wait_flag(e, 45, false)
  wait_flag(e, 45, true)
  turn_wait(e, 0)
  pause(e, 1)
  cutscene(e, 66)
  turn(e, 3)
  pause(e, 0)
end
SCRIPTED[267] = true
WAITS_FIRST[267] = true

-- story actions
story(59, {
  say(69),
})

story(60, {
  say(70),
})

story(66, {
  say(71),
})
