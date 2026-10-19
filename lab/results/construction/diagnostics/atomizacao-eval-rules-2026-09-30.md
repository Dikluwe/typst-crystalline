# Atomização comportamental de `compiler/eval/rules.rs` — 2026-09-30

## Proveniência

Medições na working tree não commitada sobre HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. O estado anterior está em `atomizacao-eval-rules-2026-09-30-before.json.gz`; o estado final, com `git status`, `git diff HEAD --stat`, SHA-256 dos owners e linhas de código, está em `atomizacao-eval-rules-2026-09-30-after.json.gz`. Os snapshots descrevem árvores não commitadas, não commits novos.

## Inventário

O estado anterior tinha 4754 linhas em `eval/rules.rs`. O hub final contém 1040 linhas, incluindo diagnósticos compartilhados, constantes de domínio, bindings estáticos e testes históricos. Foram movidas 29 funções e a macro de realização para oito owners comportamentais, cada qual com Prompt L0 exclusivo. O owner textual anterior permanece independente.

| Owner L1 | Comportamento |
|---|---|
| `rules/pdf.rs` | validação de pages/standard e conflitos PDF |
| `rules/math_set.rs` | schemas matemáticos de set |
| `rules/realize.rs` | realização causal de parágrafos |
| `rules/apply.rs` | aplicação de show rules |
| `rules/outline.rs` | outlines sintetizados |
| `rules/intercept.rs` | interceptação de conteúdo/parágrafos/rótulos |
| `rules/set.rs` | dispatcher e schemas de set rules |
| `rules/show.rs` | avaliação e captura léxica de show rules |

Os corpos dos algoritmos foram preservados e continuam na fase eval. `set.rs` permanece o maior owner, com branches de target a pesquisar separadamente; esta etapa isolou-o dos outros algoritmos, sem afirmar que seus próprios branches já são independentes. O parent conserva as referências aos Núcleos Tekt vigentes; os prompts filhos pinam os claims usados pelas respectivas famílias.

## Verificação

- Antes da extração: `cargo test -p typst-core --lib --offline --locked --quiet` — 6401 passaram, 2 ignorados (`/tmp/eval-rules-baseline.log`).
- Depois do resselo: mesmo comando — 6401 passaram, 2 ignorados (`/tmp/eval-rules-final-test.log`).
- `cargo build --offline --locked` — exit 0 (`/tmp/eval-rules-final-build.log`).
- `crystalline-lint .` — exit 0 (`/tmp/eval-rules-final-lint.log`); avisos históricos de outras áreas permanecem.
- `git diff --check` — exit 0.

A suíte verifica ausência de regressões no comportamento coberto. Isto não é benchmark nem nova medição bilateral com o vanilla pinado.
