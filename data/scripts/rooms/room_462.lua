-- Behaviours of room 462 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[848] = function(e)
  cutscene(e, 0x100 + 74)
  wait_trigger(e, 128)
  cutscene(e, 0x100 + 75)
  set_flag(452)
  pause(e, 0)
end
SCRIPTED[848] = true

-- story actions
story(330, {
  run(346),
  say(313),
})

story(331, {
  say(314),
  friday_does("leave"),
})
