# Relatório — typst-passo-806 (achado P798 #12): `model` — `#par[...]` como função dá `unknown variable: par`

**Data:** 2026-07-21
**Executor:** Kimi Code (a pedido do utilizador, nesta conversa — prompt lido de `00_nucleo/materialization/typst-passo-806.md`)
**Proveniência das medições:** commit base `0661aef91c2ebc80d754d59e936c3a6350bfd543`. Sonda "antes" com working tree contendo P799–P805a (zonas não relacionadas). Validação "depois" com working tree não commitado: P799–P806. Hora da validação: 2026-07-21 ~17:40 -0300.
**Binários:** `./target/release/typst` (rebuild 17:40), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Sonda (antes)

Comandos: `./target/release/typst -o <out>.pdf <fonte>.typ 2>&1` / `lab/typst-original/target/release/typst compile <fonte>.typ <out>.pdf 2>&1`.

| Fonte | Cristalino (antes) | Vanilla |
|---|---|---|
| `#par[conteúdo de teste]` | `error: unknown variable: par` | `conteúdo de teste` |
| `#par()` | — | `error: missing argument: body` |
| `#par(5)` | — | `error: expected content, found integer` |
| `#par(justify: true)[...]` | — | compila |

## Pontos exactos do código

**Vanilla** — `lab/typst-original/crates/typst-library/src/model/par.rs`: `ParElem` (elemento invocável) com `leading`, `spacing`, `justify`, `justification-limits`, `linebreaks`, `first-line-indent`, `hanging-indent` e `body` posicional obrigatório.

**Cristalino**: `Content::Par` **não existia** — parágrafos são implícitos (texto plano em `Sequence`; `01_core/src/entities/element_kind.rs:63` regista "parágrafo é texto plano numa Sequence"). `#set par(leading:)` já existia (`eval/rules.rs:1191`, canal custom `"par.leading"`). O nome `par` não estava registado na `Scope` global — daí `unknown variable`.

**Nota pedida pelo passo**: `Content::Par` **não existia** — mas não foi preciso criá-la: a arquitectura cristalina (parágrafos implícitos) torna o body-devolvido-directamente equivalente no caso standalone. Criar `Content::Par` ficou registado no L0 como candidato futuro apenas para o caso mid-paragraph (block-level break do vanilla).

## Passo 2 — Implementação

L0 actualizado primeiro: `00_nucleo/prompts/engine/stdlib/structural.md`, nova secção `native_par(body, leading:?)` — P806. Lint: "Nothing to fix" no `--fix-hashes` (ficheiros referenciam `structural.md` sem hash próprio em falta; `crystalline-lint .` exit 0).

- `01_core/src/engine/stdlib/structural.rs`: nova `native_par` — body `Content`/`Str`; `missing argument: body` (literal vanilla, medido); `expected content, found {type_name()}`; `leading:` Length → `Content::Styled` com custom `"par.leading"` (mesmo canal do `#set par`); `justify`/`spacing`/`linebreaks`/`first-line-indent`/`hanging-indent`/`justification-limits` aceites e ignoradas (funções nativas não têm `Sink` — limitação registada no L0); named desconhecido → erro.
- `01_core/src/engine/eval/mod.rs`: `scope.define("par", ...)` em `make_stdlib`.
- `01_core/src/engine/stdlib/mod.rs`: re-export de `native_par`.

Nota de processo (disciplina): neste passo a implementação precedeu os testes unitários por minutos (inversão); a prova "antes" do comportamento E2E já existia (`unknown variable: par`, medido na sonda) e os 7 testes cobrem a função nova, que não existia antes (não compilava). Fica registado.

## Passo 3 — Validação (depois)

Mesmos comandos:

- `#par[conteúdo de teste]` → `conteúdo de teste` == vanilla ✓
- `Texto antes\n\n#par(justify: true)[justificado]\n\nTexto depois` → output igual ao vanilla ✓ (`justify` aceite)
- `#par()` → `error: missing argument: body` ✓ (literal vanilla)
- `#par(5)` → `error: expected content, found int` — vanilla diz "integer": divergência pré-existente de nomes de tipo do projecto (`type_name()` "int"), registada, fora de âmbito.
- Observação: os erros saem com local `<detached>` (convenção actual das nativas — `Span::detached()`), não com a linha/coluna do vanilla; registado.

Testes novos (7, em `structural.rs`): body content/str devolvido, `leading:` embrulha em Styled custom, `justify` aceite e ignorado, erros (sem body literal vanilla, tipo errado, named desconhecido).

Suíte `typst-core` (`cargo test -p typst-core --lib`):
- ANTES (fim de P805): **4329** passed + 1 ignored (total 4330).
- DEPOIS: **4336** passed; 0 failed; 1 ignored (total 4337 = +7 testes novos ✓).

Lint: `crystalline-lint .` → exit 0, zero violações.
