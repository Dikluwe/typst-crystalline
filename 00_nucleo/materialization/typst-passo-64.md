# Passo 64 — Desacoplamento do Avaliador e Evolução do `NativeFunc` (DEBT-16)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/func.rs` — Onde `NativeFunc` e `FuncRepr` estão definidos.
- `01_core/src/entities/args.rs` — Onde `Args` com `items` e `named` está definido.
- `01_core/src/rules/eval.rs` — Onde mora o interceptador de `figure` e onde os args são recolhidos.
- `01_core/src/rules/stdlib.rs` — Todas as funções nativas que vão sofrer a cascata.

Pré-condição: `cargo test` — 631 L1 + 121 L3 + 50 parity, zero violations.
DEBT-12, DEBT-13 encerrados. DEBT-16 registado.

---

## Contexto

O `NativeFunc` actual aceita apenas `&[Value]` (args posicionais). Quando o
Passo 62 precisou de `caption:` para `figure()`, o único caminho foi um
interceptador hardcoded em `eval.rs` — o avaliador passou a conhecer o nome
de uma função específica da stdlib. Cada interceptador deste tipo aumenta o
acoplamento e obriga o avaliador a crescer com cada nova função que precise
de named args.

Este passo resolve o problema pela raiz: a assinatura de `NativeFunc` passa a
aceitar também o mapa de named args. O interceptador de `figure` é removido,
e a função migra para `stdlib.rs` onde sempre deveria ter estado.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Definição actual de NativeFunc (type alias, struct, ou fn pointer?)
grep -n "NativeFunc\|type Native\|Native(" \
  01_core/src/entities/func.rs 01_core/src/entities/value.rs | head -10

# 2. Localizar o interceptador de figure em eval.rs
grep -n "figure" 01_core/src/rules/eval.rs -C 3 | head -20

# 3. Confirmar se Args já tem campo named (do Passo 17)
grep -n "pub named\|named:" 01_core/src/entities/args.rs | head -5

# 4. Contar as funções nativas em stdlib.rs — dimensão da cascata
grep -c "^fn native_\|^pub fn native_" 01_core/src/rules/stdlib.rs

# 5. Ver como apply_func despacha para NativeFunc actualmente
grep -n "Native\|native\|apply" 01_core/src/rules/eval.rs | head -15
```

Reportar o output completo antes de continuar. A resposta à questão 3 é
crítica: se `Args` já tem `named: IndexMap<EcoString, Value>` (do Passo 17),
a mudança em `NativeFunc` é simples — basta passar `&args.named` junto com
`&args.items`. Se não tem, é necessário adicionar primeiro.

---

## Tarefa 1 — Verificar e expandir `Args` com named (L1)

O diagnóstico (questão 3) determina se esta tarefa é necessária. Só avançar
para o código se `Args.named` não existir.

**Se o diagnóstico confirmar que `Args.named` já existe:** saltar esta tarefa.
Ir directamente para a Tarefa 2.

**Se `Args.named` não existir**, adicionar em `01_core/src/entities/args.rs`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub items: Vec<Value>,
    /// Argumentos nomeados. EcoString como chave — padrão do projecto (ADR-0024).
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
}

impl Args {
    pub fn positional(items: Vec<Value>) -> Self {
        Self { items, named: IndexMap::default() }
    }
}
```

Actualizar `eval_args` em `eval.rs` para popular `named` com os
`ast::Arg::Named` encontrados. **Os valores nomeados são expressões que
precisam de ser avaliadas recursivamente** — não entram como AST bruto:

```rust
fn eval_args(
    args_node: ast::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Args> {
    let mut items = Vec::new();
    let mut named  = IndexMap::default();
    for arg in args_node.items() {
        match arg {
            ast::Arg::Pos(expr) =>
                items.push(eval_expr(expr, scopes, ctx)?),
            ast::Arg::Named(name_node, value_expr) => {
                // CRÍTICO: avaliar a expressão do valor antes de inserir.
                // Inserir o AST bruto causaria panic ao tentar usar o Value.
                let key   = EcoString::from(name_node.as_str());
                let value = eval_expr(value_expr, scopes, ctx)?;
                named.insert(key, value);
            },
            ast::Arg::Spread(_) => {},  // fronteira deliberada — Passo 65+
        }
    }
    Ok(Args { items, named })
}
```

---

## Tarefa 2 — Nova assinatura de `NativeFunc` (L1)

**Fazer apenas esta tarefa primeiro. O `cargo check` vai ficar vermelho.
Não entrar em pânico — é esperado.**

Em `01_core/src/entities/func.rs`, actualizar a definição para receber
`&Args` em vez de dois parâmetros separados. Isto encapsula a estrutura e
preserva a capacidade de adicionar campos futuros (ex: `Span` da chamada)
sem nova cascata:

```rust
// ANTES:
pub struct NativeFunc {
    pub name: &'static str,
    pub call: fn(&[Value]) -> SourceResult<Value>,
}

// DEPOIS:
pub struct NativeFunc {
    pub name: &'static str,
    pub call: fn(&Args) -> SourceResult<Value>,
}
```

Em `apply_func`, a invocação simplifica-se:

```rust
FuncRepr::Native(native) => (native.call)(&args),
```

```bash
cargo check  # ← vai falhar em stdlib.rs e em eval.rs. Esperado.
```

---

## Tarefa 3 — Cascata na Stdlib (L1)

### Função auxiliar de validação

Adicionar em `stdlib.rs` antes de qualquer função nativa:

```rust
/// Verifica que não foram passados argumentos nomeados não esperados.
///
/// O Typst original é rigoroso: argumentos nomeados desconhecidos são
/// erros semânticos, não silenciosos. Ignorá-los criaria uma linguagem
/// permissiva que esconde typos do utilizador.
fn expect_no_named(named: &IndexMap<EcoString, Value, FxBuildHasher>)
    -> SourceResult<()>
{
    if let Some((key, _)) = named.iter().next() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("argumento nomeado inesperado: '{}'", key),
        )]);
    }
    Ok(())
}
```

### Actualizar todas as funções nativas

Corrigir a assinatura de todas as funções nativas para aceitar `&Args`.
Funções que não usam named args chamam `expect_no_named` no início:

```rust
fn native_type(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Str(v.type_name().into())),
        _   => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("type() requer 1 argumento, recebeu {}", args.items.len()),
        )]),
    }
}

fn native_len(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    // ... lógica existente usando args.items ...
}

fn native_range(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    // ... lógica existente usando args.items ...
}

// Repetir para todas as outras funções nativas que não aceitam named args.
```
```

### Migrar `native_figure` do interceptador para a stdlib

Após corrigir as funções existentes, mover o código do interceptador de
`eval.rs` para `stdlib.rs` como uma função nativa completa:

```rust
fn native_figure(args: &Args) -> SourceResult<Value> {
    // Argumento posicional: body (obrigatório)
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(other)             => Content::text(other.to_display_string()),
        None                    => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "figure() requer um argumento posicional (body)".to_string(),
        )]),
    };

    // Argumento nomeado: caption (opcional)
    // Value::None interceptado explicitamente — caption: none → ausência de legenda.
    let caption = args.named.get("caption".into()).and_then(|v| match v {
        Value::Content(c) => Some(Box::new(c.clone())),
        Value::Str(s)     => Some(Box::new(Content::text(s.as_str()))),
        Value::None       => None,
        other             => Some(Box::new(Content::text(other.to_display_string()))),
    });

    Ok(Value::Content(Content::Figure {
        body:    Box::new(body),
        caption,
    }))
}
```

Registar `native_figure` no scope da stdlib junto com as outras funções:

```rust
// No construtor do scope da stdlib:
scope.define("figure", Value::Func(Func::native("figure", native_figure)));
```

```bash
cargo check  # ← deve ficar verde em stdlib.rs. Ainda falha em eval.rs.
```

---

## Tarefa 4 — Purificação do Avaliador (L1)

### 4a — Actualizar `apply_func` para passar `&args`

Em `eval.rs`, onde `NativeFunc` é invocada, usar a struct `Args` completa:

```rust
FuncRepr::Native(native) => (native.call)(&args),
```

### 4b — Remover o interceptador de `figure`

Localizar o interceptador (resultado do diagnóstico 2) e apagá-lo
completamente. O `eval.rs` não deve ter nenhuma referência ao nome `"figure"`
após esta tarefa.

Após remover:

```bash
grep -n "figure" 01_core/src/rules/eval.rs  # deve retornar zero linhas
```

### 4c — Verificar que os testes L3 de figure continuam a passar

```bash
cargo test -p typst-infra -- pipeline_figure
```

Se passarem, o roteamento de named args pelo `eval.rs` está correcto.

```bash
cargo check  # ← deve ficar completamente verde agora
cargo test   # ← suíte completa
```

---

## Tarefa 5 — Testes

### Testes L1 — Named args no despacho

```rust
#[test]
fn eval_named_arg_passado_para_func_nativa() {
    // Verificar que named args chegam à função via o novo mecanismo,
    // não via interceptador. Usar figure() como caso de teste canónico.
    let world = MockWorld::new(
        "#figure([Conteúdo], caption: [Legenda])"
    );
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let content = module.content().expect("deve ter content");
    assert!(matches!(content, Content::Figure { caption: Some(_), .. }),
        "figure() com caption deve produzir Content::Figure com caption: {:?}", content);
}

#[test]
fn eval_named_arg_desconhecido_retorna_erro_semantico() {
    // O Typst é rigoroso: named args não esperados devem retornar Err,
    // não ser engolidos silenciosamente (o que esconderia typos do utilizador).
    // expect_no_named() em stdlib.rs garante este comportamento.
    let world = MockWorld::new(
        "#lower(\"TEXTO\", arg_invalido: true)"
    );
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err(), "named arg desconhecido deve retornar Err");
    let err = result.unwrap_err();
    assert!(
        err[0].message.contains("inesperado") || err[0].message.contains("unexpected"),
        "mensagem deve mencionar argumento inesperado: {:?}", err[0].message
    );
}

#[test]
fn eval_figure_sem_interceptador_em_eval_rs() {
    // Este teste verifica a propriedade arquitectural: eval.rs não conhece
    // "figure". Se o interceptador ainda existir, este comportamento de teste
    // seria o mesmo, mas o teste de grep (cargo check) confirmaria a pureza.
    // Usar como smoke test do pipeline completo.
    let world = MockWorld::new("#figure([A], caption: [B])");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let content = module.content().unwrap();
    assert!(matches!(content, Content::Figure { .. }));
}
```

---

## Verificação final

```bash
# Verificar pureza arquitectural:
grep -n "figure" 01_core/src/rules/eval.rs
# Deve retornar zero linhas (ou apenas comentários)

cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] Diagnóstico 3 verificado — se `Args.named` já existia ou foi adicionado.
- [ ] `Args.named: IndexMap<EcoString, Value>` existe (do Passo 17 ou adicionado neste passo).
- [ ] `NativeFunc.call` aceita `&Args` directamente.
- [ ] Função auxiliar `expect_no_named` adicionada em `stdlib.rs`.
- [ ] Todas as funções nativas em `stdlib.rs` que não aceitam named args chamam
  `expect_no_named(&args.named)?` no início.
- [ ] `native_figure` migrada de `eval.rs` para `stdlib.rs`, usando `args.named`.
- [ ] `native_figure` registada no scope da stdlib.
- [ ] `apply_func` passa `&args` para funções nativas.
- [ ] Zero referências a `"figure"` em `eval.rs` (excepto comentários).
- [ ] Testes L3 de `figure` continuam a passar.
- [ ] DEBT-16 marcado como **encerrado** em `00_nucleo/DEBT.md`.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `Args.named` já existia ou foi adicionado neste passo.
- Quantas funções nativas em `stdlib.rs` precisaram da assinatura actualizada
  (o número confirma a dimensão real da cascata).
- Se `NativeFunc` era type alias ou struct — e qual o impacto da mudança.

**Da implementação:**
- Se o interceptador de `figure` em `eval.rs` era o único, ou se havia outros.
- Se houve warnings de imports não usados após a remoção do interceptador.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 65:**
- **GO — DEBT-17 (fixpoint da TOC):** com DEBT-16 encerrado, Passo 65
  implementa o loop de convergência para substituir as 3 passagens fixas.
- **GO — DEBT-18 (contexto temporal na TOC):** se o corpus de testes revelar
  títulos com `CounterDisplay`, Passo 65 implementa a materialização do texto
  visual dos títulos na Passagem 1.
- **NO-GO — cascata quebrou funções existentes:** se alguma função nativa
  não compilou com a nova assinatura; Passo 65 resolve antes de avançar.
