# Predicados, consultas temporais e construção de fixtures

Este lote remove 34 diagnósticos de Clippy em 23 consumers sem mudar os
resultados contratados, defaults de produto ou APIs externas. A consulta
temporal passa a procurar a última entrada elegível diretamente a partir
do fim do histórico; as demais alterações simplificam expressões e fixtures.

## Proveniência e medição anterior à decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. O diretório `predicados-fixtures-ci-2026-10-13/` conserva
`before.json.gz` com fontes, owners, referências, diagnósticos selecionados,
UTC, status e `git diff HEAD --stat`. O inventário de entrada contém 139
diagnósticos distintos de core lib/testes, medidos no lote de arredondamento.
Os spans foram conferidos contra as fontes correntes antes da edição.

`pre-validation.json.gz` e `after.json.gz` delimitam as fontes e hashes
verificados; elas permaneceram inalteradas durante a validação. Comandos,
horários e resultados ficam em `validation-results.json` e
`rustdoc-results.json`, reunidos com inventários, diffs e receitas em
`evidence.json.gz`. A ferramenta de validação é o linter publicado pinado
em `49c6bb0e584104a0c89270431d8402f5c6c61150`.

| Categoria selecionada | Diagnósticos removidos |
| --- | --- |
| field_reassign_with_default | 13 |
| needless_lifetimes | 5 |
| assign_op_pattern | 4 |
| manual_is_multiple_of | 3 |
| manual_range_contains | 3 |
| match_like_matches_macro | 2 |
| unnecessary_map_or | 2 |
| double_ended_iterator_last | 2 |

## Consultas temporais

As fontes anteriores `entities/counter_registry.rs:176-179` e
`entities/state_registry.rs:80-83` percorriam um Vec com `filter().last()`.
O predicado apenas lê `order_key`: não faz mutações, I/O ou callbacks.
`rfind` devolve a mesma última entrada elegível na ordem do Vec, incluindo
None se nenhuma corresponder. A chave continua sendo a ordem causal
registrada, com fallback numérico somente para Locations não observadas.

Os dois owners receberam essa decisão antes do código. O núcleo pinado
`realization-order.toml` permanece intacto. O histórico não foi reordenado,
nenhum update foi removido e a complexidade continua O(n) no pior caso.
Não foi medido ganho de tempo. O teste
`single_evaluation_counter_follows_causal_order` e os testes temporais dos
registries passaram. Isto não redefine o algoritmo de realização do produto.

## Expressões equivalentes

Os dois matches booleanos passam a `matches!` preservando exatamente as
variantes e nomes de métodos. As sugestões estavam marcadas MaybeIncorrect
e foram conferidas manualmente. Em segmentação CJK, `is_some_and` preserva
false para None; em glifos estendidos, `is_none_or` preserva true para None.

As três verificações de paridade usam `is_multiple_of(2)` sobre inteiros
sem sinal, mantendo o tratamento de zero. O toolchain permanece Rust 1.92.0.
Os intervalos conservam os extremos inclusivos/exclusivos. Em `calc.quo`,
a rejeição antecipada de não finitos permanece antes do intervalo; NaN não
passa a ser aceito. Os quatro `+=` operam sobre Pt; Add e AddAssign foram
inspecionados em `entities/layout_types.rs:49-73` e somam os mesmos f64.

Os cinco lifetimes elididos pertencem a helpers privados ou pub(super),
cada qual com uma única referência de entrada que determina a saída.
Não foi alterada a relação de empréstimo nem adicionada uma API pública.
As demais cláusulas dos owners permanecem iguais; em 21 deles somente o
Hash do Código mudou. Não se afirma nova paridade bilateral neste lote.

## Inicialização das fixtures

Os valores antes atribuídos logo depois de Default agora aparecem nos
inicializadores de struct. As implementações locais de Stroke,
MathMatrixOptions e PageConfig foram lidas; o Formatting usado é o tipo
de hayagriva 0.10.1, com Default derivado, cuja fonte e hash constam de
`external-reference.json.gz`.

O campo explicit de MathMatrixOptions continua explicitamente zerado antes
da cascata de estilos. Os testes de stroke preservam thickness, paint, cap
e os bits specified; as mutações posteriores desses bits permanecem.
Os clones que discriminam propriedades de stroke nos testes não foram
removidos. Os testes CSL continuam verificando a mesma morfologia.

Nos dois testes de PageConfig, `..PageConfig::default()` conserva a margem
inicial calculada para A4 e substitui somente as dimensões da fixture.
`refresh_auto` continua sendo necessário para obter a margem esperada.
A segunda troca de largura/altura no teste de mudança de dimensões permanece
posterior à primeira verificação. Portanto não se substituiu o recálculo
por uma margem já pronta nem se enfraqueceu a expectativa.

## Validação final

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes com -D warnings | 139 → 105 diagnósticos distintos; ainda falha |
| Categorias selecionadas | 34 removidos; demais contagens preservadas |
| Check workspace com todos os targets | passou |
| Suíte core | 6.705 passaram, zero falhas, cinco ignorados |
| Registro de testes | 6.710 nomes, idênticos antes/depois |
| Build, formatação e diff check | passaram |
| Rustdoc workspace, sem dependências, -D warnings | passou |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter arquitetural completo | zero erros, 380 avisos, 1.614 notas |

Nenhum teste foi removido nem foi introduzida supressão de lint. Este lote
não declara o CI completo: os diagnósticos restantes de core e das demais
crates continuam pendentes, assim como a execução remota da matriz de
release e os itens funcionais da auditoria de produção.
