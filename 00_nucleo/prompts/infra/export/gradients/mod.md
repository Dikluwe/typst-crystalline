# Prompt L0 — `infra/export/gradients/mod` — Tipos + scan + dispatch
Hash do Código: 6ec5e88f

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/gradients/mod.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0087 (gradient linear), ADR-0088 (radial), ADR-0089/0090 (conic), ADR-0091 (gradient space), ADR-0095 (dedup Arc::as_ptr)

---

## Contexto

Ponto de entrada do submódulo `gradients/`. Concentra:
- **Tipos partilhados**: `PatternRef`, `GradientObjectKind`, `GradientObject`, `RectKey`, `DedupKey`.
- **Helpers genéricos**: `rect_to_key`, `dedup_key_for`, `group_bbox_from_fields`.
- **Walkers top-level**: `scan_all_gradients`, `pattern_resources_for_page`.
- **Re-exports** das funções dos submódulos (`linear`, `radial`, `cmyk`, `relative`, `adaptive`, `conic`, `function_dict`) — fachada do cluster.

## Restrições estruturais

- L3 puro. Não FS.
- Tipos `pub(super)` ou `pub(crate)` conforme superfície de uso por outros submódulos.
- `DedupKey` é `pub(crate)` (atravessa pelo type system até `super::stream`).
- Re-exports `pub(super)` para expor a API plana do cluster a `super` (i.e. `export/`).

## Submódulos declarados

```rust
mod adaptive;
mod cmyk;
mod conic;
mod function_dict;
mod linear;
mod radial;
mod relative;
```

Cada submódulo tem L0 dedicado em `00_nucleo/prompts/infra/export/gradients/{name}.md`.

## Interface (re-exports)

`pub(super) use self::adaptive::{adaptive_n_for_stops, perceptual_distance_in_space};`
... (idem para cada submódulo)

## Walkers (scan_all_gradients, pattern_resources_for_page)

Padrão recursivo em `FrameItem::Group` (classe A pós-P273.10).
Bug latent pré-P273.10: gradients dentro de Group não eram registadas.

## Critérios de verificação

- `scan_all_gradients(doc, first_id)` produz IDs sequenciais sem colisão.
- `dedup_key_for(g, bbox)` é determinístico para mesmo `(g, bbox)`.
- Walker captura gradients em `Block` (clip) e `Transform` Groups.

## P1483 — registro de gradientes de preenchimento

### Medição anterior à decisão

Em `scan_all_gradients`/`pattern_resources_for_page` o walker
atual só reconhece `Shape.stroke: Paint::Gradient`
(`gradients/mod.rs:174-236,291-320`). A sonda processual
`gradient.linear(red, blue)` em fill no cristalino pinta sólido;
o vanilla pinta gradiente. O gradiente spot vanilla mantém
`/Separation` no shading linear/radial/conic; o cristalino
pinta sólido e não registra pattern de fill.

### Decisão interna

Registrar e declarar também `Shape.fill: Paint::Gradient`
com a mesma DedupKey de stroke e bbox efetivo; se fill e
stroke partilharem a mesma instância, não duplicar pattern.
Recursão em Group/Link/Semantic permanece, inclusive no walker
de recursos por página. Varredura e recursos incluem as três
camadas desenhadas pelo stream: background, items e foreground.
Para gradientes spot, o walker captura também o retângulo do
shape pintado (coordenadas do frame), separado do bbox do
contentor usado por `relative: parent`. O builder usa esse
retângulo para posicionar shading e pattern; não muda o
posicionamento existente dos gradientes de processo. Reuso
do mesmo gradiente em shapes distintos deve manter cada
posição própria. Nenhum novo owner de dados nem alteração
de assinatura pública.
