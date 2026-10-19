# Passo 1441 — Materializar `math.overline` com identidade e geometria MATH superior

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


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

Hashes identificador administrativo das fontes medidas:

| Fonte | identificador administrativo |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-layout/src/math/line.rs` | [dado administrativo removido] |

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

Hashes identificador administrativo dos consumers antes da proposta:

| Consumer | identificador administrativo |
|---|---|
| `01_core/src/entities/elements/math_underline.rs` | [dado administrativo removido] |
| `01_core/src/entities/content.rs` | [dado administrativo removido] |
| `01_core/src/compiler/stdlib/structural/math.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/math.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/repr.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/underline.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/mod.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/spacing.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/mod.rs` | [dado administrativo removido] |
| `01_core/src/compiler/introspect.rs` | [dado administrativo removido] |
| `01_core/src/compiler/introspect/locatable.rs` | [dado administrativo removido] |
| `01_core/src/compiler/layout/mod.rs` | [dado administrativo removido] |
| `03_infra/src/query_helpers.rs` | [dado administrativo removido] |

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
