# L0 — math/layout/attach
Hash do Código: 5d94faed

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/attach.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/attach.rs:21` locates `struct PrimesRecipe`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Slots, limits and scripts

Consume the six closed MathAttachSlot states under the pinned slot-presence
nucleus. Only Present(Content) creates a script box and contributes
SpaceAfterScript, including Present(Empty). Omitted and ExplicitNone are
geometrically omitted but remain distinct in the public carrier. Never infer
presence from width, ink, character, text, span or font.

Natural large non-integral operators and limit functions stack attachments
in display; integrals default to side scripts. MathOp's limits flag applies
in display. Explicit MathLimitsOverride uses
`limits && (inline || block)`: limits with inline true stacks even inline,
inline false stacks only display, and scripts never stacks. The wrapper's
inline_explicit bit is morphological, not geometric.

Layout the base once. Postfixed sup/sub share their base-side x origin,
modified by their respective quadrant kerns; their widths are maximized,
not added. Include all top/bottom/left/right slots and pre-script extent in
the final width without counting left extension twice. Preserve existing
script spacing and italics-correction conventions.

Consume signed MathGlyphKern for all four quadrants with the active style;
negative kern is valid. Compute script shifts from font constants, script
ink limits and base-drop limits for non-text-like fragments. Superscript
uses the cramped shift when appropriate; subscript style is cramped.
When both scripts are present, enforce sub_superscript_gap_min and the
superscript-bottom-with-subscript constraint, redistributing shifts as
required rather than fixing them from a fixture.

Limits use:
`top_shift=base.ascent+max(upper_limit_baseline_rise_min,
upper_limit_gap_min+top.descent)`;
`bottom_shift=base.descent+max(lower_limit_baseline_drop_min,
lower_limit_gap_min+bottom.ascent)`.
Emit top at negative shift, bottom at positive shift and derive full extents.

Text-like is a property of the resulting fragment, not merely a Content tag.
Large extensible operators are not text-like even inline. Composite
attach/fraction frames are not text-like; sequences and delimiters aggregate
material children, ignoring immaterial collapsed weak space. Textual MathOp
does not inherit the last shaped glyph's italics correction. For a Large
assembly carried by TextReplacement, consult its visual child glyphs for the
same existing italics-correction path; the occurrence text is not a substitute
for visual glyph metrics. Direct TextItem
has semantic correction zero; mathematical glyph leaves retain metric
correction. Postfixed subscript kern includes minus the base italics
correction; superscript does not receive that compensating term.

## Grouped primes

This owner also lays out MathPrimes, without lowering its public identity.
Counts 1–4 use U+2032/U+2033/U+2034/U+2057. Other counts, including zero,
compose U+2032 with successive offsets `i*prime_width/2` and logical width
`prime_width*(count+1)/2`. Zero is not omission or a fabricated empty slot.
Metric-derived geometry and inherited Normal class remain separate from
attachment merging in evaluation.

Acceptance covers all slots, omitted/none/empty-present differences, limits
and scripts overrides, cramped shifts, signed kerns, common postscript origin,
width/extents and grouped-prime morphology. No dynamic dispatch or code in
the entity owner is authorized.


## Caminhos residuais internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/math/layout/attach.rs` nas linhas 688 contém os itens sem consumo/visibilidade sinalizados pelo compilador. A pesquisa de usos e os dispatchers ativos estão registrados antes da edição em `lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`, com fontes, L0, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status e diffstat. Isto não é nova medição bilateral de paridade.

### Decisão e aceitação

Eliminar somente a variável sub_top sem consumidor e seu comentário contraditório. Preservar o gap simultâneo calculado com sup_b.descent + sub_b.ascent, ajustes shift_up/shift_down e todos os slots.

Correção interna em fluxo contínuo ADR-0127, sem nova assinatura pública, default de produto ou fase. Preservar os resultados de linguagem e as expectativas dos testes. Revalidar todos os targets, suíte core, build, Clippy e linhagem; não suprimir warnings para substituir a correção.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F039: 01_core/src/compiler/math/layout/attach.rs:485 (layout_attach). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F039 — `layout_attach`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Semantic`, `Line`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.


## Exaustividade nominal e origem matemática — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes.

source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.

### Prescrição exclusiva

No início de layout_attach, usar uma referência local base.source_body() e manter layout da base uma vez. No loop de unwrapped_base, normalizar b a cada iteração antes do match; conservar a precedência de wrappers e as condições limits/inline/block atuais. Normalizar o scrutinee dos dois matches sobre MathOp.text e a recursão content_is_text_like. Reescrever somente a projeção extract_char: let c = c?; match c.source_body() com os dois casos atuais e complemento nominal None. Não aceitar Text como novo script elegível, não inferir presença por largura/tinta e não apagar Source do carrier armazenado. Nenhum novo campo/algoritmo de limite ou mudança de fórmulas/kerns/shifts.

Trocar os fallbacks enumerados pelo complemento nominal atual dos braços **incondicionais**, sem alterar corpo/guard/ordem; variantes guardadas continuam no complemento para quando a condição falha. A projeção Option de attach é a única exceção estrutural: None retorna por ?, Some passa ao match nominal de Content normalizado, sem padrão profundo.

- **L04** — `01_core/src/compiler/math/layout/attach.rs:181`, `layout_attach`, Content: excluir somente `MathLimitsOverride, MathClassOverride, MathStyled, Styled`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L05** — `01_core/src/compiler/math/layout/attach.rs:189`, `layout_attach`, Content: excluir somente `MathLimitsOverride`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L06** — `01_core/src/compiler/math/layout/attach.rs:199`, `layout_attach`, Content: excluir somente `MathIdent, MathText, MathOp`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L07** — `01_core/src/compiler/math/layout/attach.rs:253`, `layout_attach`, Content: excluir somente `Text, MathText, MathIdent`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L08** — `01_core/src/compiler/math/layout/attach.rs:255`, `layout_attach`, Content: excluir somente `MathIdent, MathText, MathOp`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L09** — `01_core/src/compiler/math/layout/attach.rs:262`, `layout_attach`, Content: excluir somente `MathIdent, MathText`; guardados `(nenhum)`. Option None retorna None via ?; Some inspeciona Content normalizado e o complemento mantém None. Não criar padrão profundo Some(Content alternativo).

- **L10** — `01_core/src/compiler/math/layout/attach.rs:281`, `layout_attach`, Content: excluir somente `MathOp, MathIdent, MathText`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L11** — `01_core/src/compiler/math/layout/attach.rs:302`, `content_is_text_like`, Content: excluir somente `MathIdent, MathText, MathOp, MathDelimited, MathSequence, Sequence, MathClassOverride, MathLimitsOverride, MathStyled, Styled`; guardados `HSpace`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita. Variantes sob guard permanecem no complemento para conservar a condição recusada.

- **L12** — `01_core/src/compiler/math/layout/attach.rs:312`, `layout_attach`, FrameItem: excluir somente `Glyph`; guardados `(nenhum)`. O coletor procura somente a espécie de item explicitada; as demais não são candidatos. Conservar os asserts de presença/quantidade e não confundir None com sucesso de layout.

- **L13** — `01_core/src/compiler/math/layout/attach.rs:321`, `layout_attach`, Content: excluir somente `(nenhuma variante inteira)`; guardados `MathIdent, MathText`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita. Variantes sob guard permanecem no complemento para conservar a condição recusada.

- **L14** — `01_core/src/compiler/math/layout/attach.rs:335`, `layout_attach`, Content: excluir somente `Text, MathText, MathIdent`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L15** — `01_core/src/compiler/math/layout/attach.rs:337`, `layout_attach`, Content: excluir somente `MathIdent, MathText, MathOp`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

### Aceitação e limites

L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.
