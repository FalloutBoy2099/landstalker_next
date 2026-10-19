-- Behaviours of room 195 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[222] = function(e)
  ::L0::
  e.falls = true; e.fall_v = 0
  move_wait(e, 144)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 144)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 176)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 32)
  pause(e, 10)
  do return end  -- unknown command 4D
end

B[223] = function(e)
  pause(e, 124)
  ::L2::
  show_when_clear(e)
  e.falls = true; e.fall_v = 0
  move_wait(e, 128)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 112)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 144)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 16)
  pause(e, 10)
  e.visible = false
  pause(e, 96)
  do return end  -- unknown command 4D
end

B[224] = function(e)
  pause(e, 240)
  pause(e, 132)
  ::L4::
  show_when_clear(e)
  e.falls = true; e.fall_v = 0
  move_wait(e, 128)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 112)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 144)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 16)
  pause(e, 10)
  e.visible = false
  pause(e, 96)
  do return end  -- unknown command 4D
end

B[225] = function(e)
  pause(e, 124)
  pause(e, 124)
  show_when_clear(e)
  ::L5::
  e.falls = true; e.fall_v = 0
  move_wait(e, 144)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 144)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 176)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 32)
  pause(e, 10)
  do return end  -- unknown command 4D
end
