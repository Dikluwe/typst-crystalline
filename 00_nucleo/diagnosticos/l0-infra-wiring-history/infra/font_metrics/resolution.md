# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/font_metrics/resolution.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `7cd0e57a09ca38c79963b74e97c8f72bdee9e4c42b5bfb6687794296d974d1f1`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — infra/font_metrics/resolution
Hash do Código: 114d9d3c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/font_metrics/resolution.rs`
**ADRs:** ADR-0019, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: a família resolution está concentrada no consumer
03_infra/src/font_metrics.rs, nas regiões registradas em infra/font_metrics.md.
Corpos e fixtures são preservados pela movimentação interna; a escolha da
organização não é observable da língua. Nenhuma medição nova de paridade é
inferida dos registros históricos contra pins anteriores.

## Decisão e contrato

Owner de FontCandidate e dos métodos resolve_primary,
resolve_primary_with_math_fallback, covering e resolve_font_combo sobre
FallbackFontMetrics. Candidatos internos conservam slot_idx e units_per_em.
Mantém ordem declarada de famílias; sem primária resolvida usa catálogo
serif/sans apropriado, terminando na primeira família resolvida do catálogo.
Math explícito injeta a cadeia MATH sem duplicar slots. covering conserva o
primeiro passe por face MATH com cobertura e depois a primeira primária que
cobre. Se nenhuma cobre, consulta World::candidates_for_char e escolhe por
FontBook::select_fallback com similaridade à primeira primária. Coverage
global é definitiva: carrega só a face vencedora, não revalida todos os slots.
Mantém propagação de None e ordem exatas do algoritmo existente.
resolve_font_combo conserva identidade da família, variante e variações
explícitas do estilo. Seleção não é inferida pelo nome textual math.
Aceitação: scoring, filtro de candidatos, bypass da varredura global quando
a cadeia math cobre e preferência MATH conservam os testes MockWorld.

## Linhagem e validação

Este é o único owner deste consumer. O módulo é privado ao hub; helpers
compartilhados têm visibilidade limitada ao pai, exceto reexports crate-internos
preexistentes. Os testes locais não alteram API nem defaults. Executar a suíte
font_metrics, cargo build e crystalline-lint .; preservar o nome e o corpo dos
testes deslocados, sem transformar estrutura Rust em critério de paridade.
