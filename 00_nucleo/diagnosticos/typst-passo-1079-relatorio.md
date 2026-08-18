# Relatório de Auditoria — Passo 1079: `corpus-docs` Alargado e Pergunta Estratégica (Atomização vs Medição)

**Data**: 2026-08-18  
**Passo**: 1079 — Auditoria e Levantamento de Evidência (Corpus-Docs & Pergunta Estratégica)  
**Gate**: Nenhum (Auditoria puramente factual e consultiva — sem alteração de código ou decisões unilaterais)  
**Status**: CONCLUÍDO (Inventário factual executado, estimativas desagregadas por sub-tópicos, ressalvas metodológicas incorporadas e consultas abertas formuladas para o Dono)

---

## Parte A — Dimensionamento de `corpus-docs` Alargado

### A.1 Inventário Real de `00_nucleo/corpus-docs/math/`
O levantamento direto no repositório identificou **20 arquivos `.typ`** em `00_nucleo/corpus-docs/math/`:
1. `00-index.typ` (visão geral do módulo math e variáveis)
2. `accent.typ` (acentos matemáticos)
3. `attach.typ` (subscritos, sobrescritos e anexos)
4. `binom.typ` (coeficientes binomiais)
5. `cancel.typ` (termos cancelados / riscas)
6. `cases.typ` (sistemas por casos)
7. `class.typ` (classes de operadores e pontuação)
8. `equation.typ` (equações inline vs bloco e numeração)
9. `frac.typ` (frações regulares e inline)
10. `lr.typ` (delimitadores com redimensionamento automático)
11. `mat.typ` (matrizes e vetores multidimensionais)
12. `op.typ` (operadores nomeados e limites)
13. `primes.typ` (símbolos de derivação / primes)
14. `roots.typ` (raízes quadradas e n-ésimas)
15. `sizes.typ` (escalonamento de símbolos)
16. `stretch.typ` (setas e operadores extensíveis)
17. `styles.typ` (fontes e variantes matemáticas)
18. `underover.typ` (chaves superiores e inferiores)
19. `variants.typ` (glifos alternativos e caligráficos)
20. `vec.typ` (vetores coluna)

**Padrão estrutural**: Cada arquivo contém cabeçalho citando a URL da documentação oficial (`https://typst.app/docs/reference/math/...`), citação literal do parágrafo explicativo da funcionalidade e blocos executáveis cobrindo tanto os exemplos ilustrados quanto as regras descritas apenas na prosa.

---

### A.2 Evidência Cruzada dos Domínios Propostos vs Achados Reais

Mapeamento dos achados identificados na sequência recente (Passos 1050 a 1078), conduzidos por **investigação orientada a sintomas**, sem a existência prévia de `corpus-docs` dedicado nesses domínios:

| Domínio Proposto | Achados Reais na Conversa Recente (sem corpus-docs) | Status de Cobertura |
| :--- | :--- | :--- |
| `par` | • **P1057**: `par.spacing` (−6.05pt de offset de entrelinha)<br>• **P1059–P1061**: Margin collapsing entre parágrafos consecutivos | Alta densidade de achados |
| `block` | • **P1050**: Inset de `table`/`grid` (área estrutural, não block puro)<br>• **P1059–P1061**: Margin collapsing bloco ↔ bloco e bloco ↔ parágrafo | Alta densidade de achados |
| `heading` | • **P1063**: Espaçamento `above`/`below` ausente no cabeçalho<br>• **P1073**: Suplementos automáticos de `@f` e `@eq`<br>• **P1074**: Herança de `TextStyle` (itálico desativado forçadamente)<br>• **P1078**: Ortografia padrão de `heading` em português (`Seção` vs `Secção`) | Altíssima densidade de achados |
| `list` / `enum` | • **P1072**: `body_indent` padrão de `list` e `enum` (0pt vs 0.5em) | Média densidade de achados |
| `quote` | • **Nenhum achado registrado** (domínio nunca investigado nesta conversa) | **Ponto cego total** |

---

### A.3 Desagregação por Sub-Tópicos e Estimativa Rigorosa de Esforço

Se os 5 domínios forem mapeados com a mesma granularidade temática e rigor estrutural aplicados ao `math/` (1 arquivo por funcionalidade/propriedade da documentação oficial), a desagregação resulta no seguinte dimensionamento:

1. **`par` (~7 arquivos)**:
   - `00-index.typ` (visão geral e quebras de linha)
   - `leading.typ` (entrelinha padrão e com set rules)
   - `spacing.typ` (espaçamento entre parágrafos vs leading)
   - `justify.typ` (alinhamento justificado vs hífens)
   - `first-line-indent.typ` (indentação de primeira linha e interações com heading)
   - `hanging-indent.typ` (indentação suspensa)
   - `linebreaks.typ` (hifenização e custos de quebra)

2. **`block` (~8 arquivos)**:
   - `00-index.typ` (visão geral de containers em bloco)
   - `spacing.typ` (above/below e margin collapsing com vizinhos)
   - `inset-outset.typ` (margens internas e externas)
   - `radius-stroke-fill.typ` (aparência e bordas de bloco)
   - `width-height.typ` (dimensionamento auto, ratio e comprimento)
   - `breakable.typ` (quebra de página em blocos)
   - `clip.typ` (recorte de conteúdo excedente)
   - `sticky.typ` (comportamento de não-orfanamento)

3. **`heading` (~7 arquivos)**:
   - `00-index.typ` (visão geral de títulos)
   - `levels.typ` (níveis 1 a 6 e escalas de fonte padrão)
   - `numbering.typ` (padrões de numeração e contadores)
   - `supplement.typ` (suplementos por idioma e customizados)
   - `spacing.typ` (above/below padrão e interações com parágrafos)
   - `style-inheritance.typ` (herança de cor, itálico, família de fonte)
   - `outline-interactions.typ` (presença no índice e bookmarking)

4. **`list` / `enum` (~8 arquivos)**:
   - `00-index.typ` (visão geral de listas)
   - `marker.typ` (marcadores pontuais, arrays e funções)
   - `body-indent.typ` (recuo do corpo vs marcador)
   - `tight-spacing.typ` (listas densas vs espaçadas com quebra)
   - `enum-numbering.typ` (formatos de contagem `1.`, `a.`, `i.`)
   - `enum-start.typ` (offset inicial e continuidade)
   - `nesting.typ` (aninhamento multi-nível e marcadores contextuais)
   - `term-list.typ` (listas de termos / descrição e separadores)

5. **`quote` (~4 arquivos)**:
   - `00-index.typ` (citações inline vs em bloco)
   - `attribution.typ` (atribuição de autoria)
   - `quotes-style.typ` (aspas tipográficas contextuais)
   - `block-quote.typ` (recuo lateral e quebra em bloco)

**Total Desagregado**: **~34 arquivos `.typ`** (distribuídos em 5 novos subdiretórios de `00_nucleo/corpus-docs/`).

**Estimativa de Esforço Reavaliada**:
- A criação de cada arquivo exige extração de texto/citação da documentação oficial, elaboração de snippet reproduzível, compilação no Vanilla Typst e compilação no Crystalline com verificação de paridade.
- Baseando-se no ritmo de auditorias anteriores (onde cada lote de 10–14 casos demandou 1 a 2 passos completos com iterações de ajuste), a cobertura completa dos 34 tópicos demandaria entre **5 a 7 passos de trabalho dedicados** (aproximadamente 1 passo por domínio temático).

---

## Parte B — Evidência para a Pergunta Estratégica (Atomização vs Medição)

### B.1 O Dilema Estratégico
A questão consultiva busca avaliar o valor relativo de duas abordagens de trabalho:
- **Abordagem A (Atomização / Fatiamento Estrutural)**: Divisão de módulos volumosos (ex.: grandes hubs monolíticos) em submódulos menores para fins de arquitetura e isolamento de código.
- **Abordagem B (Disciplina de Medição Observável)**: Foco em testes de confronto contra o compilador Vanilla Typst (inspeção de PDF, medição de coordenadas, tipografia, strings e operadores de renderização).

### B.2 Evidência Factual da Trajetória Recente (P1059–P1078)
- Nos últimos ~20 passos executados (P1059 a P1078), **0 passos foram dedicados a fatiamento estrutural puro**.
- **100% dos passos** foram orientados a **medição de paridade**:
  - Resolução de margin collapsing (P1059–1061).
  - Correção de espaçamentos tipográficos e avanços (P1063, P1072).
  - Herança contextual de estilo no layout (P1074).
  - Capturas regex com grupos não-participantes (P1075).
  - Banker's Rounding na interpolação de cores (P1076).
  - Remoção de extensões não-canônicas (`len`) (P1077).
  - Alinhamento de internacionalização ortográfica (P1078).
- **Impacto Real**: A suíte de testes manteve-se 100% aprovada (5.950 testes PASS), com o ganho concreto de identificar e eliminar divergências visuais e semânticas que os testes unitários internos não capturavam por não confrontarem o output oficial do Vanilla.

### B.3 Limitações Metodológicas da Amostra
Conforme delimitado no L0:
1. **Amostra de Conveniência**: A sequência P1059–P1078 foi dedicada a liquidar uma fila pré-existente de achados de medição (catálogo P1031). Não houve iniciativas de fatiamento no mesmo intervalo porque nenhuma pendência imediata o exigia.
2. **Ausência de Comparação Simultânea**: Esta evidência não demonstra que fatiamentos estruturais sejam ineficazes ou desnecessários para manutenção de longo prazo, apenas confirma que a disciplina de medição foi suficiente para destravar paridade funcional nos itens tratados.

---

## Parte C — Consultas Abertas para Decisão do Dono do Projeto

Para instruir os próximos passos de planejamento sem viés pré-estabelecido, submetem-se as seguintes consultas:

1. **Sobre a Estratégia de Cobertura (`corpus-docs`)**:
   - **Opção 1.1 (Foco no Ponto Cego)**: Materializar exclusivamente o `corpus-docs/quote/` (~4 arquivos, ~1 passo), sanando o único domínio sem cobertura, mantendo os demais sob investigação orientada a sintomas.
   - **Opção 1.2 (Expansão Integral)**: Executar a criação sistemática dos 5 domínios (~34 arquivos desagregados, ~5 a 7 passos).
   - **Opção 1.3 (Manutenção Sob Demanda)**: Não abrir novos subdiretórios de `corpus-docs` no momento, priorizando baterias de confronto direto no PDF sobre documentos integrados complexos.

2. **Sobre a Alocação de Esforço (Fatiamento vs Medição)**:
   - **Caminho 2.1**: Priorizar a disciplina de medição empírica e paridade observável (foco direto em output do PDF e sintomas visuais), acionando refatorações e divisões de arquivos apenas quando houver impedimento técnico explícito.
   - **Caminho 2.2**: Retomar ciclos planejados de fatiamento e atomização de módulos grandes para reduzir o débito estrutural antes de prosseguir com novas fases de paridade.
   - **Caminho 2.3**: Alternar ciclicamente entre fases de medição e fases de limpeza estrutural/fatiamento.
