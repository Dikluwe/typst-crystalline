# Relatório P270.1 — L3 emit multi-space materializado (7 spaces RGB-family + perceptual)

**Data**: 2026-05-17.
**Magnitude**: M+ (real ~45 LOC L3 rename + docs + 33 testes).
**Cluster**: Visualize / Gradient / PDF export (refino L3).
**Tipo**: sub-passo .1 da série P270.
**Spec**: `00_nucleo/materialization/typst-passo-270.1.md`.

---

## §1 — Sumário executivo

L3 emit multi-space materializado via rename cosmético + docs +
tests. Helpers `oklab_sample_stops_*` renomeados
`multispace_sample_stops_*` em `03_infra/src/export.rs`. Body
literal preserved porque P270 já passou L3 multi-space
implicitamente via `<variant>.sample(t)` dispatcher.

### Marco arquitectural P270.1

**Cluster Gradient L3 emit 7/8 spaces materializado** (Oklab/Oklch/
sRGB/Luma/LinearRGB/HSL/HSV); CMYK último P270.2 para fechar 8/8.
Cluster Gradient L1+stdlib+L3 feature-complete excepto CMYK
PDF emit.

### Descoberta arquitectural P270.1.A

**P270 já passou L3 multi-space implicitamente** — helpers L3
`oklab_sample_stops_*` chamam `<variant>.sample(t)` que despacha via
P270 `interpolate_in_space` per `self.space`. **P270.1 é
maioritariamente cosmético** (rename + docs + tests).

Esta descoberta é registada em §A.2 do diagnóstico P270.1.A;
implicação: scope P270.1 reduzido vs spec original (que assumia
refactor estrutural).

### Defaults Oklab preservam bytes P263/P265/P268.2 — zero regressão

Arm Oklab em `interpolate_in_space` chama `interpolate_oklab` P262
literal → sample stops idênticos a P263/P265/P268.2 actuais. **2500
tests baseline preservados literal**.

### Cap LOC hard vs soft (sub-padrão N=1 inaugural)

P270.1 inaugura distinção formal:
- **Cap hard**: gate; estouro dispara §política condição absoluta.
- **Cap soft**: informativo; estouro registado no relatório.

Reais P270.1:
- L3 production ~45 LOC (cap hard 400 — folga 89%; cap soft 250 —
  folga 82%).
- Tests 33 (cap hard 50 — folga 34%; cap soft 35 — folga 6%).

§política condições 2 + 3 não accionadas.

---

## §2 — Diff L3 antes/depois

### §2.1 — Renomeação helpers

| Antes | Depois |
|---|---|
| `fn oklab_sample_stops(linear, n)` | `fn multispace_sample_stops(linear, n)` |
| `fn oklab_sample_stops_radial(radial, n)` | `fn multispace_sample_stops_radial(radial, n)` |
| `fn oklab_sample_stops_conic(conic, n)` | `fn multispace_sample_stops_conic(conic, n)` |

**Body literal preservado** (zero operacional change).

### §2.2 — Docstrings ampliadas

Cada helper agora documenta:
- Comportamento multi-space via P270 dispatcher (auto-discovery).
- 7 spaces materializados nominados.
- CMYK fallback temporário (pipeline natural CMYK→sRGB até P270.2).
- Default Oklab preserva bytes pré-P270.1 bit-exact.

### §2.3 — Callsites production

3 callsites em `emit_gradient_objects` actualizados (rename
mecânico via sed):

```rust
// Linha 1210 (Linear branch)
let stops = multispace_sample_stops(linear, 16);  // era oklab_sample_stops

// Linha 1227 (Radial branch)
let stops = multispace_sample_stops_radial(radial, 16);  // era oklab_sample_stops_radial

// Linha 1251 (Conic branch)
let _ = multispace_sample_stops_conic(conic, 16);  // era oklab_sample_stops_conic
```

### §2.4 — Test refs internas

~9 referências em P263/P265/P268 unit tests internas renomeadas
(rename mecânico). 3 test function names cosméticos também
renomeados (`p263_oklab_sample_stops_*` →
`p263_multispace_sample_stops_*` etc.) — não afecta assertion
behaviour; só identidade cosmética.

### §2.5 — Pipeline multi-space invocação

```rust
// P270.1 (idêntico estrutura P263 + auto-multi-space via P270)
fn multispace_sample_stops(linear: &Linear, n_samples: usize)
    -> Vec<(f32, f32, f32)>
{
    let n = n_samples.max(2);
    (0..n).map(|i| {
        let t = i as f32 / (n - 1) as f32;
        let c = linear.sample(t);  // <-- P270 dispatcher per linear.space
        let (r, g, b, _) = c.to_rgba_f32();
        (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
    }).collect()
}
```

Body literal preserved; só nome muda + docs ampliadas.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P270.1 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=7 → N=8 cumulativo** | + P270.1 anotada ADR-0091 §"Decisão L3 materializada P270.1" |
| Reutilização literal helpers cross-passos | **N=6 → N=7 cumulativo** | + P270.1 (dispatcher P270 + helpers L3 P263/P265/P268.2 templates) |
| Diagnóstico imutável (nono consumo) | **N=13 → N=14 cumulativo** | + P270.1 (vanilla `sample_stops` pipeline emit + `mix_iter`) |
| Auditoria condicional (ADR-0084) | **N=12 → N=13 cumulativo** | + P270.1 |
| Auto-aplicação ADR-0065 inline | **N=11 → N=12 cumulativo** | + P270.1 |
| **Cap LOC hard vs soft explícito** | **N=1 inaugural** | P270.1 inaugura (lição P270 estouro aplicada) |
| Anotação cumulativa cross-ADR | **N=1 → N=2 cumulativo** | P270 inaugurou (6 ADRs); P270.1 estende (6 ADRs: ADR-0091/0087/0088/0089/0090/0054) |

---

## §4 — Métricas finais

| Métrica | Pré-P270.1 | Pós-P270.1 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2500 | **2533** | +33 |
| Tests P270.1 novos | — | 33 | 21 unit pré-amostragem + 4 dispatcher + 5 E2E + 3 snapshot |
| Tests P262-P270 originais (verdes) | 2500 | 2500 | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Hashes propagados | — | 1 (L0 gradient.md) | +1 |
| ADRs totais | 78 | 78 | **0 (sem ADR nova)** |
| LOC L3 adicionado | — | ~45 | cap hard 400 (folga 89%); cap soft 250 (folga 82%) |
| LOC L3 alterado (rename + docs) | — | 15 ocorrências + 3 docstrings | preservado body literal |

### §política condições verificadas

- 1 (Dispatcher L1 P270 não é chamado directo de L3 — passa via
  `<variant>.sample(t)`; arquitecturalmente mais limpo per
  ADR-0029). ✓
- 2 (Cap L3 hard 400 — real ~45; folga 89%). ✓
- 3 (Cap testes hard 50 — real 33; folga 34%). ✓
- 4 (Defaults Oklab preservam bytes — verificado 2500 baseline). ✓
- 5 (Snapshot bytes reproduzíveis — 3 tests determinísticos
  passam). ✓
- 6 (Lint zero pós `--fix-hashes`). ✓
- 7 (Zero regressão P262-P270 — 2500 baseline preserved). ✓
- 8 (CMYK fallback decisão §A.9 — pipeline natural CMYK→sRGB;
  sem ambiguidade). ✓
- 9 (Cluster Gradient marco preservado — test
  `p270_1_export_pdf_cluster_3_variants_multispace_coexistem`
  passa). ✓
- 10 (Rename helpers L3 — sem call sites externos identificados
  fora export.rs; refactor self-contained). ✓
- 11 (Hue-wrap N=16 banding moderado — preservado paridade
  vanilla; refino candidato P-Gradient-Adaptive-Multispace futuro
  fora scope P270.1). ✓

**11 condições §política verificadas** — todas satisfeitas.

---

## §5 — Verificação regressão zero P262-P270

**2500 tests preservados literal** (baseline P262-P270):

- typst-core: 2162 preserved.
- typst-infra: 291 preserved (P270.1 adiciona +33 → 324 total).
- Outros: preserved.

Mecânica de update: rename mecânico via sed (15 ocorrências em
export.rs). Body de helpers preservado literal — defaults Oklab
produzem bytes idênticos via arm Oklab dispatcher → `interpolate_oklab`
P262.

§política condições 4 + 7 + 9 satisfeitas absolutas.

---

## §6 — Anotações cumulativas materializadas

### §6.1 — ADR-0091 anotação cumulativa P270.1

`00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md`
estendida com §"Anotação cumulativa P270.1 — L3 emit multi-space
materializado (7 spaces)".

**Conteúdo essencial**: descoberta arquitectural P270.1.A (L3 já
multi-space implícito); rename helpers + docs ampliadas; 7 spaces
materializados; CMYK preservado scope-out até P270.2; subpadrões
cumulativos; status `IMPLEMENTADO` preservado.

### §6.2 — ADR-0087/0088/0089/0090 anotações cumulativas P270.1

4 anotações cumulativas curtas (variant strategies) — cada uma
documenta rename helper L3 correspondente; body literal preservado.

Sub-padrão "Anotação cumulativa cross-ADR" N=1 → N=2 cumulativo
(P270 inaugurou; P270.1 estende).

### §6.3 — ADR-0054 anotação cumulativa P270.1

Cluster Gradient L3 emit 7/8 spaces materializado; perfil graded
DEBT-1 preservado.

### §6.4 — L0 `entities/gradient.md` anotação P270.1

Adicionada após cross-references P270 existentes. Documenta rename
helpers L3 + 7 spaces materializados + CMYK até P270.2.

Hash propagado via `crystalline-lint --fix-hashes`.

---

## §7 — README ADRs distribuição

### Total ADRs

**78 preservado** (P270.1 sem ADR nova; só anotação cumulativa
ADR-0091).

### Distribuição

| Status | Pré-P270.1 | Pós-P270.1 | Delta |
|---|---|---|---|
| `PROPOSTO` | 11 | 11 | 0 |
| `IDEIA` | 2 | 2 | 0 |
| `EM VIGOR` | 33 | 33 | 0 |
| `IMPLEMENTADO` | 30 | 30 | 0 |
| `REVOGADO` | 2 | 2 | 0 |
| `ADIADO` | 1 | 1 | 0 |
| **Total** | **78** | **78** | **0** |

### Passos-chave

Nova entrada `- **Passo 270.1**` adicionada após Passo 270.
~85 linhas (refino L3 da série P270 com 4 sub-padrões cumulativos +
1 inaugural).

### Cobertura Visualize agregada

~79-80% (P270) → **~81-83% pós-P270.1** (+2-3pp via L3 emit 7
spaces materializado; cluster Gradient feature-complete excepto
CMYK).

---

## §8 — Pendências preservadas pós-P270.1

- **P270.2** (S+ futuro) — L3 CMYK directo `/DeviceCMYK` único
  caso especial. Revoga ADR-0083 §"DeviceCMYK PDF" final. Pode
  resolver bug vanilla #4422.
- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).
- **ADR-0055bis variant-aware fonts** (M; refino Text).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** + outros Visualize.
- **Tiling activação** (Paint::Tiling).
- **P-Gradient-Adaptive-Multispace** (candidato refino futuro;
  N adaptive para HSL/Oklch/HSV banding em hue diff alto;
  paridade P268.2 adaptive N).

Decisão humana fica em aberto literal pós-P270.1.

---

## §9 — Critério aceitação checklist

- [x] **Fase A diagnóstico** `diagnostico-l3-multispace-passo-270-1.md`
      criado (§A.1-§A.15; imutável per ADR-0085).
- [x] **Descoberta P270.1.A §A.2** documentada — P270 já passou L3
      multi-space implicitamente.
- [x] **ADR-0091 anotada P270.1** §"Decisão L3 materializada".
- [x] **ADR-0087/0088/0089/0090 anotadas P270.1** (4 anotações
      curtas variant strategies).
- [x] **ADR-0054 anotada P270.1** cluster L3 7/8 spaces.
- [x] **L0 `entities/gradient.md` anotada P270.1** após secção P270;
      hash propagado.
- [x] **ADR-0083 preservada** (CMYK ainda scope-out; P270.2).
- [x] **33 tests-primeiro** adicionados antes do código L3.
- [x] **L3 rename helpers** `oklab_sample_stops_*` →
      `multispace_sample_stops_*` (sed global 15 ocorrências).
- [x] **Docstrings ampliadas** 3 helpers + body literal preserved.
- [x] **README ADRs** linha tabela ADR-0091 estendida; passo 270.1
      §"passos-chave"; total 78 preservado.
- [x] **Tests workspace** 2500 → 2533 (+33; **zero regressões**).
- [x] **Lint zero violations** pós `--fix-hashes`.
- [x] **Build cargo** exit 0.
- [x] **Snapshot bytes reproduzíveis** (3 tests determinísticos).

**11 condições §política verificadas — todas satisfeitas**.

---

## §10 — Referências

### Cross-passos

- **P270** — Gradient ColorSpace runtime L1+stdlib (ADR-0091;
  precedente directo materializado em L3 por este passo).
- **P263** — PDF Linear pipeline Oklab (template estrutural;
  renomeado P270.1).
- **P265** — PDF Radial pipeline Oklab (template; renomeado).
- **P268** — PDF Conic Type 4 Gouraud (template; renomeado).
- **P268.2** — Adaptive N hybrid (preservado; arm Oklab continua
  adaptive).
- **P269** — Radial focal_* (preservado; campo space aditivo
  cross-variant).

### ADRs

- **ADR-0091** — Gradient ColorSpace runtime + CMYK strategy
  (§"Decisão L3 materializada P270.1" anotada cumulativa).
- **ADR-0087/0088/0089/0090** — Variant strategies (anotadas
  cumulativa P270.1).
- **ADR-0054** — Perfil graded (anotada cumulativa P270.1).
- **ADR-0083** — Color paridade (§DeviceCMYK preservada P270.1;
  P270.2 fecha).
- **ADR-0085** — Diagnóstico imutável (nono consumo directo de
  fonte vanilla).

### Documentos cristalinos editados

- `03_infra/src/export.rs` (~45 LOC L3: rename 15 ocorrências +
  3 docstrings ampliadas + 33 tests P270.1 novos).
- `00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md` (anotação P270.1).
- `00_nucleo/adr/typst-adr-0087-gradient-linear-only.md` (anotação P270.1).
- `00_nucleo/adr/typst-adr-0088-gradient-radial-only.md` (anotação P270.1).
- `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md` (anotação P270.1).
- `00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md` (anotação P270.1).
- `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` (anotação P270.1).
- `00_nucleo/prompts/entities/gradient.md` (anotação P270.1 após
  secção P270; hash propagado).
- `00_nucleo/adr/README.md` (tabela ADR-0091 estendida + passos-chave
  P270.1).

### Documentos criados

- `00_nucleo/diagnosticos/diagnostico-l3-multispace-passo-270-1.md`
  (imutável; nono consumo directo de fonte vanilla).
- `00_nucleo/materialization/typst-passo-270-1-relatorio.md` (este
  relatório).

### Vanilla literal (verificável)

- `lab/typst-original/.../visualize/gradient.rs:1346` —
  `sample_stops` vanilla pipeline emit multi-space via `mix_iter`.
- `lab/typst-original/.../visualize/color.rs:1095-1176` —
  `mix_iter` multi-space.
