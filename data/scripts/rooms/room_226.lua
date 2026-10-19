-- Behaviours of room 226 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[159] = function(e)
  wait_trigger(e, 24)
  pause(e, 96)
  wait_trigger(e, 2)
  set_flag(81)
  e.visible = false; e.base_visible = false; do return end
end

B[161] = function(e)
  e.hostile = false
  wait_trigger(e, 24)
  cutscene(e, 32)
  pause_player()
  move_noclip(e, 48)
  e.hostile = true; e.dormant = false
  do return end  -- unknown command 26
end
SCRIPTED[161] = true

B[197] = function(e)
  wait_trigger(e, 24)
  pause(e, 100)
  wait_trigger(e, 2)
  tile_swap(1)
  set_flag(89)
  e.visible = false; e.base_visible = false; do return end
end

-- story actions
story(32, {
  say(30),
})
