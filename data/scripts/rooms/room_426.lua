-- Behaviours of room 426 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[116] = function(e)
  pause_player()
  move_down_until_collision(e)
  cutscene(e, 0)
  e.visible = false; e.base_visible = false; do return end
  pause(e, 0)
end
SCRIPTED[116] = true

B[117] = function(e)
  move_wait(e, 80)
  cutscene(e, 18)
  e.speed = 2
  move_until_collision(e)
  cutscene(e, 19)
  turn(e, 0)
  move_until_collision(e)
end
SCRIPTED[117] = true

-- story actions
story(18, {
  say(1),
})

story(19, {
  say(2),
})
