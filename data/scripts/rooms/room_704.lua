-- Behaviours of room 704 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[744] = function(e)
  wait_flag(e, 330, false)
  wait_flag(e, 330, true)
  pause_player()
  pause(e, 60)
  cutscene(e, 0x100 + 45)
  pause(e, 30)
  move_to(e, 30, 30)
  move_up_to(e, 32)
  cutscene(e, 0x100 + 44)
  move_down_until_collision(e)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[744] = true
WAITS_FIRST[744] = true

-- story actions
story(300, {
  say(289),
})

story(301, {
  play_sound(12),
})
