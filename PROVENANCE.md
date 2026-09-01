# Ecosystem Observer provenance audit

## Аудит походження Ecosystem Observer

Date / Дата: 2026-09-01
Result / Результат: **PASS — PREVIOUS MIXED FRAGMENT REPLACED**

Цей аудит визначає, що можна чесно перенести з `juv4uk/tauricode` до
`juv4uk/ecosystem` як canonical observer core. Він не є твердженням про
історичну першість і не замінює юридичну консультацію.

## Перевірений обсяг / Audited scope

```text
tauricode/crates/ecosystem-observer/
  Cargo.toml
  Cargo.lock
  src/*.rs
  tests/**/*.rs
```

`packages/desktop-tauri/` перевірено як відсутній upstream directory, але не
перенесено: за архітектурною межею Tauri shell/UI залишається в Tauricode.

## Evidence / Докази

1. `git cat-file -e upstream/dev:crates/ecosystem-observer` повернув absence;
   цей path не існує в upstream OpenCode.
2. Кожний tracked observer file введено комітами `juv4uk` у проміжку
   2026-08-19—2026-08-27 (`005f2d0bfd`, `40160d92d3`, `73f2078d35`,
   `5d6dba2671` та hardening commits).
3. `git blame --line-porcelain` для всіх source/test/manifest files показав
   одного recorded author: `juv4uk`.
4. Distinctive identifiers `EcosystemSnapshot`, `IdentityStatus`,
   `detect_language_contract_drift` і `discover_ecosystem` мають нуль збігів
   у `upstream/dev`.
5. Root OpenCode `LICENSE` у Tauricode був збережений незмінним; observer
   переноситься під owner MIT з окремим notice для реально похідного фрагмента.
6. Offline `cargo metadata` підтвердив MIT-compatible license expressions для
   locked Rust dependencies; точний перелік містить
   `THIRD-PARTY-NOTICES.md`.

## Усунений mixed fragment / Replaced mixed fragment

Первинний аудит знайшов у `src/git_read.rs` послідовність `GIT_SAFETY_FLAGS`,
скопійовану з OpenCode. Її видалено повністю. Замість неї незалежно визначено
один bounded requirement observer-а: Git probes не повинні брати optional
locks або refresh-ити index. Він виражений стандартною глобальною опцією Git
`--no-optional-locks`; parsing шляхів уже використовує
`status --porcelain=v1 -z`, тому додаткові platform policy overrides не
потрібні.

Після заміни source inspection не виявив інших декларацій `copied from` або
`derived from`. Історичний audit record лишається в Git history, але поточний
source більше не містить знайденого OpenCode fragment.

## Межі висновку / Limits

Git author, відсутність path в upstream і zero identifier matches доводять
repository provenance, але математично не доводять, що жоден рядок ніколи не
походив із невідомого зовнішнього джерела. Тому claim обмежений:

> За доступною Git-історією, upstream comparison і source inspection поточний
> observer є оригінальною роботою WSM; раніше знайдений OpenCode fragment
> замінено незалежною мінімальною реалізацією.

Якщо з'явиться нове джерело або суперечність, статус повертається до
`UNRESOLVED`, а NOTICE оновлюється — факти сильніші за цей звіт.

## Copy result / Результат переносу

Код скопійовано byte-for-byte з Tauricode HEAD перед ecosystem-specific
metadata additions. Канонічний шлях:

Public canonical source / Публічний канонічний source:

```text
https://github.com/juv4uk/ecosystem-observer
```

Наступний окремий gate: перевести Tauricode та private ecosystem на pinned
dependency від цього repository, довести tests + Tauri snapshot parity, і лише
тоді видаляти дубль із Tauricode. Публікація source не оголошує автоматичне
переключення завершеним.
