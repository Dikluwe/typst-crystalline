# L0 — compiler/eval/bindings/value_methods/element_where
Hash do Código: a0b25fb7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:923f1c9e4c2fd5934316ae2eef3dbb461c3bef3009de1d8931d951e793dda342
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:32890de1247262b54fa39d7ae1d640a068774a28cf1fa1c8890d0508d999af51
- 00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml sha256:19eb838b3e56711fa151d416bd24394a0a6f5c8286bb9ab2d0dd9bc542b4aee7
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:a37779d03a4ba0f16ebcc5919477bfdcf783021511a28cf8c9203c67cd64a9e5

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/bindings/value_methods/element_where.rs`. Extração interna medida no HEAD `ce27af795` e árvore limpa em 2026-09-30 23:12:44 UTC. A fonte é o consumer pai desta unidade; a extração não é medição bilateral de paridade nem prova de desempenho.

A lógica extraída estava em `01_core/src/compiler/eval/bindings/value_methods.rs:684-1209` no HEAD indicado. A classificação é extração interna de comportamento, não nova decisão de paridade.

## Contrato

Own native element identity, function.where, field catalog and locatability checks. Preserve typed identity, errors and selector morphology. Não alterar contrato público, fase do pipeline, comportamento por defeito ou compatibilidade.

## Aceitação

Os testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho da divisão estrutural.

## D05 — equation.where, correção interna de paridade

### Medição antes da decisão

Coleta equation-where-2026-10-06/before.json.gz registra pin
9f2b6e8715237cb086899a42873660fe744622e8, binários SHA-256, UTC, HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b e diff HEAD --stat/estado exato.
Tekt rejeita equation.where como função não-elemento; vanilla aceita catálogo
público, filtros block/label/body/numbering/alt, alias, composição e show.
Vanilla rejeita campos size/missing, posição, closure e aplicação parcial.
Fonte upstream: foundations/func.rs:433–462 valida field_id e mantém grupo;
math/equation.rs:58–170 separa campos públicos dos internos.
Tekt element_where.rs:63–490 omite equation; selector_matching.rs:269–310
omite Equation no matching nativo. O getter block/body e snapshot já existem.

### Decisão e aceitação

Acrescentar math.equation à tabela estática por identidade do constructor
native_math_equation já existente, via bridge interno de stdlib. Catálogo público:
block, numbering, number-align, supplement, alt, body; label continua campo
comum permitido. Equação é locatável. size e demais campos internos não são
aceitos. Preservar mensagem equation (sem prefixo math), grupo ordenado vazio,
valores não tipados dos filtros, validação de args e aliases por ponteiro.
Closures, homônimos de função e aplicação parcial continuam não-elementos.
Não mudar assinatura, enum, API pública ou fase; não executar constructor.

Fluxo contínuo ADR-0127: correção de tabela interna, sem novo contrato público
ou fase. RED→GREEN dos consumers e revalidação bilateral dos controles;
preservar D04 e não realizar math bare/D06 ou morfologia de matriz/D07.

## D08A — seletor bare de table.cell (PROPOSTO, gate ADR-0127)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
UTC, diffstat, entradas, comandos, hashes das fontes e executáveis em
00_nucleo/diagnosticos/math-residuos-table-cell-2026-10-06/{before,table-additional-before,table-aliases-before}.json.gz.
Vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8.
selector(table.cell) devolve cell, where() devolve cell.where(:), a comparação
é false, query/counter rejeitam cell não locatable. Alias verdadeiro medido
por CodeBlock conserva esses resultados. A fixture anterior que definiu alias
apenas no documento não disponibilizava a variável à expressão eval e não
prova alias positivo; table-aliases-before corrige a fixture.

Fonte: lab/typst-original/crates/typst-library/src/foundations/selector.rs:309–321
distingue dict None de Some; :421–450 valida locatabilidade separadamente.
No Tekt, entities/selector.rs:34–37 só representa grupo Element presente;
stdlib/foundations/selector.rs:69–77,348–354 omite a nativa cell; os demais
limites atuais e hashes estão em d08-owner-audit.json.

Classificação: formato, identidade e rejeição de consulta são linguagem;
o novo carrier fechado é mecânica escolhida para preservar esses observáveis.
A distinção do vanilla é explícita na fonte; não se infere intenção apenas
da saída. Inferência: a nova variante basta à fatia de seleção/locatabilidade;
a hipótese será refutada por um caso público cuja ausência de grupo ou
identidade não caiba nela. Não se presume correção de realização de células.

### Contrato materializável somente após ratificação

Conservar eval_element_where e a tabela native_element_fields: table.cell já tem catálogo e locatable=false. validate_element_locatability visita também Selector::NativeElement pela função efetiva, rejeitando cell com `cell is not locatable` no span recebido; propagar por And/Or/Where/Within/Before/After como hoje. Não converter NativeElement em Element vazio e não validar locatabilidade na construção de where.

### Fronteira incompleta e aceitação

D08A cobre apenas carrier, construção, repr, igualdade, composição e
locatabilidade. D08 permanece incompleto: D08B completa show direto, projeção
de body/campos e preparação de células explícitas/implícitas, incluindo
posições, padding, spans, header/footer e escopo lexical. D08B precisa de
seus L0s medidos e de gate próprio caso mude contrato ou fase; esta ratificação
não o autoriza. Não esconder negativos D08B ou declarar paridade universal.

Após ratificação: promover este conjunto atomicamente; escrever testes
linguísticos e verificar RED válido antes do código; materializar com
despacho estático e Rust seguro; validar 1:1/V26 antes de ressellar somente
owners deste manifesto; build, suíte core e linter sem erros. Revalidar
selector cell bare/where/alias, igualdade true/false, query/counter diretos
e compostos, função homônima, partial/closure; preservar D05, D07 e query.

## Completar projeção da caption existente — 2026-10-17

Medição anterior à decisão: caption-shape-before.json e caption-after.json em
00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17 fixam fontes, UTC,
binários, HEAD/working tree de validation-result.json e pin
9f2b6e8715237cb086899a42873660fe744622e8. Projeção de caption já produz o
carrier, mas sua CLI imprime só func:figure.caption; body/getters/fields estão
ausentes. json.encode tem o mesmo defeito. Metadata de kind rejeita a função
de caption válida. O pin em model/figure.rs:510 declara caption Locatable e
em :511-594 declara position,separator,body e kind/supplement/numbering/counter
sintetizados públicos, com figure_location interno. Não sintetizar esses dados
nesta correção; a função de elemento já existe desde a aprovação P1473.

Responsabilidade exclusiva e aceitação:

Reconhecer native_figure_caption por fn_addr_eq na tabela de elemento: nome
caption; campos públicos position,separator,body,kind,supplement,numbering,counter;
locatable=true conforme a fonte pinada. Excluir figure_location interno. Isso
corrige a classificação da função em kind e where; não materializa síntese nem
índice de captions. FG1 ainda completa esses mecanismos e audita a totalidade
do catálogo; esta correção não certifica todos os callbacks do registry.

Correção interna/paridade em fluxo contínuo ADR-0127. Não há novo campo/trait/
assinatura Rust pública, fase, modo ou default. L0 antes de testes/código, RED
real de getters/metadados/encoder/CLI, GREEN e corpus externo sem relaxar
expectativas de corpo/nome. Revalidar todos os gates da etapa sob 12 GiB, swap
zero, jobs1. Resselo só dos owners após V15/V26. FG1/SY1 permanecem incompletos
conforme follow-ups.md; nenhuma API pendente está autorizada por esta seção.


## Catálogo nativo — lote de identidades já existentes (2026-10-17)

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado. `00_nucleo/diagnosticos/native-element-catalogue-2026-10-17/`
contém `before.json.gz` (estado/diffstat e SHA-256 de todas as fontes),
`inventory.json.gz` (bindings efetivos em oito perfis de features),
`before-measurement.json.gz` (3956 casos de funções com comandos e saídas
bilaterais) e `before-batch.json.gz` (campos, filtro label, desconhecidos e
counter). Cada binário tem SHA-256; o oráculo é o pin
`9f2b6e8715237cb086899a42873660fe744622e8`. A ausência na tabela atual faz
where e figure(kind:) rejeitarem funções de elemento existentes. Não se
confunde função que produz content com identidade de elemento.

`batch-schema.json` conserva os blocos literais, hashes de blobs do pin e
linhas da fonte abaixo. typst-macros/src/elem.rs:410–421 exclui campos
internal/external do field_id; capacidades Locatable vêm da anotação elem,
não da hipótese de que Tagged ou ShowSet implicam locatabilidade.

### Decisão/classificação e responsabilidade exclusiva

Correção interna de tabela/paridade em fluxo contínuo ADR-0127: completar
somente as quinze identidades abaixo. Usar fn_addr_eq com ABI tipada;
math.op mantém ABI NativeWithEngine de seis argumentos, incluindo scopes e
engine, sem invocar o constructor. As demais usam a ABI existente de quatro
argumentos. O nome do Func nunca é discriminador; alias conserva identidade,
função homônima não a adquire. Não usar PartialEq de Func como prova do callback.

| Callback existente | Nome público | Campos selecionáveis na ordem da fonte | Locatable | Fonte no pin |
|---|---|---|---|---|
| `native_title` | `title` | `body` | true | `crates/typst-library/src/model/title.rs:31` |
| `native_document` | `document` | `path`, `format`, `title`, `author`, `description`, `keywords`, `date`, `body` | true | `crates/typst-library/src/model/document.rs:147` |
| `native_divider` | `divider` | nenhum | false | `crates/typst-library/src/model/divider.rs:41` |
| `native_list_item` | `item` | `body` | false | `crates/typst-library/src/model/list.rs:167` |
| `native_enum_item` | `item` | `number`, `body` | false | `crates/typst-library/src/model/enum.rs:230` |
| `native_terms_item` | `item` | `term`, `description` | false | `crates/typst-library/src/model/terms.rs:115` |
| `native_footnote_entry` | `entry` | `note`, `separator`, `clearance`, `gap`, `indent` | true | `crates/typst-library/src/model/footnote.rs:223` |
| `native_flush` | `flush` | nenhum | false | `crates/typst-library/src/layout/place.rs:213` |
| `native_op` | `op` | `text`, `limits` | false | `crates/typst-library/src/math/op.rs:25` |
| `native_curve_move` | `move` | `start`, `relative` | false | `crates/typst-library/src/visualize/curve.rs:174` |
| `native_curve_line` | `line` | `end`, `relative` | false | `crates/typst-library/src/visualize/curve.rs:197` |
| `native_curve_quad` | `quad` | `control`, `end`, `relative` | false | `crates/typst-library/src/visualize/curve.rs:237` |
| `native_curve_cubic` | `cubic` | `control-start`, `control-end`, `end`, `relative` | false | `crates/typst-library/src/visualize/curve.rs:284` |
| `native_curve_close` | `close` | `mode` | false | `crates/typst-library/src/visualize/curve.rs:366` |
| `native_asset` | `asset` | `path`, `data` | true | `crates/typst-bundle/src/format.rs:212` |

A capacidade é metadado de validação: não afirmar que esta tabela materializa
sintetização, índices de consulta, contadores ou renderização desses elementos.
Preservar page no caminho com engine, raw.line dinâmico e todas as entradas
anteriores. Não aceitar With/closure/plugin, não alargar para toda função,
nem alterar campos/traits/assinaturas Rust públicos, default de produto ou fase.

O mesmo callback real pode existir em vários bindings: alias global op e
format.bundle.asset devem acompanhar as identidades, respeitando o registry
atual e as features. Não criar bindings novos. Formatos raiz compartilham hoje
callback e não são legitimados por nomes; as 41 rotas restantes da matriz são
frente posterior de metadados/identidade, não paridade declarada por este lote.
Símbolos e funções indisponíveis no pin são classificados separadamente.

### Aceitação e fronteiras

Testes co-localizados: cada identidade reconhecida sob nome alias, catálogo
exato, label permitido, desconhecido rejeitado com nome público, locatabilidade
coerente para bare/where e With rejeitado. Controles de homônimo não-elemento e
page com engine preservados. RED funcional antes da tabela, GREEN depois.
Reexecutar matriz bilateral e exigir igualdade do estado anterior fora do
conjunto de aliases autorizado, preservando saídas/diagnósticos de todos os
controles. Verificar cada campo e rejeição de desconhecidos contra o pin,
sem exigir igualdade da mecânica Rust. Toda diferença residual fica explícita.

Validar build, suíte workspace, fmt, Clippy estrito, Rustdoc, ownership/V26,
linhagem reversa e linter sob 12 GiB, high10, swap0 e jobs1 sequenciais.
Resselo exclusivamente deste consumer e seu owner depois de V15/V26. FG1,
RM1, SY1 e a proposta RG1 mantêm os seus próprios escopos/gates pendentes.


## Identidade matemática nativa — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado; pin `9f2b6e8715237cb086899a42873660fe744622e8`.
`00_nucleo/diagnosticos/native-math-catalogue-2026-10-17/` fixa o estado
exato em before.json.gz: UTC, diff HEAD --stat e hashes de todos os consumers,
L0 e configuração protegidos. before-measurement.json.gz é reutilização
verificada da matriz final da etapa anterior: fontes e binário Tekt têm
identidade de hash com o baseline atual (baseline-reuse.json), não uma nova
execução. A matriz cobre os oito perfis de features e 3956 casos de funções;
26 rotas math ainda são recusadas em where/kind pelo Tekt e aceitas pelo pin.
before-batch.json.gz mede 227 casos detalhados de campos, exclusões, alias
parcial e counter, com comandos/binários/saídas e seus hashes.

batch-schema.json conserva blocos e SHA-256 das fontes primárias do pin,
file:line e declarações de ABI do Tekt; typst-macros/src/elem.rs:410–421
exclui internal/external de field_id. As anotações elem destas 26 funções
contêm Mathy e não Locatable. Isto é semântica/forma pública da linguagem,
não igualdade mecânica de Func nem inferência de intenção por render.

### Responsabilidade exclusiva e decisão

Completar a tabela estática native_element_fields com os 26 callbacks
abaixo, usando fn_addr_eq tipada à ABI existente de quatro argumentos. Todas
as entradas têm locatable=false. Reconhecer a mesma função sob alias; não
usar Func.name/PartialEq, tipo de Content retornado, strings de registry nem
execução do constructor. A tabela é exclusiva deste owner; as fachadas só
transportam os callbacks originais por reexport interno direto.

| Callback | Nome público | Campos selecionáveis, na ordem do pin | Fonte no pin |
|---|---|---|---|
| `native_accent` | `accent` | `base`, `accent`, `size`, `dotless` | `crates/typst-library/src/math/accent.rs:33` |
| `native_math_attach` | `attach` | `base`, `t`, `b`, `tl`, `bl`, `tr`, `br` | `crates/typst-library/src/math/attach.rs:20` |
| `native_math_binom` | `binom` | `upper`, `lower` | `crates/typst-library/src/math/frac.rs:134` |
| `native_cancel` | `cancel` | `body`, `length`, `inverted`, `cross`, `angle`, `stroke`, `background` | `crates/typst-library/src/math/cancel.rs:18` |
| `native_math_cases` | `cases` | `delim`, `reverse`, `gap`, `children` | `crates/typst-library/src/math/matrix.rs:238` |
| `native_math_class_for_metadata` | `class` | `class`, `body` | `crates/typst-library/src/math/mod.rs:227` |
| `native_math_frac` | `frac` | `num`, `denom`, `style` | `crates/typst-library/src/math/frac.rs:25` |
| `native_math_limits` | `limits` | `body`, `inline` | `crates/typst-library/src/math/attach.rs:86` |
| `native_math_lr` | `lr` | `size`, `body` | `crates/typst-library/src/math/lr.rs:24` |
| `native_math_mat` | `mat` | `delim`, `align`, `augment`, `row-gap`, `column-gap`, `rows` | `crates/typst-library/src/math/matrix.rs:92` |
| `native_math_mid` | `mid` | `body` | `crates/typst-library/src/math/lr.rs:46` |
| `native_math_overbrace` | `overbrace` | `body`, `annotation` | `crates/typst-library/src/math/underover.rs:50` |
| `native_math_overbracket` | `overbracket` | `body`, `annotation` | `crates/typst-library/src/math/underover.rs:82` |
| `native_math_overline` | `overline` | `body` | `crates/typst-library/src/math/underover.rs:22` |
| `native_math_overparen` | `overparen` | `body`, `annotation` | `crates/typst-library/src/math/underover.rs:114` |
| `native_math_overshell` | `overshell` | `body`, `annotation` | `crates/typst-library/src/math/underover.rs:148` |
| `native_math_primes` | `primes` | `count` | `crates/typst-library/src/math/attach.rs:62` |
| `native_math_root` | `root` | `index`, `radicand` | `crates/typst-library/src/math/root.rs:26` |
| `native_math_scripts` | `scripts` | `body` | `crates/typst-library/src/math/attach.rs:74` |
| `native_math_stretch` | `stretch` | `body`, `size` | `crates/typst-library/src/math/attach.rs:115` |
| `native_math_underbrace` | `underbrace` | `body`, `annotation` | `crates/typst-library/src/math/underover.rs:34` |
| `native_math_underbracket` | `underbracket` | `body`, `annotation` | `crates/typst-library/src/math/underover.rs:66` |
| `native_math_underline` | `underline` | `body` | `crates/typst-library/src/math/underover.rs:10` |
| `native_math_underparen` | `underparen` | `body`, `annotation` | `crates/typst-library/src/math/underover.rs:98` |
| `native_math_undershell` | `undershell` | `body`, `annotation` | `crates/typst-library/src/math/underover.rs:131` |
| `native_math_vec` | `vec` | `delim`, `align`, `gap`, `children` | `crates/typst-library/src/math/matrix.rs:33` |

Excluir num-deparenthesized e denom-deparenthesized internos de frac e gap
external de mat; row-gap/column-gap são selecionáveis. Não retirar o parâmetro
constructor gap de mat: parâmetro externo e campo de seleção são superfícies
diferentes. Label permanece campo comum; filtros preservam valores sem cast.
Preservar page/math.op com engine, equation, as quinze identidades do lote
anterior e todas as entradas históricas. Math underline/overline continuam
identidades diferentes das funções globais homônimas.

### Aceitação e limites

Invariantes compartilhadas: consumir o Núcleo Tekt pinado
`00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml`; as
responsabilidades exclusivas acima definem o segmento de cada owner.

Correção interna de tabela/composição em fluxo contínuo ADR-0127. Nenhum campo,
trait, assinatura Rust externa, compatibilidade, default ou fase muda. L0
antes dos testes/código; RED funcional por identidade ausente, GREEN dos 26
casos com filtros e exclusões; controles de With, homônimo e helpers que não
são elementos. Validar antes/depois toda a matriz, preservando observáveis
fora das identidades autorizadas e aliases efetivos (accent/cancel globais
cristalinos incluídos, sem crédito de paridade se não existem no pin).

O sucesso de where/kind não certifica getters, síntese, indexação, show,
layout ou counter. Os controles counter permanecem explícitos e a sua
admissão separada não é alterada neste lote. As quinze rotas restantes de
formatos/HTML/PDF têm investigação própria; a colisão de callback dos formatos
não é resolvida por nomes. FG1/RM1/SY1/RG1 preservam escopos e gates.

Verificar ownership/V26 antes do resselo restrito aos cinco pares; build,
suíte completa, fmt, Clippy/Rustdoc estritos, linhagem e diagnóstico global
sem nova violação, sob MemoryMax12GiB/High10GiB/swap0/jobs1 sequenciais.


## Formatos como funções de elemento — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado. `00_nucleo/diagnosticos/native-format-catalogue-2026-10-17/`
conserva before.json.gz com UTC, status, diff HEAD --stat e hashes exatos;
baseline-reuse.json prova igualdade de fontes/configuração/binário com a
matriz final anterior de 3956 casos em oito perfis. before-batch.json.gz
mede 144 casos detalhados; before-compat.json.gz mede 456 controles de
nomes, aliases e namespace. Cada sonda conserva argv, binários e saídas.
O pin é `9f2b6e8715237cb086899a42873660fe744622e8`, executável SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

As quinze rotas restantes são rejeitadas por where/kind no Tekt e aceitas
no pin. format.rs:17–27/96–146 compartilha native_format entre os cinco
formatos, logo só ponteiro não distingue schemas. pinned-schema.json.gz
conserva nove declarações upstream com blobs/hash/linhas e atributos elem.
FuncRepr é pub(crate) em entities/func.md:21 e entities/func.rs:24; a nota
anterior de que o L0 o mostrava público foi refutada em retificacao-func-repr.md.
public-contract-before.json fixa assinaturas, reexports e structs públicos
antes da mudança; a classificação é representação interna e paridade, não
novo contrato público ou inferência de intenção por render.

### Responsabilidade exclusiva

Reconhecer FuncRepr::Format pela testemunha exaustiva Svg/Png/Pdf/Html/Bundle,
com nome canônico e schemas da tabela. Não descrever formato pelo mesmo
ponteiro compartilhado, nome ou namespace. Não executar callback. Para os
quatro elementos reais restantes, usar os callbacks existentes por
fn_addr_eq: native_pdf_attach, native_pdf_artifact, native_html_elem e
native_html_frame. Não aceitar Native não marcada que reutilize callback
ou nome de formato; clone/alias da Func efetiva conserva a marca.

| Rota | Campos selecionáveis, na ordem do pin | Locatable |
|---|---|---|
| `std.format.bundle` |  | false |
| `std.format.html` | `pretty` | false |
| `std.format.html.elem` | `tag`, `attrs`, `body` | false |
| `std.format.html.frame` | `body` | false |
| `std.format.pdf` | `pages`, `standard`, `tagged`, `pretty` | false |
| `std.format.pdf.artifact` | `kind`, `body` | false |
| `std.format.pdf.attach` | `path`, `data`, `relationship`, `mime-type`, `description` | true |
| `std.format.png` | `ppi` | false |
| `std.format.svg` | `pretty` | false |
| `std.html` | `pretty` | false |
| `std.html.elem` | `tag`, `attrs`, `body` | false |
| `std.html.frame` | `body` | false |
| `std.pdf` | `pages`, `standard`, `tagged`, `pretty` | false |
| `std.pdf.artifact` | `kind`, `body` | false |
| `std.pdf.attach` | `path`, `data`, `relationship`, `mime-type`, `description` | true |

Nomes públicos são os últimos segmentos da rota (elem/frame/attach/artifact
incluídos). HtmlElem exclui css/parent/role internal; label é o filtro comum.
Somente attach é locatable entre essas nove identidades. Esta capacidade não
materializa counter, indexação ou layout. With continua não-elemento bare e
continua rejeitado por where/kind, sem descascar a aplicação parcial.
Preservar toda a tabela anterior e os caminhos page/op/raw.line.

### Aceitação e fronteiras

Consumir as invariantes compartilhadas do Núcleo Tekt pinado
`00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml`. As novas
alternativas Native/Format são agrupáveis porque usam o mesmo payload/ABI
para projeção e execução; cada uma deve ser exercitada nos testes de nome,
namespace, ponteiro, Eq/Hash, clone/With e despacho. V19 novo nessas uniões
deve ser inventariado e ligado aos testes; não adicionar suppressions.
A nova variante deve ser explicitamente nomeada nos matches fechados.

Fluxo contínuo ADR-0127: nenhuma definição, assinatura, campo ou trait
externo novo, compatibilidade, default ou fase muda. Antes do código, RED
funcional dos quinze casos por factories atuais e controle de observação
real contra Native não marcada. GREEN exige corpus completo, 144 casos de
campos/chamadas e 456 controles de compatibilidade preservados fora de
where/kind. Comparar public-contract-before/after e sizeof antes/depois.

L0 antes do código; ownership/V26 antes de resselo restrito aos onze pares.
Build, suíte workspace, Clippy/Rustdoc estritos, fmt, V1/V5/V15/V26, reverse
e diferencial global sem novo erro/aviso. Comandos pesados sequenciais sob
MemoryMax12GiB/High10GiB/swap0/jobs1. Rastrear notas V19 legítimas com cobertura.

Esta etapa fecha admissão where/kind, não todas as operações de seleção,
show/query, síntese, morfologia dos carriers ou counter. As frentes FG1, RM1,
SY1, RG1 e V16 anteriores preservam seus escopos/gates; não aplicar candidatos
antigos por cópia nem publicar.

## R2 — construção e seleção de Sequence, fluxo contínuo ADR-0127

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado.
00_nucleo/diagnosticos/sequence-constructor-2026-10-17/{before,expanded}.json.gz
e before-sources.json.gz conservam UTC, diffstat/status, binários/fontes e casos
bilaterais no pin bd5ebec285505889e8ff1d4f68804ab147a699f3. Vanilla
foundations/content/mod.rs:724–728 declara children requerido;
foundations/value.rs:644–650 define cast Content/string/symbol/none e
typst-macros/src/elem.rs:573–604 gera o constructor. Tekt
01_core/src/compiler/eval/bindings/field_access/content_methods.rs:358–410
ainda projeta fallback não chamável; o catálogo element_where não tem Sequence
e selector_matching.rs:272–337 não tem sua identidade. rules/set.rs:55–76
só recusa With de transforms e não reconhece set da Sequence. São semântica,
morfologia e diagnósticos de língua, não exigência de copiar storage upstream.
A mensagem de children() marcada pendente em R1 já coincide no baseline atual: o
getter recém-corrigido permite que o owner existente produza o erro correto.
A medição nova retifica essa pendência; não editar código de diagnóstico.

### Responsabilidade exclusiva

Acrescentar ao catálogo estático a identidade tipada native_sequence: nome
sequence, schema somente children, locatable=false. Label comum permanece
permitido pelo mecanismo atual. Não usar nome, igualdade de Func ou execução
do callback para classificar. Aliases reais são aceitos, With/closures/homônimos
com outro callback continuam rejeitados. where valida nomes sem cast de valores.
As validações existentes de locatabilidade produzem sequence is not locatable
para query/counter diretos e filtrados; preservar composição e spans.

### Aceitação e limites

Esta seção completa a responsabilidade R2 nomeada no L0 de R1. Corrigir
continuamente tabelas/composição/callback interno, sem novos campos/traits/
assinaturas Rust externas, modo padrão, fase ou quebra de compatibilidade.
L0 antes de teste e código, RED funcional, GREEN e corpus bilateral. Não
certificar guardas globais de show, todos os casts/elementos, nem paridade
universal de render. Qualquer refutador residual deve permanecer no relatório
e ganhar owner/L0 antes de intervenção, sem mudar testes para esconder o caso.

Validar build/workspace/doctests em invocação separada, Clippy/Rustdoc estritos,
fmt, V1/V5/V15/V26 e reversa, SARIF diferencial sem erro/warning novo. V19 pode
registrar agrupamento nominal prescrito sem supressão, testando alternativas.
Heavy jobs sequenciais, MemoryMax12GiB/High8GiB/swap0/jobs1, debug0/incremental0
só no ambiente de validação. Resselo após V15/V26 válido e proveniência exata.
