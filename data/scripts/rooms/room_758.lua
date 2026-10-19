-- Behaviours of room 758 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[373] = function(e)
  wait_trigger(e, 61)
  pause_player()
  move_noclip(e, 36)
  cutscene(e, 139)
  pause(e, 2)
  cutscene(e, 140)
  pause(e, 2)
  cutscene(e, 141)
  pause(e, 2)
  cutscene(e, 142)
  pause(e, 2)
  pause(e, 0)
end
SCRIPTED[373] = true

B[374] = function(e)
  wait_flag(e, 184, false)
  wait_flag(e, 184, true)
  pause_player()
  cutscene(e, 143)
  rotate_player(2)
  move_noclip(e, 40)
  cutscene(e, 144)
  turn(e, 2)
  e.speed = 4
  move_noclip(e, 4)
  move_noclip(e, 28)
  e.visible = false; e.base_visible = false; do return end
  pause(e, 0)
end
SCRIPTED[374] = true
WAITS_FIRST[374] = true

-- story actions
story(139, {
  play_sound(81),
})

story(144, {
  say(128),
})
