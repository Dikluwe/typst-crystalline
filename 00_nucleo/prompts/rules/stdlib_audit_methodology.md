# Prompt L0 — `rules/stdlib_audit_methodology` — Metodologia de varredura sistemática da stdlib

Hash do Código: 0683fad7

**Camada**: L1 / processo
**Criado em**: 2026-07-15 (Passo 765)
**Arquivos gerados**: nenhum ficheiro de código; documentos de diagnóstico em `00_nucleo/diagnosticos/achados-stdlib-<lote>.md`
**ADRs referência**: ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir)

---

## Contexto

Vários bugs graves da linha `cetz` (P678–P762) — short-circuit de `and`/`or`, `measure()` sempre `0pt`, desestruturação quebrada — só foram encontrados por acidente. A metodologia de P663/P664 (auditar divergências de linguagem confirmando directamente contra o vanilla) nunca foi aplicada de forma sistemática ao resto da stdlib. Este L0 define o processo de varredura, o seu âmbito e o formato de registo, sem ainda executar a varredura completa.

## Objetivo

Estabelecer uma metodologia reprodutível para encontrar divergências de linguagem entre o cristalino e o vanilla CLI 0.15.0 no espaço da stdlib, priorizando os pontos onde já existem indícios de lacuna.

## Decisões de processo (medidas na amostra de P765)

### 1. Âmbito

- **Âmbito inicial**: namespaces / elementos já marcados como ausentes ou parciais no Inventário 148, e os achados concretos da lente de 2026-07-15.
- **Justificação**: a amostra mostrou que verificar dois elementos isolados (`#title()` e variantes/modificadores de `symbol`) levou poucos minutos cada e revelou divergências reais. Uma varredura de toda a stdlib seria demasiado grande para um único passo; o trabalho será dividido em lotes.
- **Lote 0 (próximo)**: `#title()` e `symbol` (construtor `symbol(...)`, modificadores via field access, `repr()` de variantes).

### 2. Critério de comparação

- **Resultado do documento**: compilar o mesmo documento mínimo no vanilla e no cristalino; comparar se ambos aceitam/rejeitam e, quando aceitam, se produzem o mesmo resultado visível/morfológico.
- **Texto de mensagem de erro**: quando o observável é uma mensagem de erro (ex.: pacote não encontrado, função desconhecida), o texto é um observável legítimo de paridade (ADR-0107/ADR-0108; CLAUDE.md).
- **Não comparar**: estrutura interna de dados, bytes exactos do PDF, passos do algoritmo (mecânica-não-língua, ADR-0107).

### 3. Formato de registo de achados

- Cada lote produz um ficheiro novo: `00_nucleo/diagnosticos/achados-stdlib-<lote>.md`.
- Não se reaproveita `achados-adiados-cetz.md`, que é específico da linha `cetz`.
- Cada achado é classificado numa das categorias:
  - **Bug real de linguagem**: divergência que afecta documentos Typst válidos.
  - **Diferença aceitável**: mecânica-não-língua (ADR-0107) ou scope-out conhecido com ADR.
  - **Scope-out conhecido**: funcionalidade deliberadamente não implementada (registar referência).

### 4. Geração de casos de teste

- **Elementos isolados**: casos manuais, um por elemento (ex.: `#title("X")`, `#repr(sym.arrow.r.filled)`).
- **Argumentos nomeados de funções nativas**: reutilizar a metodologia semi-automática de P664 (listar argumentos, gerar `#set <regra>(<arg>: ...)` ou chamada directa, comparar aceitação).
- **Mensagens de erro**: casos manuais forçando erros conhecidos (ex.: pacote inexistente, ficheiro inexistente).

## Restrições estruturais

- Este L0 não legitima alterações de código. Cada achado que exija correcção vira o seu próprio passo L0 + implementação, seguindo o protocolo de nucleação.
- A varredura não depende da estrutura interna do cristalino; compila documentos contra os binários vanilla e cristalino.
- As medições incluem sempre proveniência: hash do commit, data/hora, comando exacto, versão do vanilla.

## Critérios de verificação do processo

```
Dado um elemento candidato (ex.: #title)
Quando se escreve um documento mínimo e se compila nos dois compiladores
Então o resultado é registado com classificação e proveniência

Dado um achado classificado como bug real
Então é aberto um passo de correcção separado com L0 próprio

Dado um lote completo
Então existe um ficheiro 00_nucleo/diagnosticos/achados-stdlib-<lote>.md
E nenhum achado fica só na memória do chat
```
