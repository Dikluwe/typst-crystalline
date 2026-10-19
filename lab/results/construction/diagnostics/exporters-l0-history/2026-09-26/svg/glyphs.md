---
Hash do Código: 26406d33
prompt: infra/export/svg/glyphs
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130
---

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c

# Prompt L0 — infra/export/svg/glyphs

## Medição anterior à decisão

Fonte anterior `03_infra/src/export/svg.rs:100-158, 229-237, 707-831, 1149-1284, 1783-1803`, HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, capturada em `2026-09-26T09:29:07-03:00`. Proveniência exata: `00_nucleo/diagnosticos/svg-atomization.md`. O texto histórico conserva medições e gates anteriores; não é nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo `03_infra/src/export/svg/glyphs.rs`. Possuir GlyphFontRequest e SvgGlyphFontContext, chave/contexto imutável, GlyphDefs, extração/cache de outlines e SvgGlyphPathBuilder, glifos diretos, texto shaped e writer textual HTML. Preservar fontes resolvidas, escala, inversão Y, advances/offsets, fill e fallback; não refazer shaping nem sondar fontes incidentais. Métodos de OutlineBuilder continuam métodos de trait, sem ampliar visibilidade. Os tipos públicos continuam reexportados na fachada.

Atomização estrutural reversível em L3: fluxo contínuo ADR-0127, sem mudança de contrato público, comportamento por defeito, fase ou compatibilidade. Referências ao monólito no texto histórico abaixo indicam a fonte anterior; a implementação específica passa a pertencer somente a este consumer. Não ampliar alegações de paridade nem interpretar mecanicamente novos IDs/bytes como paridade de linguagem.

## Glifos matemáticos diretos SVG

### Medição anterior à proposta

`FrameItem::Glyph` já transporta `pos`, `glyph_id`, `x_advance`, `size`,
`style` e `base_char`; `style.fill` já contém o paint. A pipeline resolve a
fonte efetiva com `FallbackFontMetrics::resolve_font_combo(base_char, style)` e
inclui o `FontKey` correspondente. O exporter recebe os bytes selecionados, mas
não a associação exata entre o glifo direto e a fonte resolvida. Como
`glyph_id` é relativo à face, sondar a primeira fonte que contém o número não
preserva identidade.

### Contrato proposto do owner SVG

- Estender o contexto explícito do exporter com uma associação imutável entre
  `GlyphFontRequest` normalizado e o `FontKey` completo resolvido pela pipeline.
  A chave inclui `base_char`, família solicitada, variante, variações e estado
  matemático relevante à resolução; não usa índice posicional, endereço, ordem
  de travessia ou identidade da ocorrência. Não alterar o variant público L1.
- Quando a associação e os bytes existirem, extrair o outline do `glyph_id` na
  face exata, aplicar escala `size / units_per_em`, posição e inversão Y uma
  única vez, e preservar `style.fill`.
- Reutilização em `GlyphDefs` inclui a identidade da fonte e o `glyph_id`; ids
  XML e ordem de `defs` são mecânica se a morfologia permanecer igual.
- O exporter confirma que o `FontKey` retornado existe na coleção fornecida e
  usa a posição encontrada somente localmente; reordenar a coleção não muda a
  fonte escolhida.
- Associação ausente, ambígua, face inválida, `units_per_em = 0` ou outline
  inexistente permanece `Unknown` e não produz glifo falso.
- `x_advance` é geometria de layout já consumida pelo posicionamento externo;
  o exporter não refaz shaping nem altera o cursor.
- Os wrappers sem fontes preservam o scope-out vigente. P1249 não autoriza
  fallback `<text>`, fonte do sistema, rede ou síntese a partir de `base_char`.

Contrato e ataques posteriores devem cobrir variante simples, assembly com
várias peças, duas fontes com o mesmo `glyph_id`, fill/alpha, transform, fonte
ausente e wrapper fontless.

