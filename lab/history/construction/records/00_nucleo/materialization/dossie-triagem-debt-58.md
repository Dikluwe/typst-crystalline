# Dossiê do gatilho DEBT-58 — material para a triagem

**Produzido em**: P328 (Lote 13, ao fechar `Figure`). **Decisão de desenho:
zero** — este dossiê é o *material com que a triagem começa*, não a triagem.
Referência cruzada: `typst-passo-328-relatorio.md` (§ "Balanço da fase"),
`00_nucleo/DEBT.md` (DEBT-58), `00_nucleo/modelo-lote-migracao-d.md`
(Contabilidade).

Com `Figure` migrada, os **element-shaped esgotam (restantes 0)** e o **gatilho
do DEBT-58 dispara**. Restam **15 variantes** não migradas: 4 `Set*` (fora da
triagem, destino F/99.E) + 11 da classe DEBT-58.

---

## 1. Inventário das variantes não migradas

Largura = `grep -rnE "Content::X\b" 01_core/src/ | wc -l` (refeito em P328, **não**
o de P317). "Menções no hub" = arms/usos em `content.rs`.

### Classe: primitivos de AST / math

| Variante | Largura (hoje) | Menções hub | Forma no hub |
|----------|---------------|-------------|--------------|
| `MathIdent` | 103 | 16 | leaf `(EcoString)` — arm próprio em plain_text/map_*; terminal |
| `MathText` | 42 | 7 | leaf `(EcoString)` — terminal |
| `MathSequence` | 25 | 6 | `(Arc<[Content]>)` — recursa em map_content; terminal map_text |

### Classe: cola estrutural

| Variante | Largura (hoje) | Menções hub | Forma no hub |
|----------|---------------|-------------|--------------|
| `Sequence` | 216 | 12 | `(Arc<[Content]>)` — o container universal; recursa em tudo |
| `Empty` | 156 | 26 | unit — o neutro; terminal em todos os matches |
| `Block` | 121 | 26 | struct ~14 campos — container body + 13 cosméticos |

### Classe: cola de texto (guideline C2/P319)

| Variante | Largura (hoje) | Menções hub | Forma no hub |
|----------|---------------|-------------|--------------|
| `Text` | 81 | 9 | leaf `(EcoString, TextStyle)` — folha de texto; observação P326 |
| `Space` | 22 | 11 | unit — cola de whitespace entre Text |

### Classe: wrappers a triar

| Variante | Largura (hoje) | Menções hub | Forma no hub |
|----------|---------------|-------------|--------------|
| `Styled` | 78 | 25 | `(Box<Content>, Styles)` — wrapper transparente de estilo |
| `Boxed` | 69 | 18 | struct ~10 campos — box inline container |
| `Labelled` | 57 | 7 | struct `{ target, label }` — wrapper de label |

### Fora da triagem — `Set*` (4) → destino F / DEBT 99.E (só para a conta fechar)

| Variante | Largura (hoje) | Menções hub |
|----------|---------------|-------------|
| `SetHeadingNumbering` | 64 | 4 |
| `SetEquationNumbering` | 18 | 4 |
| `SetPage` | 10 | 5 |
| `SetFigureNumbering` | 7 | 4 |

> Nota: a medição de F vive em `medicao-pre-f-passo-318.md`; estes números são
> de hoje (a largura cresce/encolhe com o estado do hub).

**Conta de fecho**: 62 migradas + 4 `Set*` + 11 DEBT-58 = **77** ✓.

---

## 2. As perguntas que a triagem deve responder (sem respondê-las aqui)

1. **Primitivos de AST/math** (`MathIdent`/`MathText`/`MathSequence`):
   permanecem no hub como primitivos, ou viram `Elem` como o resto da família
   math (Lote 2 migrou os *ricos* math; estes 3 ficaram de fora por serem
   "primitivos de AST")? O custo de os deixar no hub é o estado misto que o F
   herda.
2. **Cola estrutural** (`Sequence`/`Empty`/`Block`): `Sequence`/`Empty` são o
   container universal e o neutro — são "elementos" ou infraestrutura do
   próprio `Content`? `Block` tem ~14 campos (denso como o bloco grid/table do
   Lote 12) — é lote tardio ou primitivo?
3. **Cola de texto** (`Space`/`Text`): seguem a *guideline de cola* (C2/P319 —
   cola de texto fica fora do modelo D)? `Text` é a folha mais fundamental; a
   sua migração mexeria em quase todo o layouter.
4. **Wrappers** (`Styled`/`Boxed`/`Labelled`): são primitivos (transparentes ao
   conteúdo) ou um lote tardio do modelo D? `Styled`/`Labelled` são
   transparentes (descem no body/target); `Boxed` é denso (~10 campos).
5. **Custo do estado misto para o F**: o que os ~15 arms remanescentes nos 6
   matches custam à decisão F / DEBT 99.E (a superfície da StyleChain)?

---

## 3. Estado final do hub após L13

Os **6 matches** de `content.rs` (`plain_text`, `is_empty`, `map_content`,
`map_text`, `get_field`, `PartialEq`/`eq`) despacham **62 variantes migradas**
em 1 linha cada (`Self::X(e) => e.metodo()` / `(Self::X(a), Self::X(b)) => a == b`).
O que **resta** (arms próprios, não-dispatch) são as **15 não migradas**:

- **Terminais/leaf** (clonam ou devolvem valor direto): `Empty`, `Space`,
  `Text`, `MathIdent`, `MathText` + os 4 `Set*` (markers).
- **Containers com recursão própria**: `Sequence` (Vec), `MathSequence` (Vec),
  `Block` (body + cosméticos), `Boxed` (body + cosméticos), `Styled` (body),
  `Labelled` (target).

`content.rs`: **5135 linhas** (de 5782 no P313 — **−647 acumulado**). O hub
encolheu apesar de 15 variantes ainda terem arms próprios verbosos; a triagem
decide quanto mais pode encolher.

A **fotografia para o F**: os 6 matches têm hoje, cada um, ~15–20 arms
não-dispatch (as 15 não migradas, algumas com arm em vários matches). É essa a
superfície que a decisão F / DEBT 99.E (StyleChain) e a triagem do DEBT-58
consomem.

---

## Nota de fecho (P329) — a triagem aconteceu

Este dossiê era o **material**; a **triagem** está gravada (P329, decisões do
dono):

- **DEBT-58** (`00_nucleo/DEBT.md`): triado. 7 primitivos declarados (4
  definitivos + 3 provisórios), `Styled`→F, `Block`/`Boxed`/`Labelled`→lotes.
- **L0 do content** (`prompts/entities/content.md`): os 7 primitivos como
  desenho declarado do hub.
- **Contabilidade** (`modelo-lote-migracao-d.md`): roteiro L14/L15 + perf + F.

**Verificação mecânica** (a única pendência factual): `grep` confirmou que a
álgebra (`sequence()`) constrói `Sequence`/`Empty`; `Block` aparece só no seu
construtor ergonómico → **lote tardio** (L15), não primitivo. Binário resolvido
sem surpresa.

**Critério do dono**: fidelidade ao vanilla é *de comportamento*, não *de
estrutura Rust*.
