-- Behaviours of room 93 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[529] = function(e)
  e.visible = false; e.base_visible = false
  show_when_clear(e)
  wait_flag(e, 262, true)
  clear_flag(323)
  wait_flag(e, 323, true)
  tile_swap(1)
  set_flag(322)
  pause(e, 0)
end
SCRIPTED[529] = true
