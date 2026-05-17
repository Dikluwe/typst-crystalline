# Diagnóstico — Coons CMYK activação opt-in flag ON (P270.4.A)

**Status**: imutável após criação (per ADR-0085).
**Data**: 2026-05-17.
**Passo**: P270.4 (Coons CMYK activado; cluster L3 24/24 absoluto).
**Décimo segundo consumo directo de fonte** vanilla (P262-P270.3 +
**P270.4 verificação ADR-0092 estrutura + ISO 32000-1 §7.5.7.4
Coons CMYK extension**).
**Origem**: spec `00_nucleo/materialization/typst-passo-270.4.md` §1.

---

## §A.1 — ADR-0092 verificação estrutura disponível P270.3

`03_infra/src/export.rs` linhas 820-991 (P270.3):

```rust
#[allow(dead_code)]
fn bezier_control_points_for_arc(
    center: (f32, f32), radius: f32,
    start_angle: f32, end_angle: f32,
) -> [(f32, f32); 2];

#[allow(dead_code)]
fn compute_coons_patches_n_stops(conic: &Conic) -> usize;

#[allow(dead_code)]
fn emit_conic_coons_stream(conic: &Conic) -> Vec<u8>;
```

**3 helpers P270.3 disponíveis** com `#[allow(dead_code)]`. P270.4
remove as marcações (helpers usados pelo dispatcher pós-activação).

**Estrutura `emit_conic_coons_stream` P270.3 RGB**:
- 37 bytes per patch: 1 flag + 24 control points + 12 RGB corner colors.
- Layout: 12 control points (paridade ISO 32000-1 §7.5.7.4) + 4 corners
  × 3 bytes RGB.
- Template estrutural reutilizado para CMYK variant.

§política condição 1 não accionada (helpers preserved limpos).

---

## §A.2 — PROPOSTA `emit_conic_coons_stream_cmyk` variant CMYK

Paridade estrutural `emit_conic_coons_stream` P270.3 RGB:
- 12 control points × 2 coord bytes = 24 bytes per patch (preserved).
- 4 corner colors × 4 bytes CMYK = 16 bytes per patch (vs 12 RGB).
- 1 flag byte + 24 control points + 16 corner CMYK = **41 bytes
  per patch** (vs 37 RGB).
- N stops → N patches → **41N bytes total**.

Extração CMYK por corner via pattern-match `Color::Cmyk` (paridade
`multispace_sample_stops_linear_cmyk` P270.2):

```rust
match stop.color {
    Color::Cmyk { c, m, y, k } => (c, m, y, k),
    _ => {
        let (r, g, b, _) = stop.color.to_rgba_f32();
        rgb_to_cmyk(r, g, b)  // helper P270.2 reused
    }
}
```

**`rgb_to_cmyk` reutilizado literal** de P270.2 (linha 578 export.rs).
Sub-padrão "Reutilização literal helpers cross-passos" estende.

---

## §A.3 — PROPOSTA dispatcher Conic em emit_gradient_objects

Branch novo P270.4 paridade pattern P270.2 Linear+Radial:

```rust
GradientObjectKind::Conic(conic) => {
    if conic.space == ColorSpace::Cmyk {
        // P270.4 — Type 6 Coons CMYK
        let stream = emit_conic_coons_stream_cmyk(conic);
        let header = format!(
            "<< /ShadingType 6 \
               /ColorSpace /DeviceCMYK \
               /BitsPerCoordinate 8 \
               /BitsPerComponent 8 \
               /BitsPerFlag 8 \
               /Decode [0 1 0 1 0 1 0 1 0 1 0 1] \
               /Length {} >>\nstream\n",
            stream.len(),
        );
        let mut shading_bytes = header.into_bytes();
        shading_bytes.extend_from_slice(&stream);
        shading_bytes.extend_from_slice(b"\nendstream");
        // ... emit shading raw bytes per P268 pattern.
    } else {
        // P268+P268.2 preserved literal (Type 4 Gouraud RGB-family).
        let _ = multispace_sample_stops_conic(conic, 16);
        let n_adaptive = compute_adaptive_n_conic(conic);
        let stream = emit_conic_gouraud_stream(conic, n_adaptive);
        // ...
    }
}
```

**Decisão arquitectural**: 2 emit paths Conic AGORA AMBOS ACTIVOS
(Cenário A revisado fechado).

---

## §A.4 — PROPOSTA shading dictionary CMYK

| Field | Conic RGB (Gouraud P268+P268.2) | Conic CMYK (Coons P270.4) |
|---|---|---|
| `/ShadingType` | 4 (Free-Form Gouraud) | **6 (Coons Patch Mesh)** |
| `/ColorSpace` | `/DeviceRGB` | **`/DeviceCMYK`** |
| `/BitsPerCoordinate` | 8 (preservado) | 8 |
| `/BitsPerComponent` | 8 | 8 |
| `/BitsPerFlag` | 8 | 8 |
| `/Decode` | `[0 1 0 1 0 1 0 1 0 1]` (5 pares: x,y,r,g,b) | **`[0 1 0 1 0 1 0 1 0 1 0 1]`** (6 pares: x,y,c,m,y,k) |

---

## §A.5 — Tests P270.3 baseline preserved

15 tests P270.3 (8 unit + 4 E2E + 3 snapshot) verdes literal P270.4:
- Helpers Coons P270.3 estrutura preserved (apenas perdem
  `#[allow(dead_code)]`).
- `emit_conic_coons_stream` RGB body literal preserved.
- Dispatcher Conic default OFF → arm "else" pipeline Gouraud preserved.

§política condições 5 + 8 satisfeitas.

---

## §A.6 — Adaptive N NÃO se aplica a Coons (clarificação)

Coons strategy "1 patch per stop" — N = `conic.stops.len()`. **Não
há adaptive N a recalibrar** (apenas em Gouraud P268.2 onde N=32
default + adaptive hybrid 1+2 factor_delta=256).

Sub-decisão prévia "recalibrar factor_delta CMYK" foi tomada para
Conic Gouraud P268.2 (N adaptive Oklab ΔE). **NÃO aplicável a Coons
CMYK** — preservada reserva para refinos futuros qualidade visual
(candidato P-Gradient-Adaptive-Multispace).

---

## §A.7 — Defaults preservam P262-P270.3

- `space != Cmyk` → branch "else" → `emit_conic_gouraud_stream`
  literal preserved.
- 2560 baseline bit-exact preserved.
- `space == Cmyk` → branch novo P270.4 → Coons CMYK.

§política condições 5 + 8 satisfeitas absolutas.

---

## §A.8 — Bug #4422 resolvido para Conic CMYK por construção

Cristalino emit `/ColorSpace /DeviceCMYK` correcto (paridade P270.2
Linear+Radial). pdfkit #532 análogo confirma causa raiz universal:
dictionary errado por wrapper intermediário.

Cluster Gradient L3 emit pós-P270.4 resolve bug #4422 para **3
variants × CMYK** absoluto:
- Linear CMYK (P270.2 directo /ShadingType 2).
- Radial CMYK (P270.2 directo /ShadingType 3).
- **Conic CMYK (P270.4 directo /ShadingType 6 Coons)**.

---

## §A.9 — Reader compatibility Type 6 + DeviceCMYK

Industry research P270.3 §A.13 consolidada:
- **Cairo**: Type 6 + CMYK suporte universal 20+ anos.
- **Inkscape**: Type 7 Tensor (similar; Type 6 funcional).
- **Adobe Reader**: Type 6 + CMYK suporte universal.
- **Apple Preview**: Type 6 + CMYK suporte universal.
- **pdf.js**: Type 6 + CMYK suporte adequado (não problemático como
  Type 4 #6283).
- **PDFBOX**: Type 6 + CMYK suporte (vs Type 4 historical broken
  PDFBOX-2100).

**Reader compatibility universal esperado** — Coons strategy
mesh-based industry-aligned vs Gouraud que tem problemas Type 4.

PDF/A compliance: `/DeviceCMYK` directo sem ICC profile; refino
futuro candidato **P-Gradient-CMYK-ICC** (krilla paridade ICC).

---

## §A.10 — Estimativa cap LOC P270.4

| Componente | LOC estimado | Cap hard | Cap soft |
|---|---|---|---|
| `emit_conic_coons_stream_cmyk` variant | ~50-60 | 200 | 120 |
| Dispatcher branching Conic | ~30-40 | | |
| Shading dictionary CMYK format string | ~15-20 | | |
| Remoção `#[allow(dead_code)]` × 3 | -3 | | |
| **L3 production total** | **~92-117** | **200 (folga 41-54%)** | **120 (folga 3-23%)** |
| Tests (12) | ~150-200 | 18 testes | 12 testes |

**L3 production ~100 LOC**. Cap hard 200 respeitado (folga ~50%).
Cap soft 120 respeitado (folga ~20%).

Tests 12 exacto. Cap hard 18 respeitado (folga 33%). Cap soft 12
exacto.

---

## §A.11 — Cenário detectado: B1 fecho conceptual

**B1 confirmado**: activação opt-in trivial — helpers P270.3
preparados, estrutura ADR-0092 estabelecida, dispatcher branching
pattern P270.2 reutilizável.

§política condição 1 não accionada (helpers limpos P270.3).
§política condição 2 não accionada (sub-decisão adaptive N não
aplicável; clarificada §A.6).

---

## §A.12 — Decisão arquitectural — cluster L3 24/24 absoluto

Cluster Gradient L3 emit pós-P270.4:

| Variant | 7 RGB-family + perceptual | CMYK |
|---------|-----|-----|
| Linear | P270.1 ✓ `/DeviceRGB` Function 3-comp | **P270.2 ✓** `/DeviceCMYK` Function 4-comp |
| Radial | P270.1 ✓ `/DeviceRGB` (focal_* P269) | **P270.2 ✓** `/DeviceCMYK` |
| **Conic** | **P268+P268.2 ✓** `/ShadingType 4` Gouraud RGB | **P270.4 ✓** `/ShadingType 6` Coons CMYK |

**Cluster L3 24/24 absoluto** — 3 variants × 8 spaces materializados.

---

## §A.13 — Cluster Gradient L1+stdlib+L3 emit feature-complete absoluto

**Marco arquitectural máximo** pós-P270.4:
- L1 sample: 3 variants × 8 spaces (P270).
- Stdlib named args: 3 variants × 8 spaces (P270).
- L3 PDF emit: 3 variants × 8 spaces (P270.1 + P270.2 + P270.4).

**24 combinações user-facing total** completamente materializadas
end-to-end (Color → Gradient → Stdlib → L3 PDF emit).

Série P270 completa: P270 (L1+stdlib) + P270.1 (L3 7 spaces) + P270.2
(L3 CMYK Linear+Radial) + P270.3 (Coons RGB infra) + **P270.4 (Coons
CMYK activação)** = cluster Gradient feature-complete a nível
user-facing.

---

## §A.14 — Sumário decisões diagnóstico

| Item | Decisão |
|---|---|
| §A.1 Helpers P270.3 disponíveis | 3 helpers com `#[allow(dead_code)]` — P270.4 remove marcações |
| §A.2 emit_conic_coons_stream_cmyk variant | 41 bytes per patch (vs 37 RGB) |
| §A.3 Dispatcher branching | Pattern P270.2 Linear+Radial reutilizado |
| §A.4 Shading dict CMYK | `/ShadingType 6` + `/DeviceCMYK` + `/Decode 6 pares` |
| §A.5 Tests P270.3 baseline | 15 tests preservados literal |
| §A.6 Adaptive N não aplicável | Clarificação sub-decisão prévia |
| §A.7 Defaults preservam P270.3 | 2560 baseline bit-exact |
| §A.8 Bug #4422 resolvido absoluto | 3 variants × CMYK |
| §A.9 Reader compatibility | Universal esperado (Cairo 20+ anos) |
| §A.10 Cap LOC | ~100 LOC L3 (cap hard 200 folga 50%) |
| §A.11 Cenário B1 | Confirmado |
| §A.12 Cluster L3 24/24 absoluto | Materializado P270.4 |
| §A.13 Cluster feature-complete absoluto | Marco arquitectural máximo |

**Diagnóstico aprovado para passagem a sub-passo P270.4.B (anotações
cumulativas).**

---

## §A.15 — Referências

- Spec P270.4: `00_nucleo/materialization/typst-passo-270.4.md`.
- Cristalino L3 P270.3: `03_infra/src/export.rs:820-991` (3 helpers
  Coons com `#[allow(dead_code)]`).
- Cristalino L3 P270.2: `03_infra/src/export.rs:578` (`rgb_to_cmyk`
  fallback helper reused) + `601/636` (`multispace_sample_stops_*_cmyk`
  pattern).
- Cristalino L1 P270: `01_core/src/entities/gradient.rs:432`
  (`interpolate_cmyk` dispatcher arm).
- Cristalino P257 Color::Cmyk variant.
- ADR-0092 — Conic Coons Patches (anotação cumulativa P270.4
  activação).
- ADR-0091 — ColorSpace runtime + CMYK strategy (§"Conic CMYK
  scope-out preserved" revogação final P270.4).
- ADR-0083 — Color paridade (§DeviceCMYK revogação final P270.4).
- ADR-0089 — Gradient Conic-only (anotação cumulativa P270.4 dual
  emit activo).
- ADR-0054 — Perfil graded (anotação cumulativa P270.4 fecho cluster).
- ADR-0085 — Diagnóstico imutável (décimo segundo consumo).
- ISO 32000-1 §7.5.7.4 (Type 6 Coons Patch Mesh CMYK extension).
- Industry research P270.3 §A.13 (Cairo + Inkscape + Adobe + Apple
  Preview + pdf.js + PDFBOX + matplotlib + Stanislaw Adaszewski).
