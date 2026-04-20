# Passo 66 — Contexto Temporal na TOC (DEBT-18) e Prova de Fogo da Stdlib

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/introspect.rs` — Onde a TOC recolhe os títulos.
- `01_core/src/entities/content.rs` — Para confirmar a estrutura de `CounterDisplay` e listar todas as variantes do enum `Content`.
- `01_core/src/entities/counter_state.rs` — Onde o método `display_value` será adicionado.
- `01_core/src/rules/layout/counters.rs` — Para actualizar a chamada após mover a lógica para `CounterState`.
- `01_core/src/rules/stdlib.rs` — Onde a nova função `assert` será adicionada.

Pré-condição: `cargo test` — 654 L1 + 125 L3 + 50 parity, zero violations.
DEBT-16 e DEBT-17 encerrados.

---

## Contexto

### DEBT-18 — O problema

Quando a introspecção encontra um `Heading`, clona o AST do `body` directamente
para `headings_for_toc`. Se o utilizador escreveu
`= Capítulo #counter("cap").display()`, o clone contém o nó dinâmico
`CounterDisplay`. Quando a TOC é renderizada nas páginas iniciais, esse nó é
avaliado com o valor do contador naquele momento (início do documento) — não
o valor que o contador tinha quando o título ocorreu.

### A solução — Materialização de AST

Durante a Passagem 1 (Introspecção), o motor conhece o valor exacto de todos
os contadores no momento em que "pisa" num nó. Antes de guardar o clone do
título na TOC, "congela" o tempo: percorre o AST do título e substitui cada
`CounterDisplay` pelo seu valor em texto estático (`Content::Text`), usando o
`CounterState` actual.

### Prova de fogo da stdlib

Simultaneamente, implementar `assert(condition, message: ...)` na stdlib —
a primeira função com named args documentados (não apenas tolerados). Serve
como prova de que DEBT-16 foi bem pago e que o mecanismo de named args funciona
de ponta a ponta.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Localizar onde os títulos são empurrados para headings_for_toc
grep -n "headings_for_toc.push" 01_core/src/rules/introspect.rs

# 2. Listar TODAS as variantes do enum Content com a sua estrutura
grep -n "^\s*[A-Z][A-Za-z]*" 01_core/src/entities/content.rs | head -60

# 3. Confirmar que format_hierarchical e get_flat existem em CounterState
grep -n "fn format_hierarchical\|fn get_flat" 01_core/src/entities/counter_state.rs

# 4. Confirmar a estrutura exacta de CounterDisplay
grep -n "CounterDisplay" 01_core/src/entities/content.rs -A 3 | head -15

# 5. Listar funções nativas actuais para entender o padrão
grep -n "^fn native_" 01_core/src/rules/stdlib.rs
```

Reportar o output completo antes de continuar.

A resposta ao diagnóstico 2 é crítica: dela depende a lista de variantes
terminais e de containers que entrarão no `match` de `materialize_time`.
A resposta ao diagnóstico 3 confirma se `format_hierarchical` e `get_flat`
existem com os nomes esperados antes de escrever `display_value`.

---

## Tarefa 0 — Método `display_value` em `CounterState` (L1)

**Antes de qualquer outra tarefa**, adicionar o método seguinte em
`01_core/src/entities/counter_state.rs`:

```rust
/// Converte o valor actual de um contador para texto.
///
/// Centraliza a lógica de leitura aqui para que tanto `introspect.rs`
/// como `layout/counters.rs` a possam usar sem criar uma dependência
/// de `introspect` sobre `layout`.
pub fn display_value(&self, kind: &str) -> String {
    if self.hierarchical.contains_key(kind) {
        self.format_hierarchical(kind).unwrap_or_else(|| "0".to_string())
    } else {
        self.get_flat(kind).to_string()
    }
}
```

Após adicionar este método:

- Verificar se `layout/counters.rs` já duplica esta lógica. Se duplicar,
  substituir a duplicação por `state.display_value(kind)`.
- Esta tarefa não toca em `introspect.rs` nem em `stdlib.rs`.

**Porquê esta ordem:** `materialize_time` (Tarefa 1) chama `display_value`.
Se a Tarefa 0 não estiver concluída, a Tarefa 1 não compila.

---

## Tarefa 1 — Motor de Materialização (L1)

Em `01_core/src/rules/introspect.rs`, adicionar a função auxiliar:

```rust
/// "Congela" o AST substituindo nós dependentes de contexto (como CounterDisplay)
/// pelos seus valores em texto estático no momento exacto da introspecção.
///
/// Resolve DEBT-18: sem esta função, a TOC mostraria os valores dos contadores
/// no início do documento, não o valor que cada contador tinha quando o título ocorreu.
fn materialize_time(content: &Content, state: &CounterState) -> Content {
    match content {
        // O caso crítico: substituir o nó dinâmico pelo valor actual do contador.
        Content::CounterDisplay { kind } => {
            Content::text(state.display_value(kind))
        },

        // --- Containers com filhos: propagar recursivamente ---
        // Cada variante de container deve ser listada explicitamente.
        // NÃO usar `other => other.clone()` para este grupo —
        // um padrão genérico desactivaria a verificação de exaustividade
        // do compilador e variantes novas passariam sem aviso.
        Content::Sequence(seq) => {
            Content::Sequence(seq.iter().map(|c| materialize_time(c, state)).collect())
        },
        Content::Heading { level, body } => Content::Heading {
            level: *level,
            body: Box::new(materialize_time(body, state)),
        },
        Content::Strong(body) => Content::Strong(Box::new(materialize_time(body, state))),
        Content::Emph(body)   => Content::Emph(Box::new(materialize_time(body, state))),
        Content::Labelled { target, label } => Content::Labelled {
            target: Box::new(materialize_time(target, state)),
            label: label.clone(),
        },
        Content::Figure { body, caption } => Content::Figure {
            body:    Box::new(materialize_time(body, state)),
            caption: caption.as_ref().map(|c| Box::new(materialize_time(c, state))),
        },
        Content::ListItem(body) => Content::ListItem(Box::new(materialize_time(body, state))),
        Content::EnumItem { number, body } => Content::EnumItem {
            number: *number,
            body:   Box::new(materialize_time(body, state)),
        },
        Content::Link { url, body } => Content::Link {
            url:  url.clone(),
            body: Box::new(materialize_time(body, state)),
        },

        // --- Terminais: clonar directamente ---
        // Listar aqui TODAS as variantes sem filhos, conforme o diagnóstico 2.
        // Exemplo (completar com o resultado do diagnóstico 2):
        Content::Text(_)
        | Content::Space
        | Content::Empty
        | Content::Linebreak
        | Content::Outline
        | Content::CounterUpdate { .. }
        | Content::SetHeadingNumbering { .. }
        // | Content::MathXxx(...)   ← adicionar variantes matemáticas aqui
        // | Content::OutroTerminal  ← adicionar conforme o diagnóstico 2
        => content.clone(),

        // Se o compilador emitir `non_exhaustive_patterns`, a variante em falta
        // pertence a um dos dois grupos acima. Determinar se tem filhos
        // (adicionar ao grupo de containers) ou não (adicionar ao grupo de terminais).
        // Nunca usar `_ =>` como saída para este erro.
    }
}
```

**Aviso de exaustividade:** o braço de terminais deve listar explicitamente
todas as variantes sem filhos. Se o compilador emitir `non_exhaustive_patterns`,
a variante em falta pertence a um dos dois grupos acima. Determinar se tem filhos
(adicionar ao grupo de containers com propagação recursiva) ou não (adicionar ao
grupo de terminais com `.clone()`). Nunca resolver este erro com `_ =>` — isso
reintroduz o problema que o braço explícito resolve.

---

## Tarefa 2 — Congelar os Títulos na Passagem 1 (L1)

Em `introspect.rs`, no braço `Content::Heading` da função `walk`, substituir:

```rust
// ANTES — clone do AST dinâmico:
state.headings_for_toc.push((auto_label, *body.clone(), *level as usize));

// DEPOIS — AST congelado com os valores do momento presente:
let frozen_body = materialize_time(body, state);
state.headings_for_toc.push((auto_label, frozen_body, *level as usize));
```

Esta é a única mudança na lógica central — duas linhas que resolvem o problema
de arquitectura de passagens que o DEBT-18 descreve.

---

## Tarefa 3 — `assert` na stdlib (L1)

**Verificação de codificação antes de escrever:** confirmar que o ficheiro
`01_core/src/rules/stdlib.rs` está em UTF-8 antes de adicionar qualquer
string com caracteres acentuados (`ç`, `ã`):

```bash
file -i 01_core/src/rules/stdlib.rs
# Output esperado: charset=utf-8
# Se não for utf-8, converter com iconv antes de continuar.
```

Em `01_core/src/rules/stdlib.rs`, adicionar:

```rust
/// Função nativa `assert(condition, message: ...)`.
///
/// Prova de fogo do DEBT-16: a primeira função com named arg documentado
/// (não apenas tolerado). Valida que o mecanismo de named args funciona
/// de ponta a ponta.
fn native_assert(args: &Args) -> SourceResult<Value> {
    // Validar named args: apenas "message" é aceite.
    for key in args.named.keys() {
        if key.as_str() != "message" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado: '{}'", key),
            )]);
        }
    }

    // Argumento posicional: condição (obrigatório)
    let condition = match args.items.first() {
        Some(Value::Bool(b)) => *b,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("assert() requer condição booleana, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "assert() requer 1 argumento posicional (condição)".to_string(),
        )]),
    };

    // Argumento nomeado: message (opcional)
    let message = args.named.get("message".into())
        .and_then(|v| match v {
            Value::Str(s)     => Some(s.to_string()),
            Value::Content(c) => Some(c.plain_text()),
            other             => Some(other.to_display_string()),
        })
        .unwrap_or_else(|| "Asserção falhou".to_string());

    if !condition {
        return Err(vec![SourceDiagnostic::error(Span::detached(), message)]);
    }

    Ok(Value::None)
}
```

Registar no scope da stdlib:

```rust
scope.define("assert", Value::Func(Func::native("assert", native_assert)));
```

---

## Tarefa 4 — Testes

### Testes L1 — Materialização temporal

```rust
#[test]
fn materialize_time_substitui_counter_display() {
    use crate::entities::counter_state::CounterState;
    use super::materialize_time;

    let mut state = CounterState::new();
    state.update_flat("fig", 42);

    let dynamic_ast = Content::Sequence(vec![
        Content::text("Figura "),
        Content::CounterDisplay { kind: "fig".to_string() },
    ]);

    let frozen = materialize_time(&dynamic_ast, &state);

    let expected = Content::Sequence(vec![
        Content::text("Figura "),
        Content::text("42"),
    ]);

    assert_eq!(frozen, expected,
        "CounterDisplay deve ser materializado em Text com o valor do contador");
}

#[test]
fn materialize_time_preserva_terminais() {
    use crate::entities::counter_state::CounterState;
    use super::materialize_time;

    let state = CounterState::new();

    // Nós terminais sem CounterDisplay devem ser clonados sem alteração.
    let content = Content::Sequence(vec![
        Content::text("Texto estático"),
        Content::Strong(Box::new(Content::text("Negrito"))),
    ]);

    let frozen = materialize_time(&content, &state);
    assert_eq!(frozen, content, "Terminais sem CounterDisplay não devem ser alterados");
}

#[test]
fn introspect_headings_for_toc_congelados() {
    use crate::rules::introspect::introspect;

    // Simular: = Figura #counter("fig").display()
    // O CounterDisplay no título deve ser substituído pelo valor no momento
    // da introspecção — não pelo valor quando a TOC for renderizada.
    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::CounterUpdate {
            key:    "fig".to_string(),
            action: crate::entities::counter_state::CounterAction::Update(7),
        },
        Content::heading(1, Content::Sequence(vec![
            Content::text("Figura "),
            Content::CounterDisplay { kind: "fig".to_string() },
        ])),
    ]);

    let state = introspect(&content);
    assert_eq!(state.headings_for_toc.len(), 1);

    let (_, frozen_body, _) = &state.headings_for_toc[0];
    let text = frozen_body.plain_text();
    // O body congelado deve conter "7" (valor no momento da introspecção),
    // não "0" (valor no início do documento quando a TOC é renderizada).
    assert!(text.contains("7"),
        "CounterDisplay no título deve ser congelado com o valor correcto: {:?}", text);
}
```

### Testes L1 — stdlib `assert`

```rust
#[test]
fn eval_assert_true_nao_gera_erro() {
    let world = MockWorld::new("#assert(1 == 1)");
    let src = world.source(world.main()).unwrap();
    assert!(eval_for_test(&world, &src).is_ok(), "assert(true) deve ter sucesso");
}

#[test]
fn eval_assert_false_gera_erro_com_mensagem_padrao() {
    let world = MockWorld::new("#assert(false)");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err[0].message.contains("falhou") || err[0].message.contains("Asser"),
        "mensagem de erro padrão deve mencionar a asserção: {:?}", err[0].message
    );
}

#[test]
fn eval_assert_false_gera_erro_com_mensagem_personalizada() {
    let world = MockWorld::new(
        "#assert(1 == 2, message: \"Matematica falhou\")"
    );
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err());
    assert!(result.unwrap_err()[0].message.contains("Matematica falhou"));
}

#[test]
fn eval_assert_rejeita_named_arg_invalido() {
    let world = MockWorld::new("#assert(true, bla: \"bla\")");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err[0].message.contains("inesperado") && err[0].message.contains("bla"),
        "named arg desconhecido deve gerar erro: {:?}", err[0].message
    );
}
```

**Nota sobre o teste `eval_assert_false_gera_erro_com_mensagem_personalizada`:**
a string literal no código de teste foi escrita sem acentos (`"Matematica falhou"`)
para evitar que uma eventual incompatibilidade de codificação no ambiente de CI
faça o teste falhar por razão diferente da que está a ser testada. A mensagem
de erro na implementação pode continuar com acentos desde que o ficheiro seja
UTF-8 confirmado.

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] `display_value` adicionado em `counter_state.rs`. `layout/counters.rs`
  actualizado para usar `state.display_value(kind)` se havia duplicação.
- [ ] `materialize_time` implementada em `introspect.rs` com dois braços
  explícitos: containers (propagação recursiva) e terminais (clone directo).
  Sem wildcard (`_ =>` ou `other =>`) para containers.
- [ ] `materialize_time` chamada no braço `Heading` antes de `push` para
  `headings_for_toc`.
- [ ] `Content::CounterDisplay` substituído por `Content::Text` com o valor
  actual do contador via `state.display_value`.
- [ ] `native_assert` implementada com validação estrita de named args.
- [ ] `assert` registada no scope da stdlib.
- [ ] `assert(true)` não gera erro; `assert(false, message: "...")` gera erro
  com a mensagem correcta.
- [ ] DEBT-18 marcado como **encerrado** em `01_core/DEBT.md`.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Lista completa de variantes do enum `Content` separadas em containers e terminais
  — confirmar quais foram adicionadas ao braço de terminais de `materialize_time`.
- Se `layout/counters.rs` tinha lógica duplicada que foi substituída por
  `state.display_value`.
- Resultado do `file -i stdlib.rs` (confirmar UTF-8).

**Da implementação:**
- Se o compilador identificou variantes de container em falta no match de
  `materialize_time` — e quais foram adicionadas.
- Se o teste `introspect_headings_for_toc_congelados` passou à primeira ou
  foi necessário ajustar a semântica de `CounterUpdate` na introspecção.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 67:**
- **GO — expansão da stdlib:** com `assert` provando o mecanismo, Passo 67
  pode adicionar funções como `upper()`, `lower()`, `str()` com named args reais.
- **GO — suporte a `#show` rules:** com a introspecção estabilizada, Passo 67
  pode abordar `Content::Show` para transformações de estilo por tipo de nó.
- **NO-GO — `materialize_time` incompleta:** se o compilador revelar variantes
  de container não cobertas que causam regressões; Passo 67 completa a cobertura
  antes de avançar.
