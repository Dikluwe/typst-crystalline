# Passo 156A — Historiograma dos passos do projecto via Claude Code

**Série**: 156A (passo **L0-puro / administrativo**;
documentação histórica do processo de materialização ao
longo dos 155 passos via LLM externa lendo todos os
relatórios em ordem).
**Precondição**: Passo 155 encerrado; ADR-0060
IMPLEMENTADO; Fase 1 Model fechada; 1145 tests; 60 ADRs;
13 DEBTs abertos; 155 relatórios de passo em
`00_nucleo/materialization/`; blueprint manual existente
(snapshot 2026-04-25, mantém-se em paralelo).

**Numeração**: 156A. Sufixo `A` = diagnóstico-primeiro
aplicado a documentação do processo histórico do próprio
projecto. **Não bloqueia P156** (Model Fase 2 — table
foundations); são números diferentes (156A vs 156).
**Sétima aplicação** do padrão diagnóstico-primeiro
(131A/132A/140A/148/154A/156A; com a particularidade que
156A diagnostica o **histórico do projecto** em vez de
**features**).

**Mudança de prioridade vs decisão original P156A**:
escopo deslocado de "blueprint canónica = snapshot do
estado actual" para "historiograma = trajectória dos
processos de materialização". Razão: snapshot já existe
(blueprint manual); o trabalho de maior valor agora é
extrair **padrões empíricos** dos 155 passos para informar
decisões futuras.

**Natureza**: passo **L0-puro / administrativo**. **Zero
código**. **Zero testes**. **Zero ADRs criadas**. **Zero
DEBTs criados**. Único output: documento markdown gerado
por leitura directa dos 155 relatórios + relatório do
passo + ideias projecto futuro.

**Particularidade**: este passo é **executado por LLM
externa** (Claude Code) que lê os relatórios. Diferença
material face aos passos anteriores: trabalho **delegado**,
não interactivo. Spec deste passo é o **guião** que Claude
Code segue.

**ADRs aplicáveis**:
- **ADR-0034** (diagnóstico obrigatório) — espírito
  cumprido: historiograma é diagnóstico macro do método.
- **ADR-0036** (atomização) — historiograma evidencia se
  atomização foi cumprida ao longo dos 155 passos.
- **ADR-0037** (coesão por domínio) — análoga.

---

## Contexto

A blueprint manual produzida em sessão anterior é um
**snapshot estático** do estado actual: onde estamos,
estrutura arquitectural, dependências entre features,
opções para próximo passo. Útil mas limitada — não responde
à pergunta:

> **Como chegámos aqui? Que padrões emergiram? Quais foram
> as reformulações? Onde o método produziu retorno alto e
> onde foi caro?**

Esta pergunta é metodológica e tem valor decisório
material para passos futuros. Ex:

- Se diagnóstico-primeiro produziu sub-trabalho descoberto
  na materialização em 6 de 6 aplicações, é regra
  empírica.
- Se passos com sufixo `A` foram seguidos de passos `B`
  em ≤2 iterações em todos os casos, padrão é
  consistente.
- Se DEBTs abertos durante materialização foram fechados
  em média X passos depois, dá estimativa para
  planeamento.

A informação está toda em
`00_nucleo/materialization/typst-passo-NNN-relatorio.md`
para N=1..155. Cada relatório segue convenção (sumário +
sub-passos + verificação + critério + relatório). Parsing
+ síntese produz historiograma estruturado.

**P156A delega esta análise a Claude Code** lendo os 155
ficheiros em ordem cronológica. Output material: documento
em `00_nucleo/diagnosticos/historiograma-passos.md` com
linha temporal + padrões agregados + análise.

---

## Objectivo

Ao fim do passo:

1. **Historiograma canónico** em
   `00_nucleo/diagnosticos/historiograma-passos.md`
   com 4 secções:

   1. **Linha temporal completa** — 1 entrada por passo
      P1..P155 com:
      - Número e data (se inferível dos relatórios).
      - Título / objectivo declarado.
      - Tipo classificado (substantivo / administrativo /
        diagnóstico / arqueológico / fecho-DEBT /
        investigação / refino).
      - Escopo declarado / executado (XS/S/M/M+/L/XL).
      - Output (ADRs criadas / revogadas / actualizadas;
        DEBTs abertos / fechados; código L1/L2/L3/L4
        tocado; testes Δ).
      - Padrão aplicado.
      - Referências cruzadas (passo anterior obrigatório,
        passo de fecho associado, etc.).
      - Reformulação? (se P155 era P153 antes de
        reformulações, registar).

   2. **Padrões agregados** — extracção dos padrões que
      emergiram, com estatísticas:
      - **Diagnóstico-primeiro** (sufixo `A` →
        materialização `B`): lista, contagem, tempo médio
        entre `A` e `B`, taxa de descoberta de
        sub-trabalho.
      - **Arqueológico**: passos que faziam arqueologia de
        decisões existentes (P149 é o caso paradigmático).
      - **Substantivo escopo S/M/L/XL**: distribuição.
      - **Administrativo / refino**: passos sem código,
        com peso documental.
      - **Fecho-DEBT**: passos cujo critério principal era
        encerrar DEBT específica.
      - **Investigação**: passos que descobriram obstáculo
        em vez de materializar (P151 é o caso
        paradigmático).
      - **Tudo-num-passo** vs **diagnóstico-primeiro**:
        comparação de outcomes.
      Cada padrão lista os passos que o aplicaram.

   3. **Análise**:
      - **Reformulações** identificadas (e.g. série
        paridade reformulou 7 vezes; lista de reformulações
        com causa).
      - **Passos que recuaram** ou **mudaram de prioridade**
        a meio (e.g. P153 → P154A mudou de P2 para Model;
        P155 reformulou markup quote como Content::Text).
      - **Dependências entre passos** detectadas (sequências
        obrigatórias: P140A → P140B; P148 → P149 → P150;
        P154A → P154B → P155).
      - **DEBTs**: ciclo de vida — quando abriram, quando
        fecharam, distância média.
      - **ADRs**: ciclo de vida análogo — PROPOSTO →
        EM VIGOR → IMPLEMENTADO; tempo médio em cada
        status; quantas saltam etapas.
      - **Antipadrões observados**: cenários onde método
        produziu fricção (e.g. tentativa de
        tudo-num-passo que falhou e exigiu split).
      - **Saúde do projecto** ao longo do tempo: tests
        cumulativos; ADRs total; DEBTs abertos vs
        fechados.

   4. **Conclusões metodológicas**:
      - Que padrões funcionaram e por que.
      - Que padrões falharam ou foram caros.
      - Recomendações para passos futuros baseadas em
        evidência empírica (não em intuição).
      - Limitações do método identificadas.

2. **Documento de ideias para projecto futuro** em
   `00_nucleo/diagnosticos/ideias-projecto-blueprint-tool.md`
   (separado; ideias-only):
   - Ferramenta automática de geração de historiograma
     sem LLM (parsing dos relatórios estruturados).
   - Análise viabilidade similar à anterior: ~70%
     parseável; ~25% parcial; ~5% exige humano/LLM.
   - Esboço arquitectural.
   - Convenções projecto cristalino que tornam parsing
     viável.
   - Casos de uso outros projectos.
   - **Não compromete implementação**.

3. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-156a-relatorio.md`.

4. **Sem actualização** de:
   - Inventário 148.
   - DEBT.md.
   - README ADRs.
   - Blueprint manual existente (mantém-se em paralelo
     como snapshot estático complementar).

Este passo **não**:

- Toca código em L1/L2/L3/L4.
- Toca testes.
- Cria ADRs ou DEBTs.
- Materializa Model Fase 2 ou outras features.
- Implementa ferramenta automática (ideias-only no
  documento separado).
- Modifica relatórios históricos (são imutáveis;
  historiograma só os lê).
- Substitui blueprint snapshot existente.

---

## Decisões já tomadas

1. **Geração via Claude Code** — não programa sem LLM
   (separado).
2. **3 ficheiros entregues**: historiograma, ideias,
   relatório do passo.
3. **Localização** `00_nucleo/diagnosticos/`.
4. **Estrutura 4 secções** (linha temporal + padrões +
   análise + conclusões metodológicas).
5. **Escopo 155 passos completos** (não filtragem
   selectiva).
6. **Documento separado da blueprint** (não substitui).
7. **Relatórios são imutáveis** — só leitura.
8. **Sem alteração de artefactos** excepto criar 3 ficheiros
   novos.

## Decisões diferidas (resolvidas neste passo)

9. **Volume estimado do historiograma**: 155 entradas × ~10
   linhas + agregados + análise = **~1800-2500 linhas**.
   Se exceder 3000, considerar dividir em sub-ficheiros
   (`historiograma-linha-temporal.md` +
   `historiograma-analise.md`). Decisão default: **um
   ficheiro** até crescer.

10. **Tratamento de passos ausentes ou imprecisos**:
    se algum relatório `typst-passo-NNN-relatorio.md` está
    em falta ou tem informação incompleta, registar como
    **lacuna factual** no historiograma (não inferir).

11. **Tratamento de numeração não-contígua** (e.g. salto
    P135 → P137): registar como facto. Se causa for
    documentada (passo cancelado, reservado), referenciar.

12. **Sufixos especiais** (e.g. `A`/`B`/`A1`): preservar
    na entrada da linha temporal; agregar em padrões.

13. **Datas**: extrair de relatórios. Se ausentes, registar
    "data desconhecida". Não inferir do filesystem
    timestamp (não-canónico).

14. **Classificação de tipo**: 7 categorias propostas
    (substantivo / administrativo / diagnóstico /
    arqueológico / fecho-DEBT / investigação / refino).
    Se um passo se enquadra em múltiplas, escolher
    dominante; secundárias listadas.

15. **Visualizações**: além de tabelas markdown, considerar
    Mermaid timeline / gantt / pie chart de distribuição
    por tipo. Decisão default: **adicionar Mermaid quando
    a informação tem estrutura visual clara** (e.g. ciclo
    de vida ADR/DEBT). Texto puro suficiente para resto.

16. **Estatísticas agregadas**: medianas e médias para
    distância entre passos relacionados. Confiável apenas
    se N ≥ 3 instâncias.

17. **Conclusões metodológicas — quão prescritivas**:
    descritivas com qualificadores ("em 6 de 6 aplicações
    do padrão A, descobriu-se sub-trabalho") em vez de
    prescritivas absolutas ("padrão A produz sub-trabalho
    sempre"). Aceita evidência fraca como evidência fraca.

---

## Escopo

**Dentro**:

- Leitura sequencial de
  `00_nucleo/materialization/typst-passo-*-relatorio.md`
  para N=1..155 (cronológica).
- Leitura de `00_nucleo/DEBT.md`, `00_nucleo/adr/README.md`
  e ADRs individuais para cross-reference.
- Leitura do inventário 148 e diagnósticos relacionados
  para contexto.
- Geração do historiograma em
  `00_nucleo/diagnosticos/historiograma-passos.md`.
- Geração de ideias projecto separado em
  `00_nucleo/diagnosticos/ideias-projecto-blueprint-tool.md`.
- Geração do relatório do passo.
- Mermaid + ASCII para visualizações que beneficiam.

**Fora**:

- Modificação de qualquer ficheiro `.rs`, `.toml`, ou
  outro código.
- Modificação de testes.
- Implementação da ferramenta automática.
- Criação de ADRs ou DEBTs.
- Modificação de inventário 148, DEBT.md, README ADRs,
  blueprint snapshot.
- Modificação de relatórios históricos (imutáveis).
- Materialização de Model Fase 2 ou outras features.
- Trabalho em `lab/parity/`.
- Inferência de informação ausente — só leitura factual.

---

## Sub-passos

### 156A.1 — Inventário factual dos relatórios

**A.1.1 — Listagem cronológica**:

```bash
ls 00_nucleo/materialization/typst-passo-*-relatorio.md \
  | sort -V \
  | tee /tmp/relatorios-list.txt
wc -l /tmp/relatorios-list.txt
```

Confirmar contagem esperada (155). Identificar saltos ou
sufixos não-numéricos.

**A.1.2 — Extracção de cabeçalhos**:

Para cada relatório, extrair:
- Título (primeira linha `# ...`).
- Data (procurar padrão `**Data**:` ou similar).
- Tipo declarado (procurar `**Natureza**:` ou
  `**Tipo**:`).
- Sumário (primeira secção / parágrafo).

Produzir tabela inicial bruta de 155 linhas.

**A.1.3 — Extracção de outputs**:

Para cada relatório, extrair de secções típicas:
- ADRs criadas / actualizadas: regex sobre menções a
  `ADR-NNNN`.
- DEBTs abertos / fechados: regex sobre `DEBT-NN`.
- Tests count: regex sobre `\d+ passed`.
- Ficheiros tocados: secções com diffs.

**A.1.4 — Cross-reference**:

Para cada ADR e DEBT mencionada:
- Passo onde foi criada/aberta.
- Passo onde foi actualizada (se aplicável).
- Passo onde foi fechada/transitou status.

Produzir tabela ADR ciclo-de-vida + DEBT ciclo-de-vida.

### 156A.2 — Classificação dos 155 passos

**A.2.1 — Aplicar 7 categorias**:

Para cada passo, classificar em:
- **substantivo** — código L1+ tocado; feature
  materializada.
- **administrativo** — apenas documentação; sem código.
- **diagnóstico** — sufixo `A`; inventário antes de
  materialização.
- **arqueológico** — investigação de decisões existentes
  (P149 paradigmático).
- **fecho-DEBT** — critério principal era encerrar
  DEBT.
- **investigação** — descoberta de obstáculo (P151
  paradigmático).
- **refino** — actualização de plano de DEBT existente
  (P152 paradigmático).

**A.2.2 — Casos múltiplos**:

Se um passo se enquadra em múltiplas categorias, escolher
**dominante** com base em peso de output. Secundárias
listadas em parênteses.

**A.2.3 — Sufixos especiais**:

`A` → diagnóstico (com B subsequente esperado).
`B`, `B+` → materialização pós-A.
Numeração nua → categoria por outros critérios.

### 156A.3 — Construir Linha Temporal (§1 do
historiograma)

Tabela markdown com 155 linhas. Colunas:

| # | Data | Título | Tipo | Escopo | ADRs | DEBTs | Tests Δ | Padrão | Notas |

Formato compacto; cada linha 1-2 linhas físicas.

Adjacente: lista breve dos passos por ordem cronológica
em formato narrativo (e.g. "P1: setup inicial; P2-P10:
fundações L1; P11: primeiro substantivo...").

### 156A.4 — Extrair Padrões Agregados (§2)

Para cada padrão identificado em 156A.2, produzir
sub-secção:

#### §2.1 Diagnóstico-primeiro (sufixo `A`)

Lista exaustiva de passos com sufixo `A` ou padrão
diagnóstico:
- 131A → 131B
- 132A → 132B
- 140A → 140B
- 148 (sem sufixo mas é diagnóstico)
- 154A → 154B
- 156A (este)

Estatísticas:
- Quantos passos `A` foram seguidos de `B` em ≤1 passo: X.
- Em ≤2 passos: Y.
- Quantos descobriram sub-trabalho durante `A`: Z.
- Quantos resultaram em DEBT novo: W.
- Tempo médio entre `A` e `B`: T (se inferível).

Conclusão descritiva: "padrão diagnóstico-primeiro foi
aplicado N vezes; em todas as N descobriu-se
informação que afectou materialização".

#### §2.2 Arqueológico

Passos que investigaram decisões pré-existentes para
formalizar via ADR ou DEBT:
- P149 (Value::Type + Value::Args → ADR-0058 + ADR-0059).
- (outros se identificados).

#### §2.3 Substantivo por escopo

Distribuição:
- XS: lista.
- S: lista.
- M: lista.
- M+: lista.
- L: lista.
- XL: lista.

Histograma simples (texto).

#### §2.4 Administrativo / refino

Passos sem código mas com peso documental:
- P145 (uniformização cabeçalhos ADRs).
- P147 (actualização documentos paridade).
- P148 (inventário cobertura).
- P152 (refino DEBT-54).
- P154A (diagnóstico Model).

#### §2.5 Fecho-DEBT

Passos cujo critério principal era encerrar DEBT:
- P142 (fecho DEBT-1).
- P153 (fecho DEBT-52? confirmar).
- (outros).

Estatísticas:
- DEBTs fechados ao longo do projecto.
- DEBTs abertos vs fechados ao longo do tempo.

#### §2.6 Investigação

Passos que descobriram obstáculo:
- P151 (DEBT-53 → DEBT-54 aberto).
- (outros se identificados).

Característica: zero código entregue.

#### §2.7 Tudo-num-passo vs Diagnóstico-primeiro

Comparação:
- Passos "tudo-num-passo" identificados (e.g. P144 hyphenation).
- Passos "diagnóstico-primeiro" (sufixo `A`).
- Outcome: pausas mid-step? sub-DEBT? reformulação?

### 156A.5 — Análise (§3)

#### §3.1 Reformulações

Lista cronológica de reformulações detectadas:
- Série paridade reformulou 7 vezes (P148→149→150→151→152→
  153→ suspensa).
- Outras detectadas (P155 markup quote → Content::Text).

#### §3.2 Passos que recuaram ou mudaram de prioridade

- P153 → P154A: paridade suspensa, Model atacado.
- (outros).

#### §3.3 Dependências entre passos

Sequências obrigatórias detectadas:
- P140A → P140B (font multi-byte).
- P148 → P149 → P150 (paridade inicial).
- P154A → P154B → P155 (Model Fase 1).

Tabela de dependências.

#### §3.4 Ciclo de vida DEBTs

| DEBT | Aberto P | Fechado P | Distância | Tipo |
|------|----------|-----------|-----------|------|
| DEBT-1 | early | P142 | ~140 | longo |
| ... | | | | |

Estatísticas:
- Distância média.
- Distância máxima.
- DEBTs com lifetime curto (≤5 passos) vs longo (>50).

#### §3.5 Ciclo de vida ADRs

Análogo:
- PROPOSTO → EM VIGOR / IMPLEMENTADO.
- Tempo médio em cada estado.
- Quantas saltam etapas.

#### §3.6 Antipadrões

Cenários de fricção:
- Tentativas de tudo-num-passo que pausaram.
- DEBTs abertos durante materialização que indicam
  pré-condições não satisfeitas.

#### §3.7 Saúde do projecto

Mermaid line chart (texto + bloco):
- Tests cumulativos ao longo dos 155 passos.
- ADRs total ao longo dos 155 passos.
- DEBTs abertos vs fechados ao longo dos 155 passos.

### 156A.6 — Conclusões metodológicas (§4)

Síntese descritiva (não prescritiva absoluta):

1. **Padrões com retorno alto consistente** (com
   evidência).
2. **Padrões caros** (com evidência).
3. **Recomendações empíricas** para passos futuros.
4. **Limitações do método** identificadas.

### 156A.7 — Documento de ideias separado

Produzir
`00_nucleo/diagnosticos/ideias-projecto-blueprint-tool.md`
**adaptado** para o foco historiograma:

- Ferramenta sem LLM que regenera historiograma a partir
  dos relatórios.
- Análise viabilidade (parsing de cabeçalhos, contagens,
  ciclos de vida).
- Esboço arquitectural.
- Convenções projecto que tornam parsing viável.
- Casos de uso outros projectos.

### 156A.8 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-156a-relatorio.md`.

Secções:
1. Sumário executivo.
2. Inventário factual dos 155 relatórios (resultado de
   156A.1).
3. Classificação dos passos (resumo de 156A.2).
4. Historiograma gerado (referência ao ficheiro).
5. Padrões agregados — destaques.
6. Análise — destaques.
7. Conclusões metodológicas.
8. Documento ideias projecto separado (referência).
9. Próximo passo: decisão humana entre Model Fase 2 ou
   outra prioridade.
10. Verificação final.

---

## Verificação

1. ✅ `00_nucleo/diagnosticos/historiograma-passos.md`
   gerado.
2. ✅ Linha temporal cobre 155 passos (ou explica
   ausências).
3. ✅ 7 categorias de tipo aplicadas a todos os passos.
4. ✅ Padrões agregados extraídos com estatísticas
   factuais.
5. ✅ Análise inclui reformulações, mudanças de
   prioridade, dependências, ciclo de vida.
6. ✅ Conclusões metodológicas descritivas (não
   prescritivas absolutas).
7. ✅ `00_nucleo/diagnosticos/ideias-projecto-blueprint-tool.md`
   gerado.
8. ✅ Mermaid usado quando informação tem estrutura
   visual.
9. ✅ ASCII fallback presente.
10. ✅ Nenhum ficheiro tocado em L1/L2/L3/L4.
11. ✅ Nenhuma ADR criada / revogada / revisada.
12. ✅ Nenhum DEBT criado / fechado / actualizado.
13. ✅ Inventário 148 inalterado.
14. ✅ DEBT.md inalterado.
15. ✅ README ADRs inalterado.
16. ✅ Blueprint snapshot inalterado.
17. ✅ Relatórios históricos inalterados.
18. ✅ `cargo test --workspace --lib`: 1145 inalterado.
19. ✅ `crystalline-lint .` zero violations.
20. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Historiograma cobre 155 passos com classificação por
   tipo.
2. Padrões agregados identificados com estatísticas
   factuais (não inferenciais).
3. Análise inclui reformulações + dependências + ciclos
   de vida.
4. Conclusões metodológicas justificadas pela evidência
   compilada.
5. Documento de ideias separado.
6. Sem código tocado.
7. Próximo passo (decisão humana) tem âncora.
8. Relatório escrito.

---

## O que pode sair errado

- **Relatórios históricos com formato inconsistente**:
  esperado. Padronização P145 (cabeçalhos canónicos) é
  recente; relatórios antigos podem ter formatos
  variados. Tolerar; usar regex flexível; registar
  inconsistências como "metadata heterogénea pré-P145".

- **Relatórios em falta**: registar como lacuna factual.
  Não inferir conteúdo.

- **Numeração não-contígua**: registar como facto.
  Identificar causa documentada se possível.

- **Volume excede 3000 linhas**: pausar; consultar
  utilizador. Decisão default: dividir em sub-ficheiros.

- **Mermaid timeline não suportado em alguns viewers**:
  manter ASCII alternativo.

- **Classificação ambígua de tipo**: documentar
  ambiguidade como nota; escolher dominante;
  alternativas em parênteses.

- **Estatísticas baseadas em N pequeno** (e.g. apenas 6
  passos `A`): qualificar conclusões com "em N
  observações"; evitar generalização.

- **Saltos cronológicos longos** entre relatórios:
  podem indicar hiato de trabalho. Registar como facto;
  não interpretar.

- **Cross-reference entre passos produz dependências
  cíclicas** (improvável): documentar; relatar.

- **Conclusões metodológicas tendem para prescritivas**:
  Claude Code deve manter qualificadores explícitos ("em
  N de N aplicações") em vez de absolutos.

- **Tempo de execução**: 155 relatórios é volume
  significativo de leitura. Aceite. Optimização (ex:
  ler apenas cabeçalho + sumário) considerável se
  volume é problema.

- **Anomalias subtis** registadas em texto livre dos
  relatórios podem ser ignoradas se Claude Code não
  identifica padrão. Aceite — projecto separado (sem
  LLM) também terá esta limitação.

---

## Notas operacionais

- **Padrão "diagnóstico-primeiro" cumprido**: 156A faz
  inventário factual dos 155 relatórios antes de gerar
  análise sintetizada. **Sétima aplicação** do padrão na
  história do projecto. Particularidade: aplicado ao
  **histórico do próprio projecto**.

- **Trabalho delegado a LLM externa**: Claude Code lê
  os 155 relatórios e produz documento. Spec deste passo
  é o guião reproducível.

- **Resultado é meta-documento**: historiograma fala
  sobre o método do projecto. Permite calibração de
  passos futuros baseada em evidência empírica.

- **Reserva de numeração P156**: este é P156**A**;
  P156 (Model Fase 2 — table foundations) permanece
  livre.

- **Sem ADR criada**. Convenção análoga a P145 ou P148.
  Se priorização futura justificar formalizar processo
  meta, candidato a ADR específica.

- **Pós-156A**:
  - Historiograma gerado.
  - Ideias documentadas para projecto futuro
    (ferramenta sem LLM análoga, focada em historiograma).
  - Próximo passo: decisão humana (Model Fase 2 ou
    outra prioridade).

- **Coexistência com blueprint snapshot**: dois
  documentos com papéis diferentes:
  - **Blueprint** (`blueprint-projecto.md`): "onde
    estamos". Snapshot estático.
  - **Historiograma** (`historiograma-passos.md`):
    "como chegámos". Trajectória dinâmica.
  
  Ambos canónicos; complementares.

- **Princípio de actualização do historiograma**:
  apenas quando passos significativos fechem (não a cada
  passo). Análogo a blueprint. Regeneração via Claude
  Code (ou ferramenta sem LLM se projecto separado for
  materializado).

- **Quarentena vanilla**: continua opção 3. Sem mudança.

- **Série paridade**: continua suspensa em P153. Sem
  mudança.
