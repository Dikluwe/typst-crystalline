# Passo 1026 — Fase A: a premissa do gate não sobrevive à medição

**Data**: 2026-08-13
**Estado**: Fases A e B feitas. **Fase C/D bloqueadas no gate** (ADR-0127 categoria 2) — a
decisão é do dono.

## Proveniência

Árvore de trabalho não commitada sobre `HEAD = 184f1a1f2`. Binários usados, confirmados pelo
Passo 1025:

- referência vanilla: `lab/typst-original/target/release/typst` (baseline ratificado
  `a51e02804`, string `typst 0.15.1 (e0e8ca4d)`);
- cristalino: `./target/release/typst` (`typst 0.15.0 (00cd5bc5)`, `PARITY_VERSION`).

Documentos, PDFs e medições em `temp/p1026/` (script de medição incluído). Método: mesmo
documento nos dois binários → PDF → PGM a 300dpi (`pdftoppm -gray`) → extensão vertical da
tinta na banda x de 5pt a partir da tinta mais à esquerda (o delimitador esquerdo). Sem
dependências externas de imagem.

## Fase A — três correcções à tabela do plano

O plano pedia para remedir antes de decidir. Fez-se, e a tabela muda em três pontos.

### 1. O short fall do vanilla **está portado** — o erro era meu, do Passo 1024

`compiler/math/layout/stretchy.rs:57-61`:

```rust
// P912: subtrair DELIM_SHORT_FALL = 0.1em (0.1 * upem em design units) da dimensão alvo
// (só para delimitadores — o radical tem short_fall=0 no vanilla, P974)
let target_du = if apply_short_fall {
    apply_delim_short_fall(min_height_du, self.constants.upem)
} else { min_height_du };
```

O cristalino **multiplica e depois subtrai** — não "multiplica em vez de subtrair", como o
relatório do Passo 1024 afirmou e o plano deste passo herdou. A distinção fina do vanilla até
está lá: só delimitadores levam short fall, o radical leva `0` (vanilla
`math/ir/resolve.rs:1246`, registado em `root.md` §P974).

**Opção 1 do plano ("portar o short-fall absoluto") está, portanto, sem objecto.**

### 2. A divergência real é mais estreita: 10% de inflação no alvo

| | cristalino | vanilla ratificado |
|---|---|---|
| alvo bruto | `(ascent + descent) * 1.1` (`math/layout/mod.rs:390`) | `Rel::one()` = 100% da extensão (`math/ir/resolve.rs:843`) |
| short fall | `−0.1em` (`stretchy.rs:57-61`) | `−0.1em` (`glyph.rs:271`) |
| alvo final | `1.1·h − 0.1em` | `1.0·h − 0.1em` |

### 3. A inflação é **inerte no output** — "cresce com a altura" está refutado

O alvo não é a altura final: `select_variant(target_du)` escolhe a menor variante/montagem que
cobre o alvo, logo o resultado é quantizado.

| Varredura | Configurações | Divergência máxima |
|---|---:|---|
| `mat` 2→12 linhas, 11pt | 11 | **0,24pt** |
| `mat` 3 e 5 linhas, 8/9/10/11/12/14/16/18/20/24pt | 20 | **0,24pt** |

**0,24pt é exactamente 1 pixel a 300dpi**, e o sinal alterna (+0,24 / 0,00 / −0,24) — é
arredondamento de rasterização, não diferença geométrica. Zero configurações acima de 1 pixel.
A afirmação da primeira redacção ("~1,4pt numa grelha de 14pt, ~5pt numa de 50pt, e a
divergência cresce com a altura") é **falsa no output**: aquela é a diferença de *alvo*, e o
alvo é absorvido pela quantização.

Achado lateral, visível na extracção de texto: os dois compõem delimitadores altos de forma
**estruturalmente diferente** — o vanilla estica um glifo variante (um só caractere no PDF), o
cristalino monta a partir das peças `U+239B…U+23A0` (`⎛⎜…⎝`). Mesma altura final, mecânica
distinta. É divergência de mecânica, permitida (ADR-0107), e não estava registada — fica
registada aqui.

**Ausência de regime onde morde não é prova de que não exista**: a inflação pode transbordar
para a variante seguinte junto de uma fronteira que estas 30 configurações não apanharam.

## Fase B — as opções, corrigidas pela medição

O conjunto de opções do plano deixa de se aplicar (a Opção 1 já está feita; a Opção 2 assume
uma divergência visual que não se mede). As opções reais são:

**Opção A — alinhar o alvo a 100% (`* 1.1` → sem factor)**
Uma linha em `math/layout/mod.rs:390`. Fica com a mesma fórmula de alvo do vanilla
(`1.0·h − 0.1em`), removendo a única divergência restante deste eixo.
*A favor*: paridade de mecanismo completa; nada de "10%" inexplicado no código. Risco visual
**medido como baixo** — nas 30 configurações o output não muda (a inflação não estava a
morder). *Contra*: não é risco zero — junto de uma fronteira de variante não medida, a
remoção da inflação pode escolher uma variante **menor** e encurtar um delimitador; e toca o
caminho de produção de todo `mat`/`cases`.

**Opção B — manter os 110%, agora com a medição registada**
Não mexer no código; o registo em `_comum.md` §P912-folga já documenta o alvo divergente e a
medição de inércia.
*A favor*: zero risco; a divergência está documentada com prova, não por omissão. *Contra*:
mantém no código um factor sem contrapartida no vanilla, que um passo futuro vai reencontrar;
e deixa a fronteira de variante como risco latente não caracterizado.

**Opção C — caracterizar a fronteira antes de decidir**
Varredura fina (altura de grelha contínua, via `#box(height: …)` dentro da matriz) para
encontrar, se existir, o regime onde os 110% escolhem variante diferente. Só depois decidir
entre A e B, com o pior caso conhecido.
*A favor*: fecha a única incerteza que resta. *Contra*: mais um passo antes de qualquer
mudança.

## O que já foi feito neste passo (sem gate, porque é registo)

- `_comum.md` §P912-folga **reescrita**: corrige o meu erro do Passo 1024 (o short fall está
  portado), estreita a divergência ao alvo, e acrescenta a medição de output das 30
  configurações.
- `cases.md` e `matrix.md`: as notas de correcção passam a dizer o que a medição sustenta; a
  frase "cresce com a altura da matriz" fica explicitamente marcada como refutada.
- **Zero alteração de código.**

## Validação

```
crystalline-lint .      → 0 erros; 3 avisos V7 pré-existentes
cargo test --workspace  → 5842 passed; 0 failed
```
