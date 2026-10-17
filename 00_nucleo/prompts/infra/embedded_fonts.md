# L0 — Embedded font sets
Hash do Código: 528ec5d1

**Layer:** L3
**Exclusive consumer:** `03_infra/src/embedded_fonts.rs`

## Current evidence

- `03_infra/src/embedded_fonts.rs:31` — `EmbeddedFontSets`.
- `03_infra/src/embedded_fonts.rs:66` — `embedded_font_group`.
- `03_infra/src/embedded_fonts.rs:85` — `load_embedded_fonts`.
- `Cargo.lock` — typst-assets revision `c0ae970f64413cf8e39cecd56d15ee50de42eaf4`.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own loading and classification of the compiled `typst_assets::fonts()` bytes. `EmbeddedFontSets` contains `text_slots`, `text_book`, `math_code_slots` and `math_code_book`. Extract metadata through the fonts owner, create embedded slots with display path `<embedded>`, and insert valid metadata/slots together. Loading does not read the filesystem.

Classify the normalized metadata family name, not the raw filename or uncorrected nameID 1. Case-insensitive names starting with `Libertinus Serif` or exactly `New Computer Modern` belong to text. Exactly `New Computer Modern Math`, names starting with `DejaVu Sans Mono`, and unrecognized names belong to math/code. The metadata exception table belongs to `00_nucleo/prompts/infra/fonts.md`; preserve its documented family names with spaces.

The embedded asset set makes Libertinus Serif and the bundled math/code faces available independently of host installation. `00_nucleo/prompts/infra/system-world/fonts.md` exclusively owns their insertion relative to system/project fonts and the existing builders. The language's default text family belongs to L1 style owners; neither those defaults nor CLI composition are redefined here.

## Verification boundary

Verify embedded slots load without disk access, Libertinus Serif and New Computer Modern occur in text, and New Computer Modern Math/DejaVu Sans Mono occur in math/code. Positive membership controls must accompany exclusion checks; an absent family must not make an exclusion test pass vacuously.

## Transferência única para o slot — medição 2026-10-08

`03_infra/src/embedded_fonts.rs:92-96`, congelado em `00_nucleo/diagnosticos/embedded-fonts-memoria2-2026-10-08/before.json.gz`, copia slice estático para Vec, clona o Vec ao construir o slot e descarta o primeiro Vec após obter os metadados. São dois buffers por iteração de carga; não afirmar duplicação persistente de todo o catálogo por esse clone temporário.

Extrair os mesmos metadados diretamente do slice de typst_assets. Somente com metadados válidos, copiar uma vez os bytes para o Vec transferido ao new_embedded existente. Conservar composição, classificação, ordem e pareamento slot/book. Não substituir a representação pública dos assets/fonts, não alterar descoberta ou default de linguagem. Aceitação: classificação e carregamento existentes, compartilhamento source/get do owner fonts, controles de exportação e medição conjunta com proveniência.

## Catálogo por fatias estáticas — 2026-10-08

### Medição anterior à decisão

`00_nucleo/diagnosticos/custo-fixo-fontes-item2-2026-10-08/probe-results.json` mede 9.682.300 bytes nos 17 assets embutidos, ~9,24 MiB de alocação própria retida e páginas estáticas residentes do executável. O ponto de cópia está em `03_infra/src/embedded_fonts.rs:92-94`; o loader vanilla pinado em `lab/typst-original/crates/typst-kit/src/fonts.rs:136-141` passa os slices estáticos a Bytes sem Vec. A comparação de CLI simples atual reproduziu 41,83 MiB Tekt e 29,21 MiB vanilla, com texto visível igual; a lacuna não é atribuída toda a este ponto sem ensaio após mudança. Estes são fatos de armazenamento e saída observável, não intenção histórica.

### Decisão

Supersede a transferência obrigatória de cada asset para Vec: após extrair metadados do slice original, criar slot de origem estática pelo construtor interno do owner `fonts.md`. Não copiar assets não selecionados. Conservar classificação, ordem, book, path de exibição `<embedded>` e pareamento. O owner fonts possui a materialização lazy de uma face no `get`; este loader não a duplica. Correção interna ADR-0127, sem API pública, modo ou fase novos. Aceitação bilateral de texto/estilo/geometry/raster onde o baseline já coincidia, e ensaio pareado RSS/tempo com proveniência.

## Fixtures ativos e resíduos de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/embedded_fonts.rs:192` possui uma função com asserções de exclusão e presença de New Computer Modern Math, mas sem `#[test]`. Em `:135`, a sonda de famílias só imprime metadados e não contém asserção. Em `:235`, o controle DejaVu observa apenas exclusão de texto, sem presença em math/code.

Fontes, owners, diagnósticos, HEAD, working tree não commitado, UTC, status e diffstat estão em `00_nucleo/diagnosticos/fixtures-ativos-ci-2026-10-13/before.json.gz`. O registro real da suíte antes da edição confirma ausência do teste matemático. Estas evidências não constituem nova comparação com upstream.

### Decisão e aceitação

Registrar p754_newcm_math_is_not_in_text_group como teste executável, conservando suas asserções. Retirar somente p797_probe_list_all_embedded_families: o inventário impresso é diagnóstico, não verificação. Acrescentar controle positivo de DejaVu Sans Mono em math/code ao teste de exclusão existente, como já exige o contrato. Conservar todos os outros testes. A lista deve trocar exatamente o nome da sonda pelo teste matemático; validar também que a presença positiva não é satisfeita por uma lista vazia.

Manutenção interna de testes (ADR-0127), sem assinatura pública, default ou fase nova. L0 precede código; resselo limitado aos owners envolvidos. Executar suítes shell e infra, compilar todos os targets, verificar formato, linhagem e inventário de Clippy. Contagens de testes não certificam paridade.
