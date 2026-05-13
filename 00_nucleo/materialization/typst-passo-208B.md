# Passo 208B — Infra `current_location` em `EvalContext` + `native_here()`

**Série**: 208 (sub-passo `B`).
**Marco**: M9c (Bloco IV — `here()`).
**Tipo**: implementação cross-modular (infra + stdlib
func).
**Magnitude**: S-M (~2-3h) — depende de sub-mecanismo
fixado em C1.
**Pré-condição**: P208A concluído; ADR-0076 anotado;
trait 26 métodos; `EvalContext` com `introspector`
snapshot read-only (P174); pattern stdlib `native_X`
uniforme; tests 1899 verdes; 0 violations.
**Output**: 1 ficheiro (relatório curto).

---

## §1 Trabalho

Materializar `here()` stdlib func + infra subjacente em
`EvalContext`. Sub-mecanismo (i)/(ii)/(iii) decidido em
C1 com base em evidência empírica.

Reuso de dados P208A:

- Pattern stdlib uniforme `native_X(ctx, args, world,
  current_file, figure_numbering)` (A2).
- `EvalContext` em `01_core/src/rules/eval/mod.rs:86`
  com `introspector: TagIntrospector` snapshot read-only
  (A3).
- Sub-mecanismos avaliados em P208A A6 caveat:
  - (i) Eval walk avança Locator em locatable
    boundaries (mirror P185C Layouter).
  - (ii) `here()` placeholder eval-time, resolved
    layout-time (introduz `Content::Context`).
  - (iii) `here()` retorna Location do snapshot da
    iter anterior — cross-iter.
- Selector cristalino actual: `Kind` only (P175 minimal).

---

## §2 Cláusulas (5)

### C1 — Diagnóstico breve: sub-mecanismo fixado

Antes de tocar código, inventário focado em **3
sub-secções**:

1. **Vanilla `#context { ... }` block**: como evaluator
   vanilla materializa `here()` retornando Location
   real?
   - Localizar literalmente em
     `lab/typst-original/crates/typst-library/src/foundations/`.
   - Confirmar se vanilla usa contextual-block (sub-mecanismo
     ii análogo) vs locator-walk eval-time (sub-mecanismo
     i análogo).
2. **Cristalino eval walk actual**: identificar
   literalmente onde `EvalContext.introspector` é
   construído + onde `EvalContext` é avançado durante
   `eval_*`. Esperado per A3+A4: `EvalContext` é
   snapshot estático por iter; sem avanço interno.
   - Tem cristalino algum locator que avança em eval-time
     análogo ao `current_location` do Layouter (P185C)?
   - Se não, custo de adicionar é decisivo para C2.
3. **Consumer real imediato**: confirmar que `here()`
   tem caller esperado em P209 (`Selector::Before/After`
   que delegam a `here()` per P207A C5 plano). Sem
   consumer real, mesmo `here()` é over-engineering.

Critério literal para C2:

- **(i) — Locator eval-time**: se C1.2 mostrar que
  eval walk é refactorable trivialmente (~30-50 LOC)
  E consumer P209 confirmado.
- **(ii) — `Content::Context` block**: se C1.1
  revelar que vanilla usa contextual-block + cristalino
  não tem precedente; custo M+ (refactor profundo);
  preferido se semântica vanilla `#context` é
  imprescindível.
- **(iii) — Snapshot iter anterior**: se C1.2 mostrar
  custo prohibitivo para (i)/(ii) E consumer cross-iter
  é suficiente. Custo S (~30min) mas semântica diverge
  vanilla.

C1 fixa **uma** opção.

### C2 — Materializar opção fixada

L0 primeiro:

- Edição `00_nucleo/prompts/rules/eval/eval_context.md`
  (ou nome correcto) — documentar `current_location`
  field + semântica per opção C1.
- Novo L0 `00_nucleo/prompts/stdlib/here.md` —
  documentar `native_here()` assinatura, semântica,
  paralelo vanilla.

L1 depois (conforme opção):

- (Opção i) `01_core/src/rules/eval/mod.rs` — adicionar
  `current_location: Option<Location>` em `EvalContext`;
  walk advance em locatable boundaries.
- (Opção ii) Novo `Content::Context { body }` variant
  em L1; impl resolve em layout time.
- (Opção iii) `EvalContext` extension para expor
  `last_iter_location_for(label)` ou similar; sem
  walk advance.

Stdlib:

- `01_core/src/rules/stdlib/foundations.rs` (ou
  caminho exacto identificado em P208A A2) — adicionar
  `pub fn native_here(...)` paralelo a `native_query`.
- Registar no Scope global da stdlib.

### C3 — Propagação a chamada-site

- Verificar registo no scope global (esperado em
  `01_core/src/stdlib/builtins.rs` ou similar).
- Confirmar que `here()` é invocável de eval `.typ`
  source.

### C4 — Tests

Tests dedicados (~4-6):

- `p208b_here_em_block_simples` — `let x = here(); ...`
  retorna `Location` válida ou erro contextual coerente.
- `p208b_here_em_contexto_locatable` — `here()` dentro
  de heading retorna location do heading.
- `p208b_here_apos_locate` — composição básica.
- 1-2 tests específicos da opção fixada em C1 (ex:
  iter cross-iter para opção iii).

### C5 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 1903+ verdes (1899 + 4+); 0 violations.

**Regra empírica P207B §5**: `here()` é func stdlib,
não trait method. **Não propaga a `CountingIntrospector`**.
Trait mantém 26 métodos.

Anotar ADR-0076 §P208B: `✅ MATERIALIZADO {data}` +
sumário opção fixada em C1.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-208B-relatorio.md`.

Estrutura conciso (~4-6 KB) com 7 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Sub-mecanismo fixado em C1 (com evidência empírica
  literal).
- §3 Alterações em código (lista compacta).
- §4 Decisões substantivas.
- §5 Métricas (tabela compacta).
- §6 Divergências (se `P208B.div-N`).
- §7 Próximo sub-passo (P208C).

---

## §4 Não-objectivos

- `locate(selector)` (P208C).
- `Selector::Before/After` (P209).
- `Selector::Label` (P209).
- Trait method extensions (zero — `here()` é stdlib).
- Propagação a `CountingIntrospector`.
- Page-aware captura (P207E Caminho 1 fixou deferred).
- `#context { ... }` block materialização extensa (se
  C1 fixar opção ii, materializar minimal; expansão
  fica para sub-passo dedicado).

---

## §5 Riscos a evitar

1. **Inflar para opção ii sem necessidade**: se
   cristalino tem precedente para (i) ou (iii), ir
   directo. Opção ii introduz tipo Content novo —
   custo M+ que pode não pagar.
2. **Aceitar opção iii sem caveat**: semântica iter-cross
   diverge vanilla. Útil para queries entre iterações
   mas pode confundir consumer que espera "location
   actual do eval". Documentar honestamente se C1
   fixar iii.
3. **Materializar `here()` sem caller real testável**:
   se C1.3 mostrar que P209 Before/After ainda não
   existem, tests P208B devem ser sintéticos (mock
   caller). Documentar limitação.
4. **Esquecer registo no scope global**: stdlib func
   sem registo não é invocável de `.typ` source. C3
   verifica.
5. **`Layouter::current_location` confusion** (P185C):
   Layouter já tem `current_location` em fase 3. P208B
   adiciona análogo em `EvalContext` (fase 2). Nomes
   próximos; documentar distinção em L0.

---

## §6 Sub-decisão deferida adicional

Per P208A C5: P208B pode introduzir sub-decisão sobre
**onde locator é populado** se opção i for fixada:
- Em eval walk de Content (recursivo profundo).
- Em ponto específico antes de cada eval que pode
  invocar stdlib func.
- Híbrido.

Esta sub-decisão fica para dentro do trabalho de C2 se
opção i fixada. Caso contrário, irrelevante.
