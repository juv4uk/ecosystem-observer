# Ecosystem Observer

**Read-only** observer for the WSM ecosystem: headless Rust core + optional Tauri desktop shell.

```text
observation ≠ judgement
unknown ≠ false
status → source → evidence
```

No mutating actions. Ever.

---

## Quick start

```bash
# core tests
cargo test --locked

# desktop UI
cd desktop && npm install && npm run dev
```

Default discovery root: `$HOME/GitHub`.  
Overrides: `ECOSYSTEM_ROOT`, `ECOSYSTEM_REPOS`, `ECOSYSTEM_GUARD_REFERENCE`, `ECOSYSTEM_COORDINATION_ROOT` — see [`desktop/README.md`](desktop/README.md).

---

## What it observes

- Git state of configured repos
- Local runtime processes / self-reported identity
- Contract facts (with provenance and unknown states)
- Optional: `guard-reference.wsm`, topics, `guard-ask` presence
- Legacy Guard path **presence** (not “active”)
- Live `swarm-node` processes (liveness ≠ delivery / mesh convergence)

Tauricode is **not** part of this app and is not modified by its development.

---

## Українською

Канонічний read-only observer екосистеми WSM. Збирає Git-стан, процеси, ідентичності та контрактні факти; зберігає provenance і невідомі стани. UI лише спостерігає, не керує.

Код перенесено з `juv4uk/tauricode` через coordination repo зі збереженням походження — [`PROVENANCE.md`](PROVENANCE.md).

---

## English

Collects Git state, local processes, identity, and contract facts while preserving provenance and unknowns. Bounded slice can read an explicit `guard-reference.wsm`. Path presence is not activity; process liveness is not mesh convergence.

---

## Ліцензія · License

[ВОЛЬНІСТЬ](LICENSE)
