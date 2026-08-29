# P1238 — espaços polares na fronteira pública L1

**Proveniência:** HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`,
working tree não commitada, campanha fechada em
`2026-08-27T22:41:56-03:00`. Na medição havia 59 ficheiros alterados,
2247 inserções e 188 remoções.

**Veredito:** `ACCEPTED_POLAR_L1_SAMPLE_ONLY`.

O antigo stop de dependência foi retirado. Esta campanha mede alpha diretamente
e não depende de selo ou mutation score do P1236. A exceção P1252 permanece
limitada à conversão Luma e não foi usada para perdoar delta polar.

A campanha mediu separadamente Linear e Radial em Oklch, Hsl e Hsv. Para cada
um dos seis pares foram executados seam forward/reverse, controle sem wrap,
baixa cromaticidade e alpha no primeiro/último stop, em sete posições. Isso
produziu 36 fixtures, 504 observações públicas e 252 comparações vanilla versus
cristalino.

Todos os pares preservaram `42/42` amostras, sem `Unknown` ou violação. Hue foi
comparado por distância angular modular com tolerância `0.01deg`; componentes
não angulares e alpha foram comparados separadamente com tolerância `0.0001`.
Uma execução normal e outra com variantes, espaços, cenários e posições em
ordem inversa produziram os mesmos hashes canônicos para observações, métricas,
comandos, resultados e resumo. Os onze ataques históricos eram predicados
autocontidos e foram revogados; não há mutation score nem selo Tekt.

Esta conclusão cobre somente `gradient.sample(t).components(alpha:true)`, isto
é, a semântica pública L1 dos construtores Linear/Radial. Não mede nem promove
adaptive L3, raster ou evaluator SVG. Nenhum Prompt L0 ou código produtivo foi
alterado; não houve correção a materializar nesta fronteira.

Contrato, fixtures, ataques, observações, métricas, comandos, resultados e
certificado usam o prefixo `p1238-polar-` em `00_nucleo/diagnosticos/`; o runner
reproduzível é `lab/parity/matrix/p1238_polar.py`.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`
