-- Behaviours of room 722 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[519] = function(e)
  wait_trigger(e, 99)
  cutscene(e, 215)
  move_noclip(e, 16)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[519] = true

B[520] = function(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  set_flag(308)
  pause(e, 0)
end

-- story actions
story(215, {
  say(196),
})
