# P1327 — auditoria independente de escopo pré-candidato

Regime A/B, executado sem atestação de isolamento. Revisor `/root/p1327_review`:
leitura dos owners e baseline; escrita limitada a `p1327-review-*`; nenhuma
alteração de produto, contrato ou oracle. Ambiente compartilhado não constitui
isolamento técnico. Skill Tekt e ambas as referências operacionais lidas;
nenhuma ADR com `segrega` foi localizada em `00_nucleo/adr/*.md`.

## Proveniência

Inspeção em 2026-09-09T10:40:01Z, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
`git diff HEAD --stat` nesse instante:

```text
 .../prompts/compiler/eval/bindings/field_access.md | 187 +++++-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |  53 +-
 00_nucleo/prompts/compiler/eval/tests.md           |  39 +-
 00_nucleo/prompts/compiler/stdlib/loading.md       | 198 +++++-
 00_nucleo/prompts/wiring.md                        |  56 +-
 01_core/src/compiler/eval/bindings/field_access.rs | 731 ++++++++++++++++++++-
 01_core/src/compiler/eval/call_dispatch.rs         |  92 ++-
 01_core/src/compiler/eval/tests.rs                 |  10 +-
 01_core/src/compiler/stdlib/loading.rs             | 466 +++++++++++--
 04_wiring/src/main.rs                              |  30 +-
 10 files changed, 1778 insertions(+), 84 deletions(-)
```

SHA-256 das entradas inspecionadas antes do candidato:

| Arquivo | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/eval/modules.md` | `bba521089d35fcd8db850a60a790309105b73786c818f45ee9a71872366f1577` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `8205c6fc1f5ae0a5546b8ea0754913343134cf02fbe31565c6122324b1cb58e8` |
| `01_core/src/compiler/eval/modules.rs` | `a855f51063310ec00a1106e6447f7a6a3e753ef8ccbb5e17f37a2319da1c3594` |
| `01_core/src/compiler/eval/tests.rs` | `4d64677e7a0963e5c45b8ed4e284ef985549650b71dbd2f48e365fa1dafb7aeb` |
| `lab/typst-original/crates/typst-eval/src/import.rs` | `2c2119823006a7ec1be06fa9a060c91f7a89d8199b244c0097b818b067c03016` |

## Medição anterior à decisão

`modules.rs:117-250` concentra resolução, validação e binding do import.
`Value::Module` é a única categoria não literal aceita; a resolução e
`bare_name` falham antes do bloco que define o binding. O bloco `None` do
match de imports (`:218`) define o módulo sem emitir warning. O sink local
dos arquivos importados é reborrowed do caller (`:84-105`).

Vanilla ratificado `a51e02804`, `typst-eval/src/import.rs:82-105`, emite
`this import has no effect` em `source_expr.span()` somente no bare import
sem rename, depois da fonte validada, quando a AST é Ident. Renames usam
outro ramo e outra mensagem (`:64-73`). Strings/paths substituídos possuem
regras próprias que não se confundem com Ident de Module.

O L0 `modules.md:89-94` exclui expressamente os dois warnings do P1305;
`tests.md:392-396,464-466` preserva sua ausência no baseline e exige uma
exceção individualizada. Portanto implementar exige sucessão normativa
explícita, antecedendo código, somente do warning sem efeito. A exclusão
do warning de rename deve permanecer.

O cohort `module-bare-import-warning` de
`p1322-classification-selection-r2.json` identifica esse mesmo owner e
as testemunhas globais bare e alias. `p1326-closure.json` registra gates
anteriores com exit zero e limita a conclusão a diagnósticos Closure/With;
não fornece aceitação P1327.

## Decisão e owner set

Owner produtivo completo deste fragmento: apenas
`01_core/src/compiler/eval/modules.rs` ↔
`00_nucleo/prompts/compiler/eval/modules.md`.
Owner adicional exclusivamente test-only para sucessão das expectativas:
`01_core/src/compiler/eval/tests.rs` ↔
`00_nucleo/prompts/compiler/eval/tests.md`.
Não há evidência que exija alteração em sink, entidades, wiring, parser,
World, carriers, dispatcher ou lab.

ADR-0127, seção 2, fluxo contínuo ponto 3, aplica-se: diagnóstico medido
de paridade, sem assinatura/entidade pública, modo, flag, default deliberado,
fase ou incompatibilidade nova. A exceção explícita para paridade impede
classificar todo warning visível como novo default. L0-first, resselo,
RED→GREEN e revalidação continuam obrigatórios.

## Sucessão exata de testes legados

| Teste em `eval/tests.rs` | Expectativa que deve suceder |
|---|---|
| `p1305_oracles::p1305_global_bare`, `:19122` | Chamada de `observe` para `{ import std; ... }`, hoje exige `side.is_empty()` no helper `:18237`; exigir exatamente um warning em `std`. |
| `p1305_oracles::p1305_global_alias_bare`, `:19139` | Mesma ausência histórica para `import renamed`; exigir exatamente um warning no Ident `renamed`. |
| `p1305_oracles::p1305_import_binding_negatives_and_dynamic_spans`, `:19356` | Apenas os dois primeiros casos (`import std; global`, `import renamed; global`) devem conter um warning além do erro posterior preservado; os outros quatro continuam sem laterais. |
| `p1306_oracles::p1306_existing_lookup_and_repr_all_profiles`, `:19560` | Apenas `global-bare-positive` e `global-bare-alias-positive`, `:19570-19571`, sucedem ausência lateral; demais casos ficam exatos e vazios. |

Os valores, erros primários, hints, spans dos erros, perfis e repetição/ordem
não mudam. O sucessor precisa verificar cardinalidade, Warning, mensagem
integral, hints/trace vazios e span UTF-8 do Ident. Não tornar `observe`
genericamente tolerante, filtrar warnings, remover assert ou computar
expected com o candidato. As seis ocorrências de `import Ident;` encontradas
por `rg -n '\bimport [A-Za-z_][A-Za-z_0-9-]*;'` ficam nesses quatro testes.
Essa é medição estática; suíte completa ainda necessária para excluir
rotas construídas dinamicamente ou importadas em outros testes.

## Fronteiras que invalidariam o recorte

- Sucesso refere-se ao import: erro posterior no programa não elimina seu
  warning. Fonte desconhecida, tipo inválido ou erro de bare dinâmico não
  devem ganhar esse warning.
- Aplicar a qualquer Ident de Module, inclusive builtin, alias, sombra e
  módulo ordinário, sem discriminar pelo texto `std` ou nome público.
- Bare field access, literal-file, `as`, items e wildcard não recebem o
  novo warning. `import x as x` conserva dívida explícita de rename.
- Conservar binding lexical, valores, repr, visibilidade e avaliação única.
- Incluir spans do Ident com UTF-8 e origem importada, warning em erro
  posterior, escopo interno e repetição; não inferir ordem/cardinalidade
  acumulada apenas de casos globais triviais.
- Não promover paridade geral de import: fontes Func/Type/path/string
  dinâmicas não suportadas continuam fora. Claims de alvo HTML separado
  requerem execução real desse target, além de uma feature habilitada.

Veredito preliminar: recorte viável em fluxo contínuo com os dois pares
de owners descritos. Não é aceitação de candidato nem selo de refinamento.
