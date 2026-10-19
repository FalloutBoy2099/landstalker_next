-- Behaviours of room 406 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[765] = function(e)
  wait_flag(e, 346, false)
  wait_flag(e, 344, true)
  tile_swap(1)
  show_when_clear(e)
  wait_trigger(e, 121)
  pause_player()
  pause(e, 3)
  cutscene(e, 0x100 + 60)
  rotate_player(2)
  tile_swap(2)
  move_wait(e, 32)
  e.speed = 2
  move_until_collision(e)
  pause(e, 0)
end
SCRIPTED[765] = true
WAITS_FIRST[765] = true

-- story actions
story(316, {
  run(346),
  say(301),
  friday_does("leave"),
})
