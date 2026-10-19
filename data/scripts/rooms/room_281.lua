-- Behaviours of room 281 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[743] = function(e)
  wait_trigger(e, 115)
  pause_player()
  turn(e, 3)
  move_noclip(e, 24)
  rotate_player(1)
  cutscene(e, 0x100 + 42)
  playback(0)
  move_until_collision(e)
  turn(e, 2)
  move_wait(e, 8)
  turn(e, 3)
  move_wait(e, 8)
  turn(e, 0)
  move_wait(e, 8)
  move_wait(e, 8)
  set_flag(329)
  move_to(e, 54, 5)
  pause(e, 120)
  turn(e, 2)
  move_to(e, 26, 26)
  move_until_collision(e)
  turn(e, 1)
  move_until_collision(e)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[743] = true

-- story actions
story(298, {
  say(252),
})
