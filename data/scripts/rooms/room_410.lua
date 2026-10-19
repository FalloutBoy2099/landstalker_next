-- Behaviours of room 410 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[769] = function(e)
  wait_flag(e, 347, false)
  set_flag(347)
  wait_trigger(e, 123)
  cutscene(e, 0x100 + 61)
  pause(e, 0)
end
SCRIPTED[769] = true
WAITS_FIRST[769] = true

B[798] = function(e)
  wait_trigger(e, 124)
  pause_player()
  move_noclip(e, 128)
  cutscene(e, 0x100 + 67)
  sfx(47)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[798] = true

B[799] = function(e)
  wait_flag(e, 448, false)
  cutscene(e, 0x100 + 68)
  wait_trigger(e, 125)
  pause_player()
  pause(e, 18)
  cutscene(e, 0x100 + 69)
  tile_swap(1)
  set_flag(448)
  show_when_clear(e)
  pause(e, 0)
end
SCRIPTED[799] = true
WAITS_FIRST[799] = true

B[801] = function(e)
  show_when_clear(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  e.visible = false; e.base_visible = false; do return end
end

-- story actions
story(317, {
  run(346),
  say(302),
  friday_does("leave"),
})

story(323, {
  say(308),
})

story(324, {
  make_hostile(2, false),
})

story(325, {
  say(309),
  hide_entity(2),
})
