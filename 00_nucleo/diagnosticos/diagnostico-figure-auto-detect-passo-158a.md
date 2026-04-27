# Diagnóstico `figure` auto-detect — Passo P158A

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
primeiro sub-passo Model figure-kinds. **Décima quarta aplicação
consecutiva** do padrão diagnóstico-primeiro.

Refino comportamental sem alteração estrutural — Variant
`Content::Figure` permanece inalterado; counters por kind
permanecem inalterados; refino vive **apenas em `native_figure`**.

---

## 1. Assinatura vanilla `FigureElem` para auto-detecção

Fonte: `lab/typst-original/.../model/figure.rs:335-344`.

```rust
let kind = elem.kind.get_cloned(styles).unwrap_or_else(|| {
    elem.body
        .query_first_naive(&Selector::can::<dyn Figurable>())
        .map(|elem| FigureKind::Elem(elem.func()))
        .unwrap_or_else(|| FigureKind::Elem(ImageElem::ELEM))
});
```

**Comportamento vanilla**:
- Se `kind` explícito presente: usa-se directamente.
- Caso contrário: query_first_naive busca **recursivamente** o
  primeiro descendant do body que implementa `Figurable` trait.
- Se nenhum Figurable encontrado: **default `ImageElem::ELEM`**
  (kind="image").

**Trait `Figurable` em vanilla**:
- Implementada por `ImageElem`, `TableElem`, `RawElem` (entre
  outros como `EquationElem` em alguns contextos).

---

## 2. Comportamento observável fallback chain

**Cristalino P158A**:
```rust
let kind = args.named.get("kind")              // 1. explícito
    .and_then(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None })
    .or_else(|| infer_kind_from_body(&body))    // 2. inferência
    .unwrap_or_else(|| "image".to_string());    // 3. default
```

**Precedência**:
- `kind` explícito tem **precedência absoluta** (preserva
  comportamento existente para tests pré-existentes).
- Auto-detecção só activa quando `kind:` ausente.
- Default `"image"` aplica-se quando inferência devolve `None`.

---

## 3. ADR-0064 caso aplicável

**NÃO aplicável** em P158A. `kind` continua `String` directo;
sem refactor para `Option<String>`. Aplicação futura potencial
em refactor não reservado.

---

## 4. Variants Content existentes a estender

**Nenhuma**. Refino stdlib apenas. `Content::Figure` permanece
inalterado.

---

## 5. Helpers stdlib reusáveis

Nenhum directo. **Helper privado novo** `infer_kind_from_body`
em `stdlib/figure_image.rs` (~10 linhas).

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P158A | Refino futuro |
|---------|--------------|---------------|
| Auto-detecção Image/Table/Raw direct | ✓ implementado | — |
| Auto-detecção em Sequence (recursive) | ✓ implementado (paridade vanilla parcial) | — |
| Auto-detecção em outros containers (Block/Box/Pad/Styled) | ✗ scope-out | refino futuro se prioritário (NÃO reservado) |
| Custom kinds detectáveis | ✗ scope-out | sem trait `Figurable` em cristalino |
| Supplement automático por kind | ✗ scope-out | refino futuro (NÃO reservado) |
| Refactor `kind: String → Option<String>` | ✗ scope-out | candidato ADR-0064 Caso A (NÃO reservado) |
| Show selectors `figure.where(kind:)` | ✗ scope-out | per ADR-0041 + ADR-0054 graded |

---

## 7. Tests planeados

### 7.1 Auto-detecção tests (~5)

Em `stdlib/mod.rs`:
1. `figure_auto_detect_image` — `figure(image(...))` → kind="image".
2. `figure_auto_detect_table` — `figure(table(...))` → kind="table".
3. `figure_auto_detect_raw` — `figure(raw(...))` → kind="raw".
4. `figure_kind_explicit_override` — `kind:` explícito vence.
5. `figure_default_image_quando_body_nao_detectavel` — text body
   → fallback "image".

### 7.2 Sequence handling test (1)

6. `figure_auto_detect_image_dentro_de_sequence` — `figure(Sequence([
   Image, ...]))` → kind="image" via recursão.

### 7.3 Regression (1 — coberto implicitamente)

Tests pré-existentes (`native_figure_com_body_e_caption`,
`native_figure_sem_caption`, etc.) usam `Content::text(...)` como
body — auto-detecção devolve None → fallback "image" (mesmo
comportamento default actual). **Zero risco regression**.

**Δ esperado**: +6 tests novos (5 auto-detect + 1 Sequence).

---

## 8. Decisão Sequence handling — recursão limitada a Sequence

### 8.1 Comportamento vanilla

Vanilla usa `query_first_naive(&Selector::can::<dyn Figurable>())` —
**busca recursiva profunda** em todo o subtree do body.

### 8.2 Comportamento esperado pelos tests existentes

Tests existentes em `stdlib/mod.rs` (linhas 388-431) NÃO verificam
`kind` field directamente — só caption. **Zero risco regression**
independentemente da decisão.

### 8.3 Decisão adoptada

**Recursão limitada a `Content::Sequence`**:
```rust
fn infer_kind_from_body(body: &Content) -> Option<String> {
    match body {
        Content::Image { .. } => Some("image".to_string()),
        Content::Table { .. } => Some("table".to_string()),
        Content::Raw { .. }   => Some("raw".to_string()),
        Content::Sequence(seq) => seq.iter().find_map(infer_kind_from_body),
        _ => None,
    }
}
```

Justificação:
- **Sequence é wrapper trivial** muito comum (markup `[...]`
  produz Sequence se múltiplos elementos).
- **Outros containers** (Block/Box/Pad/Styled) seriam recursive
  deep — risco maior de comportamento inesperado.
- **Paridade vanilla parcial** aceitável per ADR-0033 (paridade
  observável estrutural; divergência aceite nos containers
  não-Sequence).
- **Cobertura suficiente**: `figure[#image("a.png")]` markup
  produz `figure(Sequence([Image]))` — auto-detecção activa.

### 8.4 Limitação consciente

`figure(block(image(...)))` ou `figure(pad(image(...)))` **NÃO**
auto-detecta como "image" — devolve None → fallback "image"
(coincidência beneficia comportamento, mas não por design).

---

## 9. Tests pré-existentes — verificação

Tests existentes em `stdlib/mod.rs:388-431`:
- `native_figure_com_body_e_caption`: usa `Content::text` body;
  não verifica kind. **Continua válido**.
- `native_figure_sem_caption`: idem. **Continua válido**.
- `native_figure_caption_none_value`: idem. **Continua válido**.
- `native_figure_sem_body_retorna_err`: sem body. **Continua válido**.

**Verificação**: nenhum test passa `kind:` explícito quando body
permite auto-detecção. Adição de auto-detecção não quebra nenhum
test existente.

---

## Resumo executivo

P158A materializa **auto-detecção de kind em `native_figure`**:
- Helper privado novo `infer_kind_from_body` em
  `stdlib/figure_image.rs` com **recursão limitada a Sequence**.
- Modificação trivial em `native_figure` para fallback chain
  3 níveis (`kind explícito > infer > "image"`).
- Sem alteração a variant `Content::Figure` ou layout/introspect.

**Decisões arquitecturais P158A**:
- **Recursão limitada a Sequence** (não a outros containers) —
  paridade vanilla parcial per ADR-0033.
- **Sem refactor** `kind: String → Option<String>` (NÃO
  reservado per política P158).
- **Default "image" preservado** para compatibilidade.

**Decisões diferidas (NÃO reservadas)**:
- Auto-detecção em Block/Box/Pad/Styled.
- Custom kinds via trait Figurable (sem trait em cristalino).
- Supplement automático por kind.
- Refactor `kind` para Option.

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido.
- ADR-0033: paridade observável estrutural (recursão parcial).
- ADR-0054: graded scope-out (containers não-Sequence; supplement;
  show selectors).
- ADR-0060: refino qualitativo de feature implementada.
- ADR-0064: NÃO aplicável directamente.
- ADR-0065 critério #1 (naming `infer_kind_from_body`) +
  critério #5 (scope) implícitos.

**Tests planeados**: Δ +6.

**Risco**: muito baixo. Refino comportamental aditivo; sem
alteração de variant; tests pré-existentes preservados.
