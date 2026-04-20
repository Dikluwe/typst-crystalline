# Passo 67 — Expansão da Stdlib e Motor de Mutação de Texto

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — Para listar todas as variantes do enum
  `Content` e separar containers de terminais antes de escrever `map_text`.
- `01_core/src/rules/stdlib.rs` — Para entender o padrão actual de funções
  nativas e onde registar as novas.

Pré-condição: `cargo test` — 665 L1 + 125 L3, zero violations.
DEBT-18 encerrado. `assert` provado no Passo 66.

---

## Contexto

Este passo introduz o motor `map_text`, que aplica uma closure a todos os nós
`Content::Text` de uma árvore preservando a estrutura. É a fundação técnica
para as `#show` rules (Passo 68) e para as funções de transformação de texto
da stdlib (`upper`, `lower`, `replace`).

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Listar TODAS as variantes do enum Content com a sua estrutura
grep -n "^\s*[A-Z][A-Za-z]*" 01_core/src/entities/content.rs | head -60

# 2. Confirmar a assinatura de Content::text() (construtor ou variante directa)
grep -n "fn text\|Text(" 01_core/src/entities/content.rs | head -10

# 3. Listar funções nativas actuais e o padrão de registo no scope
grep -n "^fn native_\|scope.define" 01_core/src/rules/stdlib.rs | head -20

# 4. Confirmar a assinatura de cast_str e type_name em Value
grep -n "fn cast_str\|fn type_name" 01_core/src/entities/value.rs | head -10
```

Reportar o output completo antes de continuar.

O diagnóstico 1 determina a lista de terminais que entra no braço explícito
de `map_text`. O diagnóstico 2 confirma se `Content::Text(s.into())` é a
construção correcta ou se existe um construtor normalizado a usar.
O diagnóstico 4 confirma se `cast_str` existe — se não existir, ver nota
na Tarefa 3.

---

## Tarefa 1 — Motor `map_text` (L1)

Em `01_core/src/entities/content.rs`, adicionar o método:

```rust
impl Content {
    /// Aplica uma função de transformação a todos os nós `Content::Text`,
    /// preservando a estrutura da árvore.
    ///
    /// O uso de `&mut F` permite que a closure carregue estado entre chamadas
    /// (ex: um contador de substituições restantes), o que é necessário para
    /// que `replace(count: N)` funcione correctamente através de múltiplos nós.
    pub fn map_text<F>(&self, transform: &mut F) -> Self
    where
        F: FnMut(&str) -> String,
    {
        match self {
            // O caso alvo: aplicar a transformação e construir o nó directamente.
            // Construção directa (sem passar por Content::text()) para garantir
            // que nenhuma lógica de normalização altera o resultado da closure.
            Content::Text(s) => Content::Text(transform(s).into()),

            // --- Containers com filhos: propagar recursivamente ---
            // Cada variante de container listada explicitamente.
            // NÃO usar `_ =>` ou `other =>` para este grupo —
            // variantes novas passariam sem aviso do compilador.
            Content::Sequence(seq) => {
                Content::Sequence(seq.iter().map(|c| c.map_text(transform)).collect())
            },
            Content::Heading { level, body } => Content::Heading {
                level: *level,
                body: Box::new(body.map_text(transform)),
            },
            Content::Strong(body) => Content::Strong(Box::new(body.map_text(transform))),
            Content::Emph(body)   => Content::Emph(Box::new(body.map_text(transform))),
            Content::Labelled { target, label } => Content::Labelled {
                target: Box::new(target.map_text(transform)),
                label: label.clone(),
            },
            Content::Figure { body, caption } => Content::Figure {
                body: Box::new(body.map_text(transform)),
                caption: caption.as_ref().map(|c| Box::new(c.map_text(transform))),
            },
            Content::ListItem(body) => Content::ListItem(Box::new(body.map_text(transform))),
            Content::EnumItem { number, body } => Content::EnumItem {
                number: *number,
                body: Box::new(body.map_text(transform)),
            },
            Content::Link { url, body } => Content::Link {
                url: url.clone(),
                body: Box::new(body.map_text(transform)),
            },

            // --- Terminais: clonar directamente ---
            // Listar TODAS as variantes sem filhos, conforme o diagnóstico 1.
            // A lista abaixo deve ser completada com o resultado do diagnóstico 1.
            Content::Empty
            | Content::Space
            | Content::Linebreak
            | Content::Outline
            | Content::Raw(_)
            | Content::Ref(_)
            | Content::SetHeadingNumbering { .. }
            | Content::CounterUpdate { .. }
            | Content::CounterDisplay { .. }
            | Content::MathAlignPoint
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::MathFrac(_, _)
            | Content::MathAttach { .. }
            | Content::MathRoot { .. }
            | Content::MathDelimited { .. }
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            // | Content::OutraVariante  ← adicionar conforme o diagnóstico 1
            => self.clone(),

            // Se o compilador emitir `non_exhaustive_patterns`, a variante
            // em falta pertence a containers (adicionar com propagação recursiva)
            // ou a terminais (adicionar ao braço acima).
            // Nunca usar `_ =>` para resolver este erro.
        }
    }
}
```

**Aviso de exaustividade:** se o compilador emitir `non_exhaustive_patterns`,
determinar se a variante em falta tem filhos (container) ou não (terminal) e
adicioná-la ao grupo correcto. Nunca usar `_ =>` como saída.

---

## Tarefa 2 — Funções `upper` e `lower` (L1)

Em `01_core/src/rules/stdlib.rs`, adicionar as duas funções:

```rust
fn native_upper(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Str(s.to_uppercase().into())),
        [Value::Content(c)] => {
            let mut f = |text: &str| text.to_uppercase();
            Ok(Value::Content(c.map_text(&mut f)))
        },
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("upper() espera string ou content, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "upper() requer 1 argumento".into(),
        )]),
    }
}

fn native_lower(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Str(s.to_lowercase().into())),
        [Value::Content(c)] => {
            let mut f = |text: &str| text.to_lowercase();
            Ok(Value::Content(c.map_text(&mut f)))
        },
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("lower() espera string ou content, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "lower() requer 1 argumento".into(),
        )]),
    }
}
```

Registar no scope:

```rust
scope.define("upper", Value::Func(Func::native("upper", native_upper)));
scope.define("lower", Value::Func(Func::native("lower", native_lower)));
```

---

## Tarefa 3 — Função `replace` (L1)

```rust
fn native_replace(args: &Args) -> SourceResult<Value> {
    // Validar named args: apenas "count" é aceite.
    for key in args.named.keys() {
        if key.as_str() != "count" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado: '{}'", key),
            )]);
        }
    }

    // Validar número de argumentos posicionais.
    if args.items.len() < 3 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "replace() requer 3 argumentos: fonte, padrão, substituição".into(),
        )]);
    }

    // Extrair padrão e substituição como strings.
    // Nota: se cast_str não existir em Value (confirmar diagnóstico 4),
    // usar match directo sobre Value::Str(s) => s.as_str().
    let pattern = match &args.items[1] {
        Value::Str(s) => s.to_string(),
        other => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("replace(): padrão deve ser string, recebeu {}", other.type_name()),
        )]),
    };
    let replacement = match &args.items[2] {
        Value::Str(s) => s.to_string(),
        other => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("replace(): substituição deve ser string, recebeu {}", other.type_name()),
        )]),
    };

    // Bloquear padrão vazio: replacen("", ...) entra em ciclo infinito.
    if pattern.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "replace(): o padrão de busca não pode estar vazio".into(),
        )]);
    }

    // Extrair count opcional.
    let mut remaining_count: Option<i64> = args.named.get("count".into())
        .and_then(|v| match v {
            Value::Int(i) => Some(*i),
            _ => None,
        });

    // A closure carrega `remaining_count` como estado mutável.
    // `map_text` usa `&mut F`, por isso o estado persiste entre nós do AST.
    // Isto garante que `count: N` é global ao documento, não por nó de texto.
    let mut do_replace = |text: &str| -> String {
        match remaining_count.as_mut() {
            Some(c) if *c <= 0 => text.to_string(),
            Some(c) => {
                let limit = *c as usize;
                // Contar quantas substituições vão ocorrer neste nó.
                let count_used = text.matches(pattern.as_str()).take(limit).count();
                let result = text.replacen(pattern.as_str(), replacement.as_str(), limit);
                *c -= count_used as i64;
                result
            },
            None => text.replace(pattern.as_str(), replacement.as_str()),
        }
    };

    match &args.items[0] {
        Value::Str(s) => Ok(Value::Str(do_replace(s.as_str()).into())),
        Value::Content(c) => Ok(Value::Content(c.map_text(&mut do_replace))),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("replace(): 1º argumento deve ser string ou content, recebeu {}", other.type_name()),
        )]),
    }
}
```

Registar no scope:

```rust
scope.define("replace", Value::Func(Func::native("replace", native_replace)));
```

**Nota sobre `remaining_count`:** a closure usa `text.matches(...).take(limit).count()`
para contar as ocorrências que serão substituídas antes de chamar `replacen`.
Isto é necessário porque `replacen` não devolve o número de substituições feitas.
As duas operações são consistentes porque ambas usam o mesmo `limit`.

---

## Tarefa 4 — Testes

### Testes L1 — `map_text`

```rust
#[test]
fn map_text_transforma_texto_simples() {
    let content = Content::text("hello");
    let result = content.map_text(&mut |s| s.to_uppercase());
    assert_eq!(result, Content::text("HELLO"));
}

#[test]
fn map_text_desce_em_strong() {
    let content = Content::Strong(Box::new(Content::text("hello")));
    let result = content.map_text(&mut |s| s.to_uppercase());
    assert_eq!(result, Content::Strong(Box::new(Content::text("HELLO"))));
}

#[test]
fn map_text_preserva_terminais_sem_texto() {
    let content = Content::Space;
    let result = content.map_text(&mut |s| s.to_uppercase());
    assert_eq!(result, Content::Space);
}

#[test]
fn map_text_closure_com_estado_entre_nos() {
    // Validar que o estado da closure (FnMut) persiste entre nós distintos.
    let content = Content::Sequence(vec![
        Content::text("a"),
        Content::Strong(Box::new(Content::text("a"))),
        Content::text("a"),
    ]);
    let mut count = 0usize;
    content.map_text(&mut |s| {
        count += 1;
        s.to_string()
    });
    assert_eq!(count, 3, "A closure deve ser chamada uma vez por nó Text");
}
```

### Testes L1 — `upper` e `lower`

```rust
#[test]
fn eval_upper_string() {
    let world = MockWorld::new("#upper(\"hello\")");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    assert_eq!(module.last_value_str(), "HELLO");
}

#[test]
fn eval_lower_string() {
    let world = MockWorld::new("#lower(\"HELLO\")");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    assert_eq!(module.last_value_str(), "hello");
}

#[test]
fn eval_upper_content_preserva_strong() {
    let world = MockWorld::new("#upper([hello *world*])");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let content = module.content().unwrap();
    // O texto deve estar em maiúsculas; Strong deve ser preservado.
    assert_eq!(content.plain_text(), "HELLO WORLD");
    assert!(matches!(
        content,
        Content::Sequence(_)
    ), "Estrutura deve ser preservada");
}

#[test]
fn eval_upper_rejeita_named_arg() {
    let world = MockWorld::new("#upper(\"x\", bla: 1)");
    let src = world.source(world.main()).unwrap();
    assert!(eval_for_test(&world, &src).is_err());
}
```

### Testes L1 — `replace`

```rust
#[test]
fn eval_replace_string_simples() {
    let world = MockWorld::new("#replace(\"aabbaa\", \"a\", \"x\")");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    assert_eq!(module.last_value_str(), "xxbbxx");
}

#[test]
fn eval_replace_com_count() {
    let world = MockWorld::new("#replace(\"aaaa\", \"a\", \"b\", count: 2)");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    assert_eq!(module.last_value_str(), "bbaa");
}

#[test]
fn eval_replace_padrao_vazio_gera_erro() {
    let world = MockWorld::new("#replace(\"hello\", \"\", \"x\")");
    let src = world.source(world.main()).unwrap();
    assert!(eval_for_test(&world, &src).is_err());
}

#[test]
fn eval_replace_limite_parcial_entre_nos() {
    // AST gerado pelo parser: Text("aa ") + Strong(Text("aa")) + Text(" aa")
    // count: 3 — o limite é global ao documento, não por nó.
    //
    // Nó 1 — Text("aa "): substitui 2 'a' → "bb ", remaining = 1
    // Nó 2 — Strong(Text("aa")): substitui 1 'a' → "ba", remaining = 0
    // Nó 3 — Text(" aa"): remaining = 0, intacto → " aa"
    //
    // plain_text() concatena os três nós: "bb " + "ba" + " aa" = "bb ba aa"
    let world = MockWorld::new("#replace([aa *aa* aa], \"a\", \"b\", count: 3)");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let content = module.content().unwrap();
    assert_eq!(content.plain_text(), "bb ba aa");
}

#[test]
fn eval_replace_rejeita_named_arg_invalido() {
    let world = MockWorld::new("#replace(\"x\", \"a\", \"b\", bla: 1)");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err());
    assert!(result.unwrap_err()[0].message.contains("bla"));
}
```

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] `map_text` implementado em `content.rs` com dois braços explícitos:
  containers (propagação recursiva) e terminais (clone directo). Sem `_ =>`.
- [ ] A closure `FnMut` recebe `&mut F` — estado persiste entre nós do AST.
- [ ] `upper` e `lower` implementadas para `Value::Str` e `Value::Content`.
- [ ] `upper` e `lower` registadas no scope da stdlib.
- [ ] `replace` implementada com bloqueio de padrão vazio e `count` global.
- [ ] `replace` registada no scope da stdlib.
- [ ] Todos os testes acima passam.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Lista completa de variantes do enum `Content` separadas em containers e
  terminais — confirmar que a lista de terminais em `map_text` está completa.
- Se `cast_str` existe em `Value` ou se foi necessário usar `match` directo.

**Da implementação:**
- Se o compilador identificou variantes em falta no `match` de `map_text`
  — e quais foram adicionadas.
- Se o teste `eval_replace_limite_parcial_entre_nos` passou à primeira.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 68:**
- **GO — `#show` rules:** com `map_text` provado como motor de transformação
  estrutural, Passo 68 pode implementar `Content::Show { selector, transform }`
  usando o mesmo padrão de descida recursiva.
- **NO-GO — `map_text` incompleto:** se o compilador revelar variantes de
  container não cobertas que causam regressões; Passo 68 completa a cobertura
  antes de avançar.
