# Passo 949 — Relatório (gap da peça do meio da chave: REFUTADO — é o desenho da fonte, idêntico no vanilla)

**Data**: 2026-08-01
**Estado da árvore na medição**: commit `8e3fdbdb7` (P948, HEAD de `Tekt`).
**Artefactos**: `temp/p949/` (traces, saídas fontTools/pikepdf).

---

## 1. Resumo executivo

O "gap" medido pelo dono na chave de `cases()`/matrizes com chave (1.2pt entre ⎧→⎨ e
9.5pt entre ⎨→⎩, medido entre os limites das bounding boxes das peças via `compare.py`)
**não é um defeito** — é o desenho da assembly da chave na fonte, e o vanilla produz
**exactamente o mesmo** espaçamento (medido peça a peça: 6.1/14.4/6.1/6.2pt nos dois
compiladores). A peça do meio (`uni23A8`) tem `full_advance` de 1500du — o dobro das
outras (750du) — e não é extensor **por desenho da fonte** (é o ponto central da chave,
aparece exactamente uma vez nos dois compiladores). O subsetting não corrompeu nada:
charstring de `uni23A8` **byte-a-byte idêntica** nas três vias exigidas pela Fase A.0.

## 2. Fase A.0 — subsetting não corrompeu o glifo (confirmado)

`uni23A8` (peça do meio da chave), charstring extraída de **dentro** do `CrystallineFont2`
embutido em `test_crystalline.pdf`, comparada com a do `NewCMMath-Book` embutido no PDF
vanilla e com a da fonte original (`03_infra/fixtures/fonts/NewCMMath-Book.otf`):

```
crys     /F2 cid00168  [296, 21, -21, 'hstem', 397, 108, 'vstem', 250, 750, 'rmoveto', …]
vanilla  /f1 cid00186  [296, 21, -21, 'hstem', 397, 108, 'vstem', 250, 750, 'rmoveto', …]
original uni23A8       [296, 21, -21, 'hstem', 397, 108, 'vstem', 250, 750, 'rmoveto', …]
```

Idênticas. O subsetting não alterou o desenho (a diferença de hash/tamanho dos FontFile3
citada no passo é a esperada entre subsetters diferentes — cristalino vs krilla).

## 3. Fase A — mesmo caminho, flag de extensor correcta, gap idêntico ao vanilla

**Mesmo caminho**: `{`/`}` passam por `layout_stretchy_delimiter` → `select_variant` →
`layout_assembly`/`resolve_assembly_repeat` (P906/913), exactamente o mesmo código dos
parênteses — não existe caminho legado separado de chave.

**Dados da fonte** (NewCMMath, fontTools — assembly de `{`, ordem fundo→topo):

| peça | start | end | full | ext? |
|---|---|---|---|---|
| uni23A9 (gancho fundo) | 0 | 374 | 750 | não |
| braceleft.ex | 748 | 748 | 748 | **sim** |
| uni23A8 (meio) | 374 | 374 | **1500** | não |
| braceleft.ex | 748 | 748 | 748 | **sim** |
| uni23A7 (gancho topo) | 374 | 0 | 750 | não |

A flag `isExtender` é lida correctamente (os extensores repetem para alvos altos; o meio,
`ext=0`, não repete — tal como no vanilla, cuja função `parts()` também só repete peças
com a flag).

**O gap medido existe no vanilla, com os mesmos valores**. Posições das peças da chave
(trace, casos `cases(0, x², 1)` e chaves do documento de 30 secções):

| junta | cristalino | vanilla |
|---|---|---|
| gancho topo → extensor | 6.2pt | 6.1pt |
| extensor → meio | 6.1pt | 6.1pt |
| **meio → extensor** | **14.4pt** | **14.4pt** |
| extensor → gancho fundo | 6.1pt | 6.1pt |

O "gap" de 9.5pt medido pelo dono (entre o fundo da bbox do meio e o topo da bbox do
gancho inferior) é a consequência geométrica da peça do meio ter 1500du de
`full_advance` com conector de 374du — presente no vanilla com o mesmo valor. Não há
"quase uma peça inteira sem nada desenhado": a tinta do meio (`uni23A8`, 16.5pt de
altura) cobre o intervalo — a 300dpi a chave renderiza contínua e idêntica ao vanilla
(`temp/p946/cases-bottom2-cmp.png`, reconfirmado neste passo).

**Outros delimitadores da família**: `}` tem a assembly simétrica (`uni23AB/23AC/23AD`,
mesmos valores); `⌊⌋`/`⌈⌉` não têm peça do meio — nada afectado.

## 4. Conclusão

Hipóteses do passo verificadas e refutadas com números: (a) não é subsetting corruptor
(charstring idêntica nas 3 vias); (b) não é caminho de código separado (mesma função);
(c) não é flag `isExtender` mal lida (lida e aplicada como no vanilla); (d) o gap não é
ausência de desenho — é o espaçamento inerente à peça central, **idêntico no vanilla**.
Sem causa a corrigir — Fase B não se aplica. Sem alterações de código/L0; sem benchmark
(binário inalterado, mesmo raciocínio de P947).

**Registo metodológico** (para o handoff, complementa P946/947): `compare.py` mede
distâncias entre **caixas** das peças (origens/bboxes), não a continuidade da tinta — um
espaçamento grande entre bboxes de peças consecutivas de uma assembly pode ser desenho
correcto (peça alta por desenho da fonte) e não um buraco. Antes de abrir passo por
"gap" numa assembly, verificar: (1) a mesma distância no vanilla (trace, posições de
peça), (2) o render a ≥300dpi.
