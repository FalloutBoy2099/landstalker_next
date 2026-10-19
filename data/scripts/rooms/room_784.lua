-- Behaviours of room 784 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[381] = function(e)
  pause_player()
  pause(e, 5)
  cutscene(e, 146)
  move_noclip(e, 96)
  cutscene(e, 147)
  set_flag(8)
  pause(e, 120)
  move_noclip(e, 16)
  cutscene(e, 148)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[381] = true

B[382] = function(e)
  wait_flag(e, 8, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[382] = true
WAITS_FIRST[382] = true

B[390] = function(e)
  cutscene(e, 151)
  wait_trigger(e, 68)
  pause_player()
  pause(e, 30)
  cutscene(e, 152)
  set_flag(191)
  cutscene(e, 153)
  pause(e, 0)
end
SCRIPTED[390] = true

B[391] = function(e)
  sfx(88)
  wait_flag(e, 191, true)
  move_up_to(e, 48)
  pause(e, 0)
end
SCRIPTED[391] = true

-- story actions
story(146, {
  say(130),
})

story(147, {
  say(131),
})

story(148, {
  say(132),
  run(346),
  say(133),
  friday_does("leave"),
})

story(151, {
  make_hostile(3, false),
})

story(152, {
  say(134),
  run(346),
  say(135),
  say(136),
  friday_does("leave"),
})

story(153, {
  run(151),
})
