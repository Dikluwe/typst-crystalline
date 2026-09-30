# L0 — infra/export/fonts
Hash do Código: 53634b63

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/fonts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/fonts.rs:26` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF font and Unicode helpers

Own escape_pdf_string, document collectors, char/GID mapping, widths, ToUnicode
CMap, UTF-16BE hexadecimal encoding and shaped cluster reconstruction.
Keep existing sibling visibility and no filesystem or network lookup.

The Helvetica literal helper escapes parentheses and backslash, retains printable
ASCII (including DEL under the current predicate), and replaces non-ASCII or
characters below ASCII space with "?". This is the actual restricted helper,
not a claim of full Latin-1 support. The Identity-H helper emits four-hex-digit
GIDs and maps an absent character to 0000.

Collect codepoints from plain Text and shaped char_code; collect_text_codepoints
reads only plain Text; collect_glyph_ids includes direct Glyph and shaped GIDs.
Traverse background/body/foreground and Group/Link/Semantic descendants through
FrameVisitor. Nonvisual alt never contributes glyphs or text. Ordered sets keep
unique output deterministic. Shaped mapping uses the first char_code per GID and
ignores zero-x_advance marks; shaped cluster-text collection preserves run order.

cluster_text derives boundaries from sorted unique UTF-8 byte offsets plus text
length, independently of visual LTR/RTL glyph order. Invalid/non-boundary ranges
yield an empty substring. Only the first occurrence of a cluster receives its
complete UTF-16BE text; later glyphs in the cluster receive empty hex. Preserve
ligatures and non-BMP surrogate pairs, not truncated u16 codepoints.

widths_array accepts current (gid, hex) pairs and converts the effective face's
advance to rounded 1/1000 text units, retaining missing-advance fallback 500.
to_unicode_cmap accepts GID-to-UTF-16BE strings, Identity-UCS codespace 0000–FFFF,
and batches bfchar entries at no more than 100. Empty input produces a valid
empty map without an empty bfchar batch.
These helpers do not select effective font axes, embed streams or decide remapping;
those belong to `00_nucleo/prompts/infra/export/builder/fonts.md`.

## Verification

Check escaping, ordered recursive collection, marks, ligatures, RTL clusters,
surrogate pairs, notdef and CMap validity. Extracted Unicode and positioned glyph
morphology are observables; subset sizes and exact GID assignment are not parity.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/fonts.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Recolher U+0020 de cada TextSeparator tanto no conjunto global como no fallback textual usado no subset e no ToUnicode.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

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

cluster_text não põe U+00AD autoral no mapeamento global por GID:
remover somente esse codepoint das substrings de cluster usadas para ToUnicode.
Omitir a entrada se sua substring não vazia consistia apenas em SHY; não
reservar o GID compartilhado com espaço com hex vazio. Manter fronteiras
UTF-8, marcas repetidas, ligaduras, RTL e pares substitutos existentes.
O texto lógico completo permanece no envelope por ocorrência do shaper.
A identidade não é inferida de avanço zero nem de um número específico de GID.
SHY em cluster compartilhado com letra também deve ser retirado do mapa
global, conservando a letra. Não apagar outros controles Unicode.
