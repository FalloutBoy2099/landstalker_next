-- Behaviours of room 565 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[824] = function(e)
  wait_trigger(e, 2)
  set_flag(108)
  ::L5::
  move_down_until_collision(e)
  pause(e, 20)
  pause(e, 20)
  jump_guard(e) goto L5
end
