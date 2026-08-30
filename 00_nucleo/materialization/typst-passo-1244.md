# P1244 — fechar tiling modelável sem contrato público novo

**Estado:** EXECUTADO — ENCERRADO SEM CÓDIGO; FRONTEIRA NEGATIVA RETIDA  
**Predecessor:** P1243  
**Saída:** fronteira negativa fechada; nenhum caso SVG adicional promovido.

Atualizar primeiro o L0 do owner SVG e implementar somente os corpos que P1243
provou representáveis com os tipos atuais. Preservar tamanho da célula, spacing,
relative, transform, alpha e papel fill/stroke. Proibir rasterização silenciosa,
dimensão inventada e promoção de conteúdo opaco.

Exigir RED→GREEN, grafo local fechado, raster local, ordem direta/inversa e
mutation score 1.0. Se nenhum caso adicional for modelável, encerrar sem código
e manter o fallback `tiling-content-or-size`.

## Resultado saneado — 2026-08-28

P1243 invalidou o preseal v3, mas reteve a conclusão estreita de que o universo
SVG adicional provado é vazio. As antigas `34/34` classificações não são
mutation score. Por isso P1244 foi encerrado pela segunda alternativa do
próprio passo: nenhum Prompt L0, teste novo ou código produtivo foi escrito.
O subconjunto de cor com tamanho
explícito já existente foi apenas revalidado; os restantes casos continuam
`Unknown` ou `CONTRACT-GAP`, e o fallback produtivo permanece
`tiling-content-or-size`.

Os controles direto/inverso permanecem somente como evidência histórica
current-only, sem alegação de paridade compartilhada. O auditor de fechamento
foi executado duas vezes com saída byte-idêntica. O teste
`p1227_tiling_de_cor_emite_pattern_com_tamanho_e_spacing` passou; V15/V26 e
`git diff --check` passaram. V5 mantém duas derivas externas conhecidas em
`visualize.rs` e `tiling.rs`, sem serem mascaradas como sucesso. Recibo:
`00_nucleo/diagnosticos/p1244-closure-receipt.tsv`.

Segregação Tekt não aplicável: encerramento diagnóstico sem materialização.
