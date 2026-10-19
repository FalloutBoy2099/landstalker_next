-- Behaviours of room 454 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[198] = function(e)
  wait_flag(e, 39, false)
  wait_trigger(e, 27)
  pause_player()
  turn(e, 0)
  move_noclip(e, 19)
  turn_wait(e, 1)
  cutscene(e, 40)
  wait_flag(e, 39, true)
  turn(e, 2)
  move_noclip(e, 19)
  turn(e, 1)
  pause(e, 0)
end
SCRIPTED[198] = true
WAITS_FIRST[198] = true

B[199] = function(e)
  wait_flag(e, 39, false)
  wait_trigger(e, 27)
  turn(e, 2)
  move_noclip(e, 18)
  turn_wait(e, 1)
  pause(e, 1)
  wait_flag(e, 39, true)
  turn(e, 0)
  move_noclip(e, 18)
  turn(e, 1)
  pause(e, 0)
end
SCRIPTED[199] = true
WAITS_FIRST[199] = true

-- story actions
story(40, {
  say(54),
  if_flag(35, {
      play_sound(88),
      swap_tiles(8),
      story_flag(39, true),
    }, {
      lock_controls(false),
    }),
})

story(41, {
  story_flag(40, true),
  say(50),
})

story(57, {
  say(68),
  story_flag(8, true),
})
