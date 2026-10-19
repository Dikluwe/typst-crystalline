# L0 — compiler/eval/observation/elements
Hash do Código: e0ab26b9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:f29e26ff07053f940cfe4ab5358fc9a6ac8d4d689528e169b5f8567492f52f46
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:1d697fae328277525e44cad674cecc537ceb23a1bb3ae11bb68a0a9575411ae4
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:cbd8b56af347a35e867b2c4695581721d5983ec24ee7bb022e9f9987500d40f3

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/elements.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/elements.rs:11` — `impl ObservationEq for crate::entities::elements::align::AlignElem {`.
- `01_core/src/compiler/eval/observation/elements.rs:19` — `impl ObservationEq for crate::entities::elements::underline::UnderlineElem {`.
- `01_core/src/compiler/eval/observation/elements.rs:29` — `impl ObservationEq for crate::entities::elements::v_space::VSpaceElem {`.
- `01_core/src/compiler/eval/observation/elements.rs:38` — `impl ObservationEq for crate::entities::elements::enum_item::EnumItemElem {`.

## Contract

Own observation of the remaining non-math, non-page element carriers. Compare
variant identity and every causal field, including BlockHeight and ImageElem.alt.
Distinguish absent alt, an empty string and nonempty text. Do not compare
derived frames, execute callbacks/layout or substitute textual names for
identity. Dynamic elements without immutability guarantees remain Unproven.
Nominally exhaustive destructuring must prevent silent field omissions.

BlockHeight compares Relative/Fractional nominal variants and their payloads;
different variants are Different without resolving a reference or allocation.
Image alt remains distinct independently of identical image bytes/geometry.
Boxed observation includes its fractional width carrier.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.


## Aspa semântica e resolução tardia — ratificada em 2026-10-06

O contrato desta seção é vigente e supersede as cláusulas históricas conflitantes
das seções anteriores apenas para SmartQuote. A ratificação humana autoriza a
materialização em fluxo RED→GREEN, com ownership exclusivo e pins abaixo.

### Medição anterior à decisão

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
registra pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 dos binários,
HEAD/working tree, diff HEAD --stat, UTC, fontes, argv/cwd/ambiente e stdout/stderr.
PDF usa texto extraído, HTML usa DOM/texto e diagnósticos, não igualdade de bytes PDF.
A avaliação converte aspas de markup para Text; campos double/enabled são perdidos.
A variável contendo aspas mantém glyph inglês sob uso em alemão/francês ou
smartquote(enabled:false). Primes 6'/2" viram aspas no Tekt. Show da função
nativa e nome público foram corrigidos separadamente e passam antes deste gate;
markup ainda não é SmartQuote. Constructor vazio tem fields vazio no vanilla;
double:false é campo conhecido, mas double omitido é Unset. enabled:false
permanece SmartQuote no vanilla. set double:false e show filtrado têm positivos
novos. HTML literal funciona porque recebe Text; SmartQuote programática hoje
é rejeitada. Título literal renderiza aspas; script recebe string, rejeitando
Content antes da exportação nos dois binários.

Fontes: markup_stream.rs:79–128 decide glyph em eval; smartquote.rs de layout
alterna dois flags sem texto precedente; stdlib/text/smartquote.rs:112–117 apaga
elemento disabled; observation/elements.rs:242–250 compara os campos atuais.
Vanilla typst-eval/markup.rs:141–146 cria elemento com double explícito;
typst-layout/inline/collect.rs:196–211 resolve estilo/contexto na coleta;
typst-library/text/smartquote.rs:119–158 distingue prime/apóstrofo/nesting.

### Decisão ratificada e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. Confirmação humana recebida em 2026-10-06. O contrato ratificado abaixo
supersede cláusulas históricas conflitantes apenas para aspas; ADRs/núcleos
existentes e propostas alheias permanecem intactos.

Destructure exaustivamente SmartQuoteElem com double/enabled/alternative/quotes;
observar todos os Option nominais via ObservationEq, distinguindo None de
Some(true/false), mesmo com render igual. Sem callback/render ou campo esquecido.

## D08B2 — presença lexical ratificada em 2026-10-07

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz e
presence-refuters.json conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree, diffstat, UTC, binários e argumentos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Upstream model/table.rs:734–778 possui campos Smart/Option; o Tekt
entities/elements/table_cell.rs:40–51 conserva apenas Option de valor.
X:auto e x omitido colapsam; fill:none e stroke:none perdem presença.
Os refutadores medem auto permitido em x/y/fill/align/inset/breakable,
rejeitado em colspan/rowspan/stroke; none permitido só em fill/stroke.
São observáveis da linguagem, sem obrigação de copiar a estrutura upstream.

### Contrato ratificado e responsabilidade exclusiva

Destructure TableCellElem exaustivamente e comparar presence nominalmente, além de body e valores atuais. Células com pixels iguais mas presença distinta não podem produzir prova Equal. Nenhum callback/render.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Contrato público por campo/tipos novos confirmado pelo humano em 2026-10-07; recibo table-cell-presenca-aprovacao-2026-10-07.json. Não altera fase/defaults de render.

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

Destructure Table/Cell e novos carriers exaustivamente; observar source_span, rows_explicit, planos, propriedades, visual e estágios nominais, sem callback/render. Não provar reuso pelo texto ou Hash/PartialEq da linguagem.

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

## D08B3-B — candidato, não vigente antes de ratificação

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

isolated-default: vanilla expõe x/y/inset/align/fill/breakable auto, spans 1 e stroke dict vazio; Tekt omite esses defaults e publica duas metadata contra uma. isolated-x rejeita no Tekt, funciona no vanilla. raw-fields continua só body nos dois e raw-x rejeita nos dois. isolated-table-style confirma que defaults de Table não viram os da Cell avulsa. raw-vs-prepared mede false no vanilla e true no Tekt. nested-effects/body-effects/late-rule refutam execução única e origem lexical. fresh-isolated/table, fresh-block, fresh-styled e multi-fresh são aceitos indevidamente pelo Tekt; vanilla rejeita maximum show rule depth exceeded com span do constructor novo. identity/body/alias e fresh-conditional/hidden continuam válidos. multi-identity e multi-return-table mostram que devolver it deve permitir receita anterior ainda não consumida. isolated-query mantém cell is not locatable nos dois; location é none, sem localização falsa. cell-set é negativo separado, explicitamente excluído deste pacote.

Fonte vanilla lab/typst-original/crates/typst-library/src/model/table.rs:734–778 declara defaults e propriedades; typst-realize/src/lib.rs:367–386 prepara/guarda o argumento, :418–429 visita produto com profundidade, :491–505 distingue guards por elemento, :571–586 materializa propriedades. Tekt entities/elements/table_cell.rs:111–143 só projeta lexical ou plano posicionado; compiler/realize/table_cells.rs:110–129 reaplica Cell crua sem preparo; :280–304 termina snapshot inteiro; eval/show_rule_termination.rs:65–70 trata mesma forma como ponto-fixo; eval/rules/apply.rs:139–166 revoga regra globalmente durante callback. Classificação: campos, efeitos, ordem, terminação e erro causal são língua. A estrutura de guards e materialização upstream é mecânica e não é requisito. Não se infere intenção do upstream; a escolha nominal abaixo é inferência refutável pelos controles de aliases, duas regras, novas gerações e conteúdo ocultado.

### Responsabilidade exclusiva proposta

Destructure exaustivamente TableCellElem, TableCellStandaloneResolved e TableCellResolved; observar presença, visual, stage e guards causalmente além dos demais campos. ObservationEq usado para reuso não apaga o estado que igualdade da língua ignora; não provar equivalência por texto/pixels/Hash. Nenhum callback/render.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.

## D08B3-C — defaults declarativos da Cell, fluxo contínuo

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-styles-d08b3c-2026-10-07/before.json.gz registra 58 sondas, UTC 2026-10-07T15:24:58.038947+00:00 até 15:25:13.727872+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/status/fontes/argv/cwd/ambiente completos. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3. isolated/table-set, campos individuais, late/local/reuse/spread e rejeições refutam o warning de target vazio e a ausência dos defaults. padding-span-one/empty-fixed-rows fixam spans 1 em vazios; auto-table-fallback preserva Auto explícito como delegação à Table, não ao set da Cell. show-cell-set-self altera avulsa mas não fill conhecido da Cell posicionada. Stroke/alignment fazem fold. refuters-before.json.gz estende com header, partial inset, args/condição e snapshot em Table; alias-inside-table é válido no vanilla e recursa indevidamente no Tekt.

Fonte: upstream typst-library/src/model/table.rs:734–778 declara campos, e layout/grid/resolve.rs:203–278 lê estilos antes de resolver/sobrescrever posições/fill; defaults de padding TableCell packed em model/table.rs:785–800 pinam spans 1. Tekt rules/set.rs:23–27 usa text_str de target e não reconhece FieldAccess; table_cells.rs:262–310 usa apenas propriedades lexicais no placement/herança; grid_placement.rs:80 lê x/y/spans diretamente; TableCell standalone não possui propriedades efetivas privadas. É língua (campos/ordem/efeitos/estilo/erros); a representação privada abaixo é decisão Tekt, não obrigação de copiar Packed ou igualdade Rust. Hipótese refutável por qualquer negativo destas coletas, perda de fonte lexical, mutação de valor cru, efeito repetido, guard de alias perdido ou regressão de geometria/cache.

### Responsabilidade exclusiva

Destructure exaustivamente o carrier standalone com properties e o schema privado fechado; observar todos os valores/presença/edges/estado causal, sem callback/layout. Prova de reuso não é igualdade da linguagem; nenhuma omissão silenciosa de novo dado privado.

### Gate, aceitação e limites

Correção interna de paridade/mapeamento, ADR-0127 contínuo: nenhum contrato público, compatibilidade externa, flag/default de produto ou fase nova. Props e helpers são privados em carrier já não-construível externamente; lexicais não mudam. L0 editado primeiro, resselo restrito e RED→GREEN obrigatórios. Coletas bilaterais com erros/caret/efeitos, fonte/binaries/HEAD/UTC, geometria real; build debug/release/suítes e lint sem erros. Preservar árvore anterior sem commit/reset/resselo global. Subconjunto certificado: inteiros/autos, Color/None/Auto, Stroke uniforme/None, alinhamento, Length/inset parcial de Length/Auto, Bool/Auto e schemas já tipados. Gradient/tiling/functions, strokes por lado, inset percentuais e rich casts de constructor, grupos/spans multipágina/PDF-UA, set/text posicional e defaults gerais de Table continuam incompletos, com medição/L0 próprios antes de ampliar. Não certificar por warning fallback nem copiar defeitos upstream.

Precisão medida em refuters-before: stroke-constructor-fold e align-constructor-fold preservam o construtor inteiro contra set da Cell. self-set-does-not-change-placement muda colspan observado para 2 após a ocupação já feita com 1; não é prova de nova ocupação.

### Precisão dos spans observados após show-set (refutador C)

Medição anterior: refuters-before.json.gz / self-set-does-not-change-placement, vanilla devolve colspan 2 na célula real x0 e colspan 1 nos paddings x1/x2. Fonte pinada layout/grid/resolve.rs:218–244 captura spans de ocupação mas não assenta seus campos no conteúdo; model/table.rs:785–800 assenta 1 nos paddings. Logo ocupação e getter posterior são observáveis distintos. Decisão privada: a Cell posicionada conserva TableCellResolved como plano físico; o carrier standalone privado pode coexistir apenas para propriedades observadas da receita, mantendo campos resolvidos e sobrescrevendo spans lexicalmente omitidos pela declaração de show-set. Getters/canonicalização consultam essa projeção; placement do plano completo continua usando TableCellResolved. Padding sem fonte tem spans lexicais 1 para que show-set não os altere. Guards/stage continuam causais, sem novo contrato público nem mutação da fonte original. RED de span getter antes de implementar, GREEN mais geometria/regressões obrigatórios.

### Fold de declarações de Stroke — medição extra C

extra-first.json.gz (fontes/HEAD/estado/UTC/binários completos) compara two-stroke-sets: set stroke:red seguido de 2pt produz 2pt + red no getter avulso e posicionado vanilla. stroke-render-inherits-cell-style resolve o constructor 2pt com paint red da Cell antes do fallback da Table 3pt, embora stroke-constructor-fold avulso exponha somente o constructor 2pt. Fonte pinada model/table.rs:765 declara #[fold]; resolve.rs:225–237 usa stroke.resolve(styles), distinto do getter cru. Decisão: declarações sucessivas foldam componentes specified ao publicar Value::Stroke declarativo; nenhum carrier/callback é guardado em Styles. Constructor stroke permanece inteiro no getter avulso; resolução posicionada folda o constructor com a base capturada Cell e depois Table. Adicionar somente campo privado stroke_inherited: Option<Stroke> em TableCellProperties para transportar essa base causal, sem assá-la no getter lexical. ObservationEq inclui base; canonicalização ignora base derivada de render. Table já resolvida tem base None. Clone/aliases conservam base anterior; None explícito vence. RED→GREEN antes de código; correção privada contínua ADR-0127, nenhum contrato público/fase/default novo. A cláusula de precedência inteira do constructor vale para o getter avulso, não para a resolução fold de stroke, conforme medição.

## Presença lexical Raw — ratificada em 2026-10-14

### Medição anterior à decisão

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes/binários e pin 9f2b6e8715237cb086899a42873660fe744622e8. RawElem raw.rs:21–25 perde omissão; native_raw markup.rs:78–136 colapsa ausência/false/none. Campos, igualdade e encoders distinguem esses valores no pin. Trata-se de semântica/morfologia, não mecânica Rust.

### Responsabilidade e aceitação

ObservationEq inclui set_fields; manter destruturação exaustiva e distinguir presença mesmo com valor igual.

Manifesto aprovado b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45, recibo em lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json. Esta seção completa apenas a presença anteriormente incompleta de Raw e supersede as cláusulas de ausência de máscara. Preservar contratos alheios; sem fase/default/render novo. Validar testes RED→GREEN, suíte, build e linhagem.

## Transformações tipadas — contrato aprovado

### Medição anterior à decisão

O corpus transformacoes-2026-10-14/results.json.gz e controles suplementares
registra identidade/presença perdida no carrier matrix/body. Fonte e owner
pré-migração estão em lab/history/construction/records/00_nucleo/materialization/transformacoes-tipadas-2026-10-14/base-files.json.gz
com hashes e linhas em consumers.json. A aprovação do manifesto
960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d autoriza
a nova representação e resolução no layout. Semântica/morfologia são linguagem;
formato Rust e mecanismo de reconstrução são mecânica.

### Responsabilidade exclusiva

Comparação conservadora de Transform distingue variante, presença e cada valor tipado antes de observar body. Matrix compara todos os coeficientes. Não igualar operações pela matriz final.

Testar RED→GREEN e revalidar os caminhos dependentes; o build não fecha X1–X5.
