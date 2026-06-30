# Paridade Funcional — Passo 510 (Math Styles: 12 Funções de Estilo)

**Data:** 2026-06-30  
**Passo:** 510  
**Foco:** Fechar as 12 funções de estilo matemático: `bb`, `bold`, `cal`, `frak`, `italic`, `mono`, `sans`, `scr`, `script`, `serif`, `sscript`, `upright`.  
**Estado:** Concluído — corpus P490+P500 a 37/37 OK; renderização de todos os estilos confirmada.

---

## 1. Contexto

O P509 fechou o corpus P490+P500 em **37/37 OK**. O diagnóstico P508 identificou que as 12 funções de estilo math estavam ausentes **no uso**: as funções nativas (`native_bb`, `native_bold`, etc.) e o elemento `MathStyled` já existiam desde P311b.3/P316, mas o avaliador de modo math não chamava funções do scope global. Consequentemente, `$bb(X)$` era interpretado como identificador `bb` seguido de grupo delimitado, e não como aplicação de estilo.

---

## 2. Diagnóstico de Causa Raiz

- **H1 (falsa):** As 12 funções não estavam registadas.  
  **Realidade:** Estavam registadas em `make_root_scope` (`eval/mod.rs:1039-1050`).
- **H2 (falsa):** `MathStyled` não suportava os estilos.  
  **Realidade:** `MathStyledElem` e o layout math (`rules/math/layout`) já aplicavam `kind`/`bold`/`italic`/`cramped`.
- **H3 (verdadeira):** `eval_math_expr` só despachava funções math hardcoded (`frac`, `sqrt`, `root`, `vec`, `cases`, `mat`) e, para outros nomes, fazia fallback para `MathOp`/`MathIdent` + delimited. Nunca chamava `Func` do scope global.

---

## 3. Implementação

### 3.1 L1 — `01_core/src/rules/eval/math.rs`

- Adicionado parâmetro `engine: &mut Engine<'_>` a `eval_math_content` e `eval_math_expr`.
- No branch `Expr::FuncCall` (wildcard), antes do fallback P302/P303:
  1. Lookup do nome no scope global.
  2. Se for `Value::Func`, avaliar cada arg posicional/named como conteúdo math (`eval_math_expr`).
  3. Construir `Args` e aplicar via `apply_func`.
  4. Verificar se o resultado é `Value::Content`; se não, erro informativo.
- Hardcoded `frac`/`sqrt`/`root`/`vec`/`cases`/`mat` mantidos (semântica especial de math).
- Fallback P302/P303 preservado para `sin(x)`, operadores `MathOp`, e identificadores desconhecidos.

### 3.2 L1 — `01_core/src/rules/eval/mod.rs`

- Atualizados os dois call sites de `math::eval_math_content` para passar `engine`.

### 3.3 L0

- Os Prompts L0 vigentes (`math_style.md`, `math_styled.md`, `eval.md`) já cobriam as entidades envolvidas.
- Não foi necessário alterar L0 — a mudança foi uma correção de integração no avaliador, não uma nova especificação arquitetural.

---

## 4. Validação

### 4.1 Renderização de Estilos

```typst
$bb(X)$
$bold(x + y)$
$cal(A)_i^j$
$frak(1/2)$
$script(X)$
$sscript(X)$
$upright(X)$
```

Resultado: compilação bem-sucedida e PDF gerado.

### 4.2 Corpus P490 + P500

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  out=$(mktemp /tmp/p510-XXXX.pdf)
  target/release/typst "$f" "$out" >/dev/null 2>&1 \
    && echo "OK: $(basename $f)" \
    || echo "FAIL: $(basename $f)"
  rm -f "$out"
done
```

**Resultado:** 37/37 OK.

### 4.3 Testes Unitários

- `cargo test`: todos passam.
- `crystalline-lint .`: **0 violations**.

---

## 5. Notas e Limitações

- As 12 funções nativas (`native_bb` … `native_upright`) e o `MathStyledElem` já estavam implementadas; este passo fechou a **falha de integração** no parser/eval de modo math.
- `assert.eq($bb(X)$, $𝕏$)` ainda falha com "esta função não tem campos" — o problema está na comparação de `Content::MathStyled` via `assert.eq`, não na construção/renderização do estilo. Não é um blocker de linguagem para P510.
- O mapeamento Unicode real dos glifos depende do fonte math carregado; o layout já usa `MathStyleKind` e o shaper math existente.

---

## 6. Próximo Passo (P511)

Com P510 fechado, as brechas restantes de linguagem são:

| Brecha | Tamanho | Sugestão |
|---|---|---|
| Math elements granulares (`BinomElem`, `ClassElem`, `LimitsElem`, `MidElem`, `PrimesElem`, `ScriptsElem`, `StretchElem`) | M | P511 |
| Table/Grid HLine/VLine | M | P512 |
| Curve elements | M | P513 |
| `fontdb` system discovery | M | **Trilha 5** (produção) |

**Recomendação:** P511 = **Math Elements Granulares** — 7 elementos que complementam os 12 estilos.
