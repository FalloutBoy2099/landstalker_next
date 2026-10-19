-- Behaviours of room 480 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[735] = function(e)
  wait_flag(e, 316, true)
  show_when_clear(e)
  pause(e, 0)
end
SCRIPTED[735] = true
WAITS_FIRST[735] = true

B[736] = function(e)
  wait_flag(e, 316, true)
  wait_flag(e, 318, false)
  pause(e, 15)
  pause_player()
  sfx(88)
  move_until_collision(e)
  turn(e, 0)
  move_until_collision(e)
  rotate_player(2)
  cutscene(e, 0x100 + 36)
  set_flag(8)
  pause(e, 0)
end
SCRIPTED[736] = true
WAITS_FIRST[736] = true

B[737] = function(e)
  wait_flag(e, 8, true)
  cutscene(e, 0x100 + 37)
  turn(e, 2)
  move_until_collision(e)
  tile_swap(1)
  turn(e, 3)
  move_noclip(e, 24)
  turn_wait(e, 2)
  set_flag(318)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[737] = true
WAITS_FIRST[737] = true

-- story actions
story(292, {
  say(243),
})

story(293, {
  play_sound(91),
})
