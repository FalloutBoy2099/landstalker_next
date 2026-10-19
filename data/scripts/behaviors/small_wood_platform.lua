-- Behaviours that several rooms use for the entities of this kind (small wood platform).

B[208] = function(e)
  ::L0::
  move_up_relative(e, 32)
  move_down_relative(e, 80)
  move_up_relative(e, 48)
  jump_guard(e) goto L0
end
