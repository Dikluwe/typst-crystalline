# L0 — entities/content/traversal
Hash do Código: 698473be

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/traversal.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own bottom-up content transformation and text transformation through
`map_content` and `map_text`. Traversal remains exhaustive, static,
order-preserving, and pure. Variant-specific recursion delegates to the owning
element. Replacement nodes are not recursively reprocessed unless the
existing contract already requires it.

## Acceptance

Existing traversal tests pass unchanged and the module includes local
tests.


## Realização com avaliação única

map_content recursa no body de ShowRecipe preservando snapshot; Located e Realized são ocorrências terminais para aplicação de receitas, evitando reaplicação aos corpos já realizados. Consumers que expandem contexto e estilos precisam de travessia explícita dos carriers quando pertinente.


## Realização com avaliação única

map_text transforma somente a projeção visual, conservando snapshots de origem e identidade.


## Realização com avaliação única

Travessia visual genérica recursa nos carriers. Located transforma descendentes e visita uma única ocorrência de raiz: o corpo raiz não recebe callback adicional. Realized transforma somente visual, conservando origem imutável. Isso permite expansão contextual/regex/estilos sem duplicar a ocorrência transportada.


## Realização com avaliação única

Located recursa nos descendentes sem callback duplicado de raiz; Realized recursa no produto e nunca transforma a origem consumida pela travessia visual.


## Realização com avaliação única

O callback de raiz desativado existe somente para a raiz do corpo transportado; descendants continuam sujeitos ao callback.

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

Preservar integralmente o comportamento público de map_content e map_text.
Adicionar helper pub(crate) de travessia pré-ordem, com o mesmo callback tipado
FnMut(&Content)->SourceResult<Option<Content>>, sem assinatura pública nova.
A substituição Some é terminal: não visitar seus filhos ou reprocessar o
produto. None delega a recursão exaustiva estática existente e retorna em ordem.
A implementação compartilha a enumeração de filhos do owner, sem segunda tabela
de containers no compiler, vtable, registry ou dispatcher por nome.

Localização preparada continua sendo um carrier único, e origem Realized nunca
é transformada pela travessia visual. Esta unidade não decide o que é Mathy,
blockness, estilos ou aplicação de regras; isso é responsabilidade do owner
compiler/realize/bare_math. Testar ordem, fronteira de substituição, erros e
conservação da travessia pública antes/depois, incluindo Located/Realized.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.
