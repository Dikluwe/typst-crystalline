# Diagnóstico — L3 emit multi-space pipeline (P270.1.A)

**Status**: imutável após criação (per ADR-0085).
**Data**: 2026-05-17.
**Passo**: P270.1 (refino L3 emit multi-space).
**Nono consumo directo de fonte** vanilla (P262/P264/P267/P268/P268.1/P268.2/P269/P270 + **P270.1 vanilla pipeline emit**).
**Origem**: spec `00_nucleo/materialization/typst-passo-270.1.md` §1.

---

## §A.1 — Cristalino L3 actual pipeline (3 sítios `*_sample_stops_*`)

`03_infra/src/export.rs`:

```rust
// Linha 458-471
fn oklab_sample_stops(linear: &Linear, n_samples: usize) -> Vec<(f32, f32, f32)> {
    let n = n_samples.max(2);
    (0..n).map(|i| {
        let t = i as f32 / (n - 1) as f32;
        let c = linear.sample(t);          // <-- chave §A.2
        let (r, g, b, _) = c.to_rgba_f32();
        (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
    }).collect()
}

// Linha 512-525 — `oklab_sample_stops_radial` (idêntico estrutura; usa radial.sample(t))
// Linha 530-543 — `oklab_sample_stops_conic` (idêntico; usa conic.sample(t))
```

Todos os 3 helpers usam `<variant>.sample(t)` como ponto de obtenção
de cor para cada amostra t ∈ [0, 1].

---

## §A.2 — Descoberta central: L3 já é multi-space implicitamente via P270

`Linear/Radial/Conic::sample(t)` foi actualizado em P270 para
despachar via `interpolate_in_space(c0, c1, t, self.space)`. Por
consequência, **os helpers L3 `oklab_sample_stops_*` já produzem
amostras no space correcto cross-variant** sem qualquer alteração L3
adicional.

### Verificação empírica

- `linear.space == ColorSpace::Hsl` + `linear.sample(0.5)` →
  `interpolate_in_space(c0, c1, 0.5, Hsl)` → interpolação em HSL.
- O return value é `Color` que é depois convertido para sRGB via
  `to_rgba_f32()` (pipeline L3 idêntico P263).

### Implicação arquitectural

**P270.1 é maioritariamente cosmético**: rename + docs + tests. Não
há refactor estrutural necessário porque a multi-space pipeline está
já operacional via dispatcher P270.

Cenário detectado: **B1 fecho conceptual trivial** confirmado §A.11.
§política condição 1 NÃO accionada — dispatcher P270 acessível
indirectamente via `<variant>.sample(t)`.

---

## §A.3 — Cristalino L1 dispatcher `interpolate_in_space` — acessibilidade

`interpolate_in_space` é função privada de
`01_core/src/entities/gradient.rs` (não `pub`). NÃO é chamável
directamente de L3.

**Mas L3 NÃO precisa chamá-lo directamente** — passa via
`<variant>.sample(t)` que internamente despacha. Isto é
**arquitecturalmente mais limpo**:

- L3 não precisa conhecer ColorSpace dispatch logic.
- L1 mantém encapsulamento do dispatcher.
- ADR-0029 (pureza física L1) preservada — L3 chama métodos
  públicos L1 standard.

§política condição 1 não accionada por isto.

---

## §A.4 — PROPOSTA renomeação helpers L3 (cosmética)

Helpers actuais nomeados `oklab_sample_stops_*` por convenção P263.
Pós-P270, o nome é misleading (não é mais Oklab-only).

**Renomeação proposta P270.1**:
- `oklab_sample_stops` → `multispace_sample_stops`
- `oklab_sample_stops_radial` → `multispace_sample_stops_radial`
- `oklab_sample_stops_conic` → `multispace_sample_stops_conic`

**Mecânica**: assinatura preservada (nenhum novo param — `space` vem
implícito via `<variant>.space`). Documentação ampliada para
explicar comportamento multi-space.

**Call sites production** (3 sítios em `emit_gradient_objects`):
todos chamam helpers pelo nome; rename mecânico (sed) actualiza.

**Tests P263/P265/P268.2 internos** se referem aos helpers pelo nome
original em ~6 sítios. Mecânica de update mecânica.

---

## §A.5 — PROPOSTA pipeline multi-space materializado

```rust
fn multispace_sample_stops(linear: &Linear, n_samples: usize) -> Vec<(f32, f32, f32)> {
    let n = n_samples.max(2);
    (0..n).map(|i| {
        let t = i as f32 / (n - 1) as f32;
        let c = linear.sample(t);  // <-- agora despacha por linear.space (P270)
        let (r, g, b, _) = c.to_rgba_f32();
        (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
    }).collect()
}
```

**Function body literal preservado** (zero mudança operacional);
apenas rename + comentário documentando comportamento multi-space.

Idem `multispace_sample_stops_radial` e `multispace_sample_stops_conic`.

---

## §A.6 — Vanilla pipeline equivalente

Vanilla typst `sample_stops` (`lab/typst-original/.../visualize/gradient.rs:1346`):

```rust
fn sample_stops(stops: &[(Color, Ratio)], mixing_space: ColorSpace, t: f64) -> Color {
    // chama Color::mix_iter(stops_weighted, mixing_space)
}
```

Cristalino paridade: `<variant>.sample(t)` → `interpolate_in_space(c0,
c1, local_t, self.space)`. Estrutura idêntica vanilla.

**Pré-amostragem N=16 em emit_gradient_objects** — vanilla usa
similar (krilla::Stops carrega N stops samples pré-computados).
Cristalino também usa N=16 hardcoded em P263/P265 + N adaptive em
P268.2 Conic. **Preserva paridade actual** P270.1.

---

## §A.7 — Default Oklab preserva bytes P263/P265/P268.2 — verificação

Arm Oklab em `interpolate_in_space` chama `interpolate_oklab` P262
literal. Logo, para `space: ColorSpace::Oklab` (default):

- `linear.sample(t)` para `Linear { space: Oklab, .. }` → bytes
  idênticos a `interpolate_oklab(c0, c1, local_t)` directo.
- `radial.sample(t)` idem.
- `conic.sample(t)` idem.

L3 helpers chamam `.sample(t)` e convertem via `to_rgba_f32()`:
mesma sequência operacional P263/P265/P268.2. **Bytes idênticos**.

**Verificação empírica**: 2500 tests baseline P262-P270 já preservados
literal pós-P270 (confirmado via cargo test workspace exit 0).

§política condição 4 satisfeita.

---

## §A.8 — Hue-wrap pré-amostragem N=16 — análise banding visual

Para HSL/Oklch/HSV com hue diff alto (e.g., red↔blue HSL h=0↔240):
pré-amostragem N=16 amostra 16 cores intermédias com hue
interpolada via `interpolate_hue_shorter` (P270 literal vanilla
paridade).

**Banding visual potencial?**
- N=16 amostras em hue range 0→240° (caminho shorter): cada step
  ≈ 15°. Hue diff visível ao olho casual ≈ 5-10° (perceptual);
  16° pode produzir banding moderado.
- **Comparação com vanilla**: vanilla também usa N comparável (krilla
  default ~16-32). Cristalino paridade.

**Decisão P270.1**: preservar N=16 paridade actual. Banding moderado
aceitável tradoff M+ vs L+ (adaptive N para 7 spaces fora scope
P270.1). Refino futuro candidato (paridade P268.2 adaptive N).

§política condição 11 não accionada — banding moderado expected;
não é regressão (vanilla similar).

---

## §A.9 — CMYK strategy P270.1 — fallback temporário

`<variant>.sample(t)` com `space: ColorSpace::Cmyk` chama
`interpolate_cmyk(c0, c1, t)` (P270 helper L1). Output: `Color::Cmyk
{ c, m, y, k }`. Depois L3 chama `c.to_rgba_f32()` que converte
CMYK → sRGB (P257):

```rust
Self::Cmyk { c, m, y, k } => {
    let r = (1.0 - c) * (1.0 - k);
    let g = (1.0 - m) * (1.0 - k);
    let b = (1.0 - y) * (1.0 - k);
    (r, g, b, 1.0)
}
```

**Comportamento P270.1**: CMYK gradient PDF emit usa pipeline normal
multi-space (interpolation em CMYK + sRGB conversão final em
to_rgba_f32). PDF emitido tem `/ColorSpace /DeviceRGB` (preservado;
cristalino não emite `/DeviceCMYK` ainda).

**Não há fallback Oklab nem panic** — pipeline natural funciona
correctamente, embora subóptima (gradient CMYK convertido para sRGB
perde gama CMYK).

§política condição 8 satisfeita — sem ambiguidade; comportamento
natural P270.

**P270.2** materializará `/DeviceCMYK` directo para evitar conversão
para sRGB.

---

## §A.10 — Cap LOC estimativa P270.1

| Componente | LOC estimado | Cap hard | Cap soft |
|---|---|---|---|
| Rename 3 helpers + docs ampliadas | ~30 | 400 | 250 |
| 3 callsites production rename (sed) | ~5 | | |
| ~6 tests sites com nome antigo rename | ~10 | | |
| Tests novos: 21 unit + 4 dispatcher + 5 E2E + 3 snapshot | ~250-300 | 50 testes | 35 testes |
| **L3 production total** | **~45** | **400 (folga 89%)** | **250 (folga 82%)** |
| **Tests total** | **~33** | **50 (folga 34%)** | **35 (folga 6%)** |

**Cap hard L3 (400) e cap hard testes (50) NÃO ameaçados**. §política
condições 2 + 3 não accionadas.

**Cap soft testes 35 ligeiramente excedido se passar dos 33 alvo**.
Documentável no relatório (sub-padrão "Cap LOC hard vs soft
explícito" N=1 inaugural).

---

## §A.11 — Cenário detectado: B1 fecho conceptual

**B1 confirmado**: refactor cirúrgico — rename + docs + tests. Sem
mudança estrutural necessária porque P270 já passou L3 multi-space
implicitamente via `<variant>.sample(t)` dispatcher.

§política condição 1 não accionada.

---

## §A.12 — Decisão arquitectural — Op B uniforme materializada

ADR-0091 §"Decisão L3 futura" P270.1 fechada com **Op B uniforme**:
- Pipeline cristalino L3 usa pré-amostragem N=16 → DeviceRGB para
  todos os 7 spaces (Oklab/Oklch/sRGB/Luma/LinearRGB/HSL/HSV).
- CMYK preservado scope-out P270.1; P270.2 materializa
  `/DeviceCMYK` directo.

**Convergência parcial vanilla Family A** (Oklab/Oklch/HSL/HSV) —
ambos pré-amostragem. **Divergência intencional vanilla Family B**
(sRGB/LinearRGB/Luma) — cristalino pré-amostragem unificada vs
vanilla DeviceRGB directo. Justificação: simplifica L3 (1 pipeline);
banding imperceptível em sRGB/LinearRGB/Luma com N=16.

---

## §A.13 — CMYK strategy P270.2 preview — fallback temporário P270.1

P270.1 preserva pipeline natural CMYK (interpolation em CMYK +
sRGB conversão final). Output PDF: `/ColorSpace /DeviceRGB` com
cores convertidas.

**Limitações reconhecidas**:
- Gama CMYK perdida na conversão (impressão profissional precisa
  `/DeviceCMYK` directo).
- Bug vanilla #4422 não resolvido P270.1.

**P270.2** materializa:
- `/ColorSpace /DeviceCMYK` para Conic CMYK gradients.
- Sample stops CMYK preservados (4 componentes; sem conversão sRGB).
- Stream binary Type 4 Gouraud com 4 componentes por vertex (vs 3
  actual sRGB).
- Resolve bug vanilla #4422 com implementação cristalina autónoma.

---

## §A.14 — Sumário decisões diagnóstico

| Item | Decisão |
|---|---|
| §A.2 L3 já multi-space implícito | P270.1 cosmético (rename + docs + tests) |
| §A.3 Dispatcher L1 não chamável L3 | Não importa; `<variant>.sample(t)` já despacha |
| §A.4 Rename `oklab_sample_stops_*` → `multispace_*` | Mecânica trivial |
| §A.5 Pipeline body preservado literal | Zero LOC operacional mudados |
| §A.7 Defaults Oklab preservam bytes | Verificado empíricamente 2500 baseline |
| §A.8 Hue-wrap N=16 banding moderado | Preservado paridade vanilla; refino candidato P-Gradient-Adaptive-Multispace futuro |
| §A.9 CMYK fallback pipeline natural | Sem panic; sub-óptimo até P270.2 |
| §A.10 Cap LOC respeitado | L3 ~45 LOC (cap 400; folga 89%) |
| §A.11 Cenário B1 | Confirmado |
| §A.12 Op B uniforme | Materializada conforme ADR-0091 |
| §A.13 P270.2 preview | Documentado |

**Diagnóstico aprovado para passagem a sub-passo P270.1.B (anotações
cumulativas).**

---

## §A.15 — Referências

- Spec P270.1: `00_nucleo/materialization/typst-passo-270.1.md`.
- Cristalino L3: `03_infra/src/export.rs:458-543` (3 helpers).
- Cristalino L1 P270: `01_core/src/entities/gradient.rs`
  (`interpolate_in_space` dispatcher).
- Cristalino Color P257: `01_core/src/entities/color.rs` (CMYK to
  sRGB conversão).
- Vanilla L3: `lab/typst-original/.../visualize/gradient.rs:1346`
  (`sample_stops` via `mix_iter`).
- ADR-0091 — Gradient ColorSpace runtime + CMYK strategy (§"Decisão
  L3 futura" → materializada P270.1).
- ADR-0083 — Color paridade (§DeviceCMYK preservado P270.1).
- ADR-0085 — Diagnóstico imutável (nono consumo directo de fonte).
- ADR-0087/0088/0089/0090 — Variant strategies (preservadas literal).
- P263 — PDF Linear (template estrutural; renomeado P270.1).
- P265 — PDF Radial (idem).
- P268+P268.2 — PDF Conic + adaptive N (idem).
- P270 — Gradient ColorSpace runtime L1+stdlib (precedente directo).
