# Relatório — typst-passo-863: `#show par: …` como regra de elemento

**Data:** 2026-07-23T12:56:36-03:00  
**Executor:** Kimi Code (subagente; materialização em `00_nucleo/materialization/typst-passo-863.md`).  
**Proveniência das medições:** working tree não commitado sobre commit `06a336b0c3ae42949c2bced6c4c6511b9d89bebd`.  
**Nota de base:** o working tree contém também as alterações dos passos P861, P862, P865, P866 e P867; as contagens abaixo reflectem o working tree completo vs. HEAD, não apenas o P863.

---

## Objetivo

Implementar `#show par: <transformação>` como regra de elemento aplicável a parágrafos no cristalino, em vez de rejeitar a regra ou apenas aceitar `#show par: set block(spacing: ..)`.

---

## Decisões arquiteturais

| Decisão | Justificação |
|---------|--------------|
| `Content::Par { body: Box<Content> }` como variante inline | Um parágrafo no cristalino é apenas um contentor do seu `body`; não justifica um módulo de elemento (`ParElem`) próprio nesta fase. |
| `NodeKind::Paragraph` renomeado para `NodeKind::Par` | Alinhamento com o nome da variante `Content::Par` e com a convenção `par` da linguagem Typst. |
| Paragraph realization em `01_core/src/engine/eval/rules.rs` | Funções `realize_paragraphs` / `realize_flow` / `realize_node` aplicam as regras `NodeKind::Par` ao fluxo montado por `eval_markup`. |
| `intercept_paragraphs` aplica **apenas** regras `NodeKind::Par` | Evita re-disparar regras de texto já aplicadas eager no topo do markup. |
| `#show par: set block(spacing: ..)` continua a dar warning sem registar regra | Mantém o comportamento pré-existente; `set block(spacing: ..)` sobre `par` não é suportado como regra armazenada. |
| `#set par(...)` inalterado | Continua a fluir via `StyleChain`, sem passar pela paragraph realization. |

---

## Ficheiros alterados (relevantes para P863)

- `01_core/src/entities/show.rs` — `Paragraph` → `Par`.
- `01_core/src/entities/content.rs` — variante `Par`, construtor, e braços em `elem_name`, `fmt_content`, `is_empty`, `plain_text`, `get_field`, `map_content`, `map_text`, `PartialEq`.
- `01_core/src/engine/eval/repr.rs` — braço `Content::Par` no `repr_content`.
- `01_core/src/engine/layout/mod.rs` — braço `Content::Par` transparente no `layout_content`.
- `01_core/src/engine/introspect.rs` — braços `Content::Par` em `materialize_time` e `walk`.
- `01_core/src/engine/eval/rules.rs` — `selector_matches` para `Par`, `eval_show_rule` com `native_par`, e as funções de paragraph realization (`realize_paragraphs`, `realize_flow`, `realize_node`, `intercept_paragraphs`).
- `01_core/src/engine/eval/mod.rs` — invocação final `rules::intercept_paragraphs` no retorno de `eval_markup`.
- `01_core/src/engine/eval/tests.rs` — testes P863.
- `03_infra/src/query_helpers.rs` — braços `Content::Par` em `has_any_text` e `count_variant`.
- `00_nucleo/prompts/engine/eval.md`, `00_nucleo/prompts/entities/content.md`, `00_nucleo/prompts/entities/show.md` — actualizações de L0.
- Headers `@prompt-hash` actualizados por `crystalline-lint --fix-hashes .` em 16 ficheiros de layout e `structural.rs`.

Estatísticas globais do working tree vs. HEAD (inclui P861–P867):

```text
55 files changed, 1981 insertions(+), 219 deletions(-)
```

---

## Validações

### Build

```bash
$ cargo check -p typst-core
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s

$ cargo build --workspace
    Finished dev profile [unoptimized + debuginfo] target(s) in 0.15s
```

### Testes P863

```bash
$ cargo test -p typst-core p863 -- --nocapture
running 3 tests
test engine::eval::tests::tests::p863_set_par_spacing_compila_sem_abortar ... ok
test engine::eval::tests::tests::p863_show_par_identidade_realiza_e_aplica ... ok
test engine::eval::tests::tests::p863_show_par_func_transforma_paragrafo ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 4681 filtered out
```

### Testes globais

```bash
$ cargo test --workspace
... 4681+ passed, nenhuma regressão
```

### Linter

```bash
$ crystalline-lint .
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' não é referenciado por nenhum arquivo em L1–L4. Materializar ou remover. [V7]
```

Resultado: **zero violations**, excepto o V7 pré-existente (`package_version_resolution.md` órfão).

Nota: uma primeira passagem do linter após a implementação revelou V5 (`PromptDrift`) em 16 ficheiros de layout e em `structural.rs`. Correu-se `crystalline-lint --fix-hashes .`, que actualizou os hashes; o linter voltou a zero violations.

---

## Limitações / comportamentos deliberados

- `#show par: set block(spacing: ..)` continua a emitir warning e não regista a regra; este scope-out não foi alterado.
- A paragraph realization aplica apenas regras `NodeKind::Par`. Regras de texto continuam a ser aplicadas eager no markup, evitando dupla aplicação.
- A morfologia interna do parágrafo (`Content::Par` envolvendo `body`) é específica do cristalino; a paridade é ao nível da linguagem (semântica/sintaxe/morfologia do conteúdo), não ao nível da estrutura de dados Rust (ADR-0107).

---

## Referências

- `00_nucleo/materialization/typst-passo-863.md`
- `00_nucleo/prompts/engine/eval.md`
- `00_nucleo/prompts/entities/content.md`
- `00_nucleo/prompts/entities/show.md`
- `01_core/src/engine/eval/rules.rs`
- `01_core/src/entities/content.rs`
- ADR-0107 — Paridade é com a linguagem, não com a mecânica.
- ADR-0109 — Atomização: lógica de render no arquivo da unidade na sua camada.
