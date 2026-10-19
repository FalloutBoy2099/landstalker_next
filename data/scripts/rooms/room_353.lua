-- Behaviours of room 353 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[902] = function(e)
  show_when_clear(e)
  sfx(88)
  pause(e, 0)
end
