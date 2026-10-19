-- Behaviours of room 250 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[782] = function(e)
  wait_trigger(e, 2)
  set_flag(105)
  pause(e, 0)
end
