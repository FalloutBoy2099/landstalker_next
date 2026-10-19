-- Behaviours of room 359 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[814] = function(e)
  ::L0::
  repeat_begin(e, 6)
  ::L2::
  pause(e, 5)
  move_down_until_collision(e)
  sfx(88)
  move_up_to(e, 48)
  move_wait(e, 16)
  if repeat_next(e) then goto L2 end
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[836] = function(e)
  ::L0::
  repeat_begin(e, 2)
  ::L2::
  pause(e, 5)
  move_down_until_collision(e)
  sfx(88)
  move_up_to(e, 64)
  move_wait(e, 16)
  if repeat_next(e) then goto L2 end
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
