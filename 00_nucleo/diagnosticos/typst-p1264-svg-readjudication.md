# P1264 — consolidação SVG e readjudicação dos quatro pares

**Estado:** EXECUTADO — QUATRO PARES CERTIFICADOS NO FRAGMENTO  
**Regime:** protocolo Tekt completo, executado sem atestação de isolamento

## Medição antes da decisão

A base P1255 foi preservada sem reinterpretação: 6/24 fixtures estavam
preservadas e 18/24 violadas. Depois das correções focais P1261–P1263, o
corpus completo foi reexecutado em ordem direta e inversa. Os dois percursos
foram semanticamente iguais após canonicalização das linhas e repetiram os
artefatos selados P1263.

O resultado final é 24/24 no grafo e 24/24 numericamente, sempre usando
`color_max`, `color_p95`, `alpha_max` e `alpha_p95` contra os quatro budgets
`V_*` congelados. Cada um dos quatro pares passou separadamente em 6/6:

- Linear/Oklab: `0/6` → `6/6`;
- Radial/Oklab: `0/6` → `6/6`;
- Linear/LinearRgb: `3/6` → `6/6`;
- Radial/LinearRgb: `3/6` → `6/6`.

Não houve inferência cruzada entre geometria ou espaço. O maior stopset medido
tem 46 stops, abaixo do cap 64. O custo incremental das correções P1263
permanece zero stops e zero decisões adaptativas.

## Revalidação L1 e política `Unknown`

P1234 foi executado duas vezes contra o vanilla ratificado e o binário
cristalino recompilado: 15 fixtures, 30 probes e zero divergências públicas.
As 30 fronteiras internas inacessíveis continuam corretamente `Unknown`; como
não existe divergência pública, elas não atribuem owner nem pertencem à cadeia
causal necessária para a readjudicação SVG.

P1236 também foi executado duas vezes: 6 fixtures, 42 pares,
`luma_max=0`, zero `Unknown`, 28 `Preserved` e 14
`Known-Upstream-Bug` limitados à perda de alpha vanilla já adjudicada no P1252.
Todos os artefatos das repetições P1234/P1236 foram byte-idênticos.

Casos opacos não viram sucesso: servidor ausente/ambíguo e evidência pair-local
ausente continuam `Unknown` e bloqueiam somente o par afetado.

## Ataques e certificação

Os 23 mutantes executáveis focais de P1261–P1263 permanecem rejeitados. O gate
de integração rejeitou 7/7 mutantes: promoção por agregado contraditório,
inferência Linear→Radial, inferência Oklab→LinearRgb, remoção de p95,
`Unknown` como sucesso, recibo focal parcial e hash de fixture alterado.
`integration_mutation_score=1.0`; `Unknown` foi excluído do numerador.

Os quatro gates de promoção classificatória passam e os pares ficam
`Preserved-fragment` neste corpus. Nenhuma promoção produtiva foi aplicada: o
P1264 proíbe novas correções, `paint_is_svg_native` permanece inalterado e uma
mudança do fallback por defeito requer passo próprio no gate ADR-0127. Não há
alegação de equivalência SVG geral.

## Gates finais e limite de atestação

Passaram 6.332 testes do workspace, o teste produtivo focal P1263,
`cargo build --workspace`, `cargo fmt --all -- --check`, `git diff --check`,
o lint focal V1/V5/V15/V26 e `crystalline-lint .` com zero violações. Os
warnings Rust e informativos V16–V20 preexistentes não são violações.

O P1264 não altera L0 nem código produtivo; adiciona apenas runner e recibos de
diagnóstico. Contrato, adversário, integração e veredito foram exercidos sob a
mesma autoridade `/root`; portanto, apesar dos inputs congelados e da
reprodutibilidade, o veredito proporcional é:
**EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**.
