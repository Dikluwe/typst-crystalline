# Passo 1397 — `repr` canônica de classe e estilo matemáticos

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo e regime

Corrigir duas projeções morfológicas gerais no owner
`compiler/eval/repr`: `MathClassOverride` deve nomear `class` e `body`, e
`MathStyled` com qualquer eixo explícito deve preservar
`styled(child: ..., ..)`. Isso fecha `math.dif`/`math.Dif` sem lhes criar
exceções nominais e corrige os estilos adjacentes medidos.

Aplica-se o protocolo completo `tekt-materializacao-segregada`: contrato,
oráculos, ataques, candidato e veredito pertencem a autoridades distintas.
Como o workspace é compartilhado, a entrega declara **executado sem atestação
de isolamento**. Este passo não emite selo nem veredito.

## Medição anterior à decisão

Estado causal observado em `2026-09-14T16:55:44-03:00`: HEAD limpo
`c5af9f2af02bd830ec223f777a8d144356b03597`, `git status --short` e
`git diff HEAD --stat` vazios. Identidades:

- L0 anterior `00_nucleo/prompts/compiler/eval/repr.md`, identificador administrativo
  [dado administrativo removido];
- consumer `01_core/src/compiler/eval/repr.rs`, identificador administrativo
  [dado administrativo removido];
- binário vanila ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  identificador administrativo
  [dado administrativo removido];
- binário cristalino baseline, identificador administrativo
  [dado administrativo removido].

Sondas seriais sob `ulimit -v 524288` e timeout de 60 s mediram no vanila:

```text
repr(math.dif)  = sequence(... class(class: "unary", body: styled(child: [d], ..)) ...)
repr(math.Dif)  = sequence(... class(class: "unary", body: styled(child: [D], ..)) ...)
repr(math.class("unary", [x])) = class(class: "unary", body: [x])
```

O cristalino baseline usa `class("unary", ...)`; em `dif`/`Dif`, além disso,
achata o corpo styled para `"d"`/`"D"`. Nos estilos, `bb`, `frak`, `serif` e
`inline` sobre `[x]` preservam `styled(child: [x], ..)` no vanila e são
achatados para `[x]` no cristalino; `mono` e `script` já coincidem com a forma
styled. `math.lim` coincide como `op(text: [lim], limits: true)`.

Nos probes locais válidos de `dif`, `Dif`, `class`, `bb` e `lim`, o maior RSS
foi 18.036 KiB no vanila e 33.036 KiB no cristalino, com swap zero. O probe
cristalino de `math.upright` terminou exit 1 por binding ausente e foi
descartado, sem uso decisório.

A fonte ratificada fixa a intenção: `math/op.rs:49-54` constrói `dif`/`Dif` com
`ClassElem(Unary, upright(...))`; `math/mod.rs:141-149` declara `class` e
`body`; `foundations/content/mod.rs:626-640,759-788` define a repr por campos e
a forma opaca de todo StyledElem. Os identificador administrativo dessas fontes constam no contrato
P1397. Assim a decisão mede língua antes de escolher mecânica (ADR-0108).

## Decisão e delta autorizado

Morfologia textual pública é língua (ADR-0107). No único consumer proprietário
`01_core/src/compiler/eval/repr.rs`:

- todo `MathClassOverride` usa `class(class: <classe>, body: <corpo>)` na ordem
  declarada, em forma curta/multiline pelo formatter canônico;
- todo `MathStyled` com ao menos um eixo explícito usa
  `styled(child: <corpo>, ..)`, inclusive Plain, valor igual ao default e os
  eixos variante/tamanho/cramped/bold/italic.

Não criar branches para `dif`, `Dif`, letras, classe unary ou nomes de estilo.
É permitido atualizar os testes locais contraditórios e a linhagem 1:1. Não é
permitido alterar entidades, stdlib, construção, parser, layout, export,
`MathOp`, outros formatters ou API. É correção interna de paridade e segue
fluxo contínuo ADR-0127, sem parada humana intermediária.

## RED, ataques e GREEN

O oráculo independente deve congelar `dif`, `Dif`, `math.class`, `bb`, `frak`,
`serif`, `inline`, `mono`, `script` e `lim`, executar baseline RED e vanila
positivo, repetir e inverter a ordem. O catálogo adversarial deve rejeitar:

- fields posicionais, nomes trocados, ordem invertida ou perda de field;
- correção especial apenas para `dif`/`Dif` ou apenas para unary;
- achatamento de qualquer MathStyled explícito;
- preservação somente dos kinds historicamente já verdes;
- exposição de detalhes internos de styles no lugar de `..`;
- quebra codificada ou regressão de `MathOp`/controles.

Todos os mutantes aplicáveis precisam ser mortos (`mutation_score = 1.0`).
Falha de identidade, parse, build, execução, timeout, OOM, swap ou evidência
ilegível é `Unknown`; `Unknown` obrigatório bloqueia. Casos fora da matriz não
viram sucesso implícito.

## Contenção de memória

Não executar `cargo test`, suíte de workspace nem compilar o lib-test
monolítico. Sondas e oráculo rodam um processo por vez com:

```bash
bash -lc 'ulimit -v 524288; exec timeout 60s /usr/bin/time -v <sonda-cli-p1397>'
```

Build/check/lint rodam sequencialmente, uma job, sob 2 GiB de `RLIMIT_DATA`:

```bash
bash -lc 'ulimit -d 2097152; export CARGO_BUILD_JOBS=1; exec timeout 360s /usr/bin/time -v cargo build --release --bin typst'
bash -lc 'ulimit -d 2097152; export CARGO_BUILD_JOBS=1; exec timeout 240s /usr/bin/time -v cargo check -p typst-core'
bash -lc 'ulimit -d 2097152; export RAYON_NUM_THREADS=1; exec timeout 180s /usr/bin/time -v crystalline-lint .'
```

Cada recibo registra instante, HEAD, diff/stat, argv, exit, duração, RSS e
swap. Swap diferente de zero ou RSS acima de 1.782.579 KiB interrompe a
sequência. Duas revisões sem ganho discriminatório pela mesma causa reabrem o
desenho; o teto não é elevado para fabricar verde.

## Capacidades segregadas e scope-out

- autor do contrato: escreve este passo, L0 e contrato JSON; não lê candidato
  futuro, não implementa e não emite selo/veredito;
- autor dos oráculos: escreve suíte/recibo contra contrato congelado, sem
  adaptar expected ao patch;
- adversário: escreve apenas mutantes/ataques, sem corrigir solução;
- implementador: recebe pré-selo e altera apenas o delta autorizado;
- verificador: não altera material julgado e emite recibo/veredito próprio.

Ficam fora de P1397 o binding ausente `math.upright`, outras ausências da
stdlib, paridade matemática geral, layout e export. A conclusão é estritamente
sobre a matriz morfológica congelada.
