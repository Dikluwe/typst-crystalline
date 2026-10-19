# `sec_22.typ`: delimitadores explícitos de `lr`

**Resultado.** A altura automática da página agora coincide com o vanilla pinado: 244,595 × 334,236 pt em ambos. A 120 dpi, resta uma diferença de apenas um pixel de intensidade 4/255 (zero pixels com diferença acima de 20). As oito fórmulas isoladas coincidem em dimensões; seis que já coincidiam mantiveram suas dimensões. Os PDFs, rasters e valores completos estão em `results.json` e neste diretório.

## Medição antes da decisão

A matriz anterior de `00_nucleo/diagnosticos/typs-teste-2026-09-30/` media `sec_22.typ` com Tekt 5,687 pt mais curto e 1,168% dos pixels acima do limiar 20. A sonda isolada desta frente, antes do ajuste, mostrou duas divergências:

| Fórmula | Tekt anterior | Vanilla |
|---|---:|---:|
| `lr((a/b))` | 21,153 × 20,273 pt | 22,605 × 23,012 pt |
| `lr(|a/b|)` | 15,345 × 19,976 pt | 15,345 × 22,924 pt |

As outras seis fórmulas de `sec_22.typ` já tinham dimensões iguais. A fração isolada também coincidia. Na fonte efetiva NewCMMath-Book, a largura 6,567 pt do parêntese anterior correspondia à variante v4 (597 du a 11 pt), enquanto 7,293 pt no vanilla correspondia à v5 (663 du). A barra conserva a largura horizontal em variantes diferentes, mas a variante anterior era mais curta. Isso localizou a escolha de variante, não o espaçamento global nem o exportador PDF.

O vanilla pinado em `lab/typst-original/crates/typst-layout/src/math/fenced.rs:84` deriva `relative_to` da extensão equilibrada `2*max(ascent-axis, descent+axis)` quando o grupo é balanced. Em Tekt, `01_core/src/compiler/math/layout/delimited.rs` usava essa extensão em grupos implícitos, mas o ramo explícito `layout_delimited_sized` passava a altura bruta da fração. A inferência causal foi refutada/confirmada com uma fonte sintética cujo limiar de variante fica entre as duas medidas: antes do código, o teste RED escolheu `[501, 501]` para `lr` e `[502, 502]` para grupo implícito.

## Correção

O L0 de `compiler/math/layout/delimited.md` e o L0 de `compiler/math/layout/tests.md` foram atualizados antes do código. Um helper privado em `delimited.rs:66` calcula a extensão do corpo para os dois caminhos; preserva a exceção existente do binomial sem barra, que usa a altura bruta. `layout_delimited_sized` agora aplica o tamanho de `lr` à extensão equilibrada. Não houve campo, método de trait, assinatura pública, fase de pipeline ou novo modo padrão: correção interna de paridade em fluxo contínuo segundo ADR-0127. O owner de `delimited.rs` foi ressellado isoladamente (`scoped-seal.json`); o par `tests.rs`/`tests.md` fica para resselo conjunto pelo integrador, pois outra frente também o editou.

## Verificação e proveniência

O teste `lr_fracao_usa_alvo_equilibrado_como_grupo_implicito` falhou RED e passou GREEN; `cargo build -q` passou. Na sonda após build, `lr((a/b))` passou a 22,605 × 23,012 pt e `lr(|a/b|)` a 15,345 × 22,924 pt, iguais ao vanilla. `sec_22.typ` completo ficou com raster praticamente igual, além da geometria da página igual. As seis fórmulas de controle retiveram as medidas anteriores.

Medição após correção: **2026-09-30T13:20:57.902351+00:00**, HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitada e compartilhada. `working-tree-stat.txt` e `working-tree-status.txt` guardam a lista do estado no momento da medição. SHA-256 do Tekt debug: `9dd59a1e27991365b417c98d0b042f3819421a891a8b1c1393b8d47ad590f6f8`; vanilla pinado: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. `results.json` guarda também SHA-256 da entrada e dos PDFs, dimensões, exits e dados dos pixels. O vanilla é o binário ratificado do pin `9f2b6e8715237cb086899a42873660fe744622e8`.
