-- Behaviours of room 308 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[792] = function(e)
  pause(e, 15)
  pause_player()
  move_wait(e, 128)
  pause(e, 6)
  turn(e, 0)
  move_wait(e, 128)
  e.visible = false; e.base_visible = false; do return end
end

B[793] = function(e)
  pause(e, 15)
  pause_player()
  move_wait(e, 128)
  pause(e, 3)
  cutscene(e, 0x100 + 63)
  pause(e, 3)
  turn(e, 0)
  move_wait(e, 128)
  set_flag(349)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[793] = true

-- story actions
story(319, {
  say(304),
})
