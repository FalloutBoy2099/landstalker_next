-- Behaviours of room 755 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[205] = function(e)
  ::L0::
  move_down_until_collision(e)
  move_up_to(e, 172)
  jump_guard(e) goto L0
end
