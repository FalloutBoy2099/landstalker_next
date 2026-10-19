-- Behaviours of room 368 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[817] = function(e)
  ::L0::
  move_down_until_collision(e)
  sfx(88)
  jump_guard(e) goto L0
end
