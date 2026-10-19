-- Behaviours of room 242 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[525] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  set_flag(311)
  e.visible = false; e.base_visible = false; do return end
end

B[526] = function(e)
  pause(e, 16)
  pause_player()
  cutscene(e, 218)
  move_noclip(e, 32)
  move_noclip(e, 32)
  move_noclip(e, 32)
  cutscene(e, 219)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[526] = true

B[721] = function(e)
  wait_trigger(e, 114)
  cutscene(e, 220)
  tile_swap(1)
  turn(e, 0)
  move_noclip(e, 8)
  set_flag(317)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[721] = true

B[722] = function(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  show_when_clear(e)
  pause(e, 0)
end

-- story actions
story(218, {
  say(200),
})

story(219, {
  say(201),
})

story(220, {
  say(202),
})
