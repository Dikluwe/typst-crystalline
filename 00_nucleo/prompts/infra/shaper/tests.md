# L0 — infra/shaper/tests
Hash do Código: e49276d7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95


**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/tests.rs`

## Current evidence

- `03_infra/src/shaper/tests.rs:38` — `p1404_fallback_interner_e_esparso_e_idempotente`.
- `03_infra/src/shaper/tests.rs:48` — `p1404_controles_de_formato_nao_pedem_glifo`.
- `03_infra/src/shaper/tests.rs:56` — `MockWorld`.
- `03_infra/src/shaper/tests.rs:61` — `library`.
- `03_infra/src/shaper/tests.rs:65` — `book`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/tests.rs`.

The language-demand diagnostic
`00_nucleo/diagnosticos/infra-performance-followup/language-control.json` and
pinned buffer setup at
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:980` motivate
language-sensitive regression controls. The versioned Noto Sans fixture contains
ru/sr substitutions with unequal advances, allowing width and glyph paths to be
tested without optional system fonts or fixture generation.

## Owned contract

- Own only the unit suite, mocks and real-font fixtures under shaper::tests. Preserve existing names and descendant helper access; tests do not define a new API.
- Cover text passthrough, all page layers/semantic groups, bidi/UTF-8 offsets/multiline paragraphs, script/font fallback, sparse local slot identity, variations without contamination, tracking, math TextItem/ssty and LTR/RTL reconciliation.
- Require explicit-language measurement and emission to match configured same-face controls, preserving original text/clusters. Cover Russian/Serbian localized substitutions with the versioned Noto Sans fixture; independently assert distinct advances, fresh results, shared AB/BA/repeated runs and absent-language behavior. These are internal render/cache controls, not a claim of full upstream language parity.
- Distinguish dense-local candidate identity from global slot number; cardinality alone is insufficient. Cover scoring with sparse global slots and decoy metadata at the colliding local indices, and script boundaries within one face (including leading Common and inherited characters). Verify isolated versus mixed AB/BA shaping for tracking precision and absolute tracking at different sizes; retain same-effective-increment reuse controls. Mandatory new regressions use embedded/repository fixtures rather than silently skipping missing system fonts. Keep missing-font and no-panic controls and compare language text/positions where relevant rather than claim a new upstream parity measurement.

## Verification and boundary

Run `cargo test -p typst-infra --lib shaper::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Identidade textual — controles do contrato aprovado

Medição anterior: initial.json, before/resultados.json e red.log em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/` registram
texto/caixas e a perda de SHY. O dono confirmou o contrato
`00_nucleo/prompts/entities/layout_types.md`. A compilação de fixtures exaustivas
exige tratar a variante nova. Projeções visuais descem nos filhos; extractors
textuais usam text uma vez. Expectativas de identidade discricionária passam a
SHY, mantendo U+002D literal. Não relaxar caixas/seleção. A suíte de shaper
exercita fixture Noto obrigatória e ausência de fontes, conserva envelope e
prova que text substituto não altera glyphs/avanços nem contamina o cache.

## SHY autoral não selecionado — correção de identidade

### Medição anterior à decisão

`00_nucleo/diagnosticos/shy-autoral-2026-09-28/initial.json` e
`before/resultados.json` registram estado, binários, comandos e trinta sondas
bilaterais. Em largura 300pt, repre<U+00AD>sentation extrai dois SHY no Tekt
e um no vanilla pinado. O controle spaces extrai seis versus um.
`03_infra/src/shaper/shaping.rs:313-325` transporta o cluster SHY produzido
pelo rustybuzz; `03_infra/src/export/fonts.rs:318-328` associa sua substring
ao GID de espaço. `export/builder/fonts.rs:523-535` conserva uma identidade
por GID; a stream do controle anterior associa CID 0001 a 00AD e o reutiliza
no espaço posterior. São observáveis textuais, não igualdade de bytes.
A hipótese seria refutada por ausência dessa associação ou por duplicação
já existente na string que entra no shaper. Não inferir intenção histórica.

### Decisão e limite

Correção de paridade interna em fluxo contínuo ADR-0127: reutiliza contrato
TextReplacement aprovado, sem campo, assinatura pública, flag ou mudança
de fase. Não modifica seleção de quebras ou desenho. A causa explícita é
o U+00AD existente no texto autoral, não aparência, GID ou posição.
Casos estreitos já têm divergências de seleção e não fecham a política SHY.

Com fixture Noto obrigatória, testar SHY autoral por ocorrência, preservando
glifos/avanços/posições em relação ao shaper bruto, incluindo tracking.
Conferir que envelope existente não cria substituição aninhada e fonte
ausente conserva Text. Não substituir identidade por contagem de glifos.

## SHY autoral — controle de glifos invisíveis

### Medição anterior à decisão

`00_nucleo/diagnosticos/shy-autoral-2026-09-28/first-round/after/`
refuta a primeira materialização: trailing e ligature conservam os codepoints
mas ampliam a caixa de ActualText em 3.12pt e perdem o espaço na extração comum.
O caso tracking também inclui um intervalo derivado do placeholder invisível.
`first-round/final-receipt.json` conserva o estado e as falhas.
Rustybuzz 0.20.1, src/lib.rs:32-35, documenta que o tratamento padrão troca
Default_Ignorable por glifo de espaço com avanço zero; REMOVE_DEFAULT_IGNORABLES
remove esse placeholder depois do shaping. Um cluster compartilhado de f+SHY+fi
também pode dar hex vazio ao GID de espaço, bloqueando seu mapeamento posterior.
A decisão é refutável por alteração de glyphs visíveis/ligaduras no controle
com a flag documentada ou por contaminação de runs sem SHY.

### Responsabilidade

Configurar REMOVE_DEFAULT_IGNORABLES somente nos sub-runs cujo texto autoral
contém U+00AD, em medição e emissão. Delegar a configuração a text_identity,
antes de shape; não remover SHY do texto de entrada, não filtrar por GID
ou avanço zero e não alterar shaping de runs sem SHY. Os caracteres lógicos
completos continuam na ocorrência. A flag remove placeholders, não glyphs
visíveis de seleção de quebra. Tracking incide nos clusters que restam e
não acrescenta intervalo por um placeholder que não existe mais.
Cache permanece por texto completo; a política é determinística pelo texto.
Não mudar assinatura pública, fase, fallback, features OpenType ou fontes.
Texto exclusivamente invisível e seleção de quebras continuam fora do escopo.

## Regressão da cadeia comum — 2026-09-29

O controle deve provar que a família explicitamente declarada vence quando cobre o texto, e que Libertinus Serif cobre hebraico ausente da primária antes do fallback global para Noto Sans Hebrew. Usar o corpus PDF bilateral de `correcao-rtl-restante-2026-09-29` para validar posição, texto e face no pin vanilla; uma prova isolada de nomes não fecha a paridade.
