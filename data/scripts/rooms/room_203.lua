-- Behaviours of room 203 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[228] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_trigger(e, 2)
  set_flag(90)
  cutscene(e, 102)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[228] = true

-- story actions
story(102, {
  play_sound(87),
  pause_frames(5),
  swap_tiles(8),
})
