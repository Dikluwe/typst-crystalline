# Passo 68 — Regras Show de Transformação (Intercepção Eager Blindada)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — `EvalContext` actual e braço `Expr::Show` (se existir).
- `01_core/src/entities/content.rs` — Confirmar variantes de container para `map_text`.
- `01_core/src/rules/stdlib.rs` — Onde `upper`, `lower`, `map_text` foram implementados
  no Passo 67.

Pré-condição: `cargo test` — 678 L1 + 125 L3 + 50 parity, zero violations.
`map_text`, `upper`, `lower`, `replace` implementados no Passo 67.

**DEBTs registados para este passo:**
- DEBT-19: Avaliação superficial (`NodeKind` avalia apenas o nó raiz). `map_content`
  transversal e substituição Texto→Content via Regex não suportados.
- DEBT-20: Prevenção de recursão infinita em show rules. O guard `in_show_transform`
  mitiga mas não elimina todos os casos patológicos.

---

## Contexto

O `#show selector: transform` do Typst permite transformar nós de conteúdo de
forma declarativa. Exemplo: `#show heading: it => upper(it.body)` transforma
todos os headings em maiúsculas.

A arquitectura deste passo tem quatro invariantes:

1. **Eager e no ponto de criação:** a intercepção ocorre imediatamente após a
   avaliação de uma função que retorna `Content` — não numa segunda passagem.
2. **Escopo léxico:** uma regra declarada dentro de um bloco `{ }` expira quando
   o bloco termina.
3. **Falha explícita:** transformações que retornam tipos inválidos (ex: `true`)
   geram `SourceDiagnostic`, não mascaramento silencioso.
4. **Guard anti-recursão:** `in_show_transform` impede que uma regra que emite
   o mesmo tipo de nó que interceta cause stack overflow.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Verificar se Expr::Show já existe na AST cristalina
grep -n "Show\b" 01_core/src/entities/ast/expr.rs | head -10

# 2. Verificar se EvalContext já tem show_rules ou campos relacionados
grep -n "show_rules\|in_show\|ShowRule" 01_core/src/rules/eval.rs | head -10

# 3. Confirmar que Func tem método name() (necessário para resolver NodeKind)
grep -n "pub fn name\|fn name" 01_core/src/entities/func.rs | head -5

# 4. Confirmar a assinatura exacta de map_text — crítico para a Tarefa 4
grep -n "pub fn map_text\|fn map_text" 01_core/src/entities/content.rs | head -5
# A Tarefa 4 assume: pub fn map_text<F: FnMut(&str) -> String>(&self, f: &mut F) -> Self
# Se a assinatura for diferente (ex: consume self, retorna outro tipo), o código não compila.

# 5. Verificar o braço actual de Expr::Show no eval (se existir)
grep -n "Show\|show" 01_core/src/rules/eval.rs | head -10
```

Reportar o output completo antes de continuar. A resposta à questão 3 é
crítica: se `Func` não tem `name()`, a resolução do `NodeKind` a partir do
nome da função precisa de uma abordagem alternativa (ex: comparar com o
`Func` registado na stdlib por identidade de ponteiro).

---

## Tarefa 1 — Entidade `ShowRule` (L1)

Criar `01_core/src/entities/show.rs`:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/show.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-04-13

use crate::entities::value::Value;

/// Tipo de nó de conteúdo para selectorção por tipo.
///
/// DEBT-19: subset inicial — apenas Heading e Figure.
/// Outros tipos (Raw, ListItem, Equation, etc.) adicionados em passos futuros.
#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Heading,
    Figure,
}

/// Selector de uma show rule.
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    /// Substitui ocorrências literais de um texto.
    /// Ex: `#show "A": "B"`
    Text(String),
    /// Interceta nós de um tipo específico.
    /// Ex: `#show heading: it => ...`
    NodeKind(NodeKind),
}

/// Uma regra de transformação declarada com `#show selector: transform`.
#[derive(Debug, Clone)]
pub struct ShowRule {
    pub selector:  Selector,
    pub transform: Value,
}
```

Criar o prompt L0 `00_nucleo/prompts/entities/show.md` antes de continuar:

```bash
git add 00_nucleo/prompts/entities/show.md
crystalline-lint --fix-hashes .
```

Registar os seguintes DEBTs em `01_core/DEBT.md` antes de escrever qualquer código:

```markdown
### DEBT-19 — Avaliação superficial de NodeKind (Passo 68)
O `apply_show_rules` com `Selector::NodeKind` interceta apenas o nó raiz.
Um Heading dentro de um Sequence passa despercebido. Resolução: implementar
`map_content` transversal (Passo 69).

### DEBT-20 — Guard anti-recursão booleano global (Passo 68)
O flag `in_show_transform` desliga o motor de show rules durante qualquer
transformação, incluindo regras aninhadas legítimas. Resolução: substituir
por mecanismo de marcação de nós ("já transformados") para permitir
composição de show rules sem stack overflow.

### DEBT-21 — Resolução de NodeKind baseada em string (Passo 68)
`Func::name()` é usado para identificar NodeKind via comparação de string.
Falha com aliasing e closures anónimas. Resolução: verificar identidade da
função nativa por ponteiro ou por ID interno único (stringly-typed → typed).

### DEBT-22 — Clone de show_rules por nó (Passo 68)
`ctx.show_rules.clone()` em `intercept_content` é O(N) por cada nó de
conteúdo gerado. Em documentos grandes com muitas regras, o custo acumula.
Resolução: usar `Rc<[ShowRule]>` ou indexação para partilhar a lista sem
copiar, separando o estado de mutação da leitura.
```

Registar em `entities/mod.rs`:

```rust
pub mod show;
```

---

## Tarefa 2 — Estado no `EvalContext` (L1)

Em `01_core/src/rules/eval_context.rs` (ou onde `EvalContext` está definido),
adicionar dois campos:

```rust
use crate::entities::show::ShowRule;

pub(crate) struct EvalContext<'w> {
    pub world:             Tracked<'w, dyn TrackedWorld + 'w>,
    pub depth:             usize,
    /// Show rules activas no escopo actual.
    /// Crescem com `#show` e são truncadas ao sair de um bloco.
    pub show_rules:        Vec<ShowRule>,
    /// Guard anti-recursão: impede que apply_show_rules seja invocada
    /// recursivamente durante uma transformação show (DEBT-20).
    pub in_show_transform: bool,
}

impl<'w> EvalContext<'w> {
    pub fn new(world: Tracked<'w, dyn TrackedWorld + 'w>) -> Self {
        Self {
            world,
            depth:             0,
            show_rules:        Vec::new(),
            in_show_transform: false,
        }
    }
}
```

---

## Tarefa 3 — Braço `Expr::Show` no Avaliador (L1)

Em `eval.rs`, no braço de `Expr::Show` (criar se não existir):

```rust
ast::Expr::Show(show_expr) => {
    // Avaliar o selector — pode ser uma string ou uma função da stdlib
    let selector_val = eval_expr(show_expr.selector(), scopes, ctx)?;
    let selector = match selector_val {
        Value::Str(s) => Selector::Text(s.to_string()),
        Value::Func(ref f) => {
            // Resolver o NodeKind pelo nome da função.
            // Confirmar com o diagnóstico 3 se Func::name() existe.
            match f.name() {
                Some("heading") => Selector::NodeKind(NodeKind::Heading),
                Some("figure")  => Selector::NodeKind(NodeKind::Figure),
                Some(other) => return Err(vec![SourceDiagnostic::error(
                    show_expr.selector().span(),
                    format!(
                        "o selector de show rule exige uma função nativa conhecida \
                         (ex: heading, figure). Recebeu: '{}' (DEBT-21: identificação \
                         por nome — funciona apenas com funções nativas registadas)",
                        other
                    ),
                )]),
                None => return Err(vec![SourceDiagnostic::error(
                    show_expr.selector().span(),
                    "selector de show rule requer uma função ou string".to_string(),
                )]),
            }
        },
        other => return Err(vec![SourceDiagnostic::error(
            show_expr.selector().span(),
            format!("selector inválido para show rule: {}", other.type_name()),
        )]),
    };

    // Avaliar a transformação (closure ou valor estático)
    let transform = eval_expr(show_expr.transform(), scopes, ctx)?;
    ctx.show_rules.push(ShowRule { selector, transform });

    Ok(Value::None)
},
```

---

## Tarefa 4 — Motor de Intercepção `apply_show_rules` (L1)

Em `eval.rs` ou num ficheiro dedicado `rules/show.rs`:

```rust
/// Aplica as show rules activas ao Content.
///
/// Ordem de aplicação: sequencial na ordem de declaração.
/// Ex: `#show "A": "B"` seguido de `#show "B": "C"` transforma "A" em "C".
///
/// DEBT-19: a intercepção por NodeKind avalia apenas o nó raiz. Um Heading
/// dentro de um Sequence não é intercetado — requer map_content transversal.
pub(crate) fn apply_show_rules(
    mut content: Content,
    rules:       &[ShowRule],
    ctx:         &mut EvalContext<'_>,
) -> SourceResult<Content> {
    if rules.is_empty() {
        return Ok(content);
    }

    for rule in rules {
        content = match &rule.selector {
            Selector::NodeKind(kind) => {
                let is_match = match (&content, kind) {
                    (Content::Heading { .. }, NodeKind::Heading) => true,
                    (Content::Figure  { .. }, NodeKind::Figure)  => true,
                    _                                             => false,
                };

                if is_match {
                    match &rule.transform {
                        Value::Func(func) => {
                            let args = Args::positional(vec![Value::Content(content.clone())]);
                            let result = apply_func(func.clone(), args, ctx)?;

                            match result {
                                Value::Content(c) => c,
                                // Coerção via construtor canónico — alinhado com o checklist.
                                // Content::text(s) é o ponto único de criação de texto;
                                // usar Content::Text(s.into()) directamente contornaria
                                // qualquer normalização futura aplicada pelo construtor.
                                Value::Str(s) => Content::text(s),
                                // Falha explícita para tipos inválidos
                                other => return Err(vec![SourceDiagnostic::error(
                                    Span::detached(),
                                    format!(
                                        "show rule deve retornar Content ou String, \
                                         mas retornou {}",
                                        other.type_name()
                                    ),
                                )]),
                            }
                        },
                        Value::Content(c) => c.clone(),
                        // Transformação estática inválida (ex: #show heading: 42)
                        // — falhar explicitamente (não mascarar silenciosamente)
                        other => return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "show rule com selector de tipo requer função ou Content, \
                                 recebeu {}",
                                other.type_name()
                            ),
                        )]),
                    }
                } else {
                    content
                }
            },

            Selector::Text(pattern) => {
                match &rule.transform {
                    Value::Str(s) => {
                        let replacement = s.to_string();
                        let mut do_replace = |text: &str| text.replace(pattern.as_str(), &replacement);
                        content.map_text(&mut do_replace)
                    },
                    // Transformação por função em strings não suportada neste passo.
                    // Nunca falhar silenciosamente — disparar erro explícito.
                    // DEBT-19: substituição Texto→Content via função requer split de nós.
                    Value::Func(_) => return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "transformação por função para selector de texto ainda não \
                         suportada (DEBT-19); use uma string literal: \
                         #show \"texto\": \"substituto\"".to_string(),
                    )]),
                    Value::Content(_) => return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "substituição Texto→Content não suportada (DEBT-19)".to_string(),
                    )]),
                    other => return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "tipo de transformação inválido para selector de texto: {}; \
                             use uma string literal",
                            other.type_name()
                        ),
                    )]),
                }
            },
        };
    }

    Ok(content)
}
```

---

## Tarefa 5 — Intercepção Eager com Guard Anti-Recursão (L1)

A intercepção deve ocorrer em **todos os pontos** onde a avaliação de um nó
AST produz `Value::Content` — não apenas após `apply_func`. Em Typst, a
maioria do conteúdo nasce de markup puro (`= Título`, `*Negrito*`) que o
`eval_expr` converte directamente em `Content` sem passar por uma função.

### 5a — Extrair o helper de intercepção

Definir uma função auxiliar para centralizar a lógica:

```rust
/// Aplica show rules ao Content se não estivermos já dentro de uma transformação.
/// Centraliza a intercepção para que seja chamada em todos os pontos de saída.
///
/// IMPORTANTE — Restauração incondicional da flag:
/// O operador `?` faria early return antes de `in_show_transform = false` se
/// `apply_show_rules` falhasse, deixando a flag presa em `true` e "envenenando"
/// o motor para o resto do documento. Por isso, o resultado é capturado primeiro,
/// a flag é restaurada incondicionalmente, e só então o erro é propagado.
pub(crate) fn intercept_content(
    content: Content,
    ctx:     &mut EvalContext<'_>,
) -> SourceResult<Content> {
    if ctx.in_show_transform || ctx.show_rules.is_empty() {
        return Ok(content);
    }

    ctx.in_show_transform = true;
    let rules  = ctx.show_rules.clone(); // snapshot explícito — mal necessário

    // Capturar sem short-circuit (`?` aqui envenenaria o estado)
    let result = apply_show_rules(content, &rules, ctx);

    // Restaurar INCONDICIONALMENTE — independentemente de result ser Ok ou Err
    ctx.in_show_transform = false;

    result // propagar Ok ou Err após restauração garantida
}
```

### 5b — Invocar em todos os pontos de saída de Content

**Após `apply_func`:**

```rust
let mut result_value = apply_func(func, args, ctx)?;
if let Value::Content(c) = result_value {
    result_value = Value::Content(intercept_content(c, ctx)?);
}
```

**Nos braços de markup que produzem Content directamente** (confirmar com o
diagnóstico quais existem — exemplos prováveis):

```rust
// Braço de Heading no markup (ex: ast::Expr::Heading ou equivalente):
SyntaxKind::Heading => {
    let content = Content::heading(level, body);
    // ← intercepção eager: show rules aplicam-se a headings de markup também
    let content = intercept_content(content, ctx)?;
    parts.push(content);
}

// Braço de Figure (se criada via markup, não apenas via função):
// ... idem ...
```

O diagnóstico (questão 5) revelará quais braços de `eval_expr` produzem
`Content::Heading`, `Content::Figure`, etc. directamente — adicionar
`intercept_content` em cada um deles.

**Nota sobre o guard `in_show_transform`:** é um booleano global que desliga
o motor de show rules durante uma transformação. Isto previne stack overflow
mas impede regras aninhadas. Documentar no teste anti-recursão (ver Tarefa 7)
para que o próximo engenheiro compreenda o compromisso.

---

## Tarefa 6 — Guardião do Escopo Léxico (L1)

Em `eval_block` (ou onde blocos de código são avaliados), truncar as show
rules ao sair:

```rust
fn eval_block(block: ast::CodeBlock<'_>, scopes: &mut Scopes<'_>, ctx: &mut EvalContext<'_>)
    -> SourceResult<Value>
{
    // Guardar o comprimento antes do bloco — show rules adicionadas dentro
    // do bloco não devem vazar para o escopo exterior.
    let rules_len_before = ctx.show_rules.len();

    scopes.push();
    let mut last = Value::None;
    for expr in block.body().exprs() {
        last = eval_expr(expr, scopes, ctx)?;
    }
    scopes.pop();

    // Truncar para remover regras do escopo do bloco — invariante de escopo léxico.
    ctx.show_rules.truncate(rules_len_before);

    Ok(last)
}
```

---

## Tarefa 7 — Testes

### Testes L1 — Show rules básicas

```rust
#[test]
fn eval_show_rule_text_substitui_ocorrencias() {
    let world = MockWorld::new("#show \"A\": \"B\"\nAAA");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert!(text.contains("BBB") || text.contains("B"),
        "show text rule deve substituir 'A' por 'B': {:?}", text);
    assert!(!text.contains("AAA"),
        "texto original não deve sobreviver: {:?}", text);
}

#[test]
fn eval_show_rule_funcao_no_heading() {
    // upper() aplicado ao body do heading via show rule
    let world = MockWorld::new("#show heading: it => upper(it.body)\n\n= Capítulo um");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert!(text.contains("CAPÍTULO UM"),
        "show rule deve transformar heading em maiúsculas: {:?}", text);
}

#[test]
fn eval_show_rule_falha_explicita_tipo_retorno_invalido() {
    // Retornar um booleano deve causar Err — não mascaramento silencioso
    let world = MockWorld::new("#show heading: it => true\n\n= Erro");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err(), "retornar bool de show rule deve gerar Err");
    let err = result.unwrap_err();
    assert!(
        err[0].message.contains("Content") || err[0].message.contains("String"),
        "mensagem deve mencionar os tipos aceites: {:?}", err[0].message
    );
}

#[test]
fn show_rule_respeita_escopo_lexico() {
    // A regra dentro do bloco não deve afectar o texto fora
    let world = MockWorld::new("{ #show \"A\": \"B\" }\nA");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert!(text.trim().ends_with('A') || text.contains("A"),
        "show rule do bloco não deve afectar texto exterior: {:?}", text);
}

#[test]
fn show_rule_nao_recursiva_sem_stack_overflow() {
    // Guard in_show_transform previne loop infinito (DEBT-20).
    // Nota: o guard é global — enquanto activo, NENHUMA outra show rule
    // dispara, incluindo regras para tipos diferentes. Este é o compromisso
    // arquitectural do Passo 68. Ex: #show heading → strong e #show strong →
    // upper NÃO funcionarão em cadeia (DEBT-20 cobre isto).
    let world = MockWorld::new("#show heading: it => [= X ]\n\n= A");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok() || result.is_err(),
        "avaliação deve terminar — nunca entrar em loop infinito");
}

#[test]
fn show_rule_encadeamento_texto_sequencial() {
    // A transforma em B, depois B transforma em C — resultado final deve ser C.
    // Valida que as regras são aplicadas na ordem de declaração (sequencial).
    let world = MockWorld::new("#show \"A\": \"B\"\n#show \"B\": \"C\"\nA");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert_eq!(text.trim(), "C",
        "encadeamento sequencial de show rules de texto deve produzir 'C'");
}
```

**Comportamentos a documentar (não corrigir no Passo 68):**

- **Composição de regras bloqueada (DEBT-20):** o guard `in_show_transform`
  é global — `#show heading: it => strong(it)` seguido de
  `#show strong: it => upper(it)` não aplica a segunda regra. Esperado.
- **Performance de clones (DEBT-22):** `ctx.show_rules.clone()` por cada nó
  é O(N×R). Em documentos com 10.000 nós e 5 regras, são 50.000 cópias.
  Aceitável agora; inevitável refactorizar para `Rc<[ShowRule]>` em Passo 69+.

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] Prompt L0 `entities/show.md` criado com `git add` antes de qualquer código.
- [ ] `ShowRule`, `Selector`, `NodeKind` criados em `entities/show.rs`.
- [ ] `show_rules: Vec<ShowRule>` e `in_show_transform: bool` em `EvalContext`.
- [ ] Braço `Expr::Show` em `eval.rs` regista a regra no contexto.
- [ ] `apply_show_rules` com falha explícita para tipos de retorno inválidos.
- [ ] `intercept_content()` restaura `in_show_transform = false` incondicionalmente
  antes de propagar o resultado — sem `?` entre a activação e a restauração da flag.
- [ ] Intercepção eager invocada após `apply_func` E nos braços de markup
  que produzem `Content` directamente (ex: Heading de markup).
- [ ] `Selector::Text` com `Value::Func` ou `Value::Content` gera `Err`
  explícito — zero falhas silenciosas.
- [ ] `Content::text(s)` em vez de `Content::Text(s.into())` na coerção String.
- [ ] `eval_block` trunca `show_rules` ao sair.
- [ ] Teste de escopo léxico passa.
- [ ] Teste anti-recursão termina sem panic.
- [ ] DEBT-19, DEBT-20, DEBT-21 e DEBT-22 registados em `01_core/DEBT.md`
  antes de qualquer código.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `Func::name()` existe — e qual a abordagem usada para resolver `NodeKind`
  se não existir.
- Se `Expr::Show` já existia na AST cristalina ou foi necessário adicionar o braço.
- Como `map_text` está assinado em `content.rs` (confirmar compatibilidade com
  a Tarefa 4).

**Da implementação:**
- Se `ctx.show_rules.clone()` na Tarefa 5 causou problemas de performance
  visíveis nos testes.
- Se o guard `in_show_transform` foi suficiente para o teste anti-recursão, ou
  se foram necessárias medidas adicionais.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 69:**
- **GO — DEBT-19 (`map_content` transversal):** Passo 69 implementa travessia
  completa da árvore para intercepção em profundidade — headings dentro de
  sequences são intercetados.
- **GO — mais tipos em `NodeKind`:** com o motor estável, Passo 69 adiciona
  `Raw`, `Equation`, `ListItem` ao enum e os braços correspondentes.
- **NO-GO — `in_show_transform` insuficiente:** se o teste anti-recursão revelar
  stack overflow em casos reais; Passo 69 implementa mecanismo de marcação de
  nós mais robusto.
