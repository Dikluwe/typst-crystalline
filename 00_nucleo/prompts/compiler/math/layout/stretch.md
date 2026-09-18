# Prompt L0 — `math/layout/stretch` — layout de `MathStretch`
Hash do Código: NOVO-P1446

**Camada:** L1
**Alvo:** `01_core/src/compiler/math/layout/stretch.rs`

## P1446 — feature pública `math.stretch` (PROPOSTA; gate ADR-0127)

Este owner contém a free function da feature (forma B, ADR-0109). Recebe
`MathLayouter`, `MathStretchElem`, estilo e StyleChain causal; resolve o body
natural e escolhe o tamanho efetivo por argumento explícito, cascata
`math.stretch.size`, depois `100%`.

Aplica o alvo ao eixo extensível nativo do glifo, com short-fall zero. O
percentual referencia a dimensão natural máxima incluindo attachments e a
parte absoluta resolve no font-size causal. Delega seleção de
variantes/assembly e métricas ao owner `stretchy`; não replica tabelas MATH,
não escala continuamente a tinta e não desloca a feature para a entidade.

O wrapper preserva classes/fronteiras do body. Body não extensível conserva
seu layout natural e o diagnóstico ratificado aplicável. Aceitação cobre eixo
X, eixo Y, attachments, default, absoluto+misto, cascata, nesting e fallback.
