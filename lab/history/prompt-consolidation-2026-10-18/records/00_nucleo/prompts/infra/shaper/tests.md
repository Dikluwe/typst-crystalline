# L0 — infra/shaper/tests
Hash do Código: 94ec9918

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee



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
`lab/results/construction/diagnostics/infra-performance-followup/language-control.json` and
pinned buffer setup at
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:980` motivate
language-sensitive regression controls. The versioned Noto Sans fixture contains
ru/sr substitutions with unequal advances, allowing width and glyph paths to be
tested without optional system fonts or fixture generation.

## Owned contract

- Own only the unit suite, mocks and real-font fixtures under shaper::tests. Preserve existing names and descendant helper access; tests do not define a new API.
- Cover text passthrough, all page layers/semantic groups, bidi/UTF-8 offsets/multiline paragraphs, script/font fallback, sparse local slot identity, variations without contamination, tracking, math TextItem/ssty and LTR/RTL reconciliation.
- Compare the existing fresh-metrics reconciliation entry point with the internal borrowed-metrics path on the same two-item document and World; their resulting x positions must agree.
- Require explicit-language measurement and emission to match configured same-face controls, preserving original text/clusters. Cover Russian/Serbian localized substitutions with the versioned Noto Sans fixture; independently assert distinct advances, fresh results, shared AB/BA/repeated runs and absent-language behavior. These are internal render/cache controls, not a claim of full upstream language parity.
- Distinguish dense-local candidate identity from global slot number; cardinality alone is insufficient. Cover scoring with sparse global slots and decoy metadata at the colliding local indices, and script boundaries within one face (including leading Common and inherited characters). Verify isolated versus mixed AB/BA shaping for tracking precision and absolute tracking at different sizes; retain same-effective-increment reuse controls. Mandatory new regressions use embedded/repository fixtures rather than silently skipping missing system fonts. Keep missing-font and no-panic controls and compare language text/positions where relevant rather than claim a new upstream parity measurement.

## Verification and boundary

Run `cargo test -p typst-infra --lib shaper::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Identidade textual — controles do contrato aprovado

Medição anterior: initial.json, before/resultados.json e red.log em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/` registram
texto/caixas e a perda de SHY. O dono confirmou o contrato
`00_nucleo/prompts/core/entities/layout_types.md`. A compilação de fixtures exaustivas
exige tratar a variante nova. Projeções visuais descem nos filhos; extractors
textuais usam text uma vez. Expectativas de identidade discricionária passam a
SHY, mantendo U+002D literal. Não relaxar caixas/seleção. A suíte de shaper
exercita fixture Noto obrigatória e ausência de fontes, conserva envelope e
prova que text substituto não altera glyphs/avanços nem contamina o cache.

## SHY autoral não selecionado — correção de identidade

### Medição anterior à decisão

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/initial.json` e
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

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/first-round/after/`
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

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


## Contagem de carga no carrier memoizado — 2026-10-05

Medição anterior: shaper/shaping.rs volta a chamar World::font ao emitir miss/hit, embora já detenha a fonte do shaping. O owner `00_nucleo/prompts/infra/shaper/cache.md` agora especifica Font no CachedRun privado. Testar fonte regular de fallback sob pedido italic repetido e contar consultas à fonte regular: o carrier Some deve continuar correto, sem cargas adicionais para sua emissão depois de face/shape frios. Limite de consultas não prescreve algoritmo mais restrito; uma otimização que reduza ainda mais continua válida.

## Controles bidi e resultado vazio válido (2026-10-06)

Medição anterior à decisão: controles fonte→PDF da primeira rodada em
`lab/results/construction/diagnostics/controles-bidi-pdf-2026-10-06/first-verification.json.gz`
conservam RLM residual após corrigir espaços. shaping.rs distingue atualmente
vazio/falha apenas pela ausência de items; traversal conserva fallback Text.
Verificar controle isolado com fonte versionada e cache miss/hit, sem item
Text residual, envelope inventado ou avanço. Fontes ausentes continuam com
fallback existente. Letras adjacentes não devem desaparecer. Correção interna,
sem contrato público/fase/unsafe. Este owner possui apenas os testes existentes
e este teste de regressão; invariantes compartilhadas continuam nos owners.

Completar a prova do conjunto de seleção de fontes: todos os controles bidi
explícitos dispensam cobertura; U+200B, ZWJ, ZWNJ e variation selectors não
entram por generalização. Sondas PDF certificam contexto integral e tinta.

## Identidade bidi por ocorrência — clusters certificados (2026-10-06)

### Medição anterior à decisão

`lab/results/construction/diagnostics/bidi-identidade-ocorrencia-2026-10-06/before.json`
registra HEAD, working tree, código/L0, binários e sonda rustybuzz. Com a face
DejaVu versionada por SHA e REMOVE_DEFAULT_IGNORABLES, LRM+ABC e PDI+ABC
produzem A no cluster byte 0, B no byte 4 e C no byte 5. A substring 0..4
contém controle+A. O ToUnicode visual em fonts.rs:320-347 remove controles;
text_identity.rs:15 só certifica SHY. O lote anterior verification.json.gz
em controles-bidi-pdf-2026-10-06 mede falta de U+200E/U+2069 nessas três
ocorrências, com glifos preservados. O vanilla pinado 9f2b6e871, inline/
shaping.rs:1052-1099, fornece ranges completos dos clusters à exportação.
Semântica textual/Unicode e morfologia são o alvo; GIDs/bytes/algoritmo não.
Hipótese refutável por identidade ainda ausente, duplicação em outra ocorrência
da mesma letra ou alteração da tinta após envelope certificado. Não inferir
intenção histórica.

### Classificação

Correção interna de paridade ADR-0127: usa TextReplacement aprovado, sem
campo/método/assinatura pública, fase, modo, flag, dependência ou unsafe novo.
Certificado provém dos ranges UTF-8 do shaping bem-sucedido, não de GID,
avanço, posição, aparência, fonte ou mera presença de controle no run.
Os invariantes do envelope pertencem ao núcleo text-replacement já pinado.

### Responsabilidade exclusiva

Teste causal com fonte Noto obrigatória: LRM/PDI unido a letra em range real
conserva text por ocorrência e um filho original; letras comuns repetidas não
adquirem controles. Miss/hit preservam glifos/avanços/posição/fonte/origem.
TextReplacement existente não recebe envelope filho novo; SHY mantém seus
controles existentes. A aceitação fonte→PDF usa casos repetidos e wrappers.

## Fixtures ativos e resíduos de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/shaper/tests.rs:113` calcula self.fonts.len() sem uso; `:520` cria FontBook mutável sem uso antes de construir o SystemWorld realmente consultado pelo teste.

Fontes, owners, diagnósticos, HEAD, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/fixtures-ativos-ci-2026-10-13/before.json.gz`. O registro real da suíte antes da edição confirma ausência do teste matemático. Estas evidências não constituem nova comparação com upstream.

### Decisão e aceitação

Retirar esses dois locals puros sem consumidor, preservando construção do World, ordem de registro de fontes, condições de fixtures e todas as asserções. Não criar supressão de lint nem mudar o comportamento produtivo. Limitações preexistentes de testes condicionais a fontes do sistema continuam explícitas e não são fechadas por esta limpeza.

Manutenção interna de testes (ADR-0127), sem assinatura pública, default ou fase nova. L0 precede código; resselo limitado aos owners envolvidos. Executar suítes shell e infra, compilar todos os targets, verificar formato, linhagem e inventário de Clippy. Contagens de testes não certificam paridade.


## Expressões equivalentes e CI estrito — 2026-10-14

### Medição anterior à decisão

`03_infra/src/shaper/tests.rs` nas linhas [1358, 1359, 1494] contém as expressões apontadas pelo inventário
Clippy. `lab/results/construction/diagnostics/ci-simplificacoes-2026-10-14/before.json.gz`
conserva fonte/owner, spans, HEAD, working tree não commitado, UTC e diffstat;
red.log registra os lints selecionados como erros antes de alterar código.
Os tipos e usos foram conferidos no consumer e no toolchain 1.92.0 pinado.
Esta é manutenção da expressão Rust; não é medição nova de paridade upstream.

### Responsabilidade e aceitação

Nos controles isolados de tracking, usar slices emprestadas unitárias por std::slice::from_ref para os estilos a/b. Conservar controles AB/BA/repetidos, ajustes de tamanho/tracking, clones necessários aos arrays de múltiplos estilos, fixture obrigatório e todas as asserções de glyphs/posições.

Correção interna em fluxo contínuo ADR-0127, sem API pública, default, fase,
unsafe, dependência ou supressão de lint novos. L0 precede código e resselo
fica restrito ao owner. Usar os controles funcionais existentes; não alterar
fixtures, expectativas, tolerâncias ou registrar testes que apenas espelhem
esta simplificação. Os lints selecionados devem passar como erros, seguidos
de suíte completa, build, all-targets, rustdoc, fmt, linhagem e inventário total.

## Referencial de pintura — transporte aprovado T1b — 2026-10-14

### Medição anterior à decisão

O manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
foi aprovado pelo humano para T1–T3; recibo em
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A evidência bilateral de Parent e controles Solid está no contrato aprovado;
ela não é substituída por inspeção estrutural. A rodada
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` conserva
fontes, estado sujo, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC,
diffstat e hashes dos binários; cobre contêineres, Place e Rotate com Solid
e Gradient Parent. O inventário before-source/syntax-nodes registra neste
consumer as construções/desestruturações antes da edição: [569].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Migrar construções e padrões de Group para o campo paint_root aprovado. Grupos técnicos novos e fixtures legadas são false; reconstruções transportam o valor recebido, sem inferir raiz a partir de dimensão, clipping ou transformação. Preservar os demais campos e o comportamento de desenho/recursos. Exportadores ainda não usam este metadado para patterns; essa integração é T2 e não está certificada aqui.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.


## Inicialização explícita dos fixtures — field_reassign_with_default

### Medição anterior à decisão

`03_infra/src/shaper/tests.rs` nas linhas 174, 291, 454, 627, 1126 cria defaults e em seguida sobrescreve campos locais de testes. A fonte, os spans e os L0 anteriores estão em `lab/results/construction/diagnostics/field-reassign-2026-10-15/before.json.gz`, com HEAD, working tree, UTC e diffstat; initial-diagnostics.json conserva os inicializadores apontados. Esta manutenção trata representação Rust, sem nova medição de paridade ou mudança no default do produto.

### Responsabilidade e aceitação

Inicializar os campos contíguos font/size/weight nos cinco estilos locais/helpers de teste por literal TextStyle. Preservar nomes de famílias, tamanhos, peso recebido, clones e mutação posterior de tracking, guardas de fontes do sistema, documentos e todas as asserções de glifos/posições.

Usar `..Tipo::default()` para os campos restantes e retirar `mut` apenas onde deixam de existir mutações. Sem novos tipos, builders, APIs ou supressões de lint. L0 precede código e resselo restrito (fluxo contínuo ADR-0127). O gate selecionado deve falhar antes e passar depois; preservar nomes, inputs, expectativas e tolerâncias dos testes. Revalidar Clippy all-targets, suíte workspace, build, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe do inicializador.
