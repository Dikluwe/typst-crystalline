# L0 — infra/shaper/text_identity
Hash do Código: 437903bb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/bidi-visual-identity.toml sha256:3c6209895df993843d15259c7ee95195e7f0dc732ee56cf3cc553257e30298b0
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/text_identity.rs`

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

## Responsabilidade exclusiva

Own private protect_authored_identity(Vec<FrameItem>) -> Vec<FrameItem>. Para
cada TextShaped com texto visual contendo U+00AD autoral, envolver o item
em TextReplacement com text clonado da ocorrência e um único filho original.
Outros itens ficam idênticos. Não resolver fontes, executar shape, medir, filtrar
glifos emitidos, transformar coordenadas ou percorrer wrappers.
Own também configure_authored_ignorables_buffer(&mut UnicodeBuffer, &str) privado,
que aplica a flag documentada conforme a cláusula abaixo. O caller só chama
essa unidade fora de envelope existente. U+00AD é a identidade normativa
do caractere, sem exceção de fixture, família, GID ou largura.

## Aceitação

Testes causais em shaper/tests usam fixture Noto versionada e comprovam
texto lógico único, glifos/avanços/posições preservados e não aninhamento.
Sondas fonte→PDF devem conservar SHY por ocorrência e espaços/hífens
literais, incluindo local/top e compacto/verboso. Seleção estreita e fallback
sem fonte não são certificados. Ownership 1:1; dependências não ampliam owner.

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

Configurar REMOVE_DEFAULT_IGNORABLES nos sub-runs cujo texto autoral
contém U+00AD ou um controle bidi especificado abaixo, em medição e emissão. Delegar a configuração a text_identity,
antes de shape; não remover SHY do texto de entrada, não filtrar por GID
ou avanço zero e não alterar shaping de runs sem SHY nem controles bidi. Os caracteres lógicos
completos continuam na ocorrência. A flag remove placeholders, não glyphs
visíveis de seleção de quebra. Tracking incide nos clusters que restam e
não acrescenta intervalo por um placeholder que não existe mais.
Cache permanece por texto completo; a política é determinística pelo texto.
Não mudar assinatura pública, fase, fallback, features OpenType ou fontes.
Texto exclusivamente invisível e seleção de quebras continuam fora do escopo.

## Controles bidi autorais — identidade de espaço (2026-10-06)

### Medição anterior à decisão

`00_nucleo/diagnosticos/controles-bidi-pdf-2026-10-06/before.json` registra
HEAD, working tree, fontes de código/L0 e binário anterior. As sondas
`bidi-contexto-rtl_literal_embedding-cmap-before.json` e
`bidi-contexto-rtl_forced_mark-cmap-before.json` mostram CID 0001 associado
respectivamente a U+202C e U+200F: espaços posteriores extraem controles e
fundem palavras. A extração bilateral com fontes e comandos está no lote
`00_nucleo/diagnosticos/block-align-2026-10-06/verification.json.gz`.
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:992-997`, pin
9f2b6e8715237cb086899a42873660fe744622e8, remove placeholders Default_Ignorable
após shaping. `03_infra/src/export/fonts.rs:307-328` mantém controles na
substring global por GID; `03_infra/src/export/builder/fonts.rs:523-535`
retém o primeiro mapeamento. São observáveis de texto e geometria, não
paridade de bytes ou obrigação de copiar o algoritmo upstream.
Hipótese refutável por persistência da colisão após remover placeholders e
controles apenas do mapa visual. Não inferir intenção histórica.

### Decisão interna e limites

Correção interna de paridade ADR-0127, sem campo/assinatura pública, fase,
flag, default de produto ou unsafe novo. As invariantes compartilhadas pertencem ao núcleo bidi-visual-identity pinado acima. SHY conserva envelope ActualText existente por ocorrência.
Controles bidi isolados não ganham envelope ActualText: o vanilla não os copia
como identidade dos espaços nestas sondas. Clusters mistos obedecem ao
certificado por ocorrência especificado abaixo. Aceitação exige espaços/letras separados,
clusters Unicode preservados e controle visual; extratores podem inserir seus
próprios marcadores de direção, logo texto bruto não prova identidade autoral.

Este owner possui configure_authored_ignorables_buffer privado, ativando REMOVE_DEFAULT_IGNORABLES pelo texto autoral SHY/bidi, preservando flags prévias. protect_authored_identity mantém SHY e acrescenta somente clusters bidi certificados.

## Identidade bidi por ocorrência — clusters certificados (2026-10-06)

### Medição anterior à decisão

`00_nucleo/diagnosticos/bidi-identidade-ocorrencia-2026-10-06/before.json`
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

Renomear o helper privado para protect_authored_identity. SHY mantém a regra
existente. Para bidi, examinar ranges entre offsets de cluster únicos ordenados
e text.len(); ranges fora dos limites ou que cortem UTF-8 não certificam nada.
Certificar se um range coberto por glifo contém pelo menos um controle do
conjunto bidi pinado e pelo menos um codepoint que não pertence àquele
conjunto. Sem glifos, sem range válido ou controle apenas fora dos ranges
emitidos: não criar envelope. Não generalizar para ZWJ/ZWNJ/VS.
Com certificado, envolver o TextShaped inteiro em TextReplacement, clonando
exatamente text e conservando o item como único filho, com fonte/bidi/posição/
clusters/avanços idênticos. Não alterar ToUnicode global nem relayout/shape.
O caller conserva a guarda de envelope existente para impedir aninhamento.
