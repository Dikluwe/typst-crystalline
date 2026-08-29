# P1236 — diagnóstico corrigido de alpha/Luma

**Proveniência:** HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`,
working tree não commitada em `2026-08-27T22:25:43-03:00`: 59 ficheiros
alterados, 2247 inserções e 188 remoções.

**Veredito:** `ACCEPTED_DIAGNOSTIC_FRAGMENT_CURRENT_L0`.

O gate anterior foi substituído por um probe que usa somente construções e
observações públicas: a expressão integral do gradient e
`sample(t).components(alpha:true)`. Foram medidas seis fixtures, sete posições
e dois sistemas, totalizando 84 observações e 42 pares.

Quatorze pares exibem delta de alpha de até `0.6`. Pela cláusula P1252 do L0
`entities/color`, a perda de alpha do vanilla ao converter uma cor não-Luma
para Luma é `Known-Upstream-Bug`; a preservação cristalina é normativa.
Na reexecução com o binário cristalino atual, nenhum par teve delta de
luminância. A antiga atribuição causal a L1 foi retirada: observação pública,
por si só, não prova o locus interno responsável.

Vinte e oito pares são preservados; zero ficaram `Unknown` em B03. O construtor
reconstruído B04 continua desnecessário para localizar o erro, pois a
divergência já é pública em B03. O diagnóstico não autoriza promoções SVG nem
alteração de comportamento nem equivalência funcional geral.

Gate executado: duas execuções completas byte-idênticas. Os nove ataques
anteriores eram sintéticos e foram revogados; não há mutation score nem selo
Tekt. Nenhum L0 ou código produtivo foi alterado.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`
