-- Behaviours that several rooms use for the entities of this kind (chest).

B[601] = function(e)
  wait_trigger(e, 95)
  sfx(86)
  show_when_clear(e)
  pause(e, 0)
end
