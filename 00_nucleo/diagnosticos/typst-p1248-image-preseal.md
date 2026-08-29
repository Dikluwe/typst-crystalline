# P1248 — pré-selo de imagens SVG

O recibo anterior `edce6bb…723be` fica invalidado porque os artefatos que dizia
selar não estavam presentes. A reconstrução usa morfologia visível, não
serialização XML: oito observáveis, seis casos `Preserved`, dois `Unknown` e
onze mutações negativas com testemunha, incluindo perda de forma/alpha em SVG
aninhado.

Veredito executável: `PRESEMANTIC_GATE_PASS_UNATTESTED_ISOLATION`, score 11/11
(1.0). Nenhum candidato foi medido e nenhum código produtivo mudou. Como a
mesma sessão exerceu autoria de contrato, oracle e ataques, este resultado não
é promovido a preseal segregado; serve como ensaio congelável para repetição
por autoridades independentes.
