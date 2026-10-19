-- Behaviours of room 464 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[365] = function(e)
  wait_flag(e, 8, true)
  show_when_clear(e)
  ::L4::
  move_down_until_collision(e)
  pause(e, 20)
  pause(e, 20)
  jump_guard(e) goto L4
end
SCRIPTED[365] = true
WAITS_FIRST[365] = true
