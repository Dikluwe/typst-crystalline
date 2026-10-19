# L0 — math/layout/underline
Hash do Código: 5955f121


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/underline.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/underline.rs:18` locates `pub(super) fn layout<M: FontMetrics>(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Mathematical underline and overline

Own free functions for the distinct MathUnderline/MathOverline identities.
The core only delegates; textual decorations have separate owners.
Use font-derived MATH constants, current fill and baseline-relative geometry,
without changing the public body's morphology.

Underline lays out the body without forcing cramped. Line thickness, gap and
extra descender use underbar_* constants. The line center is
`body.descent+gap+thickness/2`.
Preserve logical body width; line width alone is
`max(0,body.width-italics_correction(body))`. Positive correction shortens,
zero preserves and negative correction extends outside the box without
enlarging it. Do not clamp protruding ink to a root frame or use ink width as
the logical width. Descender includes gap, thickness and extra descender.

Overline lays out the body cramped and uses overbar_* constants. Its rule
center is `-(body.ascent+gap+thickness/2)`; width equals body logical width,
without underline's italics subtraction. Include the extra ascender.
Keep body baseline coordinates consistent; no additional fixture-derived
translation or textual decoration is required.

Both retain body's mathematical boundaries/properties; the line does not
manufacture class, text-like state or italics correction.
Acceptance covers above/below placement, width versus rule width, positive/
zero/negative correction, cramped behavior and full vertical extents.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F050: 01_core/src/compiler/math/layout/underline.rs:57 (layout). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F050 — `layout`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
