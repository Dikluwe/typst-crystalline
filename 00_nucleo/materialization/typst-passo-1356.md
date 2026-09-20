# Passo 1356 — Rebaseline integral de paridade após o sync `586e1bd43`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Retificação de autoridade e política de continuidade

Existe um único **vanilla** neste passo: o alvo ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`. O pin `a51e02804` é somente um
**snapshot histórico não autoritativo**, útil para atribuir a origem temporal
de um delta. Nunca o chamar de “vanilla antigo”, nem tratar `586e1bd43` como
“vanilla novo”. A comparação normativa de paridade é sempre:

```text
cristalino atual ↔ vanilla ratificado 586e1bd43
```

As palavras `BLOCKED` e “bloqueio” não governam a continuidade do trabalho.
Uma lacuna de observação significa apenas **alegação ainda não demonstrada**:
o resultado medido continua válido no seu escopo e o próximo ensaio deve ser
desenhado e executado pelo assistente. Só há devolução de decisão ao humano
quando faltar intenção de produto, houver interpretações incompatíveis ou for
necessária autoridade externa. Falha de harness, `Unknown` ou cobertura parcial
não são, isoladamente, pedidos para o humano comandar o rumo do experimento.

## Estado e motivação

O cristalino está sendo reescrito sob Tekt enquanto o upstream também muda. O
P1355 ratificou como vanilla o upstream/main
`586e1bd43fae6c9a973218163d3165c53ab8d16d`, árvore
`2847920eeaa98d26a41dc6fb0f9c02334b9d593b`. Entre os pins há 35 commits e
mudanças públicas deliberadas; portanto as 175 rotas divergentes medidas por
P1353 contra o snapshot `a51e02804` são uma fotografia histórica, não o saldo
corrente nem outro vanilla.

A atualização inclui, entre outras coisas, a família pública `format`, formatos
settable, `columns.separator`, correções de `luma`, mudanças de diagnóstico,
metadata HTML e novas opções CLI. O último commit do intervalo refatora a
tipagem interna do layout de fluxo; essa mecânica não recebe crédito de
paridade sem witness de linguagem.

Este passo **mede e classifica**. Não corrige o produto, não incorpora commits
posteriores de `main` e não transforma diferença nova em falha do cristalino
antes de separar mudança de alvo, ausência real e limitação do harness.

## Objetivo

Produzir uma baseline reproduzível da paridade contra `586e1bd43`, cobrindo:

1. inventário público regenerado;
2. matriz funcional bilateral nos quatro perfis;
3. atribuição do delta ao upstream, ao cristalino ou ao comparador;
4. lista priorizada e fechada das rotas ainda divergentes;
5. próximos lotes de implementação, sem implementá-los neste passo.

## Autoridades e regime

Usar o protocolo completo de materialização segregada porque o resultado será
autoridade para os próximos passos.

- **Autor do contrato:** recebe este passo, ADR-0107/0108, o pin ratificado e a
  definição canônica P1353; não lê implementação de harness candidata.
- **Autor do oráculo:** recebe o pin vanilla, o snapshot histórico e observáveis públicos;
  produz casos positivos, negativos e opacos sem ler o runner candidato.
- **Implementador do harness:** recebe contrato e oráculos selados; não altera
  expected, pins ou política de `Unknown`.
- **Adversário:** tenta substituir o vanilla pelo snapshot histórico, aceitar
  SHA errado, usar profile
  assimétrico, alias indireto omitido, kind antigo e `Unknown` promovido.
- **Verificador:** somente lê artefatos selados e saídas; não corrige runner,
  catálogo ou produto.

Registrar executor, entradas legíveis, escritas, contexto herdado, hashes e UTC.
Se o ambiente não atestar o isolamento, usar a formulação “executado sem
atestação de isolamento”.

Budget: até duas revisões focais do harness e uma execução integral final.
Duas revisões consecutivas sem novo poder discriminatório obrigam redesenho;
não autorizam relaxar o contrato.

## Fase A — congelar proveniência e escopo

Antes de qualquer medição:

1. confirmar que `586e1bd43` e sua árvore ainda existem localmente e que o pin
   é ancestral do `upstream/main` observado;
2. **não** mover o pin se `main` tiver avançado outra vez;
3. registrar HEAD, árvore de trabalho (`git diff HEAD --stat`), UTC, versões de
   Rust/Python e ferramentas externas;
4. verificar que os dois binários vanilla são byte-idênticos, reportam
   `typst 0.15.1 (586e1bd4)` e têm identificador administrativo
   [dado administrativo removido];
5. reconstruir o cristalino do HEAD medido e registrar seu identificador administrativo;
6. congelar manifesto, contrato, corpus, ordem de perfis e política de
   `Unknown` antes do runner candidato.

O pin/árvore/receita provam a fonte do vanilla. `--version` é apenas sentinela.

## Fase B — provar o harness antes da matriz

Criar um sucessor versionado do runner canônico P1353. A unidade é uma célula
`(probe, profile)` ordenada por id e pela ordem fixa:

```text
default → html → a11y-extras → html+a11y
```

Classes mínimas:

```text
MATCH_VALUE
MATCH_DIAGNOSTIC
VANILLA_ONLY
CRYSTALLINE_ONLY
DIFFERENT_VALUE
DIFFERENT_DIAGNOSTIC
UNKNOWN
```

O gate discriminatório deve obter `mutation_score = 1.0` contra, no mínimo:

- substituir o pin ratificado pelo snapshot histórico;
- aceitar o SHA do snapshot histórico como identidade do vanilla;
- trocar a ordem de resultados sem recanonicalizar;
- executar features diferentes nos dois lados;
- omitir bindings criados por `prelude_path`;
- manter `html`/`pdf` como module quando o novo vanilla os expõe como function;
- converter timeout, parser incompleto ou source ausente em MATCH;
- contar diferença de bytes renderizados como diferença de linguagem sem
  observável semântico/morfológico.

Positivos devem ser `Preserved`, mutações negativas `Violated` e fixtures
deliberadamente opacas `Unknown`. Repetição e ordem inversa devem produzir o
mesmo payload canônico.

## Fase C — regenerar o inventário público

Executar o enumerador upstream já adaptado à nova API `Library::new(formats)`
e o enumerador cristalino a partir dos binários medidos.

O catálogo é a união de:

- todas as entradas descobertas nos dois produtos;
- seeds explícitos;
- as 4.718 rotas do catálogo P1322/P1353;
- novas superfícies do intervalo upstream, mesmo quando o extrator antigo não
  consegue descobri-las automaticamente.

Cobrir simetricamente os quatro perfis. O enumerador deve reconhecer formatos
registrados fora de `typst-library` e aliases de `prelude_path`, incluindo:

```text
format
format.pdf / pdf
format.html / html
format.svg / svg
format.png / png
format.bundle / asset
```

Guardar catálogos brutos fora do Git e commitar apenas manifesto, resumo,
hashes, contagens e a lista integral de rotas não-MATCH. Um binding vanilla
comprovadamente ausente no cristalino é diferença; incapacidade estrutural do
extrator é `Unknown`.

## Fase D — matriz bilateral e atribuição do delta

Executar cada célula duas vezes e também em ordem inversa. Para atribuir a
mudança de alvo, medir com o mesmo corpus:

```text
cristalino atual ↔ snapshot histórico a51e02804
cristalino atual ↔ vanilla ratificado 586e1bd43
snapshot histórico a51e02804 ↔ vanilla ratificado 586e1bd43
```

O snapshot histórico deve ser reconstruído isoladamente do commit completo com
`TYPST_COMMIT_SHA` explícito; não depender de backup efêmero em `/tmp`.

Classificar cada rota não-MATCH como:

- `RETAINED_GAP`: já divergente e continua divergente;
- `RESOLVED_BY_PRODUCT`: gap registrado no snapshot histórico e fechado pelo cristalino;
- `TARGET_DRIFT`: mudou somente porque o upstream mudou;
- `NEW_CRYSTALLINE_GAP`: superfície nova do alvo ausente/diferente;
- `HARNESS_GAP`: comparador não representa o observável;
- `UNKNOWN`: identidade, ambiente ou observação insuficiente.

Não comparar diretamente “175 antes” com o novo total sem declarar mudança de
catálogo. Reportar interseção, entradas e saídas por path.

## Fase E — sentinelas obrigatórios do intervalo upstream

Além do corpus global, medir no nível da linguagem:

1. `type(format)`, `type(format.pdf)`, aliases `pdf/html/svg/png/asset` e scopes;
2. set rules de formato no topo, precedência documento versus CLI e rejeição
   dentro de containers;
3. `pdf(pages:, standard:, tagged:, pretty:)`, warnings e incompatibilidades;
4. `luma()` sem lightness, hint e alpha inteiro `0..255`;
5. diagnóstico de chave inválida em `array.to-dict`;
6. `columns(separator:)` com LTR, RTL, coluna vazia e conteúdo custom;
7. expansão vertical de lone block com tags e controle com múltiplos filhos;
8. pares CJK `｟｠`, `〘〙`, `〝〟` por posição/quebra observável;
9. um `<meta name="author">` por autor no HTML;
10. CLI `-i`, `watch --no-fullscreen`, `--open=VIEWER`, `--pretty[=bool]`,
    `--pdf-tagged[=bool]` e `--ppi`;
11. fronteiras exatas do formatter de duração;
12. `sys.version == version(0, 15, 1)` — nunca inferir revisão desse valor.

## Fase F — artefatos e veredito

Produzir em `00_nucleo/diagnosticos/`:

- relatório consolidado da limpeza;
- relatório consolidado da limpeza;
- relatório consolidado da limpeza;
- relatório consolidado da limpeza;
- relatório consolidado da limpeza;
- relatório consolidado da limpeza.

Cada número decisório deve trazer HEAD/working tree, UTC, comandos, hashes dos
binários e hashes dos inputs. O relatório deve listar integralmente as rotas
não-MATCH e separar claramente linguagem de mecânica.

## Gates finais

```sh
cargo fmt --all -- --check
cargo build --workspace --release --locked
cargo test --workspace --release --locked --no-fail-fast
crystalline-lint .
git diff --check
```

Os três testes conhecidos de `font-path` podem ser particionados com `TMPDIR`
dedicado, desde que a partição e todos os resultados sejam registrados.

Também exigir:

- zero V5/V15/V26;
- zero `Unknown` não explicado;
- repetição e ordem inversa canonicamente idênticas;
- entradas seladas sem drift;
- nenhuma mudança em L1–L4 neste passo;
- verificador independente aprovando catálogo, matriz e atribuição.

## Critério de fecho

O P1356 produz um saldo reproduzível contra `586e1bd43`, com catálogo novo e
atribuição path a path. O resultado não precisa ser paridade total; precisa
dizer precisamente o que falta e por quê. Cobertura parcial limita a alegação,
mas não paralisa o programa de trabalho: o assistente abre e executa o próximo
ensaio observável sem exigir que o humano administre gates metodológicos.

Depois do fecho, escrever passos de implementação separados por owner L0 e
fronteira semântica. Não misturar no mesmo lote a arquitetura `format`, layout
de colunas, HTML/PDF, CLI e correções pequenas de diagnóstico.
