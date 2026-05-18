# Progression System

## XP Formula

XP is calculated at the end of every session:

```
accuracy_mul = accuracy_percent / 100.0          // 0.0 – 1.0
mode_bonus   = WordRush:1.0 | Sentence:1.1 | Code:1.3 | Survival:1.2
diff_bonus   = Easy:0.8 | Medium:1.0 | Hard:1.3 | Insane:1.6
streak_bonus = min(1.0 + (streak_days × 0.05), 2.0)   // caps at ×2.0

xp = (wpm × accuracy_mul × mode_bonus × diff_bonus × streak_bonus) as u32
```

**Example:** 70 WPM, 95% accuracy, Hard Word Rush, 10-day streak  
`= (70 × 0.95 × 1.0 × 1.3 × 1.5) = ~130 XP`

---

## Level Thresholds

| Level | Total XP Required | Unlocks                             |
|-------|-------------------|-------------------------------------|
| 1     | 0                 | Easy words, all 4 modes             |
| 2     | 500               | Medium words                        |
| 3     | 1,500             | Hard words                          |
| 4     | 3,500             | Insane words, longer code snippets  |
| 5     | 7,000             | Survival ultra-speed mode           |
| 6+    | +4,000 per level  | Prestige color themes               |

Level is always calculated from `total_xp` in the `profile` table.  
Never store level directly — derive it at runtime.

---

## Streaks

- A streak day is counted when at least 1 session is completed that calendar day
- Streak resets to 0 if a calendar day passes with no session
- `last_played` in the `profile` table is checked on each launch
- If `last_played` is yesterday → streak continues; if older → streak resets
- Streak count is shown on the main menu and included in the results screen

---

## Difficulty Unlocking

- Difficulty is locked until the required level is reached
- Attempting a locked difficulty shows a "Reach Level X to unlock" message
- Easy is always available from level 1
- The unlock check happens in the menu before launching a mode

---

## Personal Bests

- Tracked per `(mode, difficulty)` combination — 16 possible PB slots
- A new PB is detected when `wpm > existing_best_wpm` for that slot
- PB is shown highlighted on the results screen when beaten
- All PBs are viewable on the Stats screen
