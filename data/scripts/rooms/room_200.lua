-- Behaviours of room 200 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[242] = function(e)
  pause(e, 30)
  show_when_clear(e)
  ::L3::
  wait_flag(e, 8, true)
  move_up_relative(e, 64)
  e.speed = 8
  wait_flag(e, 8, false)
  move_down_relative(e, 8)
  sfx(88)
  e.speed = 1
  jump_guard(e) goto L3
end
SCRIPTED[242] = true

B[243] = function(e)
  ::L0::
  move_wait(e, 48)
  pause(e, 30)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
