# L0 — tracking
Hash do Código: 3dd060a6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/tracking.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência anterior à decisão

A inspeção de `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` e `space.rs:162` encontrou respectivamente fronteira textual, admissão de baseline e teste de integração de espaços iniciais em owners de outras responsabilidades. O estado exato, fontes e prompts anteriores estão em `lab/results/construction/diagnostics/separacao-tracking-2026-09-29/initial.json` e `before/`. A correção já medida encontra-se em `lab/results/construction/diagnostics/correcao-tracking-2026-09-29.md`; esta separação conserva seus resultados e limites. Posições, texto e estilos são observáveis da língua; localização das funções é mecânica. A inspeção não demonstra intenção histórica nem nova paridade bilateral.

## Contrato vigente

Possuir exclusivamente residual(glyphs,index,style,upem) e width derivados, privados à crate. residual retorna zero sem glyph atual/próximo, em cluster igual, tamanho não positivo/não finito ou tracking ausente/zero/não finito. Entre clusters diferentes, resolver tracking em pontos e subtrair sua representação arredondada em i32 reconvertida para pontos; upem usa no mínimo 1. Preservar ordem das operações e semântica de casts.

width soma avanço de cada glyph convertido em pontos e residual desse índice. Não adquirir fontes, modificar glyphs, cachear, escolher fronteiras entre FrameItems ou emitir PDF. O mesmo cálculo deve alimentar medição, reconciliação e posições do PDF, sem Tc adicional ou novo contrato Glyph. Conservar teste de resíduo positivo/negativo, cluster interno e último glyph.

## Aceitação e limites

Preservar os testes causais existentes e o texto, posições, estilos, páginas e diagnósticos dos controles anteriores à separação. Ownership 1:1, pins válidos, build e lint sem erros. Movimentação interna ADR-0109/0127: funções estáticas descendentes, sem estado novo. As limitações de tracking em estilos mistos, RTL, seleção global e contagem de clusters permanecem as registradas no diagnóstico da correção.
