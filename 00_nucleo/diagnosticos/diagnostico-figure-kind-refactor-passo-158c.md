# Diagnóstico Figure.kind refactor — Passo P158C

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
quarto sub-passo Model figure-kinds (Bloco A do diagnóstico
P159B §3.4). **Vigésima aplicação consecutiva** do padrão
diagnóstico-primeiro.

Refactor cosmético: `Content::Figure.kind: String → Option<String>`
per **ADR-0064 Caso A estrito** (vanilla `Smart<Str>` →
cristalino `Option<String>`; None ↔ Auto). **Patamar Caso A
cresce N=6 → 7** com primeiro Caso A "estrito" em refactor
(não em variant aditivo). Subpadrão emergente "refactor de field
para Option" candidato a registo.

---

## 1. Assinatura vanilla `Figure.kind`

Per `lab/typst-original/crates/typst-library/src/model/figure.rs`,
vanilla `FigureElem.kind` é `Smart<FigureKind>` onde `FigureKind`
é enum com `Image`/`Table`/`Raw`/etc. Cristalino simplificou para
`String` directo em P75 (default `"image"`).

`Smart::Auto` semântica: "computa do contexto" — vanilla resolve
via Show rules. Cristalino P158A implementa via
`infer_kind_from_body` + fallback `"image"`.

**P158C refactor**: `String` → `Option<String>` (None ↔ Auto;
default `"image"` resolvido em uso, não em construção).

---

## 2. Comportamento observável

**Vanilla**: `figure(image(...))` (sem `kind:` explícito)
produz Figure com `kind = Auto` resolvido a Image.

**Cristalino actual** (P158A): `figure(image(...))` produz
Figure com `kind = "image"` (resolvido em `native_figure` via
`infer_kind_from_body`).

**Cristalino P158C** (proposto): `figure(image(...))` produz
Figure com `kind = Some("image")` (auto-detectado) **OU**
`kind = None` (se body não detectável; resolvido em uso).

**Decisão de fallback chain** (preserva backwards compat):
- `kind:` explícito → `Some(s)`.
- Auto-detect → `infer_kind_from_body(&body)` (já retorna
  `Option<String>`).
- Sem detect → **`None` directo** (sem aplicar default em
  construção).
- Caller (introspect, layout) faz `kind.as_deref().unwrap_or("image")`.

**Output observable inalterado**: tests que verificam
`"Figura 1"` no label, `"image"` no counter — todos preservam.

---

## 3. ADR-0064 Caso A (estrito)

**Caso A canónico**: `Smart<T>` vanilla → `Option<T>` cristalino
+ fallback default em uso.

**Aplicações cumulativas pós-P158C**: **N=6 → 7**.
- P156G Block.width
- P156H Box.width
- P156I Stack.spacing
- P157B TableCell.x/y
- P159A Bibliography.title + Cite.supplement
- P159C Cite.form
- **P158C Figure.kind** ← refactor (não variant aditivo)

**Distribuição cross-domínio pós-P158C**:
- Layout: 3 (P156G/H/I).
- Model: 4 (P157B + P159A + P159C + **P158C**).
- **Equilíbrio passa de 50/50 para 43/57 favorecendo Model**.

**Subpadrão emergente NOVO**: "refactor de field para Option"
N=1 (precedente novo — distinto de variant aditivo com
Option<T> field; aplicação em refactor de tipo existente).

---

## 4. Variants Content existentes a estender

**`Content::Figure`** — refactor de field (não expansão).
Estrutura actual:
```rust
Figure {
    body:      Box<Content>,
    caption:   Option<Box<Content>>,
    kind:      String,
    numbering: Option<String>,
}
```

P158C transforma:
```rust
Figure {
    body:      Box<Content>,
    caption:   Option<Box<Content>>,
    kind:      Option<String>,  // P158C: refactor
    numbering: Option<String>,
}
```

**Hash content.rs preservado esperado** — L0-baseline
interpretation (lição P159A/C/D internalizada). Refactor de
tipo interno cosmético cabe na regra; doc-comment do field
ajustado mas hash do prompt L0 preservado.

**Sítios de pattern-match a actualizar** (audit completo via
grep):

### entities/content.rs (5 sítios)
- L202 — variant declaration.
- L1148-1150 — PartialEq.
- L1290-1298 — map_content (clone preserva).
- L1548-1553 — map_text (clone preserva).
- Tests internas (~7-8 sítios em testes existentes).

### rules/introspect.rs (5 sítios + tests)
- L69-74 — materialize_time (clone preserva).
- L291 — walk arm (`kind.clone()` em entry para
  local_figure_counters).
- L323-329 — Labelled arm para Figure (`kind.as_str()` em
  figure_numbers lookup + `figure_supplement_for_lang`).
- Tests existentes (~10+ sítios constructor `kind: "image"
  .to_string()`).

### rules/layout/mod.rs (1 sítio)
- L399-414 — figure layout arm (`kind.clone()` para
  figure_progress + `kind.as_str()` para figure_numbers).

### rules/stdlib/figure_image.rs (1 sítio)
- L77-80 — `native_figure` constructor: ajustar fallback chain
  para retornar `Option<String>` directamente.
- L86-91 — `Content::Figure { kind, ... }` literal.

### rules/stdlib/mod.rs (5 sítios em tests)
- L451, L467, L486, L509, L525, L555 — asserts
  `assert_eq!(kind, "image")` etc. precisam adaptação.

**Total**: ~25-30 sítios (exclui tests internos cujo conteúdo
é constructor literal `kind: "image".to_string()`).

---

## 5. Helpers stdlib reusáveis

**Nenhum directo**. Refactor é cascading inline; cada caller
adapta-se com `.as_deref().unwrap_or("image")`.

Subpadrão emergente potencial: helper `kind_or_default(&Option<String>)
-> &str` se múltiplos callers se acumularem. Promoção diferida
N=3-4 mínima.

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P158C | Refino futuro |
|---------|--------------|---------------|
| Refactor estrito Caso A | ✓ implementado | nenhuma limitação |
| Output observable preservado | ✓ implementado | refactor cosmético |
| Helper `kind_or_default` | ✗ não criado | promoção diferida N=3-4 |
| Refactor análogo de outros String fields em Content | ✗ scope-out | NÃO reservado |
| Documentação completa de variants no L0 prompt content.md | ✗ scope-out | passo administrativo XS futuro NÃO reservado |

---

## 7. Tests planeados

### 7.1 Test novo (~1)

`figure_kind_none_resolve_para_image_default` em
`stdlib/mod.rs` — verifica que body=Text sem `kind:` explícito
produz `kind == None` em vez de `kind == "image".to_string()`;
valida o refactor.

### 7.2 Tests existentes adaptados (~5 em stdlib/mod.rs)

Tests P157A/P158A `figure_auto_detect_image/table/raw/explicit_override/default`
— ajustar asserts:
- Antes: `assert_eq!(kind, "image")`.
- Depois: `assert_eq!(kind.as_deref(), Some("image"))` ou
  `assert_eq!(kind, Some("image".to_string()))`.

### 7.3 Tests internos em content.rs / introspect.rs

Constructor literal `kind: "image".to_string()` muda para
`kind: Some("image".to_string())`. Tests verificam
comportamento (label, counter) que continua a passar.

**Δ esperado**: +1 a +3 tests novos (alinhado com esboço P159B
§3.4 range 2-4).

---

## 8. Cascading callers (decisão específica §8)

### 8.1 Identificação completa

| Sítio | Tipo de uso | Adaptação |
|-------|-------------|-----------|
| `entities/content.rs:202` | variant decl | `String → Option<String>` |
| `entities/content.rs:1148` | PartialEq | inalterado (Option PartialEq) |
| `entities/content.rs:1290` | map_content | `kind.clone()` → idem |
| `entities/content.rs:1548` | map_text | idem |
| `rules/introspect.rs:69` | materialize_time | `kind.clone()` → idem |
| `rules/introspect.rs:291` | walk counter | `kind.clone()` → `kind.as_deref().unwrap_or("image").to_string()` |
| `rules/introspect.rs:323-339` | Labelled Figure arm | `kind.as_str()` → `kind.as_deref().unwrap_or("image")` |
| `rules/layout/mod.rs:399-414` | figure progress | idem |
| `rules/stdlib/figure_image.rs:77-91` | constructor | retornar `Option<String>` directo |
| Tests existentes Figure | asserts | `Some("image".to_string())` ou `as_deref()` |

**Backwards compat trivial**: introspect/layout sempre resolvem
default `"image"` → tests pré-existentes que verificam label
"Figura 1", counter independent etc. continuam a passar.

---

## 9. Backwards compat (decisão específica §9)

**Stdlib `native_figure` assinatura externa preservada**:
- Aceita `kind: auto/none/Str` (paridade P158A).
- Apenas a representação interna muda — `kind.unwrap_or("image")`
  removido; `kind` directamente atribuído.

**Tests de label pré-existentes**:
- `introspect_resolve_label_de_figura` ("Figura 1"): preservado
  via fallback "image" em counter lookup.
- `introspect_duas_figuras_contadores_independentes`: preservado.
- Tests P158B `figure_label_*_devolve_*` (lang-aware): preservados
  via `kind.as_deref().unwrap_or("image")` antes de
  `figure_supplement_for_lang(kind, lang)`.

**Tests de stdlib `figure_auto_detect_*`**: adaptação trivial
de `kind == "image"` para `kind == Some("image".to_string())`
ou `kind.as_deref() == Some("image")`.

---

## Resumo executivo

P158C materializa **refactor `Content::Figure.kind: String →
Option<String>`** per ADR-0064 Caso A estrito:
- Variant `Content::Figure` field tipo refactored.
- ~10 sítios callers adaptados (stdlib, introspect, layout +
  tests).
- Helper inline `.as_deref().unwrap_or("image")` em callers.
- Sem novos helpers públicos.
- Sem alteração observable (output preservado).

**Decisões arquitecturais P158C**:
- **Refactor estrito Caso A** — primeiro Caso A em refactor
  (não em variant aditivo).
- **Default `"image"` resolvido em uso** (não em construção).
- **L0-baseline preserva hash content.rs** (regra default;
  lição P159A/C/D internalizada).
- **Sem helper `kind_or_default`** — promoção diferida N=3-4.

**Decisões diferidas (NÃO reservadas)**:
- Refactor análogo de outros String fields em Content variants.
- Helper público `kind_or_default(&Option<String>)`.
- Documentação completa de variants em L0 content.md.

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido.
- ADR-0033: paridade observable estrutural (output preservado).
- ADR-0054: graded scope-out de refactors análogos.
- ADR-0060: refino qualitativo Fase 2 Model.
- ADR-0064: Caso A patamar N=6 → 7 (primeiro estrito em refactor).
- ADR-0065 critério #5: scope determinado por inventário.

**Tests planeados**: Δ +1-3 tests novos + ~5 tests adaptados
(range spec 2-4).

**Risco**: baixo. Refactor cosmético com pattern já validado
N=6. Backwards compat trivial via fallback nos callers.
