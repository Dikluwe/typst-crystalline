# L0 — entities/content/traversal
Hash do Código: 4f7b1c74

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

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

### Refinamento interno da travessia pré-ordem

Inspeção em traversal.rs:30–289 e Element::map_content mostra que a delegação
existente é bottom-up até nos filhos do elemento. O helper privado pré-ordem
possui enumeração estática própria neste mesmo owner e clona campos não-content;
não duplica catálogo de containers no compiler nem modifica métodos públicos
dos elementos. Carriers Dynamic de trait legado são fronteira opaca neste helper
privado; map_content público conserva a delegação dinâmica já existente. Esse
limite não certifica math dentro de Dynamic. Elementos internos com prefixo/campos
privados usam remapped/with_body existentes.

## D08B2 — presença lexical ratificada em 2026-10-07

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-realizacao-2026-10-07/before.json.gz e
presence-refuters.json conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree, diffstat, UTC, binários e argumentos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Upstream model/table.rs:734–778 possui campos Smart/Option; o Tekt
entities/elements/table_cell.rs:40–51 conserva apenas Option de valor.
X:auto e x omitido colapsam; fill:none e stroke:none perdem presença.
Os refutadores medem auto permitido em x/y/fill/align/inset/breakable,
rejeitado em colspan/rowspan/stroke; none permitido só em fill/stroke.
São observáveis da linguagem, sem obrigação de copiar a estrutura upstream.

### Contrato ratificado e responsabilidade exclusiva

Reconstruções de TableCell preservam presence; transformações afetam body/texto sem assentar propriedades. Travessia e fase existentes ficam.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Contrato público por campo/tipos novos confirmado pelo humano em 2026-10-07; recibo table-cell-presenca-aprovacao-2026-10-07.json. Não altera fase/defaults de render.

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

Table/Cell preparados possuem vista visual exclusiva: map_content/map_text/pre-order trabalham cells/visual, conservando origens raw e planos/props/stage. Raw mantém descida anterior; durante preparação os hooks da realização delimitam Table/Cell pendentes. Não fazer render/Engine no domínio nem visitar source e visual como dois irmãos emitidos.

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
