# Known public repos and roles · Відомі публічні репо та ролі

**As of / Станом на 2026-09-11.**  
This is **knowledge**, not a hardcoded discovery filter. Discovery remains directory-based under `ECOSYSTEM_ROOT` / `$HOME/GitHub`.

Це **знання** для людей і опційної анотації, не hardcoded-фільтр discovery. Сканування лишається за директорією.

Observer does not claim authority over any of these repos.  
Observer не претендує на авторитет над жодним із цих репо.

## Core language line · Мовна лінія

| Repo | Role / Роль |
|------|------|
| **my-lisp** | Semantic source of truth. Owns `language-contract.my` (**6.0**), Canon 0+7, Advice Taker. · Семантичне джерело істини. |
| **fpga-lisp** | Hardware Lisp machine (SystemVerilog). ISA / tagged words / physical execution. · Апаратна Lisp-машина. |
| **cml** | Heterogeneous AOT middle-end / Compute IR. · AOT-компілятор middle-end. |

Compatibility = `(language-contract, ISA-contract)` + tested SHAs.  
Сумісність = пара контрактів + перевірені SHA, не спільні номери релізів.

## ABI / self-hosted WSM · ABI / самодостатній WSM

| Repo | Role / Роль |
|------|------|
| **wsm-target-contract** | Neutral **ABI contract** (`target-contract.wsm`, schema `wsm-os-target-v1`, **version 4**). Consumers: cml + wsm-os-lisp. Parse: `parse_wsm_target_contract_version`. · Нейтральний ABI-контракт. |
| **wsm-my-lisp** | Self-hosted WSM; runtime DLL for Cyberpunk. · Self-hosted + DLL. |
| **wsm-os-lisp** | Bare-metal / WSM-native Lisp machine research. |
| **wsm-os** / **wsm** | Hardware-execution counterpart; research from `()`. |

## Product / surface · Продуктова поверхня

| Repo | Role / Роль |
|------|------|
| **my-lisp-cyberpunk** | In-game product surface (RED4ext, fixed dispatch, opaque GameHandle). Does **not** own language semantics or the runtime DLL. · In-game поверхня; не володіє семантикою мови. |
| **my-idea** | IDE shell (historical parent of my-lisp extraction). |
| **chess-lisp-zero** | Chess on my-lisp + Tauri. |

## Foundation · Фундамент

| Repo | Role / Роль |
|------|------|
| **pravda** | Ukrainian legal tradition; cradle of **ВОЛЬНІСТЬ** / PACTA. · Колиска ліцензії. |
| **shiva-sutras** / **my-lisp-panini** | Pāṇini / Sanskrit formal research. |
| **mccarthy-eval** | McCarthy 1960 eval/apply in x86_64 asm. |
| **ecosystem-observer** | This repo — read-only observation only. · Лише спостереження. |

## What the observer deliberately does not do · Чого observer свідомо не робить

- Does not observe the Cyberpunk game process / RED4ext runtime. · Не спостерігає процес гри.  
- Does not treat repo presence as “healthy”. · Наявність ≠ здоров'я.  
- Does not invent roles for unknown directories. · Невідоме лишається невідомим.  
- Does not claim authority over ABI tag numbers — only reports declared `version` when readable. · Не авторитет над тегами ABI.

## Related · Пов'язане

- Status audit: [`STATUS-AUDIT-2026-09-11.md`](STATUS-AUDIT-2026-09-11.md)
- Tasks: [`../tasks.my`](../tasks.my)
