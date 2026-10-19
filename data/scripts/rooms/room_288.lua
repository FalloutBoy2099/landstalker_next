-- Behaviours of room 288 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[873] = function(e)
  wait_trigger(e, 2)
  set_flag(419)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end
