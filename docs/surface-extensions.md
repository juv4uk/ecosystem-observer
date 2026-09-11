# Surface extensions · Поверхневі розширення

**Owner decision 2026-09-10 / Рішення власника 2026-09-10**

Equal file-surface spellings (no new semantics, no mass rename):

```text
.my   ↔ .мій
.wsm  ↔ .всм
.lisp ↔ .лісп
```

Рівноправні написання розширень файлів (без нової семантики, без масового rename).

## Scope in this repo · Обсяг у цьому репо

| Area | Status |
|------|--------|
| `surface_ext` helper | Implemented |
| Guard reference path (`.wsm` / `.всм`) | Resolves twin if preferred missing |
| Legacy guard paths with `.my` / `.мій` | Same resolve |
| Contract *parsers* (`contracts.rs`) | Content-based — extension-agnostic (already OK) |
| Git discovery / process observation | **not-applicable** — no extension filters |
| Identity JSON | **not-applicable** — `.json` only |
| CI / editor tooling | **not-applicable** in this repo |

## Fixtures

- `fixtures/приклад.мій`
- `fixtures/знання.всм`
- `fixtures/програма.лісп`

Latin fixtures and paths remain green; Cyrillic is the same parser path when content is read.

## Related

Issue: ecosystem-observer#1
