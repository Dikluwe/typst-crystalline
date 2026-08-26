# Prompt L0 — métricas injetáveis de layout
Hash do Código: fafc10f7

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/metrics.rs`

## Contrato e aceitação

FontMetrics fornece advance, métricas verticais, arestas e ink bounds com
TextStyle. FixedMetrics é fallback puro e determinístico; Length em edges é
resolvido no font-size. Detecção de scripts que exigem shaping é Unicode pura;
fontes reais permanecem em L3.
