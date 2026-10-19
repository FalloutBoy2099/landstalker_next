-- Behaviours of room 6 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[83] = function(e)
  wait_trigger(e, 3)
  e.falls = true; e.fall_v = 0
  move_down_until_collision(e)
  sfx(88)
  pause(e, 0)
end

B[84] = function(e)
  wait_trigger(e, 4)
  e.falls = true; e.fall_v = 0
  move_down_until_collision(e)
  sfx(88)
  pause(e, 0)
end

B[92] = function(e)
  wait_trigger(e, 5)
  set_flag(83)
  e.falls = true; e.fall_v = 0
  move_down_until_collision(e)
  sfx(88)
  pause(e, 0)
end
