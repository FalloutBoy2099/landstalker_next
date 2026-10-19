-- Behaviours of room 91 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[918] = function(e)
  wait_trigger(e, 2)
  set_flag(98)
  tile_swap(1)
  sfx(88)
  pause(e, 0)
end
