-- Behaviours of room 751 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[206] = function(e)
  ::L0::
  move_up_relative(e, 128)
  wait_flag(e, 183, true)
  move_down_relative(e, 144)
  move_up_relative(e, 16)
  jump_guard(e) goto L0
end
SCRIPTED[206] = true

B[368] = function(e)
  show_when_clear(e)
  wait_trigger(e, 60)
  e.visible = false; e.base_visible = false; do return end
end

B[369] = function(e)
  wait_flag(e, 181, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[369] = true
WAITS_FIRST[369] = true

B[370] = function(e)
  wait_flag(e, 182, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[370] = true
WAITS_FIRST[370] = true
