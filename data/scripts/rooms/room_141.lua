-- Behaviours of room 141 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[669] = function(e)
  playback(49)
  pause(e, 60)
  cutscene(e, 0x100 + 29)
  pause(e, 210)
  cutscene(e, 0x100 + 30)
  wait_trigger(e, 113)
  cutscene(e, 0x100 + 15)
  pause(e, 0)
end
SCRIPTED[669] = true

B[676] = function(e)
  wait_trigger(e, 1)
  pause(e, 30)
  e.visible = false; e.base_visible = false; do return end
end

-- story actions
story(271, {
  go_to(142, 47, 29, 3),
  play_sound(14),
  lock_controls(true),
})

story(285, {
  intro_caption(7),
})

story(286, {
  intro_caption(8),
})
