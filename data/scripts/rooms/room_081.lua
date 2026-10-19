-- Behaviours of room 81 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[371] = function(e)
  ::L0::
  move_wait(e, 14)
  turn_wait(e, e.orientation + 1)
  jump_guard(e) goto L0
end

B[406] = function(e)
  wait_flag(e, 153, false)
  wait_trigger(e, 73)
  pause_player()
  pause(e, 3)
  cutscene(e, 164)
  set_flag(153)
  pause(e, 0)
end
SCRIPTED[406] = true
WAITS_FIRST[406] = true

-- story actions
story(164, {
  say(146),
})
