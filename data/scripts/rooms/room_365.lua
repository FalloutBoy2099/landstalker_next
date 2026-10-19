-- Behaviours of room 365 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[822] = function(e)
  wait_flag(e, 405, true)
  move_down_relative(e, 32)
  pause(e, 0)
end
SCRIPTED[822] = true
WAITS_FIRST[822] = true

B[823] = function(e)
  wait_trigger(e, 2)
  set_flag(405)
  pause(e, 0)
end
