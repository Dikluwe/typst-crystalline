# L0 — infra/font_metrics/resolution
Hash do Código: a3cf993c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/resolution.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52
- 00_nucleo/prompts/_nuclei/fonts/primary-resolution-cache.toml sha256:b18027804dddf2f9eb87a3d01168d95dffaa34ad6839bad618e758ffbff080b4
```

## Current evidence

- `03_infra/src/font_metrics/resolution.rs:17` — `FontCandidate`.
- `03_infra/src/font_metrics/resolution.rs:27` — `resolve_primary`.
- `03_infra/src/font_metrics/resolution.rs:86` — `resolve_primary_with_math_fallback`.
- `03_infra/src/font_metrics/resolution.rs:118` — `covering`.
- `03_infra/src/font_metrics/resolution.rs:193` — `resolve_font_combo`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/resolution.rs`.

## Owned contract

- Own FontCandidate and primary/MATH/covering/FontKey resolution. Preserve declared-family order, class fallback when primary lookup fails and first-resolved-family similarity reference.
- Append explicit math fallback faces when style.math is true, with slot deduplication; preserve the existing readable-face check without adding a MATH-table filter at insertion. Covering lookup first tries MATH-covering candidates, then primary coverage; preserve None and ordering.
- Global fallback uses World::candidates_for_char and FontBook::select_fallback. Coverage is authoritative for global filtering; load the winning eligible face only, not intervening slots or a second full catalog scan.
- Resolved identity includes family, variant and variations. Do not infer math semantics from family spelling; consume the pinned shared fallback-selection nucleus.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Cadeia comum alinhada com shaping — 2026-09-29

Medição: `03_infra/src/font_metrics/resolution.rs:27-65` usa só a primária quando ela resolve, enquanto `text::families()` do pin vanilla (`lab/typst-original/crates/typst-library/src/text/mod.rs:1083-1101`) acrescenta a cadeia comum antes da busca global. A mesma sonda hebraica do diagnóstico `correcao-rtl-restante-2026-09-29/after-leading` mostra face diferente na emissão. Inferência: métricas separadas também podem divergir se não consultarem a cadeia comum; refutar por face medida já idêntica sem alteração.

Acrescentar a cadeia comum ordenada e deduplicada depois das famílias declaradas legíveis no modo textual; em math, manter a cadeia matemática antes da comum. A medição de cobertura e a emissão não devem escolher famílias distintas para o mesmo caractere por causa da ordem dos catálogos.

## Reuso limitado da resolução primária — 2026-10-10

### Medição anterior à decisão

`lab/results/construction/diagnostics/content-cost-profile-2026-10-10/{resolution-calls,cpu-style-current,checked-phases}.json.gz` guarda HEAD/working tree/UTC, scripts, fontes, comandos e release `9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`. As tabelas de 1.600 células chamam select_pattern 48.019 vezes, resolve_primary 6.401 vezes e text_edges/vertical_metrics 3.200/3.201 vezes; a seleção aparece em 25–29% das amostras inclusivas de CPU. `font_metrics/resolution.rs:27–85` depende de FontList, variante efetiva e legibilidade da face para formar a cadeia base; não aplica tamanho nem variações. `:105–124` acrescenta MATH separadamente; `font_metrics.rs:49–86` já compartilha caches locais entre clones. Inferência refutável: reusar a última resolução pode amortizar scans sem retenção crescente; invalidar se identidades/cadeia não forem preservadas ou custo de lock/retenção anular o ganho.

### Responsabilidade exclusiva

Definir internamente a entrada/chave e usar a prova de identidade exata antes de resolver a cadeia base. Em hit, clonar os candidatos existentes; em miss, executar o mesmo algoritmo e publicar a entrada somente se o resultado for não-vazio e todas as faces selecionadas forem legíveis. Substituir a entrada anterior; liberar o lock antes de resolver/carregar. Os slots e unidades são baseados na face original; variações e a identidade final FontKey continuam sendo resolvidas no caminho vigente, e as adições MATH não entram neste cache. Aplicar o Núcleo pinado sem alterar assinaturas existentes.

### Aceitação e classe

Mudança interna ADR-0127 em fluxo contínuo. Teste RED de consultas repetidas nos mesmos argumentos através de clones; controles de família/regex/ordem/variante, instâncias independentes, falhas transitórias e math/variações contra instância fresca. Gate quantitativo de alocações e tempo pareado, documentos equivalentes ao release anterior e refutadores de muitas identidades, core/infra/build/lint. Aceitação de linguagem por seleção, métricas, origem/efeitos e geometria; não copiar mecânica do vanilla nem inferir hit por hash. Retirar em divergência ou regressão robusta.


## Expressões equivalentes e CI estrito — 2026-10-14

### Medição anterior à decisão

`03_infra/src/font_metrics/resolution.rs` nas linhas [66, 85, 107, 154, 220] contém as expressões apontadas pelo inventário
Clippy. `lab/results/construction/diagnostics/ci-simplificacoes-2026-10-14/before.json.gz`
conserva fonte/owner, spans, HEAD, working tree não commitado, UTC e diffstat;
red.log registra os lints selecionados como erros antes de alterar código.
Os tipos e usos foram conferidos no consumer e no toolchain 1.92.0 pinado.
Esta é manutenção da expressão Rust; não é medição nova de paridade upstream.

### Responsabilidade e aceitação

units_per_em da face já devolve u16. Manter max(1) e retirar somente casts u16→u16 nas cadeias primária, comum, MATH e covering. Ordem, deduplicação, identidade da face e elegibilidade do cache permanecem iguais.

Correção interna em fluxo contínuo ADR-0127, sem API pública, default, fase,
unsafe, dependência ou supressão de lint novos. L0 precede código e resselo
fica restrito ao owner. Usar os controles funcionais existentes; não alterar
fixtures, expectativas, tolerâncias ou registrar testes que apenas espelhem
esta simplificação. Os lints selecionados devem passar como erros, seguidos
de suíte completa, build, all-targets, rustdoc, fmt, linhagem e inventário total.


## Inicialização explícita dos fixtures — field_reassign_with_default

### Medição anterior à decisão

`03_infra/src/font_metrics/resolution.rs` nas linhas 302, 426, 555, 598 cria defaults e em seguida sobrescreve campos locais de testes. A fonte, os spans e os L0 anteriores estão em `lab/results/construction/diagnostics/field-reassign-2026-10-15/before.json.gz`, com HEAD, working tree, UTC e diffstat; initial-diagnostics.json conserva os inicializadores apontados. Esta manutenção trata representação Rust, sem nova medição de paridade ou mudança no default do produto.

### Responsabilidade e aceitação

Inicializar font e, nos dois controles matemáticos, math diretamente nos quatro literais TextStyle. Preservar famílias, mocks, contadores de consultas, seleção de face MATH e checks de fallback; não alterar o algoritmo produtivo.

Usar `..Tipo::default()` para os campos restantes e retirar `mut` apenas onde deixam de existir mutações. Sem novos tipos, builders, APIs ou supressões de lint. L0 precede código e resselo restrito (fluxo contínuo ADR-0127). O gate selecionado deve falhar antes e passar depois; preservar nomes, inputs, expectativas e tolerâncias dos testes. Revalidar Clippy all-targets, suíte workspace, build, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe do inicializador.
