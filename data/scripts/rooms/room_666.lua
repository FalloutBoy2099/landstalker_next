-- Behaviours of room 666 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[906] = function(e)
  wait_flag(e, 9, true)
  ::L3::
  move_until_collision(e)
  turn_wait(e, math.random(0, 3))
  sfx(69)
  jump_guard(e) goto L3
end
SCRIPTED[906] = true
WAITS_FIRST[906] = true

B[907] = function(e)
  wait_flag(e, 8, true)
  wait_trigger(e, 139)
  pause_player()
  pause(e, 60)
  sfx(87)
  set_flag(9)
  ::L14::
  cutscene(e, 0x100 + 87)
  pause(e, 5)
  jump_guard(e) goto L14
end
SCRIPTED[907] = true
WAITS_FIRST[907] = true

B[908] = function(e)
  e.visible = false; e.base_visible = false
  wait_flag(e, 9, true)
  pause(e, 200)
  pause(e, 200)
  pause(e, 200)
  pause(e, 200)
  pause(e, 200)
  pause(e, 200)
  pause(e, 200)
  pause(e, 200)
  pause(e, 200)
  sfx(116)
  pause_player()
  pause(e, 30)
  cutscene(e, 0x100 + 88)
  pause(e, 0)
end
SCRIPTED[908] = true

-- story actions
story(344, {
  story_flag(10, true),
  say(294),
  say(292),
})
