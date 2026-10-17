# L0 — html/whitespace
Hash do Código: c3d7a33d

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/whitespace.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

## Consumer evidence

`03_infra/src/export/html/whitespace.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Structural space handling

Own inline_html_body, ruby recognition, space_between/top_level_space_between and
SpaceSupport classification. Traverse Styled and inline wrappers without modifying
text/raw payloads. Trim only structural Sequence edges and spaces adjacent to block
or br boundaries; preserve supported inline spaces. Distinguish Boundary,
EmptyInline and Supportive, with replaced tags supporting visible space.

An isolated ASCII space lacking supportive content on both sides uses exactly
`<span style="white-space: pre-wrap">&#x20;</span>`. Top-level lookup reaching a
non-groupable node drops the boundary space rather than making a paragraph/span.
Within ruby, drop a space only when both immediate non-space neighbors are rp/rt;
base text or ordinary inline siblings still preserve spaces. Use tags' static
classification rather than duplicating display tables or implementing CSS.

SpaceFromNewline has the same local projection as Space in these helpers. CJK
normalization and textual show matching belong upstream; do not inspect neighboring
CJK characters to hide an upstream language error. Do not indiscriminately trim
raw/title or change preformatted content.

## Verification

Check edges, br/block adjacency, supportive/replaced versus empty inline nodes,
top-level boundaries, ruby counterexamples and direct Space/SpaceFromNewline pairs.


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

Encaminhar o contexto de aspas compartilhado ao helper inline stateful para
irmãos em um único fluxo; skip/trim de spaces observa só separators efetivos,
sem reset em spans inline. Manter APIs privadas wrappers atuais onde caller
precisa de fluxo isolado. Ruby/br/block continuam seguindo catálogos vigentes,
sem CSS/table de display nova e sem mudar preformatted raw/title.


## Carrier transparency repair — 2026-10-13

### Measurement before decision

`03_infra/src/export/html/whitespace.rs:31` only recognizes a direct Sequence;
`:173` classifies direct HtmlElem neighbors. The pin-verified bilateral fixtures
in `diagnosticos/auditoria-producao-tekt-2026-10-13/html-probes.json.gz` and the
current RED `diagnosticos/correcoes-producao-2026-10-13/html-cli-red.log` expose
spaces retained at br/block/ruby boundaries after Source carriers were introduced.

For this owner's whitespace operation, transparently traverse Source.body,
ShowRecipe.body, Located.body and Realized.visual at sequence entry and when
classifying spaces or neighbors. Traverse Styled with its accumulated chain; do
not drop style when rendering. Preserve the original content for inline emission.
Never inspect Realized.original or execute a captured recipe. Structural trimming
and SmartQuote context must agree for wrapped and direct inputs, including a
wrapped linebreak/ruby annotation and empty inline neighbors. No new public API,
phase, display table or raw/title trimming. This is an internal parity correction
under ADR-0127. Unit tests must fail before repair and cover wrappers, preserved
interior text, br/block/ruby boundaries and the protected-space case.


## Exaustividade nominal e espaços transparentes — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Fontes/L0/configuração, hashes, UTC, status e git diff HEAD --stat em `00_nucleo/diagnosticos/tekt-v16-whitespace-2026-10-17/before.json.gz`. AST/spans em contexts.json; 11 V16 emitidos e um forwarding de identidade latente em classified_content. measurement-before.log mede cinco diferenças entre sete pares Source. measurement-family-before.log e source-family-before-analysis.json medem 28 pares, 20 diferentes: os cinco casos de quebra à esquerda/direita e grupo atravessando Empty falham igualmente sob Source, Located, Realized.visual e Styled. Parbreak e Text continuam fronteiras/apoios iguais. É medição do algoritmo real dentro do crate em workspace descartável, não uma cópia adaptada de helpers privados nem prova global de paridade. A primeira sonda externa falhou por E0624, foi preservada e não conta como RED funcional.

### Prescrição exclusiva

- **W07** — `03_infra/src/export/html/whitespace.rs:31`, `visual_content`: Somente Source/ShowRecipe/Located/Realized são projetados neste loop. As outras variantes são o ponto de parada emprestado. Excluir do complemento somente `Source, ShowRecipe, Located, Realized`; guardados preservados `(nenhum)`.

- **W08** — `03_infra/src/export/html/whitespace.rs:117`, `space_between`: São as três células restantes da matriz 3×3. Boundary absorve a borda; Supportive/Supportive usa espaço normal; qualquer combinação restante tem inline vazio e exige espaço protegido. Excluir do complemento somente `(matriz de SpaceSupport)`; guardados preservados `(nenhum)`.

- **W09** — `03_infra/src/export/html/whitespace.rs:128`, `top_level_space_between`: São as três células restantes da matriz 3×3. Boundary absorve a borda; Supportive/Supportive usa espaço normal; qualquer combinação restante tem inline vazio e exige espaço protegido. Excluir do complemento somente `(matriz de SpaceSupport)`; guardados preservados `(nenhum)`.

- **W10** — `03_infra/src/export/html/whitespace.rs:153`, `paragraph_space_support`: Só os ramos de fronteira incondicionais saem antes da classificação de suporte. HtmlElem permanece no complemento quando o guard não se aplica. Excluir do complemento somente `Parbreak, Heading, Par`; guardados preservados `HtmlElem`.

- **W11** — `03_infra/src/export/html/whitespace.rs:215`, `supports_adjacent_space`: As variantes restantes já eram suporte inline; nomeá-las conserva o resultado true atual e exige revisão ao crescer Content. Excluir do complemento somente `Text, Sequence, Strong, Emph, Styled, HtmlElem, Empty, Space, SpaceFromNewline, Parbreak`; guardados preservados `(nenhum)`.

- **W12** — `03_infra/src/export/html/whitespace.rs:39`, `classified_content`: Depois de visual_content, Styled delega; o complemento retorna exatamente a referência já projetada. Introduzir let content = visual_content(content), sem clone ou captura @. Excluir do complemento somente `Styled`; guardados preservados `(nenhum)`.

### Aceitação

L0 antes de código; correção interna de invariantes existentes (ADR-0127 contínuo), sem campo/assinatura/trait/API, default, fase, dependência, flag ou compatibilidade nova. Complementos excluem somente braços Content incondicionais; conservar variantes guardadas no fallback e manter o guard had_newline. Grupos V19 têm a mesma decisão por caso, permanecem visíveis e não comprovam executar cada alternativa. Não introduzir allow/expect, @ ou um if-let que esconda a decisão. HTML mantém os braços Boundary absorventes existentes e fecha nominalmente as três células do complemento; não muda preformatted/raw/title/ruby, tabelas de tags ou SmartQuote.

Acrescentar regressões nos três módulos proprietários: ambos consumidores de normalização, Source único/aninhado e os quatro carriers em quebras/Empty, controles Parbreak/Text/MathIdent, preservação de span/estilo e Realized.visual separado de original; projeções matching positivas/negativas e matrizes 3×3 inline/top-level com Boundary/EmptyInline/Supportive. Não relaxar testes existentes. RED real antes da implementação, GREEN após. Sondas de variante futura Content/SpaceSupport com cfg(test) compilado: no core usar enum real; no infra desbloquear somente E0004 de dependências/unidades fora dos alvos, em cópia descartável, com braços panic sintéticos registrados e nunca executados. Nunca reparar o local selecionado na própria sonda. Isso distingue dependência que impede compilar o alvo de exaustividade efetiva do alvo. Registrar os 12 matches finais (13 braços), incluindo o helper novo; dois braços de space_with_origin compartilham um match e não contam como dois erros do compilador.

Gates: build e suíte workspace completa, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito, reversa, replay e diferencial SARIF completo. Esperado V16 152→141; V19 somente dos grupos prescritos; V20 sem crescimento. Recursos: MemoryMax=12 GiB, swap zero, um job Cargo e comandos pesados sequenciais. Sem commit/push/merge/release ou alteração de pin CI. O objetivo geral e math-stretch-composite permanecem abertos.
