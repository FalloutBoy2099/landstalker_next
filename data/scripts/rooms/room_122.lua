-- Behaviours of room 122 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[856] = function(e)
  ::L0::
  move_wait(e, 48)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 48)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 48)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 48)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[857] = function(e)
  ::L0::
  move_wait(e, 40)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[859] = function(e)
  pause(e, 20)
  e.visible = false; e.base_visible = false; do return end
end
