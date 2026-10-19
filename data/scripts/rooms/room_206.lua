-- Behaviours of room 206 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[452] = function(e)
  ::L0::
  e.falls = true; e.fall_v = 0
  pause(e, 11)
  sfx(88)
  move_wait(e, 176)
  pause(e, 10)
  do return end  -- unknown command 4D
end

B[453] = function(e)
  pause(e, 90)
  ::L2::
  e.falls = true; e.fall_v = 0
  pause(e, 12)
  sfx(88)
  move_wait(e, 176)
  pause(e, 10)
  do return end  -- unknown command 4D
end
