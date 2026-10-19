-- Behaviours of room 316 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[77] = function(e)
  ::L0::
  wait_trigger(e, 1)
  move_until_collision(e)
  turn_wait(e, e.orientation + 2)
  wait_trigger(e, 6)
  jump_guard(e) goto L0
end

B[306] = function(e)
  wait_trigger(e, 2)
  show_when_clear(e)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end
