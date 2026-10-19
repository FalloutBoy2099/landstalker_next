-- Behaviours of room 126 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[725] = function(e)
  wait_trigger(e, 2)
  wait_trigger(e, 1)
  set_flag(9)
  pause(e, 0)
end

B[726] = function(e)
  wait_flag(e, 9, true)
  show_when_clear(e)
  ::L4::
  pause(e, 16)
  move_wait(e, 96)
  turn_wait(e, 0)
  move_wait(e, 80)
  turn_wait(e, e.orientation + 2)
  pause(e, 32)
  move_wait(e, 80)
  turn_wait(e, 3)
  move_wait(e, 96)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L4
end
SCRIPTED[726] = true
WAITS_FIRST[726] = true

B[840] = function(e)
  wait_flag(e, 9, true)
  show_when_clear(e)
  e.speed = 1
  ::L5::
  move_wait(e, 144)
  turn_wait(e, 3)
  move_wait(e, 32)
  turn_wait(e, 0)
  move_wait(e, 48)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 48)
  turn_wait(e, 1)
  move_wait(e, 32)
  turn_wait(e, 0)
  move_wait(e, 144)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L5
end
SCRIPTED[840] = true
WAITS_FIRST[840] = true

B[841] = function(e)
  wait_flag(e, 9, true)
  show_when_clear(e)
  ::L4::
  move_wait(e, 48)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 64)
  turn_wait(e, e.orientation + 1)
  jump_guard(e) goto L4
end
SCRIPTED[841] = true
WAITS_FIRST[841] = true

B[892] = function(e)
  ::L0::
  move_wait(e, 64)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 1)
  jump_guard(e) goto L0
end

B[893] = function(e)
  ::L0::
  move_wait(e, 48)
  turn_wait(e, 0)
  move_wait(e, 48)
  turn_wait(e, 1)
  move_wait(e, 176)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 176)
  turn_wait(e, 2)
  move_wait(e, 48)
  turn_wait(e, 1)
  move_wait(e, 48)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
