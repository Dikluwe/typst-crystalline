# L0 — math/layout/stretch
Hash do Código: 3ea5a6b2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/stretch.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/stretch.rs:15` locates `fn leaf_char(content: &Content) -> Option<char>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Public stretch

Own the free function for MathStretchElem, in the render layer rather than
the entity. Preserve body identity and size_explicit. Resolve effective size
by explicit > math.stretch.size > 100%, retaining zero and mixed length as
values rather than absence.

Measure natural body, select its native extensible axis and apply size with
zero shortfall. Percentage references the natural maximum dimension including
attachments; absolute length uses causal font size. Delegate variant/assembly
selection to `00_nucleo/prompts/compiler/math/layout/stretchy.md`;
do not replicate MATH tables or continuously scale glyph ink.
Preserve body classes/boundaries. A nonextensible body retains natural layout
and the applicable already-ratified diagnostic; do not invent a new error.

Acceptance covers both axes, attachments, default/absolute/mixed/cascaded
sizes, nesting and fallback. Current leaf-only eligibility and silent
fallbacks do not narrow this approved contract; unresolved implementation/
diagnostic coverage is recorded in the diagnostic.


## Exaustividade nominal e origem matemática — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes.

source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.

### Prescrição exclusiva

Normalizar Source na elegibilidade leaf_char; conservar os casts e a seleção de eixo/variante/assembly/size. Isso não amplia a antiga elegibilidade leaf-only para corpos compostos nem redefine o contrato completo de native axis: o débito separado math-stretch-composite continua aberto e deve completar anexos/composições com métricas e medição próprias. Não ratificar esse fallback parcial como default normativo ou prometer paridade completa.

Trocar os fallbacks enumerados pelo complemento nominal atual dos braços **incondicionais**, sem alterar corpo/guard/ordem; variantes guardadas continuam no complemento para quando a condição falha. A projeção Option de attach é a única exceção estrutural: None retorna por ?, Some passa ao match nominal de Content normalizado, sem padrão profundo.

- **L30** — `01_core/src/compiler/math/layout/stretch.rs:20`, `leaf_char`, Content: excluir somente `Text, MathText, MathIdent, MathClassOverride, MathLimitsOverride`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L31** — `01_core/src/compiler/math/layout/stretch.rs:34`, `cascaded_size`, Value: excluir somente `Relative, Ratio, Length`; guardados `(nenhum)`. Somente Relative/Ratio/Length representam size nesta fronteira. Os demais conservam None; isso não autoriza descartar erros de validação do produtor.

### Aceitação e limites

L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.
