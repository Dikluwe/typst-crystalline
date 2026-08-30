# P1151 — transportar `Location` no conteúdo devolvido por `query`

**Data:** 2026-08-24
**Estado:** `EXECUTADO — GREEN; BASELINE INTEGRAL PRESERVADO`
**Baseline:** vanilla ratificado `a51e02804`

## Objetivo

Fazer `content.location()` e `content.location(c)` devolverem a Location exata
para conteúdo vindo de introspecção, preservando `none` para conteúdo inline.

## Medição

Nos dois binários vanilla, duas headings idênticas devolvidas por
`query(heading)` são iguais como conteúdo, mas têm Locations distintas. No
cristalino, `native_query` descarta a Location ao construir `Value::Content`.
Recuperação posterior por igualdade é portanto incorreta.

## L0 atualizado

- `00_nucleo/prompts/entities/value.md`;
- `00_nucleo/prompts/compiler/stdlib/foundations/query.md`;
- `00_nucleo/prompts/compiler/eval/bindings/field_access.md`.

## Gate ADR-0127

A solução introduz representação pública em `Value` para transportar
`Content + Location`, embora seu tipo observável continue sendo `content`.
Isso altera contrato Rust público. Não ressellar hashes nem escrever código
antes da confirmação do dono.

## Execução após confirmação

1. registrar confirmação e hashes L0;
2. RED: dois resultados idênticos de query têm Locations distintas;
3. RED: igualdade/repr/type/fields ignoram o metadado;
4. RED: conteúdo inline continua `none`;
5. materializar a representação fechada em `Value`;
6. fazer `native_query` preservar o par exato;
7. unificar despacho estático e de instância de `location`;
8. atualizar matches exaustivos sem fallback;
9. ressellar linhagem e executar testes focados, workspace e suite integral.

## Baseline das 40 falhas

Classificação nominal e causas observadas em
`00_nucleo/diagnosticos/p1151-location-e-baseline-40-falhas.md`: 30 ligadas a
I/O/fixtures e 10 semânticas antigas (1 int, 3 radial focal, 6 relative/stops).

## Execução

- Gate confirmado pelo dono em 2026-08-24 com “Continue”.
- Hora do rebaseline: `2026-08-24T22:04:41-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Hashes L0 confirmados antes do código: `e4b16821` (`value.md`),
  `11bf6ff5` (`query.md`) e `c027f00a` (`field_access.md`).

Materialização:

- `Value::LocatedContent(Content, Location)` transporta o par exato;
- `native_query` produz essa variante quando `element_at(loc)` existe;
- tipo, repr, field access, display e igualdade da linguagem delegam ao
  `Content` interno e ignoram Location;
- `eval_content_method_at` é owner único dos cinco métodos e devolve a
  Location apenas quando presente;
- formas estática e de instância usam o mesmo owner;
- conteúdo inline continua com `none`.

GREEN focado:

- P1151: 1 aprovado;
- regressão fixpoint P179/P844: 1 aprovado;
- P1150: 1 aprovado;
- P829: 23 aprovados.

Validação: `cargo check --workspace`, `cargo build --workspace`, formatação e
`git diff --check` aprovados; `crystalline-lint .` sem violações. Suite
integral: 5.183 aprovados e as mesmas 40 falhas nominais preexistentes. O
único RED transitório integral foi o teste P179/P844 que ainda exigia a antiga
representação literal `Value::Content`; atualizado para o novo contrato e
GREEN, sem absorver regressão.
