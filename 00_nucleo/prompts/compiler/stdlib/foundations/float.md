# Prompt L0 — `stdlib/foundations/float` — superfície pública de predicados `float`
Hash do Código: c95f8eb6

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/float.rs`
**Prompt pai**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: P1289
**Baseline**: vanilla ratificado `a51e02804`
**ADRs**: ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

Em `2026-08-31T10:35:47-03:00`–`10:40:57-03:00`, sobre HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88` e working tree não commitida, o
binário vanilla de SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`
devolveu `(function, "is-infinite")` para o field estático e
`[false,false,true,true,false]` para `0.0`, `42.5`, infinito positivo,
infinito negativo e NaN. A forma ligada produziu o mesmo vetor.

As duas ordens forward/reverse produziram resultados idênticos. A obtenção
do método ligado como valor, sem chamada, falhou no vanilla com
`cannot access fields on type float`; portanto, esse acesso reflexivo não faz
parte da superfície autorizada. O cristalino pré-candidato, construído da
árvore corrente em target isolado e com SHA-256
`e7c81e3a6c6f1933db95c2249286fca19f3b5991eb8bc392f49ea659cec0bd78`,
falhou por ausência do field/método.

Uma auditoria posterior das pré-condições mostrou que `float.inf` e
`float.nan` são também `MISSING_MEMBER` no cristalino
(`p1284-inventory-default.json`). Os dois lados aceitam
`float("1e999")` como infinito positivo e `float("NaN")` como NaN; estes
carriers isolam a semântica de `is-infinite` sem materializar dois fields fora
do escopo nem converter o ganho esperado de `+1` em `+3`. O primeiro selo A/B
foi invalidado e refeito a partir desta medição.

A fonte vanilla medida em
`lab/typst-original/crates/typst-library/src/foundations/float.rs:81-94`
declara `is_infinite(self) -> bool` e delega a `f64::is_infinite`. A macro
upstream é mecânica, não contrato arquitetural (ADR-0107).

## Contrato da linguagem

- `float.is-infinite` existe como `function` com `repr` exatamente
  `"is-infinite"`.
- `float.is-infinite(self)` aceita `Float` e a coerção de `Int` já medida;
  retorna `true` somente para infinito positivo ou negativo.
- Valores finitos, ambos os zeros e NaN retornam `false`; os testes usam
  `float("1e999")`/`-float("1e999")` e `float("NaN")` como carriers bilaterais.
- `(value).is-infinite()` é a forma ligada para `Value::Float` e delega à
  mesma função da forma estática, sintetizando `self` como primeiro positional.
- O acesso não chamado `(value).is-infinite` permanece ausente e mantém
  `cannot access fields on type float`.
- Aridade e named args são fechados: ausência de `self` →
  `missing argument: self`; segundo positional → `unexpected argument`;
  named desconhecido → `unexpected argument: <nome>`; tipo não coercível →
  `expected float, found <tipo>`.

## Estrutura

`float_type_field` é um match fechado que descobre somente
`is-infinite`. A nativa pura possui a fórmula `f64::is_infinite`; o dispatch
ligado insere o receiver como primeiro positional e chama a mesma nativa.
Não criar trait, registry, reflexão, fallback genérico nem duplicar a fórmula
em `field_access` ou `call_dispatch`.

As funções de integração Rust são internas à crate (`pub(crate)`), sem novo
contrato público. O constructor `float(...)` permanece no owner
`foundations/cast`; `signum`, bytes e outros fields, inclusive as constantes
`float.inf` e `float.nan`, continuam resíduos independentes do inventário.

## Verificação

Cobrir presença, `repr`, chamada estática e ligada, finitos, `±inf`, `nan`,
coerção de inteiro, missing/extra/named/type errors e ausência do valor ligado.
Os testes devem rejeitar as mutações: sempre falso, `nan` infinito, aridade
permissiva, `repr` qualificado e presença sem chamada. Finalizar com RED→GREEN,
sonda bilateral forward/reverse, `cargo build`, `cargo test --workspace`,
`git diff --check` e `crystalline-lint .` sem violations.

## P1293 — `float.is-nan` (GATE ADR-0127)

### Medição anterior à decisão

Em `2026-09-01T13:24:01-03:00`, no baseline
`7dd25ff0e222b6c7c640d6bc7957b98f94227507` com working tree não commitada
registrada em `p1293-baseline-status.txt`, o vanilla ratificado mediu
`float.is-nan` como `(function, "is-nan")`. A fonte pinada
`lab/typst-original/crates/typst-library/src/foundations/float.rs:12-30,32-39,67-80`
declara a coerção estática `Int -> Float`, instala o field e delega a
`f64::is_nan`. O recibo independente P1293 mediu: NaN `true`; finito, inteiro e
`+/-infinito` `false`; receiver ligado somente `Float`; receiver `Int` e acesso
ligado sem chamada rejeitados. Missing ancora `missing argument: self` no call;
extra, named e tipo inválido ancoram o argumento ofensivo.

### Contrato da linguagem

- `float.is-nan` existe como função de nome público curto `is-nan`.
- A forma estática aceita exatamente um `Float`, com a coerção de `Int` já
  medida, e retorna `true` somente para NaN.
- `(value).is-nan()` existe para receiver `Float` e chama a mesma nativa; não há
  segunda fórmula. Receiver `Int` não ganha método ligado por causa da coerção
  aceita na forma estática.
- Obter `(value).is-nan` sem chamada continua ausente; `float.inf`, `float.nan`,
  `signum` e bytes não entram neste incremento.
- Ausência de `self`, positional extra, named e tipo não coercível permanecem
  erros fechados com as mensagens e spans medidos; nenhum argumento é ignorado.

O match fechado `float_type_field` e o dispatch ligado são estendidos em
paralelo a `is-infinite`; não criar trait, registry, fallback reflexivo nem
duplicar a fórmula. Esta adição muda superfície pública e fica bloqueada pelo
gate humano P1293 antes do código.

## P1293.reopen-A — fronteira interna dos spans diagnósticos

### Medição anterior à decisão

Em `2026-09-01T15:13:15-03:00`, sobre HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree não commitida, o
recibo segregado `p1293-implementation-receipt-a.md` de SHA-256
`14ca51a7b440ce65e46eda9dadd7b47a21e15dd7a991b193e5d103beac42ced1`
registou quatro positivos verdes e cinco REDs somente de span. Valores, tipos,
`repr` e mensagens já coincidem. Os ranges candidato → esperado são:

- missing estático `12..14` → `0..14`;
- positional extra `12..22` → `18..21`;
- named ligado `19..32` → `20..31`;
- cast estático `12..17` → `13..16`;
- acesso ligado sem chamada `0..19` → `13..19`.

O próprio recibo refuta corrigir a falha dentro da fórmula: quatro âncoras
existem na AST de `call_dispatch` antes da agregação em `Args`, e a quinta
pertence ao `field_access`. Inferir offsets a partir de texto aqui duplicaria
parsing e falharia para o acesso sem chamada.

### Classificação e decisão

Mensagem e span são observáveis da linguagem sob ADR-0107. O contrato P1293 já
os exige exatamente; esta reabertura não muda a intenção pública e segue em
fluxo contínuo ADR-0127 depois de invalidar e refazer lineage/gate/selo.

Este owner conserva exclusivamente descoberta, coerção, fórmula
`f64::is_nan`, validação fechada e dispatch semântico. A validação deve usar a
âncora interna fornecida em `Args.span` pelo owner `call_dispatch` para
missing/extra/named/cast, sem recomputar offsets, ler fonte ou alterar as
mensagens. O erro de acesso sem chamada permanece fora deste consumer e usa a
âncora do owner `field_access`.

Não alterar `entities::Args`, nenhuma API pública, assinatura pública,
entidade, default, fase ou semântica de valores. Não criar segundo consumer ou
owner 1:N: este Prompt permanece 1:1 com
`01_core/src/compiler/stdlib/foundations/float.rs`; `call_dispatch.md` e
`field_access.md` permanecem owners 1:1 dos seus próprios consumers.
