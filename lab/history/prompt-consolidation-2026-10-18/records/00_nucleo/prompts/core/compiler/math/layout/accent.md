# L0 — math/layout/accent
Hash do Código: d8c8ff7c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/accent.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/accent.rs:29` locates `pub(super) fn layout_accent(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Complete accent carrier

Consume MathAccentElem without mutating public Content. Resolve size and
dotless independently by explicit > math.accent cascade > default:
size=100%, dotless=true. Preserve absence and explicit false/zero.
Unicode combining class Below and ⏟/⎵/⏝/⏡ select a lower accent.
Above accents use a cramped base and contextual dotless i/j when enabled;
below accents use a non-cramped base and no dotless rewriting.

Resolve horizontal size against the measured base width, with absolute
length against causal font size. Stretch uses shortfall 0.5em through
`00_nucleo/prompts/core/compiler/math/layout/stretchy.md`, not the vertical
delimiter policy. Preserve the base's identity and source presence metadata.

Above placement is baseline-relative:
`accent_y = -base.ascent + min(base.ascent, accent_base_height)`;
`ascent = max(base.ascent, -accent_y + accent.ascent)`.
Keep base descent. Below placement puts the accent beneath the base and
includes its lower ink extent. Attachment, not arbitrary fixture centering,
determines x: `base_attach - accent_attach`, with the existing metric
fallbacks for composite bases. The accent must not erase nested accent extent.

Acceptance distinguishes orientation, size 100%/150%, dotless on/off for i/j,
attachment and nested baseline geometry, using attributable font metrics.
The current oversized-accent width heuristic and incomplete attachment
attribution are audit limitations, not new normative exceptions.

## Identidade extraível dos acentos

Na sonda `.typ/sec_10.typ` e `.typ/sec_12.typ` do diagnóstico
`typs-teste-2026-09-30`, `pdftotext -layout` separa/inverte a marca combinante
e sua base enquanto a expressão da linguagem conserva a relação base+marca.
Quando a base realizada é uma folha textual de um caractere e o acento é uma
marca combinante, envolver os itens visuais posicionados em um
`TextReplacement` com a sequência base seguida da marca, por ocorrência.
Para base composta, conservar seus itens e não inventar uma serialização
textual. O envelope não muda tamanho, deslocamento, fonte nem desenho.

A sonda pós-correção mostrou `dot(dot(x))` extraído como marca+base+marca
quando o envelope interno permanece aninhado. Para acentos aninhados de uma
base textual atômica, compor a sequência de todas as marcas na ordem interna
para externa e abrir somente um envelope externo; os envelopes internos são
transparentes apenas para esse caminho, mantendo todos os filhos visuais.

## Atomização interna do owner

A resolução de `size`/`dotless`, a orientação da marca, o posicionamento
geométrico e a composição da identidade textual devem ser operações privadas
distintas dentro do consumer `accent.rs`. A identidade extraível é aplicada
depois de posicionar os itens e não altera métricas. Essa separação não
introduz consumer, API ou comportamento novo.

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/math/layout/accent.rs` nas linhas 29 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Compilar o helper interno `layout_accent` somente sob `#[cfg(test)]`, conservando assinatura e corpo para os testes existentes. O despacho produtivo usa layout_accent_elem; o wrapper preserva seu cenário histórico de tamanho natural e dotless false somente para testes.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.


## Caminhos residuais internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/math/layout/accent.rs` nas linhas 153 contém os itens sem consumo/visibilidade sinalizados pelo compilador. A pesquisa de usos e os dispatchers ativos estão registrados antes da edição em `lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`, com fontes, L0, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status e diffstat. Isto não é nova medição bilateral de paridade.

### Decisão e aceitação

Eliminar somente dx, diferença duplicada sem consumidor; manter accent_x = base_attach - accent_attach e todas as consultas de attachment, geometria e identidade do acento.

Correção interna em fluxo contínuo ADR-0127, sem nova assinatura pública, default de produto ou fase. Preservar os resultados de linguagem e as expectativas dos testes. Revalidar todos os targets, suíte core, build, Clippy e linhagem; não suprimir warnings para substituir a correção.


## Exaustividade nominal e origem matemática — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes.

source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.

### Prescrição exclusiva

Normalizar Source por empréstimo nos scrutinees de extract_base_char, one_char e atom_identity. Em dotless_base, transformar o body normalizado e reaplicar a primeira origem com os helpers existentes source_span/with_source_span; conservar explicit, size, dotless, campos, estilos e código dos demais braços. Isso torna i/j dotless também sob Source e conserva uma ocorrência textual base+marcas, sem usar a string para medir desenho.

Trocar os fallbacks enumerados pelo complemento nominal atual dos braços **incondicionais**, sem alterar corpo/guard/ordem; variantes guardadas continuam no complemento para quando a condição falha. A projeção Option de attach é a única exceção estrutural: None retorna por ?, Some passa ao match nominal de Content normalizado, sem padrão profundo.

- **L01** — `01_core/src/compiler/math/layout/accent.rs:131`, `extract_base_char`, Content: excluir somente `MathAccent`; guardados `MathText`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita. Variantes sob guard permanecem no complemento para conservar a condição recusada.

- **L02** — `01_core/src/compiler/math/layout/accent.rs:136`, `layout_accent_resolved`, Content: excluir somente `(nenhuma variante inteira)`; guardados `MathText`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita. Variantes sob guard permanecem no complemento para conservar a condição recusada.

- **L03** — `01_core/src/compiler/math/layout/accent.rs:241`, `atom_identity`, Content: excluir somente `(nenhuma variante inteira)`; guardados `MathText, MathIdent, MathAccent`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita. Variantes sob guard permanecem no complemento para conservar a condição recusada.

### Aceitação e limites

L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.
