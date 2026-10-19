-- Mir (kind 124): keeps away and casts small fireballs.

BOSS_AI[124] = function(e, c)
  if c.dist <= 16 then
    c.face_player()
    if c.dist < 3.0 then c.away_step(c.speed * DT) elseif c.dist > 5.5 then c.toward_step(c.speed * DT) end
    if e.ai_cooldown <= 0 then
      boss_fire(e, 135, 4.0, math.max(4, c.attack * 2), 1, false)
      e.ai_cooldown = 1.3 + c.rnd(100)
    end
  end
end
