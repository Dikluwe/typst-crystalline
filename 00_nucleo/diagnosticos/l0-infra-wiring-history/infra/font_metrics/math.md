# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/font_metrics/math.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `fcc10d4f701a0106e290629dbe0b5f25cb1a8779a239061d067b366b776c7454`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — infra/font_metrics/math
Hash do Código: c05b0304

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/font_metrics/math.rs`
**ADRs:** ADR-0019, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: a família math está concentrada no consumer
03_infra/src/font_metrics.rs, nas regiões registradas em infra/font_metrics.md.
Corpos e fixtures são preservados pela movimentação interna; a escolha da
organização não é observable da língua. Nenhuma medição nova de paridade é
inferida dos registros históricos contra pins anteriores.

## Decisão e contrato

Owner de extract_variants, extract_assembly e seus pares horizontais,
build_math_glyph_reverse_map, math_constants_from_face, math_kern_from_face,
ssty_eligible_text, ssty_level_of, ssty_substitute e is_variation_selector.
Converte todos os campos MATH já extraídos para os tipos neutros L1, mantendo
design units; percentagens de script e radical são razões divididas por 100.
Ausência de tabela preserva fallbacks L1; zero presente não é ausência.
Variants conservam advance do eixo de esticamento, hor_advance de hmtx e
attachment horizontal da tabela ou (hmtx + IC)/2. Assembly conserva conexões,
extensores, hor_advance de hmtx e min_connector_overlap da tabela MATH.
Kern preserva os quatro quadrantes e n alturas para n+1 valores; último
record tem correction_height None. Reverse map prefere codepoint real cmap,
preserva primeira base para glifos compartilhados e usa fallback de base,
barra vertical ou underscore quando falta codepoint. Inclui operadores grandes.
ssty é elegível para um carácter ou número ASCII com pelo menos um dígito e
no máximo um ponto. Só Script/ScriptScript em math e fora de TextItem usam
níveis 1/2; lookup/alternante ausente mantém glifo-base. Não duplica o predicado
canónico consumido pelo shaper. Nenhum tipo ttf-parser escapa para L1.
Aceitação: reverse map real e distinção TextItem/ssty preservam os testes;
leituras de variantes, assembly, constantes e kern são exercidas pelo backend
de fallback em infra/font_metrics/fallback.md.

## Linhagem e validação

Este é o único owner deste consumer. O módulo é privado ao hub; helpers
compartilhados têm visibilidade limitada ao pai, exceto reexports crate-internos
preexistentes. Os testes locais não alteram API nem defaults. Executar a suíte
font_metrics, cargo build e crystalline-lint .; preservar o nome e o corpo dos
testes deslocados, sem transformar estrutura Rust em critério de paridade.
