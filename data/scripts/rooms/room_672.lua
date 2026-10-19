-- Behaviours of room 672 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[253] = function(e)
  ::L0::
  move_wait(e, 48)
  turn(e, e.orientation + 3)
  move_wait(e, 32)
  turn(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[254] = function(e)
  ::L0::
  move_wait(e, 24)
  turn(e, e.orientation + 3)
  move_wait(e, 16)
  turn(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[255] = function(e)
  ::L0::
  move_wait(e, 12)
  turn(e, e.orientation + 3)
  move_wait(e, 8)
  turn(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[256] = function(e)
  wait_flag(e, 8, true)
  wait_trigger(e, 31)
  pause(e, 10)
  wait_trigger(e, 33)
  pause(e, 10)
  cutscene(e, 53)
  pause(e, 0)
end
SCRIPTED[256] = true
WAITS_FIRST[256] = true

B[257] = function(e)
  wait_flag(e, 8, true)
  wait_trigger(e, 32)
  pause(e, 20)
  cutscene(e, 54)
  pause(e, 0)
end
SCRIPTED[257] = true
WAITS_FIRST[257] = true

-- story actions
story(53, {
  if_flag(9, {
      say(64),
    }, {
      if_flag(10, {
          say(64),
        }, {
          say(64),
        }),
    }),
})

story(54, {
  say(65),
  if_flag(9, {}, {
      if_flag(10, {}, {
          if_flag(11, {}),
        }),
    }),
})
