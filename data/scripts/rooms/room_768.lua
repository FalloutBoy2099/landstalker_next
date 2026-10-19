-- Behaviours of room 768 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[185] = function(e)
  pause(e, 10)
  ::L2::
  move_down_until_collision(e)
  pause(e, 20)
  do return end  -- unknown command 4D
end

B[193] = function(e)
  pause(e, 250)
  ::L2::
  e.falls = true; e.fall_v = 0
  pause(e, 20)
  do return end  -- unknown command 4D
end

B[194] = function(e)
  pause(e, 100)
  ::L2::
  move_down_until_collision(e)
  pause(e, 30)
  do return end  -- unknown command 4D
end
