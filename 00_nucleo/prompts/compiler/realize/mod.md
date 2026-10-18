# L0 — compiler/realize/mod
Hash do Código: 04f3e4a5



**Camada:** L1.
**Consumer produtivo exclusivo:** `01_core/src/compiler/realize/mod.rs`.
**Governança:** ADR-0026, ADR-0029, ADR-0030, ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 e ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:825182e1dfe3591249a5efbc4c0ddbd937ebffde953c6595e0d1ed8176106435
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Medição que precede a decisão

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, `01_core/src/compiler/eval/mod.rs:872–880` avalia o programa sem show e depois com show. `03_infra/src/query_helpers.rs:511–520` escolhe entre os índices original e renderizado por cardinalidade. A auditoria `00_nucleo/diagnosticos/dupla-avaliacao-auditoria-2026-10-05.md` preserva controles e proveniência: uma receita que produz um heading faz o cristalino perder o gerado; uma que produz dois headings faz perder o original. Os corpos e comandos completos estão em `dupla-avaliacao-auditoria-2026-10-05/results.json.gz`.

O vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` avalia o principal antes da estabilização (`lab/typst-original/crates/typst/src/lib.rs:124–132`). Sua realização prepara identidade antes de aplicar a receita e conserva tags do original enquanto visita o produto (`lab/typst-original/crates/typst-realize/src/lib.rs:353–430`). Isso demonstra o mecanismo de referência; não obriga copiar vtables, unsafe, igualdade Rust ou passos exatos do algoritmo.

## Decisão de domínio

Separar execução do programa de realização de receitas. A avaliação executa o código comum uma vez e transporta conteúdo e receitas com ambiente lexical imutável. A realização pura aplica receitas, conserva ocorrências originais e geradas e devolve o conteúdo visual associado ao índice semântico. Layout e exportação não pertencem a este consumer.

Não obter o original reavaliando o arquivo, comparando tamanhos de árvores ou reconstruindo registros pelo texto final. Não usar igualdade morfológica como identidade de ocorrência. Dois elementos com campos e corpo iguais podem ser ocorrências distintas.

## Fronteira pública

O consumer possui a fachada L1 de realização e seu resultado:

```rust
pub struct RealizedDocument {
    pub visual: Content,
    pub introspector: TagIntrospector,
}

pub fn realize_document(
    content: &Content,
    world: &dyn World,
    source: &Source,
    registry: &ElementRegistry,
    previous: &TagIntrospector,
    full_error: bool,
    target: EvalTarget,
    features: Features,
) -> (SourceResult<RealizedDocument>, Vec<SourceDiagnostic>);
```

Usar os tipos existentes, com seus caminhos canônicos: Content em entities/content, TagIntrospector em entities/introspector, World em contracts/world, Source em entities/source, ElementRegistry em entities/element_registry, EvalTarget em compiler/eval e Features em entities/compiler_features. A implementação não muda esses contratos por conveniência. Recursos e configuração são recebidos explicitamente, sem filesystem, relógio, ambiente, rede ou globals mutáveis.

O snapshot previous contém os resultados contextuais da tentativa anterior. Receitas e contribuições dependentes desses resultados podem precisar de outra tentativa; isso não autoriza executar novamente o programa comum. Um resultado anterior inválido não se acumula no índice da nova tentativa.

## Transporte e ownership

O enum fechado e os payloads imutáveis que transportam conteúdo, receitas e proveniência pertencem exclusivamente a `00_nucleo/prompts/entities/content.md`. Contextos, scopes e captura lexical da avaliação pertencem exclusivamente a `00_nucleo/prompts/compiler/eval.md` e aos owners de markup, rules e closures. Este consumer não redefine entidades nem seus construtores, igualdade, hashing, projeções ou travessias.

O transporte precisa conservar corpo, regras em ordem lexical, estilo efetivo, guards, arquivo causal, ambiente capturado pelas funções e distinções de presença. Não usar PropMap, vtable, despacho dinâmico para nativos ou valores de render como substitutos de morfologia. Transporte interno não pode se tornar um novo elemento observável da linguagem nem alterar repr/casts/query dos elementos envolvidos.

## Identidade e realização

Antes de consumir visualmente um elemento locatável, registrar seu snapshot observável e associar sua identidade à ocorrência. A identidade não depende de coordenadas de layout, texto, span sozinho ou endereço de memória. A localização geométrica é associada depois pelo consumer de layout/pipeline. Repetir realização conserva a correspondência das ocorrências estáveis, sem prometer estabilidade entre edições não relacionada ao contrato existente de Locator.

Receitas recebem o elemento original com a proveniência de sua ocorrência. Se devolvem esse elemento sem substituí-lo, não registrar uma ocorrência fantasma adicional. Se produzem outros elementos, cada ocorrência produzida recebe identidade própria. Conteúdo ainda não preparado inserido mais de uma vez recebe ocorrências distintas. Referências adicionais ao mesmo it já preparado conservam a identidade original, mesmo com múltiplas emissões visuais. A sonda query double_alias no pin 9f2b6e871 retorna uma ocorrência; cloned_content retorna original e dois produzidos. Preservar identidade durante transporte não autoriza colapsar duas inserções por igualdade.

Registrar originais e elementos gerados em ordem causal de realização. O índice conserva campos, labels, relações e localizações observáveis; não escolher um dos índices por quantidade. Conteúdo visual antigo de um elemento substituído não é emitido novamente para conservá-lo consultável.

Preservar estilos lexicais, contexto do original e guards ao aplicar uma receita. A implementação de matching e terminação continua nos respectivos owners; não duplicar seu algoritmo. Preservar as divergências intencionais de linguagem já ratificadas, inclusive o contrato de terminação, até decisão nova medida. Erros e warnings mantêm spans, ordem causal, canais e limites existentes.

## Coordenação

L3 coordena avaliação, realização, layout e atualização do snapshot de páginas/posições. L1 decide a transformação e a identidade; L4 apenas compõe serviços. A mesma realização semântica serve PDF, HTML, bundle e query, respeitando target/features. Queries consultam o índice realizado e transportam os campos capturados; não fazem uma segunda avaliação nem contam árvores aproximadas para decidir sua composição.

Uma compilação sem receitas ou dependência contextual deve conservar seu conteúdo sem reconstrução desnecessária. Esse comportamento deve ser demonstrado por contagens de execução e validação, não presumido pela ausência de texto `#show` no arquivo principal.

## Aceitação linguística e de execução

- Heading substituído por texto continua consultável com corpo original.
- Um original que produz um ou dois headings conserva original e produzidos em ordem correspondente ao oráculo pinado.
- Dois headings iguais, duas inserções de um conteúdo e elementos com span compartilhado continuam distinguíveis.
- Receita que devolve it, regras aninhadas, labels, seletores where, show-set, estilos locais, imports/includes, matrizes e conteúdo contextual preservam comportamento e ordem de efeitos.
- Documento comum, com e sem show, executa seu código não contextual uma vez. Um World de teste e contadores de chamadas comprovam execução real; scopes finais iguais não são prova suficiente.
- Repetição contextual não acumula registros antigos, perde originais ou duplica contadores por mera passagem do compilador.
- PDF, HTML, bundle e query passam pelos casos pertinentes de semântica, sintaxe e morfologia; geometrias continuam validadas onde são observáveis.

Ganho de desempenho é medido por ensaio pareado antes/depois com HEAD, working tree, hashes dos binários, fontes, comandos, horários, tempo e memória preservados. Não existe meta de ganho inventada; ausência de ganho e regressões precisam ser reportadas. Uma fixture com falha linguística não qualifica uma comparação de desempenho equivalente.

## Normalização das inserções visuais

Após aplicação das receitas, completar identidades de locatables produzidos e conservar a identidade de cada Located já preparado, inclusive nas inserções visuais adicionais do mesmo alias. Novos conteúdos ainda não preparados recebem novas Locations a cada inserção. A origem de Realized é snapshot imutável e não participa da contagem de inserções visuais; alias it conserva a identidade original em sua primeira inserção. Não confundir clone do carrier com ocorrência da linguagem.

Receitas textual=true delegam a realize_captured_text após realizar os elementos do grupo; textual=false reutiliza apply_show_rules.

Finalizar inserções visuais após receitas: achatar sequências produzidas mantendo a origem em marcador Realized com visual vazio, e reutilizar normalize_markup_spaces do owner textual. Proveniência visualmente vazia não vira fronteira textual. Não normalizar espaços crus de ContentBlock antes da sua inserção.

Ao realizar novamente uma árvore parcialmente realizada, conservar suas Locations e observar a faixa já usada no grafo de entrada, incluindo origens consumidas, antes de alocar novas ocorrências. Não reservar a faixa de tentativas inválidas do previous.

Finalização não mantém contador de inserções de aliases preparados: Located já carrega a identidade. Visitar cada conteúdo ainda não preparado atribui Location nova; carriers existentes conservam seus IDs sem segunda reconstrução no callback.

Antes de executar receitas, reservar também ids de ContextBlock já presentes ou cobertos por context_id_floor dos snapshots lexicais. Alocador contextual é compilation-local e não recomeça na faixa de conteúdo capturado.


## Reaproveitamento do índice realizado — 2026-10-05

Na sequência vigente `01_core/src/compiler/realize/mod.rs:197–200` indexa o visual antes de `eval/mod.rs:888–896` aplicar wrap_page_columns. Para que o resultado possua o mesmo conteúdo publicado pelo Module, executar o helper existente Content::wrap_page_columns após finish_occurrences/normalize_insertions e antes da construção do RealizedDocument e de seu índice. O helper permanece governado por `00_nucleo/prompts/entities/content/page-structure.md`; não duplicar a lógica nem aplicar o helper novamente no caminho realizado do eval. Revalidar originais/produtos, colunas e contextos contra reconstrução fresca.

## IDs de regras capturadas — 2026-10-06

Medição: observe_occurrences em mod.rs:89-107 reserva Locations e context_id_floor, mas omite IDs das regras; show.rs:382-383 aloca regras pelo contador de EvalContext. controls-first.json refuta uma regra local aplicada a alias quando o contador reinicia. Antes de executar qualquer receita, avançar next_rule_id acima de todos os RuleIds de ShowRecipe no grafo capturado, preservando a faixa de snapshots posteriores ainda não visitados. O owner apply mantém proteção por ocorrência e a revisão de terminação confirmada em apply-l0-approved.md.txt supersede a preservação da política histórica somente para realização produtiva de localizáveis. Este consumer apenas reserva IDs e delega aplicação; assinaturas e fase não mudam.

## Inserção matemática automática — proposta sujeita ao gate ADR-0127

### Medição anterior à decisão — D06, 2026-10-06

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
UTC/diff/estado/fontes/binários nas coletas canônicas
00_nucleo/diagnosticos/math-bare-2026-10-06/{before,additional,refuters}.json.gz.
Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8; proveniência no relatório
00_nucleo/diagnosticos/math-bare-medicao-2026-10-06.md.

before: frac, mat, sqrt, lr e op nus têm uma Equation no vanilla e zero no Tekt;
math.bold([x]) e texto comum têm zero nos dois. nested e nested-equation têm
uma nos dois; reuse tem duas no vanilla. block-style/local-block/number-style
resolvem o escopo de inserção. show-generated e context-generated expõem as
mesmas lacunas. additional: Equation explícita preserva o render da fração;
it.body é rejeitado no vanilla por maximum show rule depth exceeded, enquanto
it é válido; o show posterior afeta apenas a segunda inserção. refuters mede
valor frac cru e label original versus Equation explícita.

lab/typst-original/crates/typst-realize/src/lib.rs:245–265,302–334 transforma
Mathy antes de show fora de math. 01_core/src/compiler/layout/equation.rs:680–689
projeta math nu em texto; compiler/realize/mod.rs:155–199 prepara show/identidade
sem normalização math. entities/content/traversal.rs:14–108 é bottom-up e
recursa no corpo matemático; rules/apply.rs:564–607 trata apenas localizáveis
já existentes. Estes observáveis são semântica/morfologia/render de linguagem,
não requisito de vtable, buffers, igualdade Rust ou bytes PDF.

### Responsabilidade exclusiva

Instalar o módulo descendente privado bare_math, mantendo esta fachada como
coordenadora de receita, identidade e índice. Antes de preparar ocorrências ou
aplicar ShowRecipe, normalizar inserções math com o helper do owner bare_math,
respeitando styles e rules do snapshot da receita. Não aplicar o snapshot final
a todos os conteúdos, nem modificar a origem consumida de Realized.

Adicionar exatamente esta fachada pública pura, usando os tipos existentes:

```rust
pub fn realize_math_insertions(
    content: Content,
    ctx: &mut crate::compiler::eval::EvalContext,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> crate::entities::source_result::SourceResult<Content>;
```

Ela realiza a contribuição já avaliada no Engine causal recebido: normalização,
show produtivo e preparação de ocorrências com os alocadores existentes. Não
cria novo World, EvalContext global, Source, índice/layout ou segunda execução.
Modo produtivo e opções transitórias são restaurados ao retornar, inclusive em
erro; guards, sink, IDs e proveniência continuam no contexto fornecido. Reservar
carriers presentes antes de alocar, sem colisão com as ocorrências anteriores.
A assinatura de realize_document e RealizedDocument permanece exatamente igual.
A cláusula anterior de fase inalterada é supersedida somente pela preparação
math semanticamente anterior a show/índice, após ratificação deste pacote.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.

### Snapshot contextual e origem de receita — refinamento medido

after-additional context-show-render comprova perda do show externo; a coleta
contextual usa Engine novo com rules vazio, e ContextBlock não possui campo
de regras. Helper pub(crate) capture_context_recipe transporta snapshot lexical
por Styled/custom eval.context.recipe: Value::Content(ShowRecipe com body Empty),
apenas se há regra de Equation ativa, pelo helper estático de bare_math. No Engine de contribuição, extrair esse transporte e
aplicar rules/guards/file capturados, restaurando os valores anteriores em todo
retorno. Não converter metadado privado em elemento consultável/Location.

Captura deferida de Mathy guarda Span cru em custom eval.math.origin. Se a
realização dessa receita falha, somente erro detached recebe a origem capturada;
normalizar formatação e Source pelos formatters existentes. Não inventar spans
para falhas sem prova, não sobrescrever span conhecido de produto interno.

### Guarda de contribuição sem matemática

Medição delivered-infra.log: p1493_descendente_unico_leitor e fase_html_latched
falham após finish_occurrences alocar identidades adicionais em contribuição
HTML sem Mathy. Entrada realize_math_insertions não é realização geral: quando
o helper não encontra Mathy externo e não há snapshot lexical de Equation,
devolver o Content existente antes de observar/preparar/alocar ou aplicar show.
Se há snapshot de Equation mas não há alvo matemático, também conservar o
produto. Não mudar orçamento, execução, IDs ou seleção de contextos sem math.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
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

Registrar descendente privado compiler/realize/table_cells com owner próprio.
Adicionar exatamente a fachada pública pura:

```rust
pub fn realize_table_insertions(
    content: Content,
    ctx: &mut crate::compiler::eval::EvalContext,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> crate::entities::source_result::SourceResult<Content>;
```

Usa Engine/contexto causal recebidos e restaura modos/styles/rules/guards/file
em erro e sucesso; não reavalia fonte nem cria World/índice/layout novo. Realizar
inserções de Table nos pontos efetivos (incluindo produtos de show e context),
antes de indexação final, coordenando com bare_math sem substituir sua guarda.
Captura contextual privada existente pode conservar regras Table/Cell além de
Equation; nenhum estado resolvido é armazenado em Styles/custom. O plano só é
construído após a regra da tabela deixar uma Table para emitir; child/cell é
fronteira terminal até o seu momento, evitando a travessia bottom-up prematura.


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

### Refinamento interno medido — 2026-10-07

Medição anterior à decisão: 00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/{after-second,refuters-second}.json.gz, pin/binários/HEAD/estado/fontes/UTC/argv/resultados completos.
after-second/context-table e nested; refuters-second/reused-prepared-table mostram preparo com regra ausente. capture_table_insertion em realize/mod.rs ignora ctx.apply_show_rules, ao contrário de eval/rules/intercept.rs:22–25. Hipótese refutável: a passagem original capturadora estava assando o plano sem regras. Captura de valor com apply_show_rules=false não prepara nem captura Table; a inserção efetiva e a fachada causal fazem esse trabalho. A fachada restaura snapshot eval.context.recipe também quando o produto é Table crua sem wrapper Styled.
A aceitação é bilateral, efeitos uma vez e geometria; não normalizar whitespace nem alterar contratos ratificados.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Helper crate-private expõe somente o predicado Table/Cell existente ao eval para captura causal do bloco; nenhum contrato público novo.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Histórico: finalização de ocorrências sem mudança — 2026-10-09 (substituído abaixo)

Medição anterior à decisão: `00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/table-map-passes-before.json.gz` atribui uma reconstrução de Table a `finish_occurrences` em cada fixture de 100/1.600 células sem nenhuma chamada a `locate` nessa passagem; `finish-occurrences-red-gate.json.gz` registra HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado/status/diffstat, UTC, release SHA-256 `7a37e3178feb4bd25b58998f1ff99b10e899c935b7d5a2d767b00f0845120ba0`, script/interpositor/fixtures/comandos e **498.251** pedidos glibc em `local-set-1600`, acima do gate pré-fixado de 490.000. Fonte `realize/mod.rs:44-54` chama `map_content` mesmo quando nenhum nó recebe Location. Em `entities/content/traversal.rs:25-55`, `Source` é transparente, o corpo imediato de `Located` não recebe callback, `Realized` desce apenas em visual e `ShowRecipe` desce em body. `compiler/introspect/extract_payload.rs:18-98` determina os candidatos efetivos; `is_locatable` não substitui esse predicado sem prova por variante (por exemplo, o seu arm de `OutlineEntry` não aparece no extrator). Inferência refutável: um pré-exame emprestado conservador pode eliminar a reconstrução se provar que não existe candidato efetivamente visitado; variantes desconhecidas mantêm a passagem anterior.

Decisão interna de fluxo contínuo (ADR-0127): antes de `finish_occurrences`, provar por referência a ausência de locatáveis ainda sem `Located` nos contêineres conhecidos, com as mesmas fronteiras `Source`/`Located`/`Realized`/`ShowRecipe` da travessia; se houver qualquer candidato ou variante não classificada, executar integralmente `map_content`. `entities/content/source.rs:23-26` mostra que `with_source_span` descarta o wrapper externo se o body já tiver origem, ou o wrapper se o span externo for detached; por isso apenas um `Source` canônico com span próprio não-detached e body sem span pode ser preservado diretamente. As outras formas conservam a travessia anterior. Não mudar localização, ordem, payload, enum, travessia genérica, contrato público ou fase. Aceitação: RED→GREEN de contagem/alocação, teste de árvore sem e com localização necessária, tempo pareado contra release anterior e vanilla pinado, qualificação visual/query e `cargo build && crystalline-lint .`. Retirar se houver divergência ou regressão de tempo robusta.

## Normalização de inserções com reuso — 2026-10-10

Medição anterior à decisão: snapshot-normalization-before.json.gz em diagnosticos/table-local-scaling-followup-2026-10-09 registra as fontes completas e UTC; realize/mod.rs:107–143 achata e reconstrói toda Sequence, mesmo sem sequência interna nem espaço alterado. entities/content/traversal.rs já propaga ausência explícita de substituição. Inferência: sinalizar None em sequências canônicas pode eliminar alocações; contagem e tempo contra o release capturado podem refutá-la.

Decisão interna: após os filhos, devolver None quando uma Sequence com pelo menos dois filhos não exige achatamento nem normalização de espaços. A normalização emprestada pertence ao owner textual/space e compartilha sua política com a variante in-place. Manter canonicalização de sequências vazias/unitárias e achatamento de Sequence e Realized com visual Sequence, incluindo marcador original sem visual. Preservar alterações já feitas nos filhos, proveniência, ordem, espaços inferidos/explicitados e fronteiras. Sem comparação de Content, hashes ou ponteiros para inferir mudança. Contratos públicos e fases permanecem vigentes.

Aceitação: teste RED→GREEN de reuso sem mudança e controles de achatamento/canonicalização; espaços Unicode e carriers; alocações e tempo pareado, preservação de texto/geometria/raster contra o release anterior e controles vanilla pinado; core suite, cargo build e lint. Retirar se houver divergência ou regressão robusta.

## Travessia única após propagação de ausência de mudança — 2026-10-10

### Medição anterior à decisão

`diagnosticos/content-preflight-removal-2026-10-10/{before,measurements,confirmation,allocation-red,allocation-green,validation}.json.gz` registra HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, status/diffstat, fontes, UTC, comandos, fixtures e binários. Release anterior `2d7f5dc93f6a7f15c81ff93b74270c80a14077d6753c8f79c73c9575d113d5f1`; candidato `9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`; oracle vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`/binário `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. O snapshot before preserva as guardas emprestadas; entities/content/traversal.rs:24–82 fornece a prova explícita no mapeamento. Remover as duas guardas reduz tempo pareado de blocks-400 em 1,21% nas duas coletas, seis rodadas com medianas negativas; seis casos mantêm a contagem de alocações. 34 controles de texto/geometria/raster e consultas foram preservados; a suite core permanece verde. Matemática/blocos repetidos continuam fora de comparação de trabalho equivalente com vanilla.

### Decisão interna vigente — substitui os pré-exames acima

Usar diretamente `map_content` na operação desta unidade, sem pré-exame emprestado duplicado. O protocolo de ausência explícita de mudança pertence ao Núcleo content/map-content-change pinado; manter o callback existente e suas fronteiras, ordem, erro, origem, localização e transformação. Não inferir mudança por igualdade, hash ou ponteiro; não ampliar o universo de variantes nem trocar enum por despacho dinâmico. Source continua transparente e conserva a primeira origem conforme o núcleo vigente. Nenhum contrato público, comportamento padrão, fase ou compatibilidade é alterado.

### Aceitação

Conservar controles com/sem coluna, ocorrência já localizada e pendente, origem, efeitos e query; testes core, build e lint. Gate quantitativo externo de tempo pareado passa do baseline sem ganho ao candidato com benefício repetido e sem regressão robusta, sem aumento de alocações. Reuso de Arc é prova auxiliar de engenharia; aceitação de linguagem se dá por texto, geometria, morfologia e efeitos, sem exigir bytes de PDF iguais. A melhoria é pequena e não certifica paridade global nem redução de memória residente.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C020`: `01_core/src/compiler/realize/mod.rs:127`, `realize_document`, 8 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C020` — `realize_document`: Exceção pontual justificada

Contrato público de realização reproduzido literalmente no L0: conteúdo, recursos, source, registry, snapshot anterior e configuração são explícitos. Mesmo registry atualmente sem leitura não pode ser removido da API por este lint.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C020: Contrato público de realização reproduzido literalmente no L0: conteúdo, recursos, source, registry, snapshot anterior e configuração são explícitos")], somente nesta função.

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

O inventário validado em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `00_nucleo/diagnosticos/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C020. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.


## R2-S — realização nominal de Sequence, ratificada em 2026-10-17

### Medição anterior à decisão

As coletas before/expanded/controls.json.gz e candidate.log em
00_nucleo/diagnosticos/sequence-constructor-2026-10-17 fixam HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado,
UTC/diffstat/fontes/binários e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Oito REDs iniciais; cinco passaram no candidato, show e recursão ainda não.
markup_stream.rs:240–260 aplana antes de selecionar a sequência; realize/mod.rs:
170–210 visita filhos antes de receita; apply.rs:140–170 delega não-localizáveis
ao ponto-fixo. O vanilla aceita alias e rejeita nova Sequence com mesmos
filhos. São efeitos, escopo, valores e diagnósticos de linguagem.

O dono aprovou a revisão
00_nucleo/materialization/sequence-realization-2026-10-17/review.md, manifesto
46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327,
nesta sessão. O gate ADR-0127 §2.3 cobre a ordem de realização desta nova
fronteira antes dos filhos/Table/math; não cobre novo contrato público.

### Responsabilidade exclusiva

Compor descendente privado sequences e chamar sua realização pre-order de
recipes marcados antes de realize_table_insertions/bare_math e da travessia
genérica de ShowRecipe. A fronteira Sequence selecionável pode ocultar esses
filhos. O descendente resolve seus produtos/guards; o hub conserva os caminhos
existentes de outras receitas, layouts, índices e targets. Expor somente as
rotas crate-internas necessárias a markup/invoke/apply, sem pub externo.

### Aceitação

S1–S20 ratificados, RED antes de código, GREEN bilateral e regressões; não
reduzir expectativas por limite mecânico. Todos os owners 1:1 e núcleos pinados
válidos antes do resselo. Build/workspace/doctests separados, Clippy/Rustdoc
estritos, fmt/linhagem/reversa e SARIF sem novo erro/warning. 12GiB/High8GiB/
swap0/jobs1 e comandos pesados sequenciais. Sem publicação/release.

### R2-S — ativação do descendente

A passagem existente observe_occurrences também assenta a flag privada
has_deferred_show_rules quando encontra recipe Sequence marcado. O contexto
novo de realização começa sem essa informação; assentar antes da composição
evita pular corpos vazios/locais. Sem marcador/regra Sequence, o descendente
retorna clone raso sem percorrer/reconstruir o documento. Não criar campo
público/flag de produto nem usar estado global para esse atalho.
