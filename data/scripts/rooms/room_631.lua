-- Behaviours of room 631 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[322] = function(e)
  wait_flag(e, 8, true)
  pause_player()
  tile_swap(1)
  turn(e, 3)
  move_noclip(e, 16)
  e.visible = false; e.base_visible = false
  pause(e, 80)
  cutscene(e, 108)
  pause(e, 80)
  turn(e, 1)
  e.visible = true; e.base_visible = true
  move_noclip(e, 16)
  tile_swap(2)
  cutscene(e, 107)
  turn(e, 0)
  move_noclip(e, 32)
  turn(e, 1)
  pause(e, 0)
end
SCRIPTED[322] = true
WAITS_FIRST[322] = true

B[581] = function(e)
  cutscene(e, 238)
  pause(e, 0)
end
SCRIPTED[581] = true

B[582] = function(e)
  wait_flag(e, 337, false)
  turn(e, 1)
  move_wait(e, 10)
  turn_wait(e, 2)
  wait_flag(e, 337, true)
  pause_player()
  turn(e, 3)
  move_wait(e, 10)
  turn_wait(e, 2)
  pause(e, 0)
end
SCRIPTED[582] = true
WAITS_FIRST[582] = true

B[583] = function(e)
  wait_flag(e, 337, false)
  turn(e, 3)
  move_wait(e, 10)
  turn_wait(e, 2)
  wait_flag(e, 337, true)
  turn(e, 1)
  move_wait(e, 10)
  turn_wait(e, 2)
  pause(e, 0)
end
SCRIPTED[583] = true
WAITS_FIRST[583] = true

-- story actions
story(107, {
  say(116),
})

story(108, {
  play_sound(95),
})
