# L0 — introspect/walk
Hash do Código: ea1d26d5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:8de2c32fbeeee7a30088b4ff3808e15a211f23a182673ef5cdebe54bd3eadad6
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0


## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/walk.rs`.
No estado `df72a6c4a701917ab56a7c6169764a92ea775693` com árvore limpa, `01_core/src/compiler/introspect.rs:1897-2485` contém o comportamento que esta unidade recebe. A fonte e o estado estão registrados em `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. A decisão é separá-lo por comportamento, sem alterar a API ou alegar ganho de desempenho.

## Contrato

Percorrer Content por match exaustivo e estático. Alocar Location, capturar payload e snapshot antes de Start, descer uma vez segundo cada variante e emitir End no mesmo Location após a recursão. Delegar Heading e Label a seus módulos existentes; não executar callbacks nem I/O.

As invariantes de linguagem, a ordem causal e os limites de paridade continuam definidos em `00_nucleo/prompts/core/compiler/introspect.md`; este owner não cria uma segunda interpretação. O código usa chamadas estáticas na camada L1, sem vtable, I/O ou estado global mutável.

## Aceitação

Preservar os testes de introspecção e a suíte L1, compilar e passar `crystalline-lint .` com hashes coerentes. Mudanças semânticas futuras exigem medição de linguagem e atualização do L0 antes do código.


## Realização com avaliação única

Walk registra Located em sua Location explícita, conservando snapshot do corpo. Realized percorre primeiro origem e depois produto; alias de ocorrência já registrada não duplica tags/counters. ShowRecipe pendente projeta corpo para introspecção inicial. Locator observa Locations explícitas para não colidir com expansões ordinárias. Não deduplicar elementos por texto, morfologia ou endereço.


## Realização com avaliação única

O registro da origem aceita Location explícita preparada antes da receita; ocorrências não preparadas mantêm alocação ordinária.


## Realização com avaliação única

Despacho exaustivo dos carriers precede o walk dos elementos; não duplicar sua lógica nos arms nativos.




## Medição da ordem antes da decisão

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, CounterRegistry::value_at e StateRegistry::value_at comparam Location.as_u128; queries before/after e eventos de contador fazem o mesmo. A realização prepara identidades antes de transformar, portanto o número deixa de ordenar origens e produtos. A sonda CLI e a sentinela single_evaluation_counter_follows_causal_order conservam a evidência no diagnóstico de implementação. Aplicar o núcleo realization-order; manter fallback histórico somente sem ordem registrada.


## Realização com avaliação única

Registrar ordinal causal de toda Tag::Start antes de popular registros e histórico temporal. Identidade explícita não equivale a ordinal.


## Realização com avaliação única

Reservar Locations explícitas antes de alocar ocorrências ordinárias introduzidas por expansão contextual/outline. A reserva não registra tags nem altera ordem causal.


## Realização com avaliação única

A reserva de Location é serviço interno puro reutilizado pelo layout para alocação ordinária em sincronia com o walk, sem duplicar sua implementação.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Table resolvida percorre plano completo e Cell Complete percorre visual; não indexar corpo cru ocultado, nem registrar Cell. Table e produtos mantêm Located/ordem causal; heading gerado torna-se consultável, heading ocultado não. Metadados de padding aparecem em ordem row-major uma vez, não por repetição física.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.

### Refutação e decisão interna antes de código — 2026-10-07

Medições em lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{after-fourth,refuters-fourth}.json.gz, nested-reuse-red.log e effect-order-red.json: fontes/binários/HEAD/estado/UTC/resultados.
after-fourth/resolved-rowspan e rowspan-fixed observam índice b,a,c, apesar de plano de origens a,b,c. effect-order-red.json mede que panic de a (SPAN) acontece primeiro em ambos: execução das receitas continua row-major, índice de metadados segue fim da área de rowspan (end-row,col). Vanilla grid/layouter.rs:1755–1787 emite rowspans diferidos. Somente a vista indexada preparada ordena por (y+rowspan-1,x); plano e HTML conservam row-major. refuters-fourth/reused-prepared-table mede metadados perdidos: walk.rs:88–104 ignora Located Table quando original já ocupou a Location. Original de Table consumida deve ser registrado raso, preservando payload/counter, sem percorrer caption/children ocultados. Aliases da Table emitida com a Location já registrada ainda visitam o visual preparado; guard de Location dos filhos impede duplicar contribuição. Não expandir essa política para outros elementos; Cell segue não-locatable.
Supersede apenas a projeção/guarda aqui medida, sem contrato público ou fase adicional. Refutação: diferença nos testes/recoletas, efeitos repetidos, fonte ocultada visível ou regressão de cache/geometry.

## D08B3-B — candidato, não vigente antes de ratificação

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

isolated-default: vanilla expõe x/y/inset/align/fill/breakable auto, spans 1 e stroke dict vazio; Tekt omite esses defaults e publica duas metadata contra uma. isolated-x rejeita no Tekt, funciona no vanilla. raw-fields continua só body nos dois e raw-x rejeita nos dois. isolated-table-style confirma que defaults de Table não viram os da Cell avulsa. raw-vs-prepared mede false no vanilla e true no Tekt. nested-effects/body-effects/late-rule refutam execução única e origem lexical. fresh-isolated/table, fresh-block, fresh-styled e multi-fresh são aceitos indevidamente pelo Tekt; vanilla rejeita maximum show rule depth exceeded com span do constructor novo. identity/body/alias e fresh-conditional/hidden continuam válidos. multi-identity e multi-return-table mostram que devolver it deve permitir receita anterior ainda não consumida. isolated-query mantém cell is not locatable nos dois; location é none, sem localização falsa. cell-set é negativo separado, explicitamente excluído deste pacote.

Fonte vanilla lab/typst-original/crates/typst-library/src/model/table.rs:734–778 declara defaults e propriedades; typst-realize/src/lib.rs:367–386 prepara/guarda o argumento, :418–429 visita produto com profundidade, :491–505 distingue guards por elemento, :571–586 materializa propriedades. Tekt entities/elements/table_cell.rs:111–143 só projeta lexical ou plano posicionado; compiler/realize/table_cells.rs:110–129 reaplica Cell crua sem preparo; :280–304 termina snapshot inteiro; eval/show_rule_termination.rs:65–70 trata mesma forma como ponto-fixo; eval/rules/apply.rs:139–166 revoga regra globalmente durante callback. Classificação: campos, efeitos, ordem, terminação e erro causal são língua. A estrutura de guards e materialização upstream é mecânica e não é requisito. Não se infere intenção do upstream; a escolha nominal abaixo é inferência refutável pelos controles de aliases, duas regras, novas gerações e conteúdo ocultado.

### Responsabilidade exclusiva proposta

Indexar somente visual efetivamente emitido de Cell preparada avulsa; não indexar body lexical em paralelo nem tornar Cell locatable. Posicionada mantém ordenação por plano lógico existente. Metadata das receitas/body emitidos ocorre uma vez, no contexto causal atual.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Walk Source visita body uma vez sob a mesma cadeia/label, sem tag própria. Reserva de Locations também atravessa body uma vez.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Reserva de Locations por leitura emprestada — 2026-10-09

Medição antes da decisão: `lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/stacks-deep.json.gz`
registra binário, HEAD/working tree, comando, fontes, UTC e pilhas de uma em
cada 64 alocações. No caso `local-set-1600`, 450 amostras de
`TableElem::map_content` aparecem sob `reserve_locations` durante layout,
contra 151 no controle sem set. A fonte `walk.rs:27-36` usa
`map_content` para uma inspeção sem transformações e descarta o produto.
`reserve-red-gate.json.gz` registra 1.029.471 pedidos de alocação no caso
local, com proveniência do binário anterior. Inferência refutável: percorrer
por empréstimo as variantes comuns elimina cópias sem alterar o conjunto de
Locations reservadas; gate de alocação e comparação de saída/tempo testam isso.

Decisão interna: visitar diretamente Source, Located, Realized, ShowRecipe,
Styled, Sequence, Table e TableCell, seguindo os mesmos filhos que
`map_content` visita. Table resolvida percorre células e caption; Table crua
percorre children, header, footer e caption. TableCell percorre o visual
standalone/resolved quando presente, senão body. Realized percorre visual e
origem; Located observa sua Location depois do body. Para outras variantes,
conservar o visitante `map_content` como fallback, incluindo a recursão na
origem de Realized. Não criar Content ou expor API, não alterar ordem causal de
emissão/indexação, Locator, fase ou defaults. Aceitação: a maior Location
explícita em variantes aninhadas continua reservada; saídas e erros de
linguagem equivalentes, com redução medida de cópias/alocações e comparação
pareada de tempo. Fluxo contínuo ADR-0127.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C007`: `01_core/src/compiler/introspect/walk.rs:91`, `walk`, 8 argumentos contados pelo Clippy (limiar 7).
- `C008`: `01_core/src/compiler/introspect/walk.rs:121`, `walk_at`, 10 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C007` — `walk`: Exceção pontual justificada

Travessia pura recebe os acumuladores mutáveis da compilação e o contexto lexical por empréstimo. Contador de auto-label, Locator, tags e introspector têm papéis distintos; não há estado global nem novo dono de decisões.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C007: Travessia pura recebe os acumuladores mutáveis da compilação e o contexto lexical por empréstimo")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C008` — `walk_at`: Exceção pontual justificada

A recursão adiciona forced_location e shallow_table à mesma travessia. São dados causais por ramo, que não devem ser persistidos globalmente nem fundidos ao snapshot semântico.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C008: A recursão adiciona forced_location e shallow_table à mesma travessia")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### Limite da exceção

Autorizar somente `#[expect(clippy::too_many_arguments, reason = "...")]` no item
nominal acima, com o motivo arquitetural indicado. Não aplicar no módulo, impl,
crate ou configuração global; não elevar o limiar nem abranger outro lint.
Nos casos com remoção de argumento morto, a exceção residual depende dessa correção.
Se a função deixar de exceder o limiar, retirar/revisar a expectativa; não silenciar
`unfulfilled_lint_expectations`. Esta permissão específica supera proibições
genéricas de supressão em seções anteriores somente para este lint e os itens
nominais justificados aqui. As demais obrigações de correção permanecem.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C007, C008. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.


## Carriers compactos de introspecção/show — proposta 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz em lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16 registra UTC, fontes/L0, hashes, status e git diff HEAD --stat. Os owners atuais foram lidos antes de propor esta alteração. probe-details.json registra rustc 1.92, argv, rlib/binário/sonda com SHA-256 e saída; probe-result.json registra teto de 12 GiB, swap zero, um job e eventos. A sonda é sintética de carriers e do algoritmo de registro/cópia, não benchmark de compilação/layout/RSS nem distribuição de workloads reais. Os candidatos são espelhos com tipos reais de domínio; o caso inline usa Tag e ShowRule produtivos.

Tag mede 416 bytes; Start boxed e Arc medem 48. Para 4.096 pares Start/End com payload Heading sem heap, inline solicita 3.407.872 bytes numa alocação; Box solicita 2.031.616 em 4.097; Arc solicita 2.097.152 em 4.097. Clone de Tag boxed também aloca por Start; os caminhos produtivos lidos em introspect/fixpoint.rs:86–136 e entrypoints.rs:44–66 usam Vec<Tag> temporário e leituras emprestadas, sem clonar o vetor. Walk :325–356 popula o índice por referência antes de mover info para Start. A decisão não toma o clone sintético como frequência produtiva.

Transformation mede 328 e ShowRule 376 bytes; candidatos com Box/Arc<Styles> medem 120 e 168. rules/show.rs:464–470 copia to_vec antes de cada registro; apply.rs:108–112,349–352 filtra/clona regras. Em 128 registros 100% show-set simples sem reter snapshots antigos, inline solicita 12.276.960 bytes/384 alocações, Box 8.194.560/8.640, Arc 5.530.624/512. Com snapshots retidos, picos solicitados são 3.203.856, 4.141.744 e 1.477.808. No controle com canal custom, as cópias de 4.096 regras solicitam 4.107 alocações inline, 8.203 Box e 11 Arc (crescimento do Vec incluído). As leituras de estilos são emprestadas em apply/intercept/outline/table_cells; a aplicação ainda clona o delta quando necessário.

Contraexemplo obrigatório: construir uma única coleção de 4.096 regras 100% show-set simples pede 1.540.096 bytes inline, 2.031.616 Box e 2.097.152 Arc. Arc pode reter capacidade sobrante do canal custom; no controle rico de 128 registros sem snapshots antigos, o final vivo foi 78.064 bytes inline e 156.432 Arc. Não afirmar ganho universal ou ignorar custo atômico/indireção. É mecânica de armazenamento sob ADR-0030/0107, não paridade de linguagem ou ganho cognitivo por IA medido. A inferência favorável a Arc no ciclo de cópias seria refutada por regressão de memória/tempo em um workload real relevante; manter visíveis estes controles.

### Responsabilidade exclusiva proposta

Migrar somente a emissão Tag::Start(loc, info) para Tag::Start(loc, Box::new(info)) nos pontos já existentes. Preservar construção e valores de info, população do índice antes do move, loc, End/hash, ordem de walk e efeitos/contadores. O novo allocation é uma por Start; não clonar info, mover a população, alterar deduplicação, reserva, snapshot ou recursão. A forma pública pertence a 00_nucleo/prompts/core/entities/tag.md; este owner mantém exclusivamente seu comportamento existente.

### Gate e aceitação

Proposta não vigente até ratificação humana ADR-0127 §2.1/§2.4: há mudança de payload público e compatibilidade de construção/acesso. Não editar código ou owners ativos por este pacote antes da aprovação. Após ratificação instalar estes L0, testes primeiro e RED real, implementação e resselo estritamente dos owners alterados. Não silenciar large_enum_variant, alterar limiar ou adicionar dependências/flags/defaults/fases.

T1: contratos de construção Box<ElementInfo>/Arc<Styles> e orçamento relativo dos carriers (Tag até três Location; Transformation até Content+alinhamento) devem falhar no estado antigo e passar no novo, sem fixar ABI universal. T2: Start/End/Location/payload/label, igualdade e hashes estruturais por valor, clone independente da Box e mutation isolation preservados; hash não vira formato persistido. T3: testes existentes de introspecção/convergência, nesting, heading/label, state/counter/query/metadata continuam com os mesmos esperados e erros. T4: clone de Transformation::Style compartilha o Arc; Arc::make_mut em cópia preserva estilos do original, incluindo custom, e as variantes Func/Content/Str permanecem. T5: controles existentes de show-set/func, precedência, escopo, labels, outline, table.cell e textual/regex mantêm semântica/morfologia/erros; nenhuma regra executa a mais. T6: workspace completo, build, Clippy all-targets e expectativas/lints já fechados como erro, rustdoc -D warnings com quatro índices, fmt, diff, linhagem estrita/reversa e Tekt sem aumento; 12 GiB, swap zero, um job. Registrar HEAD/working tree/hashes/diffstat/UTC e distinguir tamanhos/bytes solicitados de RSS. Não publicar release. Alguma mudança de assinaturas, campos ou fase além destes payloads exige outra proposta.

## Ratificação humana — 2026-10-16

Proposta aprovada pelo humano, manifesto SHA-256 `282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950`. O contrato acima passa a vigente; inclui materialização/testes T1–T6, sem release. Registro em `lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/approval-receipt.json`.


## Alinhamento OutlineEntry e walk exaustivo — 2026-10-16

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, lista exata git diff HEAD --stat, fontes/hashes e binários em `lab/results/construction/diagnostics/tekt-v16-introspect-walk-2026-10-16/before.json.gz`. Medições CLI anteriores em measurement-before.json e measurement-fields-before.json: no pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, entry direta aparece uma vez, sintetizada uma vez e mista duas vezes; no Tekt, a direta some da query (0 contra 1), mista perde a direta (1 contra 2), e body() direto falha cannot outline heading. Fonte: entities/elements/outline_entry.rs:61 body só aceita Heading bruto; :99 fornece NativeElement; introspect/extract_payload.rs:148 descarta OutlineEntry; walk.rs:805 trata entry terminal; outline_entries.rs:38–61 sintetiza entries após walk. Generic map_content visita element/fill (traversal.rs:221); realized.rs:139–281 usa essa travessia para certificar origens, motivo para testar um heading referenciado Located sem confundi-lo com descendente renderizado. O upstream pinado declara OutlineEntry Locatable em outline.rs:472–481. Isto mede semântica/morfologia e erros observáveis da linguagem, não impõe igualdade Rust/bytes/PDF ou mecânica vanilla. A hipótese de correção é refutada por query/fields/body errados, headings fantasmas, duplicação ou regressão em síntese/ordem/labels.

### Responsabilidade exclusiva vigente

Fechar somente os seis wildcards W1–W6 enumerados em scope.json por complementos nominais planos de Content, mantendo os corpos {}, false e None, recursões, efeitos, early returns, assinaturas, curto-circuito e ordem. W6 é o oráculo de regressão do teste old_reserve, não produção; fechar também esse match para não deixar V16 no mesmo arquivo. Não modificar table_root ou outras regras de tabelas semanticamente neste lote. Preservar OutlineEntry como terminal: element/fill são campos semânticos, não filhos que o walk deva indexar como headings ou produzir efeitos. A nova delegação do extractor deve registrar a própria entry direta via caminho NativeElement já existente, inclusive query nativa e labels, sem inventar novo payload ou tabela no walker. A síntese pós-walk existente continua, com controles de contagem e identidade de entradas diretas/geradas/mistas para impedir duplicação. Nenhuma migração de fase ou API.

Esta revisão supersede somente as ressalvas anteriores de pendência OutlineEntry neste owner; preserva os demais contratos vigentes.

### Aceitação

L0 primeiro, testes co-localizados antes da implementação e RED funcional real; manter controles existentes e a medição CLI anterior intacta. Repetir fixtures direct/synthesized/mixed com body() e com acesso ao campo, comparar resultados de linguagem, não bytes PDF. Sonda Content sentinela separada em cópia do workspace real: os seis pontos W1–W6 devem acrescentar E0004 após a correção e não antes, sem outros erros; preservar logs/inputs. Resselo só dos cinco pares afetados e grafo V15/V26 válido. Gates: build workspace, fmt, Clippy all-targets -D warnings e unfulfilled_lint_expectations, Rustdoc -D warnings, suíte workspace completa, V1/V5/V15/V26 e reversa, diff-check e SARIF diferencial. Esperado: seis V16 removidos (239→233), seis V19 informativos novos; V19 existente do complemento do extractor perde uma alternativa e os dois grupos de needs_fresh_index migram OutlineEntry de false para true. Nenhum V20 novo ou outro diagnóstico alterado após mapear deslocamentos. V19 é grupo de consequência uniforme prescrito aqui, não cobertura de execução de cada alternativa. Sem supressões ou configurações globais. 12 GiB, swap zero, um job Cargo, comandos pesados sequenciais; estado/UTC/hashes registrados. Correção interna/paridade ADR-0127, sem API/campo/trait/default/fase ou compatibilidade novos. Não publicar, commitar ou fazer push. O objetivo geral permanece aberto com os demais V16.


### Refinamento medido — preservar contexto ao atravessar Source

source-carrier-before.json.gz preserva a primeira implementação e o teste RED de alias: query nativa devolve Locations 1 e 2 em vez da Location explícita 0. Sete controles passaram, um falhou em attempt-source-carrier-validation-controls.log. O ramo Source em walk_at chama walk, que reinicializa forced_location=None e shallow_table=false. É perda de contexto pelo wrapper causal, não duas ocorrências de linguagem.

Preservar forced_location e shallow_table ao delegar o body de Source: usar walk_at com os mesmos acumuladores, chain, label e os dois valores recebidos. Não observar/alocar nova Location nesse wrapper, não alterar os outros carriers ou assinatura. Acrescentar antes desta correção o controle de Source(Table) no modo shallow com Location forçada: um único Start/End da tabela, sem Heading do corpo. O alias Located(Source(entry)) e o controle raso devem passar depois; as reservas e os corpos nominais W1–W6 permanecem. Isso restaura o núcleo de origem já pinado, na mesma fase, sem contrato público/default novos. O RED inicial de sete falhas é anterior às correções; este refinamento tem seu RED próprio registrado, sem reescrever a evidência original.


## Entradas diretas e ordem da síntese — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; order-before.json.gz/before.json.gz preservam estado, UTC, hashes e diffstat. trace-runtime-result.json e trace-runtime.log mostram a entry direta no índice puro, runtime e L3 sob NativeElement; a linguagem bare converte outline.entry em Kind(OutlineEntry), conforme foundations/selector.rs:113. O walk terminal atual não preenche esse kind. measurement-order-before.json no pin 9f2b6e8715237cb086899a42873660fe744622e8 mede BARE [Heading] versus [Heading,Direct], FILTERED [Direct,Heading] versus [Heading,Direct], STATE 2 versus 1 e COUNTER ((2,),) versus ((1,),) na Location da entrada sintetizada. Esses são observáveis de linguagem; não se exige igualdade Rust/bytes. A síntese pós-walk aloca entradas depois dos updates, mas sua posição causal é dentro do outline anterior. O núcleo realization-order já proíbe tratar identidade numérica como ordem. A correção é refutada por ordem, prefixo temporal, duplicação ou dados de entry errados.

### Responsabilidade exclusiva

No ramo OutlineEntry terminal, registrar emitted_loc no kind_index OutlineEntry quando presente. Captura de payload/Start/End continua no caminho nativo existente; não duplicar ação ou tag nem percorrer element/fill. Preservar identidade própria e aliases preparados. Cobrir query Kind e NativeElement nos testes anteriores, sem trocar seletor de linguagem para contornar o índice incompleto.

### Aceitação

Testes funcionais antes da mudança: índice bare da entry própria e aliases; ordem de três entries (dois outlines com entry direta entre eles), where e ausência de heading fantasma; estado e contador lidos na Location sintetizada devem refletir o prefixo do outline. Depois, testes de contrato dos dois helpers internos antes da sua implementação, com âncora desconhecida, duplicatas/repetição, IDs não monotônicos e append denso. Repetir nove sondas CLI sem relaxar resultados (seis originais e order/state/counter). Suíte completa, build, fmt, Clippy estrito, Rustdoc estrito, V1/V5/V15/V26/reversa/diff e SARIF: seis V16 removidos, sete V19 previstos e três mensagens existentes atualizadas, nenhum V20 novo. 12 GiB, swap zero, um job e comandos pesados sequenciais. Esta é correção interna/paridade ADR-0127, com helpers crate-private e nove pares L0/consumer; nenhuma assinatura externa, default deliberado ou fase nova. Resselo restrito, sem commit/push/release. As demais pendências de V16 continuam abertas.


## Raiz Table com origem transparente — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `lab/results/construction/diagnostics/tekt-v16-eval-math-2026-10-17/before.json.gz`. AST do parser pinado em contexts.json/all-contexts.json/wildcard-inventory.json; definições atuais por hash. A suíte anterior passou no mesmo estado executável, revalidado por hashes antes de decidir. Sem inferir intenção upstream ou benefício cognitivo da IA a partir do lint.

source-table-before.json registra a sonda externa ligada à lib real pela API
pública introspect: Realized(Table(Heading hidden), Text replacement) retorna
zero headings; a mesma origem com Source retorna um heading ocultado. O núcleo
content/source-origin já exige transparência e contribuição única; o contrato
raso de Table consumida já proíbe indexar children/caption ocultados. A diferença
observável é query(heading), não igualdade Rust. As seis sondas CLI da etapa
tekt-v16-eval-helpers-2026-10-17 passaram, portanto eram evidência insuficiente
para refutar o defeito do carrier construído pelo contrato público. Inferência
causal: table_root recursa Located/ShowRecipe mas classifica Source como false,
escolhendo o walk profundo indevido. Refutar por índice diferente, perda de
visual, identidade/ordem/counter errado ou efeitos duplicados após normalizar.
A primeira tentativa externa usou helper crate-private e falhou E0624; é erro
de montagem da sonda, nunca RED funcional. A construção pública posterior mede
o defeito e tem receita/rlib/binário/saída/UTC preservados.

### Prescrição exclusiva vigente

No helper local table_root do ramo Realized, trocar somente o scrutinee
`match c` por `match c.source_body()`. Desembrulhar Source por empréstimo antes
de classificar Table, Located e ShowRecipe; preservar todos os braços nominais,
corpos e ordem existentes. Não classificar todo Source como Table, não retirar
o carrier armazenado, não mover fase, não percorrer corpos semânticos ocultados,
não alterar reserva, forced_location, shallow_table, tag própria, assinaturas,
payloads, queries, synthesis ou comportamento de outro elemento. Source que
contém Heading continua não-Table e conserva a travessia original e visual.
Supersede a exclusão de table_root no lote anterior somente para esse reparo
interno da transparência já vigente, sem contrato público/default novo.

### Testes antes do reparo e aceitação

Adicionar neste consumer quatro testes puros pela API pública de introspecção:
Table crua consumida conserva tag própria e zero heading ocultado; Source único,
aninhado e nas duas ordens com Located e dentro de ShowRecipe não publica o
heading ocultado e conserva uma tabela; visual Heading permanece consultável
e Location explícita da Table se conserva; Source(Heading) não-Table conserva
origem e visual. RED real antes de mudar scrutinee; GREEN depois, sem mudar
expectativas. Preservar os controles Source de contexto anteriores e demais
testes. O reparo altera uma seleção privada de travessia, na mesma fase, para
restaurar contrato já ratificado: ADR-0127 contínuo. Resselo dos dois pares
somente após grafo V15/V26 válido. Repetir a sonda pública com 0/0, as seis
sondas CLI e os gates workspace/Clippy/Rustdoc/build/fmt/linhagem/reversa/SARIF.
Nenhum V20 ou diagnóstico adicional; 12 GiB, swap zero, um job e sem publicação.


### Refinamento medido — contexto de ShowRecipe em raiz Table

A primeira validação deste lote (attempt-recipe/validation-controls.log) mede
4 controles passando e 1 falhando. Source direto/aninhado e nas duas ordens
com Located deixam de publicar hidden; ShowRecipe(Source(Table)) ainda publica
uma heading ocultada. O diagnóstico anterior da raiz não abrangia o transporte
do contexto em ShowRecipe: esse ramo de walk_at delega a walk e reinicializa
forced_location=None e shallow_table=false. Estado/UTC/hashes e a falha estão
preservados antes deste refinamento; não se altera o RED original.

Antes do código, acrescentar controle recipe-plain à matriz de carriers e teste
Located(ShowRecipe(Source(Table))) com visual Heading. A Table conserva a
Location explícita, continua única e não publica o heading hidden; visual shown
permanece consultável uma vez. Confirmar RED adicional sem enfraquecer controles.

Trocar exclusivamente a delegação do ramo ShowRecipe de walk para walk_at,
passando os mesmos acumuladores, lang, chain, label_from_parent e os valores
recebidos forced_location/shallow_table. Não observar/alocar Location do carrier,
não aplicar callbacks/recipes, não trocar chain por estilos capturados nem
mudar reserva ou assinatura. O corpo continua projetado uma vez na mesma fase.
É reparo privado de contexto da identidade já preparada, equivalente à regra
vigente de Source; sem API pública, default novo ou compatibilidade nova.
Esse refinamento complementa c.source_body de table_root; não o substitui.

Replay autoriza essas duas expressões privadas e os controles novos, sem outras
mudanças. Os seis testes selecionados devem passar após o reparo; cinco novos
são deste owner e um do owner de eval. Suíte workspace esperada cresce em seis
testes sobre o estado de entrada, sem alterar os casos antigos ou os sete
ignorados. Preservar build/fmt/Clippy/Rustdoc/linhagem/reversa/SARIF e 12 GiB.

## A1 — callbacks de suplemento de Heading, 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c e working tree/UTC/hash
registrados em lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
before-pdf-probe.json.gz: heading-supplement-panic termina em sucesso no Tekt
e em A1_HEADING no vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-supplement-valid aceita ambos, mas só o vanilla usa Custom na referência.
Fonte upstream model/heading.rs:249-289 sintetiza supplement por ocorrência e
model/reference.rs:393-415 aplica a função e display. Set.rs:286-320 ignora
supplement e limpa numbering; snapshot_fields heading.rs:55-68 fixa tradução.
Isto é callback/valor observável, não exigência de mecânica upstream.

### Decisão e responsabilidade exclusiva

Ao capturar Heading com supplement custom, conservar no content da entrada
canônica um Styled com o delta causal colapsado existente, além do snapshot de
campos; não criar mapa concorrente de snapshots nem campo público. A entrada temporária permite ao pós-processador restaurar os styles causais
da função; o wrapper de captura é retirado antes de publicar Heading.
Somente a ocorrência com especificação custom recebe esse wrapper; a árvore
produtiva original e as tags/Locations não mudam. Walk continua sem Engine e
não executa callbacks.

Correção de paridade em fluxo ADR-0127: sem nova API Rust, campo público ou
transferência de lógica existente entre eval/layout. O pós-walk com Engine já
é a fronteira de aplicação de callbacks. L0 primeiro, RED e controles bilaterais,
sem callbacks em encoder/walk/layout e sem alegar paridade de todos os campos.


## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

Guardar Content semântico intacto na entrada canônica; não criar Styled para
transportar causalidade de Heading. Quando houver heading.supplement custom,
anexar os styles causais ao cabeçalho privado do snapshot existente. Preservar
mapa completo/ordem/label por último e Location; não criar segundo store nem
executar callback no walk. O entrypoint puro projeta Heading como Heading.

### Aceitação e limites

H1–H8 da revisão ratificada, RED bilateral antes do código. Nenhum contrato
público além da fachada Heading está autorizado por H2; A1-S2 cobre separadamente
cinco helpers State. Sem novos campos públicos. Styled ordinário e demais
callbacks conservam suas fronteiras. Workspace/doctests/build/Clippy/Rustdoc,
fmt, V1/V5/V15/V26/reversa e V16 sem aumento; 12GiB/High8GiB/swap0/jobs1,
comandos pesados sequenciais. Ganho de RAM exige medição, não intenção.
