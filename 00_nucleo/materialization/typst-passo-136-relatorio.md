# Passo 136 — Relatório (Fase A: estender `TextStyle` com 5 campos)

**Data**: 2026-04-24
**Precondição**: Passo 135 encerrado; 1084 total tests; zero
violations; 54 ADRs activas; 12 DEBTs abertos (DEBT-52
rastreador de 8 gaps).
**Natureza**: passo L1. **Extensão de struct + propagação**.
Zero efeito observável no PDF (infra pura). Fase A de 5 do
roadmap revisto em 135.
**ADR**: **não tocada**. ADR-0054 já cobre o critério.

---

## Sumário

`TextStyle` estendido com 5 campos (`weight, tracking, leading,
lang, font`) espelhando `StyleDelta`. `From<&StyleChain>`
propaga via 5 novos resolvers `StyleChain::weight()/tracking()/
leading()/lang()/font()`.

**Ripple detectado e resolvido**: `TextStyle` deixou de ser
`Copy` (porque `FontList` contém `Vec<FontFamily>`). 10+ call
sites adaptados com `.clone()` explícito.

**5 testes L1 novos** de propagação (um por campo) passam.
Regressão OK: bold/italic/size/fill continuam a funcionar.

**858 L1 (+5) + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1089 total** (+5 vs 1084). Zero violations.

**DEBT-52 gap 1 resolvido**. 7 gaps restantes.

---

## 136.A — Inventário confirmatório

### Findings

| Item | Resultado |
|------|-----------|
| A.1 `TextStyle` localização | `01_core/src/entities/layout_types.rs:104` |
| A.1 derives | `#[derive(Debug, Clone, Copy, PartialEq, Default)]` — **Copy presente** |
| A.1 campos | 5 (bold, italic, size, fill, heading_level) |
| A.2 `From<&StyleChain>` | `style_chain.rs:218` mapeia direct os 5 campos |
| A.2 resolvers | `chain.bold()`, `.italic()`, `.size()`, `.fill()`, `.heading_level()` existem |
| A.3 imports | não necessários (usados `crate::entities::X` qualificados) |
| A.4 Copy compatibility | **QUEBRA**: `FontList` contém `Vec` (não Copy) |
| A.5 `FrameItem::Text.style: TextStyle` | propagação automática via extensão de TextStyle ✓ |
| A.6 call sites Copy-dependent | 6 sites em layout/mod.rs + 1 em cursor.rs + 1 em content.rs + 5 em math/layout/*.rs + 3 em testes = **~15 sites** |
| A.7 tests base | L1: 853, Total: 1084 |

### Gate 136.A.4 disparou

`FontList` não-Copy quebra `TextStyle: Copy`. Decisão:
**remover `Copy` de `TextStyle`** (não via Arc wrap), adicionar
`.clone()` em call sites. Razão: `Arc<FontList>` também não
seria Copy (Arc é Clone mas não Copy); e `FontList` é uma lista
priorizada conceitualmente não-simples — referência indirecta
via Arc seria premature optimization.

Escolha documentada: ~15 sites de `.clone()` é ripple tratável.

---

## 136.B — Extensão de `TextStyle`

```rust
// derives: remove Copy.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub size: Pt,
    pub fill: Option<Color>,
    pub heading_level: Option<u8>,

    // Passo 136 (Fase A — DEBT-52):
    pub weight:   Option<u16>,
    pub tracking: Option<Length>,
    pub leading:  Option<Length>,
    pub lang:     Option<Lang>,
    pub font:     Option<FontList>,
}
```

`Default` impl derivado gera `None` para todos os novos
campos; construtores `regular/bold/italic` continuam a usar
`..Self::default()` — transparente.

---

## 136.C — Resolvers em `StyleChain`

5 novos métodos em `style_chain.rs`:

```rust
pub fn weight(&self) -> Option<u16>
pub fn tracking(&self) -> Option<Length>
pub fn leading(&self) -> Option<Length>
pub fn lang(&self) -> Option<Lang>
pub fn font(&self) -> Option<FontList>   // clona (não Copy)
```

Pattern idêntico aos existentes — top-wins com traversal linked
list. `font()` faz `.clone()` porque FontList é Clone (O(1) em
EcoString mas O(n) no Vec).

`From<&StyleChain>` estendido com 5 linhas invocando estes
resolvers.

---

## 136.D — Call sites adaptados (~15)

### `01_core/src/rules/layout/mod.rs` (7 sites)

- `let effective = TextStyle { ... }` (linha 225): adicionados
  5 campos novos com **top-wins** semântico
  (`self.style.X.or(node_style.X)`). Para Copy types
  (u16/Length/Lang): directo. Para FontList:
  `self.style.font.clone().or_else(|| node_style.font.clone())`.
- `let prev_style = self.style;` (4 sítios): → `self.style.clone()`.
- `style: self.style` inside FrameItem::Text literal (2 sítios):
  → `self.style.clone()`.

### `01_core/src/rules/layout/cursor.rs` (1 site)

- `style: self.style` inside FrameItem::Text literal →
  `self.style.clone()`.

### `01_core/src/entities/content.rs` (1 site)

- `Content::Text(..., *style)` → `Content::Text(..., style.clone())`.

### `01_core/src/rules/math/layout/*.rs` (5 sites)

- `..*style` (inside struct literal with partial fields):
  → `..style.clone()`. Em `attach.rs`, `root.rs`, `frac.rs`,
  `mod.rs` (2 ocorrências).
- `style: *style` (frame item) em `mod.rs` → `style.clone()`.

### Tests (2 sites)

- `layout_types.rs` (Frame tests): 2 sítios de reutilização
  de style — cloning intermediário.
- `layout/tests.rs:1569`: `*style` → `style.clone()`.

---

## 136.E — Testes novos (5)

Em `style_chain.rs::tests` — harness directo construindo
`StyleChain` com `StyleDelta` específico:

```rust
#[test]
fn text_style_from_chain_propaga_weight_passo_136() {
    let chain = StyleChain::default_chain()
        .push(StyleDelta { weight: Some(700), ..StyleDelta::empty() });
    let ts = TextStyle::from(&chain);
    assert_eq!(ts.weight, Some(700));
}
```

Análogos para `tracking` (Length::pt(0.5)), `leading`
(Length::em(0.65)), `lang` (Lang::ENGLISH), `font`
(FontList::single("Arial")).

Harness directo (não pipeline completo) é mais rápido e mais
preciso para validar a ponte `StyleChain → TextStyle`.

---

## 136.F — Prompts L0

`crystalline-lint --fix-hashes`: **Nothing to fix**. Prompts
abstratos não referem os campos específicos; adicionar novos
campos não altera descrição do ficheiro.

---

## 136.G — DEBT-52 actualizado

Gap 1 marcado como resolvido:

```markdown
- [x] **Fase A**: estender `TextStyle` + `From<&StyleChain>`.
      **Resolvido no Passo 136** (5 campos + 5 resolvers em
      StyleChain + 5 testes de propagação; `TextStyle` deixou
      de ser `Copy` — `.clone()` nos call sites).
```

7 gaps restantes.

---

## 136.H — Verificação

### Cargo tests

```
test result: ok. 858 passed ...       (L1 +5 vs 853)
test result: ok. 186 passed, 6 ign    (L3 inalterado)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual — 4 cenários de regressão

```bash
$ typst b.typ       # basic (heading + texto)
exit=0, PDF 939 bytes — compila OK

$ typst w.typ       # #set text(weight: 700)
exit=0, stderr: (vazio) — capturado, inerte

$ typst f.typ       # #set text(font: "Arial")
exit=0, stderr: (vazio) — capturado, inerte

$ typst h.typ       # canary hyphenate
h.typ:1:11: warning: text: propriedade 'hyphenate' ainda não suportada
exit=0
```

**Zero regressão**. Outputs idênticos aos de pré-136 (weight/font
continuam inertes no PDF; infra agora propaga os valores até
`FrameItem::Text.style`).

---

## Ficheiros tocados

| Ficheiro | Mudança | Linhas |
|----------|---------|-------:|
| `01_core/src/entities/layout_types.rs` | +5 campos em TextStyle, -Copy derive, test fixes | +12 |
| `01_core/src/entities/style_chain.rs` | +5 resolvers, +5 linhas em From, +5 tests | +85 |
| `01_core/src/rules/layout/mod.rs` | 7 `.clone()` adicionados, effective com 5 campos novos | +10 |
| `01_core/src/rules/layout/cursor.rs` | 1 `.clone()` | +0 |
| `01_core/src/rules/layout/tests.rs` | 1 `.clone()` | +0 |
| `01_core/src/entities/content.rs` | 1 `.clone()` | +0 |
| `01_core/src/rules/math/layout/*.rs` | 5 `..style.clone()` em 4 ficheiros | +0 |
| `00_nucleo/DEBT.md` | Gap 1 de DEBT-52 marcado resolvido | +3 |

**Zero ADR nova**. **Zero prompt L0 tocado**.

### Números finais

| Métrica | Antes (135) | Depois |
|---------|------:|-------:|
| L1 tests | 853 | **858** (+5) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1084** | **1089** (+5) |
| Violations | 0 | 0 |
| ADRs activas | 54 | 54 |
| DEBTs abertos | 12 | 12 (DEBT-52 gap 1 ✓) |

---

## Mudança semântica

**Zero observável**. Todos os 5 campos continuam inertes:
- `weight 700` é capturado em `StyleDelta` e agora propagado
  para `TextStyle.weight` e daí para `FrameItem::Text.style.weight`.
  **Mas export PDF ignora** `style.weight`.
- Idem para tracking, leading, lang, font.

**Passos 137-143 (fases B/C)** adicionam os consumers reais:
- 137: tracking.
- 138: leading.
- 139: weight faux-bold.
- 140-143: font + lang + PDF embedding.

**Pós-136**: a ponte está pronta. Os dados atravessam até ao
frame. Falta só o export consumi-los.

---

## Lições

1. **Gate 136.A.4 disparou como antecipado**: `TextStyle: Copy`
   + `FontList: !Copy` obrigou decisão entre remover Copy
   (opção escolhida) vs Arc wrap (indirecção extra). Ripple
   de ~15 sites foi tratável em 30min.

2. **`.clone()` em layout é barato**: `TextStyle` com 10 campos
   é ~40-48 bytes (5 bools/enums + 5 Options + embutidos).
   Clone envolve `FontList.clone()` que em O(n) mas n é
   tipicamente 1-3 famílias. Não é hot path — layout é
   eventualmente O(paragraphs × words) chamadas a clone.
   Se mostrar problema, alternativa `Arc<FontList>` disponível.

3. **Harness de teste directo vs pipeline**: escolhi
   construir `StyleChain` + `StyleDelta` directamente em
   testes de propagação (`text_style_from_chain_propaga_X`).
   Vantagens:
   - Zero dependência em MockWorld + eval.
   - Assertions precisas sobre `TextStyle` exacto.
   - Rápido (< 1ms cada).
   - Pattern simétrico aos existentes (passo 99/100).

4. **Infra antes de consumer paga-se**: se tentasse consumer
   weight antes de estender TextStyle, bateria em "weight não
   atravessa a ponte" e teria de voltar atrás. Split
   foi correcto — 1 passo infra, depois N passos de consumer
   sequencialmente.

5. **Extensão transparente para `..Default::default()`**:
   `TextStyle::regular/bold/italic` construtores usam
   `..Self::default()` — e `Default` derive cobre os 5 campos
   novos com `None` automaticamente. Zero trabalho adicional.

6. **Remover Copy é decisão boa**: alternativa Arc<FontList>
   esconderia custo atrás de indireccção sem ganho real. Com
   Clone explícito, a cadeia de chamadas fica honesta. Se
   benchmark mostrar regressão, Arc pode voltar como
   optimização informada.

---

## Estado pós-Passo 136

### DEBT-52 progresso

- ✓ **Fase A** (gap 1/8): TextStyle estendido, From propagado.

### DEBT-52 restante

7 gaps — fases B/C do roadmap:
- Consumer tracking (137).
- Consumer leading (138).
- Consumer weight faux-bold (139).
- Consumer font string (140).
- Consumer font array (141).
- Consumer lang hyphenation (142+).
- PDF font embedding real (adjacente a 140).

### Próximo passo sugerido: **Passo 137**

Consumer `tracking` (Fase B, S).

**Escopo**: aplicar `tracking` como offset adicional entre
words/glyphs em `metrics.advance` ou equivalente. Resolve
`tracking` com font-size para converter em Pt.

**Estimativa**: S (~1h). Primeira validação de que
`TextStyle.tracking` está acessível no layout e afecta output.
