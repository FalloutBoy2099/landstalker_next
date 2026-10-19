-- Behaviours of room 104 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[418] = function(e)
  wait_trigger(e, 76)
  pause_player()
  move_to(e, 28, 24)
  move_noclip(e, 64)
  turn(e, 2)
  move_noclip(e, 80)
  e.falls = true; e.fall_v = 0
  cutscene(e, 170)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[418] = true

B[419] = function(e)
  wait_trigger(e, 76)
  pause(e, 32)
  move_to(e, 28, 24)
  move_noclip(e, 64)
  turn(e, 2)
  move_noclip(e, 64)
  e.falls = true; e.fall_v = 0
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[419] = true

B[420] = function(e)
  wait_trigger(e, 76)
  pause(e, 64)
  move_to(e, 28, 24)
  move_noclip(e, 64)
  turn(e, 2)
  move_noclip(e, 40)
  e.falls = true; e.fall_v = 0
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[420] = true

B[421] = function(e)
  wait_trigger(e, 76)
  move_timed(e, 128)
  turn(e, 1)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[421] = true

B[422] = function(e)
  wait_trigger(e, 76)
  pause(e, 130)
  wait_trigger(e, 2)
  set_flag(258)
  e.visible = false; e.base_visible = false; do return end
end

-- story actions
story(170, {
  say(152),
})
