-- Behaviours of room 387 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[761] = function(e)
  wait_flag(e, 334, false)
  wait_trigger(e, 120)
  cutscene(e, 0x100 + 58)
  set_flag(334)
  pause(e, 0)
end
SCRIPTED[761] = true
WAITS_FIRST[761] = true

-- story actions
story(314, {
  run(346),
  say(303),
  friday_does("leave"),
})
