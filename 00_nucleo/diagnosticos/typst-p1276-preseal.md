# P1276 — preseal da fronteira SVG multi-space restante

**Veredito:** `CONTRACT-CANDIDATE-FROZEN`.

Foram congelados oito pares, 240 grupos de fixture e 336 observações focais.
Os 18 mutantes da definição foram executados e rejeitados. Esse
score é limitado à estrutura do contrato; nenhum mutante semântico, oracle ou
candidato produtivo foi executado neste passo.

CMYK permanece `Unknown-ADR0097`. O owner SVG e seu L0 não foram alterados; os
oito pares continuam com fallback explícito. P1277 deve materializar primeiro
os oráculos vanilla, congelá-los e somente então medir o candidato.

Proveniência: HEAD `643de866315179aeed0be8278d703658b581f694`, working tree não commitada,
medição `2026-08-29T12:39:07-03:00`. Manifesto de entradas:
`p1276-input-manifest.tsv` (`sha256:c6e2a575c75962c6aacf7896c4b74f716e6d633146521ac38c2f4afeaa9dfa6f`). Recibo do pacote:
`p1276-preseal-receipt.tsv` (`sha256:279a1edc98c366adad96eb1cf98f96516c192740504e54098c37aa699e56df79`).

**Atestação:** EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
