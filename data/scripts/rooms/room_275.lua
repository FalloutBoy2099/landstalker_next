-- Behaviours of room 275 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[56] = function(e)
  ::L0::
  move_wait(e, 48)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[57] = function(e)
  ::L0::
  move_wait(e, 32)
  turn_wait(e, e.orientation + 3)
  move_timed(e, 24)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[58] = function(e)
  ::L0::
  move_down_relative(e, 80)
  turn_wait(e, 3)
  move_wait(e, 40)
  move_up_relative(e, 80)
  turn_wait(e, 1)
  move_wait(e, 40)
  jump_guard(e) goto L0
end

B[59] = function(e)
  ::L0::
  move_wait(e, 32)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 128)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 128)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[605] = function(e)
  ::L0::
  e.falls = true; e.fall_v = 0
  move_until_collision(e)
  do return end  -- unknown command 4D
end
