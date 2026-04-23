# Passo 96.5 — Reestruturação de `stdlib.rs` em submódulos por área da stdlib

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` — ADR
  `EM VIGOR` com 7 regras + 4 ajustes.
- `00_nucleo/DEBT.md` — DEBT-46 com checkbox 96.5 pendente.
- `01_core/src/rules/stdlib.rs` — ficheiro actual, 1711 linhas.
- `01_core/src/rules/eval/mod.rs` — função `make_stdlib` (vê
  como é construído o scope global e o que consome de `stdlib.rs`).
- Passos 96.1 e 96.4 para referência do padrão.

Pré-condição: `cargo test` — 748 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 96.4 concluído.

---

## Natureza deste passo

Passo único de reestruturação. Aplica ADR-0037 a `stdlib.rs`:
divisão por área da biblioteca padrão Typst.

Em comparação com os Passos 96.1 e 96.4:
- **Divisão ainda mais mecânica**: cada função nativa é
  relativamente independente; agrupar por tema tipográfico é
  directo.
- **Pouca dependência cruzada**: funções de texto não chamam
  funções de layout, normalmente.
- **Sem struct central**: não há equivalente a `Parser` ou
  `EvalContext` a decompor — são só funções.

Expectativa: execução mais rápida que 96.4 (~50% do tempo).

**Não altera semântica observável** (ADR-0033). Preserva
todas as funções nativas registadas.

---

## Clusters propostos

Baseado na organização típica da stdlib Typst. **Verificar
estado real em Fase 0** antes de aplicar — o cristalino pode
ter subset específico das funções vanilla.

```
01_core/src/rules/stdlib.rs (antes: 1711 linhas)
    ↓ transforma-se em:
01_core/src/rules/stdlib/
    mod.rs          — pub fn make_stdlib (se mudar para cá vindo
                      do eval), ou apenas registos de submódulos
                      + re-exports públicos
    foundations.rs  — type, repr, str, int, float, bool, panic,
                      assert, print
    calc.rs         — abs, min, max, sin, cos, tan, floor, ceil,
                      round, sqrt, pow, ln, log
    text.rs         — upper, lower, starts_with, ends_with, split,
                      replace, trim, len (de texto)
    array.rs        — len (de array), first, last, push, pop,
                      insert, remove, contains, join, sorted,
                      filter, map
    dict.rs         — keys, values, at, insert, remove (de dict)
    layout.rs       — box, block, pad, align, columns, pagebreak,
                      v, h (se existirem)
    math.rs         — elementos de matemática (sum, product,
                      integral, fração, etc.)
    metadata.rs     — numbering, counter, figure, heading
                      (se pertencerem à stdlib e não ao eval)
```

### Ajustes possíveis durante execução

- Se um cluster ficar muito pequeno (< 150 linhas), absorver
  em outro (ex: `dict.rs` pode fundir com `array.rs` como
  `collections.rs` se ambos forem pequenos).
- Se um cluster ficar muito grande (> 800 linhas), subdividir.
- Se algum submódulo esperado não existir no cristalino (ex:
  `metadata` pode estar noutro lugar), não criar submódulo
  artificial.

---

## Fase 0 — Preparação

### 0.1 — Inventário das funções existentes

```bash
# Tamanho:
wc -l 01_core/src/rules/stdlib.rs

# Funções públicas:
grep -n "^pub fn\|^fn " 01_core/src/rules/stdlib.rs

# Estrutura do ficheiro (seções por comentário, structs, etc.):
grep -n "^///\|^//" 01_core/src/rules/stdlib.rs | head -40

# Testes:
grep -c "^\s*#\[test\]" 01_core/src/rules/stdlib.rs
```

Reportar:
- Linhas totais.
- Número de funções nativas.
- Se há agrupamento prévio por comentários/secções (indica
  clusters existentes que podem virar submódulos directamente).
- Se há structs helpers (ex: `NativeFunc`, `Accessor`).

### 0.2 — Verificar relação com `make_stdlib`

```bash
# Como make_stdlib consome funções da stdlib:
grep -B 2 -A 5 "make_stdlib" 01_core/src/rules/eval/mod.rs | head -30

# Imports actuais em eval/mod.rs:
grep -n "use.*stdlib" 01_core/src/rules/eval/mod.rs
```

Reportar:
- Como `make_stdlib` consome as funções (enumeração directa?
  registo via trait? builder pattern?).
- Se a reestruturação força alterações em `make_stdlib`.

### 0.3 — Criar directório

```bash
mkdir -p 01_core/src/rules/stdlib
git mv 01_core/src/rules/stdlib.rs 01_core/src/rules/stdlib/mod.rs
```

Verificar compilação:

```bash
cargo check --package typst-core 2>&1 | tail -10
```

Se `eval/mod.rs` importa `use crate::rules::stdlib;` continua
a funcionar — o directório resolve via `mod.rs`.

---

## Fase 1 — Extracções por cluster

### Ordem recomendada

Funções mais isoladas primeiro:

1. **foundations** — `type`, `repr`, conversões. Sem
   dependências cruzadas.
2. **calc** — operações matemáticas escalares. Não dependem
   de estado.
3. **text** — operações de string. Podem depender de
   `EcoString`/`String`, não de outras stdlib.
4. **array** / **dict** — colecções. Podem chamar-se
   mutuamente ocasionalmente (ex: dict.keys() retorna array).
5. **math** — se existir como stdlib e não como eval.
6. **layout** — pode depender de types de layout mas não de
   outras stdlib.
7. **metadata** / outros — último.

### Procedimento por cluster

Idêntico aos Passos 96.1 e 96.4:

1. Identificar funções do cluster (por nome, por secção
   comentada, ou por análise de semântica).
2. Criar `stdlib/<cluster>.rs` com cabeçalho:
   ```rust
   //! Funções nativas da área `<cluster>`. Extraído de
   //! `stdlib.rs` no Passo 96.5 conforme ADR-0037.

   use super::*;
   ```
3. Mover funções. Visibilidade: `pub(super)` ou `pub(crate)`
   conforme consumo.
4. Declarar `mod <cluster>;` em `stdlib/mod.rs`.
5. Actualizar `make_stdlib` (em `eval/mod.rs`) para consumir
   funções via `crate::rules::stdlib::<cluster>::func`.
6. Mover testes específicos do cluster.
7. Verificar: `cargo check` + `cargo test`. Rollback do
   cluster se falhar.

### Atenção especial: `make_stdlib`

Esta função é o ponto de registo de todas as funções nativas
no scope global. Duas estratégias possíveis:

**Estratégia A** — Cada submódulo expõe função registadora:

```rust
// stdlib/calc.rs:
pub(crate) fn register(scope: &mut Scope) {
    scope.define("abs", Func::new(abs));
    scope.define("min", Func::new(min));
    // ...
}

// eval/mod.rs:
fn make_stdlib() -> Scope {
    let mut scope = Scope::new();
    stdlib::foundations::register(&mut scope);
    stdlib::calc::register(&mut scope);
    stdlib::text::register(&mut scope);
    // ...
    scope
}
```

Vantagem: `make_stdlib` fica trivial (10 linhas). Cada
submódulo gere os seus registos.

**Estratégia B** — `make_stdlib` consome funções directamente:

```rust
// eval/mod.rs:
fn make_stdlib() -> Scope {
    let mut scope = Scope::new();
    scope.define("abs", Func::new(stdlib::calc::abs));
    scope.define("min", Func::new(stdlib::calc::min));
    // ...
}
```

Desvantagem: `make_stdlib` cresce linearmente com número de
funções. Cada nova função nativa força edição do `eval`.

**Recomendação**: **Estratégia A**. Alinha com ADR-0037 (cada
submódulo é responsável pelo seu domínio, incluindo registo).

Se a estrutura actual do `make_stdlib` for complexa (usa
tabela, macros, etc.), avaliar se a estratégia actual já
funciona bem após a divisão e manter.

---

## Fase 2 — Verificação final

### 2.1 — Tamanhos

```bash
wc -l 01_core/src/rules/stdlib/*.rs | sort -rn
```

Alvo: nenhum ficheiro > 800 linhas. `mod.rs` esperado < 200
linhas.

### 2.2 — Testes e linter

```bash
cargo test --workspace 2>&1 | tail -10
cargo run --package crystalline-lint 2>&1 | tail -5
```

Esperado: 748 L1 + 174 L3 + 6 ignorados (pode subir 3–8 por
smoke tests obrigatórios V2 se forem criados submódulos
novos suficientes). Zero violations.

### 2.3 — Testes funcionais de stdlib

```bash
cargo test --package typst-core stdlib 2>&1 | tail -15
cargo test --package typst-core calc 2>&1 | tail -10
cargo test --package typst-core text 2>&1 | tail -10
```

Todos devem passar.

---

## Fase 3 — Actualizar DEBT-46

Marcar o quinto checkbox:

```markdown
- [x] `stdlib.rs` reestruturado por módulo da stdlib (text,
      layout, math, calc, etc.). Passo 96.5. **Concluído** —
      N submódulos criados, todos abaixo de 800 linhas.
      `make_stdlib` actualizado (Estratégia A/B). Tamanho
      final de `stdlib/mod.rs`: X linhas.
```

DEBT-46 não fecha (4 checkboxes restantes).

---

## Critérios de conclusão

- [ ] Directório `01_core/src/rules/stdlib/` criado com
      `mod.rs` + submódulos.
- [ ] `stdlib.rs` já não existe como ficheiro top-level.
- [ ] 5–8 submódulos criados (ajuste registado se diferente
      do proposto).
- [ ] Nenhum submódulo > 800 linhas sem excepção Regra 6.
- [ ] `make_stdlib` em `eval/mod.rs` actualizado.
- [ ] Testes de stdlib passam sem alteração.
- [ ] `cargo test --workspace` preservado (748 ± smoke tests
      V2 dos submódulos novos).
- [ ] `crystalline-lint` → zero violations.
- [ ] Nenhum `unsafe` novo.
- [ ] DEBT-46 com quinto checkbox marcado.
- [ ] Nenhum ADR alterado.

---

## Ao terminar, reportar

Fase 0:
- Número de funções nativas no `stdlib.rs`.
- Se havia agrupamento prévio por comentários.
- Forma actual do `make_stdlib`.

Fase 1:
- Clusters identificados vs. clusters executados (ajustes).
- Estratégia aplicada ao `make_stdlib` (A ou B).
- Smoke tests criados por V2 do linter (número).

Fase 2:
- Tamanhos finais.
- Testes verdes, zero violations.

Fase 3:
- Confirmação DEBT-46 actualizado.

Observações para continuidade:
- Alguma fricção nova com ADR-0037?
- Alguma regra do linter interagiu de forma inesperada?

Go/No-Go para Passo 96.6:
- **Go incondicional** se reestruturação foi limpa. Passo 96.6
  = `layout/mod.rs` (2848 linhas), que é o mais complexo ainda
  pendente — `Layouter<M, S>` com muitos métodos.
- **Go com nota** se houve fricções.
- **No-Go** se algum teste de stdlib regressou inesperadamente.
