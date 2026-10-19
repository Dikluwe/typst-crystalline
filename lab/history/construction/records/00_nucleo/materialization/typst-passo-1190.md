# P1190 — separar canvas de domínio e composição

**Estado:** EXECUTADO — GREEN EM 2026-08-25
**Baseline condicionado:** V15=17, V26=0; P1189 GREEN.

Individualizar `entities/page_canvas.md`: conservar como owner de
`entities/page_canvas.rs` e criar `compiler/layout/page_canvas.md` para o
consumer homônimo. Auditar e, confirmada a claim comum de geometria/canvas,
criar `_nuclei/layout/page-canvas.toml` consumido pelos dois L0s. Não alterar
tipos, layout ou contrato público.

Executar RED V15/V26 duplo; L0/Núcleo/pins primeiro; resselo restrito; exigir
V15 17→16, V26=0, ausência focal em V5, sources idênticos fora da linhagem,
testes focais, build, dry-run bloqueado por 16 V15, diff limpo e índice vazio.
Fechar em `diagnosticos/typst-p1190-saneamento-page-canvas.md`.
