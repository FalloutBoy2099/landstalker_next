-- Behaviours of room 571 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[829] = function(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  show_when_clear(e)
  pause(e, 0)
end
