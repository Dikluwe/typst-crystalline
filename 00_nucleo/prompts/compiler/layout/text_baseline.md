# L0 — text_baseline
Hash do Código: 5785c8c6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/text_baseline.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência anterior à decisão

A inspeção de `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` e `space.rs:162` encontrou respectivamente fronteira textual, admissão de baseline e teste de integração de espaços iniciais em owners de outras responsabilidades. O estado exato, fontes e prompts anteriores estão em `00_nucleo/diagnosticos/separacao-tracking-2026-09-29/initial.json` e `before/`. A correção já medida encontra-se em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29.md`; esta separação conserva seus resultados e limites. Posições, texto e estilos são observáveis da língua; localização das funções é mecânica. A inspeção não demonstra intenção histórica nem nova paridade bilateral.

## Contrato vigente

Possuir admit(layouter), privado ao pai. Antes da admissão, registrar se a linha não está vazia e não contém material segundo html_frame::items_have_material. Chamar ensure_initial_baseline uma única vez. Somente se havia carriers sem material, alinhar y de cada TextSeparator ao cursor_y admitido mais baseline_offset resolvido com o tamanho do próprio estilo. Conservar x, advance, estilo, origem, ordem e demais itens.

Word/chunk chamam admit antes de emitir e novamente após wrap. O produtor Space não cria demanda vertical por espaços isolados. prepared conserva a admissão existente sem introduzir este alinhamento ou tracking. Não possuir as fórmulas de admissão regional, fechamento de linha ou medição de texto; delegá-las aos adapters existentes. O teste de espaços iniciais acompanha este owner e continua exercitando Space e Word reais.

## Aceitação e limites

Preservar os testes causais existentes e o texto, posições, estilos, páginas e diagnósticos dos controles anteriores à separação. Ownership 1:1, pins válidos, build e lint sem erros. Movimentação interna ADR-0109/0127: funções estáticas descendentes, sem estado novo. As limitações de tracking em estilos mistos, RTL, seleção global e contagem de clusters permanecem as registradas no diagnóstico da correção.

## Fecho de linha literal — medição e contrato (2026-09-29)

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/after/results.json` mostra dois parágrafos de espaços a 12pt em 30.206pt Tekt versus 31.536pt vanilla; a sonda por tamanhos 10–18pt em `size-probe.log` mostra crescimento de 1.914pt/pt no Tekt contra 2.628pt/pt vanilla. `text_baseline.rs:14-16` admite pelo `layouter.style`, que volta ao tamanho exterior ao sair de `text.rs`, mesmo quando cada TextSeparator Literal guarda o estilo efetivo. Criar helper privado que admite a baseline de uma linha somente Literal usando o estilo do carrier de maior tamanho, restaurando o estilo exterior depois; reutilizar `admit` e não criar estado/global. O conteúdo misto continua no caminho de Word/Chunk.
