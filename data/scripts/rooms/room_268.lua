-- Behaviours of room 268 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[723] = function(e)
  wait_trigger(e, 2)
  set_flag(97)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
