-- Behaviours of room 782 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[386] = function(e)
  wait_trigger(e, 2)
  wait_trigger(e, 66)
  move_to(e, 19, 19)
  move_noclip(e, 80)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[386] = true

B[387] = function(e)
  cutscene(e, 149)
  wait_trigger(e, 67)
  pause_player()
  cutscene(e, 150)
  pause(e, 0)
  set_flag(189)
  do return end  -- unknown command 5B
end
SCRIPTED[387] = true

B[388] = function(e)
  wait_flag(e, 189, true)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end
SCRIPTED[388] = true
WAITS_FIRST[388] = true

B[389] = function(e)
  sfx(88)
  wait_flag(e, 189, true)
  move_up_to(e, 48)
  pause(e, 0)
end
SCRIPTED[389] = true

-- story actions
story(149, {
  make_hostile(8, false),
})

story(150, {
  make_hostile(8, false),
  show_entity(8, true),
})
