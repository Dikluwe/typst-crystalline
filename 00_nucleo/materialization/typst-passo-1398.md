# Passo 1398 — escape canônico das strings internas de `Symbol::repr`

## Objetivo e regime

Corrigir, no owner `01_core/src/entities/symbol.rs`, a representação textual
de toda string interna de um `Symbol`: apóstrofo literal, aspas duplas com uma
única barra de escape e NUL como `\u{0}`. A regra geral preserva os escapes já
corretos de barra invertida, nova linha, tab, carriage return, variation
selectors e ZWJ. As testemunhas `sym.quote` e `math.quote` não recebem casos
especiais.

Aplica-se o protocolo completo `tekt-materializacao-segregada` porque o passo
altera morfologia pública e exige contrato, oráculos, ataques, implementação e
veredito independentes. O workspace compartilhado não oferece isolamento
ambiental forte; a linguagem proporcional obrigatória é **executado sem
atestação de isolamento**. Este artefato é contrato candidato: não sela e não
emite veredito.

## Medição anterior à decisão

Estado causal observado em `2026-09-14T17:24:01-03:00`: HEAD limpo
`762e95025a76cb8dec4b58097ca57aa878e05243`, com `git status --short` e
`git diff HEAD --stat` vazios. Identidades congeladas:

- L0 anterior `00_nucleo/prompts/entities/symbol.md`, SHA-256
  `159d961a5b85fcfad3dd668fce7097f52e5cb635a05a7ee9bfb983e1291aa95a`;
- consumer baseline `01_core/src/entities/symbol.rs`, SHA-256
  `abdf49eb37fc68ecd72f39634bbd29231da8f38e4c24fa337077bd120e8bcf03`;
- vanila ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino baseline, binário SHA-256
  `50c137a42d8a0da4ba43213c2a04e0f8896478a8fc05e25da7e9fee9247a873c`.

Sondas bilaterais seriais, sob `ulimit -v 524288` e timeout de 60 s,
produziram nove testemunhas RED:

| expressão | vanila ratificado | cristalino baseline |
|---|---|---|
| `repr(symbol("'"))` | `symbol("'")` | `symbol("\'")` |
| `repr(symbol("\""))` | `symbol("\"")` | `symbol("\\\"")` |
| `repr(symbol("\u{0}"))` | `symbol("\u{0}")` | `symbol("\0")` |
| `repr(sym.quote)` | variante `double` contém `"\""`; `single`, `"'"` | contém respectivamente `"\\\""` e `"\'"` |
| `repr(sym.quote.single)` | base `"'"` | base `"\'"` |
| `repr(sym.quote.double)` | base `"\""` | base `"\\\""` |
| `repr(math.quote)` | igual a `repr(sym.quote)` | mesmos escapes extras do cristalino |
| `repr(math.quote.single)` | igual a `repr(sym.quote.single)` | base `"\'"` |
| `repr(math.quote.double)` | igual a `repr(sym.quote.double)` | base `"\\\""` |

Os controles `symbol("\\")`, `symbol("\n")`, `symbol("\t")`,
`symbol("\r")`, `symbol("👩‍💻")`, `emoji.heart` e `sym.arrow.r` coincidiram.
O maior RSS válido foi 18.196 KiB no vanila e 33.320 KiB no cristalino; todos
os processos terminaram com swap zero. Uma primeira rodada vanila com
`--color` na posição inválida terminou exit 2 e foi descartada integralmente;
nenhum número ou output dela fundamenta a decisão.

A intenção vem da fonte, não apenas do comportamento: no vanila ratificado,
`foundations/symbol.rs:339-390` encaminha tanto símbolos simples quanto bases
e valores de variantes à mesma representação de string;
`foundations/str.rs:718-753` especifica NUL como `\u{0}`, apóstrofo literal,
aspas como `\"` e os demais caracteres por `escape_debug`. Seus SHA-256 são,
respectivamente,
`4213a43cc71d0653992ce4af5af40848ef491e81a1a5fbcdb560736b1adad18a`
e `8648ad25be763830470664c9dc022f096902a496e4b553a7d75259b25334cdb7`.

## Decisão e delta autorizado

A forma retornada por `repr` é morfologia/sintaxe observável da linguagem
(ADR-0107). O owner L0 é exclusivamente
`00_nucleo/prompts/entities/symbol.md` e seu único consumer produtivo é
`01_core/src/entities/symbol.rs`. Os demais arquivos históricos citados pelo
L0 são dependências ou consumidores, não owners.

Toda string literal emitida dentro de `Symbol::repr` deve aplicar, por scalar:

- U+0027 apóstrofo: literal `'`, sem barra;
- U+0022 aspas: exatamente `\"`;
- U+0000 NUL: exatamente `\u{0}`;
- U+005C, U+000A, U+0009 e U+000D: respectivamente `\\`, `\n`, `\t` e `\r`;
- ZWJ e variation selectors: os escapes Unicode minúsculos vigentes, sem
  truncamento, normalização ou perda do restante do grapheme.

A mesma regra vale para símbolo simples, base, valor de variante e qualquer
outra string interna emitida pelo consumer. É proibido reconhecer `quote`,
`sym`, `math`, `single`, `double`, um glyph ou posição da variante. Pode-se
alterar somente helpers privados de representação em `symbol.rs`, seus testes
locais diretamente relacionados e metadados de linhagem. Não se alteram
constructors, validação de grapheme, seleção/ordem de modifiers, tabelas de
símbolos, igualdade/hash, layout, `Value`, bindings nem `repr` de outros tipos.

Trata-se de correção interna de paridade, sem mudança de contrato Rust,
comportamento padrão ou fase de pipeline; segue em fluxo contínuo conforme
ADR-0127, sem parada humana intermediária.

## RED, ataques e GREEN

O autor de oráculos congela as nove testemunhas RED e os sete controles acima
antes do candidato. O adversário deve cobrir, no mínimo:

- deixar `escape_debug` escapar apóstrofo;
- aplicar uma segunda substituição às aspas;
- manter NUL abreviado como `\0`;
- corrigir somente base, somente variants ou somente `quote`;
- trocar os escapes corretos de barra/nova linha/tab/carriage;
- apagar ou tornar literal VS/ZWJ;
- reconhecer nominalmente qualquer testemunha;
- mudar filtragem, ordem ou quebra compacta/multilinha das variants.

Todo mutante válido precisa ser rejeitado (`mutation_score = 1.0`). Caso
positivo válido é `Preserved`, mutação negativa é `Violated` com testemunha e
somente fixture explicitamente opaca pode ser `Unknown`. Identidade ambígua,
parse/build/execução falho, timeout, OOM, swap, saída ilegível ou budget
esgotado gera `Unknown`; `Unknown` em caso obrigatório bloqueia o selo. Repetir
e inverter a ordem das sondas não pode mudar a classificação.

## Contenção de memória

Não executar `cargo test`, suíte de workspace nem compilar o lib-test
monolítico. Oráculos CLI rodam um processo por vez:

```bash
bash -lc 'ulimit -v 524288; exec timeout 60s /usr/bin/time -v <sonda-cli-p1398>'
```

Build, check e lint rodam sequencialmente, com uma job e limite de dados de
2 GiB:

```bash
bash -lc 'ulimit -d 2097152; export CARGO_BUILD_JOBS=1; exec timeout 360s /usr/bin/time -v cargo build --release --bin typst'
bash -lc 'ulimit -d 2097152; export CARGO_BUILD_JOBS=1; exec timeout 240s /usr/bin/time -v cargo check -p typst-core'
bash -lc 'ulimit -d 2097152; export RAYON_NUM_THREADS=1; exec timeout 180s /usr/bin/time -v crystalline-lint .'
```

Cada recibo registra instante, HEAD, `git diff HEAD --stat`, argv, exit,
duração, RSS e swap. Swap não zero ou RSS acima de 1.782.579 KiB interrompe a
sequência. Duas revisões sem ganho discriminatório pela mesma causa reabrem o
desenho; o limite não é ampliado para fabricar verde.

## Capacidades segregadas e scope-out

- autor do contrato: pode ler L0, baseline e fonte ratificada e escrever
  apenas este passo, L0 e contrato JSON; não lê candidato futuro, não
  implementa, não sela e não julga;
- autor dos oráculos: escreve casos e recibo contra este contrato, sem adaptar
  expected ao patch;
- adversário: escreve mutantes e ataques, sem escolher ou corrigir solução;
- implementador: recebe apenas entradas seladas e altera o delta autorizado;
- verificador: lê artefatos selados e outputs candidatos, não os altera e
  emite recibo/veredito próprio.

Ficam fora de P1398 tabelas e cobertura de símbolos, construtor/erros,
modifiers, igualdade, concatenação, markup, math/layout/export, largura do
pretty-printer e representação geral de strings fora de `Symbol`. A conclusão
será estritamente sobre o fragmento de escapes congelado.
