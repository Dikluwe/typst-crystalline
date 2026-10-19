# Diagnóstico — revisita de P1469 com veraPDF (2026-09-21)

**Não é um Prompt L0.** Documento de medição — ver `CLAUDE.md` (organização de
`00_nucleo/`). Motivação: `typst-passo-1469.md` §7.1 reduziu deliberadamente o
escopo de `format.pdf.standard`/`--pdf-standard` a "contrato de linguagem"
porque não havia validador de conformidade PDF/A/PDF/UA disponível no
ambiente. O veraPDF 1.30.2 foi instalado nesta sessão (ver memória
`reference_verapdf_installed`) especificamente para reabrir esse eixo. Este
documento mede o estado real antes de qualquer decisão de escopo para um novo
passo (ADR-0108).

## Proveniência

- Commit da árvore no momento da medição: `fe5e737c6` (working tree limpo,
  `git status --short` sem alterações relativas a P1469 nesta altura).
- Binários usados: vanilla ratificado `/usr/local/bin/typst` (`typst 0.15.1
  (586e1bd4)`) e cristalino `target/debug/typst` (`typst 0.15.1 (e14002d5)` —
  build de working tree sobre `fe5e737c6`, ver nota abaixo sobre o binário
  `release/typst` estar desatualizado/não usado nesta medição).
- Validador: `verapdf 1.30.2` (`~/.local/bin/verapdf`), perfis nativos
  (`-f <flavour>`), sem opções de reparo automático.
- Documento de teste: `#set page(width: 12cm, height: 8cm)` com heading,
  negrito, itálico, `#figure` com `rect` + `caption` (e `alt:` + `#set
  document(title: ...)` na segunda rodada, para os perfis que exigem
  acessibilidade). Ficheiros e relatórios JSON completos ficaram no
  scratchpad da sessão (não persistidos no repo — reproduzível a partir deste
  documento).

## Achado 0 — `release/typst` estava desatualizado

A primeira tentativa usou `target/release/typst`, compilado antes de P1469
(20/09), que ainda tinha o campo antigo `_pdf_standard: Option<String>`
oculto/descartado (decisão pré-retificação de P1286). Isso explica ter
aceitado nomes inválidos (`1b`, `totally-bogus-xyz`) silenciosamente com exit
0. `target/debug/typst` (build de working tree corrente) valida corretamente,
com a mesma mensagem `possible values: ...` do vanilla. **Não é uma
regressão** — é um binário obsoleto no diretório errado. Qualquer medição
futura deve confirmar o binário com `--version` antes de usar.

## Achado 1 — padrão de falhas veraPDF é muito mais estreito do que P1469 temia

`typst-passo-1469.md` §7.1 citava `krilla::configure::validate.rs` (1237
linhas) como evidência de que a conformidade real estava "espalhada pelo
resto da crate". A medição real com veraPDF mostra que, para os seis perfis
não-acessíveis medidos, o cristalino já passa a esmagadora maioria das
regras e falha por um conjunto **pequeno e quase idêntico** em cada um:

| Perfil | Regras passadas | Regras falhadas | Checks falhados |
|---|---|---|---|
| a-1b | 124 | 5 | 37 |
| a-2b | 141 | 3 | 4 |
| a-2u | 143 | 3 | 4 |
| a-3b | 143 | 3 | 4 |
| a-3u | 145 | 3 | 4 |
| a-4  | 104 | 5 | 6 |

Perfis "acessíveis" (com `alt:` na figura e `#set document(title: "...")`
para satisfazer os erros de compilação do vanilla — ver Achado 3) mostram o
mesmo padrão nuclear, sem adicionar falhas estruturais de acessibilidade
além dele:

| Perfil | Regras passadas | Regras falhadas | Checks falhados |
|---|---|---|---|
| a-1a | 130 | 5 | 37 |
| a-2a | 150 | 3 | 4 |
| a-3a | 152 | 3 | 4 |
| ua-1 | 100 | 6 | 26 |

### As 3 regras nucleares (a-2/a-3, todas as variantes b/u/a)

1. **Trailer sem `/ID`** (ISO 19005-*, cláusula 6.1.3 / 6.6.4 conforme
   versão). `03_infra/src/export/builder.rs:3352` emite
   `trailer\n<< /Size {} /Root 1 0 R{} >>...` sem entrada `/ID`. Nunca existiu
   — não é regressão de P1469.
2. **XMP sem "PDF/A Identification Schema"** (`pdfaid:part`/
   `pdfaid:conformance`, cláusula 6.6.4/6.7.11/6.2.4.3 conforme versão).
   `emit_xmp_metadata` (`builder.rs:2703-2782`) já escreve um pacote XMP
   funcional (`dc`, `xmp`, `xmpMM`, `xmpTPg`, `pdf`) mas nunca inclui o
   namespace `pdfaid`. Também **não existia antes** de P1469 (`standard` é o
   primeiro eixo que dá sentido a declarar um perfil PDF/A na XMP).
3. **DeviceRGB sem OutputIntent RGB** (cláusula 6.2.3.3/6.2.4.3). Não existe
   nenhum `/OutputIntent` no catálogo. O vanilla usa (via krilla)
   `dest_output_profile` apontando para um perfil ICC sRGB estático
   embutido — `~/.cargo/git/checkouts/krilla-*/crates/krilla/icc/sRGB-v4.icc`
   (480 bytes) ou `sRGB-v2-magic.icc` (736 bytes) conforme a versão do PDF
   (`krilla/src/configure/version.rs:54-61`), aplicado sempre que o perfil é
   "Archival" (`krilla/src/configure/validate.rs:894-...` — todas as
   variantes A1-A4, nunca UA sozinho — confirma por que `ua-1` sozinho não
   lista esta falha).

### Achado extra — a-1 (5 regras) e a-4 (5 regras) têm exigências próprias

- **a-1** (a/b): além das 3 nucleares, falha **6.2.3.2** (perfis ICCBased
  devem ser embutidos como stream — mesma causa-raiz do OutputIntent, PDF/A-1
  tem regra própria mais estrita) e **6.3.5** (subconjuntos CIDFont precisam
  de entradas adicionais no font descriptor — não investigado a fundo nesta
  medição, precisa de leitura de `krilla`/comparação byte-a-byte de fontes).
- **a-4**: falha **6.1.3** duas vezes (chave `/Info` não deve estar presente
  no trailer de ficheiros PDF/A-4, e se presente só pode conter `ModDate`) —
  o cristalino emite `/Info` com `/Producer`/`/CreationDate`/etc.
  incondicionalmente; PDF/A-4 exige suprimir isso.

### PDF/UA-1 — gap estrutural muito mais profundo, eixo à parte

`ua-1` falha 6 regras / 26 checks, todas de natureza distinta das anteriores
(sem OutputIntent/trailer/XMP-pdfaid — usa identificação UA própria em vez):

- **7.1** "Content shall be marked as Artifact or tagged as real content" —
  completude da árvore de tags (algo do conteúdo emitido não está nem
  marcado `/Artifact` nem tagueado como conteúdo real).
- **7.2** (×3) "Natural language ... shall be determined" — falta `/Lang` no
  catálogo do documento, no conteúdo de página, e nas entradas de `/Outline`.
- **7.1** catálogo sem `/ViewerPreferences << /DisplayDocTitle true >>`.
- **5** identificação PDF/UA ausente na XMP (schema próprio, distinto do
  `pdfaid` de PDF/A).

Isto é substancialmente mais trabalho que as 3 regras nucleares de PDF/A —
toca `/Lang` em pelo menos três pontos distintos do pipeline (documento,
conteúdo, outline) e a lógica de tagueamento já existente (`PdfTags`,
P1140.6), não apenas `builder.rs`. Candidato a passo(s) separado(s).

## Achado 2 — XMP `pdf:PDFVersion` ficou dessincronizado do header dinâmico

`emit_xmp_metadata` (`builder.rs:2779`) ainda escreve
`<pdf:PDFVersion>1.7</pdf:PDFVersion>` como literal fixo, mesmo depois de
P1469 ter tornado o header `%PDF-{version}` dinâmico
(`self.standard.version.as_str()`, `builder.rs` §P1469). veraPDF não acusou
isto como falha de regra nas medições acima (não é requisito formal ISO
19005), mas é uma inconsistência interna real e de baixo custo para
corrigir — não depende de decisão de escopo, é candidato a fluxo contínuo
(ADR-0127: correção de fórmula interna).

## Achado 3 — divergência não relacionada: `document(title:)` não aceita `content`

Ao montar o documento de teste para os perfis acessíveis, `#set
document(title: [Teste de conformidade])` (content) compilou no vanilla mas
falhou no cristalino com `error: expected string, found content`. Não
investigado a fundo (fora do escopo desta revisita) — registado aqui apenas
para não se perder; pode já ser um gap conhecido de outro passo. Contornado
nesta medição usando `title: "Teste de conformidade"` (string).

## Não coberto por esta medição

- PDF/A-4f/PDF/A-4e (exigem ficheiro anexado — não testado).
- PDF/UA-2 (não pedido por P1469; fora do conjunto de 17 nomes do cast).
- WCAG/wt1r/wt1a (perfis veraPDF não relacionados a PDF/A ou PDF/UA).
- Qualquer medição de bytes/estrutura mais fina do que o relatório JSON do
  veraPDF (não se leu manualmente a árvore de objetos além do que os
  `errorMessage`/`context` do relatório já apontam).

## Achado 4 — medição aprofundada dos bytes exatos (2026-09-21, mesma sessão)

Decisão do dono após apresentação do Achado 1: **aprofundar a medição antes
de decidir implementar** — nada abaixo é L0 nem código, só leitura de bytes
reais via `qpdf --show-object`/inspeção Python do PDF gerado pelo vanilla, e
grep no código-fonte cristalino já existente.

### 4.1 Trailer `/ID` — vanilla reutiliza o `InstanceID`/`DocumentID` da XMP

Medido em `vanilla_a2b.pdf` (qpdf `--show-object=trailer`):

```text
<< /ID [ (zeUzczLP8RoCxLAjPUvuuA==) (zeUzczLP8RoCxLAjPUvuuA==) ]
   /Info 47 0 R /Root 49 0 R /Size 50 >>
```

O mesmo literal `zeUzczLP8RoCxLAjPUvuuA==` aparece em `xmpMM:InstanceID` e
`xmpMM:DocumentID` na XMP — os três valores são **idênticos**, e ambas as
posições do array `/ID` também são idênticas entre si (primeira geração do
ficheiro, sem updates incrementais). Isto **não é** um hash MD5 do conteúdo
— é o mesmo texto base64-de-16-bytes que a XMP já usa. O cristalino **já
tem** essa exata primitiva: `xmp_instance_and_document_id`/
`random_xmp_id_bytes`/`base64_encode_16` (`builder.rs:372-424`, usado por
`emit_xmp_metadata`). Implementar o trailer `/ID` seria **reutilizar o
mesmo valor já calculado**, sem nova lógica de hashing.

### 4.2 XMP `pdfaid` — só os 2 elementos nucleares são exigidos pelo veraPDF

Medido em `vanilla_a2b.pdf`: `<pdfaid:part>2</pdfaid:part>
<pdfaid:conformance>B</pdfaid:conformance>` dentro do namespace
`xmlns:pdfaid="http://www.aiim.org/pdfa/ns/id/"`. O vanilla também emite um
bloco `<pdfaExtension:schemas>...</pdfaExtension:schemas>` elaborado
(descrevendo `xmpMM:InstanceID`, `pdf:Keywords/PDFVersion/Producer/Trapped`
como propriedades de schema não-nuclear) e um `<xmpMM:History>` com eventos
`saved`/`converted`. **Nenhum dos dois apareceu nas regras falhadas do
cristalino** (que já não tem nenhum dos dois) — confirma que **não são
exigidos** pelo conjunto de regras veraPDF medido aqui; são mecânica própria
do `xmp-writer`/krilla, não contrato de linguagem (ADR-0107: paridade é com
os elementos que o validador realmente exige, não com a forma XML do
vanilla). Implementar só `pdfaid:part`/`pdfaid:conformance` (2 elementos,
mapeados diretamente de `PdfArchival`/`PdfAccessibility` já existentes em
`document_format_options.rs`) é suficiente para as regras 6.6.4/6.7.11/6.7.3
medidas.

### 4.3 `/OutputIntent` — estrutura exata e reuso do ICC já vendorizado

Medido em `vanilla_a2b.pdf` (objetos 5 e 46 via `qpdf --show-object`):

```text
5 0 obj << /Type /OutputIntent /S /GTS_PDFA1
           /OutputConditionIdentifier (Custom) /OutputCondition (sRGB)
           /RegistryName () /Info (sRGB v4.2)
           /DestOutputProfile 46 0 R >>
6 0 obj [5 0 R]                        % catálogo: /OutputIntents 6 0 R
46 0 obj << /N 3 /Range [0 1 0 1 0 1] /Filter /FlateDecode >>
           stream ... endstream        % ICC RGB, 480 bytes brutos (v4.2)
```

`/S /GTS_PDFA1` é o mesmo subtype em **todas** as versões de PDF/A medidas
(1b e 2b) — não varia por versão do perfil, é o identificador GTS fixo do
tipo "Archival" (confirmado também em
`krilla/src/configure/validate.rs:894` — `output_intent()` só depende de
ser `Archival`, nunca de qual nível).

**Achado-chave:** o cristalino **já vendoriza exatamente este perfil ICC
sRGB v4.2 de 480 bytes** — `SRGB_ICC_PROFILE` em
`03_infra/src/export/images.rs:39-65` (P777, hoje usado para XObjects de
imagem RGB e para o colour space `/c0` de fills/strokes desde P956). O
`/DestOutputProfile` do `/OutputIntent` para PDF 1.7/2.0 (versão default de
a-2/a-3/a-4) pode referenciar **o mesmo stream já embutido** — via
`build_icc_profile_stream`/`srgb_icc_profile_bytes()`, já `pub(super)` em
`images.rs` e já importado por `builder.rs:131-141`. **Zero bytes novos
vendorizados são necessários para a-2/a-3/a-4.**

### 4.4 a-1 é o único perfil que precisa de um segundo perfil ICC (v2, 736 bytes)

O `cristalino_a1b.json` mostra a regra **6.2.3.2** a falhar com: "The
embedded ICC profile (Device Class = mntr, color space = RGB, version =
4.2) is either invalid or does not satisfy PDF 1.4 requirements" — ou seja,
o `SRGB_ICC_PROFILE` (v4.2, P777) **já está a ser embutido e usado** nesse
PDF (para o fill do `rect` azul), mas o perfil ICC v4 não é válido em
combinação com PDF 1.4 (a versão default de a-1). O vanilla, para a-1b, usa
um perfil **diferente**: `/Info (sRGB v2.1)`, 736 bytes brutos — mesmo
`sRGB-v2-magic.icc` que `krilla/src/configure/version.rs:56-58` seleciona
para `PdfVersion::Pdf14/15/16`. Implementar a-1 corretamente exigiria (a)
vendorizar esse segundo perfil v2, e (b) selecionar entre os dois conforme
`self.standard.version` — um eixo de complexidade adicional que a-2/a-3/a-4
não têm (esses usam PDF 1.7/2.0, já cobertos pelo perfil v4 existente).

### 4.5 a-1 também exige `/CIDSet` por subset de fonte (mais trabalhoso)

Regra **6.3.5** falhou 3 vezes em `cristalino_a1b.json`, uma por cada
`DescendantFonts` de subset (`AAAAAA+LibertinusSerif-Bold`,
`-Regular`, …): "A CIDSet entry in the Font descriptor is missing or does
not correctly identify all glyphs present in the embedded font subset".
Isto exige, por fonte subsetada emitida, um stream `/CIDSet` no font
descriptor com um bitset (1 bit por CID, `bit=1` se o glyph está presente)
— não medido ao nível de bytes exatos nesta rodada (ficaria para
implementação, não mediação); é claramente mais trabalho do que 4.1-4.3 e
toca o código de emissão de fontes (`fonts.rs`), não só `builder.rs`.

### 4.6 a-4: vanilla omite `/Info` inteiramente (não só reduz a `ModDate`)

Medido em `vanilla_a4.pdf` — trailer é `<</Size 48/Root 47 0 R/ID[...]>>`,
**sem** chave `/Info` nenhuma (nem sequer `/ModDate`). O cristalino hoje
emite `/Info` incondicionalmente com `/Producer/CreationDate/Title/Creator`
— confirmado pelo veraPDF: "Document information dictionary is present and
contains CreationDate,Title,ModDate,Creator key(s), but allowed only
ModDate", e ainda uma segunda regra exigindo `/PieceInfo` no catálogo **se**
`/Info` estiver presente. **A correção mais simples e igual à do vanilla é
suprimir `/Info` inteiramente quando `standard` inclui `PdfArchival::A4`**
— não construir uma versão reduzida dele. Puramente condicional na emissão
do trailer existente, mesmo padrão dos outros eixos de `standard`.

## Conclusão desta medição (sem decisão de escopo — ver próximo passo)

O gap para tornar PDF/A-2b/2u/2a/3b/3u/3a **totalmente conformes** (veraPDF
PASS) é pequeno e mecânico: `/ID` no trailer, schema `pdfaid` na XMP, e um
`/OutputIntent` com um perfil ICC sRGB estático (~500 bytes, mesmo material
que o vanilla usa, disponível para vendorizar). PDF/A-1 soma 2 regras
extra relacionadas a embedding de ICC/CIDFont; PDF/A-4 soma a supressão do
`/Info`. PDF/UA-1 é um esforço à parte, maior, tocando `/Lang` e a árvore de
tags em múltiplos pontos do pipeline. Esta medição não decide qual subset
implementar nem em quantos passos — isso é decisão do dono, informada por
este documento.
