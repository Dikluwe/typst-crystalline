# L0 — compiler/eval/operators/equality
Hash do Código: 6126614c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:a37779d03a4ba0f16ebcc5919477bfdcf783021511a28cf8c9203c67cd64a9e5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/equality.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/operators/equality.rs:21` — `fn apply_binary`.
- `01_core/src/compiler/eval/operators/equality.rs:91` — `fn value_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:103` — `fn values_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:138` — `fn selector_contains_element`.
- `01_core/src/compiler/eval/operators/equality.rs:163` — `fn selectors_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:203` — `fn counter_keys_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:210` — `type ContentFields =`.
- `01_core/src/compiler/eval/operators/equality.rs:213` — `fn raw_content_fields`.
- `01_core/src/compiler/eval/operators/equality.rs:226` — `fn content_values_eq`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/content/raw.rs:354–361`.
- `foundations/selector.rs:75-81`.
- `foundations/value.rs:295-299`.
- `introspection/counter.rs:243,256`.

## Contract

Own Eq/Neq/In/NotIn, values_eq and value_eq. This is language equality, not
derived Rust PartialEq or ObservationEq. Coerce Int/Float recursively, including
array/dictionary values. Containers preserve their existing key/cardinality
comparison; raw Content uses morph_canon and ignores baked render style while
retaining semantic morphology. Version compares all components with zero padding.
Length equals Relative only with zero relative component; Ratio equals Relative
only with zero absolute component and |rel-ratio| < 1e-9. Other existing families
retain their derived fallback. Neq is the negation of the same relation.

For any comparison involving LocatedContent Some, use the pinned snapshot
contract: compare canonical element function and linguistic field sets excluding
label; recursively compare values. For raw/None counterparts use only their
explicitly projected fields, without filling defaults. Field insertion order,
Location and carrier discriminant do not determine this equality. Different
field sets differ; raw is neither automatically equal nor automatically unequal
to its realized counterpart. Keep raw/raw and None fallback unchanged.

Selector::Element compares canonical Func using current function equality and
the ordered filter sequence by name position and recursive values_eq. Empty
group differs from bare Kind; aliases retain identity; NaN filters remain
nonreflexive. Propagate through nested containers and selector compositions,
preserving antecedent comparison for pairs without Element. Counter keys with
Element use the same recursive selector relation, including field order. Do not
exclude Int/Float-equal pairs by structural hash bucket or shortcut with Arc
identity. Before compares base/end/inclusive recursively; After compares
base/start/inclusive; distinct variants differ, with no document query.

Membership uses this same relation for array elements; string-in-dictionary
tests key existence and string-in-string tests substring presence. NotIn negates
these supported combinations. Unsupported combinations use the canonical
quoted-operator diagnostic. Do not modify public PartialEq/Hash, morph_canon or
the content IR to achieve language equality.

## D08A — seletor bare de table.cell (PROPOSTO, gate ADR-0127)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
UTC, diffstat, entradas, comandos, hashes das fontes e executáveis em
lab/results/construction/diagnostics/math-residuos-table-cell-2026-10-06/{before,table-additional-before,table-aliases-before}.json.gz.
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

Estender selector_contains_element para NativeElement e propagar em composições. Comparar duas folhas NativeElement por identidade estática real de funções nativas (fn_addr_eq), mantendo aliases que reusam a nativa e distinguindo funções homônimas. NativeElement difere de Element com grupo vazio. Não modificar PartialEq/Hash de Func/Value ou os branches legados sem a nova folha. Counter keys e Value::Selector usam a mesma relação de linguagem; não excluir pares por hash mecânico ou nome. Testar alias de linguagem, alias Rust com nome diferente e function diferente com nome cell.

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

## Igualdade sRGB após preservação dos componentes — 2026-10-13

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado; fontes/L0, UTC, status, diffstat e hashes dos
binários em `lab/results/construction/diagnostics/limites-cor-2026-10-13/equality-before.json.gz`.
O oráculo pinado 9f2b6e8715237cb086899a42873660fe744622e8 compara
rgb(50%,0%,0%) e rgb("#800000") como iguais, incluindo arrays, dictionaries,
in/not in e alpha. O cristalino diverge ao preservar corretamente os f32.
Controles de bytes distintos, LinearRgb e espaços diferentes permanecem falsos.

Fonte upstream `visualize/color.rs:2064-2083` compara duas cores Rgb pelos
quatro componentes u8; `:1780-1783` usa `(x * 255).round() as u8`, e NÃO o
arredondamento ties-even do to-hex. A sonda de empate 90.5/255 conserva
`to-hex = #5a0000`, mas igualdade com rgb(91,0,0) verdadeira e com rgb(90,0,0)
falsa. Recibo `equality-tie-before.json`, conservado em evidence.json.gz.

Classificação: ==, != e pertença são observáveis de linguagem, distintos da
igualdade estrutural Rust (ADR-0107). Correção interna de paridade contínua
ADR-0127, sem alterar assinatura pública, PartialEq/Hash, componentes,
serialização, fase ou modo padrão. Refinar em values_eq somente o par
Value::Color(Srgb)/Value::Color(Srgb), comparando r/g/b/alpha após a projeção
u8 medida. Usar round de empate afastado de zero, não to_srgb/to_hex que
usam ties-even. A recursão existente propaga para containers e operadores.
Manter os outros espaços, pares heterogêneos e identidades estruturais.

Testes primeiro: positivos RGB/alpha/empate, negativos de canal/alpha,
espaço diferente e LinearRgb; ==/!=/in/not-in, arrays e dictionaries.
Demonstrar explicitamente que PartialEq estrutural pode continuar falso.
Verificar RED, GREEN, build, selos e sondas bilaterais sem normalização.
Este recorte não certifica Luma, spot ou todos os consumers de igualdade.

## Styled — igualdade de filho no operador, 2026-10-17

### Medição anterior à decisão

lab/results/construction/diagnostics/text-styled-equality-2026-10-17/before-corrected-measurement.json.gz
registra24 pares/48 invocações,17 divergências, HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado, UTC/status/diffstat e hashes das fontes/binários.
Pin9f2b6e8715237cb086899a42873660fe744622e8: foundations/content/mod.rs:779–782
compara StyledElem apenas pelo child; :376–385 coalesce Styled adjacente
quando aplica mapa não vazio. Tekt usa morph_canon seguido de PartialEq
estrutural, que ainda observa Styles. TS1 conserva Length e expõe duas
regressões true→false em relative-zero/affine-absolute-part; o pin mantém true.
A correção não pode ser inferida como igualdade dos valores declarados.
O primeiro corpus continha dois corpos aninhados escritos como markup literal
e calc.nan ausente; não é RED válido desses casos. A rodada corrigida usa
chamadas como expressões e float("nan"), com todos os24 casos executando.

### Decisão interna de paridade

Refinar somente os caminhos de Content cru neste operador: Eq/Neq diretos,
values_eq recursivo e fallback raw/None em content_values_eq usam helper privado
content_language_eq. Primeiro conservar morph_canon existente. Depois criar
uma projeção efêmera exclusivamente de comparação: Styled mantém sua variante
e o filho, coalesce somente Styled diretamente adjacente e troca somente seu
mapa por Styles vazio. Os demais nós são preservados integralmente, com
filhos já visitados bottom-up. O passthrough devolve o nó completo, não default
neutro; não silenciar V16 nem trocar por identidade de Arc.

A projeção vazia nunca é entregue ao render, armazenada em Value/Engine,
publicada como morph_canon nem usada por Hash/ObservationEq/terminação. Não
é segundo tamanho, scalar fake ou apagamento de todos os wrappers. Não chamar
morph_canon outra vez sobre ela: wrapper versus plain continua distinto.
Strong/Emph/Heading e diferenças de body/fields mantêm suas identidades.
PartialEq Rust e o morph_canon público permanecem intactos; comparar Styles
estruturalmente fora deste operador continua permitido. Os ramos de snapshot
Some, campos/labels e as coerções de outros valores mantêm o contrato anterior.

Propagar a mesma relação a arrays/dicts, pertença e complemento !=. Ignorar
NaN em payload descartável de estilo não torna Float NaN reflexivo. Preservar
a primeira origem Source e não avaliar argumentos/corpo/funções de estilo.
Esta é correção interna ADR-0127 em fluxo contínuo: nenhuma assinatura, campo,
compatibilidade Rust, default de produto ou fase muda. Transparência custom-only
histórica de morph_canon não é alterada nem certificada universalmente aqui.

### Aceitação

Seis testes funcionais primeiro: estilos/igualdade estrutural segregada,
coalescência de wrappers, filhos em nós nominais/Sequence, containers/operadores,
negativos e Source/NaN. Confirmar RED→GREEN,24 pares coincidindo e nenhum
controle antes coincidente regredindo. Revalidar o corpus textual TS1, Par/Box
e RG1 preservando as diferenças de igualdade intencionalmente corrigidas.
Suíte completa/build/Clippy -D warnings/-D unfulfilled_lint_expectations,
Rustdoc -D warnings/quatro índices, fmt/diff, linhagem estrita/reversa e SARIF
diferencial sob12GiB, MemoryHigh10GiB, swap zero e Cargo jobs1. Sem commit,
push, merge ou release. Não declarar resolvido todo o universo de Content.
