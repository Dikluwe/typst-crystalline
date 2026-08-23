# P1137 — inventário inicial do denominador de paridade

## Proveniência da medição

- Medido em `2026-08-23T13:22:33-03:00`.
- HEAD: `781b207b4a5de9c2bfbe5819918a193d1d9293e5`.
- Estado: **working tree não commitada**; `git diff HEAD --stat` registrou `225 files changed, 9605 insertions(+), 4486 deletions(-)`. A lista exata e atualizada é reproduzida por `git status --short`; o estado inclui trabalho anterior do dono, portanto estes números não são usados para fechar cobertura.
- Vanilla: `upstream/main a51e02804`, binário `lab/typst-original/target/release/typst`. O texto observado `typst 0.15.1 (e0e8ca4d)` não é usado como prova do commit.
- Cristalino: `target/release/typst`, construído a partir desta árvore; texto observado `typst 0.15.0 (781b207b)`. O `0.15.0` é esquecimento conhecido, não alvo de compatibilidade.

## Denominador inicial por superfície

| Superfície | Vanilla ratificado | Cristalino atual | Classificação inicial | Fonte/reprodução |
|---|---|---|---|---|
| Comandos CLI de topo | 8: `compile`, `watch`, `init`, `eval`, `fonts`, `completions`, `info`, `help` | CLI posicional, sem subcomandos | ausentes/diferentes; contrato por comparar integralmente | ambos `--help` |
| Opções CLI | opções globais e opções próprias por subcomando | opções num único parser posicional | nome presente/contrato não comparado quando homônimo | `02_shell/src/cli.rs`; `lab/typst-original/crates/typst-cli/src/args.rs` |
| `SyntaxKind` | 137 variantes extraídas de `typst-syntax/src/kind.rs` | enum em `01_core/src/entities/syntax_kind.rs`; correspondência nominal ainda não fechada | contrato não comparado | `awk` sobre ambos enums |
| Entradas do parser | `parse`, `parse_code`, `parse_math`, reparse | parse cristalino e modos internos | agregado noutra representação; sem conclusão | `typst-syntax/src/parser.rs`; `01_core/src/compiler/parse/` |
| Globais e módulos | biblioteca construída por scopes; inclui `math`, `pdf`, cores, direções e funções | stdlib cristalina distribuída por módulos | nomes presentes, ausentes e agregados exigem extração semântica | `typst-library/src/lib.rs`; `01_core/src/compiler/stdlib/` |
| Funções/elementos/métodos | metadados `NativeFunc`, `NativeElement`, scopes e casts | dispatch e construtores próprios | contrato ainda não comparado; não inferir por nome Rust | `typst-library/src/foundations`; `01_core/src/compiler/eval/bindings/` e `stdlib/` |
| Args nomeados/defaults | metadados de casts e funções nativas | parsing manual/constructors | parcialmente agregado noutra representação | mesmas fontes; requer extrator dedicado posterior |
| Formatos de saída | `pdf`, `png`, `svg`, `html`, `bundle`; HTML requer feature | `pdf`, `png`, `svg`; HTML e bundle ausentes | SVG/PNG presentes; HTML/bundle ausentes; help cristalino desatualizado | `typst compile --help`; `02_shell/src/cli.rs:64` |
| World | `library`, `book`, `main`, `source`, `file`, `font`, `today` | os mesmos sete mais `inputs` na extensão cristalina | agregado/estendido; contratos não comparados | `typst-library/src/lib.rs:62`; `01_core/src/contracts/world.rs:32`; `03_infra/src/world.rs` |
| Packages | CLI vanilla possui paths/cache e resolução; cristalino possui infraestrutura própria | fixture local ainda não construída | `UNMEASURED`, precondição explícita | P1137-C-002 |

## Política do inventário

Este documento inicia o denominador, não calcula percentagem. Contagens de símbolos Rust não são cobertura da linguagem: cada função, método, elemento, argumento e default só entra como fechado após um caso observável. Itens internos sem superfície pública serão `NOT_APPLICABLE/MECHANICS_ALLOWED`; representações diferentes não são ausência. O que refutaria cada classificação inicial é uma medição pública que demonstre presença e contrato equivalente.

O manifesto versionado mantém IDs estáveis e separa observáveis. A próxima ampliação deve automatizar a extração de metadados de scopes em ambos os lados; regex sobre nomes de funções não serve como oráculo semântico.
