# Relatório — Passo 302 (Bug fix `sin(x)` parens)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-302.md`
**Diagnóstico Fase A**: `00_nucleo/diagnosticos/diagnostico-sin-parens-passo-302.md`
**Tipo declarado spec**: bug latente fixed durante materialização
dependente; reaplica sub-padrão §8.4 P288.
**Hipótese adoptada**: **HZ** (vanilla emite `op + delimited`) +
**A.2 → (a)** preservação via `MathSequence` + `MathDelimited` +
**A.3 → (β)** auto-lookup com args preservados.
**Baseline P301**: 2 870 testes  →  **P302**: 2 876 testes (Δ = +6)
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**19º passo
consecutivo**: P282→P302)
**Hash `content.rs`**: `82d3c47d` inalterado
**ADRs meta novas**: 0

---

## §1 — Sumário executivo

P302 resolve o bug latente exposto em P301 §9: `$sin(x)$` em math
mode descartava silenciosamente os args `(x)` quando
`lookup_math_op` encontrava operador.

**Modificação localizada** em `01_core/src/rules/eval/math.rs` arm
`FuncCall` fallback: quando lookup encontra operador, preservar
args via `MathSequence([MathOp, MathDelimited((x))])` —
**emulação do comportamento vanilla** (parser vanilla distinguish
`sin(x)` como `sin` + `(x)` delimited; cristalino emula no eval
sem refactorar parser).

**Resultado funcional**: `$sin(x)$` agora renderiza correctamente
com args visíveis; `$sin()$` (args vazios) emite só MathOp;
`$sin x$` (sem parens) preservado bit-exact P301; `$undef(x)$`
fallback original preservado.

**Resultado metodológico — §8.4 sub-padrão N=2 cumulativo
confirmado**:
- N=1 P288: NBSP fix descoberto via SmartQuote P287 materialização.
- **N=2 P302**: sin parens fix descoberto via auto-lookup P301
  materialização.

Padrão "bug latente fixed durante materialização dependente"
reaplica genuinamente. N=2 — longe de limiar tentativo N≥3 para
promoção formal. Preserved como **ferramenta emergente**.

**Lição metodológica registada**: P301 §A.5 falhou cobertura
cross-construct (cenários combinando feature nova com sintaxe
pré-existente). P302 §A.5.1 documenta que A.5 deve incluir
**matrix feature × sintaxe**.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=9 reaplica §8.7') | Bug factual confirmado; vanilla HZ confirmado; magnitude **média** |
| A.0 (ADR-0098) | ✅ `export.rs` preservado bit-exact (19º passo) |
| A.1 inventário | `MathSequence` + `MathDelimited` + `MathOp` todos pré-existentes |
| A.2 decisão | **(a)** MathSequence + MathDelimited via composição |
| A.3 integração | **(β)** auto-lookup com preservação args |
| A.4 emit | **(i)** FrameItem standard; hash preservado |
| A.5 bugs latentes | 7 cenários verificados; lição metodológica P301 §A.5 falhou cross-construct |
| A.5' anti-reflexão | **N=12 cumulativo** (P291-P302); 5 elementos novos |

Detalhe completo: `00_nucleo/diagnosticos/diagnostico-sin-parens-passo-302.md`.

---

## §3 — Materialização

### §3.1 — `01_core/src/rules/eval/math.rs` — FuncCall fallback estendido

```rust
// Outros nomes: P301 auto-lookup math (sin, cos, lim, …)
// + P302 preservação args via MathSequence + MathDelimited.
//
// **P302 (HZ confirmado)**: vanilla parser distinguish
// `sin(x)` como `sin` + `(x)` delimited; cristalino parser
// produz FuncCall em math mode (divergência) mas eval
// emula comportamento vanilla retornando
// `MathSequence([MathOp, MathDelimited((x))])`.
_ => {
    if let Some(op) = lookup_math_op(scopes, &name) {
        let pos_args: Vec<Expr<'_>> = call.args().items()
            .filter_map(|a| match a { Arg::Pos(e) => Some(e), _ => None })
            .collect();
        if pos_args.is_empty() {
            // `sin()` — args vazios; só operador.
            return Ok(op);
        }
        let body = if pos_args.len() == 1 {
            eval_math_expr(scopes, ctx, pos_args[0])?
        } else {
            // Múltiplos args: separados por `, ` (paridade vanilla).
            let mut items: Vec<Content> = Vec::new();
            for (i, expr) in pos_args.iter().enumerate() {
                if i > 0 {
                    items.push(Content::MathText(", ".into()));
                }
                items.push(eval_math_expr(scopes, ctx, *expr)?);
            }
            Content::MathSequence(std::sync::Arc::from(items))
        };
        let delimited = Content::MathDelimited {
            open:  '(',
            body:  Box::new(body),
            close: ')',
        };
        Ok(Content::MathSequence(std::sync::Arc::from(vec![op, delimited])))
    } else {
        Ok(Content::MathIdent(name.into()))
    }
}
```

**Reutilização total** de variants existentes:
- `Content::MathOp` (P298).
- `Content::MathDelimited` (pré-existente desde P55+).
- `Content::MathSequence` (pré-existente).
- `Content::MathText` (separador `, ` para múltiplos args).

**Zero variants novos**. P302 é puramente **composicional**.

### §3.2 — Zero alterações em outros sítios

| Componente | Pós-P302 |
|---|---|
| `lookup_math_op` helper (P301) | **Inalterado** |
| `MathIdent` arm (P301 modificado) | **Inalterado** — `$sin x$` sem parens preservado |
| Heurística P298 `attach.rs` | **Inalterado** |
| `make_math_module` P299 | **Inalterado** |
| Outras arms math/eval | **Inalterados** |
| L0 `rules/eval.md` | **Inalterado** — extensão dentro do contrato |
| `03_infra/src/export.rs` | **Inalterado bit-exact** — hash `66cb8ac3` (19º passo) |

---

## §4 — Testes

### §4.1 — `01_core/src/rules/eval/tests.rs` (+6 testes L1)

| Teste | Verifica |
|---|---|
| **`p302_sin_parens_produz_mathsequence_com_delimited`** | `$sin(x)$` → `MathSequence([MathOp(sin), MathDelimited((x))])` — bug fix verificado |
| `p302_lim_parens_preserva_limits_e_args` | `$lim(x)$` — limits-style + args |
| `p302_sin_args_vazios_so_mathop` | `$sin()$` → só MathOp (sem MathDelimited vazio) |
| `p302_undef_parens_continua_mathident_fallback` | `$undef(x)$` — operador desconhecido; fallback `MathIdent("undef")` |
| **`p302_regressao_sin_sem_parens_preservado`** | **INVARIANTE CRÍTICA**: `$sin x$` continua MathOp directo sem MathDelimited |
| `p302_multiplos_args_separados_por_virgula` | `$sin(x, y)$` — separação por `, ` |

Helper de teste novo `find_mathdelimited_in()` recursivo.

### §4.2 — Sem alterações em L3

P302 não modifica emit. Tests L3 existentes preservados.

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2372 passed; 0 failed; 0 ignored
test result: ok.  457 passed; 0 failed; 6 ignored
test result: ok.   24 passed; 0 failed; 0 ignored
test result: ok.    2 passed; 0 failed; 0 ignored
test result: ok.   21 passed; 0 failed; 0 ignored
                  -----
                  2876 passed total
```

Baseline P301 = 2 870; delta = **+6** (6 L1) ✓.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — `crystalline-lint --fix-hashes`

```
Nothing to fix
```

L0 `rules/eval.md` inalterado — P302 é **extensão dentro do
contrato L0 existente**, paralelo a P301.

### §5.4 — Hashes pós-P302

| Ficheiro | Pós P302 |
|---|---|
| `infra/export.rs` (`@prompt-hash`) | **`66cb8ac3` preservado bit-exact** (**19º passo consecutivo**) |
| `entities/content.rs` (`@prompt-hash`) | `82d3c47d` inalterado |
| `eval/math.rs` (`@prompt-hash`) | hash L0 inalterado |
| L0 markdown | todos preservados |

---

## §6 — Padrões metodológicos

### §6.1 — **Sub-padrão §8.4 "bug latente fixed durante materialização dependente" — N=2 cumulativo confirmado**

| Aplicação | Origem | Bug | Materialização dependente |
|---|---|---|---|
| **N=1 P288** | P287 SmartQuote materialização | NBSP eliminado por `split_whitespace()` | Fix em layout_word preservando NBSP |
| **N=2 P302** | P301 auto-lookup materialização | Args `(x)` descartados em FuncCall fallback | Fix via `MathSequence + MathDelimited` |

**Características comuns**:
- Bug introduzido (ou exposto) por materialização anterior.
- Cobertura A.5 do passo origem falhou capturar.
- Magnitude controlada (XS-S).
- Fix composicional (sem variants novos).

**N=2 cumulativo**. Longe de limiar N≥3 para promoção formal.
Preserved como **ferramenta emergente**.

### §6.2 — §8.7' "A.0.0 template" — N=9 magnitude média

| Passo | A.0.0 N | Magnitude |
|---|---:|---|
| P293-P301 | 1-8 | varia |
| P300 | (retrospectivo) | n/a |
| **P302** | **9** | **média** |

**Janela P294-P302**: máxima, baixa, média, alta, alta, baixa,
média-modesta, **média**. Continua não-monotónico — flutuação
saudável preservada.

§8.7' N=9 **adiado** seguindo standard P300/P301 (anti-padrão
conservador).

### §6.3 — §8.3 "refutação pragmática" — N=12 sem aplicação P302

P302 **confirma** bug factual (não refuta spec). §8.3 não dispara.
N=12 candidato preservado.

### §6.4 — §8.6 "A.5' anti-reflexão" — N=12 cumulativo

P291-P302 (P300 incluído como auditoria retrospectiva).

### §6.5 — ADR-0098 "single source of truth" — N=19 cumulativo

Hash `export.rs 66cb8ac3` preservado bit-exact pelos **19 passos
consecutivos** P282-P302. Invariante robusta sobre 19 features
distintas. P302 é o **19.º passo** — preservação por paradigma
"modificação eval interna composicional".

### §6.6 — Anti-padrão P273.17 §0 — 10 passos consecutivos honrados

**0 ADRs meta promovidas P293-P302** (10 passos):

| Passo | Candidatos avaliados | Promovidos |
|---|---:|---:|
| P293-P301 | 9 passos cumulativos | 0 |
| **P302** | **§8.4 N=2 + §8.7' N=9** | **0** |

**10.ª vez consecutiva** anti-padrão honrado.

### §6.7 — Lição metodológica registada: A.5 cross-construct

P301 §A.5 cobriu cenários "feature nova" mas falhou capturar
combinação "feature nova + sintaxe pré-existente". Bug `sin(x)`
emergiu pós-implementação.

**Lição P302 §A.5.1**: A.5 deve incluir **matrix feature ×
sintaxe**:

```
                  Sintaxe A (e.g. $f x$)   Sintaxe B (e.g. $f(x)$)
Feature nova     ✓ (P301 §A.5)            ✗ (P301 falhou; bug latente)
Feature antiga   ✓ (regressão bit-exact)   ✓ (regressão bit-exact)
```

Aplicação futura: A.5 de P303+ deve verificar matrix completa.

---

## §7 — Cobertura vanilla vs cristalino

P302 não altera tabelas de cobertura. **Paridade vanilla atingida**
para `$<op>(<args>)$` syntax via emulação no eval:

| Caso | Antes P302 | Pós P302 |
|---|---|---|
| `$sin x$` | `MathOp(sin)` ✓ P301 | `MathOp(sin)` ✓ preservado |
| `$sin(x)$` | `MathOp(sin)` (bug: args descartados) | `MathSequence([MathOp(sin), MathDelimited((x))])` ✓ |
| `$lim(x)$` | `MathOp(lim, limits=true)` (bug) | `MathSequence([MathOp(lim, limits=true), MathDelimited((x))])` ✓ |
| `$undef(x)$` | `MathIdent("undef")` (args descartados pré-existente) | `MathIdent("undef")` (preserved; fora de scope) |
| `$sin()$` | `MathOp(sin)` ✓ | `MathOp(sin)` ✓ caso degenerate |

**Paridade vanilla** para os 42 operadores P299 × `(args)` syntax
atingida.

---

## §8 — Frentes pendentes pós-P302

P302 fecha bug latente P301 §9. **Frentes restantes** (catálogo
em `frentes-pendentes-pos-p299.md`):

| Frente | Magnitude | Estado |
|---|---|---|
| **P295.1** nota corpo no rodapé | L | P300 prioridade #1 |
| `$undef(x)$` args descartados | XS+ | Bug latente pré-P301; cobertura potencial futura |
| P296.X toggles cancel | XS | refino cluster math |
| P297.X UnderoverKind | XS+ | cosmético |
| Cosméticos ADR-0054 graded (vários) | XS individual | agregável |

---

## §9 — Decisão sobre P303

P302 fecha bug derivado P301. P303 disponível para qualquer
frente:

1. **P295.1 nota corpo no rodapé** — prioridade P300.
2. **`$undef(x)$` args fix** — bug latente irmão (XS+).
3. **Cosméticos cleanup agregado** — múltiplos XS.
4. **Frente totalmente nova** — fora do escopo P283-P302.

Decisão fica para o operador humano.

---

## §10 — Honestidade epistémica

### §10.1 — Padrão §8.4 é genuíno, não inflado

P288 NBSP fix e P302 sin parens fix são **genuinamente distintos**
mas **arquitecturalmente paralelos**:
- Ambos descobertos pós-materialização anterior.
- Ambos fix composicional sem variants novos.
- Ambos confirmam que A.5 do passo origem falhou capturar.

N=2 cumulativo **é genuíno** — não inflado. Mas N≥3 ainda
necessário para promoção formal.

### §10.2 — A.5 cross-construct: lição honesta

P301 §A.5 não cobriu `$sin(x)$`. Lição metodológica registada em
P302 §A.5.1. **Não é culpa de P301** — é descoberta empírica que
**A.5 precisa de matrix mais ampla** quando features tocam
sintaxe pré-existente.

Aplicação futura: passos com features arquitecturalmente novas
(P303+) devem incluir A.5 matrix feature × sintaxe.

### §10.3 — Reutilização vs criação

P302 reusa **4 variants existentes** (`MathOp`/`MathDelimited`/
`MathSequence`/`MathText`). **Zero novos variants** — confirma
que cristalino tem infraestrutura composicional rica.

Pattern emergente: **bug fixes derivados podem ser puramente
composicionais** se infraestrutura suporta.

---

## §11 — Fecho

P302 fechado com:

- **+6 testes** (6 L1, 0 L3) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **0 drift** em hashes (`--fix-hashes` "Nothing to fix").
- **Hash `export.rs` preservado** bit-exact (19º passo consecutivo).
- **Hash `content.rs` inalterado**.
- **L0 `rules/eval.md` inalterado** — extensão dentro do contrato.
- **0 ADRs meta novas** — 10.ª vez consecutiva anti-padrão honrado.
- **Sub-padrão §8.4 N=2 cumulativo confirmado** (P288 + P302).

**MARCO P302**:
- **Bug latente P301 §9 resolvido** — `$sin(x)$` produz output
  correcto via emulação vanilla.
- **Sub-padrão §8.4 N=2 cumulativo genuíno** — "bug latente fixed
  durante materialização dependente" reaplica.
- **A.0.0 N=9 magnitude média** — bug factual concreto; vanilla
  HZ confirmado.
- **Reutilização total variants existentes** — MathSequence +
  MathDelimited + MathOp + MathText sem novos variants.
- **Hash `export.rs` preservado pelo 19º passo consecutivo**
  (P282→P302) — ADR-0098 robusta sobre 19 features distintas.
- **Lição metodológica registada** — A.5 deve incluir matrix
  feature × sintaxe para captar bugs cross-construct.
- **12.º paradigma distinto na sequência cumulativa** — "bug fix
  derivado de materialização anterior".

**Lição final**: P302 prova que **infraestrutura composicional
rica** (variants ortogonais como MathSequence/MathDelimited)
permite fixes localizados sem inflação arquitectural — bug fixes
derivados são **virtude do design**, não falha.
