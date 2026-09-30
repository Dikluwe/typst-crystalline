# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/pipeline/raster.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `35234caf8719a919fca7530ebb271ea4a81143b9eb74f4149c4c11e3ffb7fe9b`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Pipeline — raster
Hash do Código: aff33680

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11

**Camada:** L3
**Consumer único:** `03_infra/src/pipeline/raster.rs`

## Medição anterior à individualização

Baseline: HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree
não commitado observado em 2026-09-26. Os corpos vigentes do pipeline
e a medição por função estão no diagnóstico
`00_nucleo/diagnosticos/pipeline-atomization.md`. A classificação é
reorganização interna: assinatura, defaults, fases, diagnósticos e
observáveis de linguagem são conservados. Não é medição nova de paridade.

## Contrato proprietário

Possui as fachadas PNG: compila pelo caminho paginado comum, resolve PPI documental/override uma vez, converte por 72, resolve fontes e exporta a primeira página com o fallback vazio vigente. Conserva erros, warnings e timings.

A fachada `00_nucleo/prompts/infra/pipeline.md` conserva os caminhos
públicos por reexport explícito. Helpers compartilhados têm visibilidade
`pub(super)`, limitada à família pipeline; nenhuma API externa é adicionada.
Os corpos são transferidos sem alterar algoritmo, fases ou políticas.
As medições e os rótulos históricos de gate nas seções transferidas
conservam a proveniência original; esta individualização não reabre gates
nem atribui nova conclusão de paridade. Referências ao antigo pipeline.rs
com números de linha são medições históricas anteriores à extração.

## P1463 — resolução de `format.png.ppi` e override CLI (MATERIALIZADO)

### Medição anterior à decisão

No baseline `23cd47e4ade4d6deb7095cb7400478feffb1e9f9`, todas as fachadas
PNG convergem para
`compile_to_png_bytes_with_timings_full_error_and_features`, que constrói
`RenderOptions::default()` separadamente nos ramos com e sem fontes. Logo a
pipeline usa sempre 2 pixels por ponto e não consulta
`document.format_options.png` nem caller externo.

No vanilla ratificado, o override CLI é convertido por `ppi / 72.0`, enquanto
o valor documental é entregue diretamente como pixels por ponto. Numa página
de 72 pt × 36 pt, documento `ppi: 72` produziu 5184 × 2592; CLI `--ppi 72`
produziu 72 × 36. Essa assimetria medida é contrato deste pin e não deve ser
normalizada por intuição de unidade.

### API e resolução

Adicionar a entry point pública explícita:

```rust
pub fn compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: Features,
    ppi_override: Option<f64>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings)
```

As fachadas PNG antigas preservam assinaturas e delegam `None`. A entry point
vigente com `features` também delega `None`; L4 chama a nova entry point.
Depois de obter o `PagedDocument`, resolver exatamente:

```text
Some(cli_ppi) -> cli_ppi / 72.0
None          -> document.format_options.png.ppi
```

Converter o resultado para `f32` somente ao construir um único
`RenderOptions { pixel_per_pt, render_bleed: false }`, usado identicamente nos
ramos `export_png` e `export_png_with_fonts`. Não recalcular default por ramo,
não consultar regra depois do documento e não fazer o documento vencer
`Some`.

Este fio aplica o Núcleo `format-option-precedence` pinado acima. A conversão
`/72` é específica do caller PNG e permanece neste prompt, não no Núcleo. O
exporter `infra/export/render.rs` continua ignorante de CLI/documento e não
muda. O caminho continua exportando somente a primeira página.

### Neutralidade e aceitação

PPI não altera eval, layout em pontos, PDF, SVG, HTML, `pretty`, `compact`,
tags, features, timings nem resolução de fontes. Testes de pipeline cobrem
default documental, documento 72/2/1.5, CLI 72/288, precedência explícita,
zero/negativo finito quanto à dimensão segura, ramos com/sem fontes e
neutralidade de outros targets. Não congelar bytes PNG nem alegar não finitos.

Os testes funcionais protegem a conversão documental, a divisão específica da
CLI, a precedência, o tratamento de `None`, os dois ramos de fontes e a
neutralidade dos outros targets.

A confirmação humana posterior ao commit causal P1463 foi recebida em
2026-09-19; a API de override parcial e a resolução PNG foram materializadas e
validadas.

## P1466 — estabilização PNG e preservação de `page-numbering` (PROPOSTA; AGUARDA GATE ADR-0127)

### Retificação da unidade PNG

A regra P1463 que entregava o PPI documental diretamente a
`RenderOptions::pixel_per_pt` fica superada. A semântica pública é pixels por
polegada para as duas origens e a pipeline resolve uma única vez:

```text
effective_ppi = ppi_override.unwrap_or(document.format_options.png.ppi)
pixel_per_pt  = effective_ppi / 72.0
```

O resultado convertido para `f32` alimenta o mesmo `RenderOptions` nos ramos
com e sem fontes. `144` documental e `--ppi 144` produzem as mesmas dimensões;
o override explícito continua vencendo. Uma página A4 default mede
`1191 × 1684` pixels, não `85720 × 121232`. Zero e valores negativos finitos
continuam sob a política segura já materializada no rasterizador.

### Valor contextual de `location.page-numbering()`

Em 2026-09-20, no pin `586e1bd43`, o probe com numbering closure e
`#context repr(here().page-numbering())` produziu `(..) => ..`. O cristalino
deve preservar `Numbering::Func` no snapshot injetado durante a estabilização
e devolver a própria `Value::Func` ao contexto; o conteúdo já realizado da
margem é uma vista separada e não substitui esse valor. Corrigir o primeiro
ponto de perda comprovado sem executar relayout adicional, duplicar callback
ou fabricar a string de repr.

Aceitação exige o callback visível na margem e sua repr contextual
`(..) => ..`, além dos quatro caminhos PNG que antes abortavam. Esta mudança
do default PNG requer confirmação posterior ao commit causal conforme
ADR-0127.
