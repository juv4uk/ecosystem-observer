# Ecosystem Observer

Canonical headless, read-only observer core for the WSM ecosystem. Tauricode
is a workstation/UI consumer of this model, not its authority.

## Українською

Це канонічне headless і read-only ядро спостереження за екосистемою WSM.
Observer збирає Git-стан, локальні runtime-процеси, self-reported identity та
контрактні факти, зберігаючи provenance і невідомі стани. Bounded operational
slice також читає явно налаштований `guard-reference.wsm`, показує topics та
наявність `guard-ask`, окремо фіксує присутність legacy Guard paths і показує
живі `swarm-node` процеси. Наявність legacy path не означає, що він активний;
наявність процесу не доводить delivery або mesh convergence. Tauricode
споживає цю модель як workstation/UI, але не стає її authority.

## English

The observer collects Git state, local runtime processes, self-reported
identity, and contract facts while preserving provenance and unknown states.
The bounded operational slice reads an explicitly configured
`guard-reference.wsm`, reports topics and `guard-ask` presence, records legacy
Guard path presence separately, and lists live `swarm-node` processes. A
legacy path's presence is not evidence that it is active; process liveness is
not delivery or mesh-convergence evidence. Tauricode consumes this model as a
workstation/UI and does not become its authority.

Поточний код перенесено з `juv4uk/tauricode` через приватний coordination repo
`juv4uk/ecosystem` зі збереженням історичного походження. Повний аудит
описаний у [`PROVENANCE.md`](PROVENANCE.md).

```bash
cargo test --locked
```

Не додавайте mutating operations до observer core. Його контракт:

```text
observation != judgement
unknown != false
status -> source -> evidence
```
