# L0 — math/layout/tests
Hash do Código: c3b4f99a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8


**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/tests.rs:16` locates `fn default_style() -> TextStyle`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Test-only ownership

Own only `01_core/src/compiler/math/layout/tests.rs` and its cfg(test)
integration. Test obligations do not authorize productive geometry.
Exercise dispatch, styles, baseline/extents, spacing and feature regressions
with injected metrics and attributable oracles. Record formula/observable
provenance for new cases; fixture coordinates and bytes are not contracts.

Root fixtures preserve index_explicit: square-root controls use None/false,
explicit indices use true, without changing independent geometry oracles.

Fraction coverage distinguishes vertical/skewed/horizontal, explicit versus
omitted style, style-chain precedence, slash associativity, both
deparenthesization bits and the barless binomial. Observe axis, operand order,
separation and bar/slash presence rather than freezing font names, private
frames or operation order. Independent feature suites keep their own owners
under `00_nucleo/prompts/compiler/math/layout/`.

Acceptance observes morphology and attributable font-derived geometry;
historical RED/GREEN reports or aggregate test totals are not fresh parity
evidence.

For `lr` around a fraction, compare the same fenced body with and without
the explicit wrapper using injected OpenType-like vertical variants. The
fraction's unbalanced raw height and axis-balanced target must straddle a
variant threshold, so a test fails if explicit `lr` uses raw height while
implicit fences use the balanced extent. Observe selected glyph advance and
declared box extents for both parentheses and bars; the pinned vanilla
source's `FencedItem::relative_to_from_fragments` supplies the independent
axis-balancing rule, and bilateral `sec_22.typ` checks its product effect.

For Display Large operators, verify that a multi-part assembly carries one
source-character TextReplacement around its visual parts; the wrapper does
not alter part positions, width or extents. Preserve base italics-correction
behavior when scripts consume the wrapped visual glyphs. Verify that a
sufficient base glyph remains
unchanged before consulting MATH constructions. Exercise the case where the font's largest
vertical variant remains below DisplayOperatorMinHeight and has no assembly:
retain that largest variant, including its native horizontal advance and
ink-derived extents, rather than reverting to the base glyph. Also cover the
assembly path when the font supplies one. Attribute expected selection to the
active font's OpenType MATH construction and the pinned vanilla algorithm.

## Verificação de identidade extraível

Para `cases`, acentos e spreaders, verificar no L1 que `TextReplacement`
retém o caráter da linguagem e preserva as peças visuais; sondar extração
PDF nos exemplos locais após RED→GREEN. Diferenças de espaçamento
introduzidas apenas pelo extrator, como `.typ/sec_27.typ`, não autorizam
inserir whitespace na árvore sem causa linguística.

Testar também dois acentos combinantes aninhados: a identidade deve ser
base+marca interna+marca externa com somente um envelope no resultado final.

Em `hat(mu)`, verificar que a folha grega `MathText` entra no acento como
`𝜇` matemático, enquanto texto de vários caracteres não vira letras
matemáticas por aproximação. A sonda PDF deve preservar a composição
base+marca e medir o raster contra o vanilla.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Saneamento do CI — 2026-10-13

Estado medido: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree
não commitado. Fontes, hashes, UTC, status e diffstat estão em
`lab/results/construction/diagnostics/testes-ci-2026-10-13/before.json.gz`. O inventário
Clippy de entrada é `matematica-ci-2026-10-13/clippy-inventory.json`, conservado
no evidence.json.gz daquela rodada. Medir antes de decidir (ADR-0108).

Medição: tests.rs:8238 usa hypot no oráculo de geometria da fixture FixedMetrics
com tamanho 12; :9417/:9473/:9506/:9546 chamam drop em MathLayouter sem Drop.
Há buscas iter().any de igualdade, literais hex de agrupamento desigual,
nomes cal_L e um iterador duplamente terminado consumido até last.

Decisão: manter todas as entradas e tolerâncias. Na fixture finita e pequena,
calcular a diagonal pelo teorema de Pitágoras com sqrt de width² + height²;
não usar esse rearranjo como autorização para geometria produtiva arbitrária.
Remover drops explícitos de borrowers sem destrutor: o compilador valida o
fim dos empréstimos antes de consumir pass. Usar contains, next_back e
starts_with nas equivalências diagnosticadas; normalizar grupos hex sem
mudar valores; renomear cal_L para cal_l. Não remover execução de layout nem
alterar ordem/quantidade de callbacks. Conservar todos os testes e observáveis.

Reparo interno de testes, ADR-0127 contínuo. O RED de qualidade é o Clippy
com -D warnings. Reexecutar os casos e validar linhagem; testes sem falha
funcional anterior não recebem alegação artificial de RED→GREEN funcional.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F043: 01_core/src/compiler/math/layout/tests.rs:3361 (script_y_offset); F044: 01_core/src/compiler/math/layout/tests.rs:4226 (visit); F045: 01_core/src/compiler/math/layout/tests.rs:4240 (visit); F046: 01_core/src/compiler/math/layout/tests.rs:4963 (collect_dot_ys); F047: 01_core/src/compiler/math/layout/tests.rs:6054 (p963_y); F048: 01_core/src/compiler/math/layout/tests.rs:6516 (p971_x); F049: 01_core/src/compiler/math/layout/tests.rs:6647 (visit). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F043 — `script_y_offset`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `TextShaped`, `Line`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F044 — `visit`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard.

- **F045 — `visit`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard.

- **F046 — `collect_dot_ys`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard.

- **F047 — `p963_y`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard; guard.

- **F048 — `p971_x`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard; guard; guard.

- **F049 — `visit`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.


## TS1 — tamanho completo de texto (ratificado pelo humano, 2026-10-17)

### Medição anterior à decisão

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
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

### Ratificação

O humano aprovou TS1 e os dois contratos públicos da proposta de manifesto
771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba.
Implementação e testes S1–S16 sob 12 GiB/swap zero; sem release/merge/push.
Resselo somente após implementação e grafo válido. RG1 continua independente.

### Responsabilidade exclusiva adicional

Somente fixtures/adapters de testes: substituir Style::Size(Pt) por Size(Length absoluto), e expectativas de StyleDelta.size por Option<Length>. Conservar os observáveis linguísticos e os contratos funcionais existentes; não trocar geometria correta por snapshots da representação nova.
