# Passo 1414 — materializar `par.line` no fechamento físico de linhas

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Fechar `P1400-ELEM-011 par.line`, a última rota `ABSENT` da matriz
P1409/P1413, sem reduzir linha física a newline textual. O recorte materializa
o namespace público, set rules e numeração por pattern nas linhas realmente
fechadas pelo layout. Ao final, a matriz deve atingir `{MATCH: 6}`.

## Estado e proveniência inicial

- baseline: commit `df196b35beec0c7bf35f797c86ea7f6e7e81358d`, árvore limpa;
- cristalino: `target/release/typst`, identificador administrativo
  [dado administrativo removido];
- vanila ratificado: commit `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário identificador administrativo
  [dado administrativo removido];
- medição congelada às `2026-09-15T20:08:24-03:00`;
- regime: protocolo Tekt completo em rehearsal, executado sem atestação de
  isolamento.

## Medição antes da decisão

A fonte vanila declara `ParLine` como elemento exclusivamente configurável
por set rules e cujo constructor falha com `cannot be constructed manually`.
O marker privado é inserido na composição inline e os números são realizados
depois das quebras físicas, não durante eval.

Sob 512 MiB e uma thread, o vanila produziu:

```text
explicit:    1   alpha / 2   beta
wrap 100pt:  1 alpha beta gamma / 2 delta epsilon zeta / 3 eta theta
page scope:  1 alpha / 2 beta / form-feed / 1 gamma / 2 delta
none:        plain
right:       alpha   1 / beta    2
constructor: exit 1, error: cannot be constructed manually
```

O maior RSS vanila observado foi 31.416 KiB. O cristalino antigo não contém
`par.line`; a rota qualificada falha, e set rules pontuadas emitem warning e
deixam o conteúdo sem número. O wrap cristalino já produz as mesmas três
linhas textuais, portanto a lacuna é configuração/emissão, não line breaking.

## Oráculos congelados

Os seis arquivos em `lab/parity/corpus/p1414/` cobrem constructor proibido,
forma qualificada, explicit linebreak, wrap automático, `numbering: none`,
reset por página e margem direita com alinhamento/clearance. Seus identificador administrativo estão
registrados na medição do passo e não podem ser adaptados ao candidato.

| Fixture | identificador administrativo |
|---|---|
| `par-line-constructor-invalid.typ` | [dado administrativo removido] |
| `par-line-explicit.typ` | [dado administrativo removido] |
| `par-line-none.typ` | [dado administrativo removido] |
| `par-line-page-scope.typ` | [dado administrativo removido] |
| `par-line-right.typ` | [dado administrativo removido] |
| `par-line-wrap.typ` | [dado administrativo removido] |

Política de `Unknown`: timeout, OOM, swap, output ausente ou configuração de
callback/coluna fora deste recorte não contam como preservação. Observável
conhecido divergente é falha.

## Contrato proposto

1. `par` recebe namespace fechado com `line`; `repr(par.line) == "line"`.
   Chamada manual sempre falha e não cria `Content`.
2. `par.line` declara os campos de set `numbering`, `number-align`,
   `number-margin`, `number-clearance` e `numbering-scope` para `where` e
   validação pública, sem instância consultável inventada.
3. `#set par.line` transporta os cinco valores pela StyleChain. Patterns string
   e `none` são materializados neste passo. Alinhamentos são horizontais,
   clearance é `length | auto`, scope é `"document" | "page"`; named alheio
   e tipo inválido são erro.
4. O Cursor emite no ponto único em que uma linha com itens foi estabilizada e
   fechada. Wrap automático, linebreak explícito e migração de página usam a
   mesma rota; linhas vazias artificiais não incrementam.
5. Estado privado guarda contador documental e página causal. Scope `page`
   reinicia na primeira linha confirmada de cada página; `document` não.
6. O número formatado usa o formatter canônico, o TextStyle vigente e a
   margem externa pedida. Default é margem inicial, alinhamento afastando-se
   do texto e clearance automático proporcional ao tamanho ativo.
7. Nenhuma variante de `Content`, `NodeKind`, marker consultável, segunda
   passagem ou callback executável entra neste recorte.

## Divisão explícita da funcionalidade

Este L0 fica deliberadamente incompleto apenas para:

- `numbering` funcional (`n => content`), a completar no Passo 1415 por
  realização fora do layout puro;
- coordenação de margem em múltiplas colunas, também no Passo 1415, depois de
  medir o comportamento da última coluna.

P1414 não aceita silenciosamente callback como pattern nem reivindica paridade
nesses dois subconjuntos.

## Gate ADR-0127

O passo cria `par.line`, seus campos públicos e comportamento padrão de set.
Os L0 proprietários devem ser commitados e ressellados agora. A implementação
L1 fica bloqueada até confirmação humana explícita posterior a este contrato.

## Protocolo de memória para a implementação

- probes Typst: 512 MiB, uma thread, timeout 15 s e saída isolada em `/tmp`;
- build release: 2 GiB, um job, incremental desligado, debug zero,
  `codegen-units=512` e linker `gold`;
- lint: 1 GiB, uma thread, timeout 60 s;
- proibido `cargo test -p typst-core --lib` e `cargo check --tests`;
- executar primeiro apenas os seis oráculos P1414; matriz P1409 completa só
  depois do GREEN focal.

## Aceitação futura

Os cinco casos válidos devem coincidir bilateralmente em texto extraído, exit
e stderr; o constructor deve falhar com a mensagem pública vanila. A matriz
P1409 termina `PASS` com `{MATCH: 6}`. Build, formatação, diff-check e linter
passam nos limites, sem OOM ou swap.

## Confirmação e materialização

O contrato público acima foi commitado em
`c50abf1ab16d1de29242036ab0cb08e66170b37b` e confirmado explicitamente pelo
humano antes da escrita L1. A implementação materializou:

- o namespace `par.line`, com representação `line` e constructor sempre
  proibido;
- os cinco campos settable validados no eval e transportados pela StyleChain;
- contador privado documental/por página, incluído no checkpoint transacional;
- emissão na margem após wrap, fitting de página, correção da baseline e
  alinhamento da linha física;
- ajuste diferido intrapágina pela maior largura efetivamente observada, para
  que números de um e dois dígitos respeitem `number-align` bilateralmente;
- isolamento de subframes e ausência de incremento em flush vazio.

Callbacks de `numbering` e coordenação da última coluna continuam rejeitados e
declarados para P1415; não foram absorvidos silenciosamente.

## Verificação final e proveniência

Medição final às `2026-09-15T22:00:32-03:00`, sobre o checkpoint
`c50abf1ab16d1de29242036ab0cb08e66170b37b` mais a working tree não commitada
formada pelos oito consumers L1 alterados e por
relatório consolidado da limpeza.

- build release definitivo limitado a 2 GiB/um job: `PASS` em 3 min 40 s;
- cinco fixtures válidos: texto extraído bilateralmente idêntico;
- constructor: ambos exit 1 e mensagem pública
  `cannot be constructed manually`; somente a mecânica de apresentação da
  localização difere entre os CLIs;
- ataques: campo desconhecido, margem vertical e callback foram rejeitados;
  uma sequência até `11` encontrou e depois confirmou a correção da largura
  máxima para `number-align:left`;
- focal definitivo: seis matches, máximo 40.396 KiB RSS, sem timeout/OOM;
- re-sondagem P1409: 84 processos, `PASS`, `{MATCH: 6}`, ordem invariável,
  máximo 43.192 KiB RSS e 0 KiB swap;
- `cargo fmt --check`, `git diff --check` e `crystalline-lint .`: `PASS`;
  o linter terminou com exit 0 e zero violações;
- binário cristalino: identificador administrativo
  [dado administrativo removido];
- relatório: identificador administrativo
  [dado administrativo removido].

Regime Tekt: rehearsal completo, executado sem atestação de isolamento.
