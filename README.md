# Ecosystem Observer

Canonical read-only observer for the WSM ecosystem: a reusable headless Rust
core and a standalone Tauri desktop application. Tauricode is not part of the
application and is not modified by its development.

## Українською

Це канонічний read-only observer екосистеми WSM: повторно використовуване
headless Rust-ядро та окремий desktop-застосунок на Tauri.
Observer збирає Git-стан, локальні runtime-процеси, self-reported identity та
контрактні факти, зберігаючи provenance і невідомі стани. Bounded operational
slice також читає явно налаштований `guard-reference.wsm`, показує topics та
наявність `guard-ask`, окремо фіксує присутність legacy Guard paths і показує
живі `swarm-node` процеси. Наявність legacy path не означає, що він активний;
наявність процесу не доводить delivery або mesh convergence. Desktop observer
живе в цьому репозиторії; для його побудови не потрібні зміни в Tauricode.

## English

The observer collects Git state, local runtime processes, self-reported
identity, and contract facts while preserving provenance and unknown states.
The bounded operational slice reads an explicitly configured
`guard-reference.wsm`, reports topics and `guard-ask` presence, records legacy
Guard path presence separately, and lists live `swarm-node` processes. A
legacy path's presence is not evidence that it is active; process liveness is
not delivery or mesh-convergence evidence. The standalone desktop shell lives
in this repository and does not require changes to Tauricode.

Поточний код перенесено з `juv4uk/tauricode` через приватний coordination repo
`juv4uk/ecosystem` зі збереженням історичного походження. Повний аудит
описаний у [`PROVENANCE.md`](PROVENANCE.md).

```bash
cargo test --locked
```

## Desktop / Desktop-застосунок

`desktop/` — окремий Tauri 2 shell над тим самим `ecosystem-observer` core.
Він показує Git snapshots репозиторіїв, локальні agent/process identities,
Guard reference directory та живі `swarm-node` процеси. UI не має mutating
actions: він спостерігає, не керує.

`desktop/` is a standalone Tauri 2 shell over the same
`ecosystem-observer` core. It shows repository Git snapshots, local
agent/process identities, the Guard reference directory, and live
`swarm-node` processes. The UI has no mutating actions: it observes and does
not control.

```bash
cd desktop
npm install
npm run dev
```

За замовчуванням observer читає `$HOME/GitHub`. Шляхи можна змінити через
`ECOSYSTEM_ROOT`, `ECOSYSTEM_REPOS`, `ECOSYSTEM_GUARD_REFERENCE` та
`ECOSYSTEM_COORDINATION_ROOT`. Деталі наведено в
[`desktop/README.md`](desktop/README.md).

By default the observer reads `$HOME/GitHub`. Override discovery with
`ECOSYSTEM_ROOT`, `ECOSYSTEM_REPOS`, `ECOSYSTEM_GUARD_REFERENCE`, and
`ECOSYSTEM_COORDINATION_ROOT`; see
[`desktop/README.md`](desktop/README.md).

Не додавайте mutating operations до observer core. Його контракт:

```text
observation != judgement
unknown != false
status -> source -> evidence
```

## Ліцензія

Цей твір поширюється під [ВОЛЬНІСТЮ](LICENSE) — простим словом про свободу творити, пам'ятаючи про волю іншого.
