-- Behaviours of room 656 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[336] = function(e)
  e.visible = false; e.base_visible = false
  wait_flag(e, 178, false)
  wait_flag(e, 8, true)
  pause_player()
  move_to(e, 16, 20)
  pause(e, 2)
  set_flag(178)
  cutscene(e, 120)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[336] = true

-- story actions
story(120, {
  say(118),
  if_yes({}, {
      story_flag(178, false),
    }),
})
