# Passo 1441 — Materializar `math.overline` com identidade e geometria MATH superior

**Estado:** MATERIALIZADO — AGUARDA COMMITS DE PRODUTO E VERIFICAÇÃO
**Data da medição:** 2026-09-18T09:15:35-03:00
**Commit medido:** `3f4633b9267ccf89fa565601bbded58c259bf696`
**Árvore medida:** limpa (`git status --short` e `git diff HEAD --stat` sem saída)
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual do Passo 1422 ordena, depois de `math.overbracket`,
`math.overline`, `math.overparen`, `math.overshell`, `math.primes`,
`math.root`, `math.round`, `math.stretch`, `math.underbrace`,
`math.underbracket`, `math.underparen` e `math.undershell`. P1440 fechou
`overbracket`; portanto a próxima rota nominal é `math.overline`. `math.abs`,
deliberadamente não retomada pela sequência recente, continua residual e não
é declarada fechada.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/underover.rs` declara `OverlineElem` com apenas
`body: Content`, obrigatório e posicional. `math/ir/resolve.rs` resolve esse
body em estilo cramped e cria `LineItem` acima dele. `typst-layout` usa
`overbar_vertical_gap`, `overbar_rule_thickness` e
`overbar_extra_ascender`; a regra tem a largura lógica do body, não recebe o
ajuste de correção itálica usado pelo underline e conserva as propriedades
matemáticas do body.

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | `49fe33135766b83c16d27f84c51bd8f867cd5f3f790b075242e8043661a7e298` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |
| `lab/typst-original/crates/typst-layout/src/math/line.rs` | `d71623f47d8d3d9d9b5435820e5ca05399563642a4a48627f76ce0c8ed997b34` |

### 2.2 Observáveis públicos medidos no vanilla

Com `/usr/local/bin/typst` ratificado (`typst 0.15.1 (586e1bd4)`):

- `repr(math.overline)` produz `"overline"` e
  `math.overline == overline` produz `false`;
- `repr(math.overline([x]))` produz `overline(body: [x])`;
- `none` e string convertem para conteúdo vazio/textual no body;
- spread canônico e nesting conservam a identidade;
- a forma bare `$ overline(x) $` produz
  `equation(block: true, body: overline(body: [x]))`;
- ausência do body, segundo positional, `body:`, named desconhecido e inteiro
  falham, respectivamente, com os diagnósticos medidos de argumento ausente,
  inesperado, positional, named inesperado e tipo inválido.

### 2.3 Estado cristalino medido

A forma qualificada inexiste. A forma bare resolve para a decoração textual
homônima e aparece como `equation(block: true, body: overline[x])`, apagando
a identidade matemática e aplicando a mecânica de decoração de texto. O
carrier `MathUnderline` e seu layout baseado em constantes MATH já fornecem o
padrão estrutural, mas não podem representar `overline` sem uma identidade
pública distinta e sem inverter estilo, constantes e posição da regra.

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_underline.rs` | `6a08c552d47f7a90263b6f3e447f15ee9e42f17885201a866db910edddfa4948` |
| `01_core/src/entities/content.rs` | `94f19dcc03c543d58f3c2c3cb95816be04d6c9c41cdd553cafb5cfe438609bb3` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `ac2be30724c9ee2a8bd1eab3cf30ef2f048414e1db32d0b05a76ede5fe75694a` |
| `01_core/src/compiler/eval/math.rs` | `d601a3a647d5bc0581c1bf5e5cf9b2a05415dd6eadd41ac8a5b1fe660b7c3e38` |
| `01_core/src/compiler/eval/repr.rs` | `879c3f31ada68801d660ce1580428de3d06127aaee09e6f1667e5179c6efc8a9` |
| `01_core/src/compiler/math/layout/underline.rs` | `c4d58534f2cc25767cbaa63e1d150a50c3599fc43672ea87d030610d279a37db` |
| `01_core/src/compiler/math/layout/mod.rs` | `10abec9396e2a1a837fe00f6fa3bb53fd5436100c4e9140611a23da0a0e28780` |
| `01_core/src/compiler/math/layout/spacing.rs` | `1eebb285158be13c59393b8d37064e30d6e7b5059d7631d59de2f0cd3ebbed74` |
| `01_core/src/compiler/eval/mod.rs` | `c53257307d5fac9fec8e5184eafa2adc9aca14ba764f76edb4165d2560a283ef` |
| `01_core/src/compiler/introspect.rs` | `ba111026580465e87628c0f157b77c46f02f69c72fce318ce7bbc6e479e040bc` |
| `01_core/src/compiler/introspect/locatable.rs` | `f6d4ec3426351a79cad5dd6d26bd3ab79957b2530c4da1fc7651558c98a0800d` |
| `01_core/src/compiler/layout/mod.rs` | `61654b72c4cfb64beab6efdab01d3ad93d72c815cc4ceec7d3a6f4b9c8c8a8d4` |
| `03_infra/src/query_helpers.rs` | `7a392296e4d11a2fd777f611443a798de576a08bb063276363c0a09049957a99` |

## 3. Classificação anti-deriva

- Identidade `overline`, assinatura/casts, `repr`, estilo cramped, classe
  herdada e geometria observável são linguagem (ADR-0107).
- `LineItem`, `Position::Above`, frames e a organização das constantes são
  mecânica; podem divergir se os observáveis forem preservados.
- Inferência: a ausência de ajuste itálico na largura superior é intencional,
  pois o branch vanilla fixa `line_adjust = 0` somente para `Above`.
  Refutação: witness vanilla em que a regra superior varie com a correção
  itálica sem mudança de largura do body.
- Reutilizar a decoração textual seria uma equivalência visual acidental:
  sua identidade e seus campos `stroke`/`offset`/`extent` não pertencem à
  função matemática body-only.

## 4. Decisão L0 proposta

1. Adicionar `MathOverlineElem { body }` ao owner coeso
   `entities/elements/math_underline.rs`; não criar Prompt órfão e não
   reutilizar o `OverlineElem` textual.
2. Adicionar `Content::MathOverline(Arc<MathOverlineElem>)`, com construtor
   próprio, nome, igualdade, hash, plain-text e reconstruções estruturais.
3. Registrar `math.overline`; a forma bare delega ao mesmo builder. Exigir um
   body posicional convertível em conteúdo e rejeitar named/excedentes com os
   diagnósticos medidos, preservando spread e avaliação única.
4. Projetar sempre `overline(body: ...)`; nesting conserva a identidade e
   nunca é inferido da decoração textual ou da regra renderizada.
5. O owner de layout dispõe o body em estilo cramped, acrescenta regra acima
   com as três constantes `overbar_*`, largura lógica exata do body e sem
   ajuste de correção itálica. A free function fica em `layout/underline.rs`;
   o match central permanece magro (ADR-0109).
6. As classes esquerda/direita herdam recursivamente o body. A regra não
   troca a classe nem participa das fronteiras de spacing.
7. A variante é não-locatable e terminal nos walkers introspectivos e helpers
   aproximados L3; o walker estrutural L1 continua dono do body.

As outras dez rotas que sucedem `overline` no inventário e `math.abs`
permanecem fora de escopo.

## 5. Gate ADR-0127

Esta proposta cria elemento e variante públicos e corrige o comportamento por
defeito da rota bare, hoje capturada pela decoração textual. Portanto este
commit contém apenas passo e L0 e **PARA**. Testes, código e resselo começam
somente após confirmação humana explícita.

## 6. Execução após confirmação

1. Criar oráculos RED para binding qualificado/bare, aridade, casts, `repr`,
   nesting, estilo cramped, classe herdada, geometria e walkers.
2. Confirmar RED cristalino e GREEN vanilla onde aplicável.
3. Implementar carrier, builders, dispatch e layout na forma B.
4. Atacar mutantes: alias textual, apagar identidade, aceitar named/extra,
   não aplicar cramped, usar constantes underbar, pôr a regra abaixo, ajustar
   largura por itálico ou perder classe herdada.
5. Exigir score `1.0` para os mutantes definidos; resselo somente após
   V15/V26.
6. Validar `cargo fmt --check`, `cargo build --workspace`, suítes
   focal/completa, `git diff --check` e `crystalline-lint .`, registrando
   proveniência.

## 7. Confirmação humana

O dono confirmou explicitamente o gate ADR-0127 em 2026-09-18 após o commit
de proposta `1d2dbe984c5c47edcfcfed7f92bc1bc59ba1fa6e`. A cadeia RED→GREEN,
ataques, resselo e veredito está autorizada sem ampliar o escopo acima.

## 8. Materialização

P1441 foi materializado em 2026-09-18 sob o regime
`executed_without_isolation_attestation`. O RED atribuível apresentou oito
erros de compilação centrados na ausência de `MathOverlineElem`,
`Content::MathOverline` e helpers P1441. O candidato fechou cinco testes
focais, incluindo binding qualificado, identidade bare, assinatura/casts,
`repr`, carrier, walkers, classes e geometria superior cramped.

Dois probes públicos refutaram a primeira versão do candidato: o body bare
era projetado como texto em vez de conteúdo math e a identidade nativa
colidia com a forma bare. Os oráculos foram fortalecidos antes do selo final.
A campanha creditada, já sobre os hashes ressellados, executou as ordens
direta, direta repetida e reversa e matou 39/39 mutantes (13 famílias), score
`1.0`, zero `Unknown`. Binding, formas qualified/bare, `none`, string, spread,
nesting e os cinco diagnósticos negativos medidos coincidiram com o vanilla
ratificado.

Medição final em `2026-09-18T10:17:22-03:00`, sobre o commit de proposta
`1d2dbe984c5c47edcfcfed7f92bc1bc59ba1fa6e` com working tree não commitado;
o `git diff HEAD --stat` registrou 31 ficheiros rastreados, 355 inserções e 38
remoções, além dos artefatos P1441 ainda untracked. `cargo fmt --check`,
`git diff --check`, `cargo build --workspace` e `crystalline-lint .`
terminaram verdes; V5, V15 e V26 ficaram em zero. A suíte completa executou
5.875 testes L1: 5.867 verdes e as mesmas oito falhas históricas do baseline
P1440, nenhuma em arquivo ou observável P1441.
