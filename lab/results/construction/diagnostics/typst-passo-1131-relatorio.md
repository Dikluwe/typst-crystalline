# Passo 1131 — margem inferior de página `auto` terminada em texto

**Data:** 2026-08-21  
**HEAD base:** `781b207b4a5de9c2bfbe5819918a193d1d9293e5`  
**Estado:** working tree não commitada; P1131 foi executado sobre as alterações
já presentes de P1129/P1130. Às 2026-08-21T22:49:02-03:00,
`git diff HEAD --stat` registava 28 ficheiros, 805 inserções e 61 remoções. O
diff de P1131 está concentrado em `compiler/layout.md`, `layout/cursor.rs`,
`layout/tests.rs` e nos 11 headers adicionais resselados automaticamente para
o mesmo prompt.

## 1. Medição inicial

Binários recompilados do estado acima (`cargo build --release`). Oráculo:
`/usr/local/bin/typst`, vanilla ratificado. Recibos em `/tmp/p1131/`.

| documento | cristalino antes | vanilla | Δ altura |
|---|---:|---:|---:|
| `sec_20` | 124,772 − 20,713 = 104,059 pt | 124,772 pt | −20,713 pt |
| `sec_31` | 243,085 pt | 263,820 pt | −20,735 pt |
| `sec_38` | 128,360 pt | 149,073 pt | −20,713 pt |

`mutool trace` confirmou que as baselines finais não tinham o mesmo erro:
secção 20, 96,42544 vs 96,42545 pt; secção 31, 235,45114 vs 235,47316 pt.
Logo o defeito estava na escolha do fundo de `compute_page_height`, não no
posicionamento do texto.

## 2. Causa real

Em `cursor.rs::flush_line`, P1120 actualizava `last_block_descent_y` apenas
quando `inline_descent > 0`. Uma linha de texto comum posterior a uma equação
ou bloco deixava sobreviver o fundo desse bloco. Quando a linha já estava
drenada antes de `finish()`, o fallback `had_last_line` também não corria e
`compute_page_height()` usava o estado obsoleto.

O valor observado de 5,50 pt não provém de uma constante `0.5em` neste
caminho. As ocorrências reais de `0.5em` encontradas pertencem à calha de
numeração de equações e ao `body-indent` de listas/enums. O leading default
continua 0,65 em. A relação com P1103 é directa: é uma regressão do mesmo
estado vertical, reintroduzida ao restringir a actualização em P1120.

## 3. L0, RED e implementação

- L0 actualizado primeiro em `00_nucleo/prompts/compiler/layout.md` §P1131.
- Hash resselado para `9dff1585` em todos os consumidores do prompt.
- Teste RED: `p1131_flush_linha_texto_substitui_estado_vertical_obsoleto`
  recebeu `Some(60,8667)` quando esperava a baseline `Some(70,8667)`.
- Teste documental adicional: página `auto` com bloco seguido de texto deve
  fechar em `baseline_final + margem`.
- Implementação: se uma linha comum encontra `last_block_descent_y = Some`,
  substitui-o pela baseline actual. `None` permanece `None`, preservando o
  caminho do cursor usado por Align/Place; caixas inline continuam em
  `baseline + descent` conforme P1120.

A primeira tentativa incondicional foi refutada pelos sentinelas P898/P908 e
descartada antes do fecho: criava `Some(baseline)` quando não havia fundo de
bloco e alterava quatro casos de Align/Place sob `height:auto`.

## 4. Recibo final dos seis documentos

| documento | cristalino P1131 | vanilla | resultado do passo |
|---|---:|---:|---|
| `sec_20` | 316,033 × 124,772 pt | 316,033 × 124,772 pt | exacto |
| `sec_31` | 494,545 × 263,798 pt | 494,545 × 263,820 pt | margem corrigida; resíduo vertical preexistente −0,022 pt |
| `sec_38` | 328,164 × 149,073 pt | 328,164 × 149,073 pt | exacto |
| `sec_09` | 256,660 × 148,642 pt | 256,660 × 154,663 pt | dimensão cristalina inalterada pelo passo |
| `sec_27` | 212,843 × 189,747 pt | 212,843 × 192,110 pt | dimensão cristalina inalterada; resíduo fora do escopo |
| `sec_43` | 391,392 × 122,112 pt | 391,392 × 125,236 pt | dimensão cristalina inalterada pelo passo |

Na secção 31, a diferença final de altura é igual à diferença preexistente da
baseline final; portanto a margem inferior em si converge. P1131 não reivindica
fecho desse resíduo de posicionamento.

## 5. Gates

- Testes P1131: 2/2 verdes.
- Sentinelas inicialmente afectados: P898 1/1 e P908 3/3 verdes após a
  restrição final.
- `cargo test --workspace`: verde — core 5091, infra 799, shell 41, wiring 2,
  CLI 37, lint integration 2; doc-tests sem falhas.
- `cargo build --release`: verde.
- `crystalline-lint .`: exit 0, zero erros e zero drift de hash. O linter
  continua a listar avisos/informações V16–V20 preexistentes, não bloqueantes.

## 6. Classificação ADR-0127

Apesar do enunciado chamar o caso de “mudança de comportamento por defeito”,
a fonte e a medição classificam-no como correcção interna de paridade visual,
sem contrato público, flag, modo padrão ou mudança de fase. Pela cláusula
expressa de correcções de paridade da ADR-0127, o passo seguiu fluxo contínuo:
L0 primeiro, RED→GREEN e revalidação completa.
