# Prompt L0 — `infra` — raiz da crate `typst-infra`
Hash do Código: 4cc717a7


**Camada:** L3
**Ficheiro proprietário:** `03_infra/src/lib.rs`
**ADRs:** ADR-0129 e topologia cristalina vigente

## Responsabilidade

Este owner especifica exclusivamente a raiz estática da crate `typst-infra`.
O hub torna públicos os módulos L3 existentes:

```text
embedded_fonts, export, fallback_fonts, font_metrics, font_variant, fontdb,
fonts, image_sizer, layout, layout_bidi, measurements, package_downloader,
pipeline, plugin_host, project_init, query_helpers, runtime_info, shaper,
watch, world
```

A raiz também inclui `integration_tests` e `p307b_snapshot_tests` como módulos
privados somente sob `#[cfg(test)]`.

## Fronteira arquitetural

L3 materializa I/O e integra contratos definidos em L1. A raiz apenas declara a
topologia da crate: não implementa comportamento dos módulos filhos nem reexporta
seus itens. Formatação user-facing de diagnósticos pertence a L2 desde P119
(ADR-0050), portanto não é responsabilidade deste hub.

## Aceitação estrutural

- a raiz expõe exatamente os módulos produtivos declarados no source;
- as duas suítes privadas entram somente em builds de teste;
- nenhuma lógica de negócio ou de formatação é adicionada ao hub.

## Fora de escopo

Comportamento interno de pipeline, export, fontes, world e demais módulos pertence
aos respectivos owners. A suíte E2E pertence a `infra/integration_tests.md`.
Fixtures, testes individuais e decisões futuras de API também ficam fora deste L0.
Qualquer mudança futura na superfície `pub mod` exige nova classificação ADR-0127.
