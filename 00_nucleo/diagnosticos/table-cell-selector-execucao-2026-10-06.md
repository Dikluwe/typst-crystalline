# D08A — seletor bare de table.cell materializado

Estado: construção, forma pública, igualdade/alias, composição e rejeição
por locatabilidade corrigidas no recorte D08A. D08B (show/campos/realização
de células) permanece explicitamente aberto; não se certifica tabela universal.

## Ratificação e proveniência

Humano confirmou o pacote table-cell-selector-2026-10-06.md. Manifesto
ratificado SHA-256 `f80f85c4ac3ab747d20d98ef3f5758c59727bcf5f5d762082660099aef4dcc9c`;
receipt em materialization/table-cell-selector-aprovacao-2026-10-06.json.
Dez L0s completos e núcleo foram promovidos antes de escrever código.
O novo braço público Selector::NativeElement(Func) está coberto pelo gate.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Evidências em
table-cell-selector-2026-10-06 conservam UTC, diff HEAD --stat, status,
conteúdo/hashes das fontes, argv/cwd/ambiente, entradas, outputs e executáveis
congelados por digest. baseline-state, tested-final-code e final-code preservam
estados completos; final-results.json liga as provas e verify.py as revalida.

Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt final SHA-256 `273cedc0f41c30e6927bbe60eaacdead106db1d00ed071e88807917c0cb1d88e`.
Coleta final: 2026-10-06T22:45:14.101027+00:00 a 2026-10-06T22:45:21.810597+00:00.

## Medição antes da decisão

O diagnóstico table-cell-medicao-2026-10-06.md e before.json.gz medem:
selector(table.cell) deve ser cell, .where() deve ser cell.where(:), a comparação
deve ser false. Query/counter rejeitam cell não locatable. Alias real foi medido
em CodeBlock, não pelo binding isolado em outro documento. A fonte pinada
foundations/selector.rs:309–321 distingue grupo ausente/presente e :421–450
valida locatabilidade separadamente. ElementKind não deve ganhar cell indexável;
Element vazio já significa where presente, conforme o L0 vigente.

A variante nova é uma escolha mecânica fechada para preservar observáveis da
linguagem. Não há exigência de vtable/bytes/algoritmo upstream. A distinção do
grupo está explícita na fonte; a inferência de suficiência para D08A foi
submetida aos testes e coletas abaixo, sem generalizá-la à realização D08B.

## Implementação e ajustes internos

Selector agora conserva NativeElement sem grupo where. Só table.cell é
acrescentada às rotas diretas por fn_addr_eq, sem nome/execução de constructor
como reconhecimento; mappings Kind antigos ficam preservados. Repr usa o nome
canônico do catálogo, igualdade da linguagem usa a função efetiva e conserva
a diferença entre bare e filtro explicitamente vazio. Composição clona a folha.

Query/counter validam locatabilidade antes de consultas/observações/Counter e
ancoram os erros no value_span. Rejeição de partial/closure em native_selector
recebe a mesma correção de âncora, após medir os negativos de before. Matching
converte para o NativeElement de show já existente; introspector consome a
identidade representada, em vez de codificar [] para a nova folha. O teste de
consulta programática com equation e snapshot vazio prova essa projeção.

check.log identificou um consumer exaustivo adicional em observation/value.rs,
que usa Self::Element e por isso escapou ao inventário Selector::Element.
Seu L0 foi lido/atualizado antes do código, em fluxo contínuo ADR-0127.
A relação conservadora de observação preserva variante e payload completo de
Func; a igualdade linguística por endereço permanece no owner equality.
Esse reparo não altera contrato público/fase além do braço já ratificado.

Onze owners foram ressellados exclusivamente: os dez ratificados e o owner
de observação. seals.json conserva todos os hashes, grafo V15/V26 válido
antes do resselo e lint integral válido depois. Receipt de preservação: zero
alteração externa nas fontes/L0 desde baseline, além da família e seu núcleo.
O manifesto original de aprovação permanece imutável.

## RED→GREEN e validação

A primeira fixture red.log tinha referência inválida ao reexport de repr e
não é prova RED. valid-red.log reconstrói o owner foundations/selector de
baseline com a fixture corrigida e falha na rejeição de bare cell; o arquivo
implementado foi restaurado depois do teste. span-red.log é outro RED válido:
partial retorna span detached em vez do valor individual. Todos passaram em
core.log após o código e resselo. Há sentinelas para alias com nome Rust
diferente, função homônima, projeção no introspector e observação do payload.

- D08A: 21/21 controles integrais coincidem no binário final.
- D08B: dois controles explícitos de show seguem negativos, separados do
  certificado D08A (21/23 na coleção completa).
- D07: 29/29 e 14/14 em matrix-morfologia-2026-10-06/final-table-cell*;
  os conjuntos são distintos e não são somados a outras coletas repetidas.
- D05: 29/29 em equation-where-2026-10-06/final-table-cell.json.gz.
- Query: 36/36 em query-compilada-2026-10-06/final-table-cell.json.gz.
- D06: 38/41 em math-bare-2026-10-06/final-table-cell.json.gz, preservando
  os negativos anteriores root-render, raw-eval e raw-mat-eval.

Testes finais: core 6.582 passaram e 5 ignorados; infra 1.194 passaram;
CLI query 10 passaram. Builds debug/release concluídos. git diff --check limpo.
Linter: zero erros, 532 warnings e 1599 notes.

## Continuação D08B

Show direto ainda rejeita cell; where x:0 ainda não transforma. Posição,
células implícitas/padding, projeção de body/campos, headers/escopo e guards
precisam de medição e L0 próprios antes de materializar. A confirmação D08A
não ratifica mudança de fase nem callback/contrato novo para essa realização.
