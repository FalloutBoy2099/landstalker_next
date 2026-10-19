-- Behaviours of room 269 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[523] = function(e)
  pause(e, 15)
  pause_player()
  move_noclip(e, 32)
  turn(e, 2)
  move_noclip(e, 48)
  cutscene(e, 217)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[523] = true

B[524] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  set_flag(310)
  e.visible = false; e.base_visible = false; do return end
end

B[541] = function(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  set_flag(320)
  move_noclip(e, 32)
  turn(e, 2)
  move_noclip(e, 80)
  e.visible = false; e.base_visible = false; do return end
end

B[542] = function(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  pause(e, 16)
  cutscene(e, 0x100 + 62)
  move_noclip(e, 48)
  turn(e, 1)
  move_noclip(e, 24)
  turn(e, 2)
  move_noclip(e, 40)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[542] = true

B[543] = function(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  pause_player()
  pause(e, 32)
  move_noclip(e, 24)
  turn(e, 1)
  move_noclip(e, 24)
  turn(e, 2)
  move_noclip(e, 64)
  e.visible = false; e.base_visible = false; do return end
end

-- story actions
story(217, {
  say(198),
})

story(318, {
  say(199),
})
