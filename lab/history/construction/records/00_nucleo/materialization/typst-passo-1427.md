# Passo 1427 — convergência vertical completa de `math.frac`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Estado medido antes da decisão

P1426 fechou `math.root` e deixou 20 rotas `math.*` residuais da coorte
P1422. A leitura comparativa rejeitou dois atalhos: expor apenas a fração
vertical default chamaria um fragmento de assinatura de convergência, e
reutilizar o `overline` textual para `math.overline` colapsaria identidades
que o vanilla mantém distintas. `math.frac` é a próxima fatia que já possui
carrier matemático próprio e pode ser fechada sem essa perda semântica.

Medição executada em 2026-09-16T17:31:22-03:00 no HEAD limpo
`451f3ca3d208c997a41d93ccc597a9c117fe1de9`, tree
`fc442ca9056397eb311dcd4f759ec6b6478f71ea`. Os doze pares L0/consumer
congelados são:

| L0 | identificador administrativo L0 | consumer | identificador administrativo consumer |
|---|---|---|---|
| `compiler/stdlib/structural/math.md` | [dado administrativo removido] | `compiler/stdlib/structural/math.rs` | [dado administrativo removido] |
| `compiler/eval/math.md` | [dado administrativo removido] | `compiler/eval/math.rs` | [dado administrativo removido] |
| `compiler/eval/repr.md` | [dado administrativo removido] | `compiler/eval/repr.rs` | [dado administrativo removido] |
| `entities/elements/math_frac.md` | [dado administrativo removido] | `entities/elements/math_frac.rs` | [dado administrativo removido] |
| `entities/content.md` | [dado administrativo removido] | `entities/content.rs` | [dado administrativo removido] |
| `compiler/eval.md` | [dado administrativo removido] | `compiler/eval/mod.rs` | [dado administrativo removido] |
| `compiler/math/layout/_comum.md` | [dado administrativo removido] | `compiler/math/layout/mod.rs` | [dado administrativo removido] |
| `compiler/math/layout/frac.md` | [dado administrativo removido] | `compiler/math/layout/frac.rs` | [dado administrativo removido] |
| `compiler/eval/rules.md` | [dado administrativo removido] | `compiler/eval/rules.rs` | [dado administrativo removido] |
| `compiler/math/layout/tests.md` | [dado administrativo removido] | `compiler/math/layout/tests.rs` | [dado administrativo removido] |
| `entities/math_constants.md` | [dado administrativo removido] | `entities/math_constants.rs` | [dado administrativo removido] |
| `infra/font_metrics.md` | [dado administrativo removido] | `infra/font_metrics.rs` | [dado administrativo removido] |

No vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`math/frac.rs` (identificador administrativo
[dado administrativo removido])
declara `num`, `denom`, `style: vertical | skewed | horizontal` e dois bits
internos de remoção de parênteses. O avaliador de math (identificador administrativo
[dado administrativo removido])
e o parser/AST (identificador administrativo [dado administrativo removido]
e [dado administrativo removido])
provam que `/` é associativo à esquerda e transporta esses bits. O binário
ratificado tem identificador administrativo
[dado administrativo removido].
O layout vanilla que define a geometria dos três estilos,
`typst-layout/src/math/fraction.rs`, tem identificador administrativo
[dado administrativo removido].

Sondas públicas mediram:

| Expressão | Resultado público |
|---|---|
| `repr(math.frac())` | erro `missing argument: num` |
| `repr(math.frac([a]))` | erro `missing argument: denom` |
| `repr(math.frac([a], [b]))` | `frac(num: [a], denom: [b])` |
| `repr(math.frac(style: "skewed", [a], [b]))` | `frac(num: [a], denom: [b], style: "skewed")` |
| `repr(math.frac([a], [b], style: "vertical"))` | inclui `style: "vertical"` por ter sido explícito |
| `repr(math.frac(none, none))` | `frac(num: [], denom: [])` |
| terceiro posicional | erro `unexpected argument` |
| `num:` ou `denom:` named | erro ``the argument `<nome>` is positional`` |
| style inválido | erro com domínio `vertical`, `skewed`, `horizontal` |

`$frac(a,b)$` e `$a/b$` produzem a mesma fração vertical pública;
`$a/b/c$` associa à esquerda. Um par exterior é removido por `/`, mas um
segundo par permanece. Os bits são omitidos de `repr`, porém observáveis no
layout horizontal. Strings e `none` convertem para conteúdo; inteiros não.

## Decisão proposta

1. ampliar `MathFracElem` com estilo opcional explícito e os dois bits causais
   de deparenthesization, preservando o `line` interno usado exclusivamente
   pelo `binom` cristalino;
2. expor `MathFracStyle::{Vertical, Skewed, Horizontal}` e um constructor
   completo, mantendo wrappers legados estreitos para callsites verticais e
   para `binom` sem barra;
3. registrar `math.frac(num, denom, style:)` com aridade, casts, duplicados e
   diagnósticos medidos; `style` omitido permanece distinto de `vertical`
   explícito em `repr`;
4. fazer `frac(a,b)`, `a/b` e a função qualificada convergirem no mesmo
   carrier; somente `/` transporta os bits causais do AST;
5. materializar os três layouts: stack vertical, fração inclinada e fração
   horizontal, preservando a morfologia de parênteses no modo horizontal;
6. acrescentar e extrair as constantes OpenType MATH
   `SkewedFractionVerticalGap` e `SkewedFractionHorizontalGap`, usadas com
   `AxisHeight` na geometria inclinada; a fonte de referência
   `NewCMMath-Book.otf` tem identificador administrativo
   [dado administrativo removido];
7. tornar `math.frac.style` settable: regras validam o domínio fechado e
   guardam a string canônica no `StyleChain`; o consumer converte-a de forma
   total para `MathFracStyle`. Estilo explícito vence o cascaded e ausência em
   ambos usa vertical;
8. preservar estilo, explicitude, bits e `line` em walkers, igualdade
   observacional e reconstruções; `binom` continua vertical, sem barra e não
   herda `math.frac.style`.

A aceitação é semântica, sintática e morfológica. Estrutura Rust, sequência de
operações, fragmentação de caixas e bytes de render são mecânica; mensagens de
erro e presença explícita de `style` em `repr` são observáveis.

## Limites explícitos

P1427 não altera `math.binom`, salvo preservar sua forma sem barra; não fecha
as outras 19 rotas residuais; não exige igualdade de estrutura Rust com o
vanilla; não aceita um binding vertical-only como convergência integral.
Casos sem identidade verificável, falha de build/execução, timeout, OOM, swap,
stream truncado ou ilegível são `Unknown` e bloqueiam; nunca recebem crédito.

## Gate ADR-0127

O passo acrescenta enum/campos públicos, amplia constructor L1, cria binding
público, torna uma propriedade settable e acrescenta dois caminhos de layout.
Incide nos itens 1, 2 e 3 do ADR-0127.

Fluxo obrigatório:

1. atualizar antes do código os doze L0 proprietários congelados;
2. commitar esta proposta e os L0s;
3. **PARAR sem alterar L1 nem headers de linhagem**;
4. continuar somente após confirmação explícita do dono;
5. depois da confirmação, produzir contrato, oráculos, ataques e testes
   segregados, selar, obter RED e só então implementar.

## Regime posterior ao gate

Usar protocolo completo `tekt-materializacao-segregada`, declarando
“executado sem atestação de isolamento” no workspace partilhado. O contrato
deve cobrir binding, aridade, casts, named, style/default/explicitude, bits
causais de `/`, associatividade, set rule, walkers, repr, três layouts e
regressão de `binom`. Mutation score exigido: 1.0; `Unknown` é fail-closed.

## Gates de fecho após confirmação

- RED da ausência do binding e das três variantes de estilo;
- RED de explicitude, casts, diagnósticos, associatividade e parênteses;
- RED do set rule e da precedência elemento explícito > cascaded > vertical;
- GREEN bilateral de chamada qualificada, `frac(a,b)` e `/`;
- controle `binom` sem barra e regressões verticais preservados;
- três ordens determinísticas e hashes selados intactos;
- `cargo fmt --all -- --check`, testes focais, `cargo build`,
  `crystalline-lint .` e `git diff --check`.

## Confirmação do gate

**Confirmado explicitamente pelo dono em 2026-09-16.** A materialização fica
autorizada somente dentro do recorte completo de `math.frac` e dos doze pares
L0/consumer acima; as outras 19 rotas math permanecem fora de escopo.
