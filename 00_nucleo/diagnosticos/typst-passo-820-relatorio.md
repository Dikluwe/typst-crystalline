# Relatório — typst-passo-820: `foundations::scope` — mecanismo `Deprecation` ausente (achado #7 de P810, prioridade alta)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-820.md`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree **não commitado**. Estado nas medições "antes" (`git diff HEAD --stat`, 2026-07-21 23:49 -03): 32 ficheiros alterados vs HEAD (+1708/-200) — alterações de P823/P824/P827/P814/P815/P821/P816. Estado nas medições "depois": os anteriores + os ficheiros de P820 (§Passo 2; `git diff HEAD --stat` final: 34 ficheiros, +1996/-204). Binário cristalino rebuildado (`cargo build --release`) após a implementação.
**Binários:** `./target/release/typst` (cristalino — `typst <input> -o <out.pdf>`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`).

---

## Passo 1 — Sonda (medição ANTES)

Fixtures em `temp/p820/`. Comandos exactos: `lab/typst-original/target/release/typst compile <f>.typ <f>.pdf; echo $?` e `./target/release/typst <f>.typ -o <f>.cris.pdf; echo $?`. O `join` em causa é o **símbolo math** (`$join$` / `#sym.join`), não `array.join`/`str.join` — confirmado pela reprodução do caso do relatório P810 §7.

**(a1)** `m1.typ`: `$join$`

```text
vanilla:
warning: `join` is deprecated, use `bowtie.big` instead
  ┌─ m1.typ:1:1
  │
1 │ $join$
  │  ^^^^
exit=0

cristalino (ANTES):
.../m1.typ:1:1: error: unknown variable: join
  hint: if you meant to display multiple letters as is, try adding spaces between each letter: `j o i n`
  hint: or if you meant to display this as text, try placing it in quotes: `"join"`
exit=1
```

**(a2)** `m5.typ`: `#sym.join`

```text
vanilla:  warning: `join` is deprecated, use `bowtie.big` instead  (@1:5 — span no campo)   exit=0
cristalino (ANTES):  .../m5.typ:1:1: error: module 'sym' does not contain field "join"      exit=1
```

**(b)** `m2.typ`: `$bowtie.big$`

```text
vanilla:  exit=0  (render medido por pdftotext: ⨝)
cristalino (ANTES):  .../m2.typ:1:1: error: variável desconhecida: bowtie   exit=1
```

**Sondas complementares (classificação e âmbito):**

| Documento | Vanilla | Cristalino (ANTES) |
|---|---|---|
| `m3`: `$join a b$` | warning (idem m1), exit 0 | erro `unknown variable: join`, exit 1 |
| `m6`: `$bowtie$` | exit 0, render **⋈** (sem variante bare — cai na 1.ª, `stroked`) | erro `unknown variable: bowtie`, exit 1 |
| `m7`: `$join.r$` | warning (span na raiz @1:1), exit 0, render **⟖** | erro `variável desconhecida: join`, exit 1 |
| `m8`: `#sym.bowtie.big` | exit 0, render **⨝** | erro `module 'sym' does not contain field "bowtie"`, exit 1 |
| `m9`: `$bowtie.stroked$` `$bowtie.filled$` | exit 0, render **⋈⧓** | — |
| `m4`: `$nonexistentsymbol$` | erro `unknown variable` + 2 hints, exit 1 | idem ✓ (paridade já existente, P780) |
| `m10`: `$foo.bar$` | erro `unknown variable: foo` + 2 hints (inglês), exit 1 | erro `variável desconhecida: foo` (português) — sub-achado (b) |
| `m11`: `$sym.join$` | erro `unknown variable: sym` + 3 hints (`sym` não disponível em math) | — (registado; fora de âmbito) |
| `ctrl2`: `$gt.tri$` | warning `` `gt.tri` is deprecated, use `gt.closed` instead `` (@1:4 — span na variante), exit 0 | — (scope-out, ver §Passo 2) |

**Código identificado:**

- Mecanismo geral vanilla: `Binding::deprecated(Deprecation)` em `lab/typst-original/crates/typst-library/src/foundations/scope.rs:257-373` (`Binding` carrega `Option<Deprecation>`; `scope.rs:288` marca o binding; o warning é emitido no acesso).
- **Fonte de dados** (a que faltava — débito P772l §2.6 do handoff P798): a tabela de símbolos do vanilla vem do crate **codex** — `codex-0.3.0/src/modules/sym.txt`. 14 tags `@deprecated`: **1 de topo** (`join`, linha 650: `` `join` is deprecated, use `bowtie.big` instead ``) e **13 ao nível de variante** (`gt.tri`/`gt.tri.eq`/`gt.tri.eq.not`/`gt.tri.not` :336-342, `lt.tri`+3 :373-379, `tack.*.double` ×5 :1118-1139).
- `join` no codex (:651-654): base ⨝ + variantes `r` ⟖, `l` ⟕, `l.r` ⟗. `bowtie` (:655-663): **sem variante bare**; `stroked` ⋈, `stroked.big` ⨝, `stroked.big.l` ⟕, `stroked.big.r` ⟖, `stroked.big.l.r` ⟗, `filled` ⧓, `filled.l` ⧑, `filled.r` ⧒.
- Cristalino (ANTES): tabela de símbolos em `01_core/src/engine/stdlib/sym.rs` (`SYM_SIMPLE`/`SYM_GROUPS`, `sym_lookup` :390, `build_sym_module` :419) — sem `join`, sem `bowtie`, sem qualquer noção de depreciação. Resolução math em `01_core/src/engine/eval/math.rs` — bare :212-214 (`sym_lookup` → `MathText`), callee :148-154 (erro português `variável desconhecida`). Field access genérico em `01_core/src/engine/eval/bindings.rs:1515` (`eval_field_access`, com engine) → `eval_value_field_access` :1558 (puro, sem sink).

## Passo 2 — Implementação

Tudo em L1 (puro — tabela estática + warnings via `Sink` já existente; zero I/O).

1. **`01_core/src/engine/stdlib/sym.rs`**:
   - Novos grupos `join_variants()` e `bowtie_variants()` (caracteres verbatim do codex) registados em `SYM_GROUPS` — `bowtie` fica automaticamente no scope do módulo `sym` (`#sym.bowtie.big` passa a resolver; `bowtie.big` → `stroked.big` ⨝ pelo algoritmo já existente de "menor número de modifiers extra" de `Symbol::modified`, mesmo resultado medido no vanilla).
   - Nova tabela `SYM_DEPRECATED: &[(&str, &str)]` (nome → mensagem verbatim) + `pub fn sym_deprecation(name) -> Option<&'static str>`. **Alcance: apenas `join`** — única depreciação de topo do codex e o caso medido. As 13 depreciações ao nível de **variante** (`gt.tri*`, `lt.tri*`, `tack.*.double`) ficam em **scope-out explícito** (requerem `Option<mensagem>` por variante em `SymbolVariant` — mecanismo separado), registado no L0 `sym.md` §6/§7 e medido em `ctrl2`.
2. **`01_core/src/engine/eval/math.rs`**:
   - Resolução de `MathIdent` bare (`eval_math_expr`, braço P795) e raiz de field access (`eval_math_callee`): após `sym_lookup` resolver, se `sym_deprecation(name)` → `engine.sink.warn_note(ident.span(), msg, "")` — o símbolo **resolve** (warning, nunca erro).
   - Sub-achado (b): o braço desconhecido de `eval_math_callee` deixa de emitir `variável desconhecida: {name}` (português) e passa a usar `unknown_variable_math` (P780) — paridade de mensagem com o caminho bare e com o vanilla (`$foo.bar$` medido).
3. **`01_core/src/engine/eval/bindings.rs`** (`eval_field_access`): se o target é `Value::Module` com nome `sym` e o campo está em `SYM_DEPRECATED` → warning com span **no campo** (`access.field().span()` — medido vanilla @1:5), antes da resolução normal.
4. **L0**: `00_nucleo/prompts/engine/stdlib/sym.md` — nova §7 (medição + regras + scope-out das 13 variantes); `00_nucleo/prompts/engine/eval.md` — nova §P820 (pontos de emissão + sub-achado (b)). `crystalline-lint --fix-hashes .` executado (header de `sym.rs` → `285f70fd`; 0 drift V5).
5. **Testes** (escritos antes da implementação — falharam com `cannot find function sym_deprecation` e com os erros medidos na sonda):
   - `sym.rs` (+7 unitários): lookups `join`/`join.r`/`bowtie`/`bowtie.big`/`bowtie.filled` (caracteres medidos no vanilla via `pdftotext`), `sym_deprecation` positivo/negativo.
   - `eval/tests.rs` (+6 integração, `MockWorld` + sink): `$join$` warning+Ok, `$join.r$` warning+Ok, `#sym.join` warning+Ok, `$bowtie.big$` Ok sem warning, `$bowtie$`/`#sym.bowtie.big` Ok, `$foo.bar$` erro inglês (e asserção negativa do português).

Diff (resumo, só ficheiros de P820):

```text
 01_core/src/engine/stdlib/sym.rs    | +~90  (2 grupos, SYM_DEPRECATED, sym_deprecation, 7 testes)
 01_core/src/engine/eval/math.rs     | +~25  (2 pontos de warning + mensagem inglesa no callee)
 01_core/src/engine/eval/bindings.rs | +~12  (warning em #sym.<depreciado>)
 01_core/src/engine/eval/tests.rs    | +~110 (6 testes de integração)
 00_nucleo/prompts/engine/stdlib/sym.md | §6 scope-out + §7 P820
 00_nucleo/prompts/engine/eval.md       | §P820
```

## Passo 3 — Validação (medição DEPOIS)

Mesmos comandos da sonda, binário rebuildado. Saída literal:

```text
m1  $join$:            .../m1.typ:1:1: warning: `join` is deprecated, use `bowtie.big` instead   exit=0
m2  $bowtie.big$:      (sem saída)                                                                exit=0
m3  $join a b$:        .../m3.typ:1:1: warning: `join` is deprecated, use `bowtie.big` instead   exit=0
m5  #sym.join:         .../m5.typ:1:5: warning: `join` is deprecated, use `bowtie.big` instead   exit=0
m6  $bowtie$:          (sem saída)                                                                exit=0
m7  $join.r$:          .../m7.typ:1:1: warning: `join` is deprecated, use `bowtie.big` instead   exit=0
m8  #sym.bowtie.big:   (sem saída)                                                                exit=0
m10 $foo.bar$:         .../m10.typ:1:1: error: unknown variable: foo
                         hint: if you meant to display multiple letters as is, ... `f o o`
                         hint: or if you meant to display this as text, ... `"foo"`              exit=1
```

Mensagens verbatim, spans (@1:1 raiz em math; @1:5 campo em `#sym.join`) e exit codes **batim com o vanilla** (formato de envelope cristalino `path:line:col:` vs carets — apresentação, fora de âmbito). Renders confirmados por `pdftotext` nos PDFs cristalinos: `$join$`→⨝, `$bowtie.big$`→⨝, `$bowtie$`→⋈, `$join.r$`→⟖ — idênticos ao vanilla.

**Controlos de regressão (DEPOIS):**

| Documento | Cristalino | Esperado |
|---|---|---|
| `ctrl1`: `$alpha + beta$` `$arrow.r$` `#sym.arrow.r.filled` | exit 0, sem saída | símbolos não depreciados sem warning ✓ |
| `m4`: `$nonexistentsymbol$` | erro + 2 hints, exit 1 (inalterado) | núcleo `unknown variable` intacto ✓ |
| Núcleo de scope (os 12 testes em paridade de P810 §7) | suíte `typst-core` verde | sem regressão ✓ |
| `ctrl2`: `$gt.tri$` | exit 0, **sem** warning (vanilla avisa) | **scope-out declarado** (depreciação ao nível de variante) |

**Suítes de testes (contagens ANTES/DEPOIS):**

- `cargo test -p typst-core` — ANTES: `4406 passed; 0 failed; 2 ignored`. DEPOIS: **`test result: ok. 4419 passed; 0 failed; 2 ignored`** (+13: 7 unitários em `sym.rs` + 6 integração em `eval/tests.rs`; os 13 novos falhavam antes da implementação — compile error na função inexistente + erros medidos na sonda).
- `cargo test -p typst-infra` — ANTES: `667 passed; 0 failed; 5 ignored`. DEPOIS: **`test result: ok. 667 passed; 0 failed; 5 ignored`** (inalterada — a funcionalidade é L1; o binário E2E foi validado nos comandos literais acima).
- `crystalline-lint .` — **0 violations novas**: 0 V5 após `--fix-hashes`; restam apenas os 6 warnings V7 ("prompt órfão") pré-existentes e alheios a este passo.

## Passo 4 — Estado final

| Caso | ANTES (cristalino) | DEPOIS (cristalino) | Vanilla | Estado |
|---|---|---|---|---|
| `$join$` / `$join a b$` | erro `unknown variable`, exit 1 | warning depreciação, exit 0 | warning, exit 0 | **fechado** |
| `$join.r$` | erro `variável desconhecida`, exit 1 | warning depreciação (span raiz), exit 0, render ⟖ | idem | **fechado** |
| `#sym.join` | erro `module 'sym' does not contain field "join"` | warning depreciação (span campo @1:5), exit 0 | idem | **fechado** |
| `$bowtie$` / `$bowtie.big$` / `#sym.bowtie.big` | erro (bowtie ausente) | exit 0, renders ⋈/⨝/⨝ | idem | **fechado** |
| `$foo.bar$` (sub-achado b) | erro `variável desconhecida` (PT) | erro `unknown variable: foo` + 2 hints (EN) | idem | **fechado** |
| `$gt.tri$`, `$lt.tri$`, `tack.*.double` (depreciação de variante) | silêncio | silêncio (vanilla avisa) | warning | **scope-out declarado** (13 entradas; requer mensagem por variante em `SymbolVariant`) |
| `$sym.join$` (math) | — | `sym` resolve em math no cristalino; vanilla erra `unknown variable: sym` | erro | divergência pré-existente registada, fora de âmbito |
| Mecanismo geral `Deprecation` em `Binding` (qualquer binding, não só símbolos) | ausente | tabela data-driven `SYM_DEPRECATED` cobre símbolos de topo | `Binding::deprecated` genérico | parcial por desenho do prompt (âmbito = casos medidos) |
