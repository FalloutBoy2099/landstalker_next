-- Behaviours of room 130 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[846] = function(e)
  pause(e, 20)
  show_when_clear(e)
  ::L3::
  sfx(84)
  move_down_until_collision(e)
  sfx(88)
  do return end  -- unknown command 4D
end
