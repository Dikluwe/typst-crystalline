# L0 — scopes
Hash do Código: 49b45193


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/scopes.rs`

## Consumer evidence

`01_core/src/compiler/scopes.rs:19` locates `pub struct Scopes<'a>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Scope stack and capture

Preserve `Scopes<'a>` with public `top: Scope`, `scopes: Vec<Scope>`,
`captured: Option<Arc<Scope>>`, `captured_by: Option<Capturer>` and
`base: Option<&'a Library>`. Capture reason is present with the captured
scope and affects mutation diagnostics, not reading. Arc is local RAM ownership,
not I/O or mutable global state.

`new(base)` creates an empty top and no capture.
`with_parent(parent, capturer) -> Scopes<'static>` shares the captured scope,
starts with empty top/stack and has no base reference.
`get` searches top, previous scopes newest first, captured, then base.global.
`get_local` uses the same order but stops before base; `has_global` reports
membership in base.global for caller diagnostics, not mathematical resolution.

`snapshot` includes base.global, then captured, previous scopes oldest first
and top; nearer bindings overwrite earlier values. This preserves standard
library access when a capture is later called without a base reference.
`iter_all` traverses captured, previous scopes oldest first, then top; it
does not include base, and is not a deduplicated replacement for snapshot.

Enter pushes the old top and starts an empty scope. Exit returns the departed
scope and restores the previous one (or an empty scope if none remains).
Push_scope installs a populated top while retaining the old one.
Define writes only top. `get_mut` searches only top and previous scopes;
captured and base are read-only through this interface. A local shadow remains
mutable. Preserve these methods and their signatures.

`captured_by(name)` reports the definition-time capturer only when a nearer
mutable binding does not shadow the capture. `is_constant(name)` is true
only for names reachable solely through base. These classifications allow the
assignment owner `00_nucleo/prompts/core/compiler/eval/bindings/access.md` to
distinguish captured-variable, constant and unknown-variable diagnostics,
in that priority order. This owner does not duplicate assignment execution
or error construction. Missing names are not constants.

Acceptance covers lexical shadowing, enter/exit, captured sharing, base access,
snapshot precedence, local-only lookup and mutation classifications without
requiring the upstream Binding representation.


## Snapshot com cópia inicial do primeiro prefixo — prioridade 7

### Medição anterior à decisão

`lab/results/construction/diagnostics/snapshot-scopes-prioridade7-2026-10-08/before.json.gz`
guarda HEAD/árvore/UTC/textos e release 17e72149. scopes.rs:84–103 reinsere por
nome todos os bindings do prefixo e dos overlays. closures.rs:196 e
expression_dispatch.rs:505 capturam na definição, não em cada chamada.
`probes.json.gz`/`probe-summary.json` medem 100 capturas em closures100: 20.300
bindings de entrada, dos quais 20.100 da base, sempre de tamanho 201. Uma closure
chamada 1.000 vezes captura só uma vez. context100 captura 100 vezes; nested
mostra base e captured em chamadas diferentes. Contagem debug e chamadas release
coincidem nos controles comuns. Tempos sob GDB são intrusivos e não são prova de
fração de tempo na CLI normal.
`prototype.json` usa Scope/Scopes reais e dados inteiros sintéticos; o helper
externo inicia pelo clone completo da base (ou captured sem base). Cinco pares
por configuração, 2.000 snapshots por amostra: base201 34,09→18,94 ms,
captured201 37,47→20,79 ms, ambos201 37,50→20,93 ms. A ordem/valores foram
comparados antes do cronômetro. Não medir esses inteiros como stdlib completa.
A cópia/reinserção é mecânica; os valores, precedência, ordem e isolamento são
semântica/morfologia do contrato existente (ADR-0107/0108).

### Decisão e responsabilidade exclusiva

Começar snapshot pelo clone de base.global, se base existe; caso contrário,
pelo clone de captured, se existe; senão Scope::new. Overlay captured ocorre
somente quando ainda não foi usado como prefixo. Depois aplicar previous scopes
em ordem antiga→nova e top como antes. Isso conserva primeira posição dos nomes,
substituição pelos mais próximos, stdlib disponível na chamada e isolamento da
captura. Não filtrar nomes usados/livres; não compartilhar mudanças, omitir
stdlib ou transformar captura em lookup do estado posterior à definição.

Não mudar representação de Scope, sua new/define/get/get_mut/clone, Capturer,
campos/assinaturas públicos de Scopes, fases, defaults ou diagnósticos. Loops sem
captura não ganham Arc/alocação extra. Um helper privado pode receber FnOnce
estático para clonar o prefixo, executado no máximo uma vez; é construção pura
de RAM, nunca callback da linguagem. Sem I/O, unsafe, globais ou novo crate.
Correção interna reversível, fluxo contínuo ADR-0127; owner continua 1:1.

### Aceitação e refutação

RED→GREEN: testemunho privado conta uma cópia inicial do prefixo, em vez de sua
reinserção individual; cobre base+captured e captured sem base. Controle funcional
conserva overlays, primeira posição, mutação isolada e fallback sem prefixo.
O testemunho é de mecânica interna, não igualdade de ponteiros da linguagem.
Comparar release congelado e atual em CLI: criação repetida, chamada repetida,
contextos, loops sem captura e caso mínimo; preservar texto/caixas/diagnósticos,
com vanilla pinado qualificado antes de declarar paridade. Só manter com ganho
repetido nos workloads de captura e sem regressão relevante nos controles.
Builds dev/release, suíte core/infra pertinente, lint e resselo só deste owner.
