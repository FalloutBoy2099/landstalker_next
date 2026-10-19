-- Behaviours of room 711 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[514] = function(e)
  wait_trigger(e, 97)
  pause_player()
  move_to(e, 35, 29)
  move_noclip(e, 96)
  turn(e, 3)
  move_noclip(e, 48)
  turn(e, 2)
  move_noclip(e, 16)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[514] = true

B[515] = function(e)
  wait_trigger(e, 97)
  move_to(e, 42, 34)
  move_noclip(e, 128)
  turn(e, 2)
  move_noclip(e, 48)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[515] = true

B[516] = function(e)
  wait_trigger(e, 97)
  move_to(e, 28, 36)
  cutscene(e, 212)
  move_noclip(e, 72)
  turn(e, 2)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[516] = true

B[517] = function(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  set_flag(307)
  pause(e, 0)
end

-- story actions
story(212, {
  say(195),
})
