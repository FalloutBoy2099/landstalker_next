-- Behaviours of room 44 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[433] = function(e)
  wait_flag(e, 263, false)
  wait_trigger(e, 79)
  pause_player()
  tile_swap(1)
  move_to(e, 15, 12)
  move_noclip(e, 24)
  pause(e, 24)
  turn_wait(e, 0)
  e.speed = 1
  move_noclip(e, 32)
  turn(e, 2)
  pause(e, 60)
  move_noclip(e, 48)
  cutscene(e, 183)
  set_flag(263)
  pause(e, 0)
end
SCRIPTED[433] = true
WAITS_FIRST[433] = true

-- story actions
story(183, {
  say(162),
})
