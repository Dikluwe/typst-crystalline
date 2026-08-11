# Relatório — Passo 996: `\` quebra a linha ANTES do emparelhamento lr

**Estado do código das medições**: HEAD `5074d5021` (P995+adendo) +
alterações deste passo. Commit final: ver rodapé.
**Gate ADR-0127**: fluxo contínuo — correcção de paridade (a direcção foi
confirmada pelo dono citando a documentação oficial: `\` em math é apenas
quebra de linha), sem mudança de contrato público. L0 primeiro
(`engine/eval.md` §P996) + resselo.

## Fase A — já feita em P995 (registada no passo)

Medições vanilla (oficial 0.15.1 cacheado + main ratificado, idênticos):
`(n \ k)` = duas linhas centradas (centros 39.31 = 39.31pt) com parênteses
**naturais** (~11pt), um por linha; `(a = b \ c = d)` idem; nem mesmo com
linha alta (∑ com limites, ~32pt) o delimitador estica. Mecanismo vanilla:
`Linebreak` (`ir/resolve.rs:179-180`) + `expand_multiline_fence`
(`ir/multiline.rs:56-107`) com dimensionamento pelo segmento próprio
(`SharedFenceSizing`, `ir/item.rs:1053-1081`).

Causa cristalina: o lexer emparelha os parênteses (`Expr::MathDelimited`)
com o `Linebreak` DENTRO do corpo, e o layout esticava os delimitadores
sobre a grelha — daí o n a 0.00/0.00pt (a "folga zero" da investigação
paralela).

## Fase B — TDD

RED confirmado: `p996_paren_com_linebreak_nao_emparelha` falhou com a
estrutura exacta do bug (`math.delimited('(', sequence[n, linebreak, k],
')')`); guarda `p996_paren_sem_linebreak_continua_delimited` verde desde
o início.

Implementação (`eval/math.rs`, braço `Expr::MathDelimited`): corpo com
`Content::Linebreak` ao nível do topo **não emparelha** — emite
`MathSequence[MathText(open), …corpo…, MathText(close)]` (glifos normais,
tamanho natural); o caminho de grelha existente (P991, 1 coluna centrada)
empilha as linhas. Sem linebreak: inalterado.

GREEN: **5825 testes, 0 falhas** (5823 + 2 novos). `crystalline-lint .`:
0 violations (só V7 órfão pré-existente).

## Fase C — Revalidação

Secção 7 do canónico (`temp/revisao/depois-p996.pdf`):
- `(n \ k)`: linha 1 `(𝑛` (token 11pt, parêntese natural), linha 2 `𝑘)` —
  centros **240.50 = 240.50pt** (centradas, como o vanilla); sem peças
  esticadas (`⎛⎜⎝⎞⎟⎠` desapareceram). A "folga zero" do n deixou de
  existir por construção — já não há delimitador esticado.
- `binom(n, k)`: inalterado (esticado, com padding) — guarda visual ok.
- Confirmação visual a 300dpi: estrutura idêntica ao vanilla
  (`temp/revisao/s7-c996-1.png` vs `nk-v2-1.png`).

Benchmark canónico (`benchmark-p996-canonical.py`, antes = release
pré-P996): duas corridas — 02-lorem 1.043/1.008 (ruído; confirmado na
segunda), restantes 0.979–1.015, **médias 1.001/0.998 — sem regressão**.

## Nota

Com esta correcção, `(n \ k)` fica como o vanilla — que é **diferente** de
`binom(n, k)` (parênteses pequenos por linha vs. binómio esticado). A
equivalência gráfica `(n\k)≡binom` existe em LaTeX (`{n\choose k}`), não
em Typst — a documentação oficial define `\` apenas como quebra de linha
(confirmado pelo dono).

---
