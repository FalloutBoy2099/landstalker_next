-- Behaviours of room 592 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[121] = function(e)
  wait_trigger(e, 9)
  pause_player()
  move_noclip(e, 48)
  turn(e, e.orientation + 3)
  move_noclip(e, 12)
  turn(e, e.orientation + 1)
  move_noclip(e, 16)
  cutscene(e, 3)
  set_flag(18)
  turn(e, e.orientation + 2)
  move_noclip(e, 48)
  turn(e, e.orientation + 3)
  move_noclip(e, 60)
  turn(e, e.orientation + 1)
  move_noclip(e, 88)
  turn(e, e.orientation + 3)
  move_noclip(e, 24)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[121] = true

-- story actions
story(3, {
  say(13),
})
