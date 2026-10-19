# Passo 947 — Relatório (duplicação de operadores no content stream: REFUTADA)

**Data**: 2026-08-01
**Estado da árvore na medição**: commit `019dfbd5c` (P945, HEAD de `Tekt`) + alterações
de P946 **não commitadas** (working tree: `00_nucleo/prompts/infra/font_metrics.md`,
`03_infra/src/font_metrics.rs` modificados — **P946 ainda não foi commitado pelo dono**;
reportado aqui conforme a pré-condição do passo).
**Artefactos**: `temp/p946/stream-*.txt` (content streams brutos extraídos).

---

## 1. Proveniência de `review-now.pdf` (primeiro passo do passo)

`temp/p946/review-now.pdf` (01:38→07:49 de 2026-08-01) **não** foi gerado por um binário
commitado de P946 — P946 não está commitado à data deste relatório. Foi gerado durante a
Fase A de P946 por um binário de working tree (commit P945 + `eprintln!` de diagnóstico
em `stretchy.rs`, revertido depois). O padrão `(||`/`{|{|{`/`|||||` no seu `pdftotext`
é o esperado da ToUnicode pré-P946 (ganchos → char base, extensores → `|`).

## 2. Fase A — contagem literal de operadores no stream bruto

Extração directa dos content streams (`mutool show <obj>`), contagem de ocorrências
`Tj`/`TJ` por peça e comparação de posições — sem proxy:

### Matriz 3×3 (`temp/p945/m33-fixed.pdf` vs `m33-vanilla.pdf`)

| | cristalino | vanilla |
|---|---|---|
| operadores para os 2 parênteses | 8 `Tj` (4 peças/lado: gancho, 2 extensores, gancho) | 8 `Tj` (idem) |
| operadores para os 9 dígitos | 9 `TJ` | 9 `Tj` |
| total | **17** | **17** |

Posições das 4 peças do parêntese esquerdo: y = 49.1, 53.8, 57.9, 73.6 — **todas
distintas**, uma por peça. Nenhum par de operadores na mesma posição.

### Chave de `cases()` (`temp/p946/repro.pdf` vs `repro-vanilla.pdf`)

| | cristalino | vanilla |
|---|---|---|
| peças da chave | 5 `Tj` (y = 49.1, 55.2, 69.6, 75.7, 81.9 — distintas) | 5 (extensor partilha glyph code, 1 `Tj` por peça) |
| total na página | 9 `Tj` + 8 `TJ` = **17** | **17** |

### Matriz 4×4 de reticências (`temp/p946/doc-now.pdf`, o caso `|||||`)

Stream bruto, lado esquerdo (x = 195.2): **exactamente 7 `Tj`** — gancho `<00FC>`,
5 extensores `<00FB>` (y = 1255.5, 1260.1, 1264.6, 1269.2, 1273.8 — espaçamento regular
de ~4.6pt, todos distintos), gancho `<00FA>`. Lado direito (x = 295.3): idem. A
"duplicação" escala com o número de peças porque **é** o número de peças — cada uma
desenhada exactamente uma vez.

## 3. Conclusão — hipótese de duplicação REFUTADA, explicação alternativa registada

O content stream bruto mostra, nos três casos do padrão, **um operador de desenho por
peça de assembly, em posições distintas, com a mesma contagem do vanilla** (17 = 17,
7 por lado = 7 por lado). Não há instâncias sobrepostas nem operadores a mais — a
hipótese de reconciliação do passo ("duplicados exactamente na mesma posição,
invisíveis ao pixel-diff") é refutada directamente pelas coordenadas do stream.

A explicação alternativa, confirmada: o `pdftotext` "duplica" porque **cada peça de
assembly é um span de texto próprio** (um `Tj` por peça — correcto e igual ao vanilla),
e a ToUnicode pré-P946 mapeava ganchos → char base e extensores → `|`. Logo uma
assembly de 4 peças extrai `(| |(` → `(||(`; a chave de 5 peças extrai `{|{|{`; a matriz
de reticências com 5 extensores extrai `|||||`. N spans ≠ N desenhos duplicados; são N
peças, cada uma desenhada uma vez — que é exactamente o que uma assembly OpenType MATH
é suposta fazer (o vanilla faz o mesmo, com a mesma contagem).

A suspeita original de P945 ("dois blocos de código desenhando a mesma peça") fica
**definitivamente refutada** — agora sim com a evidência que o passo pede: stream bruto,
não proxy. P945/P946 não "mascararam" nada: não havia nada para mascarar.

## 4. Lacuna de método (registo pedido pelo passo)

Concordamos com a nota do passo, com um ajuste de direcção: a lacuna real não era de
atestação visual (pixel/charstring) mas de **interpretação de texto extraído sem
conhecer a estrutura de assembly** — `pdftotext` não distingue "N desenhos da mesma
coisa" de "N peças de uma coisa composta". Registado para o handoff:

- Quando um sintoma vem de **conteagem de operadores/caracteres** (texto extraído,
  traço), a verificação tem de descer ao **stream bruto com posições** (`mutool show`)
  e à contagem por peça — pixel-diff e charstring-diff, sozinhos, não resolvem a
  ambiguidade.
- E o inverso também já aconteceu nesta série (P946): sintomas **visuais** a baixa DPI
  simulam defeitos inexistentes — imagens de revisão de geometria a ≥200–300dpi.

## 5. Sem alterações de código / L0 / benchmark

Não havendo duplicação, não há correcção a fazer nem contrato a alterar — Fase B não se
aplica. Benchmark não corridо (zero alterações de código face ao estado de P946 — seria
medir o mesmo binário duas vezes). Sem commits nem mutações git por minha parte.

## 6. Pendência de processo reportada ao dono

**P946 não está commitado** — HEAD continua `019dfbd5c` (P945). As alterações de P946
(L0 `infra/font_metrics.md` §P946, `03_infra/src/font_metrics.rs`, relatório, JSONs do
benchmark) estão no working tree à espera do commit habitual de passo.
