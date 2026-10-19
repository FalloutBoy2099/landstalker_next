-- Behaviours of room 413 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[481] = function(e)
  show_when_clear(e)
  wait_trigger(e, 95)
  e.visible = false; e.base_visible = false; do return end
end

B[482] = function(e)
  ::L0::
  move_up_to(e, 96)
  pause(e, 20)
  move_down_until_collision(e)
  pause(e, 20)
  jump_guard(e) goto L0
end
