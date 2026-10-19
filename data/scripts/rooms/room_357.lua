-- Behaviours of room 357 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[812] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(403)
  pause(e, 0)
end

B[813] = function(e)
  wait_flag(e, 403, true)
  move_down_until_collision(e)
  pause(e, 0)
end
SCRIPTED[813] = true
WAITS_FIRST[813] = true
