-- Gola (kind 162): stands still; rings of fireballs (all four directions) and aimed ones.

BOSS_AI[162] = function(e, c)
  if c.dist <= 18 then
    c.face_player()
    if e.ai_cooldown <= 0 then
      e.ai_shots = e.ai_shots + 1
      if e.ai_shots % 2 == 0 then
        local vxs, vys = { 0, 1, 0, -1 }, { -1, 0, 1, 0 }
        for d = 1, 4 do spawn_projectile(e, 172, vxs[d] * 3.5, vys[d] * 3.5, c.shot, false, 0) end
        sfx(0x71)
      else
        boss_fire(e, 172, 4.0, c.shot, 2, false, 0x77)   -- SND_Slash1
      end
      e.ai_cooldown = 2.0 + c.rnd(100)
    end
  end
end
