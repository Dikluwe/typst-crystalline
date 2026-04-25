# Relatório — Passo 156E: pagebreak manual (Layout Fase 1 sub-passo 3)

**Data**: 2026-04-25.
**Natureza**: passo **substantivo escopo S+ agregado**;
**terceira aplicação consecutiva** de ADR-0061 (Layout Fase X
roadmap, status `PROPOSTO`); **halfway point Fase 1**
atingido (50% cobertura Layout).
**Spec**: `00_nucleo/materialization/typst-passo-156e.md`.

**Outputs materiais**:
- 1 variant novo em `Content`: `Pagebreak { weak, to }`.
- Tipo enum `Parity { Even, Odd }` em
  `01_core/src/entities/parity.rs`.
- Stdlib `#pagebreak(weak: false, to: ?)`.
- Helper `extract_parity` em `stdlib/layout.rs`.
- Cobertura exaustiva de arms em 4 ficheiros L1.
- 22 tests novos (alvo era 15-22; meta atingida no topo).
- L0 prompts: novo `entities/parity.md`; secção pagebreak em
  `entities/content.md`; hashes propagados.
- Inventário 148 actualizado (Layout 8/0/3/7/0=18 →
  9/0/3/6/0=18; cobertura 44% → **50%** — halfway point).
- README ADRs entrada P156E.
- Este relatório.

---

## §1 — Sumário executivo

P156E continuou a sequência granular Layout Fase 1 iniciada
em P156C (pad+hide) e P156D (h+v). Adicionou um variant
`Content::Pagebreak` e um tipo enum `Parity`, fechando metade
da Fase 1 declarada em ADR-0061 §6.1 (sem footnote area, que
sai da Fase 1 per decisão humana).

Trabalho material:
- **`Parity` enum** novo em `entities/parity.rs` com método
  `matches(page_number) -> bool`. Análogo a `Sides<T>`
  criado em P156C — infraestrutura genérica reusável.
- **`Content::Pagebreak { weak: bool, to: Option<Parity> }`**
  com layouter completo: força `flush_line` se necessário,
  reusa `Layouter::new_page` (`cursor.rs:128`), e se `to`
  exige paridade específica que não bate, insere segunda
  `new_page()` para ajustar.
- **Cobertura exaustiva** em todos os pattern-match `Content`
  do L1: Pagebreak tratado como leaf (clone em `map_*`,
  `materialize_time`; no-op em `walk`); `is_empty` retorna
  sempre `false` (event observável mesmo sem body, cf.
  Divider em P154B); `plain_text` vazio.
- **stdlib `#pagebreak(weak: false, to: ?)`** sem
  argumentos posicionais; helper `extract_parity` aceita
  apenas Str `"even"`/`"odd"`; outros valores ou tipos
  rejeitados com erro hard.

**Tests**: **1192 → 1214** (+22). Layout cobertura: **44%
→ 50%** (8/18 → 9/18) — halfway point Fase 1 atingido.
User-facing total: 56% → 57%.

**ADR-0061 mantém-se `PROPOSTO`** per decisão humana
2026-04-25 (anotação cumulativa após Fase 1 completa).

**Padrão granular confirmado pela terceira vez consecutiva**:
zero reformulações mid-passo, build clean ao primeiro try,
tests à primeira tentativa. Hipótese da decisão humana
2026-04-25 (12 passos granulares) está empiricamente
reforçada com N=3 aplicações consecutivas.

---

## §2 — Inventário pré-materialização (sub-passo 156E.1)

### §2.1 Estado pré-P156E verificado

```bash
grep -E "Pagebreak|Parity" 01_core/src/entities/    # zero hits
ls 01_core/src/entities/parity.rs 2>/dev/null       # NÃO existe
grep "native_pagebreak" 01_core/src/rules/stdlib/   # zero hits
grep "fn new_page" 01_core/src/rules/layout/cursor.rs   # existe (linha 128)
```

Confirmações:
- `Content` enum com **47 variants** pós-P156D (último: `VSpace`).
- **Sem** `Parity` enum.
- **Sem** `Content::Pagebreak`.
- **Sem** `native_pagebreak` em stdlib.
- `Layouter::new_page()` existe em `cursor.rs:128` como
  `pub(super)` — directamente reusável a partir de
  `layout/mod.rs::layout_content`.

### §2.2 Mecânica de page break verificada

`Layouter::new_page()` em `cursor.rs:128`:
```rust
pub(super) fn new_page(&mut self) {
    let page = Page {
        width:  self.page_config.width,
        height: self.page_config.height,
        items:  std::mem::take(&mut self.current_items),
    };
    self.pages.push(page);
    self.cursor_x = Pt(self.page_config.margin);
    self.line_start_x = Pt(self.page_config.margin);
    let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
    self.cursor_y = Pt(self.page_config.margin) + ascender;
}
```

Comportamento confirmado: commits `current_items` (mesmo
vazios) numa Page, push para `pages`, reseta cursor.
Reusável directamente para `Pagebreak`.

---

## §3 — Tipo `Parity` — forma final + diff

```rust
// 01_core/src/entities/parity.rs (novo, 80 linhas com 3 tests)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    Even,
    Odd,
}

impl Parity {
    pub fn matches(self, page_number: usize) -> bool {
        match self {
            Self::Even => page_number % 2 == 0,
            Self::Odd  => page_number % 2 == 1,
        }
    }
}
```

**Atributos**:
- 2 variantes apenas (sem `Auto` interno; representação
  externa via `Option<Parity>` — `None` == Auto).
- `Copy` derivado.
- `Eq` derivado.
- Método `matches(page_number)` com 1-indexing (página 1 ímpar).

**Tests internos**: 3 (matches_even, matches_odd, partial_eq).

**L0 prompt** criado em `00_nucleo/prompts/entities/parity.md`
(hash `ce1c056c`); hash do código `af8490cb`.

**Registado** em `01_core/src/entities/mod.rs::pub mod parity;`.

---

## §4 — Variant `Content::Pagebreak` — forma final + diff

```rust
// 01_core/src/entities/content.rs (após VSpace)
Pagebreak {
    weak: bool,
    to:   Option<Parity>,
},
```

**Atributos**:
- `weak: bool` — armazenado mas comportamento de collapse
  adiado (consistente com P156D HSpace/VSpace).
- `to: Option<Parity>` — `None` == Auto (sem ajuste);
  `Some(parity)` força próxima página à paridade.

**Construtor Rust**: `Content::pagebreak(weak, to)`.

**Stdlib**: `#pagebreak(weak: false, to: ?)` em
`stdlib/layout.rs::native_pagebreak`.

**Cobertura arms adicionados**:
- `is_empty`: sempre `false` (event observável mesmo sem
  body; cf. Divider em P154B).
- `plain_text`: `String::new()` (event sem texto).
- `PartialEq::eq`: comparação 2 fields (weak + to).
- `map_content`: terminal (clone directo).
- `map_text`: idem terminal.
- `materialize_time` (introspect): clone (leaf).
- `walk` (introspect): no-op (sem body, sem effect counters).
- `layout_content`: flush_line + new_page + verifica paridade
  + insere new_page extra se necessário.
- `measure_content_constrained`: `(0.0, 0.0)` (event sem
  dimensões dentro de cell).

---

## §5 — Cobertura exaustiva de arms

| Função | Ficheiro | Tratamento Pagebreak |
|--------|----------|---------------------|
| `Content::is_empty()` | `entities/content.rs` | sempre `false` |
| `Content::plain_text()` | `entities/content.rs` | `String::new()` |
| `PartialEq::eq` | `entities/content.rs` | comparação 2-fields |
| `Content::map_content` | `entities/content.rs` | terminal (clone) |
| `Content::map_text` | `entities/content.rs` | terminal (clone) |
| `materialize_time` | `rules/introspect.rs` | clone (leaf) |
| `walk` | `rules/introspect.rs` | no-op (leaf) |
| `layout_content` | `rules/layout/mod.rs` | flush_line + new_page + parity check |
| `measure_content_constrained` | `rules/layout/mod.rs` | `(0.0, 0.0)` |

**Verificação**: `cargo build -p typst-core` clean (sem
warnings de variantes não cobertas).

---

## §6 — `native_pagebreak` + `extract_parity` — assinaturas + registo

### §6.1 `extract_parity` helper

```rust
fn extract_parity(value: &Value) -> SourceResult<Parity> {
    match value {
        Value::Str(s) => match s.as_str() {
            "even" => Ok(Parity::Even),
            "odd"  => Ok(Parity::Odd),
            other  => Err(...),  // erro hard
        },
        other => Err(...),  // erro hard se não-string
    }
}
```

### §6.2 `native_pagebreak`

```rust
pub fn native_pagebreak(_ctx, args, _world, _file, _fig)
    -> SourceResult<Value>
{
    if !args.items.is_empty() { return Err(...); }  // sem posicionais

    let mut weak = false;
    let mut to: Option<Parity> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => match value {
                Value::Bool(b) => weak = *b,
                _ => return Err(...),  // tipo errado
            },
            "to" => to = Some(extract_parity(value)?),
            _ => return Err(...),  // named arg desconhecido
        }
    }

    Ok(Value::Content(Content::Pagebreak { weak, to }))
}
```

### §6.3 Registo em `make_stdlib`

```rust
scope.define("pagebreak",
    Value::Func(Func::native("pagebreak", native_pagebreak)));
```

Re-export em `stdlib/mod.rs`:

```rust
pub use crate::rules::stdlib::layout::{
    native_align, native_grid, native_h, native_hide, native_pad,
    native_page, native_pagebreak, native_place, native_v,
};
```

Stdlib funcs: 36 → **37** (+1).

---

## §7 — Layouter — diff

### §7.1 `Content::Pagebreak` em `layout_content`

```rust
Content::Pagebreak { weak: _, to } => {
    // 1. Termina linha em curso.
    if self.cursor_x.0 > self.line_start_x.0 {
        self.flush_line();
    }
    // 2. Força nova página.
    self.new_page();
    // 3. Verifica paridade se requerida.
    if let Some(parity) = to {
        let next_page_number = self.pages.len() + 1;
        if !parity.matches(next_page_number) {
            self.new_page();  // insere página vazia para ajustar
        }
    }
}
```

Notas:
- Reusa `Layouter::new_page` (`cursor.rs:128`) — sem novo
  método criado.
- `weak` ignorado neste passo (collapse defere).
- Página vazia inserida para ajustar paridade usa
  `page_config` actual (mesmas dimensões; sem header/footer
  porque Page actual não os tem).

### §7.2 `Content::Pagebreak` em `measure_content_constrained`

```rust
Content::Pagebreak { .. } => (0.0, 0.0),
```

Em grid measurement, pagebreak é event sem dimensões
(consume zero largura/altura na célula).

---

## §8 — Tests adicionados (lista + contagens)

### §8.1 Em `entities/parity.rs::tests` (3)

1. `parity_matches_even_pages` — modulo correto para Even.
2. `parity_matches_odd_pages` — modulo correto para Odd.
3. `parity_partial_eq` — comparação enum.

### §8.2 Em `entities/content.rs::tests` (5)

4. `pagebreak_constructor` — `Content::pagebreak(false, None)` +
   `(true, Some(Parity::Even))`.
5. `pagebreak_is_empty_returns_false` — proxy semantic.
6. `pagebreak_plain_text_vazio` — sem texto.
7. `pagebreak_partial_eq` — comparação completa (weak + to).
8. `pagebreak_map_text_preserva` — leaf preservado.

### §8.3 Em `stdlib/mod.rs::tests` (10)

9. `native_pagebreak_defaults` — sem args (weak=false, to=None).
10. `native_pagebreak_com_weak_true` — weak Bool.
11. `native_pagebreak_com_to_even` — to "even".
12. `native_pagebreak_com_to_odd` — to "odd".
13. `native_pagebreak_combina_weak_e_to` — caso composto.
14. `native_pagebreak_rejeita_to_invalido` — to "middle" → Err.
15. `native_pagebreak_rejeita_to_nao_string` — Int rejeitado.
16. `native_pagebreak_rejeita_named_arg_desconhecido` — Err.
17. `native_pagebreak_rejeita_argumento_posicional` — Err.
18. `native_pagebreak_rejeita_weak_nao_bool` — tipo errado.

### §8.4 Em `layout/tests.rs` (4)

19. `layout_pagebreak_forca_nova_pagina` — A em p1, B em p2.
20. `layout_pagebreak_to_even_quando_ja_par_nao_insere_extra`
    — paridade já bate (p2 par), sem inserção.
21. `layout_pagebreak_to_odd_insere_vazia_se_proxima_seria_par`
    — paridade não bate, insere página vazia (B em p3).
22. `layout_pagebreak_to_even_insere_vazia_se_proxima_seria_impar`
    — caso composto com 2 pagebreaks (A→p1, B→p3, C→p4).

**Total**: **22 tests novos** (alvo spec era 15-22; meta
atingida no topo). Distribuição equilibrada entre construtor,
stdlib e layout E2E.

**Tests cumulativos**: 1192 → **1214** (+22 = 3 parity +
5 content + 10 stdlib + 4 layout).

---

## §9 — L0 prompts + hashes propagados

### §9.1 L0 novo: `entities/parity.md`

Criado `00_nucleo/prompts/entities/parity.md` (~75 linhas):
- Cabeçalho canónico (camada L1, ficheiro alvo, ADRs).
- Contexto + interface pública + semântica + invariantes.
- Consumers actuais (Content::Pagebreak) + planeados
  (refino Page rico — header/footer per paridade).
- Nota sobre divergência `Smart<Parity>` vanilla →
  `Option<Parity>` cristalino.

Hash do código: `af8490cb`.

### §9.2 L0 actualizado: `entities/content.md`

Secção nova "Variant `Content::Pagebreak` — Passo 156E
(ADR-0061 Fase 1, sub-passo 3)" adicionada após secção
P156D HSpace+VSpace. Inclui:
- Forma estrutural do variant.
- Atributos `weak` + `to` (com referência a Parity).
- Comportamento dos métodos canónicos.
- Renderização layouter (flush_line + new_page + parity
  check).
- Validação em native_pagebreak (sem posicionais; named
  args válidos).
- Construtores Rust + Stdlib.
- Limitações conscientes (weak collapse adiado; página
  vazia sem header/footer; to só string em stdlib;
  pagebreak no início absoluto).
- Decisão arquitectural confirmada (variant novo per
  ADR-0061 Decisão 4).
- Nota sobre tipo `Parity` paralelo.

### §9.3 Headers `@updated` actualizados

- `entities/content.rs`: `@updated 2026-04-25` (já estava).
- `entities/parity.rs`: `@updated 2026-04-25` (novo).

### §9.4 Hashes via `crystalline-lint --fix-hashes .`

```
Fixed 2 files:
  ./01_core/src/entities/content.rs             → b632e841
  ./01_core/src/entities/parity.rs              → af8490cb

Re-running analysis... ✅ 0 drift warnings remaining
```

`entities/content.rs`: `4a7e0a99` (P156D) → **`b632e841`** (P156E).
`entities/parity.rs`: novo, **`af8490cb`**.

---

## §10 — Inventário 148 actualizado

Ficheiro: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.

**Tabela A.5 Layout — linha reescrita**:
- `pagebreak()` (manual): `ausente` ⁵ → **`implementado`** ¹⁰
  (referência: Passo 156E).

**Tabela A — Vista user-facing (resumo)**:
- Linha "Layout" (nota ⁵ ⁶ ⁸ ¹⁰): `8/0/3/7/0=18` →
  **`9/0/3/6/0=18`**.
- Total user-facing: `58/21/22/38/2=141` →
  **`59/21/22/37/2=141`**.
- Cobertura user-facing total: 56% → **57%**.

**Tabela B — Arquitectural**:
- `Content` variants (cristalino) ³ ⁴ ⁷ ⁹ ¹¹: `35/9/3/0/0=47`
  → **`36/9/3/0/0=48`** (+`Pagebreak`).
- `Content` variants (vanilla extra ausentes): `~7` → `~6`.

**Nota nova ¹⁰**: descrita transição P156E com contagens
explícitas; **halfway point Fase 1** marcado.

**Nota nova ¹¹**: descrita actualização de Tabela B.

**§7 entrada 7**: refinamento P156E documentado com lista
das 6 entradas Layout restantes (`box`, `block`, `stack`,
`repeat`, `columns`/`colbreak`, `skew`) e mapeamento para
Fase 2 (block+box+stack) e Fase 3 (columns+repeat+skew).

---

## §11 — README ADRs actualizado

Ficheiro: `00_nucleo/adr/README.md`.

- **Tabela "Estado por ADR"**: ADR-0061 mantém-se `PROPOSTO`.
- **Total**: 61 inalterado.
- **Distribuição**: PROPOSTO 11 inalterado.
- **"Passos-chave"**: entrada nova para P156E com detalhe
  de variant Pagebreak (47 → 48 variants), tipo Parity novo,
  stdlib `#pagebreak`, helper `extract_parity`, cobertura
  arms, layouter diff (reusa `new_page`), tests 1192 → 1214
  (+22), Layout 44% → 50% (halfway point), total user-facing
  56% → 57%, hashes `content.rs` `4a7e0a99` → `b632e841`,
  `parity.rs` `af8490cb`.

---

## §12 — Próximo passo

P156E encerrou-se com **halfway point Fase 1 atingido**
(50% Layout). Próximo passo é **decisão humana** entre
prioridades documentadas em ADR-0061:

- **Opção A — P156F (Layout Fase 1 sub-passo 4)**: skew
  (S, 1 trivial via Content::Transform extension). Cobertura
  → 56% (10/18).

- **Opção B — Fase 2 Layout (block + box + stack)**: M+
  agregado (3 containers ricos). Cobertura → 67%-72%.

- **Opção C — P157 (Model Fase 2 table foundations)**:
  M+ alternativo (per ADR-0060 renumerada).

- **Opção D — Footnote area + passo dedicado footnote
  Model**: sub-fase prioritária explícita declarada em
  ADR-0061 Decisão 5.

- **Opção E — Outra prioridade humana**.

**Recomendação descritiva** (derivada do historiograma P156A
§4.1 + experiência P156C+P156D+P156E): cadência granular
1-2 features por passo está a funcionar consistentemente
com N=3 aplicações (zero reformulações em todas). P156F
(skew) é a próxima sub-fase mais natural se humano quiser
fechar Fase 1 antes de entrar em Fase 2 (containers ricos).

---

## §13 — Limitações registadas

1. **`weak` collapse semantic não implementado** (consistente
   P156D). Vanilla colapsa weak adjacentes; cristalino
   mantém ambos. Refino futuro se priorizado.

2. **Página vazia inserida para ajustar paridade não tem
   cabeçalho/rodapé** (porque `Page` cristalino não os tem).
   Refino futuro com Page rico (Fase 3 ADR-0061).

3. **`to` aceita só string em stdlib** (vanilla aceita
   `Symbol::even` sem aspas). Refino se priorizado.

4. **Pagebreak no início absoluto do documento** (antes de
   qualquer texto) cria página 1 vazia + conteúdo na página
   2. Aceitável; case patológico raro. Não é "bug" — é
   semantic correcta de "force flush + new page".

5. **Sem show rules `#show pagebreak: ...`** neste passo
   (consistente com adiamento P154B/P155/P156C/P156D).

6. **Pagebreak aninhado em containers** (e.g. `pad(pagebreak())`
   ou `hide(pagebreak())`) tem comportamento bem definido
   por estrutura (cobertura recursiva), mas não testado
   explicitamente — case patológico raro.

7. **`new_page()` é `pub(super)`** — acessível de
   `layout_content` em `mod.rs` (sibling de `cursor.rs` no
   mesmo package). Sem necessidade de promover visibilidade.

8. **ADR-0061 mantém `PROPOSTO`** (per decisão humana).
   Anotação cumulativa após Fase 1 completa.

---

## §14 — Verificação final

Critérios da spec P156E (§Verificação):

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: **1214 passed**;
   0 failed; 6 ignored. (975 typst-core + 215 integration
   + 24 outros; 1192 → 1214 = +22).
3. ✅ `crystalline-lint .`: **zero violations**.
4. ✅ Hashes propagados consistentes:
   - `entities/content.rs` ↔ `entities/content.md` (`b632e841`).
   - `entities/parity.rs` ↔ `entities/parity.md` (`af8490cb`).
5. ✅ `Content::Pagebreak` variant em produção (47 → 48).
6. ✅ `Parity` enum em produção.
7. ✅ Stdlib `#pagebreak(weak: ?, to: ?)` invocável (36 → 37
   funcs).
8. ✅ Cobertura arms exaustiva.
9. ✅ Layouter força nova página com paridade verificada
   (validado por 4 tests E2E).
10. ✅ Inventário 148 reflecte cobertura aumentada (Layout
    44% → 50%).
11. ✅ README ADRs entrada P156E.
12. ✅ Sem ADR criada / revogada / revisada.
13. ✅ Sem DEBT criado / fechado.
14. ✅ ADR-0061 inalterada (mantém-se PROPOSTO).
15. ✅ Sem regressão (todos os 1192 tests pré-P156E
    continuam a passar; +22 novos passam).
16. ✅ Relatório do passo escrito (este ficheiro).

---

## §15 — Notas operacionais

- **Padrão "passos granulares" — terceira aplicação
  consecutiva**. P156C+P156D+P156E todos mantiveram a mesma
  cadência: zero reformulações, build clean, tests à
  primeira. **N=3 aplicações reforçam empiricamente** a
  hipótese da decisão humana 2026-04-25.

- **Halfway point Fase 1 atingido**: 50% Layout cobertura
  (9/18). Restantes 6 entradas: `box`, `block`, `stack`,
  `repeat`, `columns`/`colbreak`, `skew`. Mapping ao
  roadmap ADR-0061: `skew` Fase 3 trivial; `box`+`block`+
  `stack` Fase 2 containers ricos; `repeat`+`columns`/
  `colbreak` Fase 3 (columns com DEBT-56 column flow L+).

- **Tipo `Parity` novo**: análogo a `Sides<T>` em P156C.
  Infraestrutura genérica criada em passo material.
  Disponível para futuros consumers (refino Page rico —
  per ADR-0061 §6.3 — pode usar em paridade de
  header/footer).

- **Helpers reusados**: `extract_length` de P156C
  (não usado neste passo); `extract_weak` de P156D
  (não usado — implementado inline em pagebreak por
  simplicidade). **Helper novo**: `extract_parity`.
  Antecipável reuso baixo (parity é caso isolado).

- **Layouter reusou `new_page`**: sem método novo. P156D
  reusou `flush_line`; P156E reusa `new_page`. Padrão de
  reuso de helpers do Layouter está a funcionar.

- **ADR-0061 mantém PROPOSTO**: per decisão humana.
  Anotação cumulativa após Fase 1 completa.

- **Variants count**: 47 → **48** (+1). Após P156F
  (skew): 49. Após P156G (block): 50. Após Fase 2
  (block+box+stack): ~52.

- **Stdlib funcs**: 36 → **37** (+1). Após P156I: ~40.

- **Pós-156E**:
  - 6 features Layout implementadas total (pad, hide,
    h, v, pagebreak; align/move/rotate/scale já existiam).
  - Cobertura Layout: **50%** (halfway point).
  - Cobertura user-facing total: 57%.
  - **Próximo**: P156F (skew) ou alternativa humana.

- **Granularidade vs paridade rápida — N=3 confirma**:
  P156C+P156D+P156E = 3 passos consecutivos com cadência
  estável. Tests cumulativos por passo: 27, 20, 22.
  Cobertura Layout cumulativa: +11%, +11%, +6%. Slope
  decreasing porque restantes features são mais complexas
  (Fase 2 containers; Fase 3 columns).

- **Quarentena vanilla**: continua opção 3. Sem mudança.

- **Série paridade**: continua suspensa em P153. Sem
  mudança.

- **Hash do código `entities/content.rs`**: `4a7e0a99` (P156D)
  → **`b632e841`** (P156E). Hash `entities/parity.rs`:
  novo, **`af8490cb`**.

---

## §16 — Cross-references

- Spec: `00_nucleo/materialization/typst-passo-156e.md`.
- Diagnóstico (origem): `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`.
- Relatórios precedentes: `typst-passo-156c-relatorio.md`,
  `typst-passo-156d-relatorio.md`.
- ADR-0061 (aplicada): `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.
- Inventário 148 actualizado:
  `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- README ADRs: `00_nucleo/adr/README.md`.
- L0 prompts: `00_nucleo/prompts/entities/parity.md` (novo);
  `00_nucleo/prompts/entities/content.md` (secção pagebreak
  adicionada).
- Vanilla source consultado:
  `lab/typst-original/crates/typst-library/src/layout/page.rs`
  (PagebreakElem + Parity).
- Cristalino código tocado:
  - `01_core/src/entities/parity.rs` (novo, 80 linhas, 3 tests).
  - `01_core/src/entities/content.rs` (variant Pagebreak +
    construtor + cobertura arms + 5 tests).
  - `01_core/src/entities/mod.rs` (registo `pub mod parity;`).
  - `01_core/src/rules/introspect.rs` (arm Pagebreak em
    materialize_time + walk).
  - `01_core/src/rules/layout/mod.rs` (arm Pagebreak em
    layout_content + measure_content_constrained).
  - `01_core/src/rules/layout/tests.rs` (4 tests E2E).
  - `01_core/src/rules/stdlib/layout.rs` (`extract_parity`
    helper + `native_pagebreak`).
  - `01_core/src/rules/stdlib/mod.rs` (re-export + 10 tests).
  - `01_core/src/rules/eval/mod.rs` (registo em
    `make_stdlib`).
