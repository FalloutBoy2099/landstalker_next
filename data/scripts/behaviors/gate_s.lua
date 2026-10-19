-- Behaviours that several rooms use for the entities of this kind (gate s).

B[600] = function(e)
  wait_flag(e, 8, true)
  show_when_clear(e)
  sfx(88)
  wait_trigger(e, 2)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[600] = true
WAITS_FIRST[600] = true
