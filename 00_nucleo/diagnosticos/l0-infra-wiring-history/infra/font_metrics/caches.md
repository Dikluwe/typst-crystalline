# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/font_metrics/caches.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `260cdf95d744c68f38b7462e837b2a670b561436220e6bb96bcc6c9093aa966f`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — infra/font_metrics/caches
Hash do Código: 9626f785

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/font_metrics/caches.rs`
**ADRs:** ADR-0019, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: a família caches está concentrada no consumer
03_infra/src/font_metrics.rs, nas regiões registradas em infra/font_metrics.md.
Corpos e fixtures são preservados pela movimentação interna; a escolha da
organização não é observable da língua. Nenhuma medição nova de paridade é
inferida dos registros históricos contra pins anteriores.

## Decisão e contrato

Owner de CachedFace, ShapedWidthKey, AdvanceWidthKey e dos métodos
shaped_width_key, cached_shaped_width, advance_width_key,
cached_advance_width e cached_face sobre FallbackFontMetrics.
CachedFace mantém bytes Font owned junto ao Face parseado em Arc; o slice
static é apenas interno e o empréstimo devolvido por face() fica limitado
ao self. Preserva o unsafe e sua justificativa; bytes não são liberados
enquanto a face vive. Cache de faces é lazy por slot_idx e compartilhada
pelos clones do hub. Falha de leitura/parse não insere entrada.
Chaves conservam texto, bits de tamanho, família, bold, italic, peso,
direção, língua e hash de variações. Advance também distingue math,
math_text_item e math_size. Não acrescenta campos nem muda identidade.
Shaped com tracking bypassa cache; computação None não é inserida.
Mantém escopo e ordem dos mutexes, leitura antes de compute e inserção
depois, sem estado global. Não altera cache política, invalidação ou lifetime.
Aceitação: chaves distinguem variações e proveniência TextItem; cálculo e
reuso dos backends mantêm resultados e compartilhamento entre clones.

## Linhagem e validação

Este é o único owner deste consumer. O módulo é privado ao hub; helpers
compartilhados têm visibilidade limitada ao pai, exceto reexports crate-internos
preexistentes. Os testes locais não alteram API nem defaults. Executar a suíte
font_metrics, cargo build e crystalline-lint .; preservar o nome e o corpo dos
testes deslocados, sem transformar estrutura Rust em critério de paridade.
