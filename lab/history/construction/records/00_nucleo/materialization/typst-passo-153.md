# Passo 153 — P2 cristalino-only baseline (`ValueDTO` + matriz semantic)

**Série**: 153 (passo **substantivo** em `lab/parity/`;
materialização de P2 com cristalino-only baseline em paralelo
com DEBT-54).
**Precondição**: Passo 152 encerrado; DEBT-54 com plano
refinado; baseline P3 P150 preservado; 1113 tests cristalino;
12 DEBTs abertos.

**Numeração**: 153 ocupa posição que era P152 antes da
reformulação 6 (P152 = refino administrativo). Coerente com
§9 actualizado pós-P152.

**Natureza**: passo **substantivo** em `lab/parity/`. Toca:
- `lab/parity/src/value_dto.rs` (novo).
- `lab/parity/tests/eval_parity.rs` (novo).
- `lab/parity/corpus/semantic/` (subdir nova).
- `lab/parity/reports/latest.md` (re-render com P2 adicionado).
- `lab/parity/reports/history/2026-04-25-passo-153.md` (novo).
- `00_nucleo/diagnosticos/typst-paridade-plano-medicao.md`
  §9 (mini-update).

**Cristalino L1/L2/L3/L4**: **intactos**. `lab/parity/`
continua fora do workspace cristalino.

**Modelo**: análogo a P150 (P3 baseline cristalino-only).
Diferença material: P2 mede `Value` em vez de `Frame`. Mesma
estrutura de DTO, harness, matriz, relatório.

**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional) — operacionalização da
  comparação de valores.
- **ADR-0058** (P149) — `Value::Type` simplificado;
  `ValueDTO::Type` exprime forma cristalino.
- **ADR-0059** (P149) — `Value::Args` não-variant; `ValueDTO`
  não tem variant `Args` (ou tem como string).
- **ADR-0054** (perfil observacional graded).

---

## Contexto

P150 entregou matriz P3 baseline cristalino-only. Colunas
materiais ficaram `N/A` por DEBT-53 + DEBT-54. P153 estende a
mesma estratégia para P2 (eval).

P2 mede `Value`/`Module` produzido por `eval()`. Inventário
148 indica:
- **Cobertura `Value` cristalino**: 18 variants (vs 30
  vanilla); cobertura arquitectural `Value` 22% impl + 13%
  impl⁺ (cf P149) = 35% formalmente coberto.
- **Cobertura user-facing eval-related**: ~62% (categoria
  "`#let`/`#set`/`#show`/import" 8/13).

**Estratégia P153**: replica padrão P150.

1. `ValueDTO` neutro com 1 variant por `Value` cristalino +
   `Other(String)` para variants vanilla sem mapeamento.
2. Conversão `from_cristalino` real.
3. `from_vanilla` stub (matriz com colunas `N/A` material em
   vanilla).
4. Test harness sem `assert!` global (medição, não
   verificação).
5. Corpus semantic novo: 5-10 ficheiros `.typ` com `#let
   __resultado__ = <expr>`.
6. Matriz P2 incorporada em `latest.md` ao lado de P3.

**Pergunta original**: "em que paridade estamos?" — ainda
parcialmente respondida. P153 estende cobertura observacional
cristalino-only mas não preenche colunas vanilla. Resposta
substantiva continua dependente de DEBT-54 → fecho DEBT-53.

---

## Objectivo

Ao fim do passo:

1. **`lab/parity/src/value_dto.rs`** materializado:
   - `ValueDTO` enum com variants 1:1 cristalino + variantes
     `Other(String)` para vanilla.
   - Conversão `from_cristalino(&typst_core::Value) -> Self`.
   - Stub `from_vanilla_stub() -> Self`.
   - Função `compare(&self, other: &Self) -> ValueComparison`
     com semantic deep equality.
   - Notação especial para `Float` (NaN), `Func` (por nome),
     `Content` (recurse para `ContentDTO` ou stub se ainda
     ausente).

2. **`lab/parity/tests/eval_parity.rs`** materializado:
   - Itera corpus `lab/parity/corpus/semantic/`.
   - Para cada ficheiro: `eval()` cristalino → extrai
     `__resultado__` do scope → converte para `ValueDTO`.
   - Vanilla side: stub (registar `N/A` na matriz).
   - Sem `assert!` global; alimenta matriz.

3. **Corpus semantic** novo:
   `lab/parity/corpus/semantic/` (subdir) com **5-10
   ficheiros**:
   - `bool-true.typ`: `#let __resultado__ = true`.
   - `int-aritmetica.typ`: `#let __resultado__ = 2 + 3`.
   - `float-divisao.typ`: `#let __resultado__ = 10 / 4`.
   - `string-concat.typ`: `#let __resultado__ = "a" + "b"`.
   - `array-literal.typ`: `#let __resultado__ = (1, 2, 3)`.
   - `dict-literal.typ`: `#let __resultado__ = (a: 1, b:
     2)`.
   - `condicional.typ`: `#let __resultado__ = if 2 > 1 {
     "sim" } else { "nao" }`.
   - `funcao-builtin.typ`: `#let __resultado__ = len((1, 2,
     3))`.
   - `tipo-inspeccao.typ`: `#let __resultado__ = type(42)` —
     **chave**: testa a divergência ADR-0058 (cristalino
     devolve `"int"` string; vanilla devolve `Type(int)`).
   - `args-rest.typ`: `#let f(..a) = a; #let __resultado__ =
     f(1, 2, 3)` — testa ADR-0059 (cristalino e vanilla
     diferem na representação de args/array).

   Cada ficheiro com `.typ.toml` adjacente:
   ```toml
   features = ["let", "expr"]
   modo_p2 = "value_equality"
   notes = "..."
   ```

4. **Matriz P2** integrada em `lab/parity/reports/latest.md`:
   - Tabela P2 ao lado da tabela P3 existente.
   - Coluna "Compila (cristalino)" = N/N esperado.
   - Coluna "Compila (vanilla)" = N/A (DEBT-54).
   - Coluna "value_equality" = N/A (DEBT-54).
   - Coluna "type_name_match" = preliminar; mede se cristalino
     `type(x).type_name()` bate em valor textual conhecido.
     **Especulativo** — pode falhar se `type()` cristalino
     for inconsistente com tabela esperada.

5. **Cópia histórica**:
   `lab/parity/reports/history/2026-04-25-passo-153.md`
   com matriz P2 + P3 actuais. Cópias 150 e 151 preservadas
   (imutáveis).

6. **§9 dos documentos de paridade actualizado**:
   - Item P153 muda de "P2 (`value_dto.rs`)" para "P2
     cristalino-only baseline (este passo, completo)".
   - Resto inalterado.

7. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-153-relatorio.md`.

8. **Sem novo DEBT** — DEBT-54 já cobre vanilla integration
   para todos os P-níveis.

Este passo **não**:

- Toca código em L1/L2/L3/L4 cristalino.
- Materializa DEBT-54 (passo dedicado futuro).
- Materializa P4 (P153 era P4 originalmente; agora P155+
  ou similar).
- Importa corpus oficial vanilla.
- Cria ADRs novas.
- Modifica DEBT-54, DEBT-53 ou outros DEBTs.
- Implementa `ContentDTO` (referenciado por
  `ValueDTO::Content`; pode ser stub textual ou diferido
  para passo dedicado).

---

## Decisões já tomadas

1. **Cristalino-only baseline** — espelha P150. Vanilla via
   DEBT-54.
2. **Sem `assert!` global no harness** — espelha P150
   (medição, não verificação).
3. **Corpus semantic novo** — 5-10 ficheiros com
   `__resultado__`.
4. **`Float` por bits** (preserva NaN) per `definicoes.md` §P2.
5. **`Func` por nome** per `definicoes.md` §P2.
6. **`Content` recurse** — provavelmente stub textual nesta
   iteração; recurse completo é trabalho de P155 ou similar.
7. **Matriz integra P2 ao lado de P3** em `latest.md` único.

## Decisões diferidas (resolvidas neste passo)

8. **Forma de `ContentDTO` para uso de `ValueDTO::Content`**:
   - Opção A: `String` (texto extraído via `Content::plain_text`
     do cristalino).
   - Opção B: enum reduzido (Empty/Text/Sequence/Other).
   - **Default A** (mais simples; serve como stub até P155
     ter `ContentDTO` completo se necessário).

9. **Variant ordering em `ValueDTO`**: alfabética vs
   semantic. Default: **alfabética** para fácil verificação
   visual.

10. **`OrderedFloat` para `Float`**: per `definicoes.md`,
    "wrapper que aceita comparação por bits". Implementar com
    `f64::to_bits()` e comparar `u64`. NaN compara igual a
    NaN com mesma representação.

11. **`Type` em `ValueDTO`**: já decidido por ADR-0058 — em
    cristalino é `Value::Str(type_name)`. Em vanilla seria
    `Value::Type(Type)`. `ValueDTO::Type(String)` com nome
    textual; conversão `from_cristalino` extrai do
    `Value::Str`; conversão `from_vanilla` (futura)
    extrairá do `Type::name()`. **Cuidado**: este caso
    confunde mapping; documentar bem no `value_dto.rs`.

12. **Lista canónica de variants do `ValueDTO`**: 1:1 com
    cristalino:
    - None
    - Auto
    - Bool(bool)
    - Int(i64)
    - Float(u64)  // bits, OrderedFloat-like
    - Length(LengthDTO)  // ou string?
    - Color(ColorDTO)  // ou string?
    - Datetime(DateTimeDTO)  // ou string?
    - Str(String)
    - Array(Vec<ValueDTO>)
    - Dict(Vec<(String, ValueDTO)>)  // preserva ordem
    - Func(String)  // por nome
    - Content(String)  // ContentDTO simplificado opção A
    - Type(String)  // ADR-0058
    - Module(String)  // por nome
    - Other(String)  // catch-all
    
    Decisões nesta lista resolvidas em 153.2 confirmando
    forma exacta dos `Value` em cristalino.

13. **Tratamento de `Args`**: ADR-0059 — não é variant.
    `ValueDTO::Args` **não existe**. Se vanilla envia
    `Value::Args(Args)`, mapeia para `ValueDTO::Other("args")`
    com nota.

14. **Erro de eval** em corpus: se ficheiro tem erro de
    parse/eval, regista-se na matriz como linha extra
    `failed_to_eval` (consistente com P150).

---

## Escopo

**Dentro**:

- `lab/parity/src/value_dto.rs` (novo).
- `lab/parity/tests/eval_parity.rs` (novo).
- Subdir `lab/parity/corpus/semantic/` com 5-10 ficheiros +
  `.typ.toml`.
- Re-render de `lab/parity/reports/latest.md` integrando P2.
- Cópia em `lab/parity/reports/history/2026-04-25-passo-153.md`.
- Pequeno update em `report.rs` para suportar matrizes
  multi-nível (P2 + P3).
- `§9 dos documentos de paridade`.
- Relatório do passo.

**Fora**:

- L1/L2/L3/L4 cristalino.
- ADRs (sem ADR nova).
- DEBT-53 / DEBT-54 (não tocados; aguardam materialização
  vanilla).
- README dos ADRs.
- `pdf_compare.rs` ou tests P4.
- Importação de corpus oficial vanilla.
- `ContentDTO` recursivo completo.
- Calibração de comparação de `Float` em casos edge (NaN,
  +inf, denormalize) — usar default por bits.

---

## Sub-passos

### 153.1 — Inventário pré-materialização

**A.1.1 — Confirmar API actual de `Value` cristalino**:

```bash
view 01_core/src/entities/value.rs
```

Listar variants exactos. Esperado: 18. Comparar com decisão
12.

**A.1.2 — Confirmar como extrair de Module**:

```bash
grep -nE "scope|get|module" 01_core/src/entities/module.rs 2>/dev/null
grep -rn "Module::scope\|Module::get" 01_core/src/
```

Confirmar API para extrair `__resultado__` do `Module`
produzido por `eval`. Esperado: `module.scope().get("__resultado__")
-> Option<&Value>` ou similar.

**A.1.3 — Confirmar pipeline eval em `lab/parity`**:

`compile_cristalino` em `tests/layout_parity.rs` (P150)
chama eval+layout. Para P2 só precisamos de eval. Forma
candidata:

```rust
let world = build_cristalino_world(source);
let module = typst_core::eval(world);
let value = module.scope().get("__resultado__").cloned();
```

Confirmar em 153.1.A.3.

### 153.2 — `value_dto.rs`

```rust
//! ValueDTO neutro para comparação P2.

#[derive(Debug, Clone, PartialEq)]
pub enum ValueDTO {
    None,
    Auto,
    Bool(bool),
    Int(i64),
    Float(u64),                          // bits per definicoes.md §P2
    Length(String),                      // serialize textual
    Color(String),                       // serialize textual
    Datetime(String),                    // serialize textual
    Str(String),
    Array(Vec<ValueDTO>),
    Dict(Vec<(String, ValueDTO)>),       // preserve ordem
    Func(String),                        // por nome (per ADR-0017)
    Content(String),                     // plain_text (Content::plain_text)
    Type(String),                        // ADR-0058: nome do tipo
    Module(String),                      // por nome
    Other(String),                       // catch-all
}

impl ValueDTO {
    pub fn from_cristalino(v: &typst_core::Value) -> Self;
    pub fn from_vanilla_stub() -> Self {
        // DEBT-54
        ValueDTO::Other("vanilla_stub".into())
    }
    pub fn compare(&self, other: &Self) -> ValueComparison;
}

pub enum ValueComparison {
    Equal,
    Differ {
        crist: String,    // Debug repr
        vanilla: String,
    },
}
```

**`from_cristalino` mapping** explicit:

- `Value::None`         → `ValueDTO::None`
- `Value::Auto`         → `ValueDTO::Auto`
- `Value::Bool(b)`      → `ValueDTO::Bool(b)`
- `Value::Int(i)`       → `ValueDTO::Int(i)`
- `Value::Float(f)`     → `ValueDTO::Float(f.to_bits())`
- `Value::Length(l)`    → `ValueDTO::Length(format!("{l:?}"))`
- ...
- `Value::Str(s)` (quando type_name flow): se source revela
  origem como `type()`, mapeia para `ValueDTO::Type(s.into())`;
  caso contrário, `ValueDTO::Str(s.into())`. **Heurística
  difícil**; considerar marcar todos os `Str` como `Str` e
  deixar comparação revelar a divergência ADR-0058 quando
  vanilla puder ser invocado.

### 153.3 — `tests/eval_parity.rs`

```rust
#[test]
fn corpus_semantic_p2() {
    let corpus = read_corpus("lab/parity/corpus/semantic/");
    let mut matrix_builder = MatrixBuilder::p2_default();

    for entry in corpus {
        let world = world_adapter::build_cristalino_world(&entry.source);
        match typst_core::eval(world) {
            Ok(module) => {
                if let Some(v) = module.scope().get("__resultado__") {
                    let dto = ValueDTO::from_cristalino(v);
                    matrix_builder.record_eval_ok(entry.category, dto);
                } else {
                    matrix_builder.record_no_resultado(entry.category, &entry.path);
                }
            }
            Err(e) => {
                matrix_builder.record_eval_error(entry.category, &entry.path, &e);
            }
        }
        // vanilla integration: DEBT-54
    }

    matrix_builder.update_latest_with_p2();
    matrix_builder.write_history_p2_p3();
}
```

Sem `assert!`. Falhas são informação para a matriz.

### 153.4 — Corpus semantic

10 ficheiros descritos no §Objectivo item 3. Cada um com:

```typst
// bool-true.typ
#let __resultado__ = true
```

```toml
# bool-true.typ.toml
features = ["let", "bool"]
modo_p2 = "value_equality"
notes = "Caso trivial; valida pipeline eval cristalino."
```

`tipo-inspeccao.typ` é diagnóstico — testa explicitamente
ADR-0058. Quando vanilla integrar via DEBT-54, este test
revelará a divergência:
- Cristalino: `ValueDTO::Str("integer")`.
- Vanilla: `ValueDTO::Type("integer")` (provável).

### 153.5 — Re-render de `latest.md`

Adicionar tabela P2 antes da tabela P3:

```markdown
## Matriz P2 (Eval)

| Categoria | Total | Compila (crist) | Eval ok (crist) | value_equality | Compila (vanilla) | value_equality (real) |
|-----------|------:|----------------:|----------------:|---------------:|------------------:|----------------------:|
| semantic  |    10 |          10/10  |          10/10  |            N/A |               N/A |                   N/A |

## Matriz P3 (Layout)

[matriz P150 existente, possivelmente actualizada se
tests P3 ainda corram em paralelo]
```

Notas no rodapé: "Vanilla integration depende de DEBT-54;
N/A serão preenchidos quando DEBT-54 fechar e DEBT-53 fechar
em sequência."

### 153.6 — Cópia histórica

`lab/parity/reports/history/2026-04-25-passo-153.md` =
cópia integral de `latest.md` pós-153.

### 153.7 — Actualização §9

```diff
  ## 9 — Próximas acções concretas
  ...
- 5. **Passo 153+** — Implementar P2 (`value_dto.rs`) ...
+ 5. **Passo 153** — P2 cristalino-only baseline (este
+    passo, completo). `ValueDTO` + corpus semantic + matriz
+    integrada em `latest.md`. Vanilla side N/A pending
+    DEBT-54.
  6. **Passo 154+** — Implementar P4 (`pdf_compare.rs`) ...
  7. **Passo dedicado para DEBT-54** — setup vanilla
     workspace ...
```

### 153.8 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-153-relatorio.md`.

Secções:
1. Sumário executivo.
2. Inventário pré-materialização (153.1).
3. `value_dto.rs` — assinatura final + mapping.
4. `tests/eval_parity.rs` — estratégia + resultados.
5. Corpus semantic — lista final + notas sobre ficheiros
   diagnósticos (ADR-0058 / ADR-0059).
6. Matriz P2 (cópia integral de `latest.md` pós-153).
7. Próximo passo: 154 (P4 cristalino-only baseline) ou
   passo dedicado DEBT-54 ou outro.
8. Verificação final.

---

## Verificação

1. ✅ `value_dto.rs` materializado com 16 variants.
2. ✅ `from_cristalino` cobre todos os 18 variants
   cristalino sem panic.
3. ✅ `from_vanilla_stub` placeholder.
4. ✅ `compare` implementado com semantic equality.
5. ✅ `tests/eval_parity.rs` corre o corpus semantic.
6. ✅ Subdir `corpus/semantic/` com 10 ficheiros + `.typ.toml`.
7. ✅ `latest.md` tem matriz P2 ao lado de P3.
8. ✅ Cópia em `history/2026-04-25-passo-153.md`.
9. ✅ Cópias 150 e 151 preservadas.
10. ✅ §9 dos documentos de paridade actualizado.
11. ✅ Nenhum ficheiro em L1/L2/L3/L4 cristalino tocado.
12. ✅ Nenhuma ADR criada / revogada / revisada.
13. ✅ Nenhum DEBT criado / fechado / actualizado.
14. ✅ `crystalline-lint .` zero violations.
15. ✅ `cargo test --workspace --lib`: 1113 inalterado.
16. ✅ `cd lab/parity && cargo test --test eval_parity` corre.
17. ✅ `cd lab/parity && cargo test --test layout_parity`
    continua a correr (P150 baseline preservada).
18. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Matriz P2 existe em `latest.md` ao lado de P3.
2. Corpus semantic + 10 ficheiros + metadata.
3. `ValueDTO` cobre os variants cristalino.
4. Stub para `from_vanilla` (vanilla via DEBT-54).
5. Documentos de paridade actualizados.
6. Próximo passo claro.
7. Sem código tocado em L1/L2/L3/L4 cristalino.
8. Relatório escrito.

---

## O que pode sair errado

- **`Module::scope().get("__resultado__")` não funciona como
  esperado**: API pode diferir. Investigar em 153.1; ajustar
  pipeline. Se exigir accessor novo em L1, **pausar** —
  passo de adicionar accessor é separado.

- **Corpus semantic tem feature ausente**: ex,
  `args-rest.typ` exige `..a` rest spread que cristalino pode
  não suportar. Verificar antes; se ausente, marcar
  `expected_to_fail` em metadata e excluir contagem.

- **`ValueDTO::Type` mapping ambíguo**: cristalino devolve
  `Value::Str` para resultado de `type()`, mas também para
  strings normais. Sem contexto, `from_cristalino` não
  consegue distinguir. Decisão: mapear todos como `Str`;
  divergência ADR-0058 só fica visível quando vanilla
  integrar (DEBT-54).

- **`Value::Func` em cristalino**: como extrair "nome" da
  closure? Vanilla pode ter named funcs com `name()` getter;
  cristalino pode ter assinatura diferente. Investigar em
  153.1; se sem accessor de nome, mapear para
  `Func("anonymous")`.

- **`Float` NaN**: `f64::to_bits()` para NaN tem múltiplas
  representações (NaN payload). Comparação por bits **não**
  iguala NaN diferentes. Aceite — `definicoes.md` §P2 diz
  "bits idênticos (NaN inclusive — verificar comportamento
  Typst)". Documentar comportamento detectado no relatório.

- **Matriz P2 + P3 num só `latest.md` cresce**: aceite. Se
  ficar > 200 linhas, considerar separar em `latest-p2.md`
  e `latest-p3.md` em passo dedicado futuro.

- **Diff de `report.rs` cresce**: novo método
  `update_latest_with_p2`, novo `MatrixBuilder::p2_default`,
  etc. Se refactor cresce muito, pausar — possível DEBT
  separado para "matriz multi-nível em report.rs".

- **`Value::Args` aparece em corpus**: ADR-0059 declara que
  `Args` é tipo separado, não-variant. Se test produz
  `Value::Args` (improvável dado que eval encapsula), mapeia
  `Other("args")`. Documentar.

- **Eval cristalino panica em ficheiro do corpus**: bug
  cristalino. Captura via `std::panic::catch_unwind` no
  harness; regista como `eval_panic` na matriz; **pausar
  o passo** se panic é em ficheiro trivial — pode indicar
  bug fundamental em eval.

- **Compilação de ficheiros eval-only via World adapter**:
  setup pode exigir World minimal que não inclui fonts
  (eval não precisa de fonts). Reusar setup do P150 ou
  simplificar.

---

## Notas operacionais

- **Reformulação 7 da série paridade**: 148 → 149 → 150
  → 151 → 152 → **153** (P2 cristalino-only). Padrão
  estabilizado. Pelo menos 1 reformulação restante prevista
  (P154 = P4 ou DEBT-54 → fecho DEBT-53 → matriz vanilla
  real).

- **Modelo: substantivo cristalino-only análogo a P150**.
  Mesma estratégia, novo nível de paridade. Custo conhecido
  (P150 deu o template).

- **Sem ADR criada**. ADR-0058 e ADR-0059 (P149) já cobrem
  decisões arquitecturais relevantes. ADR sobre `ValueDTO`
  como infra de medição é candidato condicional pós-DEBT-54
  fecho.

- **Diferença material face a P150**: P150 produzia
  baseline pioneiro (estabelecia padrão); P153 reaplica.
  Tempo esperado menor que P150.

- **Pós-153**: matriz tem **2 níveis** (P2 + P3) com
  cobertura observacional cristalino-only. Falta P4. Após
  P154 (P4 cristalino-only) ou após DEBT-54 fecho, matriz
  evolui.

- **Diagnóstico inerente em `tipo-inspeccao.typ` e
  `args-rest.typ`**: ADR-0058 e ADR-0059 ficam testáveis
  empiricamente quando vanilla integrar. Estes ficheiros
  são canários — primeiros sinais de divergência observacional
  fundamentada por ADR. Vale a pena documentar
  explicitamente no `.typ.toml`:
  ```toml
  diagnostic_for = "ADR-0058"  # ou ADR-0059
  ```

- **`lab/parity/Cargo.toml` provavelmente intacto**: P2
  cristalino-only só usa `typst-core` (já presente).
  Confirmar em 153.1.

- **Sem nova fonte de paridade observacional real**: P153
  estende cobertura **declarada** em P2 mas não responde à
  pergunta original mais do que P150 já respondia. Resposta
  substantiva continua dependente de DEBT-54.

- **Quarentena vanilla**: assumida opção 3 (princípio
  operacional, P9 precedente). Decisão pode ser formalizada
  em ADR futura se priorização justificar.
