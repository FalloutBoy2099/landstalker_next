-- Behaviours of room 59 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[276] = function(e)
  playback(20)
  move_noclip(e, 40)
  move_noclip(e, 72)
  e.visible = false; e.base_visible = false; do return end
end

-- story actions
story(129, {
  say(119),
  if_yes({
      story_flag(8, true),
    }),
})
