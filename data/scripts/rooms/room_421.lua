-- Behaviours of room 421 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[880] = function(e)
  wait_trigger(e, 134)
  pause_player()
  pause(e, 30)
  cutscene(e, 0x100 + 76)
  set_flag(8)
  pause(e, 0)
end
SCRIPTED[880] = true

B[881] = function(e)
  e.visible = false; e.base_visible = false
  wait_flag(e, 8, true)
  set_flag(9)
  pause(e, 0)
end
SCRIPTED[881] = true

B[882] = function(e)
  e.visible = false; e.base_visible = false
  wait_flag(e, 9, true)
  set_flag(10)
  pause(e, 0)
end
SCRIPTED[882] = true

B[883] = function(e)
  e.visible = false; e.base_visible = false
  wait_flag(e, 10, true)
  pause(e, 120)
  tile_swap(1)
  pause(e, 15)
  cutscene(e, 0x100 + 77)
  pause(e, 0)
end
SCRIPTED[883] = true

-- story actions
story(332, {
  say(319),
})

story(333, {
  lock_controls(false),
})
