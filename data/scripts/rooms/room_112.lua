-- Behaviours of room 112 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[888] = function(e)
  move_down_to(e, 96)
  cutscene(e, 0x100 + 83)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[888] = true

B[889] = function(e)
  cutscene(e, 0x100 + 82)
  wait_trigger(e, 137)
  cutscene(e, 0x100 + 85)
  wait_trigger(e, 138)
  pause(e, 60)
  pause_player()
  pause(e, 30)
  cutscene(e, 0x100 + 84)
  pause(e, 0)
end
SCRIPTED[889] = true

-- story actions
story(338, {
  make_hostile(1, false),
})

story(339, {
  play_sound(113),
  effect("red_flash"),
  say(323),
  play_sound(41),
  hide_entity(2),
})

story(340, {
  play_sound(254),
  pause_frames(120),
  play_sound(121),
  go_to(113, 28, 25, 2),
  play_sound(45),
  run(346),
  friday_does("hover"),
  say(324, 60),
  say(325, 180),
  friday_does("hover"),
  say(326, 60),
  say(327, 60),
  say(328, 120),
  say(329, 180),
  say(330, 180),
  friday_does("hover"),
  play_sound(118),
  pause_frames(52),
  say(331, 60),
  say(332, 60),
  say(333, 120),
  say(334, 180),
  say(335, 180),
  say(336, 120),
  say(337, 60),
  play_sound(65533),
  effect("fade_out"),
  pause_frames(120),
  show_credits(),
})

story(341, {
  hide_entity(1),
})
