# L0 — math/layout/p1431-tests
Hash do Código: 10b1c5ae


**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/p1431_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/p1431_tests.rs:27` locates `struct TestWorld`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Independent matrix tests

Own only `01_core/src/compiler/math/layout/p1431_tests.rs`.
Use deterministic metrics and observe horizontal alignment, late MathAlignPoint
splitting, axis-specific relative gaps, every hline/vline offset including
borders/negatives, auto/custom stroke, delimiters, denominator style and extents.
Out-of-dimension offsets are diagnostic, not clamp/ignore.
Preserve the established P825/P912/P919/P923/P945/P1042 controls and
compatibility wrapper. Unattributable frame/font/stroke/geometry is Unknown
and blocks acceptance.

P26 includes unequal alignment-point counts and mixed classes. For asymmetric
`(1 & 2), (33 & 4)`, require
`x(1)+width(1)==x(33)+width(33)` and `x(2)==x(4)`.
Widths come independently from FixedMetrics::text_width at observed item size,
never from grid-produced positions. Centering subcolumns must fail the first
equation.

P27 compares default, 1em and 150% gaps in both axes.
P29 observes thickness, color and cap: auto=0.05em Square; custom/fold
preserves all three. P30 includes optional left/right delimiters in a tall
grid. P31 covers Display, Text/Inline, Script and ScriptScript with denominator
descent. P34 observes automatic Square, custom cap and compatibility Butt
in final FrameItem::Line.

M23 requires duplicate/out-of-order offsets, retaining coordinate quantity and
order; compute interior midpoints independently from cell metrics, while
borders 0/n coincide with complete box bounds.
M25 uses valid/invalid 2x3 and 3x2 matrices: hline validates rows, vline columns.
M26 calls Content::math_matrix directly, requires the complete mat carrier
and compares geometry with the equivalent common route. None of these
assertions promotes a private implementation shape or bytes to a requirement.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Atualizar somente a construção dos Values de stroke para From<Stroke>, conservando exatamente expectativas e discriminação de geometria do augment.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.

Ratificação Memória 5: o dono confirmou o manifesto SHA-256
6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237
com “confirmo” em 2026-10-08. O gate ADR-0127 deste pacote está satisfeito;
suas cláusulas propostas passam a vigorar nesta materialização.


## Exaustividade nominal e origem matemática — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes.

source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.

### Prescrição exclusiva

Manutenção somente do helper/teste ou do eixo MathSize descrito abaixo. Não ampliar travessia ou reescrever os cenários existentes deste owner.

Trocar os fallbacks enumerados pelo complemento nominal atual dos braços **incondicionais**, sem alterar corpo/guard/ordem; variantes guardadas continuam no complemento para quando a condição falha. A projeção Option de attach é a única exceção estrutural: None retorna por ?, Some passa ao match nominal de Content normalizado, sem padrão profundo.

- **L24** — `01_core/src/compiler/math/layout/p1431_tests.rs:80`, `find_mat`, Content: excluir somente `MathMatrix, Equation, Styled, MathDelimited, Sequence, MathSequence`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L25** — `01_core/src/compiler/math/layout/p1431_tests.rs:128`, `texts`, FrameItem: excluir somente `Text, TextShaped`; guardados `(nenhum)`. O coletor procura somente a espécie de item explicitada; as demais não são candidatos. Conservar os asserts de presença/quantidade e não confundir None com sucesso de layout.

- **L26** — `01_core/src/compiler/math/layout/p1431_tests.rs:148`, `lines`, FrameItem: excluir somente `Line`; guardados `(nenhum)`. O coletor procura somente a espécie de item explicitada; as demais não são candidatos. Conservar os asserts de presença/quantidade e não confundir None com sucesso de layout.

### Aceitação e limites

L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.
