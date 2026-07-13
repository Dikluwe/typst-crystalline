# P735 — Paridade de produção: namespaces `emoji` e `pdf`

## Proveniência da medição (regra 2026-07-05)

- **Commit base:** `a1111e8e4426fefd81486fb104f1d8e77034cac6` ("P734: preenche hash do commit no relatório")
- **Estado na medição final:** working tree não commitado; `git diff HEAD --stat`:
  - `01_core/src/rules/eval/mod.rs` (+6), `01_core/src/rules/eval/tests.rs` (+61), `01_core/src/rules/stdlib/mod.rs` (+6)
  - ficheiros novos (untracked): `00_nucleo/prompts/rules/stdlib/emoji.md`, `00_nucleo/prompts/rules/stdlib/pdf.md`, `01_core/src/rules/stdlib/emoji.rs`, `01_core/src/rules/stdlib/pdf.rs`
- **Hora da validação final:** 2026-07-13 ~20:35 (-03)
- **Binário vanilla de referência:** `lab/typst-original/target/release/typst` (0.14, features padrão)
- **Binário cristalino:** `./target/release/typst` (rebuild após a implementação)

## Sonda (medida antes de decidir — ADR-0108)

| Sondagem | Vanilla | Cristalino (pré-P735) |
|---|---|---|
| `#type(emoji)` | `module` | erro "unknown variable: emoji" |
| `#type(pdf)` | `module` | erro "unknown variable: pdf" |
| `#emoji.face` | 😀 (U+1F600) | — |
| `#type(pdf.attach)` | `function` | — |
| `#type(pdf.artifact)` | `function` | — |
| `#type(pdf.table.summary)` | erro (não existe) | — |

Achados da sonda:

1. O namespace `pdf` do vanilla, **no binário padrão**, contém apenas `attach` e `artifact`. As funções `pdf.table.summary` / `pdf.table.header-cell` / `pdf.table.data-cell` estão gated na feature `A11yExtras` (off por omissão) — não existem no binário de referência, portanto **não são lacuna** (paridade acidental, como `html` em P731).
2. Fonte do módulo `emoji` vanilla: crate `codex-0.2.0`, ficheiro `src/modules/emoji.txt` — 766 entradas top-level. Medido via script: **529** têm valor de um único codepoint (sem duplicados). As restantes são sequências multi-codepoint (ZWJ, modificadores de tom de pele, bandeiras regionais) — o tipo `Symbol` cristalino guarda um `char`, logo essas entradas são scope-out.
3. `emoji.face` → 😀 no vanilla embora `face` não seja entrada de 1 codepoint no codex (tem variantes); o vanilla resolve o nome simples para U+1F600. Adicionado manualmente.

## Decisão

- `emoji` → `Value::Module` ("emoji") com as 529 entradas de 1 codepoint do codex-0.2.0 + `face` → 😀 (**530** entradas), cada uma `Value::Symbol`.
- `pdf` → `Value::Module` ("pdf") com:
  - `pdf.attach(...)` → **scope-out com erro explícito** ("o exportador PDF cristalino não suporta ficheiros embutidos (scope-out)") — o observável é a mensagem de erro; silenciar seria pior (falso sucesso).
  - `pdf.artifact(body)` → **passthrough do body** (paridade de render: o vanilla renderiza o body; a marcação de artefacto no tag tree é scope-out global do exportador).

L0: `00_nucleo/prompts/rules/stdlib/emoji.md` e `00_nucleo/prompts/rules/stdlib/pdf.md` (hashes fixados via `crystalline-lint --fix-hashes .`).

## Testes (fail-first confirmado: 5/5 falhavam antes da implementação)

Em `01_core/src/rules/eval/tests.rs`:

1. `p735_type_emoji_e_pdf_sao_module` — `type(emoji)` e `type(pdf)` → `Type::Module`
2. `p735_emoji_face_e_entradas_da_tabela` — `emoji.face`/`emoji.ant`/`emoji.banana` → 😀/🐜/🍌
3. `p735_pdf_fields_sao_funcoes` — `type(pdf.attach)`/`type(pdf.artifact)` → `Type::Function`
4. `p735_pdf_attach_e_scope_out_com_erro` — erro contém "scope-out" ou "não suporta"
5. `p735_pdf_artifact_passthrough_do_body` — body passa inalterado como Content

Nota de execução: o teste 4 usava inicialmente `bytes("hi")` como argumento; o cristalino não tem constructor `bytes` e o erro vinha do argumento, não do `attach`. Substituído por string (`"hi.txt"`) — o scope-out do `attach` é independente do tipo do argumento (named-args rejeitados primeiro, paridade `expect_no_named`).

## Implementação

- `01_core/src/rules/stdlib/emoji.rs` (novo): `EMOJI_TABLE: &[(&str, char)]` (530 entradas, tabela gerada do codex-0.2.0) + `build_emoji_module()` (mirror de `build_sym_module`, P731).
- `01_core/src/rules/stdlib/pdf.rs` (novo): `make_pdf_module()` + `native_pdf_attach` (erro scope-out) + `native_pdf_artifact` (passthrough, named-args rejeitados, aridade 1).
- `01_core/src/rules/stdlib/mod.rs`: `mod emoji; mod pdf;` + `pub use` dos builders.
- `01_core/src/rules/eval/mod.rs`: `scope.define("emoji", ...)` e `scope.define("pdf", ...)` no scope global.

## Validação

| Verificação | Resultado |
|---|---|
| `cargo test -p typst-core p735` | **5 passed**, 0 failed |
| `cargo test --workspace` | **4742 passed**, 0 failed (pré-P735: 4737; +5) |
| `crystalline-lint .` | 0 violations, 0 drift |
| `cargo build --release` | OK (16.6s) |
| E2E (`#type(emoji) #type(pdf) #emoji.face #emoji.ant #emoji.banana #type(pdf.attach) #type(pdf.artifact) Antes #pdf.artifact[arte] Depois`) | `module module 😀🐜🍌 function function Antes arte Depois` |
| E2E (`#pdf.attach("hi.txt")`) | erro explícito "pdf.attach: o exportador PDF cristalino não suporta ficheiros embutidos (scope-out)", exit 1 |

## Achados

- Item "Namespaces `emoji` e `pdf` ausentes" (aberto desde a sonda de P731) **fechado**.
- Novos scope-outs registados em `achados-adiados-cetz.md`:
  - emojis multi-codepoint (ZWJ, tons de pele, bandeiras) — `Symbol` guarda um `char`;
  - `pdf.attach` sem embedding no exportador;
  - tagging de `pdf.artifact` (scope-out global do exportador);
  - `pdf.table.*` — gated em `A11yExtras` no vanilla, não é lacuna.
