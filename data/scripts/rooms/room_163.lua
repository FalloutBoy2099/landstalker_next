-- Behaviours of room 163 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[35] = function(e)
  pause_player()
  move_wait(e, 144)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 64)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 240)
  move_wait(e, 16)
  pause(e, 0)
end

B[79] = function(e)
  e.falls = true; e.fall_v = 0
  pause(e, 20)
  move_down_relative(e, 32)
  pause(e, 0)
end
