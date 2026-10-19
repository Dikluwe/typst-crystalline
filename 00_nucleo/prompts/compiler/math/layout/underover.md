# L0 — math/layout/underover
Hash do Código: 1255f393

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/underover.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/underover.rs:20` locates `pub(super) fn layout_overbrace(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Spreaders and optional annotations

Own the geometry of MathUnderover and the public over/under brace, bracket,
paren and shell entrypoints. Public identity/presence is preserved by the
core; local lowering does not replace the language carrier.

Glyph pairs (above/below) are brace U+23DE/U+23DF, bracket U+23B4/U+23B5,
paren U+23DC/U+23DD and shell U+23E0/U+23E1. Stretch the support to the body
width at 100% with zero shortfall; an annotation is not the stretch target.
Support glyphs use full-size accent geometry. Optional annotations retain
their absence/empty-present distinction, use superscript/subscript style,
and may widen the final frame without redefining the support's target.
Under annotations are cramped; over annotations inherit the appropriate
superscript style.

For an above support:
`support_y=-base.ascent+min(base.ascent,accent_base_height)`.
For below:
`support_y=base.descent+max(0,support.ink_up)`.
Use signed actual ink extents; do not substitute overbar/underbar constants
or global font ascent. Union support/base ink for compound extents.

Exact-frame-width placement uses attachments:
`pre=support_attach-base_attach`;
`post=(support.width-support_attach)-(base.width-base_attach)`;
`width=max(pre,0)+base.width+max(post,0)`.
If pre<0, base_x=0 and support_x=-pre; otherwise base_x=pre, support_x=0.
Atomic glyphs use applicable TopAccentAttachment; composite bases use
half-width. A support with one glyph uses its attachment; multi-glyph
assembly uses half the assembled width. Preserve the existing symmetric
single-glyph asymmetry compensation
`2*abs(support_attach-support.width/2)` at the support level only, not
again in an annotation wrapper.

Annotations use limit shifts, not tight ink stacking:
under baseline = `base.descent+max(lower_drop_min,
lower_gap_min+annotation.ascent)`;
over baseline = `-(base.ascent+max(upper_rise_min,
upper_gap_min+annotation.descent))`.
The support and annotation paths remain distinguishable; historical generic
one-character dispatch does not redefine an explicit public annotation.

Acceptance covers selected support variants/assemblies, signed ink gaps,
nested supports, exact width/attachments, optional annotations and font-derived
limit placement without hardcoded positions or serialized-byte equality.

## Identidade dos suportes elásticos

Um suporte de um caractere retém esse caractere como identidade extraível
única em `TextReplacement` envolvendo seus itens visuais, inclusive quando
uma montagem horizontal usa várias peças. `.typ/sec_10.typ` mostra que
extrair as peças de under/overbracket cria caracteres que não existem no
conteúdo autoral. A substituição não altera geometria ou anotação.

## Atomização interna do owner

A seleção de suporte versus anotação, o estilo da anotação, a leitura de
attachments e o envelope textual do suporte devem ser operações privadas
distintas dentro de `underover.rs`. A composição final conserva as fórmulas
geométricas acima e aplica a identidade textual apenas aos itens visuais da
peça. Nenhum novo consumer, API ou modo de produto decorre desta separação.


## Exaustividade nominal e origem matemática — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes.

source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.

### Prescrição exclusiva

Normalizar Source nos scrutinees de piece_char, nested_spreader, base_char_for_attachment e na classificação da base de layout_underover_with_attach. Preservar seleção de suporte/anotação, identidade pública, estilos, fórmulas de attachment/limites e translação. Um suporte Source de um caractere não vira anotação em tamanho de script.

Trocar os fallbacks enumerados pelo complemento nominal atual dos braços **incondicionais**, sem alterar corpo/guard/ordem; variantes guardadas continuam no complemento para quando a condição falha. A projeção Option de attach é a única exceção estrutural: None retorna por ?, Some passa ao match nominal de Content normalizado, sem padrão profundo.

- **L32** — `01_core/src/compiler/math/layout/underover.rs:37`, `piece_char`, Content: excluir somente `(nenhuma variante inteira)`; guardados `MathText`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita. Variantes sob guard permanecem no complemento para conservar a condição recusada.

- **L33** — `01_core/src/compiler/math/layout/underover.rs:53`, `nested_spreader`, Content: excluir somente `MathUnderover`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L34** — `01_core/src/compiler/math/layout/underover.rs:61`, `base_char_for_attachment`, Content: excluir somente `MathAccent`; guardados `MathText`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita. Variantes sob guard permanecem no complemento para conservar a condição recusada.

- **L35** — `01_core/src/compiler/math/layout/underover.rs:166`, `spreader_attachment`, FrameItem: excluir somente `Glyph`; guardados `(nenhum)`. O coletor procura somente a espécie de item explicitada; as demais não são candidatos. Conservar os asserts de presença/quantidade e não confundir None com sucesso de layout.

- **L36** — `01_core/src/compiler/math/layout/underover.rs:227`, `layout_underover_with_attach`, Content: excluir somente `MathUnderover`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

### Aceitação e limites

L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.
