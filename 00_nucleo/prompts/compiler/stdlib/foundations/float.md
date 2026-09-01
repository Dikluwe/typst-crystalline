# Prompt L0 — `stdlib/foundations/float` — superfície pública de `float.is-infinite`
Hash do Código: 00000000

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
`foundations/cast`; este passo não materializa `is-nan`, `signum`, bytes ou
outros fields, inclusive as constantes `float.inf` e `float.nan`, que continuam
resíduos independentes do inventário.

## Verificação

Cobrir presença, `repr`, chamada estática e ligada, finitos, `±inf`, `nan`,
coerção de inteiro, missing/extra/named/type errors e ausência do valor ligado.
Os testes devem rejeitar as mutações: sempre falso, `nan` infinito, aridade
permissiva, `repr` qualificado e presença sem chamada. Finalizar com RED→GREEN,
sonda bilateral forward/reverse, `cargo build`, `cargo test --workspace`,
`git diff --check` e `crystalline-lint .` sem violations.
