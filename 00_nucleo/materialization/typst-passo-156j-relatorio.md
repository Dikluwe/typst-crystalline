# Relatório P156J — `repeat` (Layout Fase 3 sub-passo 1; **primeira Fase 3**)

Materialização de `repeat` per ADR-0061 §"Aplicações cumulativas"
caminho 1. Continuação directa da série granular P156C-I
(2026-04-25 → 2026-04-26); P156J abre **Fase 3** com cadência
preservada (1 feature → +6%). **Oitava aplicação consecutiva**
de ADR-0061.

---

## 1. Resumo do executado

### 1.1 Diagnóstico (sub-passo .1)

Ficheiro novo: `00_nucleo/diagnosticos/diagnostico-repeat-passo-156j.md`
(7 itens canónicos per ADR-0034; **nona aplicação consecutiva**
do padrão diagnóstico-primeiro iniciado em P154A).

Conclusões-chave do inventário:
- Vanilla `RepeatElem` tem 3 fields: `body: Content` (req),
  `gap: Length` (default zero), `justify: bool` (default `true`).
- Algoritmo runtime (calcular floor(available / (body_width +
  gap)) e distribuir resto se justify) vive em `typst-layout` —
  **fora do scope cristalino actual** sem refactor inline-region.
- `extract_length` reusado N=6 vezes (subpadrão emergente).
- Padrão Smart→Option/default atinge N=6 aplicações consecutivas.

### 1.2 Variant `Content::Repeat` (sub-passo .2)

Adicionado a `01_core/src/entities/content.rs` (51 → **52** variants).

```rust
Repeat {
    body:    Box<Content>,
    gap:     Option<Length>,
    justify: bool,  // default vanilla true
}
```

Cobertura exaustiva de pattern-match em **9 sítios** (paridade
com P156I):
- Variant declaration + construtor `Content::repeat(...)`.
- `is_empty()` proxy via body.
- `plain_text()` recurse no body sem multiplicar.
- `PartialEq` cobre body/gap/justify.
- `map_content` recurse no body; preserva atributos.
- `map_text` idem.
- `introspect.rs::materialize_time` recurse + preserva atributos.
- `introspect.rs::walk` walk no body uma vez (sem multiplicar
  state — paridade vanilla).
- `layout/mod.rs::layout_content` single-render do body.
- `layout/mod.rs::measure_content_constrained` recurse para
  dimensões (paridade Block/Boxed sem inset).

### 1.3 Stdlib `native_repeat` (sub-passo .3)

Adicionado a `01_core/src/rules/stdlib/layout.rs` (41 → **42** funcs).

```rust
pub fn native_repeat(_ctx, args, _world, _file, _fig) -> SourceResult<Value>
```

Comportamento:
- `body` posicional obrigatório (Content ou Str → `Content::text`).
- `gap: Length`/`Float`/`Int` (em pt) via `extract_length` (N=6);
  `None` == zero per padrão Smart→Option (N=6).
- `justify: bool`; default **`true`** (paridade vanilla — divergência
  intencional do default Rust `bool::default() == false`,
  documentada).
- Validações: gap negativo rejeitado; gap não-length rejeitado;
  justify não-bool rejeitado; named arg desconhecido rejeitado;
  body inválido rejeitado.

Registado em `eval/mod.rs::make_stdlib` como `repeat` →
`Func::native("repeat", native_repeat)`. Re-exportado em
`stdlib/mod.rs`.

### 1.4 Tests (sub-passo .4)

**Δ tests = +19** (consistente com range esperado +18-25 da
série P156C-I):

- **6 unit tests Content::Repeat** em `entities/content.rs`:
  constructor default, constructor explícito gap/justify=false,
  is_empty proxy via body, plain_text recurse, PartialEq cobertura
  4 vias, map_text recurse + preserva atributos.
- **11 stdlib tests** em `stdlib/mod.rs`: defaults gap=None
  justify=true, str body, gap length, justify false, combina
  gap+justify, sem body, body int rejeitado, gap negativo
  rejeitado, gap não-length rejeitado, justify não-bool
  rejeitado, named arg desconhecido rejeitado + 1 regression
  cruzado P156J (Stack/Block/Box/Pad/Hide continuam funcionar).
- **2 layout E2E tests** em `layout/tests.rs`:
  `layout_repeat_renderiza_body_no_contexto_actual` (body emitido
  pelo menos uma vez) e
  `layout_repeat_counters_dentro_do_body_resolvem` (heading dentro
  de repeat renderiza via walk).

### 1.5 Propagação de hashes (sub-passo .5)

`crystalline-lint --fix-hashes .` propagou hash novo de
`entities/content.rs`: `b9ca52c4` → **`ec58d849`** (P156I → P156J).
Hash actualizado no header do ficheiro e em `prompts/entities/content.md`
("Hash do Código: e4d7b3d1" → "ec58d849").

---

## 2. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo test` workspace: 1296 + Δ; zero falhas | **1077 typst-core lib + 215 typst-shell + 24 diagnostic + 21 cli = 1337**; **0 failed** |
| 2 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 3 | Variants Content: 52 (51 → 52) | **✓ 52** (`Repeat` adicionado) |
| 4 | Stdlib funcs: 42 (41 → 42) | **✓ 42** (`repeat` registado) |
| 5 | Cobertura Layout: 78% (14/18) | **✓ 78%** (entrada `repeat` `ausente → implementado` em §A.5) |
| 6 | Hash actualizado em prompts L0 (--check-hashes) | **✓** (`ec58d849` em ambos `content.rs` e `content.md`) |

**Δ tests = +19** (range 1296→1315 nas categorias lib+integ+
diagnostic; total workspace inclui CLI inalterado em 21).

**Build limpo**: `cargo build` 3.06s sem warnings novos.

---

## 3. Análise de risco (padrão N=5 — quinta aplicação consecutiva)

P156J é **passo aditivo** (sem refactors). Mesmo assim, documentar
risco preserva o precedente N=4 (P156F/G/H/I) e mantém disciplina.

### 3.1 Riscos identificados

| Risco | Avaliação | Mitigação aplicada |
|-------|-----------|---------------------|
| Variant exhaustivo falhar em sítio fora de `content.rs` | Baixo | Cobertura sistemática em 9 sítios (paridade P156I); compilador detecta omissões via `match` exaustivo |
| Algoritmo dinâmico runtime ser exigido em testes E2E | Baixo | Tests E2E focados em paridade estrutural (single-render); algoritmo dinâmico explicitamente diferido per ADR-0054 graded |
| Default `justify=true` causar surpresa em users que esperem `false` (default Rust natural) | Médio | Documentado em diagnóstico §3 e em prompt L0 §"Padrão emergente"; **divergência intencional** para paridade vanilla |
| Conflito de naming `repeat` com `Iterator::repeat`/`str::repeat` | Nulo | `Content::Repeat` é variant, não método; sem colisão |
| Gap negativo aceito acidentalmente | Nulo | Rejeitado com erro hard (consistente com pad/block/box/stack) |
| Counters dentro de body multiplicarem state se walk fosse iterado | Nulo | Walk single-pass per design; vanilla também conta uma vez |

### 3.2 Riscos não-aplicáveis

- **Refactor de tipo existente**: zero (variant novo aditivo).
- **Mudança em arms existentes**: zero (todos os 9 sítios são
  adições isoladas).
- **Quebra de contrato API público**: zero (construtor
  `Content::repeat` é novo; nenhum existente alterado).

### 3.3 Conclusão de risco

**Risco residual: muito baixo.** Padrão "passo aditivo +
cobertura sistemática + paridade per ADR-0054 graded" replica
o tratamento bem-sucedido de P156G/H/I para containers
complexos. Zero reformulações mid-passo (oitava aplicação
consecutiva).

---

## 4. Slope cumulativo (mesa P156C-J)

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P156C | pad + hide | +11% | 22% → 33% | +27 |
| P156D | h + v | +11% | 33% → 44% | +20 |
| P156E | pagebreak | +6%  | 44% → 50% | +22 |
| P156F | skew | +6%  | 50% → 56% | +16 |
| P156G | block | +5%  | 56% → 61% | +20 |
| P156H | box | +6%  | 61% → 67% | +21 |
| P156I | stack | +5%  | 67% → 72% (target Fase 1+2) | +25 |
| **P156J** | **repeat** | +6%  | **72% → 78%** (Fase 3 sub-passo 1) | **+19** |

**Total cumulativo P156C-J**: **+56pp** Layout em 8 passos
consecutivos (22% → 78%). **+170 tests** acumulados (1145 →
1315 lib+integ+diagnostic). **Zero reformulações mid-passo**
em N=8 aplicações.

**Cobertura user-facing total**: ~53% (pré-P156C) → **~60.3%**
(pós-P156J). +7pp em 8 passos.

---

## 5. Padrões metodológicos consolidados

### 5.1 Padrões consolidados (N atualizado)

| # | Padrão | N pré-P156J | N pós-P156J |
|---|--------|------------:|------------:|
| 1 | Granularidade 1-2 features/passo | 7 | **8** |
| 2 | "Inventariar primeiro" pré-decisão | 4 | **5** |
| 3 | "Smart<T> → Option<T> ou default" | 5 | **6** |
| 4 | "§análise de risco no relatório" | 4 | **5** |
| 5 | "Reuso de template containers" (variant rico) | 3 | **4** |
| 6 | Helper `extract_length` reuso (subpadrão de §5) | 5 | **6** |

### 5.2 Padrão #3 (Smart→Option) — patamar empírico forte

P156J marca a **sexta aplicação consecutiva** do padrão simplificador:
- P156D: `weak: bool` directo (sem Smart wrapping).
- P156E: `Smart<Parity>` → `Option<Parity>`.
- P156G: `Smart<Rel<Length>>` → `Option<Length>` (Block.width).
- P156H: idem (Box.width + Box.baseline).
- P156I: `Smart<Length>` → `Option<Length>` (Stack.spacing) +
  `Smart<Dir>` → `Dir` directo com Default natural.
- **P156J**: `Length` (default zero vanilla) → `Option<Length>`
  (gap); `bool` directo (justify).

**N=6 reforça candidatura a ADR meta P156K-meta** (formalizar
padrões emergentes Smart→Option + inventariar-primeiro).

### 5.3 Padrão #6 (helper `extract_length` reuso)

Subpadrão emergente dentro de §5 ("reuso de template containers"):
o helper `extract_length` em `stdlib/layout.rs` foi reusado
**6 vezes consecutivas** (P156C/D/G/H/I/J). Promoveu-se de
helper privado de `pad` (P156C) a vocabulário canónico para
coerção de Length em named args. Promoção formal a helper público
`pub fn extract_length(...)` em `stdlib/mod.rs` é candidato a
**refactor escopo XS** futuro.

---

## 6. ADR-0061 §"Aplicações cumulativas"

Renomeada `pós-P156I` → `pós-P156J`. Tabela slope cumulativo
estendida com linha P156J. Padrões metodológicos N atualizados
(N=8 granularidade; N=5 inventariar; N=6 Smart→Option; N=5
risco; N=4 reuso template; novo subpadrão N=6 extract_length).
Estado pós-P156J detalhado: 4 entradas Layout pendentes (era 5);
caminho 1 50% concluído (1/2 features Fase 3 materializadas).

**Status mantido `PROPOSTO`**. Promoção a `IMPLEMENTADO` continua
diferida — decisão humana sobre columns/colbreak (DEBT-56) ou
scope-out formal.

---

## 7. Estado pós-P156J

- **Cobertura Layout**: **78%** (14/18 implementado puro). Target
  ADR-0061 (72%) **ultrapassado** em +6pp via Fase 3 sub-passo 1.
- **Variants Content**: **52** (era 51).
- **Stdlib funcs**: **42** (era 41).
- **Tests**: **1315** lib+integ+diagnostic (era 1296). +19.
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados em P156J.
- **ADR-0061**: PROPOSTO; §"Aplicações cumulativas" anotada
  com P156J.
- **README ADRs**: entrada P156J adicionada antes de P156I;
  total 61 ADRs inalterado.
- **Hash actualizado**: `entities/content.rs` `b9ca52c4` →
  `ec58d849`; propagado a `prompts/entities/content.md`.

### 7.1 Restantes 4 entradas Layout pendentes

- `columns`/`colbreak` (Fase 3 condicional — DEBT-56 column
  flow L+ aberto em P156B).
- `pad` parcial — refino sides individualizadas (refactor
  Sides<T> via dict).
- `place` parcial — refino column scope (parcialmente
  implementado em P84.6).
- `measure` parcial — depende ADR-0017 Introspection runtime
  adiada.

---

## 8. Decisão pós-P156J

Per spec do passo §"Pós-passo", as candidatas para próximo passo são:

1. **P156K-meta** — formalizar padrões emergentes em ADR meta:
   - Smart→Option N=6 (patamar forte).
   - Inventariar-primeiro N=5.
   - Reuso template containers N=4.
   - Helper extract_length N=6 (subpadrão).
   Enunciado já preparado (per spec). **Sem materialização de
   código novo**; apenas decisão arquitectural formal.

2. **Continuar Fase 3** — materializar columns + colbreak
   (DEBT-56 column flow L+). Escopo XL agregado se priorizado.

3. **Outra direcção** das 7 candidatas documentadas em ADR-0061
   §"Aplicações cumulativas" se contexto humano mudar.

**Recomendação implícita** (mesma que pós-P156I): pausa natural
após P156J, dado que (a) target Fase 1+2 ultrapassado, (b) Fase 3
parcialmente activada, (c) padrões consolidados a patamar forte
(N=6 em duas dimensões), (d) decisão sobre P156K-meta tem máxima
informação acumulada. Decisão humana.

---

## 9. Fechamento

P156J fecha como **passo M aditivo**, sem reformulações nem
DEBTs novos. **Oitava aplicação consecutiva** de ADR-0061;
**primeira Fase 3**. Cadência granular preservada (1 feature →
+6%). Padrões metodológicos consolidados a N=6 em duas dimensões
(Smart→Option + extract_length).

ADR-0061 mantém `PROPOSTO`; promoção continua diferida.

**Pausa natural após P156J — decisão humana sobre próxima
direcção tem máxima informação.**
