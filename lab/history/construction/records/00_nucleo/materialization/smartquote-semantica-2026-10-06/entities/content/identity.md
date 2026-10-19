# L0 — entities/content/identity
Hash do Código: 945a719e

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/identity.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

## Contract

Provide the stable diagnostic name and bounded structural debug representation
of every closed `Content` variant through `Content::elem_name` and
`Debug for Content`.

- The implementation is a nominal, exhaustive, statically dispatched match
  over `Content`.
- It must not use a wildcard arm, dynamic dispatch, a registry, or a property
  map.
- Returned diagnostic names must remain stable.
- `Space` and `SpaceFromNewline` both return `"space"`, because their origin
  distinction is causal and not part of the public element identity.
- The method remains public with the signature
  `pub fn elem_name(&self) -> &'static str`.
- Adding a `Content` variant must force this owner to classify its diagnostic
  name at compile time.
- Debug formatting remains depth-bounded and delegates to an exhaustive static
  match. Existing output strings and payload formatting must not change.

## Acceptance

- Existing callers compile without changes.
- Existing identity and field-access tests remain green.
- `cargo test -p typst-core --lib` passes.
- `crystalline-lint .` reports no ownership, lineage, or prompt-drift
  violation after hashes are resealed.


## Realização com avaliação única

Nome de elemento dos carriers internos é transparente. Debug conserva o corpo lexical/visual sem transformar carrier em elemento da linguagem.


## Realização com avaliação única

Debug de carriers internos projeta corpo/visual; identidade de ocorrência não é inferida de Debug.


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

Corrigir apenas o nome diagnóstico/public elem_name de SmartQuote para
smartquote, como a medição .double exige. Assinatura, dispatch exaustivo e
nomes restantes ficam. Debug Rust pode conservar forma interna histórica;
não usá-lo como JSON/critério de paridade. Não renomear variante Content.
