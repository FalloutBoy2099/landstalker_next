-- Zak (kind 133): closes in and throws boomerangs.

BOSS_AI[133] = function(e, c)
  if c.dist <= 16 then
    c.face_player()
    if c.dist > 2.5 then c.toward_step(c.speed * DT) elseif c.dist < 1.5 then c.away_step(c.speed * 0.6 * DT) end
    if e.ai_cooldown <= 0 and c.dist > 1.8 then
      boss_fire(e, 168, 4.5, c.shot, 1, true, 0x78)   -- SND_Slash2
      e.ai_cooldown = 2.4 + c.rnd(120)
    end
  end
end
