# Passo 1496 — materializar o catálogo pinado de símbolos

**Estado:** PLANEADO. Pré-requisito: P1495 com L0 vigente e decisão de compatibilidade; se houver opção incompatível, confirmação humana registrada.

**Entrada ratificada por P1495:** preservar os doze aliases legados como
extensões (`join{,.r,.l,.l.r}` e `{gt,lt}.tri{,.eq,.eq.not,.not}`)
nos três acessos. `math.join` mantém warning histórico silencioso;
`sym.join` e bare mantêm aviso; `spacebar` novo deve avisar em todos.
Os cinco valores `tack.*.double` foram classificados como correção
de tabela para o pin novo. A sonda
`lab/parity/tools/p1495_symbols_probe.py` mediu 198 casos:
162 RED de paridade e 36 extensões preservadas. Receita, SHA e owners
estão em `00_nucleo/diagnosticos/typst-p1495-auditoria-catalogo-simbolos.md`.
Não há autorização para remover aliases; se o desenho mudar, reabrir
o gate ADR-0127 antes de código. Nas 16 sondas bare, glifo e warning
convergentes com carrier público ainda diferente contam como
`TEXT_MATCH_CARRIER_DIFF`, não como `MATCH` integral; P1496 não
muda o carrier.

## Medição anterior à decisão

P1489 mediu `sym.chess` ausente, `sym.not.inv` ausente e `sym.tack.r.double` com glifo antigo, além dos seis módulos-pai e 12 famílias com metadados divergentes. Fonte vanilla: `codex` git `2e43ab9`; cristalino: `=0.3.0`. O adaptador recursivo está em `01_core/src/compiler/stdlib/sym.rs:575-646` e o espelho matemático em `01_core/src/compiler/stdlib/structural/math.rs:3123-3131`. Reexecutar as sondas P1495 contra binários pinados e registrar HEAD/worktree/hash antes de alterar código.

## Sequência de materialização

1. Somente após L0 e eventual gate, pinar `codex` exatamente na revisão auditada (ou dados equivalentes explicitamente pinados), atualizar `Cargo.lock` de modo controlado e com um job. Não seguir branch móvel. Confirmar que `emoji.txt` e os comportamentos de `emoji` permanecem invariantes.
2. Executar os testes RED de P1495 antes da implementação. Integrar o catálogo no owner `sym`, validar recursão, escolha de variantes e valores Unicode; conferir `math` como espelho e a precedência de identificadores bare. Manter as rotas legadas conforme a decisão P1495, sem remoção silenciosa ou assimetria entre `#sym.join` e `$join$`.
3. Confirmar GREEN de `P1489-B-002`, `P1489-B-003` e `P1489-B-004`; só então atualizar seus `expected_state` para `MATCH` com rebaseline/proveniência. Para os seis pais, verificar os 42 descendentes; para as 12 famílias, comparar valor/modificadores/`repr` em `sym` e `math`. Testar `emoji`, warnings de depreciação, extensões deliberadas e conflitos com a tabela matemática.
4. Se mudar `eval/math.rs` ou `eval/bindings/field_access.rs`, atualizar antes seus L0 proprietários indicados em P1495. Resselar apenas owners afetados após V15/V26; rodar testes focais, `CARGO_BUILD_JOBS=1 cargo build`, `crystalline-lint .` e `git diff --check`. Usar o binário vanilla já ratificado em P1489; não rebuildar lab ou jobs Cargo em paralelo.

**Gate ADR-0127:** adições/correções de entradas e paridade de valores podem fluir com L0 primeiro + RED→GREEN. Remover alias público, mudar assinatura/campo ou decidir quebra deliberada exige confirmação específica; sem ela, preservar compatibilidade e registrar extra como extensão, não como crédito de paridade. Se a confirmação faltar, este passo fica pendente — não se força GREEN por omissão de testes.
