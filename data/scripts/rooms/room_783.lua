-- Behaviours of room 783 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[380] = function(e)
  wait_trigger(e, 65)
  turn(e, 1)
  move_noclip(e, 8)
  turn(e, 2)
  move_noclip(e, 16)
  turn(e, 1)
  move_noclip(e, 12)
  turn(e, 0)
  move_wait(e, 8)
  move_wait(e, 8)
  move_wait(e, 8)
  move_wait(e, 8)
  move_wait(e, 8)
  pause(e, 8)
  set_flag(188)
  e.visible = false; e.base_visible = false; do return end
end
