-- Behaviours of room 676 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[216] = function(e)
  wait_flag(e, 47, true)
  playback(14)
  turn(e, 1)
  move_noclip(e, 40)
  turn(e, 3)
  pause(e, 1)
  e.speed = 1
  rotate_player(3)
  do return end  -- unknown command 5B
end
SCRIPTED[216] = true
WAITS_FIRST[216] = true

B[332] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  pause_player()
  pause(e, 30)
  cutscene(e, 50)
  cutscene(e, 119)
  clear_flag(8)
  jump_guard(e) goto L0
end
SCRIPTED[332] = true
WAITS_FIRST[332] = true

-- story actions
story(48, {
  say(55),
  if_yes({
      story_flag(47, true),
    }),
})

story(50, {
  make_hostile(0, false),
})
