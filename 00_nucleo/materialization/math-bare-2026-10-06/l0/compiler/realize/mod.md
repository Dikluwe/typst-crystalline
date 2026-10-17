# L0 — compiler/realize/mod
Hash do Código: 01a49249

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff

**Camada:** L1.
**Consumer produtivo exclusivo:** `01_core/src/compiler/realize/mod.rs`.
**Governança:** ADR-0026, ADR-0029, ADR-0030, ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 e ADR-0130.

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
