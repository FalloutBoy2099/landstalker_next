-- Behaviours of room 82 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[585] = function(e)
  wait_flag(e, 124, true)
  show_when_clear(e)
  e.speed = 1
  ::L5::
  move_down_until_collision(e)
  pause(e, 20)
  move_up_to(e, 80)
  jump_guard(e) goto L5
end
SCRIPTED[585] = true
WAITS_FIRST[585] = true

B[586] = function(e)
  wait_trigger(e, 95)
  set_flag(124)
  sfx(116)
  pause(e, 0)
end
