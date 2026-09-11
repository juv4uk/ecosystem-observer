# Status audit: ecosystem-observer vs live ecosystem

**Date:** 2026-09-11  
**Scope:** How far this repo has lagged behind the rest of the WSM / my-lisp ecosystem.

## Summary

**Medium–strong lag.** The observer still works for what it was built to do (local Git scan, processes, identity, Guard presence, read-only principle). It has not tracked the 9–11 September surge: Cyberpunk product surface, `wsm-target-contract`, language-contract **6.0**, and the expanded self-hosted WSM line.

## Timeline

| When | What |
|------|------|
| 2026-09-01 | ecosystem-observer extracted / published |
| ~2026-09-08 | Last meaningful code activity |
| 2026-09-08 | my-lisp language-contract **6.0** ratified |
| 2026-09-09+ | wsm-target-contract, heavy cml / fpga / wsm-* work |
| 2026-09-10–11 | my-lisp-cyberpunk created and driven hard (RED4ext, opaque GameHandle, deep-penetration tasks) |

## What still holds

- Read-only axiom: `observation ≠ judgement`, `unknown ≠ false`.
- Directory-based discovery (no hardcoded repo list that would go stale).
- Git state, partial/failed scan status, dirty paths, remotes.
- Identity / orphaned process correlation.
- Provenance audit (PROVENANCE.md) remains clean.
- Guard reference + swarm-node *presence* observation (liveness ≠ convergence).

## Concrete gaps

1. **Contracts**  
   `src/contracts.rs` fixtures and the acceptance-gate test still use the old 3.0 vs claimed 1.0 drift case (August). Live `language-contract.my` is **6.0**.

2. **New first-class repos** (no special knowledge / role annotation)  
   - `my-lisp-cyberpunk` — in-game product surface (RED4ext)  
   - `wsm-target-contract` — neutral ABI for cml + wsm-os-lisp  
   - `wsm-my-lisp` / `wsm-os-lisp` — self-hosted / bare-metal line  
   - Ongoing growth of cml / fpga-lisp / my-lisp itself

3. **tasks.my** was almost empty (only bilingual docs). Catch-up tasks added 2026-09-11.

4. **Conceptual silence** on the Cyberpunk surface and the ABI-contract axis. Observer correctly does *not* try to observe the game process, but it also did not acknowledge that the product surface exists.

## Recommended order (see tasks.my)

1. `CONTRACTS-6.0-UPDATE` — refresh fixtures/tests to 6.0, keep real-drift gate spirit.  
2. `KNOWN-REPOS-ROLES` — document current key public repos and roles (knowledge, not a filter).  
3. `WSM-TARGET-CONTRACT-AWARENESS` — light presence/version observation of the ABI repo.  
4. `CYBERPUNK-SURFACE-NOTE` — acknowledge the product surface without claiming in-game observation.  
5. `DESKTOP-SMOKE-NEW-REPOS` — UI still behaves with the newer local checkouts.  
6. `BILINGUAL-DOCUMENTATION-AUDIT` — still open from the 2026-09-08 language-debt pass.

## What this audit is not

- Not a claim that the observer is broken.  
- Not a redesign of discovery (directory scan remains correct).  
- Not authority over language or ABI contracts — only observation of their presence and declared versions.

## Criterion for "caught up enough"

- Contracts module speaks 6.0.  
- Known-repos doc exists and lists the current critical set.  
- tasks.my no longer looks abandoned.  
- Desktop does not silently drop the new repos when they sit under the scan root.
