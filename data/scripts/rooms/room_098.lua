-- Behaviours of room 98 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[290] = function(e)
  wait_trigger(e, 39)
  set_flag(156)
  pause_player()
  pause(e, 45)
  move_to(e, 24, 12)
  pause(e, 35)
  move_wait(e, 56)
  turn(e, 3)
  move_wait(e, 56)
  turn(e, 2)
  move_wait(e, 16)
  turn(e, 1)
  pause(e, 90)
  e.speed = 1
  pause(e, 240)
  pause(e, 165)
  turn(e, 0)
  move_noclip(e, 32)
  pause(e, 12)
  move_noclip(e, 16)
  pause(e, 200)
  pause(e, 90)
  turn(e, 1)
  move_noclip(e, 112)
  turn(e, 2)
  move_noclip(e, 32)
  move_noclip(e, 32)
  turn(e, 3)
  pause(e, 4)
  e.speed = 1
  ::L56::
  turn(e, math.random(0, 3))
  move_until_collision(e)
  pause(e, 5)
  jump_guard(e) goto L56
  pause(e, 0)
end

B[291] = function(e)
  wait_trigger(e, 39)
  pause(e, 65)
  move_to(e, 25, 12)
  pause(e, 15)
  playback(29)
  move_wait(e, 56)
  turn(e, 3)
  move_wait(e, 64)
  turn(e, 1)
  pause(e, 90)
  turn(e, 1)
  move_noclip(e, 32)
  e.speed = 1
  move_noclip(e, 144)
  turn(e, 2)
  move_noclip(e, 32)
  repeat_begin(e, 4)
  ::L32::
  turn(e, 3)
  e.speed = 1
  move_noclip(e, 32)
  pause(e, 16)
  if repeat_next(e) then goto L32 end
  turn(e, 2)
  move_noclip(e, 5)
  pause(e, 60)
  turn(e, 2)
  e.speed = 1
  move_noclip(e, 96)
  turn(e, 0)
  move_noclip(e, 32)
  repeat_begin(e, 3)
  ::L56::
  move_noclip(e, 32)
  pause(e, 16)
  if repeat_next(e) then goto L56 end
  turn(e, 1)
  move_noclip(e, 18)
  e.speed = 1
  ::L67::
  turn(e, math.random(0, 3))
  move_until_collision(e)
  pause(e, 5)
  jump_guard(e) goto L67
  pause(e, 180)
  pause(e, 0)
end

B[292] = function(e)
  wait_trigger(e, 39)
  pause(e, 240)
  cutscene(e, 88)
  pause(e, 60)
  do return end  -- unknown command 59
end
SCRIPTED[292] = true
