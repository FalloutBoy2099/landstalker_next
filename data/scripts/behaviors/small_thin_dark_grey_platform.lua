-- Behaviours that several rooms use for the entities of this kind (small thin dark grey platform).

B[700] = function(e)
  ::L0::
  show_when_clear(e)
  wait_flag(e, 9, true)
  e.visible = false
  wait_flag(e, 9, false)
  jump_guard(e) goto L0
end
SCRIPTED[700] = true
