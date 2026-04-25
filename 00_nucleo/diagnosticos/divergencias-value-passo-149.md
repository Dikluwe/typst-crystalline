# Divergências de `Value`: arqueologia — Passo 149

**Data**: 2026-04-25
**Âmbito**: formalização arqueológica das 2 divergências entre
`Value` cristalino e `Value` vanilla sinalizadas no inventário
148 §7.
**Output**: `typst-adr-0058-value-type-simplificado.md` +
`typst-adr-0059-args-tipo-separado.md`.

---

## 1. Inventário factual (149.1)

### 1.1 — Cristalino: `Value::Type`

`01_core/src/entities/value.rs:82`:

```rust
// Type(Type),               // tipo como valor (int, str, etc.)
```

**Variant comentada**, não está no enum. Desde o Passo 13
(materialização inicial).

**Uso actual**: a função `type(v)` devolve `Value::Str` via
`v.type_name() -> &'static str`:

```rust
// 01_core/src/rules/stdlib/foundations.rs:23
pub fn native_type(...) -> SourceResult<Value> {
    match args.items.as_slice() {
        [v] => Ok(Value::Str(v.type_name().into())),
        _   => err(...),
    }
}
```

`type(42)` retorna `Value::Str("int")`. Comparação
`type(x) == "int"` funciona; comparação `type(x) == int`
(vanilla-style com `Type` como valor) **não funciona**.

### 1.2 — Cristalino: `Args`

`01_core/src/entities/args.rs`:

```rust
pub struct Args {
    pub items: Vec<Value>,
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
}
```

**Struct separada**, não é variant de `Value`. Passada como
parâmetro `&Args` às funções nativas (stdlib). Nunca é
embrulhada em `Value::Args(...)` — tal variant não existe
(comentada em `value.rs:81`).

### 1.3 — Vanilla: `Value::Type`

`lab/typst-original/crates/typst-library/src/foundations/value.rs:~L28`:

```rust
pub enum Value {
    // ...
    Type(Type),
    // ...
}
```

`Type` é struct dedicado em `foundations/ty.rs` com:
- Nome do tipo.
- Métodos associados (scope com funções).
- Docstring/reflexão.
- Constructors para tipos primitivos (`int`, `str`, `float`, …).

Permite `type(x) == int`, `int.min`, etc.

### 1.4 — Vanilla: `Value::Args`

`lab/typst-original/.../value.rs:~L27`:

```rust
pub enum Value {
    // ...
    Args(Args),
    // ...
}
```

`Args` pode ser armazenado em variáveis, passado, inspeccionado
como qualquer `Value`.

### 1.5 — Tabela comparativa

| Aspecto | Vanilla | Cristalino |
|---------|---------|------------|
| `Value::Type` carrega | `Type` (struct rico com scope, métodos, docstring) | (não existe — comentada) |
| `type(x)` devolve | `Value::Type(...)` | `Value::Str(type_name)` |
| Comparação `type(x) == int` | funciona (Type igualdade) | **não funciona**; usa-se `type(x) == "int"` |
| `Value::Args` variant | existe | (não existe — comentada) |
| `Args` como tipo | `foundations/args.rs` → variant | `entities/args.rs` → struct separada; passada como `&Args` às nativas |
| Embrulhar Args em Value? | sim (`Value::Args(...)`) | **não**; nunca |

---

## 2. Arqueologia: quando + porquê (149.2)

### 2.1 — `Value::Type`

**Passo materializador do enum**: **Passo 13** (`typst-passo-13.md`).

Do enunciado do Passo 13, linhas 169–175, o enum já tinha
`Type(Type)` e `Args(Args)` **comentados**:

```rust
// Args(Args),               // argumentos de função
// Type(Type),               // tipo como valor (int, str, etc.)
// Module(Module),           // módulo importado — já em L1
// Plugin(Plugin),           // plugin WASM
// Dyn(Dynamic),             // valor dinâmico opaco
```

Comentário no topo do enum (`value.rs:11-16`):

> Subset de Passo 15: 9 variantes (5 primitivos + Array, Dict,
> Module, Datetime). As restantes (~21) são adicionadas quando
> os tipos dependentes migrarem para L1. Não adicionar
> variantes sem ADR e tipo migrado. Ver ADR-0017.

**Razão registada**: ADR-0017 (Passo 6) — "adiamento de
eval() e estratégia typst-library": tipos vão sendo
materializados progressivamente quando os dependentes entrarem
em L1. `Type` exigiria materializar `typst-library::ty` (com
scope, métodos, docstring, etc.) — trabalho não priorizado.

**Substituto funcional**: `v.type_name() -> &'static str` + 
`native_type` devolve `Value::Str(type_name)`. Implementado no
Passo 13/14 para permitir `#if type(x) == "int"` básico.

**Evidência de subset aceitável**: 1113 tests passam;
nenhuma feature dependente de `Type.method()` está priorizada
(int.round, str.at, etc. não são testadas no corpus actual).
`type(x) == "int"` funciona.

### 2.2 — `Value::Args`

**Passo materializador do struct**: **Passo 16**
(`typst-passo-16.md`) — introdução de funções nativas com
invocação via argumentos. `Args` foi criado como veículo de
input para `native_*` functions.

**Razão inferida** (sem passo/ADR explícita que justifique a
exclusão de `Value::Args`):

1. **Args como veículo, não valor**: nas funções nativas,
   `Args` é argumento posicional/named **para** uma função;
   o valor retornado é `SourceResult<Value>`. Raramente faz
   sentido armazenar `Args` em variável — é input efémero.
2. **ADR-0036 (atomização progressiva)** espírito: funções
   declaram explicitamente todas as dependências; `&Args`
   na assinatura é mais explícito do que `Value::Args(...)`
   empacotado.
3. **Evita bloat do Value enum**: 18 variantes é já
   significativo; adicionar uma variante pouco usada piora
   o match exaustivo em cada consumidor de Value.
4. **Não há código que pede `Value::Args`**: o eval passa
   `Args` directamente para a função; nunca empacota.

Não há registo explícito desta decisão em nenhum relatório
anterior — é **decisão de facto**, consistente por construção
ao longo de ~120 passos.

### 2.3 — Resumo arqueológico

| Divergência | Passo | Razão registada | Razão inferida |
|-------------|-------|-----------------|----------------|
| `Value::Type` | 13 | ADR-0017 (tipos migrados progressivamente; `Type` ainda não) | `Type` exige scope + métodos + docstring; fora do MVP |
| `Value::Args` | 16 (implícito) | nenhuma explícita | `Args` é input vehicle; ADR-0036 espírito; bloat do enum; uso único em stdlib |

---

## 3. Decisão de classificação (149.3)

Critério aplicado:

| Encontrámos | Classificação |
|-------------|---------------|
| Razão explícita "decisão consciente, forma final" | **ADR** |
| Sem razão registada + comportamento estável + tests OK | **ADR** (formaliza decisão de facto) |

### 3.1 — `Value::Type` → **ADR-0058**

- Razão **explícita** (ADR-0017 +
  comentário em `value.rs:14-16`): "tipos migrados quando
  dependentes estiverem em L1". `Type` é tipo composto
  (scope + métodos + docstring); sem priorização de migrar.
- Comportamento estável (`native_type` devolve `Value::Str`);
  1113 tests passam.
- **Formalizar como ADR-0058** "Tipo simplificado: `type()`
  devolve `Value::Str` em vez de `Value::Type`".
- Status: `EM VIGOR` (regra actual; revisitar se features
  método-sobre-tipo forem priorizadas).
- Não cria DEBT: não há plano actual de migrar. Se surgir
  requisito futuro (`int.round()`, `str.at()`), abrir passo
  dedicado nessa altura.

### 3.2 — `Value::Args` → **ADR-0059**

- Razão **inferida** (não registada explicitamente); decisão
  de facto ao longo de ~120 passos.
- Comportamento estável; 1113 tests passam.
- **Formalizar como ADR-0059** "`Args` como tipo separado,
  não-variant de `Value`". Invoca ADR-0036 (atomização) como
  princípio aplicado.
- Status: `EM VIGOR`.
- Não cria DEBT: não há plano de migrar para variant; a
  forma actual alinha com atomização.

### 3.3 — Outputs finais

- **2 ADRs criadas** (`ADR-0058`, `ADR-0059`).
- **0 DEBTs abertos**.
- **Inventário 148** actualizado: entradas de `Value::Type`
  e `Value::Args` na Tabela B passam de `parcial` para
  `implementado⁺` (divergência documentada por ADR).
  Referência cruzada a `ADR-0058` / `ADR-0059`.

---

## 4. Notas operacionais

- **Arqueologia é tentativa, não garantia**. Para `Value::Args`
  não há registo explícito; a razão inferida é consistente
  com padrões de ADR-0036 e com o uso real em stdlib. Se
  evidência contrária surgir no futuro, revisitar.

- **Critério conservador de classificação**: sem registo de
  razão + comportamento estável + tests OK → ADR (formaliza
  decisão de facto). Evita acumular DEBTs sem plano.

- **Divergências `Content::Styled` e `Value::Align`** do
  inventário 148 §7 ficam fora deste passo: ambas já têm
  ADR canónica (0026/0026-R1/0038 para Styled; DEBT-36
  encerrada para Align em Passo 84.5). Nenhuma formalização
  nova necessária.

- **Precedente**: se futuras divergências não-formalizadas
  forem detectadas (inventário 148 §7 lista ~10 candidatos
  residuais), padrão é "arqueologia + ADR" em passo análogo.
