# P544 — Métricas reais de fonte na pipeline de produção

## Resumo

Alterou-se a pipeline de produção para usar métricas de fonte reais
(`FallbackFontMetrics` em L3) em vez de `FixedMetrics`. O objectivo era
corrigir a junção/corte de palavras e validar a contagem de `lorem()`.

Foram necessárias três alterações coordenadas:

1. **Interface `FontMetrics` em L1** passa a receber `&TextStyle`, para
   que implementações com fallback multi-script saibam quais fontes
   primárias resolver.
2. **Entry point genérico** `layout_with_introspector_and_metrics` em L1,
   permitindo injectar métricas e image-sizer na pipeline.
3. **`FallbackFontMetrics` em L3**, que mede cada string com a face real
   que o shaper/PDF usará, incluindo o fallback padrão do shaper
   (`DejaVu Sans`, `Noto Sans`, etc.).
4. **Correcção no layout de `Text`** em L1: o lexer agrupa palavras
   separadas por um único espaço num só token `Text`; o layout passou a
   inserir `space_width()` entre palavras em vez de descartar os
   separadores via `split_whitespace()`.

## Ficheiros alterados

- `01_core/src/rules/layout/metrics.rs` — trait `FontMetrics` recebe
  `&TextStyle`; `FixedMetrics` é `Clone + Copy`.
- `01_core/src/rules/layout/mod.rs` — novo entry point genérico; todos os
  callers de `advance` actualizados; `layout_with_introspector` mantém
  `FixedMetrics` para compatibilidade de testes.
- `01_core/src/rules/layout/text.rs` — preserva espaços entre palavras.
- `01_core/src/rules/layout/cursor.rs`, `enum_item.rs`, `equation.rs`,
  `link.rs`, `list_item.rs` — callers de `advance` actualizados.
- `01_core/src/rules/math/layout/mod.rs` — caller de `advance`
  actualizado.
- `01_core/src/entities/image_sizer.rs` — `NullImageSizer` é `Copy`.
- `03_infra/src/font_metrics.rs` — implementação de
  `FallbackFontMetrics` com fallback alinhado ao shaper.
- `03_infra/src/image_sizer.rs` — `ImageSizeImageSizer` é `Copy`.
- `03_infra/src/pipeline.rs` — produção usa
  `layout_with_introspector_and_metrics(FallbackFontMetrics,
  ImageSizeImageSizer)`.
- `03_infra/fixtures/p307b/reference/*.pdf` — snapshots binários
  actualizados (output PDF mudou intencionalmente).

## Casos de validação

### Casos mínimos

```typst
// /tmp/p544/minimo.typ
commodo consequat
```

Antes: `commodoconsequat` (junção).  
Depois: `commodo consequat` (correcto).

```typst
// /tmp/p544/corte.typ
T exto
```

Nota: `pdftotext` reporta `"T exto"` devido ao kerning forte do par
`Te` em DejaVu Sans; visualmente as letras estão correctas (sem corte
real).

### `lorem(1200)`

Antes: múltiplas junções (`consecteturadipiscing`,
`exercitationullamco`, `reprehenderitin`, `Excepteursint`,
`occaecatcupidatat`).  
Depois: todas as palavras separadas; a contagem de 1200 palavras
mantém-se correcta.

### Texto misto

```typst
Texto em português com código: `fn main()` e matemática $x = 1$.
```

Visualmente as palavras do markup estão separadas. Espaços extra
ao redor do bloco `raw` e representação da matemática são problemas
preexistentes fora do âmbito de P544.

## Performance

Medição manual para `/tmp/p544/lorem.typ` (`#lorem(1200)`):

| Compilador | Tempo médio |
|------------|-------------|
| Cristalino | ~0,70 s |
| Vanilla    | ~0,13 s |

Timings internos (cristalino):

```json
{"eval_ms":0.35,"introspect_ms":0.004,"expand_context_ms":0.0003,
 "layout_ms":355.6,"shape_ms":186.1,"subset_ms":0.5,
 "render_ms":2.7,"total_ms":545.2}
```

A maior parte do tempo passou a estar em `layout_ms` (65 %) e
`shape_ms` (34 %). O `FallbackFontMetrics` resolve fontes a cada
palavra; esta é a primeira versão funcional e pode ser optimizada no
futuro (cache de candidatos, medição por run em vez de por caractere).

## Qualidade de build

- `cargo test --workspace` — passa (580 tests).
- `crystalline-lint .` — zero violations.

## Conclusão

A migração para métricas reais de fonte corrigiu as junções de palavras
em produção. A chave não foi apenas substituir `FixedMetrics`, mas
alinhá-lo com a mesma face de fallback que o shaper/PDF usa
(`DEFAULT_FALLBACK_FONTS`), e corrigir o layout de `Text` para não
perder os espaços interiores que o lexer agrupa num único token.
