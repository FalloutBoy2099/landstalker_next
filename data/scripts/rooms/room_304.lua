-- Behaviours of room 304 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[655] = function(e)
  pause(e, 255)
  pause(e, 255)
  pause(e, 255)
  pause(e, 255)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  cutscene(e, 0x100 + 12)
  pause(e, 0)
end
SCRIPTED[655] = true

-- story actions
story(268, {
  make_hostile(0, false),
})
