-- Behaviours of room 324 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[732] = function(e)
  wait_trigger(e, 2)
  set_flag(100)
  pause(e, 0)
end

B[788] = function(e)
  wait_trigger(e, 2)
  show_when_clear(e)
  ::L3::
  move_down_until_collision(e)
  pause(e, 20)
  pause(e, 20)
  jump_guard(e) goto L3
end
