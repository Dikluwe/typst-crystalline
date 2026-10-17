# Prompt L0 — `infra` — raiz da crate `typst-infra`
Hash do Código: f1a37d61

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8


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

## Contexto integral bidi ratificado — 2026-10-05

Medição anterior à decisão: o lote bilateral e a refutação Unicode estão em `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/{initial,probe}.json.gz`, com fontes/estado/binários. `layout_bidi.rs:73–82,227–240` redetecta base e inventa separadores; `shaper/bidi.rs:34` redetecta o fragmento; isto perde a entrada integral ratificada pelo humano. Correção interna de paridade, sem nova fase/assinatura pública externa.

Para spans certificados, usar registry proprietário local de strings auxiliares e mapas por input Arc, depois views BidiInfo emprestadas normais; não usar unsafe/self-reference. Embeddings virtuais LRE/RLE/PDF só quando dir do segmento difere da base; espaços reais e Linebreak não justificado LF (justify=true fora do certificado). Não retirar controles autorais. Resolver uma vez por input em cada registry; o pipeline paginado compartilha registry/views entre reorder e shape. Wrappers públicos existentes podem construir registry local.

Reorder consulta intervalo real da linha, intersects exatas por segment/range e níveis com regras de fim de linha; não midpoint nem espaços inventados. Text misto divide por fronteiras originais, preserva estilo e subspan. Itens não textuais e fontes desconhecidas não ganham certificado. Reorder conserva âncora física aceita por L1, largura/avanço reais; não aplicar fallback de suffix/coalescing ao contexto certificado.

Shape consulta os runs do input completo para span original e não auto-detecta direção de fragmento; manter offsets de subruns por travessia sequencial, emitir subspan exato para cada TextShaped. Chave de cache existente já discrimina direção; não pôr contexto em TextStyle. Reconciliation usa base do input quando existe em vez de qualquer run RTL. Fallback None anterior permanece. Testes RED→GREEN da refutação de 456 789, overrides/neutral, extração sem controles auxiliares e medição bilateral.
