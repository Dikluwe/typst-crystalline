# Passo 70 — Travessia Única e Guards Anti-Recursão (DEBT-20, DEBT-21, DEBT-23)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/show.rs` — `ShowRule`, `Selector`, `NodeKind`, para
  adicionar `RuleId`.
- `01_core/src/rules/eval.rs` — `EvalContext`, `apply_show_rules`,
  `intercept_content`, e o campo `in_show_transform: bool` a remover.
- `01_core/src/entities/content.rs` — `map_content` do Passo 69, para confirmar
  que bottom-up está correcto antes de depender dele.

Pré-condição: `cargo test` — 690 L1 + 125 L3, zero violations.
DEBT-20, DEBT-21, DEBT-23 registados.

---

## Contexto

### Porque `Content::Guarded` não funciona

A tentação natural para prevenir recursão é embrulhar o resultado da
transformação num nó marcador. O problema é o timing:

1. `#show heading: it => [= Prefixo ] + it.body` é accionado.
2. `apply_show_rules` invoca `apply_func`.
3. Dentro da closure, o utilizador cria um novo `Heading`.
4. `intercept_content` é chamado imediatamente para esse `Heading` recém-nascido
   (intercepção eager do Passo 68).
5. `apply_show_rules` é chamado de novo. O novo `Heading` ainda não tem wrapper
   — o wrapper só seria aplicado quando `apply_func` retornasse.
6. A regra faz match novamente. Stack overflow.

O wrapper no AST chega sempre tarde demais porque a intercepção é eager.

### A solução: pilha de guards no contexto de avaliação

O guard tem de viver onde a intercepção vive — no `EvalContext`. Em vez de um
booleano cego (`in_show_transform`) que bloqueia todas as regras, usa-se uma
pilha de IDs das regras actualmente em execução. Quando uma regra começa a
avaliar a sua closure, o seu ID é empurrado para a pilha. Qualquer nó gerado
dentro dessa closure passa pelo `intercept_content`, que consulta a pilha e
salta esta regra — mas pode activar outras. Quando a closure termina, o ID
é removido da pilha.

Isto resolve DEBT-20 (composição de regras) e DEBT-23 (travessia única) sem
introduzir qualquer nó novo no AST. O AST mantém-se limpo.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar todos os locais que lêem ou escrevem in_show_transform
grep -rn "in_show_transform" 01_core/src/

# 2. Confirmar a estrutura actual de ShowRule (sem id ainda)
grep -n "pub struct ShowRule" 01_core/src/entities/show.rs -A 5

# 3. Confirmar que active_guards e next_rule_id ainda não existem
grep -rn "next_rule_id\|active_guards\|RuleId" 01_core/src/

# 4. Confirmar a assinatura de intercept_content e apply_show_rules
grep -n "fn intercept_content\|fn apply_show_rules" 01_core/src/rules/eval.rs

# 5. Confirmar a assinatura de apply_func (ctx é passado por &mut)
grep -n "fn apply_func" 01_core/src/rules/eval.rs | head -5
```

Reportar o output completo antes de continuar. O diagnóstico 1 é crítico:
todos os locais que tocam `in_show_transform` precisam de ser removidos de
forma coordenada — uma remoção parcial não compila.

---

## Tarefa 0 — Actualizar DEBT.md

Antes de qualquer código, adicionar em `01_core/DEBT.md`:

```markdown
### DEBT-20 — Guard anti-recursão booleano global — ENCERRADO (Passo 70) ✓
Substituído por active_guards: Vec<RuleId> no EvalContext. Pilha de regras
activas permite composição de regras sem stack overflow.

### DEBT-21 — Resolução de NodeKind por string — MITIGADO (Passo 70)
Func::name() continua a ser usado. Aliasing não é detectado. Resolução
completa por ponteiro adiada (requer Rust >= 1.85 para fn_addr_eq estável).

### DEBT-23 — Travessia múltipla O(R×N) — ENCERRADO (Passo 70) ✓
apply_show_rules chama map_content uma única vez para todas as regras NodeKind.
```

---

## Tarefa 1 — `RuleId` e `active_guards` no `EvalContext` (L1)

Em `01_core/src/entities/show.rs`, adicionar o tipo e o campo `id`:

```rust
/// Identificador único de uma ShowRule dentro de uma sessão de avaliação.
/// Usado pela pilha active_guards no EvalContext para prevenir recursão.
pub type RuleId = u64;

#[derive(Debug, Clone)]
pub struct ShowRule {
    pub id:        RuleId,
    pub selector:  Selector,
    pub transform: Value,
}
```

Em `EvalContext`, substituir `in_show_transform` por `active_guards` e
adicionar `next_rule_id`:

```rust
pub(crate) struct EvalContext<'w> {
    pub world:         Tracked<'w, dyn TrackedWorld + 'w>,
    pub depth:         usize,
    pub show_rules:    Vec<ShowRule>,
    pub next_rule_id:  RuleId,
    /// Pilha dos IDs das regras actualmente em execução.
    /// Substituí in_show_transform (DEBT-20): em vez de bloquear todas as
    /// regras, bloqueia apenas a regra cujo ID já está na pilha, permitindo
    /// que outras regras continuem a actuar (composição).
    pub active_guards: Vec<RuleId>,
}

impl<'w> EvalContext<'w> {
    pub fn new(world: Tracked<'w, dyn TrackedWorld + 'w>) -> Self {
        Self {
            world,
            depth:         0,
            show_rules:    Vec::new(),
            next_rule_id:  1,
            active_guards: Vec::new(),
        }
    }
}
```

No braço `Expr::Show` em `eval.rs`, atribuir o ID ao criar a regra:

```rust
let rule_id = ctx.next_rule_id;
ctx.next_rule_id += 1;
ctx.show_rules.push(ShowRule { id: rule_id, selector, transform });
```

Após esta tarefa, remover todas as linhas que lêem ou escrevem
`ctx.in_show_transform` (listadas pelo diagnóstico 1). O compilador
identificará as restantes.

---

## Tarefa 2 — `apply_show_rules` com travessia única e pilha de guards (L1)

```rust
pub(crate) fn apply_show_rules(
    mut content: Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Content> {
    if rules.is_empty() {
        return Ok(content);
    }

    // Separar regras por tipo antes de qualquer travessia.
    let node_rules: Vec<_> = rules.iter()
        .filter(|r| matches!(r.selector, Selector::NodeKind(_)))
        .cloned()
        .collect();

    // --- Travessia única para todas as regras NodeKind (DEBT-23 encerrado) ---
    if !node_rules.is_empty() {
        let mut apply_node_rules = |node: &Content| -> SourceResult<Option<Content>> {
            for rule in &node_rules {
                // DEBT-20: saltar regras actualmente em execução.
                // Se este nó foi gerado durante a avaliação desta regra,
                // active_guards já contém o seu id.
                if ctx.active_guards.contains(&rule.id) {
                    continue;
                }

                let kind = match &rule.selector {
                    Selector::NodeKind(k) => k,
                    Selector::Text(_) => continue,
                };

                let is_match = match (node, kind) {
                    (Content::Heading { .. }, NodeKind::Heading)   => true,
                    (Content::Figure { .. },  NodeKind::Figure)    => true,
                    (Content::Strong(_),      NodeKind::Strong)    => true,
                    (Content::Emph(_),        NodeKind::Emph)      => true,
                    (Content::Raw(_),         NodeKind::Raw)       => true,
                    (Content::Equation { .. },NodeKind::Equation)  => true,
                    (Content::ListItem(_),    NodeKind::ListItem)  => true,
                    _ => false,
                };

                if !is_match {
                    continue;
                }

                match &rule.transform {
                    Value::Func(func) => {
                        let args = Args::positional(vec![
                            Value::Content(node.clone())
                        ]);

                        // Proteger esta regra ANTES de invocar apply_func.
                        // Qualquer nó gerado dentro da closure passará pelo
                        // intercept_content, que consultará active_guards e
                        // saltará esta regra — mas não outras (composição).
                        ctx.active_guards.push(rule.id);

                        // Capturar o resultado SEM usar `?` aqui.
                        // Se usarmos `?` antes do pop, um erro deixaria a pilha
                        // corrompida para o resto da sessão de avaliação.
                        let result = apply_func(func.clone(), args, ctx);

                        // Remover a protecção INCONDICIONALMENTE — antes de
                        // propagar qualquer erro.
                        ctx.active_guards.pop();

                        let result_content = match result? {
                            Value::Content(c) => c,
                            Value::Str(s) => Content::text(s.to_string()),
                            other => return Err(vec![SourceDiagnostic::error(
                                Span::detached(),
                                format!(
                                    "show rule deve retornar Content ou String, \
                                     recebeu {}.",
                                    other.type_name()
                                ),
                            )]),
                        };

                        // Retorno imediato após a primeira regra que fez match.
                        // O result_content já foi processado pelo intercept_content
                        // durante apply_func — regras com outros IDs já actuaram
                        // sobre os nós gerados dentro da closure.
                        return Ok(Some(result_content));
                    },
                    Value::Content(c) => {
                        return Ok(Some(c.clone()));
                    },
                    other => return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "transformação de show rule inválida: {}.",
                            other.type_name()
                        ),
                    )]),
                }
            }

            Ok(None)
        };

        content = content.map_content(&mut apply_node_rules)?;
    }

    // --- Regras de texto: map_text, sem dupla travessia ---
    for rule in rules {
        if let Selector::Text(pattern) = &rule.selector {
            if let Value::Str(s) = &rule.transform {
                let replacement = s.to_string();
                let mut do_replace =
                    |text: &str| text.replace(pattern.as_str(), &replacement);
                content = content.map_text(&mut do_replace);
            }
        }
    }

    Ok(content)
}
```

---

## Tarefa 3 — `intercept_content` simplificado (L1)

```rust
pub(crate) fn intercept_content(
    content: Content,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Content> {
    if ctx.show_rules.is_empty() {
        return Ok(content);
    }
    // Clone necessário para satisfazer o borrow checker: apply_show_rules
    // precisa de &mut ctx para apply_func, enquanto ctx.show_rules seria
    // emprestado imutavelmente ao mesmo tempo. DEBT-22: O(R) por nó.
    let rules = ctx.show_rules.clone();
    apply_show_rules(content, &rules, ctx)
}
```

---

## Tarefa 4 — DEBT-21: Mitigação da Identidade por Nome (L1)

```rust
Value::Func(ref f) => {
    match f.name() {
        Some(name) => {
            let kind = match name {
                "heading"   => NodeKind::Heading,
                "figure"    => NodeKind::Figure,
                "strong"    => NodeKind::Strong,
                "emph"      => NodeKind::Emph,
                "raw"       => NodeKind::Raw,
                "equation"  => NodeKind::Equation,
                "list_item" => NodeKind::ListItem,
                other => return Err(vec![SourceDiagnostic::error(
                    show_expr.selector().span(),
                    format!(
                        "função '{}' não é um tipo de nó suportado como selector. \
                         Tipos suportados: heading, figure, strong, emph, raw, \
                         equation, list_item. (DEBT-21: aliasing não detectado)",
                        other
                    ),
                )]),
            };
            Selector::NodeKind(kind)
        },
        None => return Err(vec![SourceDiagnostic::error(
            show_expr.selector().span(),
            "o selector de show rule deve ser uma função nativa nomeada ou uma \
             string literal. Closures anónimas não são suportadas.".to_string(),
        )]),
    }
},
```

---

## Tarefa 5 — Testes

```rust
#[test]
fn show_rule_composicao_sem_loop() {
    // DEBT-20 encerrado: a regra transforma heading em heading.
    // Durante apply_func, rule.id está em active_guards. O novo Heading
    // gerado passa pelo intercept_content mas esta regra é saltada.
    let world = MockWorld::new(
        "#show heading: it => [Prefixo: ] + it.body\n\n= Título"
    );
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert!(text.contains("Prefixo: Título"),
        "Show rule deve aplicar-se uma vez: {:?}", text);
    assert_eq!(text.matches("Prefixo:").count(), 1,
        "A regra não deve ter sido reaplicada: {:?}", text);
}

#[test]
fn show_rule_encadeamento_duas_regras() {
    // Regra 1 (id=1): heading → strong. Durante apply_func, id=1 está em
    // active_guards. O Strong gerado passa pelo intercept_content.
    // Regra 2 (id=2): strong → emph. id=2 não está em active_guards → aplica-se.
    let world = MockWorld::new(
        "#show heading: strong\n#show strong: emph\n\n= Título"
    );
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert!(text.contains("Título"),
        "Encadeamento deve produzir conteúdo: {:?}", text);
}

#[test]
fn show_rule_active_guards_limpos_apos_erro() {
    // Se apply_func retornar Err, o pop ocorre antes de propagar o erro.
    // Após o erro, active_guards deve estar vazio — pilha não corrompida.
    let world = MockWorld::new(
        "#show heading: it => true\n\n= Título"
    );
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err(), "Retornar bool de show rule deve gerar Err");
    // Se a pilha ficasse corrompida, testes subsequentes falhariam por razões
    // aparentemente não relacionadas. A ausência de tais falhas confirma o pop.
}

#[test]
fn show_rule_multiplas_regras_nodekind_travessia_unica() {
    // DEBT-23: com múltiplas regras NodeKind, map_content é chamado uma vez.
    // Verificação comportamental: cada tipo é transformado correctamente.
    let world = MockWorld::new(
        "#show heading: upper\n#show strong: lower\n\n= Titulo *Forte*"
    );
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert!(text.contains("TITULO"),
        "Heading deve ser transformado para maiúsculas: {:?}", text);
    assert!(text.contains("forte"),
        "Strong deve ser transformado para minúsculas: {:?}", text);
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
- [ ] `RuleId` e `next_rule_id` em `ShowRule` e `EvalContext`.
- [ ] `in_show_transform` removido de `EvalContext` e de todos os pontos de
  uso — diagnóstico 1 lista todos; compilador confirma os restantes.
- [ ] `active_guards: Vec<RuleId>` em `EvalContext`, inicializado vazio.
- [ ] `apply_show_rules` chama `map_content` **uma única vez** para todas as
  regras `NodeKind`.
- [ ] `push(rule.id)` antes de `apply_func` e `pop()` **antes do `?`** —
  pop incondicional, independentemente de erro.
- [ ] `intercept_content` sem referências a `in_show_transform`.
- [ ] `Content::Guarded` **não existe** — o AST não tem nós invisíveis.
- [ ] Teste `show_rule_composicao_sem_loop` passa sem stack overflow.
- [ ] Teste `show_rule_encadeamento_duas_regras` passa.
- [ ] Teste `show_rule_active_guards_limpos_apos_erro` passa.
- [ ] DEBT-20 marcado como **encerrado** em `01_core/DEBT.md`.
- [ ] DEBT-21 marcado como **mitigado** em `01_core/DEBT.md`.
- [ ] DEBT-23 marcado como **encerrado** em `01_core/DEBT.md`.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Lista completa de locais onde `in_show_transform` era usado — confirmar que
  todos foram removidos.
- Se `active_guards` causou problemas de borrow checker não antecipados dentro
  da closure passada a `map_content`.

**Da implementação:**
- Se o teste `show_rule_composicao_sem_loop` passou à primeira ou foi necessário
  ajustar onde o `push`/`pop` ocorre.
- Se o teste `show_rule_encadeamento_duas_regras` confirmou que regras com IDs
  diferentes se compõem correctamente.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 71:**
- **GO — imagens e assets:** com DEBT-20/23 encerrados, Passo 71 pode
  implementar `Content::Image` e exportação para PDF.
- **GO — DEBT-14/15 (`#set figure`):** stdlib de figuras pode ser expandida.
- **NO-GO — pilha corrompida:** se o teste de erro revelar que o pop não ocorre
  antes de propagar o erro; Passo 71 depura o push/pop antes de avançar.
