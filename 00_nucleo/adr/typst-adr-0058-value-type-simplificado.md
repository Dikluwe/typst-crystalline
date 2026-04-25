# ⚖️ ADR-0058: Tipo simplificado — `type()` devolve `Value::Str` em vez de `Value::Type(Type)`

**Status**: `EM VIGOR`
**Validado**: Passo 149 — arqueologia (`divergencias-value-passo-149.md` §3.1); 1113 tests verdes desde Passo 13.
**Data**: 2026-04-25
**Autor**: Humano + IA
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/divergencias-value-passo-149.md`](../diagnosticos/divergencias-value-passo-149.md)

---

## Contexto

Vanilla Typst define `Value::Type(Type)` como variant do enum
`Value`. `Type` é struct rico com:

- Nome do tipo (`int`, `str`, `float`, …).
- Scope de métodos associados (ex: `str.at`, `int.min`).
- Docstring / reflexão runtime.
- Construtores que permitem `type(x) == int` (igualdade
  estrutural entre `Type` instances).

Cristalino materializou `Value` no Passo 13 com subset
estratégico de 9 variants (hoje 18). Variantes `Type(Type)` e
`Args(Args)` ficaram comentadas como "futuras":

```rust
// 01_core/src/entities/value.rs:81-82
// Args(Args),               // argumentos de função
// Type(Type),               // tipo como valor (int, str, etc.)
```

Comentário do enum (linhas 14-16) invoca **ADR-0017**
("adiamento de eval() e estratégia typst-library"): tipos
migram progressivamente quando os dependentes entrarem em L1.
`Type` exigiria materializar `typst-library::ty` (scope,
métodos, docstring); não foi priorizado em 133 passos.

**Substituto funcional** (Passo 13, refinado em Passo 14):

- `Value::type_name() -> &'static str` — método sobre `Value`
  que devolve nome canónico ("int", "str", "bool", …).
- `native_type(args)` — stdlib function que devolve
  `Value::Str(args[0].type_name())`.

Resultado: `type(42)` em Typst cristalino devolve a string
`"int"`, não um valor-tipo. Comparação `if type(x) == "int" { … }`
funciona; comparação `if type(x) == int { … }` (vanilla-style)
**não funciona** porque `int` não é um valor em cristalino
(é apenas função construtora — `native_int`).

## Decisão

**`Value::Type(Type)` permanece ausente**. `native_type(v)`
continua a devolver `Value::Str(type_name)`. Sem plano de
migração para `Value::Type(Type)` no escopo actual.

### Consequências observáveis

1. **`type(x) == "int"`** funciona. **Preferido** como
   idiom de comparação em cristalino.
2. **`type(x) == int`** não funciona (erro de tipo em
   comparação: `Value::Str` vs `Value::Func`).
3. **Métodos sobre tipos** (`int.round()`, `str.at(i)`) são
   dispatched via `native_*` functions, não via método em
   `Type`.
4. **Reflexão** (listar métodos de um tipo) não disponível.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **`Value::Str(type_name)` (actual)** ✓ | Mínimo; funciona para comparação string; tests passam | Divergência vanilla; `type(x) == int` falha |
| Materializar `Value::Type(Type)` com scope + métodos | Paridade vanilla completa; reflexão runtime | Escopo grande; requer `typst-library::ty` migrada; bloqueia-se em imports circulares |
| `Value::Type(EcoString)` — variant simples com nome apenas | Middle-ground; `type(x) == Type::int` possível via construtor | Sem ganho real face ao Str; duplica representação |

**Escolha**: actual, pragmática.

## Consequências

### Positivas

- **Superfície mínima do enum `Value`**: 18 variants em vez
  de 19+. Match exaustivo menos pesado.
- **`Value` mantém-se em L1 puro**: sem dependência de
  scope/métodos/docstring (que exigiriam estruturas ricas).
- **Comportamento observável adequado** ao subset de features
  cristalino cobre: `type(x) == "int"` é suficiente para
  casos de uso actuais (condicionais simples em markup/eval).
- **Consistente com ADR-0017** (adiamento de migração) e com
  perfil observacional graded de ADR-0054.

### Negativas

- **Divergência vanilla** em idiom idiomático `type(x) == int`.
  Utilizadores a migrarem código do vanilla têm de
  re-escrever comparações usando strings.
- **Sem métodos-sobre-tipos** (`int.round`, `str.at`): se
  priorizados, exigem migração.
- **Sem reflexão runtime**: features como "listar métodos
  disponíveis" não disponíveis.

### Neutras

- `native_type` permanece como helper estável.
- Cada primitivo tem construtor separado (`native_int`,
  `native_str`, `native_float`) — estes não são variantes de
  `Value::Type` mas sim `Value::Func`.

## Plano de evolução (condicional)

Sem plano actual. **Se** alguma das seguintes for priorizada
no futuro:

- Paridade completa com `type(x) == int` (vanilla idiom).
- Métodos-sobre-tipos (`int.round()`, `str.at(i)`) nativos,
  não via `native_*`.
- Reflexão runtime (listar métodos; introspecção).

**Então** abrir passo dedicado (escopo M-L: materializar
`Type` struct; migrar `native_type`; refactorizar stdlib de
métodos). ADR-0058 não bloqueia essa migração — formaliza a
forma actual sem a fixar.

## Referências

- **ADR-0017** (adiamento de eval()) — invocada como razão
  canónica para adiar migração de tipos compostos.
- **ADR-0033** (paridade funcional) — reinterpretada por
  ADR-0054 (perfil observacional graded): string-based é
  aceitável dentro do perfil.
- **ADR-0054** (critério fecho DEBT-1) — perfil graded cobre
  a forma actual sem reabrir DEBT-1.
- **Passo 13** (materialização inicial do `Value` enum) —
  origem da decisão; `Type(Type)` comentado como variant
  futura.
- **Passo 14** (`type_name()` + `native_type`) — consolidação
  da forma actual.
- **ADR-0025** (`Int == Float`) — precedente de decisão sobre
  comparação relaxada em `Value`.
- **Inventário 148** (`typst-cobertura-vanilla-vs-cristalino.md`
  Tabela B) — registou `Value::Type` como "parcial
  (string-based)"; pós-149 reclassificado para `implementado⁺`
  com referência a esta ADR.
