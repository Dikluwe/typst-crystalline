# Metodologia histórica — varredura sistemática da stdlib

**Path anterior:** `00_nucleo/prompts/compiler/stdlib_audit_methodology.md`  
**Criado em:** 2026-07-15 (Passo 765)  
**Reclassificado em:** 2026-08-25 (P1183)  
**Artefatos produzidos:** diagnósticos `00_nucleo/diagnosticos/achados-stdlib-<lote>.md`

Este documento registra processo histórico, não é Prompt L0, não possui consumer
produtivo e não legitima código. ADR-0107, ADR-0108 e as regras arquiteturais vigentes
prevalecem em caso de divergência.

## Contexto

Vários bugs graves da linha `cetz` (P678–P762) — short-circuit de `and`/`or`,
`measure()` sempre `0pt`, desestruturação quebrada — só foram encontrados por
acidente. A metodologia de P663/P664 (auditar divergências de linguagem confirmando
diretamente contra o vanilla) não tinha sido aplicada sistematicamente ao resto da
stdlib. P765 registrou este processo de varredura, seu âmbito e o formato de registro,
sem executar a varredura completa.

## Objetivo histórico

Estabelecer uma metodologia reprodutível para encontrar divergências de linguagem
entre o cristalino e o vanilla então usado no espaço da stdlib, priorizando pontos
com indícios de lacuna. A referência ratificada vigente é o hash pinado `a51e02804`;
menções históricas a versões não substituem essa proveniência.

## Decisões de processo medidas na amostra de P765

### 1. Âmbito

- Âmbito inicial: namespaces e elementos marcados como ausentes ou parciais no
  Inventário 148, além dos achados concretos da lente de 2026-07-15.
- A amostra de `#title()` e variantes/modificadores de `symbol` revelou divergências
  reais em poucos minutos; por isso a varredura foi dividida em lotes.
- O lote 0 previsto era `#title()` e `symbol` (constructor, modificadores por field
  access e `repr()` de variantes).

### 2. Critério de comparação

- Compilar o mesmo documento mínimo nos dois compiladores e comparar
  aceitação/rejeição e resultado visível ou morfológico.
- Quando o observável for uma mensagem de erro, seu texto é evidência legítima de
  paridade.
- Não comparar estrutura interna, bytes exatos do PDF ou passos do algoritmo.

### 3. Formato de registro dos achados

- Cada lote produz `00_nucleo/diagnosticos/achados-stdlib-<lote>.md`.
- `achados-adiados-cetz.md` permanece específico da linha `cetz`.
- Cada achado é classificado como bug real de linguagem, diferença aceitável ou
  scope-out conhecido, sempre com a referência aplicável.

### 4. Geração de casos de teste

- Elementos isolados: casos manuais, um por elemento.
- Argumentos nomeados: metodologia semi-automática de P664 para listar argumentos,
  gerar chamadas e comparar aceitação.
- Mensagens de erro: casos manuais que forçam erros conhecidos.

## Restrições estruturais históricas

- Esta metodologia não legitima alterações de código. Cada correção exige seu owner
  L0 e segue o protocolo de nucleação.
- A varredura observa a linguagem pelos binários, sem depender da estrutura interna
  do cristalino.
- As medições registram commit, data/hora, comando exato e proveniência do vanilla.

## Critérios de verificação do processo

```text
Dado um elemento candidato
Quando um documento mínimo é compilado nos dois compiladores
Então o resultado é registrado com classificação e proveniência

Dado um achado classificado como bug real
Então é aberto um passo de correção separado com L0 próprio

Dado um lote completo
Então existe um diagnóstico achados-stdlib-<lote>.md
E nenhum achado fica somente na memória do chat
```
