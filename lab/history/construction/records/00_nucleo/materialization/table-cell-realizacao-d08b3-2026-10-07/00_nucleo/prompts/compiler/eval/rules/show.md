# L0 — compiler/eval/rules/show — Avaliação de show rules
Hash do Código: 8d82760c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:71d4b43f260cff61fffa8ba7831733e7a62f2517109911c8af346458d64cbd5b
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/show.rs`

## Medição antes da decisão

A working tree não commitada capturada em `00_nucleo/diagnosticos/atomizacao-eval-rules-2026-09-30-before.json.gz` contém as rotinas desta família em `01_core/src/compiler/eval/rules.rs`. A suíte L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem prova bilateral nova contra o vanilla. As medidas históricas e ressalvas do pin continuam em `00_nucleo/prompts/compiler/eval/rules.md`.

## Contrato de linguagem

Capturar estilos de set e avaliar ShowRule com identidade, seleção, transformação e escopo léxico; preservar guards, erros, callbacks, origem e morfologia.

As assinaturas, identidades, casts, defaults, precedência de erros e fase de avaliação observados permanecem. O parent mantém diagnóstico compartilhado e bindings estáticos; outros owners de selector matching, terminação e textual realization continuam exclusivos. Não duplicar esses algoritmos.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade em semântica, sintaxe e morfologia (ADR-0107/0108), ownership exclusivo e pins válidos (ADR-0129/0130).

## Reparos independentes do gate de aspas — 2026-10-06

Medição anterior à decisão: smartquote-2026-10-06/{before,fields-before}.json.gz,
com pin/HEAD/working tree/UTC e binários exatos registrados.

fields-before mede compile-show-native: vanilla Q, Tekt rejeita função smartquote.
Adicionar mapping estático por fn_addr_eq(native_smartquote) para o Selector::NativeElement
já existente. Não criar NodeKind, não despachar pelo nome (alias funciona) e
não executar constructor para identificar função. A folha programática já
existe e content_elem_func reconhece sua função. Markup continua fora desta
fatia até o gate de fase; alterar glyph/eval/layout aqui é proibido.

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

Adicionar ao mapping já existente fn_addr_eq(native_math_equation) para
Selector::NativeElement da função efetiva. Direto, alias e .where seguem o
mesmo owner de matching. Sem NodeKind novo, despacho por string ou constructor
executado para descobrir identidade. Função homônima e partial/closure não
viram elemento por seu nome. Esta entrada é correção de mapping em fluxo
contínuo; neste pacote será promovida com o gate maior, não é mudança de fase.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.

## D08B1 — projeção existente, fluxo contínuo ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-2026-10-07/before.json.gz
registra UTC, diff HEAD --stat, hashes de fontes/binários, argv/cwd e resultados
contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8.
Vanilla model/table.rs:734–778 declara body e propriedades; Tekt
entities/elements/table_cell.rs:62–109 não implementa getter; content/identity.rs:107
usa table.cell; content_methods.rs:226–254 não projeta a função real;
rules/show.rs:300–320 omite a nativa. Body/fields/func-eq e show explícito
refutam paridade. É semântica/morfologia, não igualdade Rust ou bytes PDF.
Não se infere intenção da estrutura upstream.

### Responsabilidade exclusiva

Adicionar somente mapping estático fn_addr_eq(native_table_cell) para NativeElement já existente. Não executar constructor nem classificar pelo nome.

### Limites e aceitação

RED→GREEN e coleta bilateral; cargo build e crystalline-lint sem erros.
Esta correção não muda contrato público de tipos/assinaturas nem fase.
D08B permanece incompleto: D08B2 completa presença omitido/auto/none por
contrato próprio; D08B3 completa realização de posições/estilos/padding/show.
Option None não prova omissão nem none explícito: não certificar esses casos.

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

Medição order/table-replacement-before-cells: vanilla aceita show table e omite efeitos de células; Tekt rejeita em rules/show.rs:322–331. Adicionar mapping por fn_addr_eq(native_table) para Selector::NativeElement existente, conservando aliases e where explícito. Não criar NodeKind nem identificar pelo nome; regras e matching continuam com seus owners.

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
