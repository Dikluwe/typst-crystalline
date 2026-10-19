# Passo 1477 — Núcleo PDF/A-1a/1b: segundo perfil ICC (v2) por versão, `/CIDSet`

**Estado:** MATERIALIZADO (confirmação humana explícita recebida
2026-09-22, na mesma sessão, posterior ao commit causal somente-L0
`b5a4836f8`; execução segregada A/B/C — ver §7)

**Data da auditoria:** 2026-09-22T11:54:06-03:00

**Baseline de redação:** `94415842349d4a999b181e8a435a13ada07cfc95`, tree
`e12c484ceecfeee1d5ab20949c640810b6cff5ec`, working tree limpo.

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

## 0. Origem e continuidade

Continuação direta do scope-out registado em `typst-passo-1475.md`
§2.5/§4.4/§4.5: A1a/A1b ficaram fora de P1475 porque exigem (a) um
segundo perfil ICC (v2, incompatível com o v4 já vendorizado em PDF
1.4) e (b) `/CIDSet` por fonte subsetada — "mais trabalho que 4.1-4.3",
"toca o código de emissão de fontes, não só `builder.rs`". Este passo
fecha os dois.

## 1. Objetivo

Tornar `--pdf-standard a-1a`/`a-1b` **totalmente conformes** (veraPDF
PASS). Dois itens, dois owners:

1. **Segundo perfil ICC (v2, 736 bytes)** + seleção por versão do PDF —
   owner `infra/export/images.md` (vendoriza o perfil, expõe getter) +
   `infra/export/builder.md` (consome, troca o perfil único P777 por
   seleção `PdfVersion → perfil`, usado em três lugares: `/c0` (P956),
   JPEGs RGB (P777), `/OutputIntent` (P1475/P1476)).
2. **`/CIDSet`** no `/FontDescriptor` de cada CIDFont embutida — owner
   `infra/export/builder.md` (`build_cidfont`/`build_multifont`).

## 2. Medição (2026-09-22, mesma sessão)

### 2.1 Regra de seleção do perfil ICC é por **versão do PDF**, não por archival — confirmado na fonte, não só por sintoma

`krilla/src/configure/version.rs:54-61` (checkout local
`~/.cargo/git/checkouts/krilla-dfa083154a81f6b2/7772dbe/`):

```rust
pub(crate) fn rgb_icc(&self) -> ICCProfile<3> {
    match self {
        PdfVersion::Pdf14 => SRGB_V2_ICC.clone(),
        PdfVersion::Pdf15 => SRGB_V2_ICC.clone(),
        PdfVersion::Pdf16 => SRGB_V2_ICC.clone(),
        PdfVersion::Pdf17 => SRGB_V4_ICC.clone(),
        PdfVersion::Pdf20 => SRGB_V4_ICC.clone(),
    }
}
```

Mapeamento direto para `entities/document_format_options.rs::PdfVersion`
(mesmos cinco valores, mesma ordem): `V1_4|V1_5|V1_6 → v2`,
`V1_7|V2_0 → v4`. Isto **não é específico de A1** — é a regra geral do
vanilla para qualquer documento cuja versão de PDF efetiva seja 1.4-1.6,
com ou sem `standard` archival. Como A1a/A1b são as únicas combinações
(`min_version == max_version == V1_4`,
`entities/document_format_options.rs:174-182`) que forçam essa faixa de
versão nesta fatia do produto, o efeito observável fica concentrado em
A1 — mas o contrato certo é "por versão", não "por A1", para não
reintroduzir o mesmo bug se um dia `--pdf-version 1.4` sem archival for
combinado com conteúdo RGB.

### 2.2 O perfil v2 já está vendorizável — mesma fonte/licença do v4 já usado (P777)

`~/.cargo/git/checkouts/krilla-dfa083154a81f6b2/7772dbe/crates/krilla/
icc/sRGB-v2-magic.icc` — 736 bytes, licença CC0 1.0
(`icc/README.md`: "ICC profiles are from:
https://github.com/saucecontrol/Compact-ICC-Profiles", `icc/LICENSE` =
CC0), mesma proveniência do perfil v4 já vendorizado em
`03_infra/src/export/images.rs:39-63` (`SRGB_ICC_PROFILE`, confirmado
byte-a-byte idêntico a `icc/sRGB-v4.icc` desse mesmo checkout, 480
bytes). SHA-256 do perfil v2:
`af4efe28f6d311799865f325ba39184a2e978b113ca74124d60af7de22b105f4`.

### 2.3 Três consumidores do perfil ICC partilham HOJE um único objeto — todos precisam da seleção por versão

`builder.rs:1052-1053/1433-1434/1935-1936` (um por `build_helvetica`/
`build_cidfont`/`build_multifont`): `icc_profile_id` é alocado quando
`has_rgb_jpeg(doc) || verbose` (P956/P777) e o MESMO objeto serve (a) o
`/ColorSpace /c0` dos content streams em modo verbose e (b) o
`/ColorSpace` dos XObjects de imagem JPEG RGB. `emit_pdfa_output_intent`
(P1475/P1476, `builder.rs:~2775`) aloca um **objeto próprio** separado
para `/DestOutputProfile`. Medição confirma (Achado 4.4 do diagnóstico
P1469, reconfirmado agora): usar o perfil v4 em qualquer um destes três
pontos quando a versão efetiva é 1.4-1.6 falha a regra 6.2.3.2. Os
quatro pontos de consumo (`/c0`+JPEG partilhados, mais `/OutputIntent`)
têm de usar a MESMA seleção — nunca o v4 quando a versão é 1.4/1.5/1.6.

### 2.4 `/CIDSet` — bitset determinístico a partir do conjunto de CIDs usados, sem inferência de gaps

Medido em dois documentos vanilla A1b (texto curto vs. texto variado,
`qpdf --object-streams=disable` + `zlib.decompress` do stream
`/CIDSet`):

```text
"Um parágrafo curto"           -> 1 byte, bits 11111111 (8 CIDs, 0 padding)
texto variado (~14 glifos)     -> 2 bytes, bits 11111111 11111100 (14 CIDs, 2 bits de padding)
```

Em ambos os casos o bitset é **denso desde o bit 0**, sem gaps — CID `i`
presente para todo `i < N`, resto da última byte a zero. Confirmado
contra a fonte (`03_infra/src/export/subset.rs:38-80`,
`subsetter::GlyphRemapper`): o remapper atribui `new_gid` sequenciais a
partir de 0 a cada `old_gid` remapeado, logo `glyph_mapping.values()`
forma exatamente o conjunto `0..N` sem buracos quando o subset é bem
sucedido (`!glyph_mapping.is_empty()`). Quando o subset falha (fallback
P797/P940, `glyph_mapping.is_empty()`, fonte embutida por inteiro), os
CIDs no PDF são os `old_gid` originais — **não** contíguos — mas o
conjunto usado já está disponível na variável `all_glyph_ids`/
`face_glyph_ids` (calculada antes da chamada a `subset_font_with_
mapping`, `builder.rs:1153-1165`/`1565-1566`), sem necessidade de nova
coleta. O contrato usa sempre o conjunto real de CIDs presentes (via
`glyph_mapping.values()` quando não vazio, senão o conjunto de GIDs
usados pré-subset) — nunca assume contiguidade por atalho, mesmo que a
medição confirme que o caso comum já é contíguo.

### 2.5 `/CIDSet` só se aplica quando há `/FontFile*` embutido

Fontes 100% bitmap (P941, `bitmap_only`, sem `/FontFile2`/`/FontFile3`
no descritor) não têm "programa de fonte" cuja completude o `/CIDSet`
declare — não medido diretamente (nenhum fixture A1 usa fonte bitmap),
decisão por analogia estrutural: `/CIDSet` só é emitido quando
`font_file_entry` (já existente, `builder.rs:1382-1386`/`1883-1887`) não
é vazio.

### 2.6 Medição negativa — build cristalino atual sob A1b confirma exatamente os dois gaps

```text
verapdf -f 1b cristalino-a1b.json (antes de P1477, com figura RGB)
  6.2.3.2  "embedded ICC profile ... version 4.2 ... does not satisfy PDF 1.4 requirements"
  6.3.5    "A CIDSet entry in the Font descriptor is missing..." (uma vez por FontDescriptor)
```

Confirma que só estes dois itens faltam — nenhuma outra regra falha
(reconfirma o Achado 1 original do diagnóstico P1469: a-1b tinha 5
regras/37 checks falhados na medição bilateral geral, mas 3 dessas eram
as regras nucleares já fechadas por P1475/P1476 — só 6.2.3.2 e 6.3.5 são
específicas de A1).

## 3. Contrato

### 3.1 `infra/export/images.md` — segundo perfil ICC

Em `03_infra/src/export/images.rs`, junto a `SRGB_ICC_PROFILE` (linha
39-63):

1. Nova constante `SRGB_V2_ICC_PROFILE: &[u8]` (736 bytes, vendorizada a
   partir de `sRGB-v2-magic.icc`, mesma fonte/licença CC0 já citada para
   o perfil v4 — copiar os bytes reais do ficheiro, não regenerar).
2. Novo getter `pub(super) fn srgb_v2_icc_profile_bytes() -> &'static
   [u8]`, espelhando `srgb_icc_profile_bytes()`.
3. Novo helper `pub(super) fn srgb_icc_profile_for_version(version:
   PdfVersion) -> &'static [u8]` que aplica a regra §2.1
   (`V1_4|V1_5|V1_6 -> v2, V1_7|V2_0 -> v4`) — `match` exaustivo sobre
   `PdfVersion`, nunca wildcard (V16). Requer `use
   typst_core::entities::document_format_options::PdfVersion;` novo
   neste ficheiro.

### 3.2 `infra/export/builder.md` — consumo nos quatro pontos

Trocar as quatro chamadas a `srgb_icc_profile_bytes()` por
`srgb_icc_profile_for_version(self.standard.version)`:

1. `builder.rs:1053` (`build_helvetica`, objeto `/c0`+JPEG partilhado).
2. `builder.rs:1434` (`build_cidfont`, idem).
3. `builder.rs:1936` (`build_multifont`, idem).
4. `emit_pdfa_output_intent` (`builder.rs:~2790`, objeto próprio do
   `/OutputIntent`).

Os quatro pontos ficam consistentes entre si dentro do mesmo PDF (todos
leem `self.standard.version`, nunca hardcoded). Sem `standard` nenhum,
`self.standard.version` é `V1_7` (default), preservando o perfil v4
atual — regressão zero para o caso comum.

### 3.3 `/CIDSet` — helper puro + dois pontos de emissão

Novo helper puro em `builder.rs` (perto de `build_icc_profile_stream`
ou dos outros helpers de fonte):

```rust
/// Constrói o stream `/CIDSet` — um bit por CID presente no subset,
/// `bit=1` se presente, resto do último byte a zero. Aceita qualquer
/// conjunto de CIDs (não assume contiguidade), comprimido com
/// FlateDecode (mesmo padrão de `build_icc_profile_stream`/P883).
fn build_cidset_stream(cids: &HashSet<u16>) -> Vec<u8> {
    let max_cid = cids.iter().copied().max().unwrap_or(0);
    let n_bytes = (max_cid as usize / 8) + 1;
    let mut bits = vec![0u8; n_bytes];
    for &cid in cids {
        bits[cid as usize / 8] |= 0x80 >> (cid as usize % 8);
    }
    let compressed = compress_zlib(&bits).unwrap_or(bits);
    let len = compressed.len();
    let mut obj = format!("<< /Length {len} /Filter /FlateDecode >>\nstream\n").into_bytes();
    obj.extend_from_slice(&compressed);
    obj.extend_from_slice(b"\nendstream");
    obj
}
```

Em `build_cidfont` (perto de `builder.rs:1387-1405`) e no loop de
`build_multifont` (perto de `builder.rs:1888-1906`), depois do
`self.add(font_descriptor_id, ...)`/`self.add(descriptor_id, ...)`
existente:

1. Construir o conjunto de CIDs presentes: se `!glyph_mapping.is_empty()`
   → `glyph_mapping.values().copied().collect::<HashSet<u16>>()`; senão
   → o conjunto de GIDs usados antes do subset (`all_glyph_ids`/
   `face_glyph_ids`, já calculado nesse ponto da função).
2. Se `!bitmap_only` (`build_cidfont`) / `!per_font_bitmap_only[fi]`
   (`build_multifont`): alocar `cidset_id` dinamicamente
   (`self.objects.iter().map(|(id,_)| *id).max().unwrap_or(0)+1`,
   mesmo padrão de P1475/P1476 — **não** mexer na tabela fixa
   "Alocação de Object IDs (invariante)" já documentada neste L0),
   `self.add_bytes(cidset_id, build_cidset_stream(&cids))`, e editar o
   `/FontDescriptor` já adicionado (via `insert_pdf_dict_entry`,
   `builder.rs:~3560`) com ` /CIDSet {cidset_id} 0 R`.
3. Se `bitmap_only`/`per_font_bitmap_only[fi]`: não emitir `/CIDSet`
   (§2.5).

**Aplica-se a toda CIDFont embutida com `/FontFile*`, independente de
`standard`** — não é gated a A1, mas só A1 tem uma regra veraPDF que o
exige (6.3.5 não apareceu em nenhuma medição de A2/A3/A4 desta sessão);
emitir sempre é mais simples que gatear por versão/archival e não tem
custo de conformidade nos outros perfis (campo extra tolerado,
confirmado por P1475/P1476 não terem regredido ao ganhar campos novos
condicionais semelhantes). **Ponto de decisão para confirmação humana**:
se preferir gated só a A1a/A1b (superfície de mudança menor, mas exige
condicional extra nos dois pontos de emissão), sinalizar — o contrato
por omissão aqui é "sempre", por simplicidade.

## 4. Gate ADR-0127

Universal para CIDFonts (`/CIDSet` sempre que há `/FontFile*`, item
3.3) — muda bytes de produção de **todo** export CIDFont/multifont, não
só o caminho `standard: a-1*`. O item 3.1/3.2 (seleção de perfil ICC por
versão) é gated a `self.standard.version` ∈ `{V1_4,V1_5,V1_6}` — hoje
só alcançável via `standard: a-1a/a-1b` (as únicas combinações que
forçam essa faixa), mas o gate real é a versão, não o archival. Mesmo
protocolo dos passos anteriores: L0s atualizados, hash resselado,
commit causal somente-L0, **parar para confirmação humana explícita**
citando os dois itens do contrato, a pergunta aberta de §3.3 sobre
`/CIDSet` universal vs. gated, e que a mudança de `/CIDSet` afeta todo
export CIDFont por defeito.

## 5. RED e critérios de fechamento

- **Perfil ICC por versão**: `standard: ("1.4",)` sem archival, com
  imagem JPEG RGB ou modo verbose, embute o perfil v2 (736 bytes,
  `/N 3`); `standard: ("1.7",)` ou sem `standard` continua a embutir o
  v4 (480 bytes) — regressão zero. `a-1a`/`a-1b` (versão implícita 1.4)
  embutem v2 nos três pontos partilhados E no `/OutputIntent`.
- **`/CIDSet`**: presente em todo `/FontDescriptor` com `/FontFile*`
  (qualquer `standard`, incluindo nenhum); ausente quando `bitmap_only`.
  Bitset correto: byte count = `ceil((max_cid+1)/8)`, bits nas posições
  certas (casos: subset contíguo pequeno, subset com padding não
  múltiplo de 8, fallback sem subset com CIDs esparsos — construir um
  documento com uma fonte cujo subset falhe deliberadamente, se
  possível, ou aceitar cobertura indireta do caminho fallback via teste
  unitário do helper `build_cidset_stream` isolado).
- Regressão: todos os testes de P1475 (11) e P1476 (9) continuam GREEN
  sem edição; suíte completa sem quebra.

Depois do GREEN, medir com veraPDF `a-1a`/`a-1b`: esperado zero falhas
(perfil `129`+ regras passadas). Registar contagens e proveniência no
fecho.

Build, testes completos (core/shell/infra/wiring), Clippy (comparação
por contagem), fmt, `crystalline-lint .` (0 erros),
`crystalline-lint --checks v5,v15,v26 .` (0 violações), `git diff
--check`.

## 6. Não-objetivos explícitos

- PDF/A-4f/4e sem anexo, PDF/UA-1 — não tocados.
- `/CIDSet` para o caminho `build_helvetica` (Type1/Helvetica) — não é
  CIDFont, rule 6.3.5 não se aplica.
- Grey ICC (`grey_icc`, krilla `version.rs:64-70`) — o cristalino não
  emite `/DeviceGray` via ICCBased hoje (P777 nota: "JPEGs grayscale
  continuam a usar ColorSpace directo"); fora de escopo, não medido se
  algum perfil A1 com conteúdo grayscale precisaria disto.
- Qualquer mudança em `PdfVersion`/`PdfArchival`/`PdfStandards`
  (`entities/document_format_options.rs`) — nenhum campo novo, nenhuma
  mudança de contrato público, só consumo.

## 7. Execução segregada e integração (2026-09-22)

Ver `00_nucleo/prompts/infra/export/builder.md` §P1477 "Execução
segregada e integração" para o relato completo. Resumo do lado do
passo.

### 7.1 Confirmação registada antes do dispatch

Lição de P1476 aplicada: a confirmação (incluindo a resolução do ponto
em aberto §3.3 via `AskUserQuestion` — `/CIDSet` universal) foi
registada num commit dedicado (`8b6f41db0`) **antes** de despachar os
Agentes A/B, não só na integração final.

### 7.2 Agentes — achado convergente independente

- **Agente A** (commit `fcabcb2c7`): implementou os dois itens do
  contrato. Sinalizou (sem corrigir) que `emit_pdfa_output_intent`/
  `pdfaid_elem` não incluíam A1a/A1b.
- **Agente B** (commit `0d3542314`): 11 testes (7 RED + 4 guardas).
  Identificou **o mesmo gap**, de forma totalmente independente (sem
  ver o relatório do Agente A), e escreveu o teste literal do §5 já
  esperando a extensão do gate. Corrigiu também um bug próprio
  (fronteira de caractere multi-byte numa mensagem de pânico).

A convergência independente dos dois agentes no mesmo achado é o sinal
mais forte desta sessão de que a inconsistência entre o §3 "Contrato"
(que não listava explicitamente "adicionar A1a/A1b ao `matches!`") e o
§5 "Critérios de fechamento" (que já pressupunha isso) era real, não
uma leitura equivocada de um dos dois.

### 7.3 Achados de integração (papel C, fluxo contínuo)

1. Gate de A1a/A1b estendido em `emit_pdfa_output_intent`/`pdfaid_elem`
   (confirmado diretamente contra o vanilla: `part=1/conformance=B`
   para A1b, mesma forma de A2/A3).
2. Dois testes usavam `Cantarell-VF.otf` (CFF2, subsetter falha —
   §P772u) onde precisavam de uma fonte que subseta com sucesso;
   trocados para `NotoSans-Regular.ttf`.
3. V16 (`pdfaid_elem`) corrigido com o mesmo padrão de `af_relationship_
   name`/`af_relationship_variant_name` (P1476): `pdfaid_body`
   exaustivo só sobre `PdfArchival`.
4. Clippy (`regex em loop`, 3×) corrigido nos testes novos.
5. Fixture dourada `09-cidfont.pdf` regenerada (+119 bytes, só
   `/CIDSet`).
6. Dois testes de P1475 ajustados intencionalmente (A1a/A1b saem das
   listas "deve ficar ausente").

### 7.4 RED → GREEN e gates finais

11 testes novos: 7 RED genuínos + 4 guardas. Após integração: 11/11
GREEN. Suíte completa: `typst-core` 6129/6129, `typst-shell` 83/83,
`typst-infra` 1026/1026. `typst-wiring`: 1 falha pré-existente não
relacionada (igual à de P1475/P1476). Clippy: zero avisos novos na
medição final (1 achado real corrigido antes dela). `crystalline-lint
.`: 2 achados V16 reais corrigidos; **achado residual não resolvido**
— `[PARSE]` em `tests.rs:11146`, dentro do teste `p1477_cidset_
present_in_font_descriptor_and_stream_is_dense_bitset`, bissecionado
por truncamento controlado (ver `infra/export/builder.md` §P1477 para
o relato completo da investigação) — `rustc` (via `cargo build`/`cargo
test`) aceita o ficheiro sem erro; o erro só aparece quando `tests.rs`
cresce além de ~11127 linhas, aparentemente um limite do parser
próprio do `crystalline-lint`, não confirmado como bug (fora do
alcance desta sessão auditar o binário). Gate do passo (`0 erros`) não
satisfeito literalmente — sinalizado ao dono para decisão, não
decidido unilateralmente. `crystalline-lint --checks v5,v15,v26 .`:
drift esperado nos dois L0s, 0 violações V15/V26. `git diff --check`:
sucesso.

### 7.5 Medição de conformidade veraPDF (final)

Binário cristalino recompilado, veraPDF 1.30.2, documento com
heading/negrito/itálico/figura RGB, `#set page(width: 12cm, height:
8cm)`.

| Perfil | `isCompliant` | `failedRules` | `passedRules` |
|---|---|---|---|
| A1a | true | 0 | 135 |
| A1b | true | 0 | 129 |

**Conformidade total.**

### 7.6 Veredito final

Os dois itens do contrato (§3) fecham integralmente. Com P1477, a
família PDF/A completa medida nesta sessão (P1469→P1475→P1476→P1477)
está fechada: A1a/A1b/A2a/A2b/A2u/A3a/A3b/A3u/A4/A4f/A4e todos 100%
conformes veraPDF, com e sem anexos. PDF/A-4f/4e sem anexo e PDF/UA-1
permanecem como eixos não tocados (UA-1 explicitamente fora de escopo
desde P1469 — "eixo estrutural à parte, maior").

### 7.7 Nota retroativa (2026-09-22, P1479)

O achado `[PARSE]` do `crystalline-lint` que impedia este passo de
fechar literalmente com `0 erros` (§gates finais medidos acima) foi
resolvido por `typst-passo-1479.md` — causa-raiz isolada a um
identificador local (`raw`) na função `p1477_cidset_present_in_font_
descriptor_and_stream_is_dense_bitset`/`p1477_cidset_present_for_
multifont_path` (não era tamanho acumulado de `tests.rs`, hipótese
original refutada). Nenhuma medição, decisão ou contrato deste passo
foi reaberto ou alterado — só a localização física dos testes mudou
(de `tests.rs` para `tests/p1477_pdfa_icc_version_and_cidset_tests.rs`)
e o identificador local foi renomeado.
