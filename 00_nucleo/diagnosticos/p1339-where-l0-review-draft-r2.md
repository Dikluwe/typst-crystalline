# P1339 — parecer focal final da minuta pública, R2

Revisor: `/root/p1339_where_l0_review`. Revisão de L0, executada sem
atestação de isolamento técnico. Não escrevi os L0 examinados nem código,
contrato, testes ou certificado. Este adendo preserva R1 e o parecer
preliminar; não os reescreve como se os achados nunca tivessem existido.

## Entradas finais examinadas

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado.
`git diff HEAD --stat` na captura focal registra somente estes L0:

```text
compiler/eval/bindings/value_methods.md       | 88 +
compiler/eval/operators/equality.md           | 43 +
compiler/eval/selector_matching.md            | 58 +
entities/selector.md                         | 80 +
entities/show.md                             | 46 +
5 files changed, 315 insertions(+)
```

Paths relativos a `00_nucleo/prompts/`. Artefatos P1339 não rastreados
coexistem. Hashes SHA-256 capturados antes da leitura focal:

| Minuta | SHA-256 |
|---|---|
| `entities/selector.md` | `46f1263e0dd9ff6dd5c5906f951b82ee43de1a071f38acc0d1281cfa5abd7f57` |
| `entities/show.md` | `3752c4a2bfcd9b4fed436108aa83c87dc202aabe7d036c0abda479b0debf2e9a` |
| `compiler/eval/bindings/value_methods.md` | `dec025c9b94028e845d35e02ababe6f6ce4a6c61c4fc8e918584878567985a73` |
| `compiler/eval/selector_matching.md` | `e7de0268bc35aaf9b8f573a5fffeb3ea2a55e1b661c6ac3d83bd2828379b3140` |
| `compiler/eval/operators/equality.md` | `b304c33c9ff064da2a2bd24ffee9946a2980729c8a353275bfb784e8ca3b5730` |

## Resultado da revisão focal

R1-A resolvido na redação: `selector_matching.md` P1339 limita o novo
comparador e projetor às cadeias cuja base é NativeElement. Where legado
sem essa base conserva comparador e projeção antecedentes. A instrução já
não exige simultaneamente uma troca universal de igualdade e preservação
do comportamento legado.

R1-B resolvido na redação: `value_methods.md` P1339 explicita que somente
where sucede o limite AST de P1284; não transfere outras famílias para esse
owner. As outras minutas mantêm os hashes de R1, já examinados.

Não encontrei objeção pendente à apresentação da extensão pública concreta
para aprovação ADR-0127: `entities::selector::Selector::Element` com
`function: Func` e `fields: EcoVec<(EcoString, Value)>`, e
`entities::show::Selector::NativeElement(Func)`. Grupo vazio, ordem,
identidade, igualdade da linguagem, morfologia de repr e fronteira de
matching estão suficientemente discriminados para o dono aprovar ou
rejeitar esses carriers.

Este parecer não aprova a implementação, não demonstra cobertura completa
de builtins/campos e não certifica paridade. Os L0 proprietários pendentes de
repr/query/introspector/counter e dispatch/rules devem ser atualizados antes
de seus consumers; contrato, gates discriminatórios, selo e materialização
continuam posteriores à decisão humana. V5 esperado e os resultados de
V15/V26 são responsabilidade dos recibos do autor; não os reexecutei nem os
transformo em veredito de implementação.
