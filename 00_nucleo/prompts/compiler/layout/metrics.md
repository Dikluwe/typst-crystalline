# Prompt L0 — métricas injetáveis de layout
Hash do Código: cedc0f51

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/metrics.rs`

## Contrato e aceitação

FontMetrics fornece advance, métricas verticais, arestas e ink bounds com
TextStyle. FixedMetrics é fallback puro e determinístico; Length em edges é
resolvido no font-size. Detecção de scripts que exigem shaping é Unicode pura;
fontes reais permanecem em L3.
## P1493 — linha sem extensão do carrier HTML

### Medição anterior à decisão

`FontMetrics::line_content_right` em `metrics.rs:269-317` usa a posição do
primeiro filho de todo `Semantic`; logo o shape zero-size do carrier P1493
transformava um trailing space sem tinta em largura observável.

### Contrato ratificado

A medição de linha ignora exatamente o carrier privado P1493, inclusive quando
a posição local é diferente de zero. Itens reais e wrappers semânticos comuns
conservam integralmente as métricas existentes.
