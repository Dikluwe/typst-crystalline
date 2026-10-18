# L0 — compiler/eval/rules/textual/matching
Hash do Código: f6825ad5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/eval/textual-consumed-slots.toml sha256:faaf206c9f4c20eba8b18b463ebe7836cb9671fd8672501c068eef2751f88d3c
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:f225f3039d4663e9bfcb408d26cf1cd6e557f089013bc2d95090ce16c52080c6


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual/matching.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the working tree captured in `00_nucleo/diagnosticos/atomizacao-textual-algoritmos-2026-09-30-before.json.gz`, `01_core/src/compiler/eval/rules/textual.rs:96-333` contains realized group traversal, rule precedence, matching, transformation and projection of unmatched original ranges. The `p1494` filter passed 17 tests on that state. This is a structural baseline, not a timing or new vanilla measurement.

## Decision and behavior

Own the complete realized textual group algorithm, from grouping by causal snapshot and style boundary through match selection and replacement, delegating preservation of unmatched
content to `00_nucleo/prompts/compiler/eval/rules/textual/matching/projection.md`. Search literal/regex over the largest eligible realized group of the same lexical snapshot and style chain, including separated Text/space and dynamic text nodes. Each recipe receives only its matched slice; preserve leftmost order, unmatched prefix/suffix, precedence, guard/revocation and depth limit. A replacement is not fed back into the same revoked rule. Empty regex matches remain rejected and literal '.' stays distinct from regex '.'.

While a ContentBlock is only a value, preserve raw Space/SpaceFromNewline. Its local textual show operates provisionally before normalization: replace only the matched slice, preserving unmatched separators for later insertion. Root realization normalizes the inserted value and applies its eligible snapshot. The combined local CJK show/normalization case and sequential-identical-rule override remain explicit unresolved parity obligations, not presumed successes.

The coordinator passes its existing inputs after optional normalization and early return when show is disabled. This module may access the parent snapshot and call the existing selector matcher/call dispatcher, but must not duplicate space normalization, alter the engine restoration order or introduce dynamic dispatch. The descendant projection owner preserves unmatched nodes and flattened
replacement sequences; its unmatched-slice test lives with that algorithm. No phase, public signature or default changes.

## Projection refinement evidence

At the working tree captured in
`00_nucleo/diagnosticos/atomizacao-textual-projecao-2026-09-30-before.json.gz`,
this consumer contains `project_original_range` and `append_flat_sequence` at
lines 189-223, followed by the matching transform. The slice projection test
passed in the `p1494` filter after the first movement. Their only input is
content and range/sequence structure, so projection can be a pure descendant
without access to engine state.

## Acceptance

The existing literal/regex slicing, lexical snapshot and guard tests and full core suite remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. No speedup is claimed without measurement. Header ownership is 1:1.


## Realização com avaliação única

Grupos textuais podem projetar Realized/Located e sequências exclusivamente textuais produzidas por elementos. Elementos estruturais continuam fronteiras. O snapshot causal do grupo conserva origens consumidas por splice.


## Realização com avaliação única

Um produto textual de elemento participa do match de grupo; espaços sozinhos continuam sem receita.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Agrupamento textual Source delega somente body para regex/texto; preservar origem na reconstituição de produto e não criar fronteira extra.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## Exaustividade nominal e espaços transparentes — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Fontes/L0/configuração, hashes, UTC, status e git diff HEAD --stat em `00_nucleo/diagnosticos/tekt-v16-whitespace-2026-10-17/before.json.gz`. AST/spans em contexts.json; 11 V16 emitidos e um forwarding de identidade latente em classified_content. measurement-before.log mede cinco diferenças entre sete pares Source. measurement-family-before.log e source-family-before-analysis.json medem 28 pares, 20 diferentes: os cinco casos de quebra à esquerda/direita e grupo atravessando Empty falham igualmente sob Source, Located, Realized.visual e Styled. Parbreak e Text continuam fronteiras/apoios iguais. É medição do algoritmo real dentro do crate em workspace descartável, não uma cópia adaptada de helpers privados nem prova global de paridade. A primeira sonda externa falhou por E0624, foi preservada e não conta como RED funcional.

### Prescrição exclusiva

- **W01** — `01_core/src/compiler/eval/rules/textual/matching.rs:243`, `textual_piece`: Só texto/space/empty e os carriers já projetados entram no grupo. Os demais continuam fronteiras; não atravessar Styled/ShowRecipe ou alterar snapshots/receitas. Excluir do complemento somente `Realized, Source, Located, Sequence, Empty, Text, Space, SpaceFromNewline`; guardados preservados `(nenhum)`.

### Aceitação

L0 antes de código; correção interna de invariantes existentes (ADR-0127 contínuo), sem campo/assinatura/trait/API, default, fase, dependência, flag ou compatibilidade nova. Complementos excluem somente braços Content incondicionais; conservar variantes guardadas no fallback e manter o guard had_newline. Grupos V19 têm a mesma decisão por caso, permanecem visíveis e não comprovam executar cada alternativa. Não introduzir allow/expect, @ ou um if-let que esconda a decisão. HTML mantém os braços Boundary absorventes existentes e fecha nominalmente as três células do complemento; não muda preformatted/raw/title/ruby, tabelas de tags ou SmartQuote.

Acrescentar regressões nos três módulos proprietários: ambos consumidores de normalização, Source único/aninhado e os quatro carriers em quebras/Empty, controles Parbreak/Text/MathIdent, preservação de span/estilo e Realized.visual separado de original; projeções matching positivas/negativas e matrizes 3×3 inline/top-level com Boundary/EmptyInline/Supportive. Não relaxar testes existentes. RED real antes da implementação, GREEN após. Sondas de variante futura Content/SpaceSupport com cfg(test) compilado: no core usar enum real; no infra desbloquear somente E0004 de dependências/unidades fora dos alvos, em cópia descartável, com braços panic sintéticos registrados e nunca executados. Nunca reparar o local selecionado na própria sonda. Isso distingue dependência que impede compilar o alvo de exaustividade efetiva do alvo. Registrar os 12 matches finais (13 braços), incluindo o helper novo; dois braços de space_with_origin compartilham um match e não contam como dois erros do compilador.

Gates: build e suíte workspace completa, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito, reversa, replay e diferencial SARIF completo. Esperado V16 152→141; V19 somente dos grupos prescritos; V20 sem crescimento. Recursos: MemoryMax=12 GiB, swap zero, um job Cargo e comandos pesados sequenciais. Sem commit/push/merge/release ou alteração de pin CI. O objetivo geral e math-stretch-composite permanecem abertos.


## R3 — slots consumidos e apresentação de None (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/sequence-r3-2026-10-18/{before,markup-after,
textual-before,transport-before,transport-before-sources}.json.gz conserva
HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado,
UTC/diffstat/fontes, executáveis e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Sob show literal Z que não casa, nested.children expõe um pacote Sequence
mais dois Empty no Tekt; no pin são os três filhos originais. empty-child
expõe pacote mais Empty no Tekt, mas []/A no pin. markup_stream.rs:201–249
reconstitui o vetor incluindo slots que textual.rs:103–129 zerou; matching.rs:
90–95 também zera os siblings consumidos. São detalhes de transporte vazando
para a morfologia pública. CodeBlock/None interpolado desaparece; [#none#[]A]
tem dois filhos no Tekt e três no pin, e X#none;B sob show Empty resulta XB
contra XEB. A tabela value_to_display_content omite None por contrato histórico.
No pin typst-library/src/foundations/value.rs:191–210 apresenta None como
Content::empty; typst-eval/src/markup.rs:39–56 trata SetRule/ShowRule como
fronteiras próprias e :78 apensa o valor das demais expressões. Não copiar
buffers Rust nem inferir intenção a partir da implementação.

### Decisão e limites

Correção interna de paridade ADR-0127, sem contrato Rust externo, novo campo,
modo/default ou fase. Máscara privada Vec<bool>, sem Location falsa ou carrier
público novo. A hipótese é refutada por perda de Empty/Text vazio genuíno,
deslocamento de fronteira/estilo, repetição de callback ou índices inconsistentes.
A apresentação de None muda apenas na inserção do markup, não no helper genérico
usado por Math/CodeSetTail. SetRule/ShowRule continuam suas fronteiras próprias.

RED funcional antes de código: children/fields sob receita literal não casada,
filhos []/Text vazio, valor None de literal/let/assert/condicional, aplicação do
show Empty a None e set vazio/estilo sem Empty artificial. GREEN e corpus
bilateral, controles R2-S/EOF/slots/estilos/CJK/loops, build/suítes completas,
Clippy/Rustdoc estritos, formato/linhagem/reversa/diferencial SARIF. 12GiB,
High8GiB, swap zero, jobs1 e jobs pesados sequenciais; sem publicação.

A preservação de fronteiras do valor de markup não fecha a morfologia pública
de Styled/Styles, repr de labels ou a formação do valor de módulo/include.
R3-Reflection e o lote A3/include completarão esses eixos com medição própria.
Precedência/guardas textuais e show-set literal continuam no lote R3-Textual;
nenhuma mudança dessa política é legitimada por este transporte.

Responsabilidade exclusiva: apply_textual_groups retorna máscara Vec<bool>
privada, inicialmente false. Quando replacement realmente existe, marcar os
slots start+1..end zerados como consumidos. Ausência de match não consome slot.
A precedência/transform/splice atuais permanecem nesta revisão; o lote seguinte
medirá e substituirá essa política, sem atribuir seu reparo a esta máscara.


### Limite adicional medido — grupos textuais em múltiplas caudas

transport-after.json.gz no diagnóstico R3 confirma a remoção dos Empty
consumidos e a presença dos None reais. style-tail ainda expõe um grupo
sintético Sequence(A,Space) como um filho, antes do Styled; rule-tail também
conserva forma de cauda divergente. O problema é adicional à máscara de consumo,
não somente uma diferença de repr. O pacote R3-TextualGroups completará a
projeção linguística dos grupos sintéticos em múltiplos escopos e sua igualdade/
seleção, preservando a Sequence escrita pelo autor. R3-Reflection completará
Styles e Styled públicos. Não considerar esses controles fechados nem achatar
Sequence genericamente para escondê-los. Este adendo registra o recorte ainda
incompleto; não autoriza modificar os owners dessas projeções sem L0 próprio.

## R3-T1 — realização causal ratificada (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/textual-realization-2026-10-18/{before,scope,refuters}.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não
commitado, UTC, status/diffstat, fontes/argv/hashes dos binários e pin vanilla
bd5ebec285505889e8ff1d4f68804ab147a699f3. before-sources.json.gz contém
fontes/L0 vigentes completos. Não há código candidato implementado nesta medição.
matching.rs:118–150 aplica regex reversa e depois literais em declaração;
:194–207 volta a expor produtos a passes subsequentes. textual.rs:102–123
captura snapshot completo. realize/mod.rs:177–217 visita receitas bottom-up.
apply.rs:620–648 e matching.rs:218–225 chamam funções sem capturar os filhos.
expression_dispatch.rs:125–129 já distingue insertion_rules do estado local.

Exemplos medidos: literais idênticos C/B; empate literal/regex R/L; overlap
X/YB; primeiro efeito LATER/FIRST; função encadeada inversa BBBB/BB;
produto separado AB/X; produto+vizinho X/AB; Strong ocultando filho dá
panic HIDDEN/Q; global capturado seguido de global nova dá L/R, mas local
capturado dá L/L. Entrada CJK local 水\n果 dá 水果/X. A fonte pinada
foundations/selector.rs:110–125 unifica literal escapado com regex;
typst-realize/src/lib.rs:1328–1405 escolhe o início e carrega revogação
no produto. Esses dados medem linguagem/efeitos; não ratificam mecânica Rust,
igualdade de buffers ou bytes PDF. É inferência causal que a separação de
escopos e a visita anterior aos filhos expliquem o conjunto; os controles de
acceptance.json, igualdade/fields/estilo/query e erros podem refutá-la.

### Decisão ratificada

Aplicar o núcleo textual-recipe-frontier pinado neste L0. A confirmação humana
do manifesto 47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302
ratificou esta ordem. Ele supersede neste escopo as
cláusulas históricas de execução provisória de show em ContentBlock e da
visita bottom-up de receitas pendentes, sem alterar a ordem de avaliação
de expressões comuns. Não reavaliar o programa, antecipar State/Counter,
assentar render em morfologia nem criar contrato Rust público.

### Responsabilidade exclusiva desta unidade

Substituir passes regex/literal pela seleção única de menor início, desempate léxico e revogação por produto. Prefixo e sufixo do input permanecem no ambiente original. Capturar conteúdo produzido pela função antes da realização de seus filhos; revisitar somente o produto por meio do coordenador privado. Produtos separados não viram um grupo novo só por compartilharem guardas. Constante/alias/corpo composto são processados uma vez pelas regras restantes. Text whitespace elegível e show-set com Styles reais seguem o núcleo; normalização continua no owner space. Não alterar casts de símbolos ou contratos públicos.

### Aceitação desta revisão

RED→GREEN das famílias de acceptance.json, corpus R2-S/R3 e controles de uma avaliação. Revalidar PDF e HTML separadamente; build/workspace/doctests, Clippy/Rustdoc estritos, fmt, V1/V5/V15/V26 e linhagem reversa; diferencial SARIF sem novo erro/warning. Teto 12 GiB/High8 GiB/swap0/jobs1 e comandos pesados sequenciais. Não alegar ganho de RAM antes de ensaio pareado nem zerar o gate com supressão. Sem commit/push/release.

### R3-T1 — limites fracos e unidade do produto

O candidato1, congelado em r3-t1-materializacao-2026-10-18/candidate1-sources.json.gz,
passou 51/56 sondas textuais. T30 emitiu AB em vez de X: os packets de [A]+[B]
precisam ser coalescidos no ambiente efetivo do produto antes de o coordenador
visitá-los. Não coalescer produtos já concluídos nem packets com efeitos de raiz.
T35/T51/T52 transformaram a newline inicial depois de show em S; T53 deixou o
espaço emitido pelo callback sem aplicar a regra restante. A seleção sobre o
fluxo comum exclui as bordas fracas Space/SpaceFromNewline do texto pesquisado,
conservando os carriers originais na projeção. Text whitespace continua texto.
Dentro de um produto protegido, um fluxo formado só por espaço é elegível:
trata-se do conteúdo emitido pela transformação, não de uma borda da fonte.
Calcular limites UTF-8 por nós/intervalos e não por estado por byte. Prefixo e
sufixo excluídos continuam projetados, sem mudar morfologia ou normalização.

### R3-T1 — apoio contextual de espaços emitidos

Candidato2 passou 63/64 testes, mas T36 transformou o espaço isolado emitido por
A em S onde o oracle não emite texto. T53 exige S quando esse mesmo produto
fica entre X e X. Guardas não determinam apoio textual. EvalContext transporta
privadamente um par de booleanos de apoio esquerdo/direito durante a realização
de um produto. O matcher deriva o par dos intervalos prefixo/sufixo do match e
do apoio externo vigente, restaura-o em sucesso e erro e só inclui bordas fracas
apoiadas. Um produto formado apenas por espaços é elegível quando tem apoio dos
dois lados. Text whitespace explícito continua elegível independentemente disso.
Nenhum dado público, estado global, passo de layout ou captura permanente do
apoio em valores é criado: os vizinhos dependem da inserção atual.

### R3-T1 — integração revelada pela suíte ampla

Em significant_bounds manter o complemento Content nominal, incluindo Text e
Realized guardados e excluindo só Sequence/Located incondicionais. Todos os
outros tipos são fronteiras sem intervalo de texto, conforme textual_piece.
O agrupamento V19 é intencional e não prova cobertura por alternativa.
