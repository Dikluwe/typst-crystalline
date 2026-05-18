# Passo 279 — Relatório

**Tema**: Bug fix funcional Image em Group (narrow scope α-Fase A) — emit
de `XObject /ImN Do` dentro de `FrameItem::Group` em `draw_item_local`.

**Data**: 2026-05-18
**Branch**: Tekt
**Cap LOC**: hard 200 / soft 150 L3 — **respeitado** (~+112 LOC não-test).

---

## §0 — Resumo executivo

P279 fecha **uma das três pendências documentadas em P278** (transparency
de Group emit em `draw_item_local`): a do **Image arm**. Os arms
Text / Glyph / Line preservam stubs documentados — emit real depende
de threading do font scenario (Type1 vs CIDFont single-font vs multifont)
através dos 3 stream-builders `build_page_stream_*`, scope arquitectural
M-magnitude diferido para passo dedicado **P280.X-bis-text-emit-em-group-
3-font-scenarios**.

### Critério de sucesso (5 dos 5 cumpridos)

1. ✅ Fase A diagnóstico produzido com escolha narrow scope justificada.
2. ✅ L0 `infra/export.md` actualizado; hash propagado (`export.rs:c1a785a3`).
3. ✅ Image arm em `draw_item_local` emite `/ImN Do` correcto (cm matrix
   `w 0 0 h x y cm` + `/Im{N} Do` envolto em `q ... Q`).
4. ✅ 6 testes P279 verdes (3 directos image-em-group + 3 regressão /
   stub / smoke).
5. ✅ 2 611 testes pré-existentes preserved bit-exact (typst-core 2 187
   + typst-infra 424); lint zero; build limpo.

---

## §1 — Fase A: narrow scope Image (decisão crítica)

`diagnosticos/diagnostico-p279-text-image-em-group.md`.

**Spec P279 original** pedia bug fix para 4 variantes: Text, Glyph, Line,
Image. Fase A 8/8 gates inicial **disparou em 3/8** quando se considera
Text + Glyph com o cap hard 200 LOC:

- **Gate-1 (cap LOC)**: spec original projecta ~280 LOC com Text/Glyph
  (3 stream-builders `build_page_stream_type1` / `cidfont` / `multifont`
  têm signatures de fonte distintas; cascading do `font_scenario` requer
  +1 param em cada caller). Excede cap hard 200.
- **Gate-3 (acoplamento extra)**: introduzir tipo `FontScenario` em L3
  ou novo trait `Encoder` é cleanup arquitectural não-funcional.
- **Gate-7 (LOC-pattern P273)**: P273.13 fez render real Shape em Group
  N=1; N=2 fica reservado para Image isolada — adicionar Text simultâneo
  bloqueia análise de sub-padrão emergente.

**Decisão Fase A**: narrow scope **Image-only** (opção α-narrow);
Text/Glyph/Line preservam stubs documentados com pointer para passo
P280.X-bis-text-emit-em-group-3-font-scenarios. Image é a variante
**independente de font scenario** (XObject ref por Arc pointer), pelo
que isolar Image consume +0 LOC de threading de fonte.

Pós-narrow Fase A: **0/8 gates** disparam.

---

## §2 — Materialização

### §2.1 — L0 `infra/export.md` (Protocolo de Nucleação)

Adicionada secção **"draw_item_local — emit local dentro de Group
(P273.13 + P279)"** documentando:

- Signature actualizada: `+2 params ptr_to_idx + img_refs` (cascade
  parameter, opção α-narrow Fase A).
- Image arm: emite `q\n{w} 0 0 {h} {x} {y} cm\n/{name} Do\nQ\n`.
- Text/Glyph/Line: stubs documentados; pendência explícita para
  P280.X-bis com motivação font scenario threading.
- Invariante 3-callers preservado (`build_page_stream_type1/cidfont/
  multifont` cascadeiam params idênticos).

Hash propagado via `crystalline-lint --fix-hashes .`: `export.rs:c1a785a3`.

### §2.2 — L3 código (`03_infra/src/export.rs`)

#### `draw_item_local` — Image arm + cascade

Signature actualizada (+2 params):

```rust
fn draw_item_local(
    item:       &FrameItem,
    ops:        &mut String,
    ptr_to_idx: &HashMap<usize, usize>,  // P279
    img_refs:   &[ImageRef],             // P279
)
```

Image arm:

```rust
FrameItem::Image { pos, data, width, height, .. } => {
    let ptr = Arc::as_ptr(data) as usize;
    if let Some(&idx) = ptr_to_idx.get(&ptr) {
        ops.push_str(&format!(
            "q\n{:.3} 0 0 {:.3} {:.3} {:.3} cm\n/{} Do\nQ\n",
            width.val(), height.val(),
            pos.x.0, pos.y.0,
            img_refs[idx].name,
        ));
    }
    // ptr_to_idx.get → None significa scan_all_images omitiu a
    // imagem (PNG inválido detectado em process_image_item); arm
    // silencia gracefully.
}
```

Text / Glyph / Line arms: stubs documentados apontando para
P280.X-bis-text-emit-em-group-3-font-scenarios.

#### `scan_all_images` — recursão em Groups (scope creep arquitectural)

Bug latent **pré-existente** detectado durante materialização:
`scan_all_images` iterava apenas `page.items` top-level. Images dentro
de `FrameItem::Group { items: child, .. }` (criadas via `Content::Transform`
ou `Content::Block` com clip) não eram registadas em `ptr_to_idx`. Sem
registo, o `ptr_to_idx.get(ptr)` em `draw_item_local` retornaria `None`
e o `/Im1 Do` nunca seria emitido — bug funcional silencioso.

**Padrão idêntico a P273.10 §A.7** (`scan_all_gradients` precisou de
recursão análoga para detectar Shapes com gradient em Groups).

Fix: helper interno `fn walk(items: &[FrameItem], ...)` recursivo;
lógica per-Image extraída para função privada **`process_image_item`**
para evitar duplicação (detect_format + alocação ObjectIDs +
ImageXObject::Jpeg/Png).

#### `xobject_resources_for_page` — recursão em Groups (análoga)

Mesma falha latent: o `/XObject << /Im1 X 0 R ... >>` resource dict
de uma página com Image dentro de Group ficaria vazio → `/Im1 Do`
emitido por `draw_item_local` referenciaria recurso inexistente
(PDF reader: warning "named resource not found"). Fix análogo:
helper `walk` recursivo dentro da função, mesma estrutura que
`scan_all_images`.

#### Callers actualizados (3 + 1)

`build_page_stream_type1` / `_cidfont` / `_multifont` cascadeiam
`&ptr_to_idx, &img_refs` para `draw_item_local`. Chamada recursiva
interna em `draw_item_local` (arm Group) cascade idêntico.

### §2.3 — Testes P279 (6 verdes)

`03_infra/src/export.rs` linhas 8801–9037 (~237 LOC test block):

| Teste | Verifica |
|-------|----------|
| `p279_image_top_level_preserved` | Imagem top-level continua a emitir `/Im1 Do` (regressão N=0). |
| `p279_image_em_group_emite_xobject_ref` | Imagem dentro de Group emite `/Im1 Do` no stream da página (N=1). |
| `p279_image_em_group_preserva_xobject_dedup` | Mesmo Arc usado top-level + dentro de Group: 1 XObject, 2 referências. |
| `p279_image_em_nested_groups` | Group → Group → Image: recursão N=2 OK. |
| `p279_text_em_group_continua_stub_documentado` | Text arm continua stub (não emit) — guarda contra introduzir emit parcial sem font scenario. |
| `p279_render_real_groups_n2_smoke` | Sub-padrão "Render real Groups" N=2 cumulativo (P273.13 + P279) — anti-padrão over-formalização. |

---

## §3 — Padrões emergentes (não formalizados; tracking cumulativo)

### Sub-padrão "Render real Groups" N=2 cumulativo

- **P273.13**: Shape em Group → emit real (path ops dentro de `q/Q`).
- **P279**: Image em Group → emit real (`/ImN Do` dentro de `q/Q`).
- **N actual**: 2 (Shape + Image).
- **Limiar formalização**: N≥3-4.
- **Pendência reaplicação**: Text/Glyph/Line em P280+; Curve em
  ADR-0078 §sub-fase b.

Anti-padrão **over-formalização** (P273.17 §0) → não formalizar agora.

### Sub-padrão "Narrowing within passo por cap LOC" N=2 cumulativo

- **P278 sub-op 3**: spec pedia draw_item_local Text/Line/Glyph/Image
  emit real → reformulado para transparency (4 stubs documentados).
- **P279**: spec pedia Text/Image/Line/Glyph bug fix → reformulado
  para Image-only narrow scope.
- **N actual**: 2.
- **Mecanismo**: Fase A detecta gate-1 (cap LOC) → escolhe sub-conjunto
  factually independente → preserva resto como pendência nominal.

Limiar idem N≥3-4.

### Sub-padrão "Scope creep arquitectural por falha latent em walker top-level"

- **P273.10 §A.7**: `scan_all_gradients` precisou de recursão em Groups.
- **P279**: `scan_all_images` + `xobject_resources_for_page` precisaram
  de recursão análoga.
- **N actual**: 2 (gradient + image, 3 funções no total).
- **Mecanismo**: walker que itera apenas `page.items` top-level falha
  silenciosamente quando estructura ganha Group como contentor; bug
  é descoberto **apenas no momento em que a feature dependente da
  registry passa a renderizar dentro de Group**.
- **Hipótese**: walkers análogos podem existir para text fonts, glyphs,
  ou outras estructuras de recurso — auditoria futura.

---

## §4 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L3 não-test net | +112 (cap hard 200 / soft 150 respeitados) |
| LOC L3 testes P279 | +237 (não conta cap) |
| LOC L0 export.md | +~50 (secção draw_item_local) |
| Hash L0 propagado | `export.rs:c1a785a3` |
| Testes P279 novos | 6/6 verdes |
| Testes pré-existentes typst-core | 2 187 preserved bit-exact |
| Testes pré-existentes typst-infra | 424 preserved bit-exact |
| Lint | zero violations |
| Build | clean (warnings pré-existentes em foundations.rs/import-Pt) |

---

## §5 — Pendências explícitas (futuro)

| ID nominal | Descrição | Magnitude |
|------------|-----------|-----------|
| P280.X-bis-text-emit-em-group-3-font-scenarios | Text arm em Group: emit real com cascading de `FontScenario` através de `build_page_stream_type1/cidfont/multifont`. | M |
| P280.X-bis-line-emit-em-group | Line arm em Group: emit real (path ops análogos a Shape). | XS |
| P280.X-bis-glyph-emit-em-group | Glyph arm em Group: idem Text, mas glyph indices directos. | S |
| Auditoria walkers top-level | Procurar walkers análogos a `scan_all_gradients` / `scan_all_images` que falhem silenciosamente em Groups (potencial bug latent classe). | S |

---

## §6 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: P279 é puramente L3; zero alterações
  L1.
- ✅ **ADR-0054 graded**: narrow scope α-narrow é menor mudança suficiente.
- ✅ **ADR-0085 diagnóstico imutável**: Fase A produzido pré-código;
  decisão narrow scope documentada com gates disparos.
- ✅ **ADR-0094 cap LOC Pattern 1**: hard 200 / soft 150 respeitados
  (~+112 net não-test).
- ✅ **ADR-0097 scope-out reconfirmado**: Text/Glyph/Line ficam como
  pendências nominais com mecanismo arquitectural identificado
  (font scenario threading 3 stream-builders).
- ✅ **Protocolo Nucleação**: L0 redigido antes de código L3; hash
  propagado; testes primeiro (3 directos falharam pré-fix → verdes
  pós-fix); header `@prompt-hash` correcto em `export.rs`.
