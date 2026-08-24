# Passo 1140.16 — Rebaseline da superfície pública após P1140.1–15

**Estado:** executado  
**Data:** 2026-08-24  
**Continua:** P1140.15  
**Natureza:** diagnóstico; não autoriza correção de código  
**Relatório previsto:** `00_nucleo/diagnosticos/typst-p1140.16-rebaseline-superficie-publica.md`

## 1. Objetivo

Reconstruir, a partir dos binários e fontes vigentes, o inventário diferencial
da superfície pública da linguagem Typst depois das correções P1140.1–15.

O inventário original de P1140 foi produzido antes dessas correções e não pode
mais decidir prioridades sem nova medição. Este passo deve eliminar itens já
fechados, confirmar as divergências ainda observáveis e produzir uma fila
atomizada para P1140.17 e seguintes.

## 2. Regra de escopo

Este passo é estritamente diagnóstico:

- pode reconstruir utilitários de medição em `lab/`;
- pode executar vanilla e cristalino;
- pode atualizar artefatos e relatórios em `00_nucleo/diagnosticos/`;
- não altera L0, L1, L2, L3 ou L4;
- não implementa bindings, membros, símbolos nem metadados;
- não escolhe uma correção antes de obter a nova matriz.

Se o instrumento estiver defeituoso, corrigir somente o instrumento em `lab/`
e registrar a causa. Uma mudança necessária fora de `lab/` interrompe o passo.

## 3. Baseline e proveniência inicial

Estado observado ao escrever este passo:

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitado;
- hora: `2026-08-24T13:12:04-03:00`;
- `git diff HEAD --stat`: `73 files changed, 548 insertions(+), 486 deletions(-)`;
- vanilla ratificado: upstream/main `a51e02804`;
- binário vanilla: `/usr/local/bin/typst` e, após confirmação, o build em
  `lab/typst-original/target/release/typst`;
- binário cristalino: rebuild do workspace vigente, nunca um artefato antigo
  identificado apenas por `--version`.

Antes de cada número decisório, registrar novamente HEAD, hora e
`git diff HEAD --stat`. Se o estado mudar durante o passo, separar as medições
por estado em vez de combiná-las.

## 4. Fontes e instrumentos

Reutilizar, após auditoria:

- `lab/surface-inventory/src/main.rs`;
- `lab/surface-inventory/merge.py`;
- `lab/surface-inventory/run_probes.py`;
- `lab/surface-inventory/probes.json`;
- extrator vanilla `p1140-inventory` em `lab/typst-original`;
- `00_nucleo/diagnosticos/superficie-linguagem-p1140.json` apenas como
  baseline histórica, nunca como entrada que force o resultado novo.

Confirmar que os instrumentos distinguem:

1. binding global;
2. membro público;
3. kind público;
4. função presente sem metadados verificáveis;
5. símbolo/alias;
6. binding extra no cristalino;
7. feature não habilitada por padrão.

## 5. Preparação dos binários

1. Registrar proveniência completa.
2. Confirmar que `/usr/local/bin/typst` corresponde ao vanilla ratificado.
3. Reconstruir o cristalino a partir da working tree vigente.
4. Não usar a string `--version` isoladamente como prova de proveniência.
5. Executar uma sentinela conhecida em ambos os binários antes do inventário.

Se os dois binários vanilla disponíveis divergirem na sentinela, parar e
diagnosticar a referência antes de produzir contagens.

## 6. Reconstrução do inventário

Gerar artefatos temporários independentes para vanilla e cristalino. O fluxo
de referência é:

```sh
cd lab/typst-original
cargo run --release -p p1140-inventory -- /tmp/p1140.16-vanilla.json

cd ../..
cargo run --offline --release \
  --manifest-path lab/surface-inventory/Cargo.toml -- \
  /tmp/p1140.16-crystalline.json /tmp/p1140.16-vanilla.json

python3 lab/surface-inventory/merge.py \
  /tmp/p1140.16-vanilla.json \
  /tmp/p1140.16-crystalline.json \
  00_nucleo/diagnosticos/superficie-linguagem-p1140.16.json \
  --generated-at <timestamp-UTC-da-execução>
```

Adaptar os comandos somente se a auditoria do instrumento provar necessidade;
registrar qualquer adaptação no relatório.

## 7. Probes observáveis

Executar novamente os probes públicos por linguagem, incluindo controles
positivos e todos os casos anteriormente divergentes. No mínimo revalidar:

- globais `html`, `linebreak`, `page`, `parbreak` e `path`;
- kinds de `math.equation` e `math.sqrt`;
- amostra estratificada de coleções, strings, datetime, inteiros, counter e
  content;
- controles conhecidos como `calc.abs`, `math.sum` e `sym.arrow`;
- bindings corrigidos em P1140.1–15, para provar que não reapareceram.

Usar observáveis de linguagem como `repr(type(path))`. Não classificar bytes,
estrutura Rust ou igualdade mecânica como paridade.

Salvar a saída bruta em
`00_nucleo/diagnosticos/superficie-linguagem-p1140.16-probes.json`.

## 8. Reprodutibilidade

Repetir merge e probes sem alterar código nem timestamp lógico do artefato.

- o merge deve ser byte-idêntico quando recebe as mesmas entradas e timestamp;
- resultados de probes devem coincidir semanticamente;
- qualquer instabilidade deve ser investigada, não arredondada ou ocultada;
- preservar comandos, exit codes, stdout e stderr relevantes.

## 9. Classificação da fila

Classificar cada divergência confirmada por:

1. observável da linguagem afetado;
2. classe (`MISSING_BINDING`, `MISSING_MEMBER`, `WRONG_KIND`, símbolo/alias,
   metadado não verificado ou extra cristalino);
3. módulo e Prompt L0 dono provável;
4. necessidade ou não de gate ADR-0127;
5. dependências entre correções;
6. tamanho da família, sem usar quantidade bruta como prioridade automática;
7. teste mínimo capaz de produzir RED.

Itens feature-gated, sobretudo `html`, ficam numa fila própria. Não misturar
ausência default legítima com ausência de binding não feature-gated.

## 10. Regra para escolher P1140.17

Escolher a primeira correção somente depois da classificação. A recomendação
deve favorecer a menor unidade dona que:

- tenha divergência reproduzível por linguagem;
- possua L0 identificável;
- possa ser fechada com teste RED→GREEN isolado;
- não dependa de uma decisão arquitetural ainda ausente;
- não combine famílias independentes apenas por proximidade nominal.

Se o primeiro candidato alterar contrato público, P1140.17 deve começar pelo
L0 e pelo gate ADR-0127. O P1140.16 não antecipa essa confirmação.

## 11. Aceitação

O passo fecha quando:

1. os dois inventários foram regenerados a partir das referências corretas;
2. a matriz nova não reutiliza contagens históricas como se fossem atuais;
3. probes conhecidos e controles positivos foram repetidos;
4. merge e classificação são reproduzíveis;
5. cada número decisório possui proveniência completa;
6. artefatos JSON e relatório estão em `diagnosticos/`;
7. nenhuma camada L0–L4 foi alterada pelo passo;
8. existe recomendação concreta e atomizada para P1140.17;
9. `crystalline-lint .` continua com exit 0.

## 12. Relatório

Escrever
`00_nucleo/diagnosticos/typst-p1140.16-rebaseline-superficie-publica.md`
contendo:

- proveniência dos binários, fontes e working tree;
- comandos exatos;
- comparação histórica P1140 versus P1140.16;
- contagens novas por classe;
- lista explícita de itens fechados desde o inventário anterior;
- divergências confirmadas e refutadas;
- limitações do instrumento;
- fila atomizada com L0 dono e gate provável;
- recomendação justificada do P1140.17.

## 13. Fora de escopo

- implementar qualquer item da fila;
- adicionar metadados de assinatura a `Func`;
- habilitar HTML por padrão;
- importar tabelas de símbolos em massa;
- usar `tekt-cargo-dsm` como oráculo de bindings da linguagem;
- medir paridade por bytes PDF ou estrutura interna Rust.

## 14. Resultado da execução

Executado em 2026-08-24 sem alterações em L0–L4 atribuíveis ao passo. Foram
regenerados os dois inventários, ampliados e repetidos os probes e confirmada
a reprodutibilidade byte a byte.

A rebaseline eliminou `WRONG_KIND`, confirmou quatro bindings globais
ausentes — um deles feature-gated — e escolheu `parbreak` como unidade mínima
para P1140.17. O relatório previsto contém números, proveniência e fila.
