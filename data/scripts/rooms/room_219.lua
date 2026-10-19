-- Behaviours of room 219 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[156] = function(e)
  wait_trigger(e, 22)
  pause_player()
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  pause(e, 1)
  move_noclip(e, 1)
  cutscene(e, 30)
end
SCRIPTED[156] = true

-- story actions
story(30, {
  story_flag(3, true),
})
