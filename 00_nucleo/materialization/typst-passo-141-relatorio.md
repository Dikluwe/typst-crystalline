# Passo 141 — Relatório (array fallback chain; ADR-0055 `IMPLEMENTADO`)

**Data**: 2026-04-24
**Precondição**: Passo 140B encerrado; 1111 total tests; zero
violations; 55 ADRs com ADR-0055 em `PROPOSTO`; DEBT-52 em 5/8
gaps resolvidos; hash L0 `prompts/infra/pipeline.md` =
`367f8790`.
**Natureza**: passo **L3** (corpo de `resolve_font`) + edição
**L0** (`prompts/infra/pipeline.md`). **Zero alteração em L1**.
**Zero crates novas**.
**ADR**: **ADR-0055** transita `PROPOSTO → IMPLEMENTADO` neste
passo (par 140B+141 completa a paridade básica).

---

## Sumário executivo

`resolve_font` deixou de ser single-family. Em vez de tentar
apenas `font_list.as_slice().first()?`, agora itera todas as
famílias em ordem, devolvendo os bytes da primeira família a
completar `FontBook::select` + `World::font`. Comportamento
vanilla directo para `#set text(font: ("A", "B", "C"))`.

**Gap 6 do DEBT-52 fechado** — array fallback chain materializada.
Junto com o gap 5 (140B), forma a paridade básica da ADR-0055,
que **transita a `IMPLEMENTADO`**.

**Tests**: 1111 → **1116** (+5: 4 unit `resolve_font_lista_*`
+ 1 integração L3 `font_wiring_array_fallback_*`). Os 7 testes
unit pré-existentes do `pipeline::tests` (3 do 140B no
`resolve_font` + 4 do `first_font_from_doc`) continuam verdes
sem alteração — o teste `match_primeiro` corresponde naturalmente
ao caso "lista de tamanho 1, match imediato". `cargo build`
limpo. `crystalline-lint .` zero violations.

---

## 141.2 — Alteração em `resolve_font`

Diff central em `03_infra/src/pipeline.rs`:

```diff
- /// Resolve a primeira família de `font_list` ...
- /// Apenas a primeira família é tentada — array fallback chain
- /// é Passo 141.
+ /// Itera `font_list.as_slice()` em ordem. Para cada família,
+ /// consulta `font_book.select(name, &FontVariant::default())`;
+ /// se devolve `Some(index)`, chama `world.font(index)`; primeira
+ /// família que completa ambos os passos vence.
+ ///
+ /// Cenário patológico (índice stale: `select` devolve `Some` mas
+ /// `world.font` devolve `None`) **continua** a tentar as famílias
+ /// seguintes — não curto-circuita.
  fn resolve_font(
      font_list: &FontList,
      font_book: &FontBook,
      world:     &dyn World,
  ) -> Option<Vec<u8>> {
-     let first   = font_list.as_slice().first()?;
      let variant = FontVariant::default();
-     let index   = font_book.select(&first.name, &variant)?;
-     let font    = world.font(index)?;
-     Some(font.as_slice().to_vec())
+     for family in font_list.as_slice() {
+         if let Some(index) = font_book.select(&family.name, &variant) {
+             if let Some(font) = world.font(index) {
+                 return Some(font.as_slice().to_vec());
+             }
+         }
+     }
+     None
  }
```

**Forma escolhida**: `for` explícito com `if let` aninhado. Razão:
permite deixar visível o branching de "stale index continua a
tentar". `iter().find_map` curto-circuitaria em `select` ou
`world.font` retornando `None`, mas não distingue os dois casos
no fluxo — não há perda de funcionalidade prática mas o `for`
mostra a semântica directamente. Decisão de estilo (ver "Que
pode sair errado" do enunciado, item 2).

**Assinatura preservada**: `fn resolve_font(&FontList, &FontBook,
&dyn World) -> Option<Vec<u8>>`. Zero impacto em callers.

---

## 141.3 — Testes unitários (4 novos)

Adicionados a `pipeline::tests` em
`03_infra/src/pipeline.rs:362+`:

| Teste | Cenário | Mock setup | Assertion |
|-------|---------|------------|-----------|
| `resolve_font_lista_match_indice_0` | `["A", "B", "C"]` | `FontBook` com "A" no índice 0 | bytes de "A" |
| `resolve_font_lista_match_indice_1` | `["X", "B", "C"]` | `FontBook` só com "B" | bytes de "B" |
| `resolve_font_lista_match_indice_2` | `["X", "Y", "C"]` | `FontBook` só com "C" | bytes de "C" |
| `resolve_font_lista_sem_match_devolve_none` | `["X", "Y", "Z"]` | `FontBook` só com "Outra" | `None` |

Helper privado novo:

```rust
fn font_list_multi(names: &[&str]) -> FontList {
    let families = names.iter()
        .map(|n| FontFamily::new(EcoString::from(*n)))
        .collect();
    FontList::new(families).expect("lista não-vazia")
}
```

`FontFamily` re-importado no módulo `tests` (tinha sido removido
no 140B por estar não-usado). `EcoString` já estava em scope via
`use ecow::EcoString` introduzido no 140B.

Mock `FontMockWorld` reutilizado sem alteração — única mudança
é a estrutura do `FontBook::infos()` (uma família por teste em
vez de duas) e os bytes injectados (`0xAA`/`0xBB`/`0xCC`/`0xFF`
para distinguir entre asserts).

---

## 141.4 — Teste de integração L3 (1 novo)

Ficheiro: `03_infra/src/integration_tests.rs` (mod
`integration`, junto aos 4 testes `font_wiring_*` do 140B).

| Teste | Cobre |
|-------|-------|
| `font_wiring_array_fallback_primeira_falha_segunda_vence` | `#set text(font: ("FontQueNaoExiste", <família-real>))` produz PDF com marker `CrystallineFont` (segunda família embutida) e **não** contém `/BaseFont /Helvetica`. |

Mesmo padrão de `discover_any_system_fonts() → eprintln!("[skip]")
→ early return` do 140B quando ambiente não tem TTFs canónicos.
No Linux deste passo (DejaVu + Liberation + Open Sans + Noto
instalados), a probe encontrou família real e o teste correu
assertions reais.

**Assert negativa adicional** (`!blob.contains("/BaseFont
/Helvetica")`) reforça que o caminho CIDFont foi tomado, não o
fallback — relevante porque `Helvetica` aparece nas dictionaries
de Type1 só quando o pipeline cai em `export_pdf` (sem font).

---

## 141.6 — ADR-0055 transita `IMPLEMENTADO`

Diff em `00_nucleo/adr/typst-adr-0055-font-consumer-cidfont.md`:

```diff
- **Status**: `PROPOSTO`
+ **Status**: `IMPLEMENTADO`
```

Adicionada secção "Materialização" no fim do ADR com:
- Passo 140B → wiring single-font (decisão 3, gap 5).
- Passo 141 → array fallback chain (decisão 4, gap 6).
- Decisões 5/6/7 explicitamente scope-out (não bloqueiam fecho).
- Variant-aware registada como limitação conhecida → ADR-0055bis
  candidato natural se priorizada.

**`00_nucleo/adr/README.md` não tocado**: a tabela de status
nesse ficheiro só lista até ADR-0037; ADR-0055 nunca foi lá
adicionada (consistente com o estado dos restantes ADRs
0038-0054). O spec autoriza este tratamento ("se tiver índice
de status").

---

## 141.7 — Edição L0 `prompts/infra/pipeline.md`

Duas secções actualizadas:

1. **"Pipeline `eval → introspect → layout → (dispatch
   export)`"** — descrição do dispatch agora explicita
   "**Itera todas as famílias** ... **Primeira família a
   completar ambos os passos vence**" e separa as decisões 3
   (single-font per document) e 4 (array fallback chain) com
   referência aos passos materializadores. Marca ADR-0055 como
   `IMPLEMENTADO`.

2. **"Helpers privados de dispatch"** — corpo da descrição do
   `resolve_font` reescrito para o loop (em vez de "primeira
   família"). Descreve o cenário patológico stale-index
   explicitamente. Final da secção fecha com nota de scope-out
   (multi-font Passo 142, ADR-0055bis variant-aware).

---

## 141.8 — Hash L0 propagado

```
sha256sum 00_nucleo/prompts/infra/pipeline.md
00e4ebd3...  (8 chars iniciais)
```

(Hash extraído via `crystalline-lint .` que reporta drift V5
quando código está desactualizado — `Hash L0: 00e4ebd3`.)

Header de `03_infra/src/pipeline.rs`:

```diff
- //! @prompt-hash 367f8790
+ //! @prompt-hash 00e4ebd3
```

`@updated 2026-04-24` mantém-se (mesmo dia do 140B).

`crystalline-lint .` confirma zero violations após propagação.

---

## DEBT-52 — actualização

Diff em `00_nucleo/DEBT.md` (secção DEBT-52, "Âmbito"):

```diff
- - [ ] Consumer `font` array (fallback chain).
+ - [x] Consumer `font` array (fallback chain).
+       **Resolvido no Passo 141** — ...
+       **Fase C básica completa.**
```

Gaps fechados: 5 → **6**. Restantes (ambos opcionais segundo
ADR-0054):
- Gap 7 — lang hyphenation (Passo 143 candidato; requer crate).
- Gap 8 — font dict (ADR-0054bis condicional; requer `regex`
  em L1).

Fase C **básica** completa. Fecho de DEBT-1 é **acção
separada** — não acontece neste passo (ver "Próximo passo" e
notas operacionais do enunciado).

---

## Limitações preservadas

Sem alteração face ao 140B:

1. **Selecção variant-aware** — `FontVariant::default()`
   continua a ser usado. ADR-0055bis candidata.
2. **Multi-font per document** — Passo 142 opcional permanece
   no roadmap.
3. **Subsetting** — out-of-scope DEBT-1; candidato ADR-0056
   futura.
4. **Shaping (rustybuzz)** — DEBT-53 candidato XL, não aberto.
5. **Lang/hyphenation** — gap 7 inalterado; Passo 143 opcional.
6. **Reprodutibilidade de testes em CI** — limitação 7 do
   140B persiste: tests `font_wiring_*` 1, 4 e o novo do 141
   dependem de fonts no sistema.

Nenhuma limitação nova introduzida pelo Passo 141.

---

## Próximo passo: encerrar DEBT-1 (acção separada)

Conforme enunciado e notas operacionais do 141:

> **Fecho de DEBT-1**: decisão pós-141. Candidatos:
> - Passo curto dedicado com relatório de fecho formal.
> - Entrada directa em `DEBT.md` movendo DEBT-1 para secção
>   "encerrados" (Secção 2).
> - Adiar fecho até multi-font (142) ou hyphenation (143) se
>   priorizados brevemente.
> Recomendação: passo curto dedicado, para registar claramente
> o cumprimento de ADR-0054.

Alternativas posteriores:
- **Passo 142** (M, opcional) — multi-font per document;
  invalida limitação 2.
- **Passo 143** (M, opcional) — lang hyphenation; gap 7 do
  DEBT-52.
- **DEBT-53** (XL, candidato) — rustybuzz integration.
- **ADR-0055bis** (se necessário) — selecção variant-aware.

---

## Verificação final

| Item | Estado |
|------|--------|
| `resolve_font` itera toda a `FontList` | ✅ |
| 4 unit tests `resolve_font_lista_*` | ✅ |
| 7 unit tests pré-existentes do `pipeline::tests` | ✅ verdes |
| 1 integration test `font_wiring_array_fallback_*` | ✅ |
| 4 integration tests `font_wiring_*` do 140B | ✅ verdes |
| `cargo test --workspace --lib` | 1095 passed (+5 vs 140B; 6 ignored pré-existentes) |
| `crystalline-lint .` | ✅ zero violations |
| Hash L0/L3 sincronizado | ✅ `00e4ebd3` |
| L1 de domínio intacto | ✅ (git: zero diff em `01_core/`) |
| DEBT-52 gap 6 marcado `[x]` | ✅ (5/8 → 6/8 gaps resolvidos) |
| ADR-0055 transita a `IMPLEMENTADO` | ✅ |
| Secção "Materialização" adicionada à ADR-0055 | ✅ |
| L0 `prompts/infra/pipeline.md` actualizado | ✅ |
