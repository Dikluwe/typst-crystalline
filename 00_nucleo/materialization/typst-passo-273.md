# typst-passo-273 — P-Gradient-Relative-Custom (activa `relative: RelativeTo` cross-variant)

**Magnitude**: M (cap composto: L1 hard ≤ 80 / soft ≤ 50 LOC + stdlib hard ≤ 50 / soft ≤ 30 LOC + L3 hard ≤ 150 / soft ≤ 100 LOC + testes hard ≤ 30 / soft ≤ 22).
**Cluster**: Visualize / Gradient (activação feature cross-variant; refino L1+stdlib+L3).
**Tipo**: passo principal P273. Refino estratégico — activa campo `relative: RelativeTo` em Linear/Radial/Conic preserved scope-out em ADRs anteriores.
**Origem**: relatório P272 §"sequência pós-P272" + relatório P271 §"sequência pós-P271" candidato directo M.
**Sequência**: P272 (cluster L3 emit estratégia única Coons 8/8 simplified) → **P273 (relative RelativeTo cross-variant)** → cluster Gradient ganha mais 1 feature user-facing (paridade vanilla).
**Decisões prévias**:
- Utilizador escolheu (1) Numeração P273 passo principal; (2) Pesquisa industry proactiva primeiro (consolidada inline §"Pesquisa empírica industry"); (3) Parent bbox passado via contexto Rust (não PDF `/Matrix`).
- Decisão (3) é arquitecturalmente significativa — diverge solução PDF spec elegante (`/Matrix` shading dictionary) em favor de transform em Rust auditável. Trade-off: +LOC L3 vs +clarity Rust pipeline.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L1/stdlib/L3 nunca antes de prompt L0 + ADR. Ordem: diagnóstico → ADR-0091 anotação cumulativa P273 + ADR-0087/0088/0089/0092 + ADR-0054 anotações cumulativas paralelas → prompt L0 → fix-hashes → testes-primeiro → código.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-relative-custom-passo-273.md` imutável. **Décimo quarto consumo directo de fonte** (P262-P272 + **P273 vanilla `relative: RelativeTo` literal + ISO 32000-1 §7.5.4.2 PDF /Matrix shading**).

3. **Sem ADR nova P273** — refino cross-variant; pattern análogo P269 (focal_*) + P270 (space). ADR-0091 §"ColorSpace runtime + CMYK strategy" expandida cumulativamente cobrindo cross-variant runtime fields. Sub-padrão "Anotação cumulativa em vez de ADR nova" **N=11 → N=12 cumulativo** consolidação clara persistente.

4. **ADR-0091 anotação cumulativa P273** — centro de aplicação cross-variant runtime fields. ADR-0091 §"Cross-variant runtime fields" nova secção P273 lista cumulativamente:
    - `space: ColorSpace` (P270).
    - `focal_center/focal_radius: Ratio` (P269; Radial only mas pattern análogo).
    - **`relative: RelativeTo`** (P273; este passo).

5. **ADR-0087/ADR-0088/ADR-0089/ADR-0092 anotações cumulativas P273** — cada variant ganha campo `relative` (4 anotações paralelas).

6. **ADR-0054 anotação cumulativa P273** — perfil graded DEBT-1 cobertura estendida.

7. **ADR-0083 preservada literal** — Color paridade vanilla preservada.

8. **ADR-0029 verificação obrigatória Fase A §A.X** — campo `relative: RelativeTo` em L1 deve ser pura metadata (não invade pureza física). Sample method preserved literal.

9. **ADR-0039 preservado** — TextStyle intocado.

10. **ADR-0018 preservado** — implementação autónoma; sem dependências externas.

11. **Crystalline-lint zero violations** obrigatório.

12. **Reutilização literal helpers cross-passos** **N=11 → N=12 cumulativo**:
    - Pattern `Smart<T>` (P266 + P270 — defaults Auto).
    - L1 enum pattern (P270 `ColorSpace`).
    - Stdlib named arg parsing (P270 + P269 templates).
    - L3 dispatcher dual pattern (P270.4 + P272).

13. **Vanilla read-first autorizado** — `lab/typst-original/crates/typst-library/src/visualize/gradient.rs` `relative: Smart<RelativeTo>` literal.

14. **Pesquisa industry proactiva aplicada** (sub-padrão N=4 → **N=5 cumulativo** limiar formalização clara muito ultrapassado):
    - PDF `/Matrix` em shading dictionary existe literal (iText/PDFTron APIs `setMatrix`/`getMatrix`).
    - Type 2/3/6 shading dictionaries aceitam `/Matrix` entry — transforma unit space → target coordinate space.
    - Pattern dictionary também tem `/Matrix` afectando shading coordinates.
    - **Decisão arquitectural cristalino**: parent bbox via contexto Rust (utilizador escolheu) — PDF `/Matrix` permanece identity; cristalino L3 calcula coordinates transformadas em Rust antes de emit.
    - Trade-off documentado: +LOC L3 (~30-50) vs +clarity Rust pipeline auditável vs PDF reader interpretation variation.

15. **Sub-padrão "Aplicação meta-ADR (ADR-0094)" N=1 → N=2 cumulativo** — segunda aplicação Cap LOC hard/soft Pattern 1 pós-formalização P271.

16. **Regressão tests P262-P272 preservada literal** — 2557 baseline preservado. Defaults `relative: Auto` resolve `Self`; pipeline P272 preserved bit-exact.

17. **Cap LOC hard vs soft explícito** (6ª aplicação consolida sub-padrão N=6):
    - Caps por camada conforme tabela acima.
    - Cap soft estouro regista relatório; cap hard estouro dispara §política.

---

## §1 — Sub-passo P273.A — Diagnóstico empírico vanilla relative + cristalino refactor surface

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-relative-custom-passo-273.md`.

### Comandos exactos a executar

```bash
# 1. Vanilla relative campo + enum + parsing
rg -n "relative:\s*Smart|RelativeTo|Relative::|RelativeTo::Self|RelativeTo::Parent" lab/typst-original/crates/typst-library/src/visualize/gradient.rs | head -30

# 2. Vanilla emit relative resolution (onde Smart<RelativeTo> resolve para Self/Parent)
rg -n "RelativeTo|relative.*resolve|resolve.*relative" lab/typst-original/crates/typst-library/src/visualize/ | head -30

# 3. Vanilla L3 emit usa /Matrix ou Rust transform?
rg -n "/Matrix|Matrix.*\[" lab/typst-original/crates/typst-pdf/src/ 2>/dev/null | head -20

# 4. Cristalino L1 struct fields 3 variants (estado pre-P273)
rg -n "struct Linear|struct Radial|struct Conic|pub stops|pub angle|pub center|pub radius|pub space" 01_core/src/entities/gradient.rs | head -40

# 5. Cristalino stdlib gradient.linear/radial/conic named args parsing (P270 templates)
rg -n "native_gradient_linear|native_gradient_radial|native_gradient_conic|args.named" 01_core/src/rules/stdlib/gradients.rs | head -30

# 6. Cristalino L3 dispatcher Conic pós-P272 (template structural)
rg -n "GradientObjectKind::Conic|emit_conic_coons_stream|conic.space" 03_infra/src/export.rs | head -20

# 7. Cristalino contexto emit_gradient_objects (onde passar parent bbox?)
rg -n "fn emit_gradient_objects|emit_gradient_objects\(" 03_infra/src/export.rs | head -10

# 8. Tests baseline P272
cargo test -p typst-cristalino-infra gradient 2>&1 | tail -10
```

### Estrutura do diagnóstico (§A.1 a §A.16)

```
§A.1 Vanilla `relative: Smart<RelativeTo>` literal:
     - Tipo: Smart<RelativeTo>.
     - RelativeTo enum: { Self, Parent } (cast literal).
     - Default Smart::Auto → resolve_auto produz Self (maior parte casos)
       ou Parent (contexto específico; verificar).
     - Cada variant Linear/Radial/Conic tem campo.

§A.2 Vanilla resolve_auto Smart<RelativeTo>:
     - Verificar literal; cristalino default Auto resolve consistente.
     - Pode ser: Auto → Parent quando inside fill paint context; Self caso contrário.

§A.3 Vanilla L3 emit estratégia:
     - Verificar se vanilla usa PDF /Matrix shading dictionary.
     - Se sim, cristalino diverge intencional (parent bbox Rust contexto per
       decisão utilizador).
     - Se não, vanilla também usa transform Rust — paridade.

§A.4 Cristalino L1 estado pré-P273:
     - 3 structs Linear/Radial/Conic com fields actuais (P262/P264/P267 +
       P269 focal_* Radial + P270 space cross-variant).
     - Sem campo relative.

§A.5 Cristalino stdlib named args pre-P273:
     - Linear: angle, space (P270).
     - Radial: center, radius, focal_center (P269), focal_radius (P269), space (P270).
     - Conic: center, angle, space (P270).
     - **Adicionar named arg "relative"** cross-variant P273.

§A.6 Cristalino L3 dispatcher Conic pós-P272 (estado para extender):
     - if conic.space == Cmyk → Coons CMYK.
     - else → Coons RGB N=stops*4.
     - **Adicionar lógica relative**: se conic.relative == Parent, escalar
       coordinates via parent bbox.

§A.7 PROPOSTA L1 enum RelativeTo (novo):
     - enum RelativeTo { Self_, Parent } (Self_ porque Self é palavra reservada Rust).
     - Default impl que retorna Self_.
     - Smart<RelativeTo> tipo composto.

§A.8 PROPOSTA L1 fields cross-variant:
     - Linear: + relative: Smart<RelativeTo>.
     - Radial: + relative: Smart<RelativeTo>.
     - Conic: + relative: Smart<RelativeTo>.
     - Defaults Smart::Auto.

§A.9 PROPOSTA stdlib named arg "relative":
     - Parsing: args.named("relative") -> Smart<RelativeTo>.
     - Validação: string "self" | "parent" | "auto".
     - Default Auto.

§A.10 PROPOSTA contexto Rust parent bbox:
     - emit_gradient_objects ganha param adicional `parent_bbox: Option<Rect>`.
     - Quando relative resolve Parent, cristalino L3 usa parent_bbox.
     - Quando relative resolve Self, parent_bbox ignorado; pipeline P272 preserved.

§A.11 PROPOSTA L3 dispatcher refactor (cap soft 100 LOC; hard 150):
     - resolve_relative(relative: Smart<RelativeTo>) -> RelativeTo (helper novo).
     - apply_parent_transform(coords, parent_bbox) -> coords_transformed (helper novo).
     - Linear/Radial: /Coords transformed via parent_bbox.
     - Conic: Coons patches centro/radius transformed via parent_bbox.

§A.12 ADR-0029 pureza física L1 verificação:
     - Campo relative é enum metadata; sample() não usa.
     - Não invade pureza física L1.
     - Confirmar.

§A.13 Defaults preservam P272:
     - Smart::Auto → resolve Self (default vanilla).
     - relative == Self → pipeline P272 literal preserved.
     - 2557 baseline bit-exact preserved.

§A.14 Cenário detectado:
     - **B1 fecho conceptual** (refino cross-variant trivial; P272 pipeline preserved).
     - **B2 sub-passos improvável** (todos refinos similares cross-variant).

§A.15 Estimativa cap LOC:
     - L1: ~50 LOC (RelativeTo enum + 3 fields + Smart wrapping + defaults).
     - Stdlib: ~25 LOC (parsing named arg).
     - L3: ~80 LOC (resolve_relative + apply_parent_transform + 2 callsites dispatchers).
     - Tests: ~80-100 LOC (~20-25 tests).
     - Cap hard L1/stdlib/L3 com folga 30-60%.

§A.16 Decisão arquitectural — parent bbox via contexto Rust
     (utilizador escolheu; PDF /Matrix permanece identity; trade-off
     +LOC vs +clarity).
```

### Critério de aceitação Fase A

- §A.1 + §A.2 confirmam vanilla `Smart<RelativeTo>` + resolve_auto.
- §A.3 confirma vanilla strategy (PDF /Matrix vs Rust transform); cristalino diverge se necessário.
- §A.10 + §A.11 confirmam refactor L3 cabe no cap hard 150.
- §A.12 confirma ADR-0029 pureza física L1 preserved.
- §A.13 confirma defaults preservam P272 bit-exact.

---

## §2 — Sub-passo P273.B — Anotações cumulativas (sem ADR nova)

### B.1 — ADR-0091 anotação cumulativa P273 (cross-variant runtime fields)

Adicionar após §"Anotação cumulativa P272":

```
## Anotação cumulativa P273 — Cross-variant runtime fields (`relative: RelativeTo`)

**Data**: 2026-05-17.
**Motivo**: cluster Gradient ganha campo `relative: Smart<RelativeTo>`
cross-variant (Linear/Radial/Conic). Activação user-facing paridade
vanilla.

**Estratégia materializada P273**:
- L1 enum RelativeTo { Self_, Parent } com Default Self_.
- L1 cada variant ganha `relative: Smart<RelativeTo>` (default Auto).
- Stdlib named arg `relative` cross-variant (parsing "self"/"parent"/"auto").
- L3 dispatcher dual:
  - Auto → resolve Self → pipeline P272 preserved literal.
  - Parent → resolve Parent → apply_parent_transform via parent_bbox
    (contexto Rust).

**Decisão arquitectural não-PDF-/Matrix**:
- Cristalino calcula coordinates transformadas em Rust antes de emit.
- PDF `/Matrix` shading dictionary permanece identity.
- Trade-off: +LOC L3 vs +clarity Rust pipeline auditável.
- Alternativa rejeitada (PDF /Matrix): mais elegante PDF spec
  perspective mas reader interpretation variation; cristalino opta
  por controlo Rust.

**Cluster Gradient cross-variant runtime fields cumulativos
materializados** (lista canónica P273):
1. `space: ColorSpace` (P270; 8 spaces).
2. `focal_center/focal_radius: Ratio` (P269; Radial only).
3. **`relative: Smart<RelativeTo>`** (P273; cross-variant).

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=11 → N=12
cumulativo consolidação clara persistente.

**Defaults preservam P272 literal**:
- `Smart::Auto` resolve `Self` (vanilla default).
- Pipeline P272 literal preserved quando relative resolve Self.
- 2557 baseline bit-exact preserved.

**Helpers reutilizados literal**:
- Pattern Smart<T> (P266 + P270 enum templates).
- L1 enum pattern (P270 ColorSpace).
- Stdlib named arg parsing (P270 templates).
- L3 dispatcher dual (P272 RGB/CMYK).
- Sub-padrão "Reutilização literal helpers cross-passos" N=11 → N=12.

**Industry research consolidada P273 (sub-padrão N=4 → N=5)**:
- PDF /Matrix em shading dictionary existe (iText/PDFTron APIs).
- Type 2/3/6 aceitam /Matrix entry — transform unit space → target.
- Cristalino diverge per decisão utilizador (parent bbox via contexto
  Rust).
```

### B.2 — ADR-0087 anotação cumulativa P273

```
## Anotação cumulativa P273 — Linear ganha `relative: RelativeTo`

Linear L1 struct ganha campo `relative: Smart<RelativeTo>` (default
Auto resolve Self). Stdlib named arg `relative` parsing. L3 dispatcher
ganha branch transform via parent_bbox quando relative == Parent.
Defaults preservam P262/P263/P270.1/P270.2 literal. Ver ADR-0091
§"Anotação cumulativa P273".
```

### B.3 — ADR-0088 anotação cumulativa P273

```
## Anotação cumulativa P273 — Radial ganha `relative: RelativeTo`

Radial L1 struct ganha campo `relative: Smart<RelativeTo>` (default
Auto resolve Self). Preserved P269 focal_*; preserved P270 space.
Stdlib named arg `relative` parsing. L3 dispatcher ganha branch
transform via parent_bbox quando relative == Parent. Defaults
preservam P264/P265/P269/P270.1/P270.2 literal. Ver ADR-0091
§"Anotação cumulativa P273".
```

### B.4 — ADR-0089 anotação cumulativa P273

```
## Anotação cumulativa P273 — Conic ganha `relative: RelativeTo`

Conic L1 struct ganha campo `relative: Smart<RelativeTo>` (default
Auto resolve Self). Stdlib named arg `relative` parsing. L3
dispatcher Conic unified P272 ganha branch transform via parent_bbox
quando relative == Parent (Coons patches centro/radius transformed).
Defaults preservam P267/P272 literal (RGB N=stops*4 + CMYK N=stops).
Ver ADR-0091 §"Anotação cumulativa P273".
```

### B.5 — ADR-0092 anotação cumulativa P273

```
## Anotação cumulativa P273 — Coons patches transformados via parent_bbox

Conic Coons dispatcher P272 estendido — quando relative resolve
Parent, Coons patches centro/radius transformados via parent_bbox
em Rust (PDF /Matrix permanece identity). Pipeline P272 preserved
quando relative resolve Self. Ver ADR-0091 §"Anotação cumulativa
P273".
```

### B.6 — ADR-0054 anotação cumulativa P273

```
P273 — cluster Gradient ganha relative: RelativeTo cross-variant
runtime field; lista canónica runtime fields agora 3 elementos
(space + focal + relative). Perfil graded DEBT-1 cobertura estendida.
```

### B.7 — L0 `entities/gradient.md` anotação P273

Adicionar após anotação P272:

```
**Anotação P273**: cluster Gradient cross-variant ganha campo
`relative: Smart<RelativeTo>` (default Auto resolve Self preserva
P272). Stdlib named arg `relative` cross-variant. L3 dispatcher
dual:
- Self (default) → pipeline P272 preserved literal.
- Parent → coordinates transformadas via parent_bbox (contexto
  Rust; PDF /Matrix identity).

RelativeTo enum { Self_, Parent } com Default Self_. Sub-padrão
"Anotação cumulativa em vez de ADR nova" N=12; "Reutilização literal
helpers" N=12; "Cap LOC hard/soft" N=6. Ver ADR-0091 §"Anotação
cumulativa P273".
```

### B.8 — Hashes propagados

`crystalline-lint --fix-hashes` propaga hashes em L0. Zero violations.

---

## §3 — Sub-passo P273.C — Materialização L1+stdlib+L3 (testes primeiro)

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0091 + ADR-0087/0088/0089/0092 + ADR-0054 anotações §2.
3. L0 anotação §2.B.7.
4. `crystalline-lint --fix-hashes`.
5. **Testes-primeiro** — adicionar ~20-25 testes ANTES de qualquer LOC.
6. L1 código — RelativeTo enum + 3 fields + Smart wrapping + defaults.
7. Stdlib código — named arg parsing 3 variants.
8. L3 código — resolve_relative + apply_parent_transform + 3 dispatcher branches.
9. Verificação final.

### Cap LOC

- **L1 hard**: 80 LOC. **L1 soft**: 50 LOC.
- **Stdlib hard**: 50 LOC. **Stdlib soft**: 30 LOC.
- **L3 hard**: 150 LOC. **L3 soft**: 100 LOC.
- **Testes hard**: 30. **Testes soft**: 22.

### Alteração L1 esperada

```rust
// 01_core/src/entities/gradient.rs P273

/// Define a que bounding box o gradient é relativo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RelativeTo {
    #[default]
    Self_,  // Self é palavra reservada Rust; trailing underscore.
    Parent,
}

// 3 variants ganham field:
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    pub space: ColorSpace,  // P270
    pub relative: Smart<RelativeTo>,  // P273 — default Auto
}

pub struct Radial {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    pub focal_center: Axes<Ratio>,  // P269
    pub focal_radius: Ratio,  // P269
    pub space: ColorSpace,  // P270
    pub relative: Smart<RelativeTo>,  // P273
}

pub struct Conic {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub angle: Angle,
    pub space: ColorSpace,  // P270
    pub relative: Smart<RelativeTo>,  // P273
}

// Resolve helper (L1 ou L3)
impl<T: Default> Smart<T> {
    pub fn or_default(self) -> T {
        match self {
            Smart::Auto => T::default(),
            Smart::Custom(v) => v,
        }
    }
}

// Construtores preserved:
pub fn linear(stops, angle) -> Self {
    // P273 default Auto:
    Gradient::Linear(Arc::new(Linear {
        stops, angle,
        space: ColorSpace::Oklab,
        relative: Smart::Auto,
    }))
}
// linear_with_relative construtor novo opcional.
```

### Alteração stdlib esperada

```rust
// 01_core/src/rules/stdlib/gradients.rs P273

pub fn native_gradient_linear(args) -> SourceResult<Value> {
    // P262/P270 parsing preserved +
    let relative = match args.named.get("relative") {
        Some(Value::Str(s)) if s == "self" => Smart::Custom(RelativeTo::Self_),
        Some(Value::Str(s)) if s == "parent" => Smart::Custom(RelativeTo::Parent),
        Some(Value::Str(s)) if s == "auto" => Smart::Auto,
        Some(other) => return Err(...),
        None => Smart::Auto,  // default
    };
    
    // Whitelist named estendida com "relative".
    for key in args.named.keys() {
        if !["angle", "space", "relative"].contains(&key.as_str()) { erro... }
    }
    
    Ok(Value::Gradient(Gradient::linear_with_relative(stops, angle, space, relative)))
}

// Análogo native_gradient_radial / native_gradient_conic.
```

### Alteração L3 esperada

```rust
// 03_infra/src/export.rs P273

// Helper resolve relative
fn resolve_relative(relative: Smart<RelativeTo>) -> RelativeTo {
    relative.or_default()  // Auto → Self_
}

// Helper apply parent transform (coordinate transformation)
fn apply_parent_transform(
    local_coords: (f32, f32, f32, f32),
    parent_bbox: Option<Rect>,
) -> (f32, f32, f32, f32) {
    match parent_bbox {
        Some(bbox) => {
            // Escalar local_coords (unit space 0..1) para bbox.
            // ... cálculo transform
        }
        None => local_coords,  // sem parent context; preserves Self behavior.
    }
}

// emit_gradient_objects ganha param adicional
fn emit_gradient_objects(
    &mut self,
    gradient_objs: &[GradientObjectKind],
    parent_bbox: Option<Rect>,  // P273 — novo
) {
    for obj in gradient_objs {
        match obj {
            GradientObjectKind::Linear(linear) => {
                let relative = resolve_relative(linear.relative);
                let coords = match relative {
                    RelativeTo::Self_ => P272_coords_local(linear),  // preserved literal
                    RelativeTo::Parent => apply_parent_transform(P272_coords_local(linear), parent_bbox),
                };
                // ... emit /ShadingType 2 + coords
            }
            // Análogo Radial / Conic
        }
    }
}
```

### Estrutura testes esperada

**Unit L1 RelativeTo + fields** (5 tests):
- `p273_relative_to_default_self`.
- `p273_smart_auto_resolves_self`.
- `p273_smart_custom_parent_resolves_parent`.
- `p273_linear_default_relative_auto`.
- `p273_radial_conic_default_relative_auto`.

**Unit stdlib** (6 tests):
- `p273_stdlib_linear_relative_self`.
- `p273_stdlib_linear_relative_parent`.
- `p273_stdlib_linear_relative_auto`.
- `p273_stdlib_radial_relative_parent`.
- `p273_stdlib_conic_relative_parent`.
- `p273_stdlib_relative_invalido_erro`.

**Unit L3 dispatcher** (5 tests):
- `p273_resolve_relative_auto_self`.
- `p273_apply_parent_transform_identity_no_bbox`.
- `p273_apply_parent_transform_scale_to_bbox`.
- `p273_emit_linear_relative_self_preserva_p263`.
- `p273_emit_linear_relative_parent_transforms_coords`.

**E2E PDF** (4 tests):
- `p273_export_pdf_linear_relative_self_preserva_p270_1`.
- `p273_export_pdf_radial_relative_parent_transforms_coords`.
- `p273_export_pdf_conic_relative_parent_coons_transformed`.
- `p273_export_pdf_cluster_3_variants_relative_coexistem`.

**Regressão P262-P272** (verificar verdes):
- 2557 baseline preserved literal (defaults Auto resolve Self).

Total esperado: 5 + 6 + 5 + 4 = **20 testes**. Cap soft 22 / hard 30; folga.

---

## §4 — Sub-passo P273.D — README + relatório

1. **ADR-0091** anotação cumulativa P273 fechada.
2. **ADR-0087/0088/0089/0092** anotações cumulativas P273 adicionadas.
3. **ADR-0054** anotação cumulativa P273 adicionada.
4. **ADR-0083** preservada literal.
5. **README.md** actualizar:
   - Tabela cobertura Visualize PDF (+~1pp via relative cross-variant; cluster Gradient ganha feature user-facing adicional).
   - Entrada P273 ~50-70 linhas (refino cross-variant; 5 anotações cumulativas).
   - Cross-reference ADR-0091 §"Anotação cumulativa P273".
6. **Distribuição ADRs preservada** — total 81 mantido (sem ADR nova; 5 anotações cumulativas + 1 L0).
7. **Relatório** `00_nucleo/materialization/typst-passo-273-relatorio.md`:
   - Métricas finais (esperado 2557 + 20 = ~2577).
   - Fase A §A.3 vanilla strategy documentada.
   - Diff L1+stdlib+L3 antes/depois.
   - Sub-padrões + N cumulativo (3 atingem limiar formalização clara).
   - Regressão zero 2557 baseline preserved.
   - **Cluster Gradient ganha relative: RelativeTo cross-variant** — paridade vanilla user-facing adicional.

---

## §política de paragem

1. **Fase A §A.2 vanilla resolve_auto Smart<RelativeTo> não-determinístico ou
   contexto-dependente** — cristalino default Auto resolve Self consistente;
   se vanilla resolve Parent em fill paint context, cristalino diverge
   intencional (documentar). §política condição confirmar.

2. **Fase A §A.3 vanilla usa PDF /Matrix em shading dictionary**:
   - Cristalino diverge intencional (decisão utilizador parent bbox via
     contexto Rust).
   - Documentar trade-off literal em ADR-0091 §"Anotação cumulativa P273".

3. **Cap LOC L1 hard (80) ou stdlib (50) ou L3 (150) ameaça ser
   ultrapassado** — refactor maior que estimativa §A.15.

4. **Cap testes hard (30) ameaça ser ultrapassado**.

5. **§A.13 verificação falha**: defaults Smart::Auto não preservam bytes
   P272 literal. §política absoluta.

6. **§A.12 ADR-0029 pureza física L1 invadida** — campo relative em L1
   acoplado a I/O ou state mutável. Refactor expandido.

7. **Crystalline-lint reporta violations** após anotações.

8. **Regressão tests P262-P272** — qualquer test anterior falha. §política
   absoluta.

9. **emit_gradient_objects callers externos param adicional** — outros
   sítios export.rs ou wiring chamam emit_gradient_objects directamente;
   refactor expande. Verificar Fase A.

10. **Parent bbox sem contexto disponível** — alguns call sites podem não
    ter parent bbox; passar `None` deve ser semanticamente equivalente
    a relative Self (per apply_parent_transform None branch).

11. **Anotações cross-ADR 5 ADRs coerência** — cada anotação refere
    ADR-0091 §"Anotação cumulativa P273".

12. **Industry research achados §"Pesquisa empírica industry" factualmente
    incorrectos** — verificar PDF /Matrix shading dictionary literal via
    iText/PDFTron docs.

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N pós-P273 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=11 → N=12 cumulativo consolidação clara** | + P273 ADR-0091 anotada |
| Reutilização literal helpers cross-passos | **N=11 → N=12 cumulativo consolidação clara** | + P273 (Smart<T> + enum + parsing + dispatcher) |
| Cap LOC hard vs soft explícito | **N=5 → N=6 cumulativo consolidação total** | + P273 |
| **Fase A com industry research proactiva** | **N=4 → N=5 cumulativo (limiar formalização clara muito ultrapassado)** | + P273 PDF /Matrix shading research |
| **Aplicação meta-ADR (ADR-0094)** | **N=1 → N=2 cumulativo** | + P273 Cap LOC + industry research aplicação prática |
| Anotação cumulativa cross-ADR | **N=6 → N=7 cumulativo** | + P273 (5 ADRs anotadas + 0054) |
| Diagnóstico imutável (décimo quarto consumo) | **N=18 → N=19 cumulativo** | + P273 |
| Auditoria condicional (ADR-0084) | **N=17 → N=18 cumulativo** | + P273 |
| Auto-aplicação ADR-0065 inline | **N=17 → N=18 cumulativo** | + P273 |

### Marco arquitectural P273

**Cluster Gradient cross-variant runtime fields canónica** materializada
3/3 elementos (space + focal + relative). Cluster ganha mais 1 feature
user-facing paridade vanilla.

**Divergência intencional vanilla PDF /Matrix** documentada — cristalino
calcula coordinates em Rust auditável vs PDF reader interpretation;
trade-off +LOC vs +clarity.

**Sub-padrão "Fase A com industry research proactiva" N=5 cumulativo
limiar formalização clara muito ultrapassado** — confirma valor
metodológico ADR-0094 Pattern 3.

### Sequência pós-P273

Pendências preservadas:
- **P-Gradient-CMYK-ICC** (S-M; krilla paridade ICC; PDF/A compliance).
- **P-Gradient-Adaptive-Multispace** (S; HSL/Oklch banding refino).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 / Stroke<Length> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

---

## §referências cross-passos

- **P262/P264/P267** — Variant L1+stdlib (preservados; campo relative aditivo).
- **P263/P265/P272** — L3 emit templates (preserved; dispatcher relative aditivo).
- **P269** — Radial focal_* (preserved; precedente cross-variant runtime fields).
- **P270 série completa** — ColorSpace runtime + L3 emit (preservado; precedente Smart<T> + named arg).
- **P272** — Coons unified (preserved; dispatcher Conic relative aditivo).
- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa P273 cross-variant runtime fields).
- ADR-0087/0088/0089 — Variant strategies (anotadas cumulativa P273 relative aditivo).
- ADR-0092 — Conic Coons (anotada cumulativa P273 patches transformados).
- ADR-0054 — Perfil graded (anotada cumulativa P273).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 anotação cumulativa aplicado).
- ADR-0094 — Meta-operacional specs (Cap LOC hard/soft + industry research aplicação prática).
- ADR-0029 — Pureza física L1 (verificação §A.12).
- ADR-0085 — Diagnóstico imutável (décimo quarto consumo).

---

## §0.1 — Notas de execução para Claude Code

- **Fase A §A.1 + §A.2 críticas** — confirmar vanilla `Smart<RelativeTo>` literal + resolve_auto default Self.
- **Fase A §A.3 vanilla L3 strategy** — PDF /Matrix vs Rust transform. Cristalino diverge intencional per decisão utilizador.
- **Fase A §A.9 emit_gradient_objects callers** — verificar refactor surface area antes de adicionar param parent_bbox.
- **Fase A §A.12 ADR-0029 pureza física L1** — campo relative deve ser pura metadata (sample method não usa). §política condição 6 absoluta.
- **Defaults Smart::Auto preservam bytes P272** literal — §política condição 5 absoluta. Verificar via `cargo test p262_ p264_ p265_ p267_ p268_ p269_ p270_ p272_`.
- **Regressão tests P262-P272 zero** (2557 baseline) — §política condição 8 absoluta.
- **Industry research consolidada inline ADR-0091 §"Anotação cumulativa P273"** — sub-padrão "Fase A com industry research proactiva" N=5 cumulativo.
- **Anotações cross-ADR 6 ADRs (ADR-0091/0087/0088/0089/0092/0054)** — verificar coerência.
- **Cap hard L1/stdlib/L3 (80/50/150) + testes hard 30** — gate absoluto.
- **Cap soft L1/stdlib/L3 (50/30/100) + testes soft 22** — informativo.
- **Helper Smart<T>::or_default já existe ou criar** — verificar Fase A; se P266 já materializou Smart<T> pattern, reutilizar.
- **Cluster Gradient cross-variant runtime fields canónica 3/3** documentada em relatório §1 + ADR-0091 §"Anotação cumulativa P273".
- **Relatório final esperado**: 2557 + 20 = ~2577 testes verdes; hash drift L0; lint zero; ADRs 81 preservado.
