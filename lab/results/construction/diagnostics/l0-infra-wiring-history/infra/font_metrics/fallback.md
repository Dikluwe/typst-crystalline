# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/font_metrics/fallback.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `d412c4bf5be17f3c06001dd9f39066eacef9ce89a4f2d6e69028ad1bc1f0d569`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — infra/font_metrics/fallback
Hash do Código: 7d3faba1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/font_metrics/fallback.rs`
**ADRs:** ADR-0019, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: a família fallback está concentrada no consumer
03_infra/src/font_metrics.rs, nas regiões registradas em infra/font_metrics.md.
Corpos e fixtures são preservados pela movimentação interna; a escolha da
organização não é observable da língua. Nenhuma medição nova de paridade é
inferida dos registros históricos contra pins anteriores.

## Decisão e contrato

Owner do impl FontMetrics para FallbackFontMetrics e de face_kerning.
Mantém medição carácter a carácter na face resolvida, clonando a face para
aplicar variações de TextStyle sem mutar a face cacheada. Mantém kerning
legacy kern seguido de kerx somente intra-fonte; não adiciona GPOS aqui.
advance_shaped delega ao shaper e usa a cache de faces compartilhada.
Resolução e caches delegam a infra/font_metrics/resolution.md e
infra/font_metrics/caches.md. Mede advance, métricas verticais, cap-height,
edges, ink bounds assinados e não negativos e bbox por glyph-id mantendo os
mesmos candidatos, unidades, fórmulas e fallbacks FixedMetrics/0.6 atuais.
IC e ssty respeitam math_text_item e o predicado proprietário de
infra/font_metrics/math.md; não remapeia codepoint de ink pelo estilo math.
Constantes usam a primeira face primária com MATH/constants; kern, variantes,
assemblies, IC e attachments mantêm a face ativa selecionada e os fallbacks
existentes. Ausência de fontes preserva os defaults atuais.
Aceitação: leituras MATH reais, kerning, shaping árabe e variações de peso
conservam os testes. Nenhum trait, assinatura, fonte padrão ou fase muda.

## Linhagem e validação

Este é o único owner deste consumer. O módulo é privado ao hub; helpers
compartilhados têm visibilidade limitada ao pai, exceto reexports crate-internos
preexistentes. Os testes locais não alteram API nem defaults. Executar a suíte
font_metrics, cargo build e crystalline-lint .; preservar o nome e o corpo dos
testes deslocados, sem transformar estrutura Rust em critério de paridade.
