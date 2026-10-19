-- Behaviours of room 292 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[325] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  pause(e, 70)
  move_down_until_collision(e)
end

B[326] = function(e)
  wait_flag(e, 8, true)
  move_up_relative(e, 48)
  pause(e, 80)
  move_down_relative(e, 48)
end
SCRIPTED[326] = true
WAITS_FIRST[326] = true

B[327] = function(e)
  wait_flag(e, 9, true)
  pause(e, 0)
end
SCRIPTED[327] = true
WAITS_FIRST[327] = true

B[328] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(9)
  pause(e, 0)
  move_down_until_collision(e)
end
