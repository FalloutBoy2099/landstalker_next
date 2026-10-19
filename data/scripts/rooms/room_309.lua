-- Behaviours of room 309 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[770] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(140)
  pause(e, 0)
end

B[771] = function(e)
  wait_flag(e, 140, true)
  show_when_clear(e)
  ::L4::
  move_down_until_collision(e)
  pause(e, 20)
  pause(e, 20)
  jump_guard(e) goto L4
end
SCRIPTED[771] = true
WAITS_FIRST[771] = true
