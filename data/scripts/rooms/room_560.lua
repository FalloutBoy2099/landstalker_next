-- Behaviours of room 560 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[437] = function(e)
  e.visible = false; e.base_visible = false
  wait_trigger(e, 81)
  pause_player()
  rotate_player(0)
  cutscene(e, 186)
  set_flag(273)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[437] = true

B[444] = function(e)
  ::L0::
  wait_flag(e, 288, false)
  wait_trigger(e, 86)
  pause_player()
  pause(e, 32)
  cutscene(e, 194)
  wait_trigger(e, 87)
  jump_guard(e) goto L0
end
SCRIPTED[444] = true
WAITS_FIRST[444] = true

B[445] = function(e)
  wait_flag(e, 288, false)
  wait_flag(e, 288, true)
  pause_player()
  move_to(e, 47, 22)
  pause(e, 30)
  cutscene(e, 195)
  turn(e, 3)
  move_noclip(e, 80)
  rotate_player(3)
  turn_wait(e, 1)
  cutscene(e, 196)
  wait_trigger(e, 88)
  e.speed = 2
  turn(e, 3)
  move_noclip(e, 72)
  set_flag(289)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[445] = true
WAITS_FIRST[445] = true

B[446] = function(e)
  wait_flag(e, 289, true)
  move_to(e, 33, 22)
  wait_trigger(e, 89)
  cutscene(e, 197)
  turn(e, 2)
  move_noclip(e, 48)
  turn(e, 3)
  move_noclip(e, 24)
  turn(e, 2)
  move_noclip(e, 32)
  set_flag(290)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[446] = true
WAITS_FIRST[446] = true

B[447] = function(e)
  wait_flag(e, 290, true)
  move_to(e, 30, 32)
  wait_trigger(e, 90)
  turn(e, 3)
  move_noclip(e, 48)
  turn(e, 2)
  move_noclip(e, 56)
  turn(e, 1)
  move_noclip(e, 24)
  turn(e, 2)
  move_noclip(e, 32)
  turn(e, 3)
  move_noclip(e, 24)
  turn(e, 2)
  move_noclip(e, 32)
  turn(e, 3)
  move_noclip(e, 8)
  turn(e, 2)
  move_noclip(e, 24)
  turn(e, 3)
  move_noclip(e, 16)
  turn_wait(e, 1)
  set_flag(291)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[447] = true
WAITS_FIRST[447] = true

B[448] = function(e)
  wait_flag(e, 291, true)
  move_to(e, 21, 50)
  pause(e, 0)
end
SCRIPTED[448] = true
WAITS_FIRST[448] = true

B[449] = function(e)
  wait_flag(e, 9, true)
  pause_player()
  move_noclip(e, 24)
  turn(e, 0)
  move_noclip(e, 48)
  set_flag(293)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[449] = true
WAITS_FIRST[449] = true

B[870] = function(e)
  wait_flag(e, 288, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[870] = true
WAITS_FIRST[870] = true

-- story actions
story(186, {
  say(172),
})

story(194, {
  run(346),
  say(165),
  if_yes({
      hide_entity(-1),
    }, {
      friday_does("leave"),
    }),
})

story(195, {
  say(166),
})

story(196, {
  say(167),
  friday_does("leave"),
})

story(197, {
  say(168),
})
