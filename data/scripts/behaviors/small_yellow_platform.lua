-- Behaviours that several rooms use for the entities of this kind (small yellow platform).

B[174] = function(e)
  wait_trigger(e, 2)
  move_up_relative(e, 48)
  pause(e, 0)
end

B[501] = function(e)
  wait_trigger(e, 2)
  move_up_timed(e, 48)
  pause(e, 0)
end
