# Passo 1476 — Núcleo PDF/A-4f/4e: `/AFRelationship`, `/AF`, extensão de `/Info`/`/OutputIntent`/`pdfaid` a anexos

**Estado:** ESCRITO — AGUARDA CONFIRMAÇÃO ADR-0127

**Data da auditoria:** 2026-09-22T08:38:17-03:00

**Baseline de redação:** `647a4024ee7a663df0f1effff5c77d538982748f`, tree
`20b5f11b9cd943eb7add7f04baa1a94432535573`, working tree limpo.

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

## 0. Origem e continuidade

Continuação direta de dois scope-outs já registados:

1. `typst-passo-1475.md` §2.5 — A4f/A4e "não medidos (exigem ficheiro
   anexado)", explicitamente deixados fora de P1475.
2. `infra/export/builder.md` §P1286 (código, docstring de
   `emit_attachments`, `builder.rs:2415-2419`): "A relação associada ao
   ficheiro permanece no domínio para futuros perfis PDF/A, mas o export
   normal espelha o vanilla e não escreve `/AFRelationship` nem `/AF`" —
   este passo é exatamente esse "futuro".

Escolhido pelo dono via `AskUserQuestion` em 2026-09-22, entre quatro
opções (A1a/1b, A4f/4e, PDF/UA-1, retomar depreciação de P1474).

## 1. Objetivo

Tornar `--pdf-standard a-4f`/`a-4e` **totalmente conformes** (veraPDF
PASS) quando o documento contém pelo menos um `#pdf.attach(...)`, e
fechar de caminho o gap idêntico que já existia (não medido antes) para
`a-3b`/`a-3u`/`a-3a` + anexo — mesma causa raiz, mesmo código
(`emit_attachments`), mesmo owner.

Cinco itens, todos em `03_infra/src/export/builder.rs`
(`infra/export/builder.md`, owner único, já P1475):

1. `/AFRelationship` na Filespec — gated a `A3a/A3b/A3u/A4f/A4e`.
2. `/AF` no catálogo (array de referências às Filespecs) — mesmo gate.
3. `/Info` suprimido — estender de `Some(A4)` (P1475) para
   `Some(A4|A4f|A4e)`.
4. `/OutputIntent` — estender `needs_output_intent` (P1475) para incluir
   `A4f|A4e`.
5. XMP `pdfaid` — nova terceira forma: A4f/A4e usam `part`+`rev`+
   `conformance` (letra `F`/`E`), diferente de A4 sozinho (`part`+`rev`,
   sem `conformance`, P1475) e de A2/A3 (`part`+`conformance`, sem
   `rev`).

## 2. Medição (2026-09-22, mesma sessão)

Todas as medições usam `/usr/local/bin/typst` (vanilla ratificado) e
`~/.local/bin/verapdf` 1.30.2. Documento de teste base
(`#pdf.attach("data.txt", relationship: "supplement", mime-type:
"text/plain", description: "dados de teste")` + heading + parágrafo;
uma segunda variante acrescenta `#figure(rect(fill: blue), caption:
[...])` para exercitar `DeviceRGB`/`OutputIntent`).

### 2.1 Vanilla A4f/A4e com anexo: 100% conforme

```text
verapdf -f 4f out-a4f.pdf  -> isCompliant=true failedRules=0 (109 passedRules)
verapdf -f 4e out-a4e.pdf  -> isCompliant=true failedRules=0 (109 passedRules)
```

### 2.2 A4f vs A4e: única diferença é a letra de conformidade na XMP

`diff -a` byte-a-byte entre os dois PDFs vanilla (mesmo documento fonte)
mostra exatamente duas diferenças: os IDs aleatórios (não observáveis) e

```text
A4f: <pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev><pdfaid:conformance>F</pdfaid:conformance>
A4e: <pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev><pdfaid:conformance>E</pdfaid:conformance>
```

**Achado que corrige a expectativa de P1475 §2.2**: A4f/A4e usam as
**três** chaves (`part`+`rev`+`conformance`), não apenas `part`+`rev`
como o A4 "puro" medido em P1475. O eixo `conformance` não desapareceu
em PDF/A-4 — só desaparece quando não há sufixo `f`/`e` (não há "PDF/A-4"
sem qualificador com letra própria; `f`/`e` são as únicas duas variantes
com letra nesta família).

### 2.3 Filespec ganha `/AFRelationship`; catálogo ganha `/AF`

Medido em `out-a4f.pdf` (`qpdf --object-streams=disable`, bytes crus, não
normalizados por `--qdf` — ver nota §2.5 sobre `--qdf` reescrever tipos
de string):

```text
1 0 obj << /AF [2 0 R] ... /Names << /EmbeddedFiles << /Names [(data.txt) 2 0 R] >> >> ... >>
2 0 obj << /AFRelationship /Supplement /Desc (dados de teste)
           /EF << /F 8 0 R /UF 8 0 R >> /F (data.txt) /Type /Filespec
           /UF (data.txt) >>
```

Testado também **sem** `relationship:` explícito
(`#pdf.attach("data.txt")`): `/AFRelationship /Unspecified` — mapeamento
confirmado 1:1 com o enum já existente em `01_core/src/entities/
elements/pdf_attach.rs:16-21` (`AttachedFileRelationship::{Source, Data,
Alternative, Supplement}`, mesmos nomes de variante que os quatro valores
PDF; `None` do domínio → `/Unspecified`, quinto valor que o enum do
domínio não tem variante própria — literal fixo quando `relationship` é
`None`).

### 2.4 A3 (sem f/e) + anexo tem exatamente o mesmo padrão — confirmado, não assumido

```text
verapdf -f 3b out-a3b-attach.pdf -> isCompliant=true failedRules=0 (146 passedRules, 870 checks)
```

Catálogo: `/AF [3 0 R] ... /OutputIntents 6 0 R ...` (OutputIntent já
cobria A3 desde P1475; só faltava `/AFRelationship`/`/AF`). `/Info`
**presente** (não suprimido — A3 nunca suprime `/Info`, só a família A4,
já correto desde P1475). Confirma que o gate de `/AFRelationship`/`/AF`
é **por família que suporta anexos conformes** (A3\*, A4f, A4e — ISO
19005-1/2 não definem embedded files conformes; A3 introduziu-os; A4f/A4e
são o equivalente em PDF/A-4), não só A4f/A4e.

### 2.5 Sem `standard` nenhum: nem `/AFRelationship` nem `/AF`

`typst compile doc.typ out-plain-attach.pdf` (sem `--pdf-standard`):
Filespec sem `/AFRelationship`, catálogo sem `/AF`. Confirma o gate — não
é comportamento universal (ao contrário do trailer `/ID` de P1475, que
era universal). Nota metodológica: `qpdf --qdf` **reescreve** o tipo de
algumas strings (litera/hex) para uma forma canónica de leitura — bytes
crus (`qpdf --object-streams=disable`, sem `--qdf`) é que provam o tipo
real (`/UF (data.txt)` string literal, não hex, para nome puramente
ASCII; confirmado com nome acentuado que `/UF` vira hex UTF-16BE
`<feff...>` só quando há caracteres não-ASCII — mecânica de codificação,
não contrato de conformidade, não replicada aqui — ver §6).

### 2.6 `/Info` suprimido e `/OutputIntent` presente em A4f/A4e — mesma regra de A4

Trailer de `out-a4f.pdf`: `<< /Root ... /Size ... /ID [...] >>`, sem
`/Info`. Objeto `/OutputIntent` idêntico em forma ao medido para A2b/A4
em P1475 (`/S /GTS_PDFA1`, mesmo perfil ICC sRGB). Confirma que os itens
3-4 do contrato (P1475, já existentes) só precisam de **estender o
conjunto gated**, não de lógica nova.

### 2.7 Medição negativa — build cristalino parcial (P1475, antes deste passo) confirma exatamente os 3 gaps

Compilado com o binário cristalino atual (após P1475, antes deste passo)
sob `a-4f` com anexo + figura RGB:

```text
verapdf -f 4f cristalino-a4f-rich.pdf
  -> isCompliant=false, failedRules=5, failedChecks=7
  6.1.3  "/Info não pode estar presente..."           (2 instâncias)
  6.9    "Filespec deve ter /AFRelationship..."
  6.2.4.3 "DeviceRGB sem /OutputIntent..."
  6.7.3  "esquema de identificação PDF/A (pdfaid) em falta"
```

Nenhuma falha relativa a `/AF` ausente, `/EF /UF` ausente, ou `/Params`
ausente — os três primeiros itens do contrato (§1) são as únicas causas
de não-conformidade confirmadas por medição direta; `/AF` (item 2) não é
exigido por esta suite de regras veraPDF mas é replicado por completude
estrutural e paridade com o vanilla, custo marginal zero (mesmo ID da
Filespec já calculado) — ver §6 para o que fica de fora por não ser
exigido.

## 3. Contrato

Owner único: `infra/export/builder.md` / `03_infra/src/export/
builder.rs`. Nenhuma mudança em `01_core/src/entities/elements/
pdf_attach.rs` (o campo `relationship: Option<AttachedFileRelationship>`
já existe, já é parseado, já chega a `PagedDocument.attachments` — só
não era consumido na emissão PDF) nem em nenhum L0 de construção/parsing
(`compiler/stdlib/pdf.md`, `entities/elements/pdf_attach.md`, etc.) —
zero contrato público novo.

### 3.1 Helper de gate — família que suporta anexos conformes

```rust
fn archival_supports_embedded_files(archival: Option<PdfArchival>) -> bool {
    matches!(
        archival,
        Some(
            PdfArchival::A3a | PdfArchival::A3b | PdfArchival::A3u |
            PdfArchival::A4f | PdfArchival::A4e
        )
    )
}
```

Reusado pelos itens 3.2/3.3 abaixo. Não inclui `A2*`/`A4` (sem `f`/`e`)
nem `A1*` — nenhum destes suporta embedded files conformes no ISO 19005
correspondente (nem foi medido para eles).

### 3.2 `/AFRelationship` na Filespec + `/AF` no catálogo

Em `emit_attachments` (`builder.rs:2420`), remover a nota "não escreve
`/AFRelationship` nem `/AF`" do docstring (P1286, agora obsoleta) e:

1. Mapear `attachment.relationship` (`Option<AttachedFileRelationship>`,
   já em escopo) para o nome PDF via helper puro
   `af_relationship_name(Option<&AttachedFileRelationship>) -> &'static
   str` (`None -> "Unspecified"`, `Source -> "Source"`, `Data -> "Data"`,
   `Alternative -> "Alternative"`, `Supplement -> "Supplement"` —
   mapeamento 1:1 confirmado §2.3).
2. Quando `archival_supports_embedded_files(self.standard.archival)`,
   incluir ` /AFRelationship {nome}` no dicionário da Filespec (mesmo
   `self.add(filespec_id, ...)` existente, só o `format!` ganha o campo
   condicional); caso contrário, omitir (comportamento atual preservado
   byte-a-byte, confirmado §2.5).
3. Acumular os `filespec_id` de todas as attachments processadas neste
   loop (`Vec<usize>`); ao fim da função, se
   `archival_supports_embedded_files(...)` **e** o vetor não estiver
   vazio, editar o catálogo (objeto 1) com
   `/AF [{ids "0 R "}...]` via `insert_pdf_dict_entry` (helper já
   existente, `builder.rs:3464`-ish, mesmo usado por P1475's `/OutputIntents`).

### 3.3 `/Info` suprimido — estender o `match` de P1475

Em `emit_info` (`builder.rs:2680`), trocar a comparação pontual

```rust
if self.standard.archival == Some(PdfArchival::A4) {
```

por

```rust
if matches!(
    self.standard.archival,
    Some(PdfArchival::A4 | PdfArchival::A4f | PdfArchival::A4e)
) {
```

Comentário atualizado citando P1476 além de P1475 (não apagar a
referência a P1475 — acrescentar).

### 3.4 `/OutputIntent` — estender `needs_output_intent`

Em `emit_pdfa_output_intent` (`builder.rs:2728`), acrescentar
`PdfArchival::A4f | PdfArchival::A4e` aos braços do `matches!` existente
(`builder.rs:2729-2736`). Nenhuma outra mudança nessa função — o mesmo
perfil ICC sRGB e o mesmo `/S /GTS_PDFA1` aplicam-se (confirmado §2.6).

### 3.5 XMP `pdfaid` — nova terceira forma (A4f/A4e)

Em `pdfaid_elem` (`builder.rs:2830`), acrescentar dois braços novos ao
`match` exaustivo existente, antes do braço `Some(PdfArchival::A4)`:

```rust
Some(PdfArchival::A4f) => {
    "<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev><pdfaid:conformance>F</pdfaid:conformance>".to_string()
}
Some(PdfArchival::A4e) => {
    "<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev><pdfaid:conformance>E</pdfaid:conformance>".to_string()
}
```

Remover `A4f`/`A4e` do braço `None | Some(PdfArchival::A1a | A1b | A4f |
A4e)` existente (ficam só `None | Some(A1a | A1b)` nesse braço de
scope-out). O `match` continua exaustivo sobre `Option<PdfArchival>` —
compilador valida.

## 4. Gate ADR-0127

Nenhum item deste contrato é universal (todos gated a `standard:`
explícito, confirmado §2.5) — mais próximo de "fluxo contínuo"
(correção/extensão de um caminho opt-in já existente) do que P1475's
item 1. Mesmo assim, muda bytes de produção para `a-3*`+anexo/`a-4f`/
`a-4e` (comportamento por defeito **desses** flags), e o próprio P1286
já tinha deixado escrito que reabrir isto exigia decisão própria — segue
o mesmo protocolo dos passos anteriores desta sessão: L0 atualizado,
hash resselado, commit causal somente-L0, **parar para confirmação
humana explícita** citando os cinco itens do contrato (§1) e a família
de archivals afetada (`A3a/A3b/A3u/A4f/A4e`).

## 5. RED e critérios de fechamento

- `/AFRelationship` presente com o valor certo (`Unspecified` por
  omissão, e os quatro nomes explícitos) quando `archival` ∈
  `{A3a,A3b,A3u,A4f,A4e}`; ausente para `None`/A1/A2/A4 (sem f/e) —
  regressão negativa explícita.
- `/AF` no catálogo é um array com uma referência por Filespec, mesmo
  gate; ausente fora dele.
- `pdfaid` para A4f: `part=4 rev=2020 conformance=F`; A4e:
  `part=4 rev=2020 conformance=E`; A4 sozinho continua `part=4 rev=2020`
  sem `conformance` (regressão P1475).
- `/Info` ausente para A4/A4f/A4e; presente para A3\*/A2\*/A1\*/sem
  standard (regressão).
- `/OutputIntent` presente para A2\*/A3\*/A4/A4f/A4e; ausente fora desse
  conjunto (regressão P1475 + extensão).
- Regressão: todos os testes de P1475 (11 novos) continuam GREEN sem
  edição; suíte completa `typst-infra`/`typst-core`/`typst-shell` sem
  quebra.

Depois do GREEN, medir com veraPDF: `a-3b`/`a-3u`/`a-3a` + anexo, `a-4f`
+ anexo, `a-4e` + anexo — esperado **zero falhas** em todos (mesmo
patamar de P1475). Documento de teste deve incluir conteúdo RGB (para
exercitar a regra 6.2.4.3/OutputIntent, medida só na variante "rica" —
§2.7) além do anexo em si. Registar contagens e proveniência no fecho.

Build, testes completos (core/shell/infra/wiring), Clippy (comparação
por contagem contra este baseline), fmt, `crystalline-lint .` (0
erros), `crystalline-lint --checks v5,v15,v26 .` (0 violações),
`git diff --check`.

## 6. Não-objetivos explícitos

- `/Params` (`/Size`, `/ModDate`) no stream `/EmbeddedFile` — vanilla
  emite, veraPDF **não** exigiu na medição §2.7 (ausência não gerou
  falha). Não implementado — sem benefício de conformidade medido,
  custo não nulo (reusar formato de data já usado por `/Info`, que
  usa `D:YYYYMMDDHHMMSS` sem offset, diferente do offset com apóstrofo
  medido no `/Params/ModDate` do vanilla — formatos diferentes,
  precisaria de medição própria). Candidato a passo futuro se um
  validador mais estrito o exigir.
- `/UF` codificado como UTF-16BE hex apenas para nomes não-ASCII
  (vanilla, §2.5) — cristalino já emite sempre hex UTF-16BE
  (`utf16be_hex_string`, P1286), válido mas não byte-idêntico ao
  vanilla para nomes ASCII. Divergência de forma, não de conformidade
  (ADR-0107) — não é objeto deste passo, não afeta nenhuma regra
  medida.
- `A1a`/`A1b` — sem anexo medido aqui; scope-out de P1475 §2.5 continua
  de pé (ICC v2 + `/CIDSet`).
- PDF/UA-1 — eixo à parte, não tocado.
- Qualquer normalização/deduplicação de `/AFRelationship` quando o mesmo
  ficheiro é anexado duas vezes com relações diferentes — não medido,
  comportamento existente de "anexar duas vezes" já rejeitado noutro
  ponto (`emit_attachments`, erro pré-existente não relacionado).
