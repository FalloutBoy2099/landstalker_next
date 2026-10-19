-- Behaviours of room 72 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[411] = function(e)
  pause_player()
  move_noclip(e, 64)
  turn_wait(e, 2)
  cutscene(e, 168)
  e.hostile = true; e.dormant = false
end
SCRIPTED[411] = true

B[412] = function(e)
  move_noclip(e, 62)
  turn(e, 2)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[412] = true

B[413] = function(e)
  pause(e, 80)
  wait_trigger(e, 2)
  set_flag(256)
  e.visible = false; e.base_visible = false; do return end
end

-- story actions
story(168, {
  say(150),
})
