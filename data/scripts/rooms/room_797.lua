-- Behaviours of room 797 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[45] = function(e)
  ::L0::
  move_down_timed(e, 64)
  move_up_to(e, 80)
  move_down_timed(e, 16)
  jump_guard(e) goto L0
end
