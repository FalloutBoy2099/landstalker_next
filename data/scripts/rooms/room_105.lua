-- Behaviours of room 105 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[423] = function(e)
  wait_trigger(e, 77)
  set_flag(259)
  pause_player()
  cutscene(e, 171)
  move_to(e, 26, 31)
  e.falls = true; e.fall_v = 0
  move_noclip(e, 64)
  cutscene(e, 172)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[423] = true

B[424] = function(e)
  wait_trigger(e, 77)
  pause(e, 32)
  move_to(e, 26, 31)
  move_noclip(e, 64)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[424] = true

-- story actions
story(172, {
  say(152),
})
