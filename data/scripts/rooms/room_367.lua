-- Behaviours of room 367 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[815] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(404)
  pause(e, 0)
end

B[816] = function(e)
  wait_flag(e, 404, true)
  e.speed = 1
  ::L4::
  move_until_collision(e)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L4
end
SCRIPTED[816] = true
WAITS_FIRST[816] = true
