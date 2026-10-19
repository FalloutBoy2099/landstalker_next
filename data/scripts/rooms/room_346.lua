-- Behaviours of room 346 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[887] = function(e)
  show_when_clear(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  set_flag(426)
  e.visible = false; e.base_visible = false; do return end
end

B[903] = function(e)
  pause(e, 20)
  pause_player()
  sfx(84)
  e.falls = true; e.fall_v = 0
  pause(e, 20)
  sfx(94)
  e.speed = 1
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[903] = true

B[904] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
