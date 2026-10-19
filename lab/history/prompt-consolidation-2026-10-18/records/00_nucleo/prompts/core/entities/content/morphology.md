# L0 — entities/content/morphology
Hash do Código: c55097d1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/projection-exhaustiveness.toml sha256:71273fd4ed96ebc9bb2910e90f03cde1ecdd3543d7cf02a934c945f2c159a880
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:1d697fae328277525e44cad674cecc537ceb23a1bb3ae11bb68a0a9575411ae4


**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/morphology.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own `Content::morph_canon`, the canonical form used for language-level
morphology. Preserve ADR-0107: rendering mechanics and baked style details do
not become language identity. The implementation must preserve every
canonical form and equality outcome.

## Acceptance

Existing morphology and language-equality tests pass and the module contains
simultaneous verification.


## Realização com avaliação única

Canonicalização morfológica projeta body/visual dos carriers internos, sem usar identidade de ocorrência como morfologia. Preservar a terminação P348.

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

Source spans não entram na igualdade da linguagem. Campos resolvidos observáveis não são apagados por serem derivados: raw cell != it preparado conforme resolved-equality; duas raw cells iguais == true. Visual de show/flags de estágio são mecânica/render, não campos da linguagem. Canonização usa projeção nominal dos campos conhecidos do owner da Cell; não confundir novas coordenadas conhecidas com bytes físicos. Não alterar política global de terminação nesta fase. Table-root-equality é refutador de defaults gerais de Table fora da certificação deste recorte.

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

Canonização de Cell observa propriedades conhecidas da linguagem, incluindo defaults avulsos: cru != preparado por conhecimento de campos, não por flags/render. Apagar source_span/guards/stage/visual de show da igualdade da linguagem, preservando distinção stroke omitido preparado {} versus None e propriedade Auto versus desconhecida. Dois snapshots com mesmas propriedades e body são iguais independentemente de histórico de regras; uma Cell lexical completamente explicitada e o preparado equivalente devem comparar pela mesma projeção conhecida. Não borrar defaults Table nem transformar dados em despacho dinâmico; não mudar canonização de outros elementos. Refutar com getters/fields/igualdade e não com PartialEq do Rust.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.

## D08B3-C — defaults declarativos da Cell, fluxo contínuo

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-styles-d08b3c-2026-10-07/before.json.gz registra 58 sondas, UTC 2026-10-07T15:24:58.038947+00:00 até 15:25:13.727872+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/status/fontes/argv/cwd/ambiente completos. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3. isolated/table-set, campos individuais, late/local/reuse/spread e rejeições refutam o warning de target vazio e a ausência dos defaults. padding-span-one/empty-fixed-rows fixam spans 1 em vazios; auto-table-fallback preserva Auto explícito como delegação à Table, não ao set da Cell. show-cell-set-self altera avulsa mas não fill conhecido da Cell posicionada. Stroke/alignment fazem fold. refuters-before.json.gz estende com header, partial inset, args/condição e snapshot em Table; alias-inside-table é válido no vanilla e recursa indevidamente no Tekt.

Fonte: upstream typst-library/src/model/table.rs:734–778 declara campos, e layout/grid/resolve.rs:203–278 lê estilos antes de resolver/sobrescrever posições/fill; defaults de padding TableCell packed em model/table.rs:785–800 pinam spans 1. Tekt rules/set.rs:23–27 usa text_str de target e não reconhece FieldAccess; table_cells.rs:262–310 usa apenas propriedades lexicais no placement/herança; grid_placement.rs:80 lê x/y/spans diretamente; TableCell standalone não possui propriedades efetivas privadas. É língua (campos/ordem/efeitos/estilo/erros); a representação privada abaixo é decisão Tekt, não obrigação de copiar Packed ou igualdade Rust. Hipótese refutável por qualquer negativo destas coletas, perda de fonte lexical, mutação de valor cru, efeito repetido, guard de alias perdido ou regressão de geometria/cache.

### Responsabilidade exclusiva

Canonizar propriedades conhecidas efetivas, ignorando guards/stage/visual/source_span. Usar projeção tipada privada, normalizar Auto/omissão conhecida/spans 1; não introduzir desigualdade mecânica entre snapshot que tem/que não tem propriedades privadas equivalentes. Preservar dict stroke vazio e inset parcial, distinguindo omissão de edge e zero. Raw totalmente explicitado equivale ao preparado de mesmos campos; raw ainda desconhecido difere. Outros tipos conservam política global de morfologia/terminação.

### Gate, aceitação e limites

Correção interna de paridade/mapeamento, ADR-0127 contínuo: nenhum contrato público, compatibilidade externa, flag/default de produto ou fase nova. Props e helpers são privados em carrier já não-construível externamente; lexicais não mudam. L0 editado primeiro, resselo restrito e RED→GREEN obrigatórios. Coletas bilaterais com erros/caret/efeitos, fonte/binaries/HEAD/UTC, geometria real; build debug/release/suítes e lint sem erros. Preservar árvore anterior sem commit/reset/resselo global. Subconjunto certificado: inteiros/autos, Color/None/Auto, Stroke uniforme/None, alinhamento, Length/inset parcial de Length/Auto, Bool/Auto e schemas já tipados. Gradient/tiling/functions, strokes por lado, inset percentuais e rich casts de constructor, grupos/spans multipágina/PDF-UA, set/text posicional e defaults gerais de Table continuam incompletos, com medição/L0 próprios antes de ampliar. Não certificar por warning fallback nem copiar defeitos upstream.

Precisão medida em refuters-before: stroke-constructor-fold e align-constructor-fold preservam o construtor inteiro contra set da Cell. self-set-does-not-change-placement muda colspan observado para 2 após a ocupação já feita com 1; não é prova de nova ocupação.

### Precisão dos spans observados após show-set (refutador C)

Medição anterior: refuters-before.json.gz / self-set-does-not-change-placement, vanilla devolve colspan 2 na célula real x0 e colspan 1 nos paddings x1/x2. Fonte pinada layout/grid/resolve.rs:218–244 captura spans de ocupação mas não assenta seus campos no conteúdo; model/table.rs:785–800 assenta 1 nos paddings. Logo ocupação e getter posterior são observáveis distintos. Decisão privada: a Cell posicionada conserva TableCellResolved como plano físico; o carrier standalone privado pode coexistir apenas para propriedades observadas da receita, mantendo campos resolvidos e sobrescrevendo spans lexicalmente omitidos pela declaração de show-set. Getters/canonicalização consultam essa projeção; placement do plano completo continua usando TableCellResolved. Padding sem fonte tem spans lexicais 1 para que show-set não os altere. Guards/stage continuam causais, sem novo contrato público nem mutação da fonte original. RED de span getter antes de implementar, GREEN mais geometria/regressões obrigatórios.

### Fold de declarações de Stroke — medição extra C

extra-first.json.gz (fontes/HEAD/estado/UTC/binários completos) compara two-stroke-sets: set stroke:red seguido de 2pt produz 2pt + red no getter avulso e posicionado vanilla. stroke-render-inherits-cell-style resolve o constructor 2pt com paint red da Cell antes do fallback da Table 3pt, embora stroke-constructor-fold avulso exponha somente o constructor 2pt. Fonte pinada model/table.rs:765 declara #[fold]; resolve.rs:225–237 usa stroke.resolve(styles), distinto do getter cru. Decisão: declarações sucessivas foldam componentes specified ao publicar Value::Stroke declarativo; nenhum carrier/callback é guardado em Styles. Constructor stroke permanece inteiro no getter avulso; resolução posicionada folda o constructor com a base capturada Cell e depois Table. Adicionar somente campo privado stroke_inherited: Option<Stroke> em TableCellProperties para transportar essa base causal, sem assá-la no getter lexical. ObservationEq inclui base; canonicalização ignora base derivada de render. Table já resolvida tem base None. Clone/aliases conservam base anterior; None explícito vence. RED→GREEN antes de código; correção privada contínua ADR-0127, nenhum contrato público/fase/default novo. A cláusula de precedência inteira do constructor vale para o getter avulso, não para a resolução fold de stroke, conforme medição.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Source é transparente nesta projeção, delegando ao body; não expõe span como identidade de língua nem executa receitas. Segmentação conserva origem por segmento quando aplicável.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## V16 — projeções explícitas de Content (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/tekt-v16-content-views-2026-10-17/before.json.gz registra
UTC/status/diff HEAD --stat, fontes/L0 e 2129 hashes conferidos. contexts-before.json
mapeia os seis V16 por AST/linhas; projection-plan.json guarda o domínio real
Content (120 variantes) e os complementos por função. before-probe-result.json
conserva 17280 controles simbólicos de padrões e guards: o mutante Content
futuro compila nos seis locais (zero E0004 selecionados). É mecânica de proteção
de domínio, sem nova prova de paridade ou de leitura por IA. Payloads são
opacos no modelo; ele não substitui controles reais dos algoritmos.

before-field-probe.json usa a rlib atual e o programa field-control.rs com hash,
argv, UTC e source freeze: 48 de 84 leituras diferem quando Source, Located,
ShowRecipe ou Realized estão dentro de Styled/Styled aninhado. Fora de Styled,
os quatro já preservam campos. fields.rs:10–13 reconhece só Equation/Styled;
seu guard recusa os carriers antes da delegação. O núcleo source-origin já
exige transparência; o owner fields já exige body/visual dos outros três.
É invariância de campos da língua, não exigência de estrutura Rust vanilla:
esses carriers são internos e não têm syntax própria no pin. Não inferir
intenção do defeito; refutam a correção perda de precedência, ausência/none,
campos indevidos em não-equações, mudança do visual para original ou execução
de receitas. O pin de linguagem permanece 9f2b6e8715237cb086899a42873660fe744622e8;
esta medição metamórfica não é declarada comparação bilateral nova.

### Responsabilidade exclusiva

Em morph_canon, conservar a transformação e toda política de tabela/termi­nação. Substituir somente o None final pelo complemento nominal de 112 variantes, incluindo Styled quando o guard semântico recusa. Não alterar formas canônicas, estilos, source spans, recipes ou walkers.

A lista nominal por função, em ordem da declaração vigente, é derivável das
variantes não cobertas incondicionalmente pelos braços anteriores; guardar
Styled no complemento se o guard puder recusar. Nos campos, variantes com
nome literal ou guard conservam também o caso de campo desconhecido; variantes
já delegadas para qualquer f ficam fora do complemento. O domínio string
continua aberto. O núcleo pinado projection-exhaustiveness partilha esta regra;
o owner continua exclusivo e source-origin conserva sua obrigação vigente.

### Aceitação e limites

L0 antes de testes/código; fluxo contínuo ADR-0127, sem campo/assinatura/trait
público, compatibilidade, default ou fase nova. RED funcional de fields antes
de código, sem usar falhas de montagem de sonda como RED. Após o reparo, os 84
controles reais de carriers devem coincidir; testar precedência de estilos e
não-equações (Realized consulta visual, não original). A sonda estrutural deve
passar de zero para seis E0004 selecionados ao adicionar uma variante somente
no modelo descartável. Seus 17280 casos simbólicos conservam a classificação,
exceto os 96 casos do helper que passam a reconhecer os quatro carriers;
cada variante de cada complemento V19 deve ser exercitada separadamente.
Essa prova não é equivalência de payloads: exige replay dos corpos anteriores,
regressão real e invariância dos algoritmos não alterados. Não adicionar
matriz produtiva que apenas espelhe o match.

Esperado: remover exatamente seis V16, acrescentar seis V19 nominais uniformes
prescritos, preservar os demais diagnósticos por fonte/regra/severidade/mensagem
e não acrescentar V20 ou supressão. Grafo V15/V26 antes de resselo dos cinco
pares. Build, testes focais e workspace, Clippy all-targets -D warnings/-D
unfulfilled_lint_expectations, Rustdoc estrito, fmt, linhagem estrita/reversa,
replay, source freeze e diff --check. Pesados sequenciais MemoryMax12GiB,
High10GiB/swap0/jobs1. Sem commit/push/merge/release. As frentes de linguagem,
contratos pendentes e o objetivo global não são encerrados por este lote.
