# Prompt L0 — `compiler/stdlib/structural/math` — nativas de matemática
Hash do Código: 88c233dc

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/math.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/math/{accent,cancel,op,underover,style}.rs`. Co-mudança: P290-301 e P317 movem `native_accent`, `native_cancel`, `native_op`, `native_underover`, `op_value` e `make_math_module` como bloco.

---

## Contexto

As nativas de matemática que vivem no stdlib estrutural — os construtores de `Content`
matemático e a montagem do módulo `math`. O **layout** destes elementos é outro eixo
inteiro (`compiler/math/layout/`); este nó só constrói o conteúdo.

**Técnica**: `make_math_module` monta um `Value::Module` com scope fechado de
funções, operadores, espaçamentos e símbolos math. A tabela é construída uma
vez; não há lookup dinâmico por registry.

## Instrução

| Nativa | Assinatura | Emite |
|---|---|---|
| `accent` | `accent(base, accent)` — ambos posicionais obrigatórios | `Content::MathAccent` |
| `cancel` | `cancel(body)` | `Content::MathCancel` |
| `class` | `class(class, body)` | `Content::MathClassOverride` |
| `underover` | `underover(base, under: ?, over: ?)` | `Content::MathUnderover` |
| `op` | `op(text, limits: false)` | `Content::MathOp` |

### `math.class` — as 10 classes

`class` é obrigatório e tem de ser **uma das 10 strings do cast do vanilla** (P772y,
P825a). Nome desconhecido produz a **mesma mensagem de cast** do vanilla; argumento
não-string reporta o tipo no formato longo, via `vanilla_type_name_class`
(`str → string`, `int → integer`, …). Variantes internas fora do cast são rejeitadas.

### `make_math_module()`

Constrói `math` como `Value::Module` com os **41 operadores** vanilla
pré-definidos, o elemento chamável `equation`, `op`, os cinco
espaçamentos nomeados (P895) e as funções math explicitamente registradas.
`dif`/`Dif` levam `upright` explícito — não itálico (§P962).

### P1140.3-A — `math.sqrt` função, não símbolo

**Medição anterior à decisão (2026-08-23, vanilla `a51e02804`):**
`repr(type(math.sqrt)) == "function"` e `repr(math.sqrt([x])) ==
"root(radicand: [x])"`. No cristalino anterior, `math.sqrt` era o símbolo `√`
importado de `sym`, logo não chamável; `sym.sqrt` também era um path extra que
o vanilla rejeita. O avaliador de modo math já construía corretamente
`Content::math_root(None, radicand)` por um braço sintático duplicado.

O módulo registra `math.sqrt` como `Value::Func` nativa antes da cópia dos
símbolos. A nativa aceita exatamente um `Content`, rejeita named args e emite
`Value::Content(Content::math_root(None, radicand))`. O modo math deve delegar
à mesma unidade sem duplicar o contrato de aridade. A entrada extra
`sym.sqrt` é removida da tabela cristalina; a cópia `sym → math` nunca
sobrescreve funções já registradas. `calc.sqrt` permanece numérico e separado.

`math.root` é `MISSING_MEMBER`, não `WRONG_KIND`, e fica fora de P1140.3-A até
passo próprio classificá-lo.

### P1140.3-B — primeira fase pública de `math.equation`

**Medição anterior à decisão (2026-08-23, vanilla `a51e02804`):**
`repr(type(math.equation)) == "function"`; o caso mínimo representa como
`equation(body: [x])`, e `block: true` como
`equation(block: true, body: [x])`. Ausência de `body`, segundo positional e
named desconhecido são erros. O vanilla também aceita `numbering`,
`number-align`, `supplement` e `alt`.

P1140.3-B substitui o `Value::None` histórico por uma função/elemento chamável
que aceita exatamente um `Content` posicional obrigatório e `block: bool`,
default `false`, e produz `Content::Equation`. A nativa e a sintaxe `$...$`
partilham a construção dona da morfologia. Named args desconhecidos ou ainda
não materializados são rejeitados, nunca ignorados.

**Incompletude deliberada:** `numbering`, `number-align`, `supplement` e `alt`
ficam fora desta primeira fase e serão materializados pelo **P1140.4**, com
transporte, `repr`, `set`/`show`, referências e acessibilidade. O transporte já
vigente de `#set math.equation(numbering: ...)` pela style chain permanece.

## Restrições Estruturais

- L1 puro.
- As mensagens de erro de `class` são o observável (ADR-0108) — comparar verbatim, não
  por classe de erro.

## Critérios de Verificação

```
$accent(a, hat)$              → MathAccent
$cancel(x)$                   → MathCancel
$class("binary", x)$          → MathClassOverride
$class("zz", x)$              → Err (mesma mensagem de cast do vanilla)
$class(1, x)$                 → Err "found integer" (nome longo)
$op("lim", limits: true)$     → MathOp com limits
math.op existe no módulo      → true (P895)
repr(type(math.equation))     → "function"
repr(math.equation([x]))      → "equation(body: [x])"
repr(type(math.sqrt))         → "function"
repr(math.sqrt([x]))          → "root(radicand: [x])"
sym.sqrt                      → campo ausente
$dif$                         → upright, não itálico (§P962)
```

## P1132m — forma completa de `dif`/`Dif`

O binding reproduz `math/op.rs`: uma `Sequence` de `HSpace` absoluto
`Length::em(1/6)` com `weak=true`, seguido de `MathClassOverride(Unary)`
sobre o `MathStyled(italic:false)` já especificado por P962. A unidade `em`
torna o espaço proporcional ao tamanho matemático activo. O colapso nas
bordas pertence ao layouter (`compiler/math/layout/_comum.md` §P1132m).
