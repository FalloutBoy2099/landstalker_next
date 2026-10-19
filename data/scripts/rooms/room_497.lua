-- Behaviours of room 497 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[513] = function(e)
  pause_player()
  pause(e, 240)
  cutscene(e, 211)
  pause(e, 0)
end
SCRIPTED[513] = true
