-- Behaviours of room 188 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[81] = function(e)
  ::L0::
  wait_trigger(e, 1)
  move_wait(e, 98)
  turn_wait(e, e.orientation + 2)
  pause(e, 15)
  jump_guard(e) goto L0
end

B[112] = function(e)
  ::L0::
  move_down_until_collision(e)
  pause(e, 20)
  move_up_to(e, 120)
  jump_guard(e) goto L0
end
