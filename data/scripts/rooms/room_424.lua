-- Behaviours of room 424 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[119] = function(e)
  wait_trigger(e, 7)
  pause_player()
  move_noclip(e, 64)
  cutscene(e, 1)
  move_noclip(e, 32)
  set_flag(16)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[119] = true

-- story actions
story(1, {
  say(9),
})
