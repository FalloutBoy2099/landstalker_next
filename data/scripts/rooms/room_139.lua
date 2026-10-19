-- Behaviours of room 139 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[663] = function(e)
  playback(47)
  pause(e, 200)
  show_when_clear(e)
  sfx(89)
  pause(e, 60)
  rotate_player(1)
  e.falls = true; e.fall_v = 0
  move_until_collision(e)
  pause(e, 50)
  cutscene(e, 0x100 + 13)
  pause(e, 0)
end
SCRIPTED[663] = true

B[719] = function(e)
  pause(e, 240)
  pause(e, 120)
  cutscene(e, 0x100 + 23)
  pause(e, 0)
end
SCRIPTED[719] = true

-- story actions
story(269, {
  go_to(140, 32, 20, 2),
  lock_controls(true),
})

story(279, {
  intro_caption(2),
})
