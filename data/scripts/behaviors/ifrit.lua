-- Ifrit (kind 160): stays 4 to 6 cells away and shoots, three at a time now and then.

BOSS_AI[160] = function(e, c)
  if c.dist <= 16 then
    c.face_player()
    if c.dist < 3.5 then c.away_step(c.speed * DT) elseif c.dist > 6.0 then c.toward_step(c.speed * DT) end
    if e.ai_cooldown <= 0 then
      e.ai_shots = e.ai_shots + 1
      boss_fire(e, 161, 4.5, c.shot, (e.ai_shots % 3 == 0) and 3 or 1, false)
      e.ai_cooldown = 1.4 + c.rnd(100)
    end
  end
end
