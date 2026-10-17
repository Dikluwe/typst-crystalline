# L0 — math/layout/spacing
Hash do Código: e89a1097

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/spacing.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/spacing.rs:29` locates `fn base_math_class(content: &Content) -> MathClass`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Mathematical classes and automatic gaps

Own node_math_class, promote_vary, spacing_between and compute_gaps.
Classify named MathIdent from its resolved Unicode symbol, not the first letter
of the name; unresolved identifiers and MathText retain their existing
first-scalar classification. TextItem is Alphabetic; MathOp is Large regardless
of its limits flag. MathStyled and MathLimitsOverride preserve the body's math
class; MathAttach inherits the base's math class. Mathematical underline,
overline and public spreaders preserve their body's boundary classes.
MathClassOverride explicitly supplies its class. MathMid is Relation;
MathPrimes is Normal. Optional matrix/vector delimiters expose Opening/Closing
only on present sides, otherwise Normal. Preserve existing fenced/fence
boundary handling and sequence edge classes.

Space and SpaceFromNewline remain neutral Normal; newline origin does not
trigger textual fallback, CJK normalization or show/regex processing.
First-scalar classification and composite accent/cancel/underover inheritance
limitations are audit items, not a language-wide definition.

Promote Vary to Binary after Normal, Alphabetic, Closing or Fence; otherwise
it stays unary-like without fabricating a Binary gap. Apply ordered spacing
rules (size in em):
1. Before Punctuation: zero; after Punctuation: 1/6.
2. After Opening or before Closing: zero.
3. Relation–Relation: zero; other Relation boundaries: 5/18.
4. Binary boundaries: 2/9.
5. Large boundaries: 1/6 except the established Opening/Fence exclusions.
6. No explicit rule: zero, subject to spaced-item fallback.

Unary has no dedicated override that would defeat a later Binary rule.
Keep an explicit zero rule distinct from absence of a rule. Only absence
allows fallback to the active font's measured text-space advance for spaced
TextItem/atomic textual markup or Fence. Do not hardcode a point width or
trigger fallback from SpaceFromNewline.
Compute n-1 gaps sequentially with Vary promotion; in script size, class
spacing is suppressed, not merely reduced. The caller supplies script state
when math_script or Script/ScriptScript applies. Preserve alignment-boundary
spacing delegation and explicit-zero priority there too.

Acceptance observes class overrides, boundary asymmetry, Large operators,
integrals, symbol names, textual/fence fallback, punctuation and script
suppression using attributable geometry. Generic spaced frames, normal-class
spaced wrappers, per-item mixed sizes and composite inheritance remain
limitations requiring separate evidence; do not encode their defects as defaults.


## Realização com avaliação única

Classe matemática dos carriers internos delega à projeção visual, conservando regras de espaçamento nativas.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

A delegação transparente já owned de ShowRecipe.body também aceita Source.body, sem executar origem separada, publicar carrier ou atribuir Location. Preservar classificação/preparo/serialização conforme a operação desta unidade.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Fluxo de controle e verificações executáveis — 2026-10-13

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. 01_core/src/compiler/math/layout/spacing.rs:807-811; :368-390 contém o trecho medido. O inventário Clippy e a leitura dos owners distinguem código sem efeito, dados inválidos deliberados e verificações vazias. Fontes, L0, UTC, status, diffstat e comandos estão em `00_nucleo/diagnosticos/controle-fluxo-ci-2026-10-13/before.json.gz` e nas evidências do mesmo diretório. Não é nova medição de paridade nem inferência sobre intenção upstream.

### Decisão e aceitação

O teste de hierarquia de espaçamento deve chamar spacing_between e verificar as larguras normativas Relation=5/18em, Binary=2/9em e Large=1/6em, em tamanhos distintos. Não se limitar a comparar as constantes. Retirar somente os padrões Equation e Styled duplicados no fallback e a regra Opening/Fence–Fence/Closing já coberta pela regra anterior geral; conservar os braços prioritários e seu resultado zero explícito.

Preservar APIs públicas, comportamento padrão, fase e expectativas de linguagem existentes. Revalidar a suíte apropriada, Clippy, build e linhagem. Correção interna em fluxo contínuo ADR-0127.

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/math/layout/spacing.rs` nas linhas 396 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `00_nucleo/diagnosticos/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Compilar o helper interno `spacing_between` somente sob `#[cfg(test)]`, conservando assinatura e corpo para os testes existentes. A produção usa spacing_between_class por compute_gaps. O wrapper de retorno zero no catch-all permanece nos testes de classes; a função produtiva e seu Option continuam intactos.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.


## Exaustividade nominal e origem matemática — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `00_nucleo/diagnosticos/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes.

source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.

### Prescrição exclusiva

Normalizar content.source_body() nos predicados is_textual_spaced_item/is_bare_pipe e cada child.source_body() na elegibilidade de Sequence textual. Não mudar classes, regras de gap, prioridade zero explícito, promoção Vary, fatores ou tamanho de fonte. Restabelecer somente a classificação que Source não pode alterar.

Trocar os fallbacks enumerados pelo complemento nominal atual dos braços **incondicionais**, sem alterar corpo/guard/ordem; variantes guardadas continuam no complemento para quando a condição falha. A projeção Option de attach é a única exceção estrutural: None retorna por ?, Some passa ao match nominal de Content normalizado, sem padrão profundo.

- **L28** — `01_core/src/compiler/math/layout/spacing.rs:532`, `is_textual_spaced_item`, Content: excluir somente `Text, Sequence`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L29** — `01_core/src/compiler/math/layout/spacing.rs:541`, `is_bare_pipe`, Content: excluir somente `MathText, MathIdent, MathStyled, Styled`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

### Aceitação e limites

L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.
