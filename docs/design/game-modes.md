# Game Modes

## Overview

| Mode          | Focus      | Timed? | Lives? | Score Basis         |
|---------------|------------|--------|--------|---------------------|
| Word Rush     | Speed      | Yes    | No     | WPM                 |
| Sentence      | Accuracy   | No     | No     | Accuracy % + CPM    |
| Code Snippets | Precision  | No     | No     | CPM + Accuracy %    |
| Survival      | Endurance  | Yes    | Yes    | Words destroyed + time |

---

## 1. Word Rush

**Concept:** Type as many words as possible before the timer runs out.

**Flow:**
1. Player selects difficulty and timer length (30s / 60s / 120s)
2. Words appear in a line; the current target word is highlighted
3. Player types the word and presses Space to confirm
4. Correct → next word immediately; wrong → word flashes red, player retypes
5. Timer hits 0 → session ends, results screen appears

**Rules:**
- Backspace is allowed within the current word only
- No skipping words (must type each one correctly before moving on)
- Final score = WPM calculated over the full timer duration

**Word source:** `assets/words/{difficulty}.txt`

---

## 2. Sentence / Paragraph Mode

**Concept:** Type a full quote or paragraph with high accuracy. No time pressure.

**Flow:**
1. A quote or paragraph is displayed in full
2. Current word is highlighted; already-typed words are dimmed
3. Correct chars appear in green, wrong chars appear in red inline
4. Backspace allowed freely within the current word
5. Finishing the last word ends the session

**Rules:**
- Spaces between words are required (Space advances to next word)
- Accuracy is measured character by character
- No timer — this mode rewards accuracy over raw speed

**Content source:** Hardcoded in `src/content/sentences.rs`

---

## 3. Code Snippets

**Concept:** Type a real code snippet exactly — whitespace, indentation, and all.

**Flow:**
1. A short code snippet (8–20 lines) is shown with syntax coloring
2. Player types it line by line, including all indentation
3. Each line is validated when Enter is pressed
4. Wrong lines flash red and must be retyped correctly
5. Finishing the last line ends the session

**Rules:**
- Indentation is part of the content — tabs and spaces must match exactly
- Enter key advances to next line (not Space)
- Backspace allowed freely on the current line
- Score = CPM + accuracy %

**Snippet source:** `assets/code/{python,bash,rust}.txt`  
Format: snippets separated by `---` delimiter in each file

---

## 4. Survival — Falling Words

**Concept:** Words fall down the screen. Type them before they reach the bottom.

**Flow:**
1. Words appear at the top and scroll down one row per tick
2. Player types any visible word and presses Space to destroy it
3. If a word reaches the bottom row → lose 1 life
4. 3 lives total; losing all 3 ends the session
5. Scroll speed increases every 30 seconds

**Rules:**
- Player can target any word on screen (not forced sequential order)
- Partially typed input is shown in a dedicated input box at the bottom
- New words spawn at random horizontal positions at the top
- Spawn rate also increases over time alongside speed

**Score:** Words destroyed + survival time in seconds  
**Word source:** `assets/words/{difficulty}.txt` (same word lists, shuffled)

---

## Difficulty Tiers

| Tier    | Word Length | Unlock Level | XP Multiplier |
|---------|-------------|--------------|---------------|
| Easy    | 3–4 chars   | 1 (default)  | ×0.8          |
| Medium  | 5–7 chars   | 2            | ×1.0          |
| Hard    | 8–12 chars  | 3            | ×1.3          |
| Insane  | rare/long   | 4            | ×1.6          |
