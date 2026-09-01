# Ecosystem Observer

Canonical headless, read-only observer core for the WSM ecosystem. Tauricode
is a workstation/UI consumer of this model, not its authority.

## Українською

Це канонічне headless і read-only ядро спостереження за екосистемою WSM.
Observer збирає Git-стан, локальні runtime-процеси, self-reported identity та
контрактні факти, зберігаючи provenance і невідомі стани. Tauricode споживає
цю модель як workstation/UI, але не стає її authority.

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
