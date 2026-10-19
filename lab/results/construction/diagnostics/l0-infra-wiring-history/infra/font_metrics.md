# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/font_metrics.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `d1f4ee0fbad5a83bfbbcb28338f1e07a7a533debf6544ff169e552fa5ce27626`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — infra/font_metrics — composição dos backends de métricas
Hash do Código: d87e162e

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/font_metrics.rs`
**ADRs:** ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: font_metrics.rs:31-294 contém extratores MATH e reverse map;
:301-750 contém métricas de face única e GSUB ssty; :760-923 contém constantes
e kern; :931-1206 contém faces/chaves/caches; :1212-1393 resolve fontes;
:1431-2005 implementa métricas de fallback; :2007-3456 agrega testes dessas
famílias. A organização física é mecânica, não observável pela linguagem.
Inventário da árvore e baseline pertencem ao diagnóstico de atomização.

## Decisão e contrato proprietário

O hub declara basic, math, fallback, resolution e caches como descendentes
privados. Cada consumer tem exatamente um owner sob infra/font_metrics/.
FontBookMetrics e FallbackFontMetrics mantêm nomes, lifetimes e campos privados
aqui; os módulos descendentes implementam comportamento sobre esses tipos.
Não cria structs públicas, métodos públicos, traits, flags ou fases.

from_bytes conserva parse no índice 0, None para bytes inválidos, upem zero
protegido por 1000 e reverse map precomputado pelo owner
infra/font_metrics/math.md. new(world) conserva World emprestado e caches
lazy em Arc<Mutex<_>>. Clone conserva World e compartilha todas as caches,
incluindo a cache de faces do shaper. Não altera ordem ou políticas dos caches.

Preserva os caminhos crate-internos existentes por reexports de
edge_offset_pt (infra/font_metrics/basic.md), ssty_eligible_text e
build_math_glyph_reverse_map (infra/font_metrics/math.md). Esses helpers
continuam pub(crate), os demais módulos e helpers são internos ao hub.
ttf-parser e rustybuzz permanecem L3; o trait FontMetrics e seus tipos neutros
L1 não mudam. Comportamentos específicos são legitimados exclusivamente por
infra/font_metrics/basic.md, infra/font_metrics/math.md,
infra/font_metrics/fallback.md, infra/font_metrics/resolution.md e
infra/font_metrics/caches.md.

## Aceitação

Constructores conservam comportamento e os testes existentes. As famílias
preservam fórmulas, unidades, assinaturas, ordem de fontes, GSUB ssty,
proveniência TextItem e resultados de fixtures. Os testes ficam junto à
responsabilidade dona. cargo build e crystalline-lint . fecham com zero erros.
Histórico anterior é diagnóstico em
00_nucleo/diagnosticos/font-metrics-l0-before-atomization.md.
