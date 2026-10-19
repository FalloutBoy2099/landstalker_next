-- Behaviours of room 92 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[207] = function(e)
  ::L0::
  move_down_until_collision(e)
  pause(e, 20)
  move_up_to(e, 80)
  jump_guard(e) goto L0
end
