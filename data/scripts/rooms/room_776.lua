-- Behaviours of room 776 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[150] = function(e)
  ::L0::
  move_wait(e, 68)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 52)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 12)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 40)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 56)
  turn_wait(e, e.orientation + 1)
  move_timed(e, 12)
  turn_wait(e, e.orientation + 1)
  jump_guard(e) goto L0
end

B[151] = function(e)
  move_wait(e, 8)
  ::L2::
  turn_wait(e, e.orientation + 1)
  move_wait(e, 52)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 12)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 40)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 56)
  turn(e, 1)
  move_wait(e, 12)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 68)
  jump_guard(e) goto L2
end

B[152] = function(e)
  ::L0::
  move_wait(e, 12)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 40)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 56)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 12)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 68)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 52)
  turn_wait(e, e.orientation + 1)
  jump_guard(e) goto L0
end

B[153] = function(e)
  move_wait(e, 48)
  ::L2::
  turn_wait(e, e.orientation + 1)
  move_wait(e, 12)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 68)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 52)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 12)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 40)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 56)
  jump_guard(e) goto L2
end

B[173] = function(e)
  ::L0::
  move_down_until_collision(e)
  sfx(88)
  pause(e, 10)
  move_up_to(e, 80)
  jump_guard(e) goto L0
end
