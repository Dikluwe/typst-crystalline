# ⚖️ ADR-0094: Meta-operacional de specs (Cap LOC hard/soft + Reutilização helpers + Industry research proactiva)

**Status**: `EM VIGOR` (criada directamente; paridade pattern P260
ADR-0084/0085 e P268.1 ADR-0090)
**Data**: 2026-05-17
**Autor**: Humano + IA
**Validado**: Passo 271 — formaliza 3 sub-padrões empíricos N≥4
cumulativos sobre como specs são escritas e executadas.
**Diagnóstico prévio**: inline em P271 §1.C + §1.D + §1.E (paridade
auto-aplicação ADR-0065 inline — passo administrativo XS).
**Passo origem**: P271 (passo administrativo XS meta-formalização)
**Cluster**: Metodologia / Specs / Operacional
**Tipo**: meta-ADR formalizando 3 sub-padrões empíricos N≥4 cumulativos

---

## Contexto

Cluster Gradient série P270 (5 sub-passos: P270/P270.1/P270.2/
P270.3/P270.4) produziu 3 sub-padrões sobre **como specs são
escritas e executadas** que atingiram limiar formalização clara:

1. **"Cap LOC hard vs soft explícito"** N=4 (P270.1/P270.2/P270.3/
   P270.4).
2. **"Reutilização literal helpers cross-passos"** N=10 consolidação
   clara (P265 a P270.4 cumulativo).
3. **"Fase A com industry research proactiva"** N=4 (P270/P270.2/
   P270.3/P270.4).

Empiria estabelecida; meta-ADR formaliza para specs futuras.

---

## Decisão — Pattern 1: Cap LOC hard vs soft explícito

**Motivação**: lição operacional pós-P270 — caps LOC L1/stdlib
estouraram ~6%/~60% mas §política condição 4 não disparou (cap "ou"
ambíguo); estouro silencioso. P270.1 inaugurou distinção explícita.

**Como aplicar em spec**:

- **Cap hard** (gate; estouro dispara §política condição absoluta):
  - Valor único por camada (L1/stdlib/L3/testes).
  - Estouro força parar e perguntar antes de continuar.
- **Cap soft** (informativo; estouro regista no relatório):
  - Valor menor que hard (~50-70% do hard).
  - Estouro continua mas é registado §"Cap soft estourou" no
    relatório.
- **Magnitude global**: estouro reformula spec inteira (raro;
  cenário B2 sub-passos).

**Formato em spec**:

```text
| Camada | Cap hard | Cap soft | Estimativa empírica       |
|--------|----------|----------|---------------------------|
| L3     | 250 LOC  | 150 LOC  | ~80-120 (§A.X diagnóstico)|
| Testes | 35       | 25       | ~20-25                    |
```

**§política condição relacionada (exemplo)**:

```text
N. Cap LOC L3 hard (250) ameaça ser ultrapassado — refactor maior
   que estimativa §A.X. Confirmar antes de continuar.
```

---

## Decisão — Pattern 2: Reutilização literal helpers cross-passos

**Motivação**: cap LOC contenção via reutilização; consistência
cross-passos; redução risco bug duplicação.

**Como aplicar em spec**:

1. §0 princípio explícito identifica helpers reutilizáveis com
   referência ao passo origem.
2. §1 Fase A confirma disponibilidade via `rg` literal.
3. §política condição cobre gap (helpers indisponíveis ou não
   reutilizáveis literal).
4. §3 estrutura código mostra reutilização explícita (e.g.
   `interpolate_oklab` literal P262).

**Quando NÃO aplicar**:

- Helper precisa modificação significativa: criar variant novo
  paridade estrutural (e.g. `multispace_sample_stops_conic_cmyk`
  paridade `multispace_sample_stops_conic` RGB).
- Helper L1 não chamável de L3 por ADR-0029: extrair para crate
  compartilhada ou promover a `pub` controlado (e.g.
  `color_to_oklab_with_alpha` P268.2).

---

## Decisão — Pattern 3: Fase A com industry research proactiva

**Motivação**: lição P268.1 + P268.1-correção — pesquisa industry
reactiva pós-divergência custou re-trabalho (Type 1 vs Type 6 vs
Type 4 confusion). P270 inaugurou pesquisa preventiva.

**Quando aplicar**:

- Decisão arquitectural não-trivial com múltiplas opções viáveis.
- Vanilla actual usa abstracção opaca (e.g. krilla SweepGradient).
- Bug conhecido em vanilla (e.g. #4422) sem causa raiz documentada.
- Standard PDF/W3C/ISO permite múltiplas estratégias.

**Como aplicar em spec**:

1. **ANTES** de escrever Fase A filesystem:
   - `web_search` queries específicas (3-9 fontes).
   - Consolidar achados literais com citações.
2. Incluir achados em §0 princípio explícito.
3. Fase A complementa com filesystem verification.
4. Achados literais embebidos em ADRs novas / anotações §"Pesquisa
   empírica industry" / "Industry research consolidada".

**Quando NÃO aplicar**:

- Refino paramétrico trivial (constante recalibração; e.g. P270.2
  fallback CMYK→sRGB natural).
- Refactor cirúrgico baseado em precedent cristalino directo.

---

## Histórico empírico

### Sub-padrão "Cap LOC hard vs soft explícito" (N=4)

| N | Passo | Cap hard / Cap soft (L3) | Real | Estado |
|---|---|---|---|---|
| 1 | P270.1 | 400 / 250 | ~45 | inaugural; folga grande |
| 2 | P270.2 | 250 / 150 | ~138 | soft estourou ~8 LOC (~5%); hard respected |
| 3 | P270.3 | 350 / 250 | ~250 | soft no limite exacto; hard respected |
| 4 | P270.4 | 200 / 150 | ~120 | ambos respected; folga 40%/20% |

Consolidação clara N=4. Pattern operacional estabelecido. Sub-padrão
"estouro soft regista relatório sem disparar §política" verificado
empíricamente.

### Sub-padrão "Reutilização literal helpers cross-passos" (N=10)

| N | Passo | Helpers reutilizados |
|---|---|---|
| 1 | P265 | Helpers P263 Linear PDF emit |
| 2 | P267 | Helpers Oklab P262 em L1 Conic sample |
| 3 | P268 | Helpers Oklab N=16 P263+P265 em emit_conic_gouraud_stream |
| 4 | P268.2 | oklab_delta_e + interpolate_oklab P262 em compute_adaptive_n_conic |
| 5 | P269 | Helpers Oklab P262/P265 em Radial focal sample |
| 6 | P270 | Helpers Color P257 conversões cross-space |
| 7 | P270.1 | interpolate_in_space L1 P270 + 3 helpers L3 P263/P265/P268 templates |
| 8 | P270.2 | interpolate_in_space arm Cmyk P270 + 3 helpers L3 P270.1 templates |
| 9 | P270.3 | dispatcher P270 + helpers L3 P270.1 templates Coons emit |
| 10 | P270.4 | 3 helpers Coons P270.3 + helpers P270.2 CMYK + P257 to_cmyk_f32 + P270 dispatcher arm Cmyk |

Consolidação clara N=10. Mecanismo principal contenção LOC growth ao
longo cluster Gradient.

### Sub-padrão "Fase A com industry research proactiva" (N=4)

| N | Passo | Pesquisa industry |
|---|---|---|
| 1 | P270 | Vanilla docs + blog 2023 + issue #4422 (causa raiz CMYK gradient bug pre-spec) |
| 2 | P270.2 | Vanilla CMYK emit + pdfkit #532 + ISO 32000-1 §7.5.7 + PDF DeviceCMYK structure |
| 3 | P270.3 | Cairo Type 6/7 + Inkscape + W3C Workshop 2021 + pdf.js #6283 + PDFBOX-2100 + matplotlib #18034 + Stanislaw Adaszewski + Typst blog 2023 + ISO 32000-1 §7.5.7.4 (9 fontes) |
| 4 | P270.4 | Bug #4422 vanilla GitHub validation + reader compatibility Type 6 + DeviceCMYK |

Consolidação clara N=4. Pattern operacional estabelecido. Distingue-se
de Fase A standard (filesystem read) — adiciona pesquisa web preventiva.

---

## Consequências

- ✅ Specs futuras seguem pattern consolidado (caps explícitos;
  reutilização proactiva; pesquisa preventiva).
- ✅ Cap LOC contenção verificável via §"estouro soft ~X% registado".
- ✅ Decisões arquitecturais informadas antes de spec (não pós).
- ⚠️ Adiciona ~30-60 min preventivos a specs com decisão arquitectural
  não-trivial.
- ⚠️ Tabela hard/soft adiciona verbosidade a specs simples (cap soft
  pode ser omitido se hard sozinho cobre).

---

## Alternativas consideradas

- **Cap único (sem hard/soft)**: rejeitada — lição P270 estouro
  silencioso.
- **Cap hard apenas (sem soft)**: rejeitada — soft permite registo
  empírico sem disparar §política (utilizado P270.1 inaugural).
- **Pesquisa industry reactiva sempre**: rejeitada — re-trabalho
  P268.1 + P268.1-correção justifica preventiva para casos
  arquiteturais.

---

## Critério revisão

Esta ADR pode ser revisitada se:

- Cap soft frequentemente estourado sem consequência: pode ser
  removido.
- Reutilização helpers tornar-se constrangimento de design (Liskov
  violation; helpers acoplados demais): refactor amplo.
- Pesquisa industry preventiva tornar-se pro-forma (sempre confirma
  Fase A standard): omitir.

---

## Subpadrões aplicados

- **Passo administrativo XS criar/promover ADR**: N=5 → **N=6
  cumulativo** (P156K/P160A/P229/P254/P268.1/**P271**).
- **Auto-aplicação ADR-0065 inline**: N=15 → N=16 cumulativo
  (diagnóstico empírico embebido §"Histórico empírico" desta ADR).
- **Meta-formalização sub-padrões empíricos N≥4**: N=2 → **N=3
  cumulativo** (P260 ADR-0084/0085 + P268.1 ADR-0090 + **P271
  ADR-0093/0094**).

---

## Referências

- ADR-0084 + ADR-0085 (formalização P260; precedente meta-ADR EM
  VIGOR criada directa).
- ADR-0090 (formalização P268.1; precedente meta-ADR EM VIGOR criada
  directa).
- ADR-0093 (companheira P271; meta-metodologia evolução ADRs).
- ADR-0029 (pureza física L1; relacionado Pattern 2 helpers
  cross-camada).
- ADR-0083, ADR-0090, ADR-0091, ADR-0092 (ADRs instanciadoras dos
  sub-padrões; anotadas cumulativa P271).
- Passos cumulativos §"Histórico empírico" desta ADR (P265, P267,
  P268, P268.2, P269, P270, P270.1, P270.2, P270.3, P270.4).
