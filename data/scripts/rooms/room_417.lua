-- Behaviours of room 417 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[831] = function(e)
  wait_flag(e, 450, false)
  cutscene(e, 0x100 + 70)
  wait_trigger(e, 126)
  pause_player()
  set_flag(8)
  wait_trigger(e, 127)
  pause_player()
  pause(e, 18)
  cutscene(e, 0x100 + 72)
  tile_swap(1)
  tile_swap(2)
  set_flag(450)
  show_when_clear(e)
  pause(e, 0)
end
SCRIPTED[831] = true
WAITS_FIRST[831] = true

B[832] = function(e)
  wait_flag(e, 8, true)
  move_noclip(e, 128)
  cutscene(e, 0x100 + 71)
  sfx(47)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[832] = true
WAITS_FIRST[832] = true

B[833] = function(e)
  wait_flag(e, 450, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[833] = true
WAITS_FIRST[833] = true

-- story actions
story(326, {
  make_hostile(1, false),
})

story(327, {
  say(310),
})

story(328, {
  say(311),
  hide_entity(1),
})
