-- Behaviours of room 415 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[751] = function(e)
  ::L0::
  wait_trigger(e, 119)
  pause_player()
  turn(e, 2)
  move_to(e, 16, 15)
  move_noclip(e, 64)
  turn_wait(e, 1)
  cutscene(e, 0x100 + 57)
  playback(53)
  turn(e, 0)
  move_noclip(e, 64)
  pause(e, 5)
  move_to(e, 16, 1)
  jump_guard(e) goto L0
end
SCRIPTED[751] = true

B[752] = function(e)
  wait_trigger(e, 2)
  show_when_clear(e)
  set_flag(333)
  do return end  -- unknown command 2E
end

-- story actions
story(313, {
  say(300),
})
