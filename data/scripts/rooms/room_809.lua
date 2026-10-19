-- Behaviours of room 809 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[647] = function(e)
  wait_trigger(e, 2)
  e.falls = true; e.fall_v = 0
  wait_flag(e, 295, false)
  pause(e, 30)
  cutscene(e, 0x100 + 11)
  set_flag(295)
  pause(e, 0)
end
SCRIPTED[647] = true

-- story actions
story(267, {
  say(251),
})
