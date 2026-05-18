# Diagnóstico Fase A P273.17.A — Reflexão metodológica formal cluster Gradient + 3 ADRs meta novas

**Data**: 2026-05-18.
**Passo**: typst-passo-273.17.A.
**Magnitude**: S documental (~30 min).
**Cluster**: Visualize / Gradient (passo administrativo de fecho).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085 — passo administrativo S+ sem materialização código.
**Vigésimo oitavo consumo directo de fonte**.

---

## §A.1 — Inventário ADRs existentes

### Verificação literal

`ls 00_nucleo/adr/typst-adr-*.md | wc -l` retorna **93 ficheiros**.

README declara em §"Total pós-P273.6: 81 ADRs preservado" (e
preserved até P274 + P273.5-P273.16). Diferença 12 entre 93 files
e 81 vigentes corresponde a:
- ADRs revogadas com ficheiro preserved (ADR-0007, ADR-0028, ADR-0090 etc.).
- ADRs com sufixo -R1 (ex.: ADR-0026-R1).

**Total pós-P273.16: 81 ADRs vigentes preserved** (confirmado).

### Numeração próxima disponível

`ls 00_nucleo/adr/typst-adr-*.md | tail`:
- typst-adr-0094-meta-operacional-specs.md (último).
- Próximos disponíveis: **0095, 0096, 0097** ✓.

**Verificação empírica**: zero ADRs criados entre P271 (ADR-0094) e
P273.17. P273.5-P273.16 todos anotaram ADR-0091 cumulativa sem
criar ADRs novas (sub-padrão "Anotação cumulativa em vez de ADR
nova" — décima sexta anotação consecutiva pós-P273.16).

---

## §A.2 — Verificação empírica sub-padrões N≥3-4

### Sub-padrão 1: "Dedup `Arc::as_ptr` resources" (ADR-0095) — N=3

**Aplicações cumulativas literais**:

| N | Passo | Mecanismo | Chave |
|---|---|---|---|
| 1 | **P73** | `image_resources` em export.rs PDF emit | `HashMap<usize, usize>` com `Arc::as_ptr(data) as usize` |
| 2 | **P263** | `pattern_resources` em export.rs gradient emit | `HashMap<usize, usize>` com `Arc::as_ptr(linear/radial/conic) as usize` |
| 3 | **P273.12** | `pattern_resources` bbox-aware via `DedupKey` | `HashMap<DedupKey, usize>` com `DedupKey { arc_ptr, bbox: Option<RectKey> }` |

**N=3 cumulativo confirmado**. Limiar formalização N=3-4 atingido.

### Sub-padrão 2: "Pattern DEBT-37 `cell_origin_*` replicado" (ADR-0096) — N=4

**Aplicações cumulativas literais**:

| N | Passo | Mecanismo | Status consumer |
|---|---|---|---|
| 1 | **P84.6** | `cell_origin_x/y/w: Option<f64>` Layouter | DEBT-37 registado consumer-pending |
| 2 | **P273.5** | `apply_parent_transform` + `parent_bbox: Option<Rect>` Layouter | `#[allow(dead_code)]` consumer-pending |
| 3 | **P273.6** | Block save/restore real `parent_bbox` + cascade ~86 sites | DEBT-37 fechado (consumer activo) |
| 4 | **P273.9** | Grid cell paralelo a `cell_origin_*` (consumer real) | Pattern reused via parallel save/restore |

**N=4 cumulativo confirmado**. Limiar formalização N=3-4 crossado com folga.

### Sub-padrão 3: "Scope-out reconfirmado por Fase A" (ADR-0097) — N=3

**Aplicações cumulativas literais**:

| N | Passo | Razão NO-GO | Outcome |
|---|---|---|---|
| 1 | **P273.14** | Constraints **externas** (CMYK-ICC: profile licensing + crate externa + invariante L0 export.md) | SCOPE-OUT-RECONFIRMED; 3 pré-requisitos identificados |
| 2 | **P273.15** | Constraints **internas** (Bbox medido: custo perf O(N²) + ausência demanda) | SCOPE-OUT-RECONFIRMED; 2 pré-requisitos identificados |
| 3 | **P273.16** | Bloqueador **estrutural aceito** (Bbox.y topo-exacto: P156H + ADR-0078 §sub-fase b; descoberta empírica DEBT-56 fechado P221 actualiza premissa spec) | SCOPE-OUT-RECONFIRMED; 3 pré-requisitos identificados |

**N=3 cumulativo confirmado** com 3 razões NO-GO distintas e
legítimas. Limiar formalização N=3-4 atingido.

---

## §A.3 — Estrutura documento reflexão

Fixada per spec §A.3:

1. **§1 Trajectória factual P273.5-P273.16** — sequência
   cronológica com magnitudes, decisões, outcomes.
2. **§2 Sub-padrões emergentes** inaugurados/consolidados (N≥1).
3. **§3 Limiares formalização atingidos** — 3 sub-padrões → 3 ADRs.
4. **§4 Descobertas metodológicas** — caps soft sub-estimados;
   Fase A factual prevalece; cleanups XS; bugs latents.
5. **§5 Pendências residuais** — 3 scope-outs + 2 candidatos XS.
6. **§6 Trade-offs aceitos** — `/DeviceCMYK`; 3γ.2.γ; baseline-y.
7. **§7 Anti-padrões evitados** — over-formalização; scope creep.
8. **§8 Reflexão final** — cluster Gradient como caso de estudo.

---

## §A.4 — Decisão 1 fixada: localização documento reflexão

**Fixada**: **1β — `00_nucleo/diagnosticos/typst-cluster-gradient-reflexao.md`**.

Razões:
1. Dir `00_nucleo/diagnosticos/` já contém outputs documentais
   não-código (incluindo `trabalho-previo-externo.md` dos 3 NO-GO).
2. Sem criar dir novo `meta/` que precisaria justificação.
3. Naming `typst-cluster-gradient-reflexao.md` consistente com
   convenção `typst-passo-NNN-*.md`.

---

## §A.5 — Decisão 2 fixada: estado inicial ADRs

**Fixada**: **2α — todas EM VIGOR directo**.

Razões:
1. Sub-padrões já têm N≥3 evidência empírica concreta — não é
   proposta especulativa.
2. Paridade com ADR-0093/0094 P271 criados directamente EM VIGOR.
3. ADRs meta são documentação retrospectiva por natureza.

---

## §A.6 — Decisão 3 fixada: numeração cronológica por inauguração

**Fixada**: **cronológica por inauguração do sub-padrão**:

| ADR | Sub-padrão | Inauguração |
|---|---|---|
| **0095** | Dedup `Arc::as_ptr` resources | **P73** (image_resources) — mais antigo |
| **0096** | Pattern DEBT-37 `cell_origin_*` replicado | **P84.6** (Grid cell) |
| **0097** | Scope-out reconfirmado por Fase A | **P273.14** (CMYK-ICC) — mais recente |

Razão: ordem cronológica reflecte profundidade histórica do padrão.

---

## §A.7 — Análise de risco

| Risco | Estado |
|---|---|
| ADRs duplicam conteúdo | ✅ Mitigado — 3 ADRs naturezas distintas (mecânica L3 / técnica L1 / decisão metodológica) |
| Documento reflexão duplica ADRs | ✅ Mitigado — ADRs focam mecânica formal; reflexão foca trajectória |
| Over-formalização | ✅ Mitigado — 7 sub-padrões N=1-2 NÃO formalizados (anti-padrão explícito) |
| Sub-padrões mal calibrados | ✅ Mitigado — §A.2 verificação literal de cada inauguração |
| Numeração colide | ✅ Mitigado — §A.1 confirma 0095/96/97 livres |

---

## §A.8 — Critério de aceitação Fase A

- ✓ §A.1 confirma 81 ADRs vigentes; 0095/96/97 disponíveis.
- ✓ §A.2 verificação empírica N≥3 para cada sub-padrão (3+4+3).
- ✓ §A.3 estrutura documento reflexão fixada (8 secções).
- ✓ §A.4 Decisão 1 fixada: 1β `00_nucleo/diagnosticos/`.
- ✓ §A.5 Decisão 2 fixada: 2α EM VIGOR directo.
- ✓ §A.6 Decisão 3 fixada: cronológica (P73→P84.6→P273.14).

**Fase A produzida — critério §A.8 cumprido absoluto.**

---

## §A.9 — Plano de implementação

### Cap documental

- **Hard cap**: ~1500 linhas markdown (3 ADRs × 300-400 + reflexão
  400-500).
- **Soft cap**: ~1200 linhas.
- **L1/L3**: 0 LOC (passo administrativo).
- **Tests**: 0 novos.

### Ordem literal

1. Fase A (este documento).
2. ADR-0091 décima sétima anotação cumulativa.
3. Criar ADR-0095 EM VIGOR.
4. Criar ADR-0096 EM VIGOR.
5. Criar ADR-0097 EM VIGOR.
6. Criar documento reflexão.
7. Actualizar README ADRs (81 → 84; EM VIGOR cresce 3).
8. Relatório final.

---

*Diagnóstico imutável produzido em 2026-05-18. Vigésimo oitavo
consumo. Decisões 1β + 2α + 3 cronológica fixadas; 3 sub-padrões
verificados empíricamente (N=3+4+3) crossing limiar formalização
N=3-4; pronto para Fase C documental (~1500 linhas markdown;
0 código).*
