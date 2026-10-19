-- Behaviours of room 140 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[664] = function(e)
  playback(48)
  pause(e, 240)
  pause(e, 180)
  cutscene(e, 0x100 + 25)
  pause(e, 240)
  cutscene(e, 0x100 + 27)
  wait_trigger(e, 113)
  cutscene(e, 0x100 + 14)
  pause(e, 0)
end
SCRIPTED[664] = true

B[665] = function(e)
  pause(e, 104)
  ::L2::
  move_up_to(e, 128)
  turn_wait(e, 1)
  move_wait(e, 64)
  move_down_to(e, 64)
  turn_wait(e, 3)
  move_wait(e, 64)
  jump_guard(e) goto L2
end

B[666] = function(e)
  ::L0::
  move_wait(e, 64)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 112)
  turn_wait(e, e.orientation + 1)
  jump_guard(e) goto L0
end

B[667] = function(e)
  pause(e, 32)
  ::L2::
  pause(e, 64)
  move_wait(e, 112)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L2
end

B[668] = function(e)
  ::L0::
  move_up_to(e, 160)
  move_down_to(e, 16)
  jump_guard(e) goto L0
end

-- story actions
story(270, {
  go_to(141, 44, 44, 3),
  lock_controls(true),
})

story(281, {
  intro_caption(4),
})

story(283, {
  intro_caption(6),
})
