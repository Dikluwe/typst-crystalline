# L0 — stdlib/loading/encoding
Hash do Código: 7aeb5e1b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:923f1c9e4c2fd5934316ae2eef3dbb461c3bef3009de1d8931d951e793dda342
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:f29e26ff07053f940cfe4ab5358fc9a6ac8d4d689528e169b5f8567492f52f46

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/loading/encoding.rs`.
No HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` e árvore limpa, `01_core/src/compiler/stdlib/loading.rs:40-646; encoders em 360-645` contém o comportamento correspondente. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. É separação interna conforme ADR-0109/0127/0129; não há novo contrato público ou desempenho medido.

## Contrato

Compartilhar a projeção textual de Content/LocatedContent com func e campos públicos ordenados, a leitura causal de Args para encoders e a normalização comum do token numérico. Snapshot presente é autoritativo; ausente usa somente a projeção legada contratada. A unidade não emite formato final nem executa callbacks.

As regras detalhadas e evidências históricas do comportamento permanecem em `00_nucleo/prompts/compiler/stdlib/loading.md`; esta unidade as herda sem alterar paridade nem fases do pipeline. O consumer opera em L1, recebe dados/World por argumento e não introduz I/O direto ou estado global mutável.

## Aceitação

Preservar o resultado linguístico e os diagnósticos existentes. Compilar, passar a suíte L1 e `crystalline-lint .` com ownership e hashes válidos.


## Transparência na projeção estrutural — 2026-10-14

### Medição antes da decisão

content-serialization-2026-10-14/before/results.json.gz registra HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
status/diffstat, UTC, fontes/owners, comandos e binários pinados. json.encode
e yaml.encode perdem children de Sequence, child/styles de Styled e payload
de Raw quando recebem Source. encoding.rs:24–109 só reconhece variantes
na raiz; elem_name/getters atravessam wrappers, mas o match de campos não.
A CLI mede corretamente Source sequence/styled, embora possua lacunas próprias.

### Decisão e responsabilidade exclusiva

Antes da projeção estrutural, atravessar Source/ShowRecipe/Located por body
e Realized por visual, sem executar callbacks, copiar defaults ou modificar
ocorrências. Usar a variante pública alcançada para o nome de função e o
fallback contratado. Snapshot Some permanece autoritativo, inclusive vazio;
não o substituir pelo fallback nem inventar label/location. O núcleo
content-source-origin passa a ser pinado explicitamente por este owner.

Preservar os campos estruturais já contratados para Empty/Sequence/Styled/
Raw/Metadata/Link/Box/Block/HTML. Correção interna ADR-0127; nenhuma API, flag
ou fase nova. Testes RED de Source e wrappers compostos, snapshots Some/None,
encoders reais JSON/YAML e nomes canônicos. Raw block e lang:none continuam
incompletos por falta de presença no modelo, a completar em raw-presenca;
não inferir esse dado nem certificar serialização universal de Content.

## Presença lexical Raw — ratificada em 2026-10-14

### Medição anterior à decisão

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes/binários e pin 9f2b6e8715237cb086899a42873660fe744622e8. RawElem raw.rs:21–25 perde omissão; native_raw markup.rs:78–136 colapsa ausência/false/none. Campos, igualdade e encoders distinguem esses valores no pin. Trata-se de semântica/morfologia, não mecânica Rust.

### Responsabilidade e aceitação

Projeção textual Raw usa getter para text,block,lang conhecidos nessa ordem. Snapshot Some permanece exato, inclusive vazio; Source transparente.

Manifesto aprovado b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45, recibo em lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json. Esta seção completa apenas a presença anteriormente incompleta de Raw e supersede as cláusulas de ausência de máscara. Preservar contratos alheios; sem fase/default/render novo. Validar testes RED→GREEN, suíte, build e linhagem.

## Completar projeção da caption existente — 2026-10-17

Medição anterior à decisão: caption-shape-before.json e caption-after.json em
lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17 fixam fontes, UTC,
binários, HEAD/working tree de validation-result.json e pin
9f2b6e8715237cb086899a42873660fe744622e8. Projeção de caption já produz o
carrier, mas sua CLI imprime só func:figure.caption; body/getters/fields estão
ausentes. json.encode tem o mesmo defeito. Metadata de kind rejeita a função
de caption válida. O pin em model/figure.rs:510 declara caption Locatable e
em :511-594 declara position,separator,body e kind/supplement/numbering/counter
sintetizados públicos, com figure_location interno. Não sintetizar esses dados
nesta correção; a função de elemento já existe desde a aprovação P1473.

Responsabilidade exclusiva e aceitação:

Na projeção textual, mapear somente o nome interno FigureCaption para func
público caption. Usar os campos conhecidos fornecidos pelos owners de getters/
fields; Source transparente e snapshot Some autoritativo, inclusive vazio,
continuam exatos. Não materializar defaults/síntese de Figure ao serializar
uma FigureCaption solta. Não mudar formatos nem normalização numérica.

Correção interna/paridade em fluxo contínuo ADR-0127. Não há novo campo/trait/
assinatura Rust pública, fase, modo ou default. L0 antes de testes/código, RED
real de getters/metadados/encoder/CLI, GREEN e corpus externo sem relaxar
expectativas de corpo/nome. Revalidar todos os gates da etapa sob 12 GiB, swap
zero, jobs1. Resselo só dos owners após V15/V26. FG1/SY1 permanecem incompletos
conforme follow-ups.md; nenhuma API pendente está autorizada por esta seção.
