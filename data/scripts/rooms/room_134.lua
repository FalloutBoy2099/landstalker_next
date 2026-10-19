-- Behaviours of room 134 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[757] = function(e)
  ::L0::
  wait_flag(e, 8, false)
  show_when_clear(e)
  e.speed = 8
  move_down_until_collision(e)
  sfx(88)
  e.falls = true; e.fall_v = 0
  e.speed = 4
  move_wait(e, 36)
  turn_wait(e, 0)
  move_wait(e, 72)
  sfx(84)
  pause(e, 10)
  do return end  -- unknown command 4D
end
SCRIPTED[757] = true
WAITS_FIRST[757] = true
