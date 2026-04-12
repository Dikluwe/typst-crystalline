# Passo 58 — Contadores Genéricos e Intervenção Manual (`step` / `update`)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/counter_state.rs` — `CounterState` com `heading`
  hierárquico e `heading_numbering: bool` do Passo 57.
- `01_core/src/rules/eval.rs` — Braço `Expr::MethodCall` actual (se já
  existe) ou o wildcard onde `counter(heading).step()` cai actualmente.
- `01_core/src/entities/content.rs` — Variantes actuais incluindo
  `CounterDisplay` e `SetHeadingNumbering` do Passo 57.

Pré-condição: `cargo test` — 599 L1 + 113 L3 + 50 parity, zero violations.
O estado de heading funciona numa única passagem.

---

## Contexto

O Typst não limita contadores a secções. O utilizador pode numerar equações,
figuras, tabelas, ou criar contadores arbitrários
(`#counter("meu_contador").update(5)`). Além disso, pode manipular o estado
de qualquer contador explicitamente:

- `.step()` — avança o contador em 1.
- `.update(n)` — força o contador para o valor `n`.

Este passo generaliza o `CounterState` do Passo 57 (que só tinha o vector
de headings) para suportar chaves arbitrárias, e ensina o `eval` a interceptar
chamadas de método sobre `counter(...)`.

### O que o `MethodCall` parece na AST

Em Typst, `counter(heading).step()` é representado como:

```
MethodCall {
    receiver: FuncCall { callee: Ident("counter"), args: [Ident("heading")] },
    method:   Ident("step"),
    args:     [],
}
```

O diagnóstico confirmará a anatomia exacta — os nomes dos campos podem
diferir na AST cristalina.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Anatomia de MethodCall na AST cristalina
grep -n "MethodCall\|fn receiver\|fn method\b" \
  01_core/src/entities/ast/expr.rs | head -15

# 2. Verificar se Expr::MethodCall já tem braço no eval
grep -n "MethodCall" 01_core/src/rules/eval.rs | head -10

# 3. Confirmar que Equation existe no Content (para decidir se merece
#    contador flat neste passo ou apenas no Passo 59)
grep -n "Equation\|Figure" 01_core/src/entities/content.rs | head -10

# 4. Ver a estrutura actual de CounterState para planear a refactorização
grep -A 20 "pub struct CounterState" \
  01_core/src/entities/counter_state.rs | head -25
```

Reportar o output completo antes de continuar. A resposta à questão 1 é
a mais crítica: determina como extrair o nome do contador e o nome do método
a partir do nó AST.

---

## Tarefa 1 — Generalização do `CounterState` (L1)

Refactorizar `01_core/src/entities/counter_state.rs` para suportar chaves
arbitrárias. O campo `heading` hierárquico migra para dentro do
`HashMap<String, Vec<usize>>`. A flag `heading_numbering` migra para
`numbering_active: HashMap<String, bool>`.

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct CounterState {
    /// Contadores hierárquicos (ex: heading).
    /// Chave "heading" → `[1, 2]` representa a secção 1.2.
    hierarchical: HashMap<String, Vec<usize>>,
    /// Contadores planos (ex: equation, figure, ou chaves arbitrárias).
    flat: HashMap<String, usize>,
    /// Flags de numeração activa por chave.
    pub numbering_active: HashMap<String, bool>,
}

impl CounterState {
    pub fn new() -> Self { Self::default() }

    /// Verifica se a numeração está activa para uma chave.
    pub fn is_numbering_active(&self, key: &str) -> bool {
        self.numbering_active.get(key).copied().unwrap_or(false)
    }

    /// Avança o contador hierárquico para o nível indicado.
    /// Comportamento idêntico ao `step_heading` do Passo 57,
    /// mas agora opera sobre `self.hierarchical.entry(key)`.
    pub fn step_hierarchical(&mut self, key: &str, level: usize) {
        let level = level.max(1);
        let counter = self.hierarchical.entry(key.to_string()).or_default();
        counter.truncate(level);
        if counter.len() < level {
            counter.resize(level - 1, 0);
            counter.push(1);
        } else {
            if let Some(last) = counter.last_mut() {
                *last += 1;
            }
        }
    }

    /// Formata o contador hierárquico. Retorna `None` se vazio.
    pub fn format_hierarchical(&self, key: &str) -> Option<String> {
        let counter = self.hierarchical.get(key)?;
        if counter.is_empty() {
            None
        } else {
            Some(counter.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("."))
        }
    }

    /// Avança um contador plano em 1.
    pub fn step_flat(&mut self, key: &str) {
        *self.flat.entry(key.to_string()).or_insert(0) += 1;
    }

    /// Força um contador plano para um valor específico.
    pub fn update_flat(&mut self, key: &str, value: usize) {
        self.flat.insert(key.to_string(), value);
    }

    /// Lê o valor actual de um contador plano.
    pub fn get_flat(&self, key: &str) -> usize {
        self.flat.get(key).copied().unwrap_or(0)
    }
}
```

### Compatibilidade com o Passo 57

Os métodos `step_heading` e `format_heading` do Passo 57 eram chamados
directamente no Layouter. Substituí-los agora por `step_hierarchical("heading", level)`
e `format_hierarchical("heading")`. A flag `heading_numbering: bool` passa a
ser `is_numbering_active("heading")`.

Actualizar todos os call sites no Layouter (`layout.rs`) antes de continuar.

---

## Tarefa 2 — `CounterAction` e `Content::CounterUpdate`

### 2a — Enum `CounterAction`

Criar em `01_core/src/entities/counter_state.rs` (junto com `CounterState`,
ou num ficheiro separado se preferir):

```rust
/// Instrução de modificação de um contador.
#[derive(Debug, Clone, PartialEq)]
pub enum CounterAction {
    /// Avança o contador em 1 (flat) ou avança o nível (hierárquico).
    Step,
    /// Força o contador para o valor indicado.
    Update(usize),
}
```

### 2b — Variante `Content::CounterUpdate`

Em `01_core/src/entities/content.rs`:

```rust
CounterUpdate {
    key: String,
    action: CounterAction,
},
```

Actualizar `plain_text()` (retornar `""`) e `is_empty()` (retornar `false`).

### 2c — Braço no Layouter

Em `layout.rs`, consumir a instrução sem desenhar nada:

```rust
Content::CounterUpdate { key, action } => {
    match action {
        CounterAction::Step => {
            // Tentar como hierárquico primeiro se a chave for "heading",
            // caso contrário usar contador plano.
            if key == "heading" {
                // step_hierarchical sem nível explícito avança o nível 1
                self.counter.step_hierarchical("heading", 1);
            } else {
                self.counter.step_flat(key);
            }
        },
        CounterAction::Update(val) => {
            self.counter.update_flat(key, *val);
        },
    }
},
```

---

## Tarefa 3 — Intercepção de `MethodCall` no `eval`

No braço `Expr::MethodCall` de `eval.rs` (criar o braço se ainda não existir):

```rust
Expr::MethodCall(call) => {
    // Extrair o nome do método — confirmar API com o diagnóstico
    let method_name = call.method().as_str(); // ou .get(), dependendo da API

    // Verificar se o receiver é uma chamada a counter(...)
    // A API exacta depende do diagnóstico (ex: call.receiver() pode ser
    // Expr::FuncCall ou SyntaxNode que requer from_untyped)
    if let Some(counter_key) = extract_counter_key(call.receiver()) {
        return eval_counter_method(&counter_key, method_name, call.args(), ctx);
    }

    // Fallback para MethodCall não relacionado com contadores
    Ok(Value::None)
},
```

### Função auxiliar `extract_counter_key`

```rust
/// Extrai o nome do contador de uma expressão `counter(key)`.
/// Retorna None se a expressão não for uma chamada a `counter`.
fn extract_counter_key(receiver: Expr<'_>) -> Option<String> {
    let call = match receiver {
        Expr::FuncCall(c) => c,
        _ => return None,
    };
    // Verificar que o callee é o identificador "counter"
    let callee_name = match call.callee() {
        Expr::Ident(id) => id.as_str().to_string(),
        _ => return None,
    };
    if callee_name != "counter" { return None; }

    // Extrair o primeiro argumento posicional como chave string
    // No Typst, counter(heading) passa o tipo heading como valor;
    // aqui tratamos o nome do identificador como chave string.
    let first_arg = call.args().items().next()?;
    match first_arg {
        ast::Arg::Pos(Expr::Ident(id)) => Some(id.as_str().to_string()),
        ast::Arg::Pos(Expr::Str(s))    => Some(s.get().to_string()),
        _ => None,
    }
}
```

### Função auxiliar `eval_counter_method`

```rust
fn eval_counter_method(
    key:    &str,
    method: &str,
    args:   ast::Args<'_>,
    ctx:    &mut EvalContext<'_>,
) -> SourceResult<Value> {
    match method {
        "step" => Ok(Value::Content(Content::CounterUpdate {
            key:    key.to_string(),
            action: CounterAction::Step,
        })),

        "update" => {
            // Extrair o valor numérico do primeiro argumento
            // Segurança: se o argumento não for Int, ignorar silenciosamente
            let val = args.items().next()
                .and_then(|arg| match arg {
                    ast::Arg::Pos(expr) => {
                        if let Ok(Value::Int(n)) = eval_expr(expr, ctx) {
                            Some(n.max(0) as usize)
                        } else {
                            None
                        }
                    },
                    _ => None,
                })
                .unwrap_or(0);
            Ok(Value::Content(Content::CounterUpdate {
                key:    key.to_string(),
                action: CounterAction::Update(val),
            }))
        },

        // get(), display() e outros — fallback até motor de introspecção completo
        _ => Ok(Value::Content(Content::CounterDisplay {
            kind: key.to_string(),
        })),
    }
}
```

**Nota:** `Value::Content(...)` só existe se `Value` já tiver a variante
`Content`. Se não existir ainda, retornar directamente `Ok(Value::None)`
e produzir o `Content::CounterUpdate` via `eval_markup` como side-effect
no array de partes — seguir o mesmo padrão usado para `Content::Labelled`
no Passo 56.

---

## Tarefa 4 — Actualizar `CounterDisplay` no Layouter

O braço `Content::CounterDisplay` do Passo 57 só sabia mostrar headings.
Generalizar:

```rust
Content::CounterDisplay { kind } => {
    let text = if self.counter.hierarchical.contains_key(kind.as_str()) {
        self.counter.format_hierarchical(kind)
            .unwrap_or_else(|| "0".to_string())
    } else {
        self.counter.get_flat(kind).to_string()
    };
    self.layout_node(&Content::text(text), self.style)
},
```

---

## Tarefa 5 — Testes

### Testes L1 — `CounterState` genérico

```rust
#[test]
fn step_flat_incrementa() {
    let mut s = CounterState::new();
    s.step_flat("equation");
    assert_eq!(s.get_flat("equation"), 1);
    s.step_flat("equation");
    assert_eq!(s.get_flat("equation"), 2);
}

#[test]
fn update_flat_forca_valor() {
    let mut s = CounterState::new();
    s.step_flat("figure");
    s.update_flat("figure", 5);
    assert_eq!(s.get_flat("figure"), 5);
}

#[test]
fn step_hierarchical_comportamento_identico_ao_passo_57() {
    let mut s = CounterState::new();
    s.step_hierarchical("heading", 1); // [1]
    s.step_hierarchical("heading", 2); // [1, 1]
    s.step_hierarchical("heading", 1); // [2]
    assert_eq!(s.format_hierarchical("heading"), Some("2".to_string()));
}

#[test]
fn contadores_independentes_nao_interferem() {
    let mut s = CounterState::new();
    s.step_flat("equation");
    s.step_flat("equation");
    s.step_flat("figure");
    assert_eq!(s.get_flat("equation"), 2);
    assert_eq!(s.get_flat("figure"),   1);
}
```

### Testes L1 — `eval` intercepta MethodCall

```rust
#[test]
fn eval_counter_step_gera_counter_update() {
    let world = MockWorld::new("#counter(\"equation\").step()");
    let src = world.source(world.main()).unwrap();
    // O eval deve produzir Content::CounterUpdate { key: "equation", action: Step }
    // sem pânico. O mecanismo exacto de verificação depende de como o eval
    // expõe o Content produzido (via module.content() ou outro).
    assert!(eval_for_test(&world, &src).is_ok());
}

#[test]
fn eval_counter_update_gera_counter_update_com_valor() {
    let world = MockWorld::new("#counter(\"fig\").update(3)");
    let src = world.source(world.main()).unwrap();
    assert!(eval_for_test(&world, &src).is_ok());
}
```

### Testes L1 — Layout consome `CounterUpdate` sem desenhar

```rust
#[test]
fn counter_update_nao_produz_items_visuais() {
    use crate::entities::counter_state::{CounterAction, CounterState};
    use crate::rules::layout::layout_with_state;

    let content = Content::CounterUpdate {
        key:    "equation".to_string(),
        action: CounterAction::Update(5),
    };
    let doc = layout_with_state(&content, CounterState::new());
    // CounterUpdate não deve produzir nenhum FrameItem
    let total_items: usize = doc.pages.iter().map(|p| p.items.len()).sum();
    assert_eq!(total_items, 0, "CounterUpdate não deve gerar items visuais");
}

#[test]
fn counter_update_seguido_de_display_mostra_valor_correcto() {
    use crate::entities::counter_state::{CounterAction, CounterState};
    use crate::rules::layout::layout_with_state;

    let content = Content::Sequence(vec![
        Content::CounterUpdate {
            key:    "equation".to_string(),
            action: CounterAction::Update(5),
        },
        Content::CounterDisplay { kind: "equation".to_string() },
    ]);
    let doc = layout_with_state(&content, CounterState::new());
    assert!(doc.plain_text().contains("5"),
        "CounterDisplay deve mostrar '5' após Update(5): {:?}", doc.plain_text());
}
```

### Testes L3 — Pipeline completo

```rust
#[test]
fn pipeline_counter_step_nao_quebra_pdf() {
    let (world, _dir) = world_from_str("#counter(\"equation\").step()");
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty(), "PDF não deve estar vazio");
}

#[test]
fn pipeline_counter_update_nao_quebra_pdf() {
    let (world, _dir) = world_from_str("#counter(\"fig\").update(3)");
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty());
}
```

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint .
```

Critérios de conclusão:
- [ ] `CounterState` refactorizado com `hierarchical` e `flat` como
  `HashMap<String, ...>`.
- [ ] `step_heading` / `format_heading` / `heading_numbering` do Passo 57
  substituídos pelos equivalentes genéricos; call sites no Layouter
  actualizados.
- [ ] `CounterAction` criado e `Content::CounterUpdate` adicionado ao enum.
- [ ] Layouter consome `CounterUpdate` sem produzir items visuais.
- [ ] `eval` intercepta `MethodCall` para `step` e `update` sobre
  expressões `counter(key)`.
- [ ] `CounterDisplay` generalizado para chaves hierárquicas e planas.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- API exacta de `MethodCall` na AST cristalina — nomes dos métodos
  para aceder ao receiver, ao nome do método, e aos args.
- Se `Expr::MethodCall` já tinha braço no eval ou caia no wildcard.
- Se `Content::Equation` existe — relevante para decidir se o Passo 59
  atribui contador automático a equações ou apenas a figuras.

**Da implementação:**
- Se `Value::Content` existia ou foi necessário outro mecanismo para
  propagar `Content::CounterUpdate` a partir do eval.
- Se a substituição dos métodos do Passo 57 por equivalentes genéricos
  causou alguma regressão.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 59:**
- **GO — contadores automáticos**: `CounterUpdate` via MethodCall
  funciona; Passo 59 atribui contadores automáticos a `Content::Equation`
  e `Content::Figure` no Layouter (sem intervenção manual do utilizador).
- **GO — resolução de Ref via Label**: se a infraestrutura de Labels do
  Passo 56 está pronta para ser ligada ao `CounterState`, Passo 59
  resolve `Content::Ref` usando o contador do elemento a que a Label está
  associada.
- **NO-GO — MethodCall bloqueado**: se a AST cristalina não expõe
  `MethodCall` como variante de `Expr` (apenas como `FieldAccess` ou
  outro padrão), Passo 59 resolve a representação antes de avançar.
