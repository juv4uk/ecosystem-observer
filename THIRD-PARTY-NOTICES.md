# Third-party notices / Повідомлення про сторонні фрагменти

## OpenCode

`src/git_read.rs` contains the `GIT_SAFETY_FLAGS` configuration-value sequence
copied from `packages/opencode/src/git/index.ts` in the Tauricode/OpenCode
history. The surrounding Rust probe implementation is original WSM work; the
flag sequence retains the upstream notice below.

`src/git_read.rs` містить послідовність конфігураційних значень
`GIT_SAFETY_FLAGS`, скопійовану з `packages/opencode/src/git/index.ts` в
історії Tauricode/OpenCode. Код Rust довкола probe є оригінальною роботою WSM;
для послідовності прапорців зберігається upstream notice:

```text
MIT License

Copyright (c) 2025 opencode

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Rust dependencies / Залежності Rust

The locked dependency set was inspected offline through Cargo metadata on
2026-09-01. It consists of MIT-compatible dependencies: `serde`, `serde_json`,
`itoa`, `proc-macro2`, `quote`, `syn` (MIT OR Apache-2.0), `memchr` (Unlicense
OR MIT), `unicode-ident` ((MIT OR Apache-2.0) AND Unicode-3.0), and `zmij`
(MIT), including their locked support crates. Their licenses apply to those
dependencies, not to original observer source.

Locked dependency set перевірено offline через Cargo metadata 2026-09-01.
Залежності мають MIT-сумісні умови, наведені вище; їхні ліцензії стосуються
самих залежностей, а не оригінального коду observer-а.
