# Passo P273.17 — Reflexão metodológica formal cluster Gradient + 3 ADRs meta novas

**Tipo**: passo administrativo S+ — reflexão metodológica formal + formalização de 3 sub-padrões emergentes que atingiram limiar N=3-4.
**Magnitude estimada**: S+ (~3-5 horas documental; 0 LOC código; ~1000-1500 linhas markdown across 4 ficheiros novos).
**Pré-requisitos**: P273.16 fechado (SCOPE-OUT-RECONFIRMED; cluster Gradient feature-complete declarável).
**Cluster**: Visualize / Gradient (passo administrativo de fecho; **não materializa código**).
**Aplica ADRs**: ADR-0091 (anotação cumulativa P273.17 décima sétima consecutiva); ADR-0093 (Pattern 2 anotação cumulativa N=13); ADR-0094 (Pattern 1 não aplicável — sem cap LOC; documental).

---

## §0 — Contexto

Sub-decisão γ completo: documento reflexão + 3 ADRs meta novas. P273.17 fecha o ciclo metodológico do cluster Gradient com:

1. **Documento reflexão `reflexao-cluster-gradient.md`** — síntese da trajectória P273.5-P273.16 como caso de estudo metodológico. Output legível standalone.

2. **ADR-0095** — formaliza sub-padrão "Scope-out reconfirmado por Fase A" (N=3 atingiu limiar; razões NO-GO distintas; trabalho de diagnóstico legítimo per ADR-0054 graded).

3. **ADR-0096** — formaliza sub-padrão "Pattern DEBT-37 `cell_origin_*` replicado" (N=4 atingiu limiar; consumer-pending introduzido P84.6; consumer activado em sucessivos passos).

4. **ADR-0097** — formaliza sub-padrão "Dedup `Arc::as_ptr` resources" (N=3 atingiu limiar; image_resources P73 + pattern_resources P263 + pattern_resources_bbox_aware P273.12).

### Por que 3 ADRs meta + documento (γ completo)

A opção γ completo materializa formalmente as 3 lições metodológicas mais consolidadas do cluster Gradient. Argumentos:

- **Limiares N=3-4 atingidos pelos 3 sub-padrões em passos diferentes** mas dentro da trajectória P273.5-P273.16. Acumulação dentro de cluster único é evidência forte de consolidação metodológica.
- **3 ADRs meta diferentes naturezas**: Scope-out (decisão), Pattern DEBT-37 (técnica), Dedup Arc (mecânica). Não há overlap conceptual — cada ADR captura conceito distinto.
- **Documento reflexão captura a sequência narrativa** que ADRs individuais perdem por estrutura formal. Trajectória, decisões inversas (P273.10 inserido por priorização B), descobertas empíricas (P273.16 DEBT-56 fechado P221) ficam preserved no doc reflexão.

Trade-off vs γ leve (1 ADR + doc): 3 ADRs meta documenta 3 lições em vez de 1; custo ~2x tempo; output 3x mais formalizado.

Trade-off vs δ (anotação ADR-0094): perde formalização individual; sub-padrões ficam como anotações cumulativas dispersas.

### Sub-padrões a NÃO formalizar (preserved como emergentes N=1-2)

- **L3-only parent_bbox** (N=2): bom candidato mas N=2 ainda baixo; aguardar reaplicação fora cluster.
- **Template-passo replicado literal** (N=2): mesma razão.
- **Layout duplo arquitectural aceite** (N=1): inaugural.
- **Extract helper de replicação inline** (N=1): inaugural.
- **Triplicação Group bbox** (N=1): inaugural.
- **Bug arquitectural intencional corrigido** (N=1): inaugural.
- **Bug latent corrigido em scope creep** (N=1 ou 2 dependendo de calibração): contagem ambígua.
- **Sub-passos consecutivos do mesmo cluster** (N=11 intra-cluster): valor empírico baixo; "fizemos muitos passos" é descrição, não padrão accionável.

### Anti-padrão evitado

P273.17 **não tenta formalizar todos os sub-padrões emergentes do cluster** — apenas os 3 com limiar atingido e valor metodológico claro. Resistir à tentação de over-formalizar é parte da disciplina.

---

## §1 — Sub-passo P273.17.A — Fase A diagnóstico

**Magnitude**: S documental (~30-45 min).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-17-diagnostico.md`.

### §A.1 — Inventário ADRs existentes

Listar literal:

- Total ADRs pré-P273.17: **81** (preserved desde P271; confirmado em P273.5-P273.16 relatórios).
- ADRs meta existentes referenciadas (ADR-0091/0093/0094/0085/0029/0054/0019/0034/0064/0065/0078).
- **Próximos números disponíveis**: **0095, 0096, 0097** (assumindo nenhuma criada entre P271 e P273.17 — verificar empíricamente).
- Status distribuição: PROPOSTO 11, EM VIGOR 35, IMPLEMENTADO 35 (totais aproximados — verificar empíricamente em README ADRs).

### §A.2 — Inventário sub-padrões a formalizar

Para cada um dos 3 sub-padrões, listar literal:

#### Sub-padrão 1: "Scope-out reconfirmado por Fase A" (ADR-0095)

- **Inauguração**: P273.14 (CMYK-ICC; constraints externas).
- **Reaplicação 1**: P273.15 (Bbox medido pós-layout; constraints internas).
- **Reaplicação 2**: P273.16 (Bbox.y topo-exacto; bloqueador estrutural aceito; descoberta empírica DEBT-56 fechado P221).
- **N=3 cumulativo** crossing limiar formalização N=3-4.
- **3 razões NO-GO distintas e legítimas**: externa / interna / estrutural aceita.
- **Output mecânico**: Fase A com decisão go/no-go binária + documento `trabalho-previo-externo.md` quando NO-GO.

#### Sub-padrão 2: "Pattern DEBT-37 `cell_origin_*` replicado" (ADR-0096)

- **Inauguração**: P84.6 (`cell_origin_x/y/w` para Grid cell — consumer-pending registado DEBT-37).
- **Reaplicação 1**: P273.5 (`apply_parent_transform` + `parent_bbox` Layouter — consumer-pending).
- **Reaplicação 2**: P273.6 (consumer real Block via cascade ~86 sites — DEBT-37 fechado).
- **Reaplicação 3**: P273.9 (Grid cell paralelo a `cell_origin_*`).
- **N=4 cumulativo** crossing limiar formalização N=3-4 com margem.
- **Output mecânico**: campo `Option<T>` introduzido em estrutura sem consumer activo (`#[allow(dead_code)]`); consumer activado em passo subsequente quando contexto madura.

#### Sub-padrão 3: "Dedup `Arc::as_ptr` resources" (ADR-0097)

- **Inauguração**: P73 (image_resources — `HashMap<usize, usize>` indexado por `Arc::as_ptr`).
- **Reaplicação 1**: P263 (pattern_resources — mesmo mecanismo aplicado a gradient patterns).
- **Reaplicação 2**: P273.12 (pattern_resources bbox-aware — `DedupKey { arc_ptr, Option<RectKey> }` estendendo a chave).
- **N=3 cumulativo** crossing limiar formalização N=3-4.
- **Output mecânico**: `HashMap<chave, idx>` para deduplicação de resources PDF reutilizáveis; chave evolui (ptr puro → tuplo com contexto).

### §A.3 — Inventário estrutura documento reflexão

Secções propostas:

1. **§1 Trajectória factual P273.5-P273.16** — sequência cronológica com magnitudes, decisões, outcomes.
2. **§2 Sub-padrões emergentes inaugurados ou consolidados** — N≥1 documentado.
3. **§3 Limiares formalização atingidos** — 3 sub-padrões formalizados via ADRs novas.
4. **§4 Descobertas metodológicas** — caps soft sub-estimados, Fase A factual prevalece sobre premissa, cleanups XS, bugs latents.
5. **§5 Pendências residuais** — scope-outs reconfirmados, candidatos XS NÃO reservados.
6. **§6 Trade-offs aceitos** — `/DeviceCMYK` vs ICC; 3γ.2.γ vs medição; baseline-y vs topo-exacto.
7. **§7 Anti-padrões evitados** — over-formalização, scope creep cego.
8. **§8 Reflexão final** — cluster Gradient como caso de estudo metodológico.

### §A.4 — Decisão 1 — Localização do documento reflexão

Opções:

- **1α — `00_nucleo/meta/reflexao-cluster-gradient.md`** (novo dir `meta/`).
- **1β — `00_nucleo/diagnosticos/typst-cluster-gradient-reflexao.md`** (dir existente para diagnósticos).
- **1γ — `00_nucleo/adr/typst-cluster-gradient-reflexao.md`** (alongside ADRs).

Recomendação spec: **1β** (dir existente; convenção naming `typst-` prefixo). Razões:
1. Dir `00_nucleo/diagnosticos/` já contém outputs documentais não-código.
2. Sem criar dir novo (`meta/`) que precisaria justificação.
3. Naming `typst-cluster-gradient-reflexao.md` segue convenção `typst-passo-NNN-*.md`.

### §A.5 — Decisão 2 — Estado inicial das ADRs novas

Opções:

- **2α — Todas `EM VIGOR` directo**: paridade com ADR-0093/0094/0064/0065 (meta-ADRs criados directamente EM VIGOR).
- **2β — Todas `PROPOSTO`**: paridade com ADR-0066 (criada PROPOSTO para futura promoção). Mais conservador.

Recomendação spec: **2α**. Razões:
1. Sub-padrões já têm N≥3 evidência empírica concreta — não é proposta especulativa.
2. Paridade com ADR-0093/0094 que formalizaram sub-padrões em P271 directamente EM VIGOR.
3. ADRs meta são por natureza documentação retrospectiva; "PROPOSTO" seria desfasado.

### §A.6 — Decisão 3 — Numeração ADR

- **Decisão**: 0095, 0096, 0097 (consecutivos). Confirmar disponibilidade em §A.1.
- **Ordem**: por ordem alfabética conceptual ou cronológica de inauguração do sub-padrão?
  - Cronológica: DEBT-37 (P84.6) → Dedup Arc (P73 — anterior!) → Scope-out (P273.14).
  - Por ordem cronológica de inauguração: Dedup Arc (0095) → DEBT-37 (0096) → Scope-out (0097).
  - Recomendação: **cronológica por inauguração** — 0095 Dedup, 0096 Pattern DEBT-37, 0097 Scope-out.

Decisão final na Fase A.

### §A.7 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| ADRs duplicam conteúdo umas das outras | 3 ADRs meta podem ter overlap | §0 explicita cada ADR captura conceito distinto |
| Documento reflexão duplica ADRs | Reflexão tem secções equivalentes às ADRs | §A.3 estrutura documento foca trajectória + descobertas; ADRs focam mecânica formal |
| Over-formalização | Tentação de formalizar N=1-2 sub-padrões também | §0 anti-padrão explícito: 3 ADRs apenas |
| Sub-padrões mal calibrados | N≥3 verificação fiável? | §A.2 verificação literal de cada inauguração + reaplicações |
| Numeração colide | Outros passos podem ter criado ADRs entre P271 e P273.17 | §A.1 verificação empírica |
| LOC L0 cresce | Anotações L0 em `entities/gradient.md`? | P273.17 é passo administrativo; sem anotação L0 esperada |

### §A.8 — Critério de aceitação Fase A

- §A.1 confirma ADRs disponíveis (0095/0096/0097).
- §A.2 verificação empírica N≥3 para cada sub-padrão.
- §A.3 estrutura documento reflexão fixada.
- §A.4-A.6 Decisões 1+2+3 fixadas.

---

## §2 — Sub-passo P273.17.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental.

Anotar ADR-0091 — décima sétima anotação consecutiva.

Template:

```
## Anotação cumulativa P273.17 — Reflexão metodológica formal cluster Gradient

**Data**: 2026-05-XX.
**Motivo**: cluster Gradient feature-complete declarável pós-P273.16
(P273.5-P273.16 trajectória 12 sub-passos consecutivos). P273.17
formaliza 3 sub-padrões que atingiram limiar formalização N=3-4
via 3 ADRs meta novas + documento reflexão.

**ADRs criadas EM VIGOR**: ADR-0095 (Dedup `Arc::as_ptr` resources;
N=3), ADR-0096 (Pattern DEBT-37 `cell_origin_*` replicado; N=4),
ADR-0097 (Scope-out reconfirmado por Fase A; N=3).

**Documento reflexão**: `00_nucleo/diagnosticos/typst-cluster-gradient-reflexao.md`
— síntese da trajectória + descobertas metodológicas.

**Sub-padrões NÃO formalizados** (preserved emergentes N=1-2):
L3-only parent_bbox, Template-passo replicado, Layout duplo,
Extract helper inline, Triplicação Group bbox, Bug arquitectural
intencional corrigido, Bug latent scope creep. Anti-padrão
over-formalização evitado.

**Cluster Gradient pós-P273.17**: feature-complete + sub-padrões
formalizados + reflexão documentada. Pronto para saída definitiva.
```

---

## §3 — Sub-passo P273.17.C — Materialização (sem código)

**Magnitude**: S+ documental.

### Ordem literal

1. Fase A §1 produzida + Decisões fixadas.
2. ADR-0091 anotação §2 escrita.
3. **Criar ADR-0095** — "Dedup `Arc::as_ptr` resources" EM VIGOR.
4. **Criar ADR-0096** — "Pattern DEBT-37 `cell_origin_*` replicado" EM VIGOR.
5. **Criar ADR-0097** — "Scope-out reconfirmado por Fase A" EM VIGOR.
6. **Criar documento reflexão** `00_nucleo/diagnosticos/typst-cluster-gradient-reflexao.md`.
7. **Atualizar README ADRs** — tabela + distribuição (81 → 84; EM VIGOR 35 → 38; passos-chave entrada P273.17).
8. **Anotações cumulativas em ADRs pré-existentes** que mencionam os sub-padrões formalizados:
   - ADR-0054 (graded — Scope-out via ADR-0095).
   - ADR-0085 (diagnóstico imutável — usado em P273.14-P273.16).
   - DEBT-37 (Grid cell — pattern via ADR-0096).
9. Verificação final.

### Cap LOC (ADR-0094 Pattern 1 — documental)

- **L1 hard cap**: 0 LOC (passo administrativo).
- **L3 hard cap**: 0 LOC.
- **Tests hard cap**: 0 novos.
- **Documental hard cap**: ~1500 linhas markdown (3 ADRs × ~300-400 linhas + documento reflexão ~400-500 linhas).
- **Documental soft cap**: ~1200 linhas.

### Estrutura ADR-0095 — "Dedup `Arc::as_ptr` resources" (esboço)

```markdown
# ⚖️ ADR-0095: Dedup `Arc::as_ptr` resources (PDF reusable objects)

**Status**: `EM VIGOR`
**Data**: 2026-05-XX
**Autor**: Humano + IA
**Diagnóstico prévio**: `00_nucleo/diagnosticos/typst-passo-273-17-diagnostico.md` §A.2.3

## Contexto

[Inauguração P73 image_resources + reaplicação P263 pattern_resources + reaplicação P273.12 pattern bbox-aware.]
[N=3 cumulativo — limiar formalização N=3-4 atingido.]

## Decisão

Quando L3 export precisa de deduplicar resources PDF (imagens, patterns,
fontes, ...) cuja identidade é dada por `Arc<T>`, a chave de dedup
canónica é `Arc::as_ptr(x) as usize`. Para casos onde contexto adicional
distingue ocorrências do mesmo Arc, chave evolui para tuplo
`(arc_ptr, contexto)` (P273.12 DedupKey).

## Análise pureza paridade ADR-0029

L3 puro (export). L1 não toca.

## Consequências

- Dedup PDF resources funcional para Arc-identidade.
- Quando contexto matters (e.g. parent_bbox para gradients), chave
  expandida preserva correctness sem perder dedup quando contextos
  idênticos.
- Custo: HashMap<usize, idx> ou HashMap<DedupKey, idx>; lookup O(1).

## Alternativas

- α — Dedup por valor: comparar conteúdo. Custo: hash + eq por conteúdo.
- β — Sem dedup: PDF cresce N× para N usos. Rejeitado.

## Precedentes citáveis

P73 image_resources, P263 pattern_resources, P273.12 DedupKey bbox-aware.

## Próximos passos

Reaplicação a outros resources (fontes via P266 já usa pattern similar).
```

### Estrutura ADR-0096 — "Pattern DEBT-37 `cell_origin_*` replicado" (esboço)

```markdown
# ⚖️ ADR-0096: Pattern DEBT-37 — campo Layouter consumer-pending

**Status**: `EM VIGOR`
**Data**: 2026-05-XX
**Autor**: Humano + IA

## Contexto

[Inauguração P84.6 `cell_origin_x/y/w` + reaplicações P273.5/P273.6/P273.9.]
[N=4 cumulativo crossing limiar.]

## Decisão

Quando refino estrutural precisa de campo novo no `Layouter` que será
consumido em passo subsequente (não no passo da introdução), o pattern é:

1. Campo `Option<T>` adicionado ao struct Layouter.
2. Set/reset (save/restore LIFO) no arm consumidor estrutural.
3. `#[allow(dead_code)]` enquanto consumer não existe (consumer-pending).
4. Consumer activado em passo subsequente (e.g. L3 dispatcher).
5. `#[allow(dead_code)]` removido quando consumer activo.

## Análise pureza paridade ADR-0029

L1 puro. Campo é tipo dado (`Option<f64>` ou `Option<Rect>`); save/restore
é gestão RAM. Sem I/O.

## Consequências

- Refino incremental sustentável — campo introduzido com magnitude S;
  consumer activado com magnitude S separadamente.
- DEBT registado durante consumer-pending; fechado quando consumer activo.
- Disciplina: `#[allow(dead_code)]` é dívida visível; força fechamento.

## Alternativas

- α — Materializar campo + consumer no mesmo passo: magnitude M+; possível
  scope creep.
- β — Não introduzir campo: consumer não tem como persistir estado entre
  arms.

## Precedentes citáveis

P84.6 (Grid `cell_origin_x/y/w`), P273.5 (`apply_parent_transform`), P273.6
(consumer Block), P273.9 (Grid cell `parent_bbox`).

## Próximos passos

Aplicar pattern a outros campos Layouter que requerem consumer-pending
(e.g. `cell_available_h` em Grid; outros).
```

### Estrutura ADR-0097 — "Scope-out reconfirmado por Fase A" (esboço)

```markdown
# ⚖️ ADR-0097: Scope-out reconfirmado por Fase A

**Status**: `EM VIGOR`
**Data**: 2026-05-XX
**Autor**: Humano + IA

## Contexto

[Inauguração P273.14 + reaplicações P273.15 + P273.16.]
[N=3 cumulativo crossing limiar; 3 razões NO-GO distintas.]

## Decisão

Para pendências classificadas como "refinos qualitativos opcionais"
(per ADR-0054 graded), Fase A pode legitimamente decidir **NO-GO** com
output documental (`trabalho-previo-externo.md`) em vez de materialização.

3 razões NO-GO legítimas:
- **Constraints externas** (licensing, invariantes do projecto, crates
  não-autorizadas). Exemplo: P273.14 CMYK-ICC.
- **Constraints internas** (custo perf, ausência demanda empírica
  concreta). Exemplo: P273.15 bbox medido pós-layout.
- **Bloqueador estrutural aceito** (limitação consciente pré-existente
  documentada em ADR ou L0; refactor fora de escopo). Exemplo: P273.16
  bbox.y topo-exacto.

## Análise pureza paridade ADR-0029

N/A — decisão metodológica. Pureza preservada (zero código).

## Consequências

- NO-GO **não é falha** — é cumprimento honesto do critério "verificar
  empíricamente".
- Documento `trabalho-previo-externo.md` regista pré-requisitos para
  reabrir como GO futuro.
- Output documental é output legítimo per ADR-0054 graded.

## Alternativas

- α — Materializar mesmo sem demanda/com bloqueador: over-engineering.
- β — Saltar passo: pendência fica sem trabalho de diagnóstico documentado.

## Precedentes citáveis

P273.14 (constraints externas), P273.15 (constraints internas), P273.16
(bloqueador estrutural aceito).

## Próximos passos

Aplicar a outras pendências classificadas como refino qualitativo opcional.
```

### Estrutura documento reflexão (esboço)

8 secções per §A.3 acima. Cada secção 50-100 linhas markdown. Estilo:
factual, sem hyperbole, documentando trajectória sem prescrever.

### Verificação final

- ADRs 81 → 84 (3 EM VIGOR novas).
- Documento reflexão criado em `00_nucleo/diagnosticos/`.
- README ADRs atualizado.
- ADR-0091 anotação cumulativa décima sétima.
- Anotações cumulativas em ADR-0054, ADR-0085, DEBT-37.
- Hashes preserved (passo documental).
- Tests workspace 2644 preserved bit-exact (zero código).
- Lint zero.
- Sub-padrões §4 atualizados.

---

## §4 — Sub-padrões cumulativos pós-P273.17

| Sub-padrão | Pós-P273.16 | Pós-P273.17 |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 23 | **18 (a contagem pode parecer paradoxal)** |
| Reutilização literal helpers cross-passos | 17 | 17 (preserved) |
| Cap LOC hard vs soft explícito | 16 | 16 (preserved — passo administrativo sem cap LOC) |
| Aplicação meta-ADR (ADR-0093) | 12 | 13 |
| Aplicação meta-ADR (ADR-0094) | 12 | 12 (preserved — sem Pattern 1 cap aplicado) |
| Diagnóstico imutável | 32 | 33 (28º consumo) |
| Sub-passos consecutivos cluster | N=12 | **N=13 cumulativo emergente** |
| **Passo administrativo XS/S criar ADR EM VIGOR** | N=? (existe precedente P271 + P156K) | **N=? cresce** |

**Paradoxo aparente do sub-padrão "Anotação cumulativa em vez de ADR nova"**: contagem cumulativa preserved (já era 23 pós-P273.16 — anotação P273.17 em ADR-0091 cresce para 24); ao mesmo tempo, P273.17 **cria 3 ADRs novas** em vez de anotar. **Não é violação**: o sub-padrão refere-se a anotação **em vez de** criar ADR para anotações cumulativas de decisões existentes. P273.17 cria ADRs **meta novas** para sub-padrões emergentes (não anotações cumulativas). Conceitos diferentes.

### Sub-padrões emergentes preserved (não formalizados em P273.17)

§0 anti-padrão explícito: 7 sub-padrões emergentes N=1-2 ficam preserved sem formalização. Aguardar reaplicação fora cluster para consolidação cross-cluster.

---

## §5 — Limitações conscientes P273.17

- 3 ADRs meta novas captam **apenas os sub-padrões com N≥3 e valor metodológico claro**. Outros sub-padrões emergentes preserved.
- Documento reflexão é narrativa retrospectiva — não substitui passos individuais (cada passo tem o seu relatório imutável).
- Numeração ADR 0095/0096/0097 assume disponibilidade — verificação empírica necessária.
- ADRs EM VIGOR directo (Decisão 2α) — paridade P271 ADR-0093/0094.
- Reflexão captura trajectória mas não prescreve metodologia para clusters futuros — cada cluster tem natureza própria.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico.
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.8 + revê Decisões 1+2+3.
5. Utilizador executa P273.17.B + P273.17.C → relatório com 3 ADRs + documento reflexão.
6. Utilizador upload do relatório.
7. Claude web analisa + propõe saída do cluster Gradient (próximo cluster do projecto).

---

## §7 — Pendências preservadas pós-P273.17

Inalteradas vs P273.16:

- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Pendências cluster Gradient preserved:

- 3 scope-outs reconfirmados (P273.14/15/16).
- 2 candidatos XS NÃO reservados (helper-group-bbox, content-md-debt56-update).
- 1 pendência fora cluster exposta (draw-item-local-text-image).

**Pós-P273.17 cluster Gradient encerrado definitivamente**: feature-complete + qualitativo + refino estrutural + cleanup + dedup bbox-aware + render Groups + 3 scope-outs documentados + **sub-padrões formalizados via 3 ADRs meta + documento reflexão**.

---

## §8 — Critério de fecho do passo

P273.17 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.8 cumprido.
- ADR-0091 anotada (décima sétima anotação consecutiva).
- **ADR-0095 criada EM VIGOR** com estrutura canónica.
- **ADR-0096 criada EM VIGOR** com estrutura canónica.
- **ADR-0097 criada EM VIGOR** com estrutura canónica.
- **Documento reflexão** `typst-cluster-gradient-reflexao.md` criado em `00_nucleo/diagnosticos/`.
- README ADRs atualizado (81 → 84; EM VIGOR cresce 3).
- Anotações cumulativas em ADR-0054 + ADR-0085 + DEBT-37 onde aplicável.
- Zero código L1/L3.
- Tests workspace 2644 preserved bit-exact.
- Lint zero.
- Cap documental respeitado.

---

## §9 — Numeração — nota

Spec usa **P273.17** continuando a sequência decimal. **Último** sub-passo da sequência terminar cluster Gradient (passo administrativo de fecho).

Sequência completa:

- ✓ P273.5 — relative cross-variant.
- ✓ P273.6 — Block save/restore.
- ✓ P273.7 — Boxed save/restore.
- ✓ P273.8 — Cleanup 4 warnings.
- ✓ P273.9 — Grid + Stack + Pad.
- ✓ P273.10 — Group L3-only scan.
- ✓ P273.11 — Extract Stack helper.
- ✓ P273.12 — Dedup bbox-aware.
- ✓ P273.13 — Fix draw_item_local Group (INSERIDO).
- ✓ P273.14 — CMYK-ICC (SCOPE-OUT-RECONFIRMED).
- ✓ P273.15 — Bbox medido (SCOPE-OUT-RECONFIRMED).
- ✓ P273.16 — Bbox.y topo-exacto (SCOPE-OUT-RECONFIRMED).
- **P273.17 — Reflexão metodológica + 3 ADRs meta** (este passo; passo administrativo S+).

**Total**: 13 sub-passos consecutivos cluster Gradient.

Pós-P273.17, próximo passo natural: **sair do cluster Gradient definitivamente** para outro cluster (A1-A5 da decisão anterior B).

---

## §10 — Reflexão meta-meta

P273.17 documenta a trajectória metodológica do cluster Gradient — mas é ele mesmo parte dessa trajectória. Sub-padrão "passo administrativo XS/S criar ADRs meta" tem precedente:

- **P156K** — ADR-0064 + ADR-0065 EM VIGOR.
- **P271** — ADR-0093 + ADR-0094 EM VIGOR.
- **P273.17 (este)** — ADR-0095 + ADR-0096 + ADR-0097 EM VIGOR.

N=3 cumulativo se contado retrospectivamente. Padrão consolidado: clusters complexos terminam com passo administrativo que formaliza sub-padrões emergentes via ADRs meta.

Esta reflexão meta-meta **não é formalizada** em P273.17 (anti-padrão over-formalização explícito). Documentada para futuro: se algum cluster futuro terminar com mesmo padrão, então N=4 e candidato meta-ADR formal.

A disciplina é: documentar honestamente o que existe, formalizar apenas o que atinge limiar com valor metodológico claro, resistir à tentação de formalizar tudo.
