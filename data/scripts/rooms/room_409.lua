-- Behaviours of room 409 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[766] = function(e)
  wait_flag(e, 347, false)
  wait_flag(e, 345, true)
  tile_swap(1)
  show_when_clear(e)
  wait_trigger(e, 122)
  pause_player()
  pause(e, 3)
  rotate_player(2)
  tile_swap(2)
  move_wait(e, 16)
  e.speed = 2
  move_wait(e, 16)
  e.speed = 4
  move_wait(e, 16)
  e.speed = 8
  move_until_collision(e)
  pause(e, 0)
end
SCRIPTED[766] = true
WAITS_FIRST[766] = true
