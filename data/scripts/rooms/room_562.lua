-- Behaviours of room 562 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[438] = function(e)
  e.visible = false; e.base_visible = false
  wait_trigger(e, 82)
  pause_player()
  cutscene(e, 187)
  set_flag(274)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[438] = true

-- story actions
story(187, {
  say(173),
})
