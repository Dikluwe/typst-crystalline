# L0 — stdlib/foundations/selector
Hash do Código: 429d7dea

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:a37779d03a4ba0f16ebcc5919477bfdcf783021511a28cf8c9203c67cd64a9e5

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/selector.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/selector.rs:23`, `01_core/src/compiler/stdlib/foundations/selector.rs:269`, `01_core/src/compiler/stdlib/foundations/selector.rs:286` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own native_selector, element_kind_of_native_func and parse_selector_arg used by query/locate. Use static identity mapping, not callbacks, dynamic registry, vtables or function-name matching.

Preserve Location, Selector and Label semantic identity. Preserve the entire Element selector including function identity, empty filter group, group order and Values: clone it rather than reconstructing Kind or retaining only the first filter. Construction does not require locatability; text.where remains a valid show selector even when query/counter reject it. Known kind strings, label strings "<sec>", and the established bare heading/figure/table/metadata element functions retain their conversions. Non-element functions fail `only element functions can be used as selectors`. Recognize native_outline_entry by address as OutlineEntry, not outline.entry.body.

Accept an existing Selector unchanged and a Regex after empty checks: empty string `text selector is empty`; empty regex `regex selector is empty`; non-empty regex matching empty text `regex matches empty text`. Valid regex returns Selector::Regex. Preserve arity, types and exact diagnostics. The registry owns the established Type::Selector binding; construction/parsing must not change it or promote additional bare functions implicitly.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

## Seletor nativo de math.equation — 2026-10-06

### Evidência antes da decisão

`00_nucleo/diagnosticos/plano-debitos-2026-10-06/math-before.json.gz`
conserva pin/HEAD/árvore/UTC/fontes/comandos/SHA. Equação explícita é locatable,
mas query(math.equation), alias e selector(math.equation) são rejeitados.
`01_core/src/compiler/stdlib/foundations/selector.rs:269-283` reconhece apenas
heading/figure/table/metadata/outline.entry por identidade estática e omite
o constructor já existente de equation. `structural/math.rs:97-103` registra
o mesmo constructor nativo no módulo público; o índice já trata Equation.
O pin devolve a equação nesses controles. ElementKind::Equation já existe;
não é necessário ampliar enums ou reconstruir a biblioteca para reconhecer.

### Decisão e aceitação

O hub estrutural expõe somente um bridge interno pub(super) para
math::equation::native_math_equation, com acesso limitado ao subtree stdlib;
não ampliar visibilidade de módulos, API exportada de compiler::stdlib ou
assinaturas. O owner selector acrescenta a função à tabela estática de
ElementKind::Equation usando fn_addr_eq, mantendo alias pela função real.
Não usar nome do Func ou string math.equation como identidade. Não executar
constructors, World ou make_math_module em produção para descobrir binding.

Testar parse_selector_arg/native_selector, alias nativo com nome diferente e
func diferente com nome math.equation. CLI deve observar número de equações e
ordem/fields dos elementos já existentes. Não envolver matemática bare em
Equation, alterar fase, promover todas as funções ou alegar correção da
morfologia de matriz. .where é responsabilidade de element_where e fica
fora deste lote; table.cell também permanece independente. Correção interna
de mapeamento/paridade em fluxo contínuo ADR-0127.

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

native_selector e parse_selector_arg reconhecem table.cell exclusivamente por fn_addr_eq(native_table_cell), produzindo Selector::NativeElement com ausência de grupo. Conservar os mapeamentos Kind legados, erros de funções não-elemento, closures e aplicação parcial; não promover implicitamente todo o catálogo native_element_fields. Aliases da mesma nativa retêm identidade e repr cell; função homônima não recebe essa identidade. selector(table.cell) é válido independentemente de locatabilidade. where vazio continua Element explicitamente filtrado.

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

## D08A — diagnósticos negativos de funções não-elemento

Medição antes da decisão: table-cell-selector-2026-10-06/before.json.gz,
partial e closure, registra rejeição e mensagem coincidentes mas span detached
no Tekt; vanilla ancora ao valor da primeira posição. O branch Func inválido
usa Span::detached em selector.rs:69–77; Args conserva value_span causal.
Correção interna ADR-0127: apenas esse erro de reconhecimento em native_selector
usa o primeiro value_span posicional não detached, com fallback args.span;
preservar a mensagem, classificação e demais branches. RED→GREEN de aplicação
parcial e revalidação bilateral fecham o diagnóstico, sem nova API ou fase.

## A1 — reparo de paridade medido em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c; estado/UTC/fontes em
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
Vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3. before-pdf-probe.json.gz
no mesmo diagnóstico congela controles bilaterais de campo, admissão, callback
e init. A auditoria anterior paridade-novo-vanilla-2026-10-17/inventario.md
E02–E04 registra os limites de carrier e as fontes discriminantes; não se usa
dump/estrutura Rust como critério. Estes observáveis são linguagem.

### Responsabilidade exclusiva e aceitação

Para funções nativas do catálogo native_element_fields já existente, a
construção/parsing bare admite NativeElement quando não houver a conversão Kind
legada. Não executar constructor e não comparar pelo nome. .with/closure/homônimo
não ganham identidade de elemento. Preservar Kind legados e a distinção bare
versus where vazio; construção não valida locatabilidade. Query/counter continuam
validando locatabilidade em suas fronteiras. Esta seção supersede a restrição
histórica de não promover outras funções, somente para elementos já catalogados.

L0 primeiro, RED funcional e GREEN bilateral. Não alterar contrato público,
default de produto, fase ou compatibilidade; gate A1-S1 é independente. Preservar
1:1 e núcleos pinados; resselo após grafo válido. Build/workspace/Clippy estrito,
Rustdoc/fmt/linhagem estrita e reversa, jobs sequenciais sob MemoryMax12GiB,
High8GiB/swap0/jobs1. Debug0/incremental0 somente no ambiente de validação para
reduzir picos; comparar consumo medido sem inferir ganho do produto.

Converter também a função real native_footnote em Kind(Footnote), preservando
a chave existente do counter e a igualdade bare versus selector(f).
