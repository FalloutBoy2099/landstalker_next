-- Behaviours of room 202 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[114] = function(e)
  ::L0::
  move_down_until_collision(e)
  e.speed = 1
  move_up_to(e, 64)
  e.speed = 4
  jump_guard(e) goto L0
end
