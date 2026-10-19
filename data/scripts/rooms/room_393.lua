-- Behaviours of room 393 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[910] = function(e)
  ::L0::
  move_down_until_collision(e)
  sfx(88)
  do return end  -- unknown command 4D
end
