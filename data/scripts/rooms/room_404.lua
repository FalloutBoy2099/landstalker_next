-- Behaviours of room 404 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[834] = function(e)
  wait_trigger(e, 2)
  show_when_clear(e)
  tile_swap(1)
  set_flag(453)
  pause(e, 0)
end
