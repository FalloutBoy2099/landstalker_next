-- Behaviours of room 331 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[660] = function(e)
  show_when_clear(e)
  wait_trigger(e, 95)
  e.visible = false; e.base_visible = false; do return end
end

B[677] = function(e)
  wait_trigger(e, 95)
  show_when_clear(e)
  pause(e, 0)
end
