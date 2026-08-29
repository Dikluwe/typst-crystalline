# P1243 — revalidação do pré-selo de tiling SVG

**Veredito:** `PRESEAL_INVALIDATED_NEGATIVE_BOUNDARY_RETAINED`.

## Medição

O runner `lab/parity/matrix/p1243_revalidate.py` comparou o manifesto v3 com
as entradas atuais. Quatro entradas protegidas mudaram:

- passo P1242: `528b…` → `e70c…`;
- inventário P1242: `ef85…` → `d877…`;
- L0 `entities/tiling.md`: `d76b…` → `bbcf…`;
- L0 `compiler/stdlib/tiling-stdlib.md`: `8087…` → `3d70…`.

O L0 SVG e os artefatos v3 de contrato, oráculos, casos e verificação
permaneceram byte-idênticos. Mesmo assim, qualquer mudança de entrada protegida
invalida o selo segundo o protocolo.

A suite histórica tem 34 classificações corretas, distribuídas em 19
`Unknown`, 10 `CONTRACT-GAP`, três `PASS` e duas `FAIL`. Essa razão
`34/34` mede consistência classificatória; não é
`mutações semanticamente negativas rejeitadas / mutações válidas`.
Consequentemente, não há mutation score atual e `PRESEAL_SEALED` foi retirado.

## Fronteira retida

A conclusão estreita permanece: zero casos SVG adicionais foram provados
`Preserved`. Os dois controles positivos são current-only e o vanilla rejeita
as duas sintaxes; eles não provam paridade compartilhada. Image, Gradient,
conteúdo arbitrário, size auto, relative parent, stroke e offset/angle
continuam `CONTRACT-GAP`; os demais observáveis sem witness bilateral
continuam `Unknown`.

P1244 deve encerrar sem código, salvo nova evidência bilateral materializada a
partir dos L0 atuais. Nenhuma implementação ou mudança pública está autorizada.

## Proveniência

- HEAD: `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`.
- Working tree não commitada: 59 ficheiros tracked alterados, 2247 inserções e
  188 remoções.
- Medição: `2026-08-28T06:51:59-03:00`.
- Duas execuções do auditor produziram saída byte-idêntica.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. Esta foi uma auditoria do selo
histórico, não uma nova autoria segregada ou um novo preseal.
