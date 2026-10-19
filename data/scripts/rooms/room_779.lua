-- Behaviours of room 779 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[379] = function(e)
  tile_swap(2)
  wait_trigger(e, 64)
  pause_player()
  turn(e, 3)
  move_noclip(e, 16)
  set_flag(187)
  tile_swap(1)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
