# Passo 1137 — Matriz integral de paridade com o Typst vanilla ratificado

## 0. Natureza deste documento

Este é um **Passo de Execução**, tático e temporário. Não é Prompt L0 e não
legitima alterações em L1–L4.

O objetivo deste passo é iniciar uma matriz reproduzível de paridade entre o
cristalino e a **linguagem/produto Typst**, não apenas comparar PDFs. A matriz
deve medir sintaxe, semântica, morfologia, layout, formatos, CLI e ecossistema.

O alvo exclusivo é o vanilla ratificado:

```text
upstream/main a51e02804
binário: lab/typst-original/target/release/typst ou /usr/local/bin/typst
```

Não usar como alvo uma tag nominal `0.15.0`/`0.15.1`, nem inferir proveniência
pela string `--version`. Confirmar o hash pinado conforme `AGENTS.md`.

---

## 1. Problema que este passo corrige

Os recibos existentes são valiosos, mas não sustentam uma afirmação de
paridade global:

- `lab/parity/reports/latest.md` ainda descreve uma baseline antiga;
- parte do corpus mede apenas introspecção estrutural;
- a varredura recente das secções matemáticas mede sobretudo render PDF;
- inventários percentuais anteriores receberam atualizações ad hoc, não uma
  recontagem integral contra o hash ratificado;
- “compila”, “não dá erro”, “PDF abre” e “pixels coincidem” são observáveis
  diferentes e não podem ser fundidos numa única coluna `pass`.

Consequentemente, este passo **não começa atribuindo uma percentagem**. Primeiro
constrói o denominador, registra a proveniência e mede cada contrato público.

---

## 2. Pergunta de trabalho

> Para cada comportamento público do vanilla ratificado, o cristalino possui
> o mesmo comportamento de linguagem observável; se não, a diferença está
> classificada como ausência, implementação parcial, divergência, defeito do
> harness ou mecânica deliberadamente diferente?

Paridade segue ADR-0107:

- **semântica**: valor, tipo, efeito, diagnóstico e estado observável;
- **sintaxe**: aceitação/rejeição, precedência, parsing e recuperação;
- **morfologia**: forma do conteúdo como objeto da linguagem;
- **render/layout**: geometria visual quando ela é o observável;
- **mecânica interna**: não exige igualdade de Rust, algoritmo ou bytes, salvo
  quando os bytes/protocolo forem a própria interface pública.

---

## 3. Escopo da matriz

A matriz possui sete eixos independentes.

| Eixo | Superfície mínima | Observáveis |
|---|---|---|
| S — Sintaxe | lexer, parser, markup, code, math | aceita/rejeita, AST/morfologia, span e diagnóstico |
| E — Eval/linguagem | valores, operadores, closures, módulos, controle de fluxo | valor, tipo, repr, erro, estado |
| B — Biblioteca | módulos, funções, métodos, elementos, argumentos e defaults | presença, assinatura de língua e resultado |
| I — Introspecção | labels, query, counter, state, context, show/fixpoint | sequência, localização e resultado público |
| L — Layout | texto, math, model, páginas, grids, tabelas, footnotes | dimensões, posições, paginação e morfologia |
| X — Exportação | PDF, SVG, PNG, HTML | render, texto, links, metadados e acessibilidade |
| C — CLI/ecossistema | comandos, opções, exit codes, packages, fontes | stdout/stderr, ficheiros, código de saída e resolução |

Um caso pode pertencer a vários eixos, mas cada observável recebe uma linha
própria para evitar que um match visual esconda divergência semântica.

---

## 4. Unidade canônica da matriz

Criar um registro por caso/observável com, no mínimo:

```text
id
eixo
feature
caso
fonte_typ
observavel
oraculo
cristalino
estado
classe
proveniencia_vanilla
proveniencia_cristalino
comando_reproducao
artefactos
nota
```

Valores fechados de `estado`:

```text
MATCH
DIFF
ABSENT
PARTIAL
ERROR
UNMEASURED
NOT_APPLICABLE
```

Valores fechados de `classe` quando `estado != MATCH`:

```text
LANGUAGE_SEMANTICS
LANGUAGE_SYNTAX
LANGUAGE_MORPHOLOGY
PUBLIC_DIAGNOSTIC
PUBLIC_CLI
PUBLIC_FORMAT
MECHANICS_ALLOWED
HARNESS_DEFECT
BASELINE_DEFECT
UNKNOWN
```

`UNKNOWN` não fecha item. Toda inferência deve dizer o que a refutaria, como
exige ADR-0108.

---

## 5. Artefactos a produzir nesta primeira execução

### 5.1 Manifesto versionado

Criar em `lab/parity/matrix/`:

```text
schema.json
manifest.yaml
README.md
```

- `schema.json`: valida campos e enums acima;
- `manifest.yaml`: lista inicial dos casos, mesmo que `UNMEASURED`;
- `README.md`: comandos de reprodução e política de classificação.

Não guardar resultados gerados como especificação. O manifesto descreve os
casos; resultados e números pertencem a relatórios/artefactos.

### 5.2 Inventário do denominador

Gerar um inventário novo, a partir do código do vanilla ratificado e do
cristalino atual, cobrindo pelo menos:

1. comandos e opções da CLI;
2. `SyntaxKind` e entradas públicas do parser;
3. módulos globais e seus bindings;
4. funções, métodos e elementos expostos;
5. argumentos nomeados e valores por defeito;
6. formatos de saída;
7. capacidades de World: fontes, ficheiros, packages, data/time e inputs.

Salvar o diagnóstico em:

```text
00_nucleo/diagnosticos/paridade-matriz-p1137-inventario.md
```

O inventário deve distinguir:

- “ausente no cristalino”;
- “nome presente, contrato ainda não comparado”;
- “agregado noutra representação cristalina”;
- “não aplicável por ser mecânica interna”.

Não usar LOC como proxy de cobertura.

### 5.3 Relatório baseline

Criar:

```text
00_nucleo/diagnosticos/paridade-matriz-p1137-baseline.md
```

O relatório deve conter:

- hash exato do HEAD;
- `working tree limpa` ou `working tree não commitada` com
  `git diff HEAD --stat`;
- data/hora da medição;
- identidade dos dois binários;
- número de casos por eixo e estado;
- lista integral de `DIFF`, `ABSENT`, `PARTIAL`, `ERROR` e `UNMEASURED`;
- limitações do harness;
- nenhuma percentagem agregada sem denominador explícito.

### 5.4 Runner mínimo

Adaptar ou criar ferramenta somente em `lab/parity/` capaz de:

1. executar o mesmo caso nos dois compiladores;
2. capturar exit code, stdout, stderr e artefactos;
3. normalizar apenas campos comprovadamente voláteis;
4. aplicar comparadores diferentes por observável;
5. emitir resultado estruturado e relatório legível;
6. nunca transformar automaticamente `DIFF` em `MATCH` por tolerância não
   documentada.

Nesta primeira execução, o runner precisa provar pelo menos um caso de cada
eixo S, E, B, I, L, X e C. Não é necessário fechar o corpus inteiro.

---

## 6. Sentinelas iniciais obrigatórias

Incluir casos que garantam que a matriz detecta diferenças conhecidas e não
produz um “zero diffs” falso:

1. `sys.version` — deve detectar vanilla `version(0, 15, 1)` versus estado
   cristalino vigente;
2. CLI `--help` — deve detectar comandos/opções ausentes ou diferentes;
3. formato HTML — classificar capacidade ausente, não erro genérico;
4. PNG e SVG — confrontar capacidade real com o texto público do `--help`;
5. package `@preview/...` — medir resolução, sem rede se fixture local for
   suficiente; caso contrário registrar pré-condição;
6. parágrafo RTL com duas quebras — preservar as duas unidades de parágrafo;
7. uma função/elemento com argumento vanilla ainda scope-out;
8. uma entrada sintática inválida — comparar erro e span;
9. um caso estrutural de `query`/`state`/`counter`;
10. uma secção matemática visual já conhecida como MATCH e uma com resíduo.

Uma sentinela conhecida como divergente só passa quando o runner retorna a
classificação esperada (`DIFF`/`ABSENT`), não quando força igualdade.

---

## 7. Comparadores por observável

Não criar um comparador universal.

| Observável | Estratégia inicial |
|---|---|
| exit code | igualdade exata |
| stdout/repr | igualdade sem normalização semântica |
| diagnóstico | classe, mensagem pública e span; normalizar apenas paths |
| valor eval | DTO tipado, não apenas string formatada |
| estrutura | sequência e propriedades públicas selecionadas |
| geometria | coordenadas em pt com tolerância declarada por campo |
| raster | AE/pixel diff + imagem de diferença; não substitui geometria |
| PDF | observáveis extraídos, não igualdade bruta de bytes |
| SVG/HTML | árvore/atributos públicos normalizados de forma documentada |
| CLI | comandos, opções, defaults, stdout/stderr e exit code |

Cada tolerância deve ter unidade, razão e fonte. É proibido introduzir
tolerância escolhida apenas para fazer um caso passar.

---

## 8. Sequência RED → GREEN da infraestrutura

1. Escrever testes do schema e do runner.
2. Demonstrar RED com:
   - manifesto inválido;
   - sentinela `sys.version`;
   - comando CLI ausente;
   - resultado esperado que não coincide.
3. Implementar apenas a infraestrutura necessária.
4. Demonstrar GREEN dos testes do harness.
5. Rodar a baseline mantendo divergências reais como resultados válidos.

“Runner verde” significa que ele classifica corretamente matches e diffs; não
significa que o cristalino atingiu paridade.

---

## 9. Travas arquiteturais

- Esta primeira execução é de **medição e infraestrutura em `lab/`**.
- Não corrigir L1–L4 enquanto se constrói o denominador.
- Não ler `00_nucleo/context/` nem `00_nucleo/materialization/` sem path
  explicitamente fornecido pelo humano.
- Se a medição revelar correção de produção, abrir item separado.
- Antes desse código, auditar/atualizar o Prompt L0 correspondente.
- Aplicar ADR-0127: parar no gate humano para contrato público, comportamento
  por defeito, fase do pipeline ou quebra de compatibilidade; correções internas
  de paridade seguem fluxo contínuo com L0 primeiro.
- Não importar `lab/` a partir de L1–L4.

---

## 10. Critérios de aceitação do Passo 1137

O passo fecha somente quando:

- [x] a proveniência do vanilla ratificado foi verificada;
- [x] schema, manifesto e README existem em `lab/parity/matrix/`;
- [x] o denominador inicial foi extraído do código atual, sem LOC como proxy;
- [x] há pelo menos um caso executado por eixo S/E/B/I/L/X/C;
- [x] sentinelas conhecidas impedem falso “zero diffs”;
- [x] cada número decisório possui proveniência reproduzível;
- [x] resultados distinguem `MATCH`, `DIFF`, `ABSENT`, `PARTIAL`, `ERROR` e
      `UNMEASURED`;
- [x] o relatório lista integralmente tudo que não é `MATCH`;
- [x] nenhum código de produção foi alterado para fazer a baseline passar;
- [x] testes do runner passam;
- [x] `cargo test --manifest-path lab/parity/Cargo.toml` passa;
- [x] `crystalline-lint .` continua sem violações bloqueantes;
- [x] `git diff --check` passa.

O resultado esperado deste passo não é “paridade completa”. É uma matriz
honesta, reexecutável e suficientemente sensível para orientar os passos de
correção seguintes.

---

## 11. Entrega para o passo seguinte

Ao final, priorizar divergências segundo esta ordem:

1. comportamento público incorreto que altera documentos (`LANGUAGE_*`);
2. contratos presentes mas parciais;
3. capacidades totalmente ausentes;
4. diagnósticos/CLI públicos;
5. formatos e acessibilidade;
6. mecânica permitida — documentar, não “corrigir”.

Cada lote posterior deve citar IDs estáveis da matriz e fechar casos por nova
medição contra o mesmo hash vanilla. Nenhum caso é removido do denominador ao
ser corrigido; muda de estado e preserva histórico.
