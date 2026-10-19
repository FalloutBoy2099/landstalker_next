-- Behaviours of room 255 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[521] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  set_flag(309)
  e.visible = false; e.base_visible = false; do return end
end

B[522] = function(e)
  pause(e, 15)
  pause_player()
  move_noclip(e, 32)
  turn(e, 2)
  move_noclip(e, 48)
  cutscene(e, 216)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[522] = true

-- story actions
story(216, {
  say(197),
})
