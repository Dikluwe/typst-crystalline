---

# P475 — Sonda + fecho Trilha 8: `Boxed.stroke-overhang` + `Rel<Length>` em `extract_sides`

> **Passo:** 475
> **Data:** 2026-06-27
> **Foco:** Identificar e fechar os itens 7/8 e 8/8 de Trilha 8. Hipótese: (A) `Boxed.stroke-overhang` — único scope-out restante de `BoxedElem` (P156H); (B) ligação de `Rel<Length>` (P469) a `extract_sides_lengths` para que `pad(x: 10%)` funcione.
> **Trilha:** 8 — Refinos de stdlib e tipos.
> **Tipo:** Sonda-first + Materialização condicional.
> **Tamanho:** S (sonda XS + materialização S cada).
> **ADR-0117 Cláusula 4:** `BoxedElem` em `entities/elements/boxed.rs` (P329); `extract_sides` em `rules/stdlib/layout.rs` (P156L); `Rel<Length>` em `entities/rel.rs` (P469). Verificar antes de propor campo novo.

---

## Contexto

O P474 confirmou Trilha 8 em 6/8 completo, com dois itens ainda por identificar. O roteiro original lista como itens pendentes de Trilha 8:

- `pad`/`corners`/`sides` — confirmado completo em P156L/P242/P250 (P474).
- Parâmetros configuráveis de decorações (`offset`, `extent`, `size`) — confirmado completo em P448/P449/P471 (P474).
- `Value::Relative` — implementado em P469.
- `Symbol` — implementado em P471.
- Marcadores list/enum — implementados em P470.
- i18n caption — implementado em P470.

**O que resta (hipótese baseada no conhecimento do projeto):**

1. **`Boxed.stroke-overhang`** — documentado como único scope-out restante de `BoxedElem` desde P156H. `BlockElem` completou 10/10 em P250; `BoxedElem` ficou em 5/6.
2. **`Rel<Length>` em `extract_sides_lengths`** — `pad(x: 10%)` deveria funcionar mas `extract_sides` aceita apenas `Length` absoluto. O scope-out de P156L foi: "refino futuro para `Rel<Length>`". P469 materializou o tipo; a ligação ainda está pendente.

**Ambas as hipóteses precisam de confirmação via sonda antes de qualquer código.**

---

## ADR-0108 — Medir antes de decidir (sondas obrigatórias)

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `BoxedElem` tem campo `stroke_overhang`? | `entities/elements/boxed.rs` | 🟡 |
| `stroke-overhang` é mencionado como scope-out em `boxed.md`? | `prompts/entities/elements/boxed.md` | 🟡 |
| `extract_sides_lengths` existe e aceita apenas `Length`? | `rules/stdlib/layout.rs` | 🟡 |
| `extract_sides_lengths` aceita `Rel<Length>`? | idem | 🟡 |
| `native_pad` chama `extract_sides_lengths`? | idem | 🟡 |
| `Rel<Length>` tem método `resolve(context: Length) -> Length`? | `entities/rel.rs` | ✅ (P469) |
| `Value::Relative(Rel<Length>)` existe? | `entities/value.rs` | ✅ (P469) |
| Outro item pendente Trilha 8 além dos dois hipóteses? | grep roteiro + DEBT.md | 🟡 |
| Bloqueadores técnicos? | Nenhum conhecido | ✅ |

**Todas as sondas 🟡 devem ser executadas com `grep`/`file:line` antes de escrever código.**

---

## Sub-item A — `Boxed.stroke-overhang` (hipótese)

### A.1 — O que é `stroke-overhang`

No vanilla, `stroke-overhang` controla se o contorno (`stroke`) de um `box` se expande para fora dos limites do elemento (overhang) ou fica contido dentro. É a propriedade que distingue stroke interno de stroke externo — análogo a `box-sizing: border-box` vs `content-box` em CSS.

No cristalino, `BoxedElem` tem `stroke: Option<Stroke>` desde P247. Se `stroke-overhang` não está implementado, o contorno usa sempre um dos dois comportamentos por defeito.

### A.2 — Decisão condicional à sonda

Se a sonda confirmar que `stroke-overhang` é um campo em falta:

```rust
// Em BoxedElem:
pub stroke_overhang: bool,   // P475 NOVO — true = stroke expande para fora; false = dentro
```

Default: `true` (paridade vanilla — overhang por defeito).

`native_box` aceita `stroke-overhang: Bool` como named arg. No layout de `BoxedElem`, o `stroke` é emitido com ou sem offset dependendo do valor.

Se a sonda revelar que `stroke-overhang` já está implementado ou que não é o item pendente, a hipótese é descartada e substituída pelo resultado da sonda.

### A.3 — Impacto no layout

**Ficheiro:** arm de `Content::Boxed` no layouter (ou `layout/boxed.rs`)

```rust
let stroke_rect = if e.stroke_overhang {
    // stroke expande para fora: pos.x - stroke_width/2
    ShapeKind::Rect { /* aumentado por stroke_width/2 em cada lado */ }
} else {
    // stroke contido dentro: pos.x + stroke_width/2
    ShapeKind::Rect { /* reduzido */ }
};
```

---

## Sub-item B — `Rel<Length>` em `extract_sides_lengths`

### B.1 — Estado atual

`extract_sides_lengths` em `stdlib/layout.rs` converte um argumento de `native_pad` (e outros consumers) para `Sides<Length>`. Aceita:
- `Value::Length(l)` → uniforme nos 4 lados
- `Value::Dict` com keys `"left"`, `"right"`, `"top"`, `"bottom"`, `"x"`, `"y"`, `"rest"` → cada um `Value::Length(l)`

Não aceita `Value::Relative(Rel<Length>)`. O scope-out de P156L foi declarado como "refino futuro para `Rel<Length>`".

### B.2 — O que é necessário

Para que `pad(x: 10%)` funcione, `extract_sides_lengths` (ou uma variante nova `extract_sides_lengths_or_rel`) deve aceitar `Value::Relative` e armazená-lo como `Rel<Length>`.

**Problema:** `Sides<Length>` usa `Length` absoluto. Se `pad` aceitar `Rel<Length>`, o tipo do campo muda para `Sides<Rel<Length>>` — o que pode ter impacto em cascata.

**Alternativa menos disruptiva:** `Sides<RelOrAbs>` onde `RelOrAbs = Rel<Length>` (que já pode representar um absoluto com `rel: 0.0`). O tipo `Rel<Length>` de P469 já funciona para ambos os casos (`Rel { rel: 0.0, abs: l }` = absoluto; `Rel { rel: 0.5, abs: Length::zero() }` = 50%).

**Mas:** `pad` aplica inset no layout. O Layouter resolve `Length` directamente em pontos. Se `Sides<Rel<Length>>`, o layouter precisa de contexto para resolver — a largura ou altura da página/container.

### B.3 — Decisão de subset

Dado que a resolução de `Rel<Length>` requer contexto de layout (Trilha 7, já fechada, mas o consumer `pad` não estava ligado), a questão é: o `Layouter` tem acesso à largura da página actual no arm de `Content::Pad`?

**Sonda necessária:** verificar o arm de `Content::Pad` no Layouter e confirmar se `regions.current.width` ou `page_config.width` está acessível ali.

Se sim, a implementação é:

```rust
// Em extract_sides (nova variante ou extensão):
Value::Relative(rel) => {
    // Resolve com contexto de largura da página
    // Armazenar Rel<Length> no tipo Sides; resolver no layouter
    RelOrAbs::Relative(rel)
}
```

Se não, o item é scope-out para quando o Layouter tiver melhor acesso a contexto.

### B.4 — Subset minimal aceitável

Se a resolução de contexto for complexa, o subset mínimo útil é: `pad(all: 10%)` resolve contra a largura da página actual no momento do layout. Sem `pad(x: 10%, y: 5%)` diferenciado por eixo neste passo.

---

## Tests

### Sub-item A (se confirmado como pendente)

- **L1:** `BoxedElem` com `stroke_overhang: true` preservado em `map_content`.
- **L2:** `native_box(body, stroke: red, stroke-overhang: false)` produz `BoxedElem` com `stroke_overhang: false`.
- **L2:** Layout com `stroke_overhang: true` emite stroke com offset externo.
- **L2:** Layout com `stroke_overhang: false` emite stroke com offset interno.

### Sub-item B (se confirmado como viável)

- **L1:** `extract_sides_rel(Value::Relative(Rel::from_percent(10.0)))` retorna `RelOrAbs::Relative(...)`.
- **L2:** `native_pad(rest: 10%)` produz `PadElem` com `inset: Sides<Rel<Length>>` com `rel: 0.1`.
- **L2:** Layout de `Content::Pad` com `inset` relativo resolve correctamente para `10% * page_width`.
- **L2:** `native_pad(x: 5%, top: 2pt)` — misto relativo/absoluto.

---

## Spec L0

### Actualizados (condicional à sonda)

- `entities/elements/boxed.md` — campo `stroke_overhang` (se adicionado); marcar 6/6 scope-outs fechados.
- `rules/stdlib/layout.md` — `extract_sides_lengths` alargada para `Rel<Length>`; `native_pad` aceita `%`.
- `engine/layout/pad.md` (ou `mod.md`) — resolução de `Rel<Length>` no arm de `Content::Pad`.
- Roteiro — **Trilha 8: COMPLETA** (8/8).

---

## Scope-out explícito

- **`stroke-overhang` em `BlockElem`** — `Block` não tem stroke-overhang no vanilla; apenas `box`. Confirmado scope-out.
- **`Sides<Rel<Length>>` com resolução por eixo** (`x: 10%, y: 5%` com resolução diferente por eixo) — subset com `rest: 10%` uniforme neste passo.
- **`pad` com `Rel<Length>` em contexto de célula de grid** — requer contexto da célula. Apenas contexto de página neste passo.
- **`box(inset: 10%)`** — `Boxed.inset` usa `Sides<Length>` actualmente; ligar a `Rel<Length>` é refino adicional ao sub-item B.

---

## Critério de fecho

- [ ] Sondas executadas: `BoxedElem.stroke_overhang` (presente/ausente), `extract_sides_lengths` aceita `Rel<Length>` (sim/não), `pad` arm do Layouter tem acesso a contexto de largura — todos com `file:line` e conclusão.
- [ ] Decisão de arquitectura fixada para cada sub-item pós-sonda.
- [ ] Sub-item A: se confirmado pendente — `stroke_overhang: bool` adicionado a `BoxedElem`; `native_box` aceita `stroke-overhang:`; layout aplica.
- [ ] Sub-item B: se viável — `extract_sides_rel` (ou extensão) aceita `Value::Relative`; `native_pad` aceita `%`; layout resolve com contexto de largura.
- [ ] Testes verdes para cada sub-item implementado.
- [ ] Spec L0 actualizada.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 8: 8/8 COMPLETA** (se ambos sub-items implementados) ou estado actualizado com scope-outs declarados.

---

## Cenários possíveis pós-sonda

| Resultado | Acção |
|-----------|-------|
| A confirmado pendente + B viável | Implementar ambos. Trilha 8: COMPLETA. |
| A confirmado pendente + B scope-out (sem contexto no Layouter) | Implementar apenas A. Trilha 8: 7/8 com B como scope-out declarado. |
| A já implementado (sonda surpreendente) + B viável | Implementar apenas B. Trilha 8: COMPLETA. |
| Ambos já implementados (Caso A duplo) | Zero código. Actualizar roteiro: Trilha 8 COMPLETA. |
| Ambos são scope-out | Trilha 8: 6/8 com divergências declaradas. Pivot para P476 em outra trilha. |

---

## Próximo passo (após P475)

Com Trilha 8 fechada ou em 7/8 scope-out declarado:

- **Sonda Trilha 5** — viabilidade de `rustybuzz` no `Cargo.toml` (XS, ~10 min). Determina se Trilha 5 é materializável.
- **Espaços de cor Trilha 4** — `cmyk`, `oklab`, `oklch` user-facing como `Value::Color` variants (M). Extensão da Trilha 4 já completa.
- **LoF/LoT com page numbers Trilha 6** — requer 2-pass convergente; provável scope-out longo prazo.

---

## Estado pós-P474 (para referência)

| Passo | Descrição | Estado | Trilha |
|-------|-----------|--------|--------|
| P465 | `repr()` completo | ✅ FECHADO | 8 |
| P466 | Métodos array/dict/str | ✅ FECHADO | 8 |
| P467 | Sonda `Selector::Where` | ✅ FECHADO | 3 |
| P468 | Estilos numéricos `[1]`, `[2]` | ✅ FECHADO | 6 |
| P469 | `Value::Relative` (`Rel<Length>`) | ✅ FECHADO | 8 |
| P470 | Marcadores list/enum + i18n caption | ✅ FECHADO | 8 |
| P471 | `Symbol` + `highlight` + `sub`/`super` size | ✅ FECHADO | 8 |
| P472 | Back-refs + ibid. + LoF/LoT | ✅ FECHADO | 6 |
| P473 | `op. cit.` + `#show regex(...)` | ✅ FECHADO | 6+3 |
| P474 | Sonda fecho Trilha 3 + Trilha 8 pad/corners | ✅ FECHADO | 3+8 |
| **P475** | Fecho Trilha 8: stroke-overhang + Rel pad | 🔄 EM PREPARAÇÃO | 8 |

**Trilha 1: COMPLETA.**
**Trilha 2: COMPLETA.**
**Trilha 3: COMPLETA.**
**Trilha 4: COMPLETA.**
**Trilha 6: 4/5 completo.**
**Trilha 7: COMPLETA.**
**Trilha 8: 6/8 completo** (pré-P475).
**Inventário de débitos: LIMPO.**
