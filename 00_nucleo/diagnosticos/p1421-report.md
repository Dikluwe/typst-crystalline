# P1421 — relatório de materialização

## Resultado

O fragmento funcional do P1421 foi implementado. O caminho HTML agora executa
`ContextBlock` entre eval e export com `EvalTarget::Html`, as features e a
política de erro causais; propaga erros e warnings; rejeita expansão incompleta;
remove apenas markers comprovadamente executados; e entrega o conteúdo
semântico ao exporter existente sem criar `PagedDocument` nem entrar no layout
paginado.

As três fixtures HTML congeladas ficaram bilaterais ao vanilla ratificado em
`direct`, `direct-repeat` e `reverse`:

- assert contextual do target: exit 0, 154 bytes, SHA-256
  `52569b3f0175d7ded4383171fa34c563019206cbc8bac63a3572e775bb72adcf`;
- `#context target()`: exit 0, 165 bytes, SHA-256
  `1f51255aef2eaf4fcbfdb0b08152ba5710258427f93be68692d5c200bc9321d0`;
- `html.elem("div")` contextual: exit 0, 165 bytes, SHA-256
  `fe18965c5ffb798226ea7d3335fda25c93b8a69817e9647e49812aa99c23f04c`.

O controle paginado continuou verde. A ausência da feature HTML e o panic
contextual continuam a falhar sem artefato. Os cinco testes independentes
passaram após quatro falhas RED no baseline. O gate mutante R2 executou os dez
mutantes contra os hashes finais: 10 `Violated`, zero sobreviventes, zero
`Unknown`, score `1.0`.

## Focal P1420

A reexecução de `html.area` e `math.abs` foi estável nas três ordens. O antigo
controle de target HTML deixou de bloquear a observação. O resultado focal é:

- `html.area`: `PARTIAL` (`qualified=ABSENT`, `alternative=MATCH`);
- `math.abs`: `PARTIAL` (`qualified=ABSENT`, `alternative=DIFFERENCE`).

Isto não reclassifica as outras 64 rotas nem fecha a campanha P1420.

## Gates

Passaram:

- `cargo test -p typst-infra p1421_ -- --nocapture` — 5/5;
- `cargo test -p typst-infra p821_context_target_expande_paged -- --nocapture`;
- `cargo build` e `cargo build --release`;
- `cargo fmt --all -- --check`;
- `crystalline-lint .` — zero violations;
- `git diff --check`;
- mutation gate R2 — 10/10, score `1.0`.

Proveniência detalhada: `p1421-results.json`. Mutation receipt final:
`p1421-mutation-results.json`, SHA-256
`db5b8788a43b6b34c4e3b6d238370dec8d3132839f125e2747b1c48bfaeb2985`.

## Limitação de atestação

O ensaio foi executado sem atestação de isolamento. Além disso, o preseal
protegeu os bytes completos dos dois L0 antes do resselo obrigatório. O
`crystalline-lint --fix-hashes` alterou somente a linha recíproca
`Hash do Código` em cada prompt, mas o manifesto não definiu canonicalização
nem exceção para essa alteração. Portanto, pela regra literal do protocolo, a
cadeia selada foi invalidada.

Consequentemente, este relatório afirma que o fragmento funcional,
arquitetural e mutante passou, mas não afirma conformidade/atestação plena do
protocolo segregado. Reparar essa limitação exige uma nova cadeia que congele
explicitamente a projeção semântica do L0 ou preveja o resselo recíproco antes
da implementação; não é lícito inventar essa exceção retroativamente.

## Limite do veredito

O resultado cobre apenas expansão contextual HTML no fragmento congelado, sem
layout paginado, e os controles declarados. Não prova paridade HTML geral,
paridade Typst geral nem encerramento do P1420.
