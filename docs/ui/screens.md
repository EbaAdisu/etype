# UI Screens

All screens are rendered with Ratatui in the terminal. No mouse required — keyboard only.

---

## Main Menu

```
  ╔══════════════════════════════════════╗
  ║            e t y p e                ║
  ║   Level 3  •  streak: 7 days  🔥    ║
  ║   Total XP: 2,341                   ║
  ╠══════════════════════════════════════╣
  ║                                     ║
  ║   [1]  Word Rush                    ║
  ║   [2]  Sentence Mode                ║
  ║   [3]  Code Snippets                ║
  ║   [4]  Survival                     ║
  ║                                     ║
  ╠══════════════════════════════════════╣
  ║   [s]  Stats & History              ║
  ║   [h]  Key Heatmap                  ║
  ║   [?]  Help                         ║
  ║   [q]  Quit                         ║
  ╚══════════════════════════════════════╝
```

---

## Difficulty Selector (shown after choosing a mode)

```
  ┌─ Word Rush — Select Difficulty ─────┐
  │                                     │
  │   [1]  Easy     (Level 1+)          │
  │   [2]  Medium   (Level 2+)          │
  │   [3]  Hard     (Level 3+)  ← you   │
  │   [4]  Insane   (Level 4+)  🔒      │
  │                                     │
  │   Timer: [30s]  [60s]  [120s]       │
  │                                     │
  │   [Enter] Start    [Esc] Back       │
  └─────────────────────────────────────┘
```

Locked difficulties show a lock icon and "Reach Level X" tooltip.

---

## Word Rush — Live Game

```
  ┌─ Word Rush — Hard — 60s ───────────────────────────┐
  │  Time: ████████████░░░░░░░░  00:42                 │
  │  WPM: 74      CPM: 370      Accuracy: 96.2%        │
  ├────────────────────────────────────────────────────┤
  │                                                    │
  │   complete   [balance]   fixture   prompt   narrow │
  │                                                    │
  │   > balanc█                                        │
  │                                                    │
  └────────────────────────────────────────────────────┘
```

- Current target word is in brackets `[word]`
- Completed words are dimmed/grey
- Timer bar depletes left to right
- Input line at bottom shows what's being typed

---

## Sentence Mode — Live Game

```
  ┌─ Sentence Mode ────────────────────────────────────┐
  │  CPM: 312      Accuracy: 98.1%      Words: 14/32   │
  ├────────────────────────────────────────────────────┤
  │                                                    │
  │  "The quick brown fox jumps over the lazy dog,     │
  │   and the dog just sat there looking unimpressed." │
  │                                                    │
  │   The quick brown fox jumps [over] the lazy...     │
  │   ✓ ✓ ✓ ✓ ✓ ✓       ↑current                      │
  │                                                    │
  │   > ove█                                           │
  │                                                    │
  └────────────────────────────────────────────────────┘
```

---

## Code Snippets — Live Game

```
  ┌─ Code Snippets — Python ───────────────────────────┐
  │  CPM: 280      Accuracy: 97.5%      Line: 4/12     │
  ├────────────────────────────────────────────────────┤
  │                                                    │
  │  def fibonacci(n):                  ✓              │
  │      if n <= 1:                     ✓              │
  │          return n                   ✓              │
  │  ►     return fibonacci(n-1) + ...  ← current     │
  │          ...                                       │
  │                                                    │
  │   >     return fibonacci█                          │
  │                                                    │
  └────────────────────────────────────────────────────┘
```

- Completed lines show a checkmark ✓
- Current line has a ► arrow
- Indentation must match exactly

---

## Survival — Live Game

```
  ┌─ Survival — Medium ────────────────────────────────┐
  │  Lives: ♥ ♥ ♥     Score: 23     Speed: 2x          │
  ├────────────────────────────────────────────────────┤
  │                                                    │
  │  basket                    pivot                   │
  │                                                    │
  │           noble                                    │
  │                      fetch                         │
  │  globe                                             │
  │                   manor                            │
  │ ─────────────────────────────────── danger zone ── │
  │                                                    │
  │   > globe█                                         │
  └────────────────────────────────────────────────────┘
```

- Words scroll downward each tick
- Danger zone line near the bottom — words crossing it are about to cost a life
- Input box at bottom — type any visible word to destroy it

---

## Results Screen

```
  ╔══════════════════════════════════════╗
  ║         Session Complete!           ║
  ╠══════════════════════════════════════╣
  ║  Mode:      Word Rush — Hard         ║
  ║  Duration:  60s                      ║
  ╠══════════════════════════════════════╣
  ║  WPM:       87      ★ NEW BEST!      ║
  ║  CPM:       435                      ║
  ║  Accuracy:  97.3%                    ║
  ╠══════════════════════════════════════╣
  ║  XP Earned:    +148                  ║
  ║  Total XP:     2,489                 ║
  ║  ▓▓▓▓▓▓▓▓░░░░  Level 4 in 1,011 XP  ║
  ║  Streak:       8 days                ║
  ╠══════════════════════════════════════╣
  ║  [r] Play again    [m] Main menu     ║
  ╚══════════════════════════════════════╝
```

- XP bar shows progress toward next level
- "NEW BEST!" appears in gold when a personal best is beaten
- Level-up gets a special animation if it occurs this session

---

## Stats Screen

```
  ┌─ Stats & History ──────────────────────────────────┐
  │  Personal Bests                                    │
  │  ─────────────────────────────────────────────     │
  │  Word Rush   Easy:87  Med:74  Hard:62  Ins:—       │
  │  Sentence    Easy:91  Med:80  Hard:71  Ins:—       │
  │  Code        Easy:65  Med:—   Hard:—   Ins:—       │
  │  Survival    Easy:43  Med:38  Hard:—   Ins:—       │
  │                                                    │
  │  Recent Sessions                                   │
  │  ─────────────────────────────────────────────     │
  │  2026-05-31  Word Rush   Hard   87 WPM   97.3%    │
  │  2026-05-31  Sentence    Med    80 WPM   98.1%    │
  │  2026-05-30  Survival    Easy   43 WPM   94.2%    │
  │  ...                                               │
  │                                                    │
  │  [Esc] Back                                        │
  └────────────────────────────────────────────────────┘
```

---

## Key Heatmap Screen

```
  ┌─ Key Heatmap (all-time) ───────────────────────────┐
  │  Green = fast & clean    Red = slow or error-prone  │
  │                                                    │
  │  [q][w][e][r][t][y][u][i][o][p]                    │
  │   [a][s][d][f][g][h][j][k][l]                      │
  │    [z][x][c][v][b][n][m]                           │
  │                                                    │
  │  (each key colored by error rate + avg delay)      │
  │                                                    │
  │  Worst keys:                                       │
  │    f — 34 errors  (avg 410ms)                      │
  │    p — 12 errors  (avg 380ms)                      │
  │    b — 8 errors   (avg 350ms)                      │
  │                                                    │
  │  [Esc] Back                                        │
  └────────────────────────────────────────────────────┘
```

---

## Help Screen

```
  ┌─ Help ─────────────────────────────────────────────┐
  │  Global                                            │
  │    q / Ctrl+C   Quit                               │
  │    Esc          Back / Cancel                      │
  │    ?            Toggle this screen                 │
  │                                                    │
  │  In-Game                                           │
  │    Space        Confirm word (Word Rush/Sentence)  │
  │    Enter        Confirm line (Code Snippets)       │
  │    Backspace    Delete last char                   │
  │    Ctrl+W       Delete entire current word/input   │
  │                                                    │
  │  Navigation                                        │
  │    1-4          Select mode or difficulty          │
  │    s            Stats screen                       │
  │    h            Heatmap screen                     │
  │                                                    │
  │  [Esc] Back                                        │
  └────────────────────────────────────────────────────┘
```
