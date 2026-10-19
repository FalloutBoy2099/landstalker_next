-- Behaviours of room 533 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[646] = function(e)
  wait_trigger(e, 112)
  cutscene(e, 0x100 + 10)
  pause(e, 0)
end
SCRIPTED[646] = true

-- story actions
story(266, {
  if_flag(33, {}, {
      say(248),
    }),
})
