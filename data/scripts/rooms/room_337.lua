-- Behaviours of room 337 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[891] = function(e)
  wait_flag(e, 407, true)
  show_when_clear(e)
  ::L4::
  wait_trigger(e, 1)
  pause(e, 30)
  e.falls = true; e.fall_v = 0
  move_down_until_collision(e)
  pause(e, 180)
  do return end  -- unknown command 4D
end
SCRIPTED[891] = true
WAITS_FIRST[891] = true
