-- Behaviours of room 661 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[602] = function(e)
  cutscene(e, 240)
  ::L2::
  wait_trigger(e, 105)
  pause(e, 2)
  cutscene(e, 241)
  pause(e, 8)
  jump_guard(e) goto L2
end
SCRIPTED[602] = true

B[604] = function(e)
  wait_trigger(e, 106)
  set_flag(340)
  pause(e, 0)
end
