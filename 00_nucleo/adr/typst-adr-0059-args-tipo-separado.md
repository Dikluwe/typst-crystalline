# ⚖️ ADR-0059: `Args` como tipo separado, não-variant de `Value`

**Status**: `EM VIGOR`
**Validado**: Passo 149 — arqueologia (`divergencias-value-passo-149.md` §3.2); 1113 tests verdes desde Passo 16.
**Data**: 2026-04-25
**Autor**: Humano + IA
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/divergencias-value-passo-149.md`](../diagnosticos/divergencias-value-passo-149.md)

---

## Contexto

Vanilla Typst define `Value::Args(Args)` como variant do enum
`Value`:

```rust
// lab/typst-original/.../value.rs
pub enum Value {
    // ...
    Args(Args),
    // ...
}
```

Permite armazenar `Args` em variáveis, passá-los como valores,
inspeccioná-los como qualquer `Value`.

Cristalino, desde o Passo 13, deixou a variant **comentada**:

```rust
// 01_core/src/entities/value.rs:81
// Args(Args),               // argumentos de função
```

E materializou `Args` como struct separado no Passo 16 para
suportar invocação de funções nativas:

```rust
// 01_core/src/entities/args.rs
pub struct Args {
    pub items: Vec<Value>,
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
}
```

`Args` é **input vehicle** para funções nativas — é construído
pelo eval e passado por `&Args` às `native_*` functions. Nunca
é embrulhado em `Value::Args(...)`.

## Decisão

**`Args` permanece struct separada em `01_core/src/entities/args.rs`,
não-variant de `Value`**. Funções nativas recebem `&Args` como
parâmetro explícito na assinatura:

```rust
pub fn native_type(
    _ctx: &mut EvalContext,
    args: &Args,                     // ← parâmetro explícito, não via Value
    _world: &dyn World,
    _current_file: FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value>;
```

`eval()` constrói `Args` a partir dos argumentos parseados e
invoca `native_fn(ctx, &args, world, …)`. Argumentos não são
*valores* de Typst — são veículo.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **`Args` struct separada, `&Args` explícito** ✓ | Alinhado com ADR-0036 (atomização); menor superfície de `Value`; separação clara input-vehicle vs valor armazenável | Divergência vanilla |
| `Value::Args(Args)` — variant (vanilla-style) | Paridade vanilla; `Args` pode ser armazenado em variáveis | Bloat do enum; semântica pouco usada; `match` exaustivo mais pesado em 20+ consumidores de `Value` |
| `Args` como `Value::Dict` especial | Reusa tipo existente | Perde distinção positional vs named; semântica obscura |

**Escolha**: actual. Expressa explicitamente a fronteira entre
"valor armazenável" (`Value`) e "veículo de invocação" (`Args`).

## Consequências

### Positivas

- **Alinhamento com ADR-0036 (atomização progressiva)**:
  funções nativas declaram `&Args` como dependência explícita
  na assinatura; sem empacotar/desempacotar via `Value`.
- **Superfície mínima do enum `Value`**: 18 variantes em vez
  de 19+. `match` exaustivo em ~20 consumidores de `Value`
  permanece manejável.
- **Fronteira semântica clara**: o que utilizador pode
  armazenar/passar em Typst (`Value`) é separado do veículo
  de invocação (`Args`). `#let a = type(1)` atribui
  `Value::Str` à `a`; `#let x = args` não é expressável em
  Typst cristalino — coerente com restrição à exposição do
  veículo.
- **Sem overhead** de conversão quando eval invoca native: o
  `&Args` fluí directo.

### Negativas

- **Divergência vanilla**: código vanilla que manipula
  `Value::Args` (raro em user code) não migra directamente.
- **Sem exposição do `Args` ao utilizador**: se feature futura
  exigir "inspeccionar argumentos da função actual", requer
  materialização nova.
- **Duplicação parcial**: `Value::Array` + `Value::Dict`
  podem representar positional+named separadamente; `Args`
  unifica-os. Decisão aceite dado o uso único em stdlib.

### Neutras

- `Args` é `pub struct` em `entities/args.rs` — acessível aos
  callers de eval se necessário (ex: debug).
- Invocação de funções no eval chama directamente
  `func.apply(ctx, &args, world, …)` sem passo intermédio por
  `Value`.

## Plano de evolução (condicional)

Sem plano actual. **Se** alguma das seguintes for priorizada
no futuro:

- User code manipular `args` dinamicamente (ex: varargs com
  passagem transparente).
- Feature de reflexão que exija inspeccionar `Args` como
  valor.
- Migração de `native_*` para closures típicas que exijam
  `Args` como primeira-classe.

**Então** abrir passo dedicado (escopo S-M: adicionar
`Value::Args(Args)` variant; actualizar ~20 consumidores de
`Value` com novo arm; manter `Args` struct + parâmetro
`&Args` por compat). ADR-0059 não bloqueia essa migração —
formaliza a forma actual sem a fixar.

## Referências

- **ADR-0036** (atomização progressiva) — princípio
  arquitectural invocado: dependências explícitas na
  assinatura de funções. `&Args` na signatura de `native_*`
  aplica directamente.
- **ADR-0017** (adiamento de eval) — contexto geral da
  estratégia "subset inicial; variantes adicionadas quando
  os dependentes migrarem para L1".
- **ADR-0033** (paridade funcional) — divergência
  estrutural (struct vs variant) permitida; divergência
  observacional (user code não-migrável) aceite dentro do
  perfil observacional graded de ADR-0054.
- **ADR-0054** (critério fecho DEBT-1) — perfil graded cobre.
- **Passo 13** (materialização inicial do `Value` enum) —
  origem da decisão; `Args(Args)` comentado como variant
  futura.
- **Passo 16** (funções nativas) — consolidação da forma
  actual; `Args` struct materializado em `entities/args.rs`.
- **Inventário 148** — registou `Value::Args` como "parcial
  (tipo separado)"; pós-149 reclassificado para
  `implementado⁺` com referência a esta ADR.
