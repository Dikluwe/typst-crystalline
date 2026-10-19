# L0 — math/layout/callbacks
Hash do Código: 7a81478d


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/callbacks.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/callbacks.rs:20` locates `pub struct MathCancelRequestId`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Validated in-memory callback transcript

Own MathCancelRequestId (equation Location, structural occurrence, line),
MathCancelRequest (id, Func, default Angle, Span, StyleChain),
MathCancelResolution, SealedMathCallbacks, MathCallbackPassState and the
closed Pending/Complete outcome. This module neither executes Func nor
depends on Engine, World, infrastructure or I/O.

Each concrete layout attempt has a fresh state: optional validated read
store and local RefCell transcript. Never use global/thread-local state,
cross-attempt cache or shared mutable runtime. Reserve cancel occurrence at
dispatcher entry before descending; cross uses the same occurrence and
line=0/1. Span is diagnostic data, not identity.

Resolve a request only when id, function, default and collapsed styles match.
Absent, stale, different or unconsumed resolutions prevent completion;
do not accept partial store success. Pending carries requests only, never a
PagedDocument. Complete is available only after integral transcript validation.
Rejected internal layout attempts discard their transcripts. Only the accepted
candidate participates in later realization.

Snapshot styles by retaining the lexical StyleChain (including custom) and
pushing effective TextStyle fields with representable StyleDelta slots,
without reapplying defaults or inventing properties for internal math flags.
Matching uses collapsed semantic styles, not pointer identity or a textual hash.

Cross emits two requests and therefore two calls, each with the same positive
default angle measured from the body. Reflect only the returned second angle.
Without cross, inverted is applied after resolution; with cross it is ignored.
Angles change line geometry, not MathBox dimensions.

`finish_transcript(self) -> Result<(), Vec<MathCancelRequest>>` performs
integral validation independently of document ownership. Existing
`finish(document)` delegates to it and preserves MathLayoutPassOutcome.
The general coordinator may combine independent families without introducing
par.line execution or dependence into this owner.

Execution, return-type casting, finite convergence and span-anchored function
diagnostics belong to the pipeline's existing realization owner. Those callers
must not export a provisional document. Acceptance covers stable identity,
cross cardinality, style matching, stale/extra/missing store entries, discarded
attempts, repeated/reflowed requests and document-free Pending.


## Tamanho dos enums — decisão medida 2026-10-16

### Medição anterior à decisão

00_nucleo/diagnosticos/clippy-enums-2026-10-16/before.json.gz conserva fontes/L0, hashes globais, HEAD, working tree não commitado, UTC e diffstat; callers.txt registra usos no workspace. probe-details.json e measurements.json guardam rustc 1.92/x86_64, comandos/binário/rlib e tamanhos/alocações de candidatos isolados antes de código. O enum privado é espelhado fielmente da fonte; os tipos públicos são medidos diretamente do rlib. O observador unsafe existe somente no executável diagnóstico, nunca na camada produtiva. É medição de representação Rust, não igualdade de linguagem, intenção upstream ou prova de ganho global.

### Responsabilidade exclusiva e aceitação

Preservar MathLayoutPassOutcome::Pending(Vec<MathCancelRequest>) e Complete(PagedDocument), públicos, diretamente por valor. A busca dos usos produtivos mostra retorno e consumo por match, não coleção de outcomes. A sonda Complete-only com documento vazio e black_box mede inline sem alocação contra uma por Complete na candidata Box: 20.000 extras por amostra. O enum é 440 bytes e a candidata 24; o documento de 440 continua existindo separadamente na caixa. Portanto reduzir esse container não é reduzir a memória total do resultado completo. Manter a fronteira direta e não adicionar Box ao percurso validado. Usar somente #[expect(clippy::large_enum_variant, reason="E003: resultado único de math layout por valor; Box adiciona alocação ao Complete, sem reduzir o documento")] no enum. A sonda não é benchmark de layout real nem caracteriza todos os workloads.

Fluxo contínuo ADR-0127: esta etapa altera somente representação/constructor privados ao crate e metadados nominais; os contratos externos dos outcomes e entidades permanecem. L0 antes do código, resselo restrito. Gate de tamanho privado RED→GREEN e Clippy large_enum_variant deve confirmar três spans selecionados antes e resolver um com código e dois com expectativas satisfeitas. Não elevar limiar ou silenciar módulo/impl/crate; não estender expectativas a Tag/Transformation ou ao request privado. Preservar fixtures/esperados e testar os controles existentes de outline (incluindo oito casos P1405), contexto/replay, callbacks e pipeline. Validar workspace completo/build/Clippy/expectativas/Rustdoc/fmt/linhagem, sem avisos Tekt novos, sob 12 GiB, swap zero e um job. Tag/Transformation continuam medidos, sem alteração pública autorizada neste pacote.


## TS1 — tamanho completo de texto (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`00_nucleo/diagnosticos/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação obrigatória

Este candidato altera dois contratos públicos, descritos em api.md. Não é
L0 vigente até aprovação explícita. Hash do Código antigo é apenas contexto,
não resselo de implementação inexistente. Materializar somente após ratificar
TS1; sem release/merge/push. Testes S1–S16 e gates sob12GiB, swap zero.

### Responsabilidade exclusiva adicional

Adapter de snapshot: TextStyle.size Pt já resolvido entra como Length absoluto em StyleDelta.size. Preservar shape/payload, callback demand e replay sem multiplicar em novamente.
