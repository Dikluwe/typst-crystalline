# L0 — sequence
Hash do Código: 98ca32d3
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:363fc473f68d7a9faea59460d462eae77c4eb7c6df1360b91a30f6b6f0dd92ef


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/sequence.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/sequence.rs:201`, `01_core/src/compiler/layout/sequence.rs:583`, `01_core/src/compiler/layout/sequence.rs:805`, `01_core/src/compiler/layout/sequence.rs:24`, `01_core/src/compiler/layout/sequence.rs:36`, `01_core/src/compiler/layout/sequence.rs:65`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own ordered sibling iteration, lookahead/sticky handling, item grouping, block-margin collapse and vertical fractional preflight. Dispatch each actual child through the hub, retaining boundary and page events. SpaceFromNewline has the same width/transparency/grouping/closing behavior as Space; keep its Content origin and do not normalize CJK, show/regex, Empty, Parbreak or Styled into one category.

For a finite supported region, preflight the same sibling slice without emitting items, Locations or pages: reserve fixed items/gaps, exclude fractional Block body/intrinsic height, sum finite weights and distribute nonnegative remaining height proportionally; zero weight/sum is safe and does not divide. Save/restore the private concrete Block allocation and use the re-entry guard. Do not write allocations into entities.

Recognize a fractional Block through Styled and singleton-substantive Sequence wrappers iteratively; Space, SpaceFromNewline and Empty are transparent. Ambiguous/multiple substantive children are not flattened. Ordinary finite flow retains its existing base; continuous page columns use the remaining height from the entry cursor. Only the dedicated explicit-segment entry removes the segmented-column guard and uses the same cursor-relative regional base. Cell/subframe/note contexts and nested/multiregion distribution remain fail-closed outside the source's certified paths.

### Certified inline prefixes

A supported prefix is scanned from the existing sibling slice, with scalar metric/style accumulators rather than a speculative emission engine. Plain Text/space/empty and a direct Styled/Strong/Emph Text wrapper retain their current restricted style-delta gates. Cosmetic-only wrappers do not manufacture independent fixed lines; disallowed nested/compound geometry/style cases stay opaque. Leading, font/script/tracking/edge changes and arbitrary custom channels are not implicitly accepted.

Preserve the present size-only certifications: a single reduced text wrapper; a direct outer-sized fragment paired with a larger Styled(Text) in either order; two size-only wrappers in supported decreasing/equal-above-outer cases; and the exact certified single-wrap pairs. No-wrap certification requires the real injected-width gate, not a nominal fixture size. The fixed reserve accounts for the initial top edge, actual current-line maximum/descent, the selected next ascent/leading and the paragraph-to-block boundary consistently with `00_nucleo/prompts/compiler/layout/line_commit.md`. In the above-outer decreasing/equal cases, do not double-reserve a boundary that runtime already delivers.

The certified boundary belongs to the prefix/first-fractional-Block handshake and is applied once even if trailing whitespace already closed the line. Closing early does not lose the boundary, and ordinary/rejected prefixes acquire no new displacement.

A certified single wrap requires exactly two direct size-only Styled(Text) single-word fragments, a wrap at their boundary and a complete vertical reserve that fits. Preserve equal/unequal above-outer pairs, outer/larger pairs and below/above-outer pairs in both orders. Reject multiword, additional fragment, composed delta, nested wrapper, internal/multiple wrap or insufficient reserve. A first overlong word is deferred only until the second fragment proves the complete supported form; an incomplete overlong prefix remains opaque.

Record the existing wrap/Block indices and boundary. Request the certified line-close hooks before entering the second wrapper and before terminal whitespace/Block. Reserve per-line ascent/descent/outer-leading, not the global maximum twice. When the first overlong word closes inside text emission, arm the existing one-shot so that first close uses the certified formula. For a smaller singleton, retain its own baseline/next-ascent and only outer leading; when it is second, apply the source's single baseline correction before its wrapper. The hub stores the arm, `00_nucleo/prompts/compiler/layout/line_commit.md` consumes it, and `00_nucleo/prompts/compiler/layout/sub_frame.md` isolates it.

### Grouping and flow boundaries

Preserve sticky headings, list/enum/term grouping, loose-item gap reset and enum-counter lifetime according to the current iterator, without asserting that every Parbreak resets an enum. Keep block-chain below/above collapse and terminal fractional-tail marking; do not move Block's own frame-end or Place's own anchor into a global scan. Direct private HTML markers do not create a pending material line/gap/wrap/flush; wrappers retain their existing materiality.

The certified matrix remains a bounded compatibility subset, not a full flow simulator or arbitrary fraction-parity claim. Collected structs, algorithms and exported bytes may differ from vanilla; observable content, order, pages, positions and styles may not be redefined by this documentation.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Preparação e seletores — 2026-09-28

### Medição anterior à decisão

O lote bilateral `00_nucleo/diagnosticos/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` mede 32 entradas, com estado exato em initial.json e before/, contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Em 140pt/margem 8pt, simple escolhe delta epsilon zeta eta, optimized escolhe delta epsilon zeta e junta eta à linha seguinte; Tekt ignora a estratégia. Em 240pt justificado, optimized admite eta na primeira linha reduzindo os espaços. Fonte: eval/rules.rs:3070-3100 e stdlib/structural/par.rs:131-150 validam e descartam linebreaks; text.rs:41 e word_layout.rs:29 escolhem incrementalmente; vanilla inline/mod.rs:194-196 resolve auto pela justificação, linebreak.rs:raw_ratio/raw_cost aplica redução até 1/3 e custos cúbicos/quadráticos; model/par.rs:Limits::SPACING_DEFAULT fixa 2/3..1.5. Estilos adjacentes, literal leading/trailing e conteúdo opaco estão medidos separadamente. Observação não prova intenção histórica; inferência refutável: os scope-outs L0 explícitos de preparação/optimized legitimaram o adiamento, pois ainda vedam a implementação global apesar do transporte aceito.

### Decisão e escopo

Correção interna de paridade ADR-0127: sem novos contratos públicos, flags ou fase. Primeira preparação global certificada: parágrafo latino composto por Text, Space, SpaceFromNewline, Empty, Sequence, Styled tipográfico e Strong/Emph, com hifenização desligada, direção LTR, sem tracking/math/smallcaps, largura finita e constante, fora de colunas/células/floats. O conjunto completo deve ser certificado antes de emitir; uma unidade recusada preserva o caminho anterior. Hifenização, Unicode UAX14 completo, links/objetos/linebreaks, largura variável, limites customizados e seleção incremental restante continuam explicitamente incompletos; este lote não declara estado da arte nem otimização completa da língua.

Antes de layout_parts, tentar o conjunto inteiro pelo owner paragraph_plan. Se recusado, salvar guard, suprimir planejamento de fragments durante layout_parts e restaurar; preservar toda coordenação existente e fractional preflight. Não atomizar por import/dyn.

### Entrada terminal sob configuração global

O lote after-pdf inicial (after-first.log e PDF) mostra que native já escolhe as linhas optimized, mas #set ainda coincide com greedy: o Sequence exterior contém configuração opaca, recusa preparação e suprime inclusive o wrapper terminal que cobre todo o texto. Medição de sequence.rs:943/975 mostra a entrada real no último sibling. Decisão: antes de despachar o último sibling substantivo, se não há linha material pendente, permitir nova tentativa sob seus estilos; siblings finais Space/SpaceFromNewline/Empty não acrescentam conteúdo. Restaurar o guard imediatamente depois do dispatch. Não liberar fragmento que tenha texto posterior ou prefixo material; preservar scopes e preflight fracionário.

O planner do conjunto pode certificar todos os segmentos separados por Parbreak explícito antes da emissão. Esta cláusula não permite segmentar por conteúdo opaco nem alterar a iteração/preflight dos caminhos recusados.

## Refinamento de escopo integral e fronteira explícita

### Medição anterior à decisão

first-round/acceptance.json registra 14 falhas nas 397 entradas: negative/native/empty-fold/restore/plain-disabled e regressões lexicais passam a planejar antes do wrapper integral de Styles, lendo custos exteriores e usando font/leading exteriores. Fonte: paragraph_plan.rs admite default Simple antes da descida; paragraph_prepare percorre Styled restaurando a chain; sequence.rs:947-956 reabilita somente o último sibling substantivo. terminal-break conserva 3+1 em vez de 2+2 porque o último sibling é Pagebreak e o texto anterior permanece suprimido. Isto refuta a admissão antecipada; não é autorização para promover um fragmento parcial a escopo compartilhado.

### Decisão

Quando body, ou segmento completo entre Parbreaks, tem exatamente um sibling substantivo Styled (outros são Empty/Space/SpaceFromNewline), recusar a tentativa exterior e deixar style_scope entrar normalmente. Sem alterar chain/Style por antecipação, sem deduzir configuração por glyph e sem emitir metade de um conjunto recusado. Após a entrada, tentar novamente sob o escopo correto pelo dispatch existente. Preservar overrides inline parciais.

No owner Sequence, também reabilitar a tentativa quando o sibling seguinte substantivo é Pagebreak e a linha atual não tem material. Esta é uma fronteira explícita do fluxo: o sibling corrente pode fornecer o parágrafo completo antes da quebra; prefixos já materiais continuam suprimidos. Não segmentar por conteúdo opaco genérico, não alterar preflight de blocos/notas/colunas nem o despacho de pagebreak. Testar restauração/ratios desativados e repetir corpus lexical, com os mesmos inputs e expectativas.

### Reentrada antes de Parbreak medida antes de corrigir

second-round/acceptance.json registra quatro falhas: os dois controles additional-scopes/positive-first (adicionados depois da implementação, sem medição Tekt prévia) e as regressões costs/restore e overhang/restore. Positive-middle já coincide. O primeiro wrapper integral antes de Parbreak continua suprimido porque não é terminal e o guard novo só reconhece Pagebreak; o fluxo posterior entra no escopo normalmente. Parbreak é uma fronteira explícita de parágrafo com a mesma evidência de fechamento necessária. Estender a reabilitação do guard ao próximo sibling substantivo Parbreak, além de Pagebreak, sempre com current_line sem material. Não abrir tentativa antes de sibling inline ou conteúdo opaco genérico. A certificação do parágrafo continua inteira; a iteração apenas restaura possibilidade de tentativa em cada fronteira material de parágrafo. Manter inputs e expectativas dos quatro controles e repetir regressões após rebuild.
