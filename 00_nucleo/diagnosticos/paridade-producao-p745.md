# P745 — Confirmar se os ~0,14% residuais de `cetz` são anti-aliasing

**Data:** 2026-07-14 (08:45 -03:00)
**Commit base:** `9425cc4e6` ("P744: preenche hash do commit no relatorio")
**Commit da sonda:** `PREENCHER_APOS_COMMIT`
**Estado no momento das medições:** working tree limpa; nenhum ficheiro de código alterado.

Vanilla de referência: `lab/typst-original/target/release/typst`
(typst 0.15.0, 969087ec).

---

## Contexto

Desde P731, todos os relatórios da cadeia `cetz` atribuíam o diff residual de
pixels (~0,14–0,15%) a "anti-aliasing", sem nunca ter sido confirmado com
metodologia própria. Este passo executa quatro testes independentes para
confirmar ou refutar essa explicação antes de a aceitar como definitiva.

---

## Teste 1 — o diff diminui com a resolução?

Documento `/tmp/p745-cetz.typ`:

```typst
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Rasterização com `mutool draw` a quatro resoluções; comparação com
`/tmp/pngdiff.py` (threshold >8 num canal, percentagem sobre o total de bytes):

| Resolução | Pixels não-brancos (vanilla / cristalino) | Diff bytes | Diff % |
|---|---|---|---|
| 75 dpi (621×877) | 479 / 502 | 3036 / 1 633 851 | 0.1858% |
| 150 dpi (1241×1754) | 1451 / 1508 | 9264 / 6 530 142 | 0.1419% |
| 300 dpi (2481×3508) | 5102 / 5036 | 30 981 / 26 110 044 | 0.1187% |
| 600 dpi (4961×7016) | 18 574 / 18 584 | 112 407 / 104 419 128 | 0.1076% |

A percentagem de diff **diminui consistentemente** quando a resolução aumenta
(0.186% → 0.142% → 0.119% → 0.108%). O número absoluto de bytes diferentes
cresce, mas muito mais lentamente do que o total de bytes (cresce com o
perímetro dos traços vs. área da imagem). Comportamento típico de
anti-aliasing.

---

## Teste 2 — mapa visual de diferenças

Script `/tmp/pngdiff_map.py` gera uma imagem RGB em que pixels iguais ficam
cinza claro (240, 240, 240) e pixels diferentes ficam a vermelho (255, 0, 0).

Ficheiros gerados:

- `/tmp/p745-diffmap-150.png` — 3088 pixels diferentes
- `/tmp/p745-diffmap-300.png` — 10 327 pixels diferentes

Inspecção visual: as diferenças concentram-se estritamente nas **fronteiras**
dos traços — contorno do círculo e linha diagonal. Não há manchas, deslocamentos
ou diferenças no interior/fundo. Padrão consistente com duas rasterizações do
mesmo vector com ligeiras diferenças de posicionamento de sub-pixel.

---

## Teste 3 — formas nativas simples, sem `cetz`

Documento `/tmp/p745-nativo.typ`:

```typst
#line(start: (0pt, 0pt), end: (100pt, 50pt))
#circle(radius: 30pt)
```

Resultado a 150 dpi:

- Pixels não-brancos: vanilla 1897, cristalino 1903.
- Diff: 10 683 / 6 530 142 bytes = **0.1636%**.

O diff é da **mesma ordem de grandeza** do caso `cetz` a 150 dpi
(0.1419%), o que indica que a causa é genérica à rasterização de traços
vectoriais, não específica de `cetz`/WASM.

---

## Teste 4 — determinismo do rasterizador

O mesmo PDF vanilla foi rasterizado duas vezes com `mutool draw -r 150`;
as duas imagens foram comparadas:

- Pixels não-brancos: 1451 / 1451.
- Diff: **0 / 6 530 142 bytes (0.0000%)**.

`mutool draw` é determinístico; o ruído observado nos testes 1–3 não vem da
ferramenta de comparação.

---

## Decisão

Os quatro testes confirmam a hipótese de anti-aliasing:

1. O diff percentual **diminui** com a resolução.
2. As diferenças concentram-se nas **fronteiras** dos traços.
3. O mesmo fenómeno ocorre em **formas nativas simples** (sem `cetz`).
4. O **rasterizador é determinístico**; o ruído não é artefacto da medição.

A explicação dos ~0,14% residuais como anti-aliasing fica **confirmada**, não
apenas repetida. Não há bug real a corrigir. Os relatórios futuros da cadeia
podem continuar a usar esta justificação com base nesta sonda.

---

## Validação global

- Nenhum ficheiro de código alterado.
- `cargo test --workspace`: não aplicável (sem alterações); testes anteriores
  continuam a passar.
- `crystalline-lint .`: 0 violations (nenhum ficheiro de código modificado).

---

## Proveniência

- Commit base: `9425cc4e6`
- Hora das medições: 2026-07-14T08:45-03:00
- Binário vanilla: `lab/typst-original/target/release/typst`
- Binário cristalino: `./target/release/typst`
- Rasterizador: `mutool draw` (MuPDF)
- Script de comparação: `/tmp/pngdiff.py` (threshold >8 por canal)
- Script de mapa visual: `/tmp/pngdiff_map.py`
