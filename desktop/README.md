# Ecosystem Observer Desktop / Десктопний оглядач екосистеми

Standalone read-only Tauri shell for the canonical `ecosystem-observer` core.
This application belongs to `ecosystem-observer`; Tauricode is not its source
tree and is not modified by this desktop application.

Самостійний read-only Tauri shell для канонічного ядра
`ecosystem-observer`. Застосунок належить репозиторію
`ecosystem-observer`; Tauricode не є його source tree і не змінюється цим
desktop-застосунком.

```text
repositories + /proc + Guard reference
                  ↓
       EcosystemSnapshot
                  ↓
        Tauri read-only UI
```

## Run / Запуск

```bash
cd desktop
npm install
npm run dev
```

Environment / Змінні середовища:

- `ECOSYSTEM_ROOT` — repository parent; default `$HOME/GitHub`;
- `ECOSYSTEM_REPOS` — comma-separated repository names;
- `ECOSYSTEM_GUARD_REFERENCE` — explicit Guard reference path;
- `ECOSYSTEM_COORDINATION_ROOT` — coordination repository, default
  `$HOME/ecosystem`.

The UI never upgrades observation into judgement. A present legacy path is not
an active authority; a live `swarm-node` process is not proof of delivery or
mesh convergence.

UI не підвищує observation до judgement. Наявний legacy path не стає активною
authority; живий процес `swarm-node` не доводить delivery або mesh convergence.
