# Passo 69 — Intercepção Transversal e Expansão do AST (DEBT-19)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — Para listar todas as variantes do enum
  `Content` e classificar cada uma como container ou terminal antes de escrever
  `map_content`. A variante `Content::Equation` requer atenção especial (ver
  Tarefa 1).
- `01_core/src/entities/show.rs` — `NodeKind`, `Selector`, `ShowRule` criados
  no Passo 68.
- `01_core/src/rules/eval.rs` — `apply_show_rules` e `intercept_content` do
  Passo 68, para entender o que o `map_content` vai substituir.
- `01_core/src/rules/stdlib.rs` — Para registar as funções sentinela novas.

Pré-condição: `cargo test` — 684 L1 + 125 L3, zero violations.
DEBT-19 registado. `apply_show_rules` interceta apenas o nó raiz (superficial).

---

## Contexto

O `apply_show_rules` do Passo 68 aplica cada regra apenas ao nó raiz do
`Content`. Um `Heading` dentro de um `Sequence` passa despercebido. Este passo
resolve o DEBT-19 implementando o motor `map_content`, que percorre a árvore
de baixo para cima (bottom-up) e aplica a transformação a cada nó. O
`apply_show_rules` é reescrito para usar `map_content` em vez de actuar apenas
na raiz.

### Regras estruturais para este passo

- **Separação de travessias:** regras de nó (`NodeKind`) usam `map_content`.
  Regras de texto (`Selector::Text`) usam `map_text`. Uma árvore nunca é
  percorrida duas vezes para a mesma regra.
- **Bottom-up:** os filhos são processados antes do nó pai. Um nó substituído
  pela transformação não é reavaliado no mesmo ciclo.
- **Sem wildcard:** o `match` de `map_content` deve listar todas as variantes
  explicitamente, pelo mesmo motivo do `map_text` — variantes novas não passam
  em silêncio.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Listar TODAS as variantes do enum Content com a sua estrutura
grep -n "^\s*[A-Z][A-Za-z]*" 01_core/src/entities/content.rs | head -60

# 2. Verificar a definição exacta de Content::Equation
grep -n "Equation" 01_core/src/entities/content.rs -A 3 | head -15
# Crítico: determinar se Equation armazena Box<Content> (→ container)
# ou apenas dados primitivos como String/bool (→ terminal).

# 3. Verificar MathAttach, MathRoot, MathDelimited — se contêm filhos
grep -n "MathAttach\|MathRoot\|MathDelimited" 01_core/src/entities/content.rs -A 5 | head -30

# 4. Confirmar a assinatura actual de apply_show_rules no Passo 68
grep -n "fn apply_show_rules\|fn intercept_content" 01_core/src/rules/eval.rs | head -5

# 5. Confirmar NodeKind actual (Passo 68 declarou Heading e Figure)
grep -n "enum NodeKind" 01_core/src/entities/show.rs -A 10 | head -15
```

Reportar o output completo antes de continuar. Os diagnósticos 2 e 3 são
críticos: a classificação de `Equation`, `MathAttach`, `MathRoot` e
`MathDelimited` como container ou terminal determina directamente a estrutura
do `match` em `map_content`.

---

## Tarefa 1 — Motor `map_content` (L1)

Em `01_core/src/entities/content.rs`, adicionar o método:

```rust
use crate::error::SourceResult;

impl Content {
    /// Percorre a árvore AST de baixo para cima (bottom-up), aplicando
    /// `transform` a cada nó após processar os seus filhos.
    ///
    /// Se `transform` retornar `Some(new_content)`, o nó é substituído e
    /// o novo nó não é reavaliado no mesmo ciclo (sem reentrada).
    /// Se retornar `None`, o nó processado (com filhos já transformados)
    /// é mantido.
    pub fn map_content<F>(&self, transform: &mut F) -> SourceResult<Self>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        // Passo 1: processar os filhos (bottom-up).
        let processed = match self {
            // --- Containers: propagar recursivamente ---
            // Listados explicitamente. Sem `_ =>` para este grupo.
            Content::Sequence(seq) => {
                let mut new_seq = Vec::with_capacity(seq.len());
                for c in seq {
                    new_seq.push(c.map_content(transform)?);
                }
                Content::Sequence(new_seq)
            },
            Content::Heading { level, body } => Content::Heading {
                level: *level,
                body: Box::new(body.map_content(transform)?),
            },
            Content::Strong(body) => Content::Strong(Box::new(body.map_content(transform)?)),
            Content::Emph(body)   => Content::Emph(Box::new(body.map_content(transform)?)),
            Content::Labelled { target, label } => Content::Labelled {
                target: Box::new(target.map_content(transform)?),
                label: label.clone(),
            },
            Content::Figure { body, caption } => Content::Figure {
                body: Box::new(body.map_content(transform)?),
                caption: caption.as_ref()
                    .map(|c| c.map_content(transform))
                    .transpose()?
                    .map(Box::new),
            },
            Content::ListItem(body) => Content::ListItem(Box::new(body.map_content(transform)?)),
            Content::EnumItem { number, body } => Content::EnumItem {
                number: *number,
                body: Box::new(body.map_content(transform)?),
            },
            Content::Link { url, body } => Content::Link {
                url: url.clone(),
                body: Box::new(body.map_content(transform)?),
            },
            // Nós matemáticos com filhos:
            Content::MathSequence(seq) => {
                let mut new_seq = Vec::with_capacity(seq.len());
                for c in seq {
                    new_seq.push(c.map_content(transform)?);
                }
                Content::MathSequence(new_seq)
            },
            Content::MathFrac(num, den) => Content::MathFrac(
                Box::new(num.map_content(transform)?),
                Box::new(den.map_content(transform)?),
            ),
            // ⚠️ ALERTA — verificar diagnósticos 2 e 3 antes de compilar:
            //
            // Content::Equation:
            //   Se for Equation(Box<Content>) → adicionar aqui com recursão.
            //   Se for Equation(String) ou Equation(bool) → mover para terminais.
            //
            // Content::MathAttach { base, top, bottom } → adicionar aqui se
            //   os campos forem Box<Content>. Exemplo:
            //   Content::MathAttach { base, top, bottom } => Content::MathAttach {
            //       base: Box::new(base.map_content(transform)?),
            //       top:  top.as_ref().map(|c| c.map_content(transform)).transpose()?.map(Box::new),
            //       bottom: bottom.as_ref().map(|c| c.map_content(transform)).transpose()?.map(Box::new),
            //   },
            //
            // Content::MathRoot { index, radicand } → idem se campos forem Box<Content>.
            // Content::MathDelimited { open, body, close } → idem.
            //
            // Se o compilador emitir `non_exhaustive_patterns`, a variante em
            // falta pertence aqui (se tiver filhos) ou no grupo de terminais.
            // Nunca usar `_ =>` para resolver este erro.

            // --- Terminais: clonar directamente ---
            // Listar TODAS as variantes sem filhos, conforme os diagnósticos 1–3.
            Content::Text(_)
            | Content::Space
            | Content::Empty
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
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            // | Content::Equation(_)  ← mover aqui se for terminal (ver alerta acima)
            // | Content::OutraVariante ← adicionar conforme diagnóstico 1
            => self.clone(),
        };

        // Passo 2: aplicar a transformação ao nó já processado.
        // Se `transform` retornar Some, o novo nó substitui; se None, mantém.
        match transform(&processed)? {
            Some(new_content) => Ok(new_content),
            None              => Ok(processed),
        }
    }
}
```

---

## Tarefa 2 — Expansão de `NodeKind` (L1)

Em `01_core/src/entities/show.rs`, expandir o enum:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Heading,
    Figure,
    Strong,
    Emph,
    Raw,
    Equation,
    ListItem,
}
```

Em `eval.rs`, no braço `Expr::Show`, expandir o `match f.name()`:

```rust
match f.name() {
    Some("heading")   => Selector::NodeKind(NodeKind::Heading),
    Some("figure")    => Selector::NodeKind(NodeKind::Figure),
    Some("strong")    => Selector::NodeKind(NodeKind::Strong),
    Some("emph")      => Selector::NodeKind(NodeKind::Emph),
    Some("raw")       => Selector::NodeKind(NodeKind::Raw),
    Some("equation")  => Selector::NodeKind(NodeKind::Equation),
    Some("list_item") => Selector::NodeKind(NodeKind::ListItem),
    Some(other) => return Err(vec![SourceDiagnostic::error(
        show_expr.selector().span(),
        format!(
            "o selector de show rule exige uma função nativa conhecida \
             (ex: heading, figure, strong, emph, raw, equation, list_item). \
             Recebeu: '{}' (DEBT-21: identificação por nome)",
            other
        ),
    )]),
    None => return Err(vec![SourceDiagnostic::error(
        show_expr.selector().span(),
        "o selector de show rule exige uma função nativa nomeada ou uma \
         string literal.".to_string(),
    )]),
}
```

---

## Tarefa 3 — `apply_show_rules` reescrito com `map_content` (L1)

Substituir a implementação superficial do Passo 68 pela versão transversal.
O ficheiro a editar é o mesmo onde `apply_show_rules` foi implementado no
Passo 68 (confirmar com o diagnóstico 4).

**DEBT-23 — Travessia múltipla O(R×N):** a implementação abaixo percorre a
árvore inteira uma vez por regra activa. Com R regras e N nós, o custo é
O(R×N). Num sistema maduro, `map_content` seria chamado uma única vez por
`apply_show_rules`, testando todas as regras aplicáveis dentro da closure para
cada nó. Isso exigiria que `map_content` recebesse uma lista de regras em vez
de uma closure genérica — refactorização adiada para o Passo 70. Registar em
`DEBT.md` antes de escrever qualquer código:

```markdown
### DEBT-23 — Travessia múltipla em apply_show_rules (Passo 69)
apply_show_rules percorre a árvore uma vez por ShowRule activa: O(R×N).
Resolução: map_content chamado uma única vez, com todas as regras testadas
por nó dentro da closure. Requer mudança de assinatura de map_content.
```

**Sobre `ctx` dentro da closure:** a closure `apply_rule` captura `ctx` por
`&mut` e é passada a `map_content` como `&mut F`. O borrow checker aceita
isto desde que `ctx` não esteja simultaneamente emprestado por outra
referência no mesmo âmbito — o que é verdade dentro do braço
`Selector::NodeKind`. Se o compilador recusar, extrair o corpo do braço para
uma função auxiliar:

```rust
fn apply_node_rule(
    content: Content,
    kind: &NodeKind,
    transform: &Value,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Content> { ... }
```

Não passar `ctx` como parâmetro de `map_content` — isso criaria dependência
de `entities` sobre `rules`, invertendo a hierarquia de módulos.

```rust
pub(crate) fn apply_show_rules(
    mut content: Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Content> {
    if rules.is_empty() {
        return Ok(content);
    }

    for rule in rules {
        match &rule.selector {
            Selector::NodeKind(kind) => {
                // Clonar os valores necessários para a closure antes de entrar
                // no map_content — o borrow checker não permite capturar `rule`
                // e `ctx` mutavelmente ao mesmo tempo.
                let kind = kind.clone();
                let transform_val = rule.transform.clone();

                let mut apply_rule = |node: &Content| -> SourceResult<Option<Content>> {
                    let is_match = match (node, &kind) {
                        (Content::Heading { .. }, NodeKind::Heading)  => true,
                        (Content::Figure { .. },  NodeKind::Figure)   => true,
                        (Content::Strong(_),      NodeKind::Strong)   => true,
                        (Content::Emph(_),        NodeKind::Emph)     => true,
                        (Content::Raw(_),         NodeKind::Raw)      => true,
                        // Ajustar o padrão de Equation conforme a estrutura
                        // confirmada no diagnóstico 2:
                        (Content::Equation(_),    NodeKind::Equation) => true,
                        (Content::ListItem(_),    NodeKind::ListItem) => true,
                        _ => false,
                    };

                    if !is_match {
                        return Ok(None);
                    }

                    match &transform_val {
                        Value::Func(func) => {
                            let args = Args::positional(vec![Value::Content(node.clone())]);
                            // O guard in_show_transform é restaurado
                            // incondicionalmente em apply_func (Passo 68).
                            // Não colocar `?` entre a activação e a restauração.
                            let result = apply_func(func.clone(), args, ctx)?;
                            match result {
                                Value::Content(c) => Ok(Some(c)),
                                Value::Str(s) => Ok(Some(Content::text(s.to_string()))),
                                other => Err(vec![SourceDiagnostic::error(
                                    Span::detached(),
                                    format!(
                                        "a show rule deve retornar Content ou String, \
                                         recebeu {}.",
                                        other.type_name()
                                    ),
                                )]),
                            }
                        },
                        Value::Content(c) => Ok(Some(c.clone())),
                        other => Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "transformação de show rule inválida: {}.",
                                other.type_name()
                            ),
                        )]),
                    }
                };

                content = content.map_content(&mut apply_rule)?;
            },

            Selector::Text(pattern) => {
                // Regras de texto continuam a usar map_text — sem dupla travessia.
                if let Value::Str(s) = &rule.transform {
                    let replacement = s.to_string();
                    let mut do_replace = |text: &str| text.replace(pattern.as_str(), &replacement);
                    content = content.map_text(&mut do_replace);
                }
            },
        }
    }

    Ok(content)
}
```

---

## Tarefa 4 — Funções sentinela na stdlib (L1)

Estas funções permitem que `#show strong: ...` resolva `Strong` pelo nome
da função, seguindo o mesmo mecanismo que `heading` e `figure` usam desde
o Passo 68. Confirmar com o diagnóstico 5 quais já existem antes de
adicionar duplicados.

```rust
fn native_strong(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    // Aceitar tanto Content ([texto]) como Str ("texto") — o utilizador pode
    // invocar #strong([corpo]) ou #strong("corpo"); ambas as formas são válidas.
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::Text(s.to_string().into()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("strong() espera content ou string, recebeu {}", other.type_name()),
        )]),
        None => Content::Empty,
    };
    Ok(Value::Content(Content::Strong(Box::new(body))))
}

fn native_emph(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::Text(s.to_string().into()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("emph() espera content ou string, recebeu {}", other.type_name()),
        )]),
        None => Content::Empty,
    };
    Ok(Value::Content(Content::Emph(Box::new(body))))
}

fn native_raw(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    // raw() aceita apenas string — não faz sentido semântico aceitar Content aqui.
    let text = match args.items.first() {
        Some(Value::Str(s)) => s.to_string(),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("raw() espera string, recebeu {}", other.type_name()),
        )]),
        None => String::new(),
    };
    Ok(Value::Content(Content::Raw(text)))
}
```

Registar no scope:

```rust
scope.define("strong",    Value::Func(Func::native("strong",    native_strong)));
scope.define("emph",      Value::Func(Func::native("emph",      native_emph)));
scope.define("raw",       Value::Func(Func::native("raw",       native_raw)));
// "equation" e "list_item" adicionados quando os tipos forem usados em testes reais.
```

---

## Tarefa 5 — Testes

### Testes L1 — `map_content`

```rust
#[test]
fn map_content_substitui_heading_em_sequence() {
    // Valida que um Heading dentro de um Sequence é intercetado —
    // o problema central do DEBT-19.
    let content = Content::Sequence(vec![
        Content::text("Antes"),
        Content::heading(1, Content::text("Titulo")),
        Content::text("Depois"),
    ]);

    let result = content.map_content(&mut |node| {
        if matches!(node, Content::Heading { .. }) {
            Ok(Some(Content::text("SUBSTITUIDO")))
        } else {
            Ok(None)
        }
    }).unwrap();

    assert_eq!(result.plain_text(), "AntesSUBSTITUIDODepois");
}

#[test]
fn map_content_bottom_up_pai_vê_filhos_transformados() {
    // O pai recebe os filhos já transformados — confirmar a ordem bottom-up.
    let content = Content::Strong(Box::new(Content::text("original")));

    let result = content.map_content(&mut |node| {
        match node {
            // Texto transformado para maiúsculas
            Content::Text(s) => Ok(Some(Content::text(s.to_uppercase()))),
            // Strong recebe o filho já transformado — não "original"
            Content::Strong(body) => {
                let text = body.plain_text();
                assert_eq!(text, "ORIGINAL",
                    "Strong deve receber filho já transformado: {:?}", text);
                Ok(None)
            },
            _ => Ok(None),
        }
    }).unwrap();

    assert_eq!(result.plain_text(), "ORIGINAL");
}

#[test]
fn map_content_nao_reavaliar_no_substituido() {
    // Um nó substituído não deve ser passado à closure novamente.
    let content = Content::heading(1, Content::text("X"));
    let mut call_count = 0usize;

    content.map_content(&mut |node| {
        if matches!(node, Content::Heading { .. }) {
            call_count += 1;
            Ok(Some(Content::text("substituido")))
        } else {
            Ok(None)
        }
    }).unwrap();

    assert_eq!(call_count, 1, "Heading deve ser processado exactamente uma vez");
}
```

### Testes L1 — show rules transversais

```rust
#[test]
fn show_rule_map_content_transversal() {
    // DEBT-19: heading dentro de sequence deve ser intercetado.
    // No Passo 68, este teste falharia — a intercepção era superficial.
    let world = MockWorld::new("#show heading: it => upper(it.body)\n\n= Titulo Escondido");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert!(text.contains("TITULO ESCONDIDO"),
        "map_content não processou nós aninhados: {:?}", text);
}

#[test]
fn show_rule_multiplos_tipos_independentes() {
    // Regras para Strong e Emph aplicam-se independentemente.
    let world = MockWorld::new("#show strong: upper\n#show emph: lower\n*A* e _B_");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert!(text.contains("A") && text.contains("b"),
        "Regras separadas para Strong e Emph devem aplicar-se independentemente: {:?}", text);
}

#[test]
fn show_rule_texto_usa_map_text_nao_map_content() {
    // Regras Selector::Text continuam a usar map_text — sem dupla travessia.
    let world = MockWorld::new("#show \"a\": \"x\"\naaa");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert_eq!(text.trim(), "xxx",
        "Selector::Text deve substituir todas as ocorrências: {:?}", text);
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
- [ ] `map_content` implementado em `content.rs` com dois braços explícitos:
  containers (recursão bottom-up) e terminais (clone directo). Sem `_ =>`.
- [ ] `Content::Equation` classificada correctamente como container ou terminal
  após o diagnóstico 2. Decisão documentada em comentário junto ao braço.
- [ ] `MathAttach`, `MathRoot`, `MathDelimited` classificados com base no
  diagnóstico 3. Se forem containers, têm recursão explícita.
- [ ] `NodeKind` expandido com `Strong`, `Emph`, `Raw`, `Equation`, `ListItem`.
- [ ] `apply_show_rules` reescrito para usar `map_content` em seletores de nó
  e `map_text` em seletores de texto — nunca dupla travessia.
- [ ] `apply_show_rules` usa `match` na raiz do ciclo: regras `Text` nunca
  invocam `map_content`.
- [ ] `Content::text()` usado como construtor em `apply_show_rules` — nunca
  `Content::Text()` directamente.
- [ ] DEBT-23 registado em `00_nucleo/DEBT.md` antes de qualquer código.
- [ ] Funções sentinela `strong`, `emph`, `raw` registadas na stdlib.
- [ ] Testes `map_content_substitui_heading_em_sequence` e
  `show_rule_map_content_transversal` passam.
- [ ] DEBT-19 marcado como **encerrado** em `00_nucleo/DEBT.md`.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Estrutura de `Content::Equation` — container ou terminal, e porquê.
- Estrutura de `MathAttach`, `MathRoot`, `MathDelimited` — mesma questão.
- Quais funções sentinela já existiam da Tarefa 4 do Passo 69 da especificação
  original versus quais foram adicionadas agora.

**Da implementação:**
- Se o compilador identificou variantes em falta no `match` de `map_content`
  — e quais foram adicionadas.
- Se o teste `map_content_bottom_up_pai_vê_filhos_transformados` passou à
  primeira ou revelou que a ordem de processamento era top-down.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 70:**
- **GO — DEBT-20 (composição de show rules):** com `map_content` estável,
  Passo 70 pode substituir o guard booleano `in_show_transform` por um
  mecanismo de marcação de nós que permita regras em cadeia sem stack overflow.
- **GO — DEBT-21 (resolução de NodeKind por ponteiro):** substituir
  identificação por nome de string por identidade de função nativa.
- **GO — DEBT-23 (travessia única):** Passo 70 refactoriza `apply_show_rules`
  para chamar `map_content` uma única vez, testando todas as regras por nó
  dentro da closure. Requer mudança de assinatura de `map_content`.
- **NO-GO — `map_content` incompleto:** se variantes de container não cobertas
  causarem regressões; Passo 70 completa a cobertura antes de avançar.
