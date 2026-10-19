-- Behaviours of room 56 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[302] = function(e)
  wait_trigger(e, 42)
  pause_player()
  move_noclip(e, 20)
  turn(e, 3)
  move_noclip(e, 8)
  rotate_player(1)
  pause(e, 1)
  cutscene(e, 100)
  set_flag(8)
  wait_flag(e, 9, true)
  pause(e, 16)
  move_noclip(e, 16)
  set_flag(161)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[302] = true

B[303] = function(e)
  wait_flag(e, 8, true)
  move_to(e, 32, 18)
  move_noclip(e, 64)
  cutscene(e, 101)
  set_flag(9)
  pause(e, 0)
end
SCRIPTED[303] = true
WAITS_FIRST[303] = true

-- story actions
story(100, {
  say(109),
})

story(101, {
  say(110),
})
