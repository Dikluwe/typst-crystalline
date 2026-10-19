# Materialização — Passo 1117: Paridade da Secção 36

## Arquivos Modificados
1. `03_infra/src/export/stream.rs`: Emissão de `-matrix.d` em `FrameItem::Group` no exportador PDF.
2. `01_core/src/compiler/layout/transform.rs`: Pivô central $T(w/2, h/2) \cdot M \cdot T(-w/2, -h/2)$ e supressão de quebra forçada `flush_line()`.

## Resultados
- Sobrescrito/subscrito e limites de integrais corrigidos e validados.
- Suíte total do workspace: 5.961 testes OK.
