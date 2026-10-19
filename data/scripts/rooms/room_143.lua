-- Behaviours of room 143 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[649] = function(e)
  ::L0::
  repeat_begin(e, 2)
  ::L2::
  tile_swap(1)
  pause(e, 30)
  tile_swap(2)
  pause(e, 30)
  tile_swap(3)
  pause(e, 30)
  if repeat_next(e) then goto L2 end
  tile_swap(1)
  pause(e, 255)
  jump_guard(e) goto L0
end

B[683] = function(e)
  pause_player()
  pause(e, 30)
  cutscene(e, 0x100 + 33)
  pause(e, 210)
  cutscene(e, 0x100 + 35)
  pause(e, 60)
  cutscene(e, 0x100 + 34)
  pause(e, 210)
  cutscene(e, 0x100 + 24)
  pause(e, 240)
  pause(e, 60)
  pause(e, 120)
  cutscene(e, 0x100 + 19)
  pause(e, 0)
  pause(e, 240)
  pause(e, 240)
  pause(e, 240)
  sfx(253)
  pause(e, 240)
  cutscene(e, 0x100 + 19)
  pause(e, 0)
end
SCRIPTED[683] = true

-- story actions
story(275, {
  effect("fade_out"),
  go_to(148, 32, 38, 2),
  lock_controls(false),
})

story(280, {
  intro_caption(3),
})

story(289, {
  intro_caption(11),
})

story(290, {
  intro_caption(12),
})

story(291, {
  say(281, 180),
})
