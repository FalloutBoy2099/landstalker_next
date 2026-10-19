-- Behaviours of room 35 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[429] = function(e)
  tile_swap(1)
  playback(42)
  pause(e, 12)
  cutscene(e, 178)
  set_flag(261)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[429] = true

B[436] = function(e)
  wait_flag(e, 261, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[436] = true
WAITS_FIRST[436] = true

-- story actions
story(178, {
  say(157),
})
