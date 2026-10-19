-- Behaviours of room 57 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[395] = function(e)
  wait_trigger(e, 37)
  pause_player()
  move_noclip(e, 80)
  cutscene(e, 156)
  playback(19)
  turn(e, 0)
  e.speed = 2
  move_noclip(e, 40)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[395] = true

-- story actions
story(156, {
  say(138),
})
