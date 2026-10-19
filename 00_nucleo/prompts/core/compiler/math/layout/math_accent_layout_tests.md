# L0 — core/compiler/math/layout/math_accent_layout_tests
Hash do Código: fbd7c308


**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/math_accent_layout_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/math_accent_layout_tests.rs:29` locates `struct NullWorld`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Independent accent tests

Own only `01_core/src/compiler/math/layout/math_accent_layout_tests.rs`.
Observe attributable FrameItems, not serialized bytes.
Upper accents stay above cramped bases and preserve attachment/baseline
regressions; Unicode Below and the four explicit lower support symbols stay
below non-cramped bases. Only the upper path performs dotless realization.

Effective Rel<Length> size resolves against base width with shortfall 0.5em.
Synthetic identifiable variants distinguish 100% from 150%. Size and dotless
independently obey explicit > cascade > default, without manufacturing carrier
presence. Both transformations preserve every causal field. Dotless morphology
covers i and j with both polarities.

Preserve the exact-threshold witness: base=1200du, first variant=750du.
Shortfall 0.5em gives target=700du and selects that variant; 0.4em gives
800du and selects the larger variant; 0.6em gives 600du and retains the base.
This discriminates the actual threshold, not merely percentage differences.

Unattributable font, variant, glyph, frame, cascade or geometry is Unknown.
Unknown fails closed in mandatory cases and never kills a mutant.

## Leitura de identidade por ocorrência

Após o carrier visual de acentos ganhar `TextReplacement`, as observações
de posição/tamanho continuam a medir os filhos `Text`/`TextShaped` e `Glyph`
originais. O helper de teste percorre o envelope de forma transparente, sem
substituir a assertiva de geometria pela string `ActualText` nem alterar o
caso sem envelope. RED integrado: os testes de orientação/dotless e de
precedência de size deixaram de encontrar suas folhas diretas.

A base `i`/`j` sem dotless dentro de matemática é `𝑖`/`𝑗` após a
aplicação vigente de itálico por defeito; o teste compara essa morfologia
realizada, não o token ASCII de entrada. A forma dotless continua `ı`/`ȷ`.
O acento inferior também conserva o itálico da base; sua orientação é
medida pelas posições dos filhos visuais.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F041: 01_core/src/compiler/math/layout/math_accent_layout_tests.rs:94 (visit). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F041 — `visit`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
