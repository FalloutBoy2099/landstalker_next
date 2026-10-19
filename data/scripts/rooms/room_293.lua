-- Behaviours of room 293 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[794] = function(e)
  pause_player()
  pause(e, 30)
  turn_wait(e, 3)
  pause(e, 10)
  cutscene(e, 0x100 + 64)
  set_flag(350)
  move_wait(e, 16)
  move_wait(e, 16)
  move_wait(e, 16)
  pause(e, 0)
end
SCRIPTED[794] = true

B[795] = function(e)
  pause(e, 30)
  turn_wait(e, 3)
  pause(e, 12)
  move_wait(e, 16)
  move_wait(e, 32)
  pause(e, 0)
end

-- story actions
story(320, {
  say(305),
})
