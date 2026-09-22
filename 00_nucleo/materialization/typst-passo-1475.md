# Passo 1475 — Núcleo PDF/A-2/3/4: trailer `/ID`, XMP `pdfaid`, `/OutputIntent`, supressão de `/Info` em A4

**Estado:** MATERIALIZADO (confirmação humana 2026-09-22, posterior ao
commit causal somente-L0 `7a20ec72d`; execução segregada A/B/C — ver §7)

**Data da auditoria:** 2026-09-22T00:31:50-03:00

**Baseline de redação:** `8380beef50e89a7d3523cd818dbf6c9b80dff01f`, tree
`6fa2a6ba2cefd41f9d5307638850f6a08d5f6dfb`, working tree limpo.

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

## 0. Origem e continuidade

Este passo é a continuação explícita da nota de scope-out registada em
`entities/document_format_options.md` §P1469 ("Um passo futuro, quando
houver uma biblioteca de geração PDF/A/UA conforme...") e do fechamento do
diagnóstico `00_nucleo/diagnosticos/typst-p1469-revisita-verapdf-2026-09-21.md`
(Achado 4), que mediu bytes exatos do vanilla e concluiu que o gap para
conformidade real de PDF/A-2/3 é "pequeno e mecânico". A precondição que
bloqueava P1469 (falta de validador) está resolvida — veraPDF 1.30.2 foi
instalado nessa sessão anterior e usado para medir os achados abaixo.

## 1. Objetivo

Tornar `format.pdf.standard`/`--pdf-standard` com `a-2b`/`a-2u`/`a-2a`/
`a-3b`/`a-3u`/`a-3a` **totalmente conformes** (veraPDF PASS nos perfis
nativos correspondentes) e `a-4` conforme nas quatro regras medidas em
`typst-p1469-revisita-verapdf-2026-09-21.md` Achado 1, implementando
exatamente os quatro itens medidos no Achado 4 desse diagnóstico:

1. Trailer `/ID` — **universal** (todos os exports, não só PDF/A — ver §2.1).
2. XMP `pdfaid` (identificação PDF/A) — só quando `standard.archival` ∈
   `{A2a,A2b,A2u,A3a,A3b,A3u,A4}`.
3. `/OutputIntent` com perfil ICC sRGB reutilizado — mesmo conjunto de
   archivals do item 2.
4. Supressão de `/Info` — só `archival == Some(A4)`.

`a-1a`/`a-1b`/`a-4f`/`a-4e`/`ua-1` **não são tocados** por este passo —
motivo medido em §2.4/§2.5, ficam scope-out explícito para passo(s)
futuro(s) próprio(s).

## 2. Medição confirmatória desta sessão (complementa o Achado 4)

Todas as medições abaixo usam `/usr/local/bin/typst` (vanilla ratificado,
`typst 0.15.1 (586e1bd4)`) em `2026-09-22T00:3x-03:00`, commit de working
tree `8380beef5` (sem alterações relativas a este passo).

### 2.1 Trailer `/ID` é emitido em **todo** PDF vanilla, não só PDF/A

Documento sem `standard` nenhum (`= Teste simples\nUm parágrafo comum.`):

```text
trailer
<</Size 30/Root 29 0 R/Info 27 0 R/ID[(Bak2wzDJVikeVJ60ke2gBw==)(Bak2wzDJVikeVJ60ke2gBw==)]>>
```

Confirma o Achado 4.1 (o mesmo texto base64 de 16 bytes aparece duas vezes
no array `/ID`, idêntico ao par instance/document da XMP) e estende a
conclusão: **isto não é uma feature de PDF/A**, é comportamento universal
do vanilla. Corrigir só quando `standard.archival.is_some()` deixaria o
cristalino sem `/ID` em todo PDF comum — divergência estrutural
desnecessária e evitável ao mesmo custo. Por isso o item 1 do contrato é
universal (todos os `build_helvetica`/`build_cidfont`/`build_multifont`),
não condicionado a `standard`.

**Nota ADR-0127**: por afetar o trailer de **todo** PDF exportado (não só
o caminho `standard:` opt-in), este item é, por si, uma mudança de
comportamento por defeito do produto — motivo adicional para o gate de
confirmação humana explícita deste passo (não basta a confirmação de
P1469, que só cobria o header de versão).

### 2.2 XMP `pdfaid` diverge de esquema entre A1-3 e A4

Medido em `vanilla_a2b.pdf` (Achado 4.2, sessão anterior):
`<pdfaid:part>2</pdfaid:part><pdfaid:conformance>B</pdfaid:conformance>`.

Medido **nesta sessão** em `a4_test.pdf` (`--pdf-standard a-4`):

```xml
<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev>
```

**Achado novo, não previsto pelo Achado 4.2**: PDF/A-4 **não tem**
`pdfaid:conformance` (a ISO 19005-4 eliminou o eixo a/b/u que 1/2/3 têm) —
em vez disso usa `pdfaid:rev` com o ano da revisão (`"2020"`, ano de
publicação da ISO 19005-4). Confirmado por ausência: `grep` por
`pdfaid:conformance` no PDF/A-4 gerado não encontra nada. O esquema
`pdfaid` tem **duas formas**, não uma:

- A2/A3 (`a`/`b`/`u`): `<pdfaid:part>{2|3}</pdfaid:part>
  <pdfaid:conformance>{A|B|U}</pdfaid:conformance>` — letra derivada do
  sufixo do nome (`A2b`→`"B"`, medido; `A2a`→`"A"`, `A2u`→`"U"`,
  `A3a`→`"A"`, `A3b`→`"B"`, `A3u`→`"U"` por extensão direta da mesma
  convenção nominal ISO, não medidos individualmente mas sem ambiguidade
  — o sufixo do variant name *é* a letra de conformidade).
- A4: `<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev>` — sem
  `conformance`.

### 2.3 `/OutputIntent` idêntico entre a-2b e a-4 (confirma Achado 4.3 além do medido antes)

Objeto medido nesta sessão para `a4_test.pdf`:

```text
9 0 obj << /DestOutputProfile 16 0 R /Info (sRGB v4.2) /OutputCondition (sRGB)
           /OutputConditionIdentifier (Custom) /RegistryName () /S /GTS_PDFA1
           /Type /OutputIntent >>
```

Byte-a-byte igual em forma ao objeto 5 medido para `a2b` no Achado 4.3
(`/S /GTS_PDFA1` fixo, mesmas seis chaves). Confirma: um único helper de
emissão serve A2/A3/A4 sem parametrizar por nível — só o `archival` decide
**se** emitir, nunca **como**.

### 2.4 `/Info` ausente em A4 (reconfirma Achado 4.6); trailer de A4 não tem `/Info`

```text
<< /ID [ (epe3CdpPPIg4J3uGmIxIwg==) (epe3CdpPPIg4J3uGmIxIwg==) ] /Root 31 0 R /Size 32 >>
```

Sem `/Info` — reconfirma Achado 4.6 nesta sessão, com o trailer já
mostrando `/ID` presente e `/Info` ausente simultaneamente (os dois
comportamentos não conflitam).

### 2.5 Por que A1a/A1b/A4f/A4e ficam fora

- **A1a/A1b**: `min_version(A1a|A1b) == max_version(A1a|A1b) == PdfVersion::V1_4`
  (`entities/document_format_options.rs:174-182`) — A1 é **sempre** PDF 1.4,
  nunca 1.7. O Achado 4.4 mediu que o perfil ICC v4.2 (`SRGB_ICC_PROFILE`,
  já usado por `/c0`/imagens desde P956/P777) **falha** a regra 6.2.3.2 em
  PDF 1.4 (`"is either invalid or does not satisfy PDF 1.4 requirements"`).
  Reusar o mesmo perfil para `/OutputIntent` em A1 reproduziria a mesma
  falha — implementar A1 corretamente exige vendorizar o segundo perfil ICC
  v2 (736 bytes, `sRGB-v2-magic.icc`) e alternar por versão, mais `/CIDSet`
  por fonte subsetada (Achado 4.5, toca `fonts.rs`, não só `builder.rs`).
  Scope-out explícito — passo próprio.
- **A4f/A4e**: não medidos (Achado 4, "Não coberto") — exigem ficheiro
  anexado; comportamento do `/OutputIntent`/`pdfaid` nesses dois perfis não
  foi confirmado. Scope-out explícito, não assumir igual a A4 sem medir.
- **UA1**: eixo estrutural distinto (`/Lang`, árvore de tags, `/MarkInfo
  /DisplayDocTitle`) — Achado 1 já classificou como "candidato a passo(s)
  separado(s)", confirmado outra vez pela observação nesta sessão de que
  `--pdf-standard a-4` sozinho já ativa `/Lang`/`/MarkInfo`/`/StructTreeRoot`
  no catálogo vanilla (efeito colateral de tagging automático não
  investigado — fora de escopo deste passo, que não mexe em tagging).

### 2.6 Achado 2 do diagnóstico anterior — bônus de baixo custo, mesmo commit

`emit_xmp_metadata` (`builder.rs:2779`, ver §3.2) escreve
`<pdf:PDFVersion>1.7</pdf:PDFVersion>` como literal fixo mesmo após P1469
tornar o header dinâmico. Corrigir para `self.standard.version.as_str()` é
independente de `archival` (aplica-se a qualquer `standard.version`) e não
depende de decisão de escopo — incluído neste passo por proximidade de
código (mesma função, mesmo commit), não como item novo do contrato
ADR-0127 (é correção de fórmula interna, ADR-0127 fluxo contínuo).

## 3. Contrato — `infra/export/builder.md` (owner único, `03_infra/src/export/builder.rs`)

Todos os quatro itens ficam neste único owner. Nenhuma mudança em
`entities/document_format_options.rs` (o `PdfStandards`/`PdfArchival` já
tem todos os valores necessários — `archival: Option<PdfArchival>` já
propagado desde P1469) nem em `infra/export/images.rs` (reusa
`srgb_icc_profile_bytes()`/`build_icc_profile_stream()`, ambos já
`pub(super)` e já importados por `builder.rs:130-141`, mesmo padrão de uso
já existente em `builder.rs:1043/1423/1924`, P777 — sem alteração de
assinatura). `builder.rs:115` ganha `PdfArchival` no `use` existente de
`document_format_options::{PageRanges, PdfStandards}` (hoje não importa
`PdfArchival`, necessário para os `match`/`matches!` de 3.3-3.5).

### 3.1 Trailer `/ID` — universal

Em `emit_xmp_metadata` (`builder.rs:2703`), a chamada existente
`let (instance_id, document_id) = xmp_instance_and_document_id(self.document_id);`
(linha 2754) já produz os dois valores base64 necessários. Adicionar campo
`xmp_ids: Option<(String, String)>` ao `struct PdfBuilder` (perto de
`document_id`, `builder.rs:744`), inicializado `None` em `new()`; em
`emit_xmp_metadata`, logo após calcular `(instance_id, document_id)`, gravar
`self.xmp_ids = Some((instance_id.clone(), document_id.clone()))` (o uso
local do `format!` da XML continua igual, só ganha a persistência).

Em `serialize()` (`builder.rs:3316`), antes de construir o `trailer` (linha
3348-3358), computar:

```rust
let id_entry = self
    .xmp_ids
    .as_ref()
    .map(|(inst, doc)| format!(" /ID [ ({inst}) ({doc}) ]"))
    .unwrap_or_default();
```

e inserir `{id}` no `format!` do trailer, entre `info_ref` e o fecho `>>`
(ordem exata não é observável — qualquer posição das três chaves dentro do
dicionário é válida em PDF; manter a ordem `/Size /Root {info} {id}` por
simplicidade de diff). `xmp_ids` é `None` apenas no caminho teórico em que
`emit_xmp_metadata` não foi chamado — não acontece hoje (todas as três
variantes `build_*` chamam-na antes de `serialize`, `builder.rs:1058/
1438/1939`), mas o fallback `unwrap_or_default()` evita pânico se isso
mudar no futuro (mesmo padrão defensivo já usado para `info_ref`).

Nenhuma flag nova; nenhum novo parâmetro público. Aplica-se
independentemente de `self.standard`.

### 3.2 `pdf:PDFVersion` dinâmico (Achado 2, fluxo contínuo)

Em `emit_xmp_metadata`, trocar o literal `<pdf:PDFVersion>1.7</pdf:PDFVersion>`
(`builder.rs:2779`) por
`<pdf:PDFVersion>{}</pdf:PDFVersion>", self.standard.version.as_str()` (novo
argumento `pdf_version = self.standard.version.as_str()` na chamada
`format!` existente). Sem `standard` explícito, `PdfStandards::default()`
produz `"1.7"` — byte-idêntico ao literal anterior.

### 3.3 XMP `pdfaid` — gated a `{A2a,A2b,A2u,A3a,A3b,A3u,A4}`

Adicionar ao `emit_xmp_metadata`, antes de fechar `<rdf:Description>` (a
posição relativa aos outros campos não é observável para veraPDF — inserir
depois de `xmpMM:DocumentID`/`RenditionClass`, antes de `pdf:PDFVersion`,
por proximidade temática com os outros identificadores):

```rust
let pdfaid_elem = match self.standard.archival {
    Some(a @ (PdfArchival::A2a | PdfArchival::A2b | PdfArchival::A2u)) => {
        format!("<pdfaid:part>2</pdfaid:part><pdfaid:conformance>{}</pdfaid:conformance>", pdfaid_letter(a))
    }
    Some(a @ (PdfArchival::A3a | PdfArchival::A3b | PdfArchival::A3u)) => {
        format!("<pdfaid:part>3</pdfaid:part><pdfaid:conformance>{}</pdfaid:conformance>", pdfaid_letter(a))
    }
    Some(PdfArchival::A4) => "<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev>".to_string(),
    _ => String::new(),
};
```

com um helper puro `fn pdfaid_letter(a: PdfArchival) -> &'static str` que
mapeia `A2a|A3a -> "A"`, `A2b|A3b -> "B"`, `A2u|A3u -> "U"` (único ponto
onde a letra de conformidade é derivada — evita repetir o `match` três
vezes). O namespace `xmlns:pdfaid="http://www.aiim.org/pdfa/ns/id/"` só
aparece no `<rdf:Description ...>` de abertura **quando `pdfaid_elem` não é
vazio** (mesma condicionalidade que os outros namespaces desta função não
têm hoje, porque hoje todos são incondicionais — este é o primeiro
namespace condicional do bloco XMP; declarar `xmlns:pdfaid` sempre, mesmo
sem uso, não quebra nada mas emite um namespace morto em todo PDF não-A —
preferir condicional, replicando o vanilla, que só declara `pdfaid` quando
`standard` inclui archival).

### 3.4 `/OutputIntent` — gated ao mesmo conjunto de 3.3

Novo método privado `fn emit_pdfa_output_intent(&mut self)`, chamado nas
três variantes `build_*` no mesmo ponto relativo (logo após
`self.emit_info(doc)`, antes de `self.emit_xmp_metadata(doc)` —
`builder.rs:1057/1437/1938`, mantendo a mesma posição relativa nas três
para preservar alocação de IDs idêntica entre caminhos):

```rust
fn emit_pdfa_output_intent(&mut self) {
    let needs_output_intent = matches!(
        self.standard.archival,
        Some(
            PdfArchival::A2a | PdfArchival::A2b | PdfArchival::A2u |
            PdfArchival::A3a | PdfArchival::A3b | PdfArchival::A3u |
            PdfArchival::A4
        )
    );
    if !needs_output_intent {
        return;
    }
    // `build_icc_profile_stream`/`srgb_icc_profile_bytes` já estão em
    // escopo via `use super::{...}` (builder.rs:130-141), mesmo padrão de
    // uso já existente em builder.rs:1043/1423/1924 (P777).
    let profile_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
    self.add_bytes(profile_id, build_icc_profile_stream(srgb_icc_profile_bytes()));

    let intent_id = profile_id + 1;
    self.add(
        intent_id,
        format!(
            "<< /Type /OutputIntent /S /GTS_PDFA1 /OutputConditionIdentifier (Custom) \
             /OutputCondition (sRGB) /RegistryName () /Info (sRGB v4.2) \
             /DestOutputProfile {profile_id} 0 R >>"
        ),
    );

    if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
        insert_pdf_dict_entry(content, &format!(" /OutputIntents [{intent_id} 0 R]"));
    }
}
```

Reusa `insert_pdf_dict_entry` (já usado por `emit_tag_structure`,
`builder.rs:3309`) em vez do padrão manual `rfind(">>")` usado pelos
`emit_outlines`/`emit_xmp_metadata` mais antigos — ambos os padrões
coexistem no ficheiro hoje; preferir o helper existente por ser o mais
recente e não duplicar lógica de insertion. Não reutiliza o ICC object já
alocado para JPEGs RGB (P777) mesmo quando ambos existem no mesmo
documento — aloca sempre o seu próprio objeto. Isto é uma decisão
mecânica livre (ADR-0107): a dedução/partilha do objeto ICC entre P777 e
este item é uma otimização de tamanho de ficheiro sem efeito observável
para veraPDF (não verifica contagem de objetos), fora do escopo de
"conformidade" que este passo mede. Se um `git blame`/revisão futura
quiser deduplicar, é mudança mecânica, não precisa de novo gate
ADR-0127.

**Limitação aceite e registada** (não medida, não implementada): se
`standard:` combinar `a-2*`/`a-3*` com a versão explícita `"1.4"` (fora do
default, que é `"1.7"` para essas duas famílias — `max_version`), o ICC
v4.2 reusado pode reproduzir a mesma falha 6.2.3.2 medida para A1 em
PDF 1.4 (§2.5). Não medido nesta fatia porque o veraPDF do Achado 1 testou
sempre a versão default de cada família. Ficar ciente é suficiente para
este passo — não é motivo para não avançar (o caso combinado é incomum:
declarar A2/A3 e simultaneamente forçar a versão mínima do range).

### 3.5 Supressão de `/Info` — só `archival == Some(A4)`

Em `emit_info` (`builder.rs:2668`), adicionar guarda no topo:

```rust
fn emit_info(&mut self, doc: &PagedDocument) {
    if self.standard.archival == Some(PdfArchival::A4) {
        return; // ISO 19005-4 6.1.3 — /Info não pode estar presente.
    }
    // ... resto inalterado
}
```

`self.info_id` permanece `None`; a lógica já existente em `serialize()`
(`info_ref = self.info_id.map(...).unwrap_or_default()`, `builder.rs:3348`)
já omite `/Info` do trailer sem nenhuma outra mudança. Não se aplica a
`A4f`/`A4e` (fora do `match`, scope-out §2.5) nem a nenhum outro archival.

## 4. Gates independentes dentro do passo

Um único owner (`infra/export/builder.md`), portanto um único gate:

1. medir (feito acima, §2) — confirmar hash/proveniência;
2. atualizar `infra/export/builder.md` com os quatro itens do §3, citando
   `file:line` exatos medidos;
3. ressellar hash (`crystalline-lint --fix-hashes .`), reverter
   `@prompt-hash` de `builder.rs` (`git checkout --`, reseal só junto do
   código);
4. commit causal somente-L0;
5. **parar para confirmação humana explícita**, citando os quatro itens e
   a nota ADR-0127 de §2.1 (trailer `/ID` é universal, não gated).

## 5. RED e critérios de fechamento

Casos exigidos (Agente B, sem ver a implementação):

- **Trailer `/ID`**: qualquer export (sem `standard`) tem `/ID [ (X) (Y) ]`
  no trailer, com `X == Y` no caso comum (sem `--document-id` externo) e
  `X`/`Y` batendo exatamente com `xmpMM:InstanceID`/`DocumentID` do mesmo
  PDF (mesma instância de build). Com `CRYSTALLINE_PDF_FIXED_EPOCH`, valor
  determinístico (mesma fixture já usada por outros testes de XMP).
- **`pdf:PDFVersion` dinâmico**: `standard: ("2.0",)` sem archival produz
  `<pdf:PDFVersion>2.0</pdf:PDFVersion>`; sem `standard`, continua `"1.7"`
  (regressão zero).
- **`pdfaid`**: `a-2b`/`a-2u`/`a-2a`/`a-3b`/`a-3u`/`a-3a` produzem
  `pdfaid:part` + `pdfaid:conformance` com a letra certa; `a-4` produz
  `pdfaid:part=4` + `pdfaid:rev=2020` sem `conformance`; sem archival,
  nenhum elemento `pdfaid:*` nem `xmlns:pdfaid`; `a-1b`/`a-1a`/`a-4f`/
  `a-4e`/`ua-1` (sozinhos) também **não** produzem `pdfaid:*` (scope-out
  confirmado negativamente).
- **`/OutputIntent`**: presente com `/S /GTS_PDFA1` e `/DestOutputProfile`
  apontando a um stream `/N 3 /Filter /FlateDecode` para os sete archivals
  do §3.4; ausente para `None`/A1/A4f/A4e/UA1-sozinho.
  `/OutputIntents` no catálogo é um array de um elemento.
- **`/Info`**: ausente do trailer/objects quando `archival == Some(A4)`;
  presente (comportamento atual) para todos os outros casos, incluindo
  `A4f`/`A4e`.
- Regressão: todos os testes PDF existentes (P1462/P1463/P1464/P1465/
  P1467/P1468/P1469) continuam GREEN sem edição.

Depois do GREEN, medir com veraPDF (binário `~/.local/bin/verapdf`, já
instalado) os seis perfis `a-2b/a-2u/a-2a/a-3b/a-3u/a-3a`: esperado
**zero falhas** nas 3 regras nucleares (trailer/pdfaid/OutputIntent);
`a-4`: esperado **zero falhas** nas 5 regras do Achado 1 (as 3 nucleares +
as 2 de supressão de `/Info`). Registar o relatório JSON do veraPDF (contagem
de regras passadas/falhadas) no fecho deste passo, mesma proveniência
exigida pela regra de registo de medições do `CLAUDE.md`.

Executar suíte completa (`typst-core`/`typst-shell`/`typst-infra`),
Clippy (comparação por contagem, mesma metodologia de P1471-P1473), build,
fmt, `crystalline-lint .` (0 erros), `crystalline-lint --checks v5,v15,v26
.` (0 violações), `git diff --check`.

## 6. Não-objetivos explícitos

- PDF/A-1a/1b, PDF/A-4f/4e, PDF/UA-1 — scope-out (§2.5), passos futuros.
- Qualquer parametrização de `/OutputIntent` por perfil além do `S` fixo
  (`GTS_PDFA1` é o único subtype GTS observado em todos os archivals
  medidos, A1 a A4 — não há "GTS_PDFA2"/"GTS_PDFA3"/"GTS_PDFA4").
- Deduplicação do objeto ICC entre P777 (imagens) e este passo — mecânica,
  não observável, fora de escopo (§3.4).
- `/Lang`/tagging automático observado incidentalmente em `a-4` (§2.5) —
  não implementado, não é requisito das 3 regras nucleares nem das 2 de
  A4 medidas no Achado 1.

## 7. Execução segregada e integração (2026-09-22)

Confirmação humana explícita recebida em 2026-09-22, na mesma sessão,
posterior ao commit causal somente-L0 `7a20ec72d`. Ver
`00_nucleo/prompts/infra/export/builder.md` §P1475 "Execução segregada e
integração" para o relato completo (achados de integração, gates finais
medidos) — esta secção resume só o essencial do lado do passo.

- **Agente A** (implementação, commit `eeeee4538`): os cinco itens do
  contrato §3, sem ver os testes. Reproduziu e corrigiu por conta própria
  o bug de infra conhecido (worktree no baseline errado). Mediu e
  reportou proativamente (não corrigiu, fora do seu mandato) que
  `xmp_instance_and_document_id` pré-existente gera `InstanceID`/
  `DocumentID` sempre distintos — achado que evitou uma falsa regressão
  na integração.
- **Agente B** (testes, commit `e090fc29f`): 11 testes novos em
  `03_infra/src/export/tests.rs`, via a API pública do builder (nunca
  construção direta de tipos internos ainda inexistentes). 6 RED
  genuínos + 5 guardas de não-regressão, confirmados contra o mesmo
  baseline `7a20ec72d` antes da integração.
- **Papel C** (integração, o próprio dono desta execução): aplicou os
  dois patches sem conflito de merge (ambos tocando `builder.rs`
  extensivamente); corrigiu a asserção de teste `X==Y` incorreta
  (achado de integração, não bug de implementação — Agente A já tinha
  sinalizado a causa); corrigiu um V16 real (`_ =>` genérico em
  `pdfaid_elem`, virou `match` exaustivo); regenerou os 9 fixtures
  dourados de `p307b_snapshot_tests` (+62 bytes cada, confirmado
  byte-a-byte que nada mais mudou) via `UPDATE_P307B_SNAPSHOTS=1`.

### RED → GREEN

11 testes novos (`p1475_pdfa_conformance_core_tests`,
`03_infra/src/export/tests.rs`): 6 RED genuínos (falha de asserção,
nunca erro de compilação) + 5 guardas de não-regressão já GREEN no
baseline. Após integração e a correção da asserção `X==Y`: **11/11
GREEN**. Suíte completa `typst-infra --lib`: **1006/1006** (995
pré-existentes + 11 novos, incluindo os 9 fixtures regenerados). Suíte
`typst-core --lib`: **6129/6129** (não afetada, confirmação de
não-regressão). Suíte `typst-shell --lib`: **83/83**.

### Gates finais medidos

```text
cargo build --workspace                    -> sucesso
cargo test -p typst-core --lib               -> 6129/6129
cargo test -p typst-shell --lib               -> 83/83
cargo test -p typst-infra --lib               -> 1006/1006
cargo test -p typst-wiring                    -> 1 falha pré-existente não
                                                  relacionada (P1471, oracle
                                                  desatualizado de `pdf`
                                                  module/function), confirmada
                                                  no baseline por Agente A
cargo clippy -p typst-infra --all-targets --message-format=json
    comparado por contagem contra baseline
    `7a20ec72d` em worktree isolada           -> zero avisos novos
crystalline-lint .                             -> 0 erros/fatais
crystalline-lint --checks v5,v15,v26 .         -> drift esperado em builder.rs
                                                  (L0 mudou, código ainda não
                                                  resselado); 0 violações
                                                  V15/V26
git diff --check                               -> sucesso
```

### Medição de conformidade veraPDF (final)

Binário cristalino recompilado neste commit (`target/debug/typst`),
veraPDF 1.30.2, documento de teste com heading/negrito/itálico/figura
com caption, `#set page(width: 12cm, height: 8cm)`. Todos os sete
archivals do escopo:

| Perfil | `isCompliant` | `failedRules` | `passedRules` |
|---|---|---|---|
| A2b | true | 0 | 144 |
| A2u | true | 0 | 146 |
| A2a | true | 0 | 153 |
| A3b | true | 0 | 146 |
| A3u | true | 0 | 148 |
| A3a | true | 0 | 155 |
| A4  | true | 0 | 109 |

**Conformidade total** (não só as regras nucleares/medidas do Achado 1 —
zero falhas em qualquer regra do perfil nativo veraPDF para os sete).
Resultado excede a meta original do passo.

### Veredito final

Os cinco itens do contrato (§3) fecham integralmente. `a-1a`/`a-1b`/
`a-4f`/`a-4e`/`ua-1` permanecem scope-out explícito (§2.5/§6), sem
nenhuma alegação de conformidade — confirmado negativamente pelos testes
(`p1475_pdfaid_absent_for_scope_out_profiles_confirmed_negatively`,
`p1475_output_intent_absent_for_none_a1_a4f_a4e_ua1_alone_regression_guard`).
Dois achados de integração (asserção de teste `X==Y` incorreta; V16 de
qualidade) foram corrigidos em fluxo contínuo, sem reabrir o gate
ADR-0127 — nenhum dos dois altera contrato público, comportamento por
defeito adicional, ou fase de pipeline além do já decidido e confirmado
em §4.
