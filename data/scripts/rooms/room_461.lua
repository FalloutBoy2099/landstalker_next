-- Behaviours of room 461 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[512] = function(e)
  wait_flag(e, 8, true)
  ::L3::
  move_down_until_collision(e)
  pause(e, 20)
  pause(e, 20)
  jump_guard(e) goto L3
end
SCRIPTED[512] = true
WAITS_FIRST[512] = true

B[847] = function(e)
  wait_flag(e, 451, false)
  pause(e, 2)
  pause_player()
  cutscene(e, 0x100 + 73)
  set_flag(451)
  pause(e, 0)
end
SCRIPTED[847] = true
WAITS_FIRST[847] = true

-- story actions
story(329, {
  run(346),
  say(312),
  friday_does("leave"),
})
