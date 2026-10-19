-- Behaviours of room 709 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[258] = function(e)
  wait_flag(e, 26, true)
  wait_flag(e, 27, false)
  wait_trigger(e, 34)
  pause_player()
  move_to(e, 29, 28)
  move_up_to(e, 16)
  move_noclip(e, 16)
  turn(e, 2)
  move_noclip(e, 32)
  turn(e, 1)
  do return end  -- unknown command 2C
end
SCRIPTED[258] = true
WAITS_FIRST[258] = true
