# L0 — compiler/eval/observation/elements
Hash do Código: db1012b4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

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


## Aspa semântica e resolução tardia — proposta aguardando ADR-0127

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

### Decisão proposta e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. PARAR
antes de testar/materializar este pacote até o dono confirmar. Candidatos
são cópias completas para revisão, não os L0s vigentes. Após confirmação,
substituir cláusulas conflitantes do contrato histórico apenas para aspas;
ADRs/núcleos existentes e propostas alheias permanecem intactos.

Destructure exaustivamente SmartQuoteElem com double/enabled/alternative/quotes;
observar todos os Option nominais via ObservationEq, distinguindo None de
Some(true/false), mesmo com render igual. Sem callback/render ou campo esquecido.
