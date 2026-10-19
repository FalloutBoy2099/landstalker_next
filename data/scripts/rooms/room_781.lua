-- Behaviours of room 781 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[392] = function(e)
  wait_trigger(e, 69)
  set_flag(8)
  pause(e, 0)
end

B[393] = function(e)
  ::L0::
  wait_trigger(e, 70)
  cutscene(e, 155)
  wait_trigger(e, 71)
  jump_guard(e) goto L0
end
SCRIPTED[393] = true

B[394] = function(e)
  wait_flag(e, 8, true)
  set_flag(190)
  cutscene(e, 154)
  pause(e, 0)
end
SCRIPTED[394] = true
WAITS_FIRST[394] = true

-- story actions
story(155, {
  say(137),
  if_yes({
      story_flag(8, true),
    }),
})
