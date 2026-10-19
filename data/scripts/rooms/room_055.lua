-- Behaviours of room 55 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[275] = function(e)
  wait_trigger(e, 37)
  pause_player()
  move_noclip(e, 88)
  cutscene(e, 76)
  playback(19)
  pause(e, 8)
  e.speed = 2
  turn(e, 0)
  move_noclip(e, 44)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[275] = true

-- story actions
story(76, {
  say(87),
})
