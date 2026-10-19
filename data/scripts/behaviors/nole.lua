-- Nole (kind 165): walks at the player and throws axes that come back.

BOSS_AI[165] = function(e, c)
  if c.dist <= 16 then
    c.face_player()
    if c.dist > 1.6 then c.toward_step(c.speed * 0.8 * DT) end
    if e.ai_cooldown <= 0 and c.dist > 2.0 then
      boss_fire(e, 166, 5.0, c.shot, 1, true, 0x77)   -- SND_Slash1
      e.ai_cooldown = 3.0 + c.rnd(150)
    end
  end
end
