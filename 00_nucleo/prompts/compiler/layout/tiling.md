# Prompt L0 — `compiler/layout/tiling` — materialização privada de padrões
Hash do Código: b68ec646

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/tiling.rs`
**Origem:** P1245/P1254
**ADRs:** ADR-0026, ADR-0029, ADR-0107, ADR-0109, ADR-0127, ADR-0129

## Medição anterior à decisão

O `Tiling` anterior conserva apenas `TilingBody` fechado e os exporters recebem
esse valor ainda declarativo em `Paint::Tiling`. O vanilla ratificado
`a51e02804`, `crates/typst-library/src/visualize/tiling.rs:99-185,278-376`,
executa layout do corpo uma vez, deriva `size:auto`, resolve offset contra
`size + spacing` e aplica offset antes de angle. O `Layouter` cristalino já
oferece `layout_sub_frame`, e `FrameItem::Group` já transporta recorte,
transformação e filhos pelo pipeline normal. Esta é medição de semântica e
morfologia, não obrigação de copiar o `Frame` vanilla (ADR-0107).

## Decisão

- `compiler/layout/tiling.rs` é o único owner da materialização privada.
- `shape.rs` mantém o despacho estático e delega a uma free function deste
  módulo, conforme a forma B da ADR-0109.
- O corpo declarativo é layoutado exatamente uma vez em subframe. `size:auto`
  deriva a extensão desse resultado; size explícito fixa a célula.
- Pitch é `size + spacing` por eixo. Offset relativo resolve contra o pitch;
  angle finito é aplicado depois do offset.
- A saída usa somente `FrameItem::Group`/`Shape` existentes: grupos de célula
  repetidos e recortados pela geometria do fill. Não existe `ResolvedTiling`
  público, novo `FrameItem`, I/O, rasterização ou layout no exporter.
- `relative:auto` permanece contextual: self em shapes; texto e parent ficam
  fora deste consumer até seus owners fornecerem contexto equivalente.
- Fill é materializado sem manter `Paint::Tiling` no shape de fundo. Stroke
  tiling que exija recorte por contorno permanece `Unknown` explícito; não é
  promovido por sucesso de fill.
- Geometria, pitch ou tamanho não finitos/não positivos mantêm o paint como
  `Unknown` e nunca entram em loop de repetição.

## Testes

- body arbitrário é layoutado uma vez e repetido por pitch assimétrico;
- `size:auto` deriva extensão do subframe;
- offset resolve percentuais contra pitch e angle sucede offset;
- fill gera grupos recortados e remove fallback representativo;
- entradas inválidas não repetem nem promovem paridade;
- ausência de suporte de stroke permanece separada do fill.
