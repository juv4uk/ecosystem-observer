# Known public repos and roles (observer knowledge)

**As of 2026-09-11.**  
This is **knowledge for humans and for optional annotation**, not a hardcoded discovery filter. Discovery remains directory-based under `ECOSYSTEM_ROOT` / `$HOME/GitHub`.

Observer does not claim authority over any of these repos. It only needs to stop being silent about the current shape of the ecosystem.

## Core language line

| Repo | Role |
|------|------|
| **my-lisp** | Semantic source of truth. Owns `language-contract.my` (currently **6.0**), Canon 0+7, Advice Taker direction. |
| **fpga-lisp** | Hardware Lisp machine (SystemVerilog). Owns ISA / tagged words / physical execution. Second independent implementation of the language. |
| **cml** | Heterogeneous AOT middle-end / Compute IR. Lowers my-lisp semantics toward CPU, CUDA, FPGA, freestanding x86_64. |

Compatibility is the pair `(language-contract, ISA-contract)` + tested SHAs — not shared release numbers.

## ABI / self-hosted WSM line

| Repo | Role |
|------|------|
| **wsm-target-contract** | Neutral machine-readable **ABI contract** for the first x86_64 WSM target. Equal consumers: cml (emits) and wsm-os-lisp (runtime). Breaks the old CML ↔ wsm-os-lisp cycle. |
| **wsm-my-lisp** | Self-hosted WSM implementation (Lisp + assembler), growing independence from my-lisp's Rust bootstrap. Hosts the runtime DLL used by Cyberpunk. |
| **wsm-os-lisp** | WSM-native Lisp machine / bare-metal integration research. |
| **wsm-os** | Hardware-execution counterpart to wsm. |
| **wsm** | Independent research from `()` toward mathematics. |

## Product / surface

| Repo | Role |
|------|------|
| **my-lisp-cyberpunk** | In-game product surface: RED4ext adapter, fixed host-dispatch, opaque GameHandle. Does **not** own language semantics or the runtime DLL. |
| **my-idea** | IDE / language-development shell (historical parent of my-lisp extraction). |
| **chess-lisp-zero** | Chess engine on my-lisp + Tauri shell. |

## Foundation / research

| Repo | Role |
|------|------|
| **pravda** | 1000-year Ukrainian legal tradition research; cradle of **ВОЛЬНІСТЬ** and PACTA. License foundation for the ecosystem. |
| **shiva-sutras** / **my-lisp-panini** | Pāṇini / Śiva-sūtras formal research; foundation work for Sanskrit surface / semantics. |
| **mccarthy-eval** | McCarthy 1960 eval/apply in real x86_64 assembly. |
| **ecosystem-observer** | This repo — read-only observation only. |

## What the observer deliberately does **not** do

- Does not observe the Cyberpunk game process or RED4ext runtime state.
- Does not treat presence of a repo as “healthy” or “complete”.
- Does not invent roles for unknown directories under the scan root — unknown stays unknown.

## Related

- Status lag audit: [`STATUS-AUDIT-2026-09-11.md`](STATUS-AUDIT-2026-09-11.md)
- Tasks: [`../tasks.my`](../tasks.my)
