# P1240 — pré-selo raster local

**Veredito histórico:** `PRESEAL REJECTED`; score `7/10`.

**Veredito vigente:** `ACCEPTED_AUXILIARY_RECT_RASTER_HARNESS`; sem selo.

As defesas contra crop, erosão, resize, fitting e alinhamento oportunista
passaram. Faltaram composição de transforms ancestrais, prova de cobertura de
stroke/mask e auditoria completa de leituras. Nenhum raster foi executado.
Receipt: `133b087aed8fdcc8ee7d0aa9e9bac08e93af4a0da501080570b4fda906d59da2`.

## Retificação executável

P1240 foi retomado com `lab/parity/matrix/p1240_raster.py`. A viewport passa a
ser derivada exclusivamente do grafo e geometria declarada: composição de
transforms ancestrais, expansão de stroke e interseção da máscara completa.
Referências quebradas ou geometria pintada não suportada ficam `Unknown`.

Sete testes cobrem estabilidade Linear/Radial/pattern em 1x/2x/4x,
invariância de IDs/defs/translação externa, composição ancestral, stroke e
mask retangular. Percursos de escala normal e inverso geraram artefatos
canônicos byte-idênticos. A lista de inputs no recibo não é auditoria de
syscalls. Path/wedges Conic estão fora do fragmento.

Os dez ataques históricos misturavam controles e mutações hipotéticas sem
fontes mutadas ou recibos; foram revogados. Não há mutation score nem selo.

O selo vale apenas para o fragmento local registrado. Não promove qualquer
par de espaço SVG por si só.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
