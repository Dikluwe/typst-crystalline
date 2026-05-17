# typst-passo-271 — Meta-formalização sub-padrões metodológicos via ADR-0093 + ADR-0094 EM VIGOR

**Magnitude**: XS (cap composto: 0 LOC L1/L3/stdlib; só documentação ADRs + anotações cumulativas + README).
**Cluster**: Metodologia / Specs / Documentação (administrativo).
**Tipo**: passo administrativo XS. Pattern análogo P260 (ADR-0084 + ADR-0085) / P156K / P160A / P268.1. Sub-padrão "Passo administrativo XS criar/promover ADR" **N=5 → N=6 cumulativo**.
**Origem**: relatório P270.4 §7 + §8 "candidato meta-ADR formalização URGENTE"; 5 sub-padrões atingiram limiar formalização clara N=4-10.
**Sequência**: P270 série completa → **P271 meta-formalização** → decisão humana abre próximo cluster/refino.
**Estratégia decidida**:
- Numeração P271 (passo principal; série P270 fechou).
- Estrutura D (2 ADRs temáticas cobrem 5 sub-padrões).
- ADRs entram `EM VIGOR` directos (sem PROPOSTO transitório; paridade P260 ADR-0084/0085 + P268.1 ADR-0090).

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md preservada literal** — sem código L1/L3/stdlib alterado. Apenas documentação. Ordem reduzida: registo histórico aplicações cumulativas → ADR-0093 + ADR-0094 criadas EM VIGOR → anotações cumulativas em ADRs/passos precedentes → README.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). **Sem Fase A filesystem nem web research nova** — diagnóstico empírico é o histórico de aplicações cumulativas dos sub-padrões registado literal em §1 desta spec; meta-evidência consolidada nas próprias ADR-0093 + ADR-0094 secções §"Histórico empírico".

3. **Sub-padrão "Passo administrativo XS criar/promover ADR"** **N=5 → N=6 cumulativo** (P156K/P160A/P229/P254/P268.1/**P271**).

4. **ADRs entram EM VIGOR directos** — pattern P260 (ADR-0084 + ADR-0085) + P268.1 (ADR-0090). Formalização de padrões empíricos N≥4 não requer PROPOSTO transitório porque a empiria já está estabelecida via aplicações cumulativas.

5. **Anti-pattern "ADR para tudo" salvaguardado** — meta-ADRs limitam-se a:
   - Sub-padrões com N≥4 (limiar formalização clara estabelecido).
   - Aplicabilidade cross-cluster comprovada (não só uma série isolada).
   - 5 sub-padrões deste passo satisfazem ambos critérios.

6. **ADR-0091/ADR-0092/ADR-0090/ADR-0083 preservadas literal** — meta-ADRs novas referem-nas como instâncias materializadas; não revogam.

7. **ADR-0054 anotação cumulativa P271** — perfil graded DEBT-1 reforçado por meta-formalização operacional.

8. **ADR-0039 preservado** — TextStyle intocado.

9. **Crystalline-lint zero violations** obrigatório (sem hash drift L0).

10. **Não inaugura sub-padrão "Meta-formalização cumulativa"** — meta-meta seria anti-pattern. P271 aplica pattern P260 directamente (passo administrativo XS para formalizar empiria).

11. **Não cria Fase A filesystem** — esta spec lista §1 histórico cumulativo literal; ADRs novas embebem como secção §"Histórico empírico". Substitui ficheiro `diagnostico-meta-passo-271.md` por documentação inline nas ADRs.

12. **Tabela ADRs precedentes para anotação cumulativa** — limitada a 4 ADRs onde sub-padrões foram materialmente instanciados (ADR-0083, ADR-0090, ADR-0091, ADR-0092). Outros passos referenciados via §"Histórico empírico" na ADR mas sem anotação cumulativa individual (evita dispersão).

---

## §1 — Histórico empírico consolidado (substitui Fase A filesystem)

Inline em spec; embebido literal em ADR-0093 + ADR-0094 §"Histórico empírico".

### §1.A — Sub-padrão "ADR scope-out revogado parcialmente" (N=6)

Sub-padrão: ADR existente com §"scope-outs" pode ter elementos revogados parcialmente em passos futuros sem revogação total da ADR. Status mantém `EM VIGOR` ou `IMPLEMENTADO`; revogação documentada via anotação cumulativa.

| N | Passo | ADR alvo | Elemento revogado |
|---|---|---|---|
| 1 | P267 | ADR-0088 §Conic | Conic L1+stdlib materializado (Conic sai do scope-out variants não materializados) |
| 2 | P269 | ADR-0088 §focal_* | focal_center + focal_radius materializados |
| 3 | P270 | ADR-0083 §ColorSpace runtime | ColorSpace runtime L1+stdlib activado |
| 4 | P270.2 | ADR-0083 §DeviceCMYK PDF | Linear+Radial CMYK L3 emit directo |
| 5 | P270.3 | ADR-0090 §Type 6 Coons | Type 6 Coons como estratégia adicional Conic (Type 7 preserved scope-out) |
| 6 | P270.4 | ADR-0083 §DeviceCMYK PDF + ADR-0091 §Conic CMYK | Revogação final absoluta (Conic CMYK activado) |

Limiar formalização clara N=4 ultrapassado em P270.2; pattern estabelecido sólido por N=6 P270.4.

### §1.B — Sub-padrão "Anotação cumulativa em vez de ADR nova" (N=10)

Sub-padrão: actualização significativa a ADR existente pode usar anotação cumulativa em vez de criar ADR nova, quando actualização preserva decisão de fundo da ADR-alvo.

| N | Passo | ADR-alvo | Motivo |
|---|---|---|---|
| 1 | P258.B | ADR-0070 style_chain | Refino sem revogar |
| 2 | P259.B | ADR-0073 geometry | Anotação Visualize |
| 3 | P263 | ADR-0087 Linear | PDF emit anotado |
| 4 | P265 | ADR-0088 Radial | PDF emit anotado |
| 5 | P268 | ADR-0089 Conic | PDF emit anotado |
| 6 | P268.2 | ADR-0089 Conic | Refino adaptive N |
| 7 | P270 | ADR-0083 + 5 ADRs cluster Gradient | Anotação cross-ADR (sub-padrão "Anotação cumulativa cross-ADR" inaugural) |
| 8 | P270.1 | ADR-0091 + ADR-0087/0088/0089/0090 | L3 multi-space refino |
| 9 | P270.2 | ADR-0091 + ADR-0083 + ADR-0087/0088/0089/0090 | CMYK directo Linear+Radial |
| 10 | P270.4 | ADR-0092 + ADR-0091 + ADR-0083 + ADR-0089 + ADR-0054 | Coons CMYK activação |

Consolidação clara N=10. Pattern estabelecido como mecanismo principal de evolução documental ADRs.

### §1.C — Sub-padrão "Reutilização literal helpers cross-passos" (N=10)

Sub-padrão: helpers materializados em passos anteriores devem ser reutilizados literal em passos subsequentes em vez de duplicar ou re-implementar. Cap LOC reduzido via reutilização.

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

Consolidação clara N=10. Mecanismo principal contenção LOC growth ao longo cluster Gradient.

### §1.D — Sub-padrão "Cap LOC hard vs soft explícito" (N=4)

Sub-padrão: spec deve distinguir caps hard (gate; dispara §política) de caps soft (informativo; regista no relatório). Inauguração veio como lição operacional pós-P270 (cap "ou" L1/stdlib não disparou §política apesar de estouro stdlib 60%).

| N | Passo | Cap hard / Cap soft (L3) | Real | Estado |
|---|---|---|---|---|
| 1 | P270.1 | 400 / 250 | ~45 | inaugural; folga grande |
| 2 | P270.2 | 250 / 150 | ~138 | soft estourou ~8 LOC (~5%); hard respected |
| 3 | P270.3 | 350 / 250 | ~250 | soft no limite exacto; hard respected |
| 4 | P270.4 | 200 / 150 | ~120 | ambos respected; folga 40%/20% |

Consolidação clara N=4. Pattern operacional estabelecido. Sub-padrão "estouro soft regista relatório sem disparar §política" verificado empíricamente.

### §1.E — Sub-padrão "Fase A com industry research proactiva" (N=4)

Sub-padrão: passos com decisão arquitectural não-trivial devem incluir pesquisa industry preventiva ANTES da spec, em vez de reactiva pós-divergência (lição P268.1 + P268.1-correção).

| N | Passo | Pesquisa industry |
|---|---|---|
| 1 | P270 | Vanilla docs + blog 2023 + issue #4422 (causa raiz CMYK gradient bug pre-spec) |
| 2 | P270.2 | Vanilla CMYK emit + pdfkit #532 + ISO 32000-1 §7.5.7 + PDF DeviceCMYK structure |
| 3 | P270.3 | Cairo Type 6/7 + Inkscape + W3C Workshop 2021 + pdf.js #6283 + PDFBOX-2100 + matplotlib #18034 + Stanislaw Adaszewski + Typst blog 2023 + ISO 32000-1 §7.5.7.4 (9 fontes) |
| 4 | P270.4 | Bug #4422 vanilla GitHub validation + reader compatibility Type 6 + DeviceCMYK |

Consolidação clara N=4. Pattern operacional estabelecido. Distingue-se de Fase A standard (filesystem read) — adiciona pesquisa web preventiva.

---

## §2 — Sub-passo P271.A — ADR-0093 criação EM VIGOR

Ficheiro novo `00_nucleo/adr/typst-adr-0093-meta-metodologia-evolucao-adrs.md`.

### Estrutura ADR-0093

```
# ADR-0093 — Meta-metodologia de evolução de ADRs (scope-out revogado parcialmente + Anotação cumulativa)

**Status**: EM VIGOR (criada directamente; paridade pattern P260 ADR-0084/0085 e P268.1 ADR-0090)
**Data**: 2026-05-17
**Passo origem**: P271 (passo administrativo XS meta-formalização)
**Cluster**: Metodologia / ADRs / Documentação
**Tipo**: meta-ADR formalizando 2 sub-padrões empíricos N≥6 cumulativos

## Contexto

Cluster Gradient série P270 (5 sub-passos) produziu 2 sub-padrões
metodológicos sobre evolução de ADRs que atingiram limiar formalização
clara N≥6:

1. **"ADR scope-out revogado parcialmente"** N=6 cumulativo —
   P267/P269/P270/P270.2/P270.3/P270.4.
2. **"Anotação cumulativa em vez de ADR nova"** N=10 cumulativo
   consolidação clara — P258.B/P259.B/P263/P265/P268/P268.2/P270/
   P270.1/P270.2/P270.4.

Ambos sub-padrões são sobre **como ADRs evoluem** ao longo do tempo
em cristalino: quando criar nova, quando anotar, quando revogar
parcialmente. Empiria já estabelecida; meta-ADR formaliza para
referência cross-cluster futura.

## Decisão — Pattern 1: ADR scope-out revogado parcialmente

**Quando aplicar**:
- ADR existente tem §"scope-outs" listando elementos não-materializados.
- Passo futuro materializa um (ou alguns) desses elementos sem
  invalidar decisão de fundo da ADR.
- Elementos revogados são localmente identificáveis (não revogam ADR
  inteira).

**Como aplicar**:
1. Anotar §"scope-outs" da ADR-alvo: elementos revogados marcados
   `revogado P<passo>` ou `~~strikethrough~~`.
2. Status da ADR-alvo preservado (`EM VIGOR` ou `IMPLEMENTADO`).
3. Adicionar §"Anotação cumulativa P<passo>" na ADR-alvo documentando
   revogação parcial.
4. Cross-reference em ADR nova (se criada) ou nas notas do passo.

**Quando NÃO aplicar**:
- Revogação invalida decisão de fundo: usar status `REVOGADO` e criar
  ADR substituta.
- Mudança expande decisão original significativamente: criar ADR nova
  com cross-reference (não anotar).

## Decisão — Pattern 2: Anotação cumulativa em vez de ADR nova

**Quando aplicar**:
- Actualização significativa a ADR existente sem mudar decisão de
  fundo.
- Refino paramétrico, materialização condicional, ou extensão local.
- Decisão é "como" evolui, não "se" evolui.

**Como aplicar**:
1. Adicionar §"Anotação cumulativa P<passo>" no fim da ADR-alvo.
2. Status preserved (`EM VIGOR` ou `IMPLEMENTADO`).
3. Conteúdo essencial: motivo, alteração estrutural, helpers
   reutilizados, defaults preservados, sub-padrões aplicados.
4. Cross-reference em README §"passos-chave" da entrada do passo.

**Quando NÃO aplicar**:
- Decisão arquitectural distinta surge: criar ADR nova.
- Anotação tornaria-se mais larga que a ADR original: criar ADR nova.
- Múltiplas ADRs precisam anotação simultânea coerente: avaliar
  sub-padrão "Anotação cumulativa cross-ADR" (N≥3 cumulativa via
  P270/P270.1/P270.2/P270.3/P270.4).

## Histórico empírico

[Tabelas §1.A (N=6 scope-out parcial) + §1.B (N=10 anotação
cumulativa) inseridas literal da spec P271]

## Decisão complementar — Anotação cumulativa cross-ADR

Sub-padrão derivado: passo único pode anotar múltiplas ADRs
simultaneamente quando alteração afecta cluster inteiro. Aplicado em
P270/P270.1/P270.2/P270.3/P270.4 (5 aplicações cumulativas; 6 ADRs
em P270 inaugural; 5 ADRs em P270.4 final). Não requer meta-ADR
separada — sub-caso de Pattern 2.

## Consequências

+ Mecanismo documentado preserva linhagem cumulativa ADRs.
+ Reduz proliferação ADRs (10 anotações cumulativas em série P270
  vs 10 ADRs novas hipotéticas).
+ Pattern recuperação via histórico empírico cross-cluster.
- Risco proliferação anotações cumulativas: salvaguarda via §"Quando
  NÃO aplicar".

## Alternativas consideradas

- **Status REVOGADO sempre que scope-out muda**: rejeitada — invalida
  ADRs que ainda têm decisão de fundo válida (e.g. ADR-0090 Type 4
  RGB preserved após Type 6 sair scope-out).
- **ADR nova sempre que actualização significativa**: rejeitada —
  proliferação documental (cluster Gradient teria ~78 ADRs em vez
  de actuais 79; ADRs perdem coerência).
- **Sub-padrões não formalizar**: rejeitada — N≥6 e N=10 muito
  ultrapassam limiar; meta-formalização para referência futura é
  factualmente justificada.

## Critério revisão

Esta ADR pode ser revisitada se:
- Sub-padrão "ADR scope-out revogado parcialmente" começar a
  invalidar decisões de fundo (deveria forçar REVOGADO; ADR-0093
  inválida).
- Anotações cumulativas tornarem-se larger que ADRs originais
  (forçaria criar ADRs novas; ADR-0093 inválida).
- Sub-padrão cross-ADR atingir N≥5: candidato meta-formalização
  dedicada ADR-0095.

## Subpadrões aplicados

- Passo administrativo XS criar ADR: N=5 → N=6 cumulativo.
- Auto-aplicação ADR-0065 inline: N=15 → N=16 cumulativo.

## Referências

- ADR-0084 + ADR-0085 (formalização P260; precedente pattern).
- ADR-0090 (formalização P268.1; precedente pattern).
- Passos cumulativos §1.A + §1.B desta ADR.
```

---

## §3 — Sub-passo P271.B — ADR-0094 criação EM VIGOR

Ficheiro novo `00_nucleo/adr/typst-adr-0094-meta-operacional-specs.md`.

### Estrutura ADR-0094

```
# ADR-0094 — Meta-operacional de specs (Cap LOC hard/soft + Reutilização helpers + Industry research proactiva)

**Status**: EM VIGOR (criada directamente; paridade P260 + P268.1)
**Data**: 2026-05-17
**Passo origem**: P271
**Cluster**: Metodologia / Specs / Operacional
**Tipo**: meta-ADR formalizando 3 sub-padrões empíricos N≥4 cumulativos

## Contexto

Cluster Gradient série P270 (5 sub-passos) produziu 3 sub-padrões
sobre **como specs são escritas e executadas** que atingiram limiar
formalização clara:

1. **"Cap LOC hard vs soft explícito"** N=4 (P270.1/P270.2/P270.3/P270.4).
2. **"Reutilização literal helpers cross-passos"** N=10 consolidação
   clara (P265 a P270.4 cumulativo).
3. **"Fase A com industry research proactiva"** N=4 (P270/P270.2/
   P270.3/P270.4).

Empiria estabelecida; meta-ADR formaliza para specs futuras.

## Decisão — Pattern 1: Cap LOC hard vs soft explícito

**Motivação**: lição operacional pós-P270 — cap LOC L1/stdlib
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
```
| Camada | Cap hard | Cap soft | Estimativa empírica |
|---|---|---|---|
| L3    | 250 LOC | 150 LOC | ~80-120 (§A.X diagnóstico) |
| Testes | 35     | 25      | ~20-25 |
```

**§política condição relacionada**:
```
N. Cap LOC L3 hard (250) ameaça ser ultrapassado — refactor maior
   que estimativa §A.X. Confirmar antes de continuar.
```

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
  compartilhada.

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

## Histórico empírico

[Tabelas §1.C (N=10 reutilização) + §1.D (N=4 cap hard/soft) + §1.E
(N=4 industry research) inseridas literal da spec P271]

## Consequências

+ Specs futuras seguem pattern consolidado (caps explícitos;
  reutilização proactiva; pesquisa preventiva).
+ Cap LOC contenção verificável via §"estouro soft ~X% registado".
+ Decisões arquitecturais informadas antes de spec (não pós).
- Adiciona ~30-60 min preventivos a specs com decisão arquitectural
  não-trivial.
- Tabela hard/soft adiciona verbosidade a specs simples (cap soft
  pode ser omitido se hard sozinho cobre).

## Alternativas consideradas

- **Cap único (sem hard/soft)**: rejeitada — lição P270 estouro
  silencioso.
- **Cap hard apenas (sem soft)**: rejeitada — soft permite registo
  empírico sem disparar §política (utilizado P270.1 inaugural).
- **Pesquisa industry reactiva sempre**: rejeitada — re-trabalho
  P268.1 + P268.1-correção justifica preventiva para casos
  arquiteturais.

## Critério revisão

Esta ADR pode ser revisitada se:
- Cap soft frequentemente estourado sem consequência: pode ser
  removido.
- Reutilização helpers tornar-se constrangimento de design (Liskov
  violation; helpers acoplados demais): refactor amplo.
- Pesquisa industry preventiva tornar-se pro-forma (sempre confirma
  Fase A standard): omitir.

## Subpadrões aplicados

- Passo administrativo XS criar ADR: N=5 → N=6 cumulativo.
- Auto-aplicação ADR-0065 inline: N=15 → N=16 cumulativo.

## Referências

- ADR-0084 + ADR-0085 (formalização P260; precedente meta-ADR).
- ADR-0090 (formalização P268.1; precedente meta-ADR).
- ADR-0029 (pureza física L1; relacionado Pattern 2 helpers cross-camada).
- Passos cumulativos §1.C + §1.D + §1.E desta ADR.
```

---

## §4 — Sub-passo P271.C — Anotações cumulativas em ADRs precedentes

Cada anotação curta cross-reference às novas meta-ADRs:

### §4.1 — ADR-0083 anotação cumulativa P271

Adicionar após §"Anotação cumulativa P270.4":

```
## Anotação cumulativa P271 — Sub-padrões formalizados

Sub-padrão "ADR scope-out revogado parcialmente" aplicado a esta ADR
em P270/P270.2/P270.4 formalizado via **ADR-0093 EM VIGOR**
(meta-metodologia evolução ADRs).

Anotações cumulativas P270/P270.4 desta ADR participam pattern
"Anotação cumulativa em vez de ADR nova" também formalizado
ADR-0093.
```

### §4.2 — ADR-0090 anotação cumulativa P271

Adicionar após §"Anotação cumulativa P270.3":

```
## Anotação cumulativa P271 — Sub-padrões formalizados

Sub-padrão "ADR scope-out revogado parcialmente" aplicado a esta ADR
em P270.3 (Type 6 Coons sai do scope-out) formalizado via ADR-0093
EM VIGOR.

Sub-padrão "Anotação cumulativa em vez de ADR nova" aplicado em P270.3
formalizado via ADR-0093 EM VIGOR.
```

### §4.3 — ADR-0091 anotação cumulativa P271

Adicionar após §"Anotação cumulativa P270.4":

```
## Anotação cumulativa P271 — Sub-padrões formalizados

Esta ADR é centro de aplicação dos sub-padrões formalizados:
- "Anotação cumulativa em vez de ADR nova" (4 anotações cumulativas
  P270.1+P270.2+P270.3+P270.4) → ADR-0093.
- "Anotação cumulativa cross-ADR" (cada anotação afecta múltiplas
  ADRs simultâneo) → ADR-0093 §"Anotação cumulativa cross-ADR".
- "Reutilização literal helpers cross-passos" (helpers P262/P265/
  P268/P270/P270.1-P270.4) → ADR-0094.

Ver ADR-0093 + ADR-0094 EM VIGOR para meta-formalização.
```

### §4.4 — ADR-0092 anotação cumulativa P271

Adicionar após §"Anotação cumulativa P270.4":

```
## Anotação cumulativa P271 — Sub-padrões formalizados

Esta ADR aplica sub-padrões formalizados:
- "Fase A com industry research proactiva" (P270.3 pesquisa 9 fontes
  pré-spec) → ADR-0094.
- "Cap LOC hard vs soft explícito" (P270.3 N=3; P270.4 N=4
  consolidação) → ADR-0094.
- "Reutilização literal helpers cross-passos" (helpers P270.3 RGB
  reutilizados P270.4 CMYK) → ADR-0094.
- "Anotação cumulativa em vez de ADR nova" (P270.4 anotação) →
  ADR-0093.

Ver ADR-0093 + ADR-0094 EM VIGOR.
```

### §4.5 — ADR-0054 anotação cumulativa P271

Adicionar após §"Anotação cumulativa P270.4":

```
P271 — meta-formalização sub-padrões metodológicos via ADR-0093 +
ADR-0094 EM VIGOR. Perfil graded DEBT-1 reforçado por mecanismos
operacionais documentados (caps explícitos, reutilização literal,
industry research proactiva).
```

### §4.6 — L0 prompt — sem alteração

L0 `entities/gradient.md` **não recebe** anotação P271 — meta-ADRs são metodológicas, não tocam semântica gradient. Hash drift zero L0.

---

## §5 — Sub-passo P271.D — README + relatório

### D.1 — README ADRs

- **Tabela**: 2 linhas novas (ADR-0093 EM VIGOR + ADR-0094 EM VIGOR).
- **Distribuição**: PROPOSTO 11 preserved; **EM VIGOR 33 → 35** (+2); IMPLEMENTADO 31 preserved; total **79 → 81**.
- **Passos-chave**: entrada P271 ~40-60 linhas (passo administrativo XS análogo P260; cross-reference 2 meta-ADRs).

### D.2 — Relatório

`00_nucleo/materialization/typst-passo-271-relatorio.md`:

- §1 Sumário executivo (2 meta-ADRs EM VIGOR criadas; 5 sub-padrões formalizados; sem código alterado).
- §2 ADR-0093 estrutura sumário.
- §3 ADR-0094 estrutura sumário.
- §4 Anotações cumulativas 4 ADRs precedentes + ADR-0054.
- §5 README distribuição actualizada.
- §6 Métricas finais (testes preservados 2572; ADRs 79 → 81; lint zero; LOC zero).
- §7 Sub-padrões aplicados + N cumulativo.
- §8 Pesquisa empírica embebida nas ADRs (sem ficheiro filesystem novo).
- §9 Critério aceitação checklist.
- §10 Referências.

---

## §política de paragem

1. **ADR-0093 ou ADR-0094 estrutura ambígua** — anotação cross-ADR conflito com texto meta-ADR. Confirmar.

2. **Crystalline-lint reporta violations** após anotações em ADRs (improvável; ADRs documentais não têm hashes).

3. **README distribuição inconsistente** — total ADRs antes/depois não bate (esperado: 79 → 81; PROPOSTO 11 preserved; EM VIGOR 33 → 35; IMPLEMENTADO 31 preserved).

4. **Tests workspace regressão** — 2572 baseline preservado obrigatório. Sem código alterado; regressão indica build cache stale.

5. **Cap LOC ameaçado** — passo é XS por definição (cap 0 LOC L1/L3/stdlib); qualquer alteração de código indica scope creep.

6. **Tabelas históricas §1.A-§1.E factualmente incorrectas** — verificar contra relatórios passos cumulativos.

7. **Anotações cumulativas cross-reference duplicadas** — alguma ADR já tem anotação P271 (improvável; primeira aplicação).

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N pós-P271 | Nota |
|---|---|---|
| Passo administrativo XS criar/promover ADR | **N=5 → N=6 cumulativo** | + P271 (P156K/P160A/P229/P254/P268.1/**P271**) |
| Auto-aplicação ADR-0065 inline | N=15 → N=16 | + P271 |
| Meta-formalização sub-padrões empíricos N≥4 | **N=2 → N=3 cumulativo** | P260 (ADR-0084/0085) + P268.1 (ADR-0090) + **P271 (ADR-0093/0094)** — sub-padrão emergente N=3; candidato meta-meta-ADR futura se N≥4-5 |

### Marco metodológico P271

**Cluster Gradient série P270 produziu cluster metodológico** —
sub-padrões formalizados meta-ADRs paridade P260. Cristalino agora
tem ferramenta operacional consolidada para specs futuras: caps
explícitos hard/soft; reutilização literal helpers; industry
research proactiva; evolução ADRs via scope-out parcial e anotação
cumulativa.

**Sub-padrão "Meta-formalização sub-padrões empíricos N≥4" N=3
cumulativo** — emergent; pode atingir limiar formalização se passos
futuros gerarem novas meta-ADRs. Candidato meta-meta-ADR no horizonte
(não urgente).

### Sequência pós-P271

Decisão humana fica em aberto entre pendências preservadas
relatório P270.4 §7:

- **P-Gradient-Coons-RGB-Final** (M; converge Conic RGB Type 4 → Type 6).
- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).
- **P-Gradient-CMYK-ICC** (S-M; PDF/A compliance).
- **P-Gradient-Adaptive-Multispace** (S; HSL/Oklch hue diff alto).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266).
- **P-Footnote-N** (M).
- **DEBT-33 / Stroke<Length> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

---

## §referências cross-passos

- **P260** — ADR-0084/0085 criadas EM VIGOR (precedente meta-ADR; pattern aplicado P271).
- **P268.1** — ADR-0090 criada EM VIGOR (precedente meta-ADR; pattern aplicado P271).
- **P262-P270.4** — cluster Gradient cumulativo (origem 5 sub-padrões formalizados P271).
- ADR-0093 — Meta-metodologia evolução ADRs (criada EM VIGOR P271).
- ADR-0094 — Meta-operacional specs (criada EM VIGOR P271).
- ADR-0083 — Color paridade (anotada cumulativa P271).
- ADR-0090 — Type 4 strategy (anotada cumulativa P271).
- ADR-0091 — ColorSpace runtime (anotada cumulativa P271).
- ADR-0092 — Conic Coons (anotada cumulativa P271).
- ADR-0054 — Perfil graded (anotada cumulativa P271).
- ADR-0085 — Diagnóstico imutável (preservado; pesquisa empírica inline neste passo paridade P260).

---

## §0.1 — Notas de execução para Claude Code

- **Sem código L1/L3/stdlib alterado**. Cap 0 LOC. Se qualquer alteração código tentada, §política condição 5 dispara.
- **Sem hash drift L0** — meta-ADRs não tocam semantic prompts.
- **Tests workspace 2572 preservados** — sem testes novos; sem regressão.
- **ADR-0093 + ADR-0094 estrutura literal** segue §2 + §3 spec; texto pronto para copiar.
- **Anotações cumulativas 4 ADRs + ADR-0054** são edições pequenas (~5-15 linhas cada).
- **Distribuição ADRs**: total 79 → 81 (ADR-0093 + ADR-0094 EM VIGOR; sem passagem PROPOSTO paridade P260 + P268.1).
- **Lint zero violations**: sem hashes a propagar.
- **Tabelas históricas §1.A-§1.E**: verificar literal contra relatórios passos referenciados (P267, P269, P270, P270.1, P270.2, P270.3, P270.4, P258.B, P259.B, P263, P265, P268, P268.2).
- **Relatório final esperado**: 2572 testes verdes preservados; hash drift 0; lint zero; ADRs 79 → 81; zero LOC.
- **Marco "Cluster metodológico formalizado"** documentado em relatório §1 + ADR-0093 §"Contexto" + ADR-0094 §"Contexto".
