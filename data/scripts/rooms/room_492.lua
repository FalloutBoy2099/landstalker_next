-- Behaviours of room 492 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[629] = function(e)
  wait_flag(e, 8, true)
  cutscene(e, 253)
  move_noclip(e, 80)
  turn(e, e.orientation + 2)
  e.hostile = true; e.dormant = false
  sfx(47)
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[629] = true
WAITS_FIRST[629] = true

B[630] = function(e)
  wait_flag(e, 312, false)
  cutscene(e, 251)
  wait_trigger(e, 110)
  pause_player()
  pause(e, 2)
  cutscene(e, 252)
  pause(e, 0)
  cutscene(e, 254)
  set_flag(312)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[630] = true
WAITS_FIRST[630] = true

-- story actions
story(251, {
  make_hostile(3, false),
})

story(252, {
  say(226),
  make_hostile(3, false),
})
