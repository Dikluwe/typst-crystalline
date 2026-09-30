# Inventário de atomização comportamental do L1 — 2026-09-30

## Escopo e proveniência

Varredura estática de todos os `01_core/src/**/*.rs` no working tree não commitado de 2026-09-30 18:25:36 UTC, HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. A captura [JSON comprimida](inventario-atomizacao-l1-2026-09-30.json.gz) contém a lista completa de arquivos, linhas, `@prompt`, primeiro `#[cfg(test)]`, SHA-256 dos principais candidatos, `git status --porcelain=v1 --untracked-files=all` e `git diff HEAD --stat`. Foram encontrados **577 arquivos Rust** em L1. Linhas incluem testes e comentários; não são métrica de performance nem prova de facilidade da extração.

A seleção abaixo cruzou tamanho, fronteiras de comportamento (`fn`, registry, dispatcher), filhos já existentes e L0 vigente. É uma lista de oportunidades de pesquisa e leitura, não autorização para alterar contratos. Em cada extração: atualizar o L0 proprietário antes do código, manter `1:1` (ADR-0129), conservar dispatch estático (ADR-0109), testar regressões e ressellar hashes. Partilha entre prompts exige Núcleo Tekt quando a mesma norma pertencer a mais de um owner.

## Fila imediata em `compiler/stdlib/structural/math.rs`

Os nove filhos produtivos já separados são `accent`, `cancel`, `cases`, `class`, `equation`, `frac`, `matrix`, `root` e `vector`. O parent ainda tem comportamento produtivo até aproximadamente a linha 1685; as seções de testes começam depois. Os intervalos são do snapshot acima.

| Ordem | Unidade candidata | Código atual | Limite proposto / cuidado |
|---|---|---|---|
| 1 | `underline` | `48–80` | Cast e constructor próprios; binding fica no catálogo. |
| 2 | `lr` e `mid` | `82–203` | Dois owners de feature; `lr` tem size e cast de conteúdo, `mid` tem construção própria. Preservar bridges para eval. |
| 3 | `overbrace`, `underbrace`, `underbracket`, `underparen`, `undershell`, `overbracket`, `overparen`, `overshell` | `205–603` | Oito owners por elemento, com helper compartilhado apenas se existir invariante real; preservar lowering e diagnósticos. |
| 4 | `primes` | `605–654` | Parser/constructor isolado. |
| 5 | `overline`, `norm`, `round`, `abs`, `stretch` | `656–861` | Cinco owners por comportamento; manter bridges e casts de size de cada um. |
| 6 | `attach` | `863–974` | Cast triestatal e carrier. Atualizar o pin do Núcleo `math-attach-slot-presence` e a norma remanescente no parent com especial cuidado para V26. |
| 7 | `binom` | `976–1043` | Constructor e validação isolados; bridge para eval. |
| 8 | `underover` | `1079–1149` | Nativa global e carrier; reexport direto, sem tocar no layout. |
| 9 | `op` | `1151–1223` | Precedência arg explícito → style chain → default requer teste de contexto; `op_value` no catálogo (`1225–1229`) pode permanecer lá. |
| 10 | `scripts` e `limits` | `1231–1326` | Dois constructors de override; preservar explicitude de `inline`. |

`vanilla_type_name_class` (`1055–1062`) é helper compartilhado por várias unidades: mantê-lo no parent enquanto houver múltiplos consumidores. `make_math_module` (`1328–1685`) é sobretudo catálogo e dispatch estático; seu tamanho isolado não justifica outro runtime ou registro dinâmico. Os testes P1291/P1283/P1472 no parent podem ser separados como módulos **de teste**, depois da lógica produtiva, para navegação; isso não cria oportunidade algorítmica.

## Outros candidatos produtivos de alto retorno de leitura

| Prioridade | Arquivo atual | Fronteiras comportamentais verificadas | Preparação |
|---|---|---|---|
| Alta | `compiler/eval/rules.rs` (4.754 linhas) | `eval_set_rule` por target/propriedade (`1631–3555`), `eval_show_rule` (`3570+`), realização de parágrafos (`656–855`), interceptação (`1370–1628`), diagnósticos PDF (`121–413`). `textual/` já é filho. | Individualizar L0s por target/fase sem mudar fase do pipeline; manter dispatcher e ordem das regras. |
| Alta | `compiler/stdlib/collections.rs` (3.542) | Métodos de `array` (`725–1656`), `dict` (`1659–1735` e `2277–2346`), `str`/char (`1738–2276`), `bytes` (`180–373`), `arguments` (`2349–2456`); dispatch de instância/estático (`66–179`, `374–723`) pode ficar magro. | Separar owners por tipo e preservar calling convention self-first, spans e identidade da função. |
| Alta | `compiler/stdlib/layout.rs` (2.966) | `page` (`479–846`), `grid` (`242–478`), `stroke` (`1005–1067`, `2369–2807`), `block`/`box`/`stack`/`columns` (`1533–2264`), espaçamentos e breaks (`1282–1532`, `2808+`). | Extrair por constructor e manter casts compartilhados na camada stdlib. |
| Alta | `compiler/introspect.rs` (5.724, inclui testes) | Conversão de referências bibliográficas (`67–~285`), materialização de outlines (`922–988`), tempo (`989–1375`), população de tags (`1376–1692`), caminhada (`1897–2485`). Já existem `from_tags`, `fixpoint`, `heading`, `labelled`, `locatable`. | Conferir se cada bloco restante já está delegado; preservar índice de localização e fixpoint. |
| Alta | `compiler/stdlib/html.rs` (3.233, testes após ~2117) | `html.frame` (`60–259`), catálogo de tags, nativas tipadas (`1399–1466`), casts de atributos (`1467–2033`), `html.elem` (`2034–2116`). | Seccionar por frame, typed attributes, generic elem; L0 atual é extenso e precisa divisão cuidadosa. |
| Alta | `compiler/eval/repr.rs` (2.760, testes após 1630) | Representação de números/unidades (`202–329`), paint/cor/gradiente (`330–570`), conteúdo matemático (`571–680`, `1374–1467`), `repr_content` (`681–1373`), seletores e containers (`1468–1629`). | Separar por domínio sem alterar morfologia textual nem ordem/canonicalização. |
| Alta | `compiler/stdlib/loading.rs` (3.458, testes após 1453) | Encoders JSON/TOML/YAML (`40–646`), decoders JSON/YAML/TOML/CBOR/XML/CSV (`651–1095`), leitura e named CSV (`1101–1452`). | Owners por formato; manter I/O somente pela abstração `World` e diagnósticos de origem. |
| Média | `compiler/eval/math.rs` (1.671) | Avaliação de callee/argumentos (`130–599`), `eval_math_expr` (`600–1554`), parsing de delimitadores e opções de vec (`1555–1671`). | Separar passos algorítmicos dentro da mesma fase eval; a expressão central pode continuar como dispatch exaustivo. |
| Média | `compiler/stdlib/calc.rs` (4.512, testes a partir de 1573) | Funções básicas (`133–600`), trig/log (`601–995`), aritmética inteira/combinatória (`996–1484`), `erf` (`1485–1549`); catálogo (`56–132`). | Separar famílias e testes sem confundir tamanho total com código de produção. |
| Média | `compiler/stdlib/shapes.rs` (1.676, testes após 1332) | Rect/square/ellipse/circle, line, polygon e curve têm construtores e casts próprios. | Owners por shape; parsers comuns de paint/stroke continuam compartilhados no stdlib. |
| Média | `compiler/stdlib/color.rs` (1.386, testes após 967) | Ajustes light/dark/mix/saturate/rotate, componentes/espaço, spot colorant/tint. | Separar métodos de cor e spot mantendo o dispatch e casts comuns. |
| Média | `compiler/stdlib/gradients.rs` (1.058) | Construtores linear/radial/conic, métodos sample/samples/sharp/repeat, parsing de stops/space. | Separar constructor e operações de instância; preservar domínio de ColorSpace. |
| Média | `compiler/stdlib/structural/table_grid.rs` (923) | Table, cell, header, footer e variantes grid. | Individualizar comportamento de header/footer/cell sem duplicar o cast de inset/align. |
| Média | `compiler/eval/bindings/value_methods.rs` (1.335) | Métodos state, color, counter, selector, element. | Owners por tipo; verificar interações com field access. |
| Condicional | `compiler/math/layout/mod.rs` (3.159) | Lowering de decorações (`72–232`), geometria comum, flatten/style/default (`2209–2733`). Já há filhos por `accent`, `attach`, `cancel`, `cases`, `frac`, `matrix`, `root`, `vec` etc. | Extrair somente corpos com comportamento próprio; manter dispatcher fechado e estado do layouter conforme L0 `_comum`. |
| Condicional | `compiler/layout/sequence.rs` (1.156) e `compiler/layout/grid.rs` (1.698) | Preflight/segmentação de sequências e construção de grid. | São famílias algorítmicas reais; medir interfaces e estado compartilhado antes de dividir. `grid_placement.rs` já existe. |
| Condicional | `entities/gradient.rs` (2.216, testes após 1075) e `entities/color.rs` (1.896, testes após 1142) | Interpolação por espaço e conversões de cor. | Algoritmos puros podem ganhar arquivos próprios na mesma camada; evitar mover render para entidades ou alterar tipos públicos. |

## Não priorizar só pelo tamanho

`compiler/layout/tests.rs` (23.369), `compiler/eval/tests.rs` (22.315) e `compiler/math/layout/tests.rs` (9.699) são principalmente suítes de teste. Podem ser organizadas por feature por ergonomia, em tarefa distinta. `compiler/layout/mod.rs` (2.758) já atua como owner de estado, checkpoints, dispatcher e façade; `entities/content/construction.rs` agrega constructors curtos de dados. Sua fragmentação só vale para uma responsabilidade concreta demonstrada, não para reduzir LOC. `compiler/eval/call_dispatch.rs`, `compiler/eval/mod.rs` e `compiler/eval/bindings/field_access.rs` também misturam coordenação e testes; auditar os blocos produtivos antes de abrir novos owners.

## Ordem recomendada

1. Terminar as unidades de `math.rs` em fatias pequenas e com L0 próprio; `underline`/`lr`/`mid` são as próximas de menor acoplamento. Tratar `attach` após preparar o pin do Núcleo Tekt.
2. Passar para `collections` e `stdlib/layout`, onde as famílias são claras e o ganho de localidade é maior.
3. Atacar `eval/rules` e `introspect` em fatias medidas, porque concentram estado, ordem e fixpoint.
4. Continuar em `loading`, `html`, `repr`, `calc`, shapes e color.
5. Revisar os blocos condicionais de layout e entities apenas após confirmar que ainda há comportamento de feature fora do owner.

Nenhuma das oportunidades acima demonstra ganho de runtime. A vantagem pretendida é localizar cada algoritmo e facilitar comparação, substituição experimental e testes por comportamento.
