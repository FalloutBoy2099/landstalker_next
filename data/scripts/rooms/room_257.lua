-- Behaviours of room 257 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[244] = function(e)
  ::L0::
  move_up_relative(e, 144)
  pause(e, 30)
  move_down_relative(e, 224)
  pause(e, 30)
  move_up_relative(e, 80)
  jump_guard(e) goto L0
end
