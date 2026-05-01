# Análise retrospectiva — degradação metodológica do projecto Typst Cristalino

## Contexto

O projecto Typst Cristalino acumulou ao longo de muitas sessões
uma camada de meta-comentário que cresceu sem controlo:
"padrões metodológicos" numerados (N=X), ADRs meta sobre como
escrever passos, "subpadrões emergentes" com "limiares de
formalização", reservas de identificadores, métricas
cumulativas. Esta camada começou útil e tornou-se ruído.

A estrutura técnica do projecto (camadas L0-L4, `crystalline-lint`,
hashes L0, ADRs sobre decisões reais como ADR-0033 paridade
observable ou ADR-0054 graded) parece intacta. O que apodreceu
é a camada de auto-comentário sobre o processo.

Esta análise serve para identificar **quando** e **como** a
degradação começou. O objectivo não é corrigir nada agora —
é produzir mapa quantitativo que sirva de base para decisões
futuras.

---

## O que fazer

Análise retrospectiva de **todos os passos do projecto** desde
P1 até ao último executado. Material a inspeccionar:

- `00_nucleo/materialization/typst-passo-*-relatorio.md`
- `00_nucleo/adr/typst-adr-*.md`
- `00_nucleo/diagnosticos/diagnostico-*.md`

Output: ficheiro markdown único em
`00_nucleo/diagnosticos/analise-degradacao-metodologica.md`
com secções definidas abaixo.

---

## Métricas a recolher

Para cada passo executado, registar:

1. **Identificador do passo** (ex: P42, P156C, P159A).
2. **Tamanho do relatório** em caracteres e linhas.
3. **Número de ADRs mencionadas** no relatório (únicas e total).
4. **Número de "padrões" referidos** (ocorrências de "padrão",
   "subpadrão", "patamar", "limiar").
5. **Número de métricas N=X** (ocorrências do formato `N=\d+`).
6. **Número de menções a "cumulativo/cumulativa"**.
7. **Número de menções a "consolidado/consolida"**.
8. **Número de menções a "cross-domínio/cross-feature"**.
9. **Número de "aplicações" contadas** (ocorrências de
   "aplicação concreta", "aplicação cumulativa", "aplicação
   isolada").
10. **Reservas criadas no passo** (texto que indica reserva de
    identificador futuro: "P15X reservado", "ADR-00XX
    reservada", etc.).

Para cada ADR criada, registar:

1. **ID e ficheiro**.
2. **Data de criação** (do ficheiro ou do conteúdo).
3. **Status actual**.
4. **Categoria**: ADR técnica (decisão sobre código/tipos/
   pipeline) vs ADR meta (decisão sobre como escrever passos,
   formalização de padrões, convenções metodológicas).
   Critério de classificação na secção "Como classificar ADRs"
   abaixo.
5. **Tamanho em linhas**.

---

## Como classificar ADRs

**ADR técnica** = decisão sobre o código, arquitectura, tipos,
pipeline, paridade com vanilla, performance, ou
domínio do projecto. Exemplos típicos:
- "ADR-0033 paridade observable" (decisão sobre semântica).
- "ADR-0054 perfil graded" (decisão sobre subset).
- "ADR-XXXX sobre quarentena vanilla" (decisão sobre estrutura
  do repositório).
- "ADR-XXXX autorização de crate externa" (decisão sobre
  dependências).

**ADR meta** = decisão sobre o **processo de escrever passos**
ou formalização de padrões metodológicos. Exemplos típicos:
- "ADR Smart→Option/default" (formaliza um padrão de tradução
  recorrente).
- "ADR Inventariar primeiro" (formaliza um sub-passo
  obrigatório nos enunciados).
- "ADR sobre granularidade de passos".

Em caso de dúvida, marcar como "limiar" e listar separadamente.

---

## Output esperado

Ficheiro `00_nucleo/diagnosticos/analise-degradacao-metodologica.md`
com 5 secções:

### Secção 1 — Tabela de passos

Tabela com uma linha por passo, ordem cronológica, colunas
correspondentes às métricas 1-10 acima.

### Secção 2 — Tabela de ADRs

Tabela com uma linha por ADR, ordem cronológica, colunas:
ID, data, status, categoria (técnica/meta/limiar), linhas.

### Secção 3 — Curvas de evolução

Para cada métrica 2-9 dos passos, descrever a evolução em prosa:
- Valor médio nos primeiros 20 passos.
- Valor médio nos últimos 20 passos.
- Identificação do passo concreto onde cada métrica começou a
  crescer de forma sustentada (definir como: 3 passos
  consecutivos com valor superior à mediana histórica até
  esse ponto multiplicada por 2).

Sem inventar interpretações elaboradas. Só números e o passo
onde o valor mudou.

### Secção 4 — Cronologia de ADRs meta

Lista cronológica de **todas as ADRs meta** identificadas (não
técnicas), com:
- ID e título.
- Passo em que foi proposta (se identificável).
- Passo em que foi criada.
- Frequência de menção em passos posteriores.

Esta secção identifica explicitamente quando começou a
proliferação de ADRs meta.

### Secção 5 — Interpretação

Texto curto (máximo 2 páginas) que responda às perguntas:

1. **Em qual passo aparece a primeira ADR meta?** Identificar
   o passo concreto.
2. **A partir de qual passo as métricas inflacionadas crescem
   consistentemente?** Identificar o passo concreto.
3. **Há ponto de viragem único, ou degradação gradual?**
   Resposta baseada nos números.
4. **Qual a relação entre criação de ADRs meta e crescimento
   das métricas?** Resposta baseada nos números (correlação
   temporal observada, não inferida).

A interpretação deve ser **literal e descritiva**. Não inventar
narrativas causais. Não recomendar acções. Não inflar com
linguagem como "patamar", "limiar", "consolidação", "deriva
arquitectural". Apenas descrever o que os números mostram.

---

## Restrições importantes

- **Não escrever nova ADR meta** sobre esta análise.
- **Não criar reservas de identificadores** para acções
  futuras.
- **Não propor sub-passos seguintes** ("após esta análise,
  P161 deveria...").
- **Não inflar a interpretação** com formalismos numerados.
- **Não classificar a análise como passo PNNN** — é trabalho
  retrospectivo, não passo de migração.

A análise é instrumento. Qualquer decisão sobre o que fazer
com os resultados é decisão humana posterior.

---

## Estimativa de esforço

O projecto tem provavelmente entre 150 e 200 passos. Inspecção
de cada relatório individualmente é viável com scripts simples
(grep + awk + contagens). Não é necessário executar tests nem
build. Trabalho puramente de leitura e contagem.

Se algum relatório está em formato muito divergente do padrão
ou faltam ficheiros para passos referenciados em outros lugares,
documentar a lacuna na secção 5 sem tentar reconstruir.

---

## Critério de conclusão

- Ficheiro `analise-degradacao-metodologica.md` produzido.
- 5 secções presentes.
- Tabelas das secções 1 e 2 completas (uma linha por passo /
  ADR identificada).
- Secção 5 responde às 4 perguntas com referências numéricas
  às tabelas das secções 1, 2, 3, 4.
- Sem código novo escrito.
- Sem ADR nova criada.
- Sem reservas novas estabelecidas.
