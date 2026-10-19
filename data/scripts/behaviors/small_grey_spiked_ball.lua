-- Behaviours that several rooms use for the entities of this kind (small grey spiked ball).

B[450] = function(e)
  ::L0::
  move_until_collision(e)
  pause(e, 100)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
