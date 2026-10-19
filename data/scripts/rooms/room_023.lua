-- Behaviours of room 23 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[55] = function(e)
  ::L0::
  e.falls = true; e.fall_v = 0
  pause(e, 16)
  sfx(88)
  e.speed = 2
  move_wait(e, 224)
  pause(e, 10)
  do return end  -- unknown command 4D
end

B[101] = function(e)
  move_up_relative(e, 16)
  pause(e, 0)
end

B[102] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  do return end  -- unknown command 48
end

B[103] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  do return end  -- unknown command 48
end

B[104] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  do return end  -- unknown command 48
end

B[105] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  do return end  -- unknown command 48
end
