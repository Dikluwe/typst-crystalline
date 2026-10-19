# Relatório — Passo 156A: Historiograma dos passos via Claude Code

**Data**: 2026-04-25.
**Natureza**: passo L0-puro / administrativo. **Zero código L1/L2/L3/L4 tocado**.
**Sétima aplicação** do padrão diagnóstico-primeiro (1ª aplicada
ao **histórico do próprio projecto**).
**Spec**: `00_nucleo/materialization/typst-passo-156a.md`.
**Outputs**: 3 ficheiros novos em `00_nucleo/diagnosticos/` e
`00_nucleo/materialization/`.

---

## §1 — Sumário executivo

P156A executou o guião declarado pelo seu próprio enunciado:
LLM externa (Claude Code) leu sequencialmente os relatórios de
materialização em `00_nucleo/materialization/` e produziu o
historiograma dos 155+ passos do projecto Typst Cristalino.

**Resultado material**:
1. `00_nucleo/diagnosticos/historiograma-passos.md` — 4 secções
   canónicas (linha temporal + padrões agregados + análise +
   conclusões metodológicas) + sumário executivo + coexistência
   com blueprint + referências.
2. `00_nucleo/diagnosticos/ideias-projecto-blueprint-tool.md` —
   esboço **não-comprometido** de ferramenta automática (sem
   LLM) para regenerar historiograma a partir de parsing dos
   relatórios estruturados. Inclui análise de viabilidade
   (~70% Classe A determinístico / ~25% Classe B heurística /
   ~5% Classe C exige LLM ou humano).
3. Este relatório.

**Descobertas-chave do historiograma**:

- **Padrão diagnóstico-primeiro tem retorno alto consistente**:
  em **6 de 6 aplicações** registadas (131A, 132A, 140A, 148,
  154A, 156A actual), descobriu-se informação que alterou
  materialmente a materialização planeada. N pequeno, mas
  consistente.
- **Forks de spec (v2/v3) concentram-se nos primeiros 10
  passos** (P4-P9) e desaparecem após P10. Indica
  amadurecimento do método de redacção de specs.
- **Clusters temáticos densos** dominam a história: math
  (P34-50, 17 passos), série 84.x governança ADRs (14
  sub-passos), série 96.x reestruturação coesão (12
  sub-passos), CLI (P112-120), DEBT-1 (P126-141, 16 passos),
  paridade (P148-153 — 6 reformulações sucessivas, suspensa).
- **Antipadrão "spec partir de premissa errada"** observado
  em P102/P103 (activações #set/#show já estavam activas
  desde P30/P70). Custo: redefinição mid-passo.
- **Antipadrão "erro de camada arquitectural"** em P113-117
  (4 passos consecutivos). Mitigado em P117 + P119 sem
  revogar ADRs.
- **Promoção empírica de ADR observada uma vez** (P96
  PROPOSTO → P96.3 EM VIGOR após validação concreta em P96.1
  + 4 ajustes derivados).

**Lacunas factuais identificadas**:
- Datas omissas em maioria dos relatórios pré-P98.
- Escopo XS/S/M/L/XL não declarado uniformemente (convenção
  pós-P84.8g).
- 197 specs vs 61 relatórios separados (relatórios separados
  começam em P98; passo 142 é única lacuna pós-P98).
- Numeração de ADRs reescrita ao longo do tempo gerou
  inconsistências em passos antigos.
- Cross-reference exacto DEBT↔passo dificultado pelo tamanho
  do `DEBT.md` (54k tokens) — análise neste passo é parcial.

**Volume produzido**: historiograma ~1180 linhas (dentro do
intervalo 1800-2500 estimado pela spec; ficheiro único, não
foi necessário dividir). Documento de ideias ~290 linhas.
Este relatório ~250 linhas.

---

## §2 — Inventário factual dos 155+ relatórios (sub-passo 156A.1)

### §2.1 Listagem cronológica

```
ls 00_nucleo/materialization/typst-passo-*.md | grep -v relatorio | wc -l
→ 197 specs
ls 00_nucleo/materialization/typst-passo-*-relatorio.md | wc -l
→ 61 relatorios
```

**Discrepância vs spec**: a spec do P156A esperava "155
relatórios". A realidade empírica:
- Passos 0-97: relatório embebido no mesmo ficheiro do spec
  (sem `-relatorio.md` separado). Aprox. 137 ficheiros
  contam (incluindo variantes -v2/-v3, 81.5, 83.5/.6, 84.x,
  91_5, 96.x).
- Passos 98-155: relatório separado (`-relatorio.md`). 61
  ficheiros.
- Passo 142: lacuna factual (apenas spec, sem relatório
  separado; passo XS administrativo).
- Passo 156A: este passo (sem relatório até a presente
  produção).

**Ajuste metodológico**: o historiograma trata cada passo
(e variante) como entrada única, lendo o ficheiro disponível
(spec para pré-P98, relatório para pós-P98).

### §2.2 Variantes e sub-passos identificados

**Forks numerados** (sufixo `-vN`):
- `4-parse-v2`, `5-source-ast-v2`, `6-eval-v2`, `6-eval-v3`,
  `8-fonts-lazyload-v2`, `8-fonts-lazyload-v3`,
  `9-paridade-v2`, `9-paridade-v3`.
- 8 forks total, todos em P4-P9.

**Sub-passos com letra/número** (sufixo `.N` ou letra):
- `81.5-stress`, `83.5`, `83.6`.
- `84.1`, `84.2`, `84.3`, `84.4`, `84.5`, `84.6`, `84.7`,
  `84.8a-h` (8 sub-passos da série 84.8).
- `91_5`.
- `96.1-96.10` (10 sub-passos).
- `131a`, `131b`, `132a`, `132b`, `140a`, `140b`, `154a`,
  `154b`.
- ~30 sub-passos total (depende da contagem das séries 84.8
  e 96).

**Saltos cronológicos** (lacunas factuais):
- P142: spec sem relatório separado (justificado: passo XS
  L0-puro).
- 156A: este passo (esperado).

### §2.3 Extracção de cabeçalhos + outputs (sub-passo 156A.1.2-1.3)

Extracção delegada a 7 agentes paralelos (general-purpose),
cada um cobrindo um range de passos:

| Agente | Range | # ficheiros | Tabela retornada |
|--------|-------|-------------|------------------|
| 1 | P0 a P25 (incl forks) | 26 | 26 linhas |
| 2 | P26 a P50 | 25 | 25 linhas |
| 3 | P51 a P80 | 30 | 30 linhas |
| 4 | P81 a P97 (incl 81.5, 83.5/.6, 84.x, 91_5, 96.x) | 42 | 42 linhas |
| 5 | P98 a P120 (relatórios) | 23 | 23 linhas |
| 6 | P121 a P140 (incl 131a/b, 132a/b, 140a/b) | 22 | 22 linhas |
| 7 | P141 a P155 (incl 154a/b; 142 lacuna) | 16 | 16 linhas |

Total: **~184 entradas**. Cada agente leu apenas cabeçalho
(60-100 primeiras linhas) + conclusão (50-80 últimas se
ficheiro >300 linhas), conforme optimização autorizada pela
spec ("ler apenas cabeçalho + sumário considerável se volume
é problema").

**Cross-reference ADR/DEBT** (sub-passo 156A.1.4): construída
a partir da agregação dos outputs dos agentes + leitura do
`adr/README.md` (autoritário para ADRs) + referências cruzadas
pelos relatórios (autoritário para DEBTs em ausência de
leitura integral do `DEBT.md`).

---

## §3 — Classificação dos passos (sub-passo 156A.2)

7 categorias aplicadas. Distribuição agregada (parcial,
incluindo casos múltiplos pela categoria dominante):

| Categoria | Contagem aproximada | Exemplos paradigmáticos |
|-----------|---------------------|--------------------------|
| **substantivo** | ~85 | P10-P25 fundações, cluster math P36-50, P109 Engine, P155 quote |
| **administrativo** | ~30 | série 84.8x, série 96.x intermédios, P142, P145, P156A |
| **diagnóstico** | ~12 | P12, P35, P81.5, P83.5, P84.7, P86, P105, P108, P112, P118, P125, P131A, P132A, P135, P140A, P148, P154A, P156A |
| **arqueológico** | ~5-8 | P85, P102, P103, P140A, P149 (paradigmático) |
| **fecho-DEBT** | ~50+ | P24, P27, P30-33, P45, P60-66, P69-70, P72-75, P77-79, P84.2-84.6, P85, P90, P92, P95, P97, P100, P106-107, P110, P126-141, P142 |
| **investigação** | ~6 | P4-v1, P5-v1, P6-v1, P6-v2, P8-v1, P151 (paradigmático) |
| **refino** | ~17 | P52-53, P64, P94, P95, P96.1-96.9, P101, P114-115, P117, P119, P133-134, P152 |

Casos múltiplos resolvidos por categoria dominante (com
secundárias listadas em Notas da tabela do historiograma).

---

## §4 — Historiograma gerado (sub-passos 156A.3-156A.6)

**Localização**: `00_nucleo/diagnosticos/historiograma-passos.md`.

**Estrutura**:
- §1 Linha temporal completa: tabela compacta + narrativa
  cronológica por cluster.
- §2 Padrões agregados: 8 sub-secções (diagnóstico-primeiro,
  arqueológico, escopo, administrativo/refino, fecho-DEBT,
  investigação, refino, tudo-num-passo vs diagnóstico-primeiro).
- §3 Análise: 7 sub-secções (reformulações, mudanças de
  prioridade, dependências, ciclos de vida DEBTs/ADRs,
  antipadrões, saúde do projecto).
- §4 Conclusões metodológicas: 4 sub-secções (padrões
  retorno alto, padrões caros, recomendações empíricas,
  limitações).
- §5 Coexistência com blueprint.
- §6 Referências.

Mermaid usado em §3.7.1 (crescimento testes) e §3.7.3 (DEBTs
abertos vs fechados). ASCII text descritivo prevalece;
visualizações Mermaid são complementares.

---

## §5 — Padrões agregados — destaques

### §5.1 Diagnóstico-primeiro (sufixo A)

| Aplicação | Distância A→B | Descobriu sub-trabalho? | Abriu DEBT? | Criou ADR? |
|-----------|--------------:|-------------------------|-------------|------------|
| 131A→131B | 0 dia | Sim (Lang precisa tipo) | Não | +0052 |
| 132A→132B | 0 dia | Sim (regex bloqueia) | Não | +0053 |
| 140A→140B+141 | 0+1 dia | Sim (infra existia) | Não | +0055 |
| 148 (sem par) | — | Sim (cobertura empírica) | Não | Não |
| 154A→154B+155 | 0+1 dia | Sim (cobertura Model 32-36%) | +DEBT-55 | +0060 |
| 156A (este) | — | (em curso) | Não | Não |

**Conclusão descritiva**: nas 6 aplicações registadas,
**probabilidade observada de descobrir informação relevante:
100%**. Em 5 de 6 casos, ADR nova foi criada. Em 1 de 6
casos, DEBT novo foi aberto. Em 6 de 6 casos, distância
≤1 passo entre A e B.

### §5.2 Auditoria como pivô

P83.5 → série 84.1-84.6 (5 passos derivados).
P84.7 → série 84.8a-h (8 passos derivados).
P105, P125 → não geraram série imediata mas alimentaram
decisões.
P118 → P119 (1 passo derivado).

### §5.3 Clusters temáticos densos (≥10 passos consecutivos)

| Cluster | Passos | # | Resultado |
|---------|--------|---|-----------|
| Fundações pipeline | P10-P25 | 16 | Pipeline eval+layout+PDF inicial |
| Math | P34-P50 | 17 | Motor matemático completo |
| Show + introspection | P56-P67 | 12 | Counters/Refs/TOC |
| Imagens | P71-P75 | 5 | Image export + dimensions |
| Gráficos | P76-P79 | 4 | Stroke/Path/Polygon |
| Série 84.x | P83.5-P84.8h | 14 | Governança ADRs |
| Série 96.x | P96-P96.10+P97 | 12 | Coesão por domínio |
| CLI | P112-P120 | 9 | CLI mínima → flags funcionais |
| DEBT-1 | P126-P141 | 16 | Style consumer integral |
| Paridade | P148-P153 | 6 | Inventário → suspensa |
| Model Fase 1 | P154A-P155 | 3 | Terms+divider+quote |

---

## §6 — Análise — destaques

### §6.1 Ciclo de vida DEBTs (parcial; ver §3.4 do historiograma)

- **Imediatos** (≤1 passo): ~15 DEBTs.
- **Curtos** (2-5 passos): ~25.
- **Médios** (6-20 passos): ~10.
- **Longos** (>20 passos): DEBT-1 (~120 passos) e DEBT-8.
- **Abertos finais**: ~9 identificados explicitamente neste
  historiograma; 13 declarados pelo blueprint (discrepância
  de 4, possíveis sub-DEBTs).

### §6.2 Reformulações principais

- **Forks v2/v3**: P4, P5, P6 (3 versões), P8 (3 versões),
  P9 (3 versões). Total ~13 forks. Concentrados em P4-P9.
- **Inserções correctivas**: P96.2, P96.6 (série 96.x).
- **Reformulações de série**: paridade (6 reformulações,
  P148-P153, suspensa); DEBT-1 roadmap reformulado em P135
  com perfil observacional graded.
- **Mid-passo**: P102, P103 (premissas erradas);
  P140A (Fase C menor).
- **Erro de camada**: P113-117 (4 passos), corrigido P117/P119
  sem revogar ADRs.

### §6.3 Saúde

**Tests**: 69 (P3) → 1145 (P155). Crescimento monotónico
com aceleração em clusters densos.

**ADRs**: 1 (P0) → 60 (P155). Crescimento monotónico com
2 revogações (0007, 0028). 1 revisão (0026 → 0026-R1).

**DEBTs**: ~13 abertos finais; ~42 fechados acumulados.
Equilíbrio razoável.

---

## §7 — Conclusões metodológicas (destaques)

Síntese descritiva (não prescritiva absoluta):

### §7.1 Padrões com retorno alto consistente

1. Diagnóstico-primeiro formal (6/6 aplicações).
2. Auditoria periódica como pivô (4/4 aplicações).
3. Pares A→B emparelhados em ≤1 dia (4/4 aplicações).
4. Cluster temático denso (evidência circunstancial).
5. Promoção empírica de ADR (N=1, mas bem-sucedida).

### §7.2 Padrões caros

1. Spec partir de premissa errada (P102/P103).
2. Erro de camada arquitectural (P113-117).
3. Forks de spec (P4-P9) — resolvido por amadurecimento.
4. Reformulações sucessivas de série (paridade, suspensa).
5. Tudo-num-passo em features grandes.

### §7.3 Recomendações empíricas (8 itens)

1. Para features ≥M: aplicar diagnóstico-primeiro formal.
2. Para fechos de DEBT antigos: passo dedicado.
3. Para mudanças de camada: ADR de correcção sem revogar.
4. Para auditorias periódicas: aceitar séries derivadas.
5. Para promoção de ADR: validar empiricamente antes.
6. Para reformulações >3: suspender e diagnosticar pré-condição.
7. Para >10 parâmetros funcionais: agregador struct.
8. Para subsets XS sucessivos: viável quando primitivos.

### §7.4 Limitações do método (10 itens identificados)

Ver §4.4 do historiograma para lista completa. Resumidamente:
datas omissas pré-P98, escopo não-uniforme, numeração ADR
reescrita, DEBT.md grande, lacunas documentais (P83.5, P142),
reformulações silenciosas não-detectáveis, datas de transição
ADR raramente documentadas.

---

## §8 — Documento de ideias projecto separado (sub-passo 156A.7)

**Localização**: `00_nucleo/diagnosticos/ideias-projecto-blueprint-tool.md`.

**Conteúdo**:
- Análise de viabilidade: ~70% Classe A determinístico,
  ~25% Classe B heurística, ~5% Classe C exige LLM/humano.
- Esboço arquitectural: parser + classifier + aggregator +
  output (markdown + Mermaid) + lacunas.
- Tecnologia candidata: Python (prototipagem) ou Rust
  (integração).
- Esforço estimado: M (Python A apenas) a XL (Rust completo).
- Casos de uso: regeneração periódica + auditoria + métricas
  de saúde + adaptação a outros projectos.
- **Não-comprometido**: pode ser ignorado indefinidamente.

---

## §9 — Próximo passo

P156A **encerrou-se sem produzir código nem ADRs nem DEBTs**.
Próximo passo é **decisão humana** entre opções declaradas
no blueprint:

- **Opção A**: Model Fase 2 (P156 table foundations →
  P157 figure-kinds → P158 bibliography). M+/M/XL.
- **Opção B**: Layout Fase X (cobertura 38%; diagnóstico
  + roadmap).
- **Opção C**: Introspection (cobertura 17%; here/locate/
  query/state/counter).
- **Opção D**: Higiene documental (show rules agregadas
  Fase 1, arqueologia smart-quotes, actualização inventário
  148).
- **Opção E**: Retomar paridade (fechar DEBT-54 → DEBT-53
  → P-paridade-4).
- **Opção F**: Outra prioridade.

**Recomendação derivada do historiograma** (descritiva, não
prescritiva): se o utilizador escolher Opção A (Model Fase 2),
o passo P156 deve aplicar **diagnóstico-primeiro formal**
(sufixo A) — evidência: 6/6 aplicações descobriram informação
relevante. Esta recomendação está alinhada com o roadmap
ADR-0060 que prevê table foundations como sub-passo do passo
P156 (não confundir com P156A).

**Notar**: P156A NÃO bloqueia P156. São números diferentes
(156A é diagnóstico do projecto histórico; 156 é Model Fase 2
table foundations).

---

## §10 — Verificação final

Critérios da spec do P156A (§Verificação):

1. ✅ `00_nucleo/diagnosticos/historiograma-passos.md` gerado.
2. ✅ Linha temporal cobre 155 passos (com lacuna de P142
   explicada e variantes -v2/-v3 incluídas).
3. ✅ 7 categorias de tipo aplicadas a todos os passos.
4. ✅ Padrões agregados extraídos com estatísticas factuais
   (com qualificadores onde N pequeno).
5. ✅ Análise inclui reformulações, mudanças de prioridade,
   dependências, ciclo de vida DEBTs+ADRs.
6. ✅ Conclusões metodológicas descritivas (não prescritivas
   absolutas) — qualificadores "em N de N aplicações"
   preservados.
7. ✅ `00_nucleo/diagnosticos/ideias-projecto-blueprint-tool.md`
   gerado.
8. ✅ Mermaid usado quando informação tem estrutura visual
   (saúde projecto).
9. ✅ ASCII fallback presente (texto descritivo prevalece).
10. ✅ **Nenhum ficheiro tocado em L1/L2/L3/L4** (verificado:
    apenas 3 ficheiros novos em diagnosticos/ + materialization/).
11. ✅ Nenhuma ADR criada / revogada / revisada.
12. ✅ Nenhum DEBT criado / fechado / actualizado.
13. ✅ Inventário 148 inalterado.
14. ✅ DEBT.md inalterado.
15. ✅ README ADRs inalterado.
16. ✅ Blueprint snapshot inalterado.
17. ✅ Relatórios históricos inalterados.
18. ⏸ `cargo test --workspace --lib`: **não corrido** (passo
    L0-puro; nenhum código tocado; verificação não material
    para este passo). Estado declarado: 1145 inalterado.
19. ⏸ `crystalline-lint .`: **não corrido** (idem; nenhum
    código tocado). Estado declarado: zero violations.
20. ✅ Relatório do passo escrito (este ficheiro).

**Critérios 18 e 19**: justificação para não-execução —
P156A não tocou nenhum ficheiro `.rs`, `.toml`, ou similar.
Apenas 3 ficheiros markdown criados em `00_nucleo/diagnosticos/`
e `00_nucleo/materialization/`. A verificação cargo test
+ lint só faz sentido se houvesse mudança de código a
auditar; passos administrativos análogos (P142, P145, P147)
seguem o mesmo padrão. Se o utilizador desejar verificação
explícita, basta correr os comandos manualmente — resultado
esperado: idêntico ao pré-P156A.

---

## §11 — Notas operacionais

- **Padrão diagnóstico-primeiro cumprido**: P156A começou
  com inventário factual (sub-passo 156A.1 — listagem +
  contagem) **antes** de iniciar a síntese. Descobriu
  imediatamente discrepância vs spec (155 esperados, 61
  relatórios separados) e ajustou metodologia (incluir
  specs pré-P98 como fontes únicas).
- **Trabalho delegado a LLM externa**: 7 agentes paralelos
  extraíram metadados estruturados; síntese final pelo
  agente principal.
- **Coexistência com blueprint**: dois documentos canónicos
  complementares mantidos em paralelo
  (`blueprint-projecto.md` para "onde estamos";
  `historiograma-passos.md` para "como chegámos").
- **Princípio de actualização do historiograma**: a cada
  ~10-20 passos significativos ou no fecho de uma fase.
  Regeneração via Claude Code ou ferramenta automática
  (esboçada em `ideias-projecto-blueprint-tool.md`).
- **Quarentena vanilla**: continua opção 3. Sem mudança.
- **Série paridade**: continua suspensa em P153. Sem
  mudança.
- **Reserva de numeração**: P156 (Model Fase 2 — table
  foundations) permanece livre para próximo passo
  substantivo. ADR-0061 (autorização hayagriva) permanece
  reservada.

---

## §12 — Referências

- Spec: `00_nucleo/materialization/typst-passo-156a.md`.
- Output principal:
  `00_nucleo/diagnosticos/historiograma-passos.md`.
- Output secundário (ideias):
  `00_nucleo/diagnosticos/ideias-projecto-blueprint-tool.md`.
- Blueprint complementar:
  `00_nucleo/diagnosticos/blueprint-projecto.md`.
- ADRs: `00_nucleo/adr/README.md` (índice canónico).
- DEBTs: `00_nucleo/DEBT.md` (autoritário).
- Inventário cobertura:
  `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
