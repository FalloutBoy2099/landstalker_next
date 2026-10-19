-- Behaviours of room 633 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[182] = function(e)
  wait_flag(e, 41, false)
  wait_trigger(e, 29)
  set_flag(41)
  pause_player()
  cutscene(e, 45)
  move_to(e, 35, 51)
  move_noclip(e, 48)
  turn_wait(e, 3)
  cutscene(e, 46)
  turn(e, 0)
  move_noclip(e, 80)
  turn(e, 3)
  move_noclip(e, 48)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[182] = true
WAITS_FIRST[182] = true

B[183] = function(e)
  wait_flag(e, 41, false)
  wait_flag(e, 41, true)
  move_to(e, 35, 51)
  pause(e, 16)
  move_noclip(e, 32)
  pause(e, 1)
  turn(e, 0)
  move_noclip(e, 80)
  turn(e, 3)
  move_noclip(e, 32)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[183] = true
WAITS_FIRST[183] = true

B[266] = function(e)
  clear_flag(26)
  pause(e, 0)
end

-- story actions
story(46, {
  say(45),
})
