-- Behaviours of room 762 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[375] = function(e)
  wait_trigger(e, 62)
  pause_player()
  pause(e, 20)
  cutscene(e, 145)
  turn(e, 2)
  set_flag(185)
  move_noclip(e, 104)
  turn_wait(e, 0)
  pause(e, 4)
  turn(e, 3)
  move_noclip(e, 28)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[375] = true

-- story actions
story(145, {
  say(129),
})
