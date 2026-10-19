-- Behaviours of room 180 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[120] = function(e)
  wait_trigger(e, 8)
  cutscene(e, 2)
  set_flag(17)
  pause(e, 0)
end
SCRIPTED[120] = true

B[169] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(37)
  pause(e, 0)
end

-- story actions
story(2, {
  say(10),
})

story(8, {
  friday_does("appear"),
  say(11),
  say(12),
  friday_does("leave"),
})
