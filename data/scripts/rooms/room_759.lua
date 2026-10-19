-- Behaviours of room 759 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[304] = function(e)
  ::L0::
  move_down_until_collision(e)
  pause(e, 15)
  move_up_relative(e, 128)
  jump_guard(e) goto L0
end
