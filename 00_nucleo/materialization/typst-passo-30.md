# Passo 30 — DEBT-1: StyleChain

**Pré-condições**:
- Passo 29 concluído: 394 testes, zero violations
- `DEBT-1` visível em `DEBT.md`
- `Content::Sequence` já usa `Arc<[Content]>` (ADR-0026, Passo 26) ✓
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
# Confirmar pré-condição ADR-0026
grep -n "Sequence" 01_core/src/entities/content.rs
# Deve mostrar: Sequence(Arc<[Content]>)

# Confirmar estado actual de TextStyle
grep -n "pub struct TextStyle\|bold\|italic\|size" \
  01_core/src/entities/layout_types.rs

# Confirmar raw pointer no ImportGuard está documentado
grep -n "mut Vec\|raw\|invariant\|valid\|safety" \
  01_core/src/rules/eval_context.rs | head -10

# Confirmar assinatura de eval_for_test_with_limits
grep -n "eval_for_test_with_limits" 01_core/src/rules/eval.rs
```

**Parar se qualquer pré-condição falhar.**

---

## Contexto

DEBT-1 regista que `TextStyle` é uma struct plana:

```rust
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub size: f64,
}
```

O Typst original usa `StyleChain` — uma lista ligada imutável de deltas,
onde cada nó contém apenas as propriedades que diferem do pai. Custo de
clone: O(1) (só clonar o `Arc` do nó de topo). Custo de leitura: O(N)
percorrendo a cadeia até encontrar o primeiro delta que define a propriedade.

A struct plana tem custo de clone O(K) onde K é o número de propriedades,
e não suporta `#set` rules porque não há forma de representar "herdar do
pai excepto bold=true".

**O que este passo faz**: implementar `StyleChain` mínima — suficiente
para suportar `#set text(bold: true)` e herança em blocos aninhados.
Não é paridade total com o original (que tem centenas de propriedades
e um sistema de Show rules). É a fundação que desbloqueia DEBT-1.

**O que este passo não faz**:
- `#show` rules (bloqueado por StyleChain, mas é trabalho separado)
- Propriedades além de `bold`, `italic`, `size` (adicionadas progressivamente)
- Paridade total com o sistema de styles do original

---

## Tarefa 1 — Diagnóstico

```bash
# Ver TextStyle actual em detalhe
cat 01_core/src/entities/layout_types.rs | grep -A 20 "pub struct TextStyle"

# Ver a definição actual de Content::Text
grep -n "Text\|Content" 01_core/src/entities/content.rs | head -20

# Contar quantos sítios constroem Content::Text — crítico para Tarefa 3c
grep -rn "Content::Text(" \
  01_core/src/ 03_infra/src/ 2>/dev/null

# Ver como TextStyle é usado no layout
grep -n "TextStyle\|text_style\|\.bold\|\.italic\|\.size" \
  01_core/src/rules/layout.rs | head -30

# Ver como TextStyle é usado no export
grep -n "TextStyle\|text_style\|\.bold\|\.italic\|\.size\|Content::Text" \
  03_infra/src/export.rs | head -30

# Ver como eval produz/usa TextStyle actualmente
grep -n "TextStyle\|text_style\|Content::Text" \
  01_core/src/rules/eval.rs | head -20

# Ver se #set já tem algum tratamento em eval_expr
grep -n "SetRule\|set_rule\|Expr::Set" \
  01_core/src/rules/eval.rs | head -10
```

**Parar. Reportar output antes de qualquer código.**

Questões a responder:
1. `Content::Text` já transporta `TextStyle`, ou é só `EcoString`?
2. Quantos sítios constroem `Content::Text(...)` — listar todos.
3. `layout.rs` lê o estilo de `Content::Text` ou de uma variável externa?
4. `export.rs` acede a `Content::Text` directamente ou só via `FrameItem`?
5. `Expr::SetRule` já tem um arm em `eval_expr` ou está ausente?

---

## Tarefa 2 — Definir `StyleChain` em L1

Criar ou actualizar `01_core/src/entities/style_chain.rs`:

```rust
// @layer: L1
// @updated: 2026-XX-XX

use std::sync::Arc;

/// Um delta de estilo — apenas as propriedades que este nó define explicitamente.
/// Propriedades ausentes são herdadas do nó pai na cadeia.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleDelta {
    pub bold:   Option<bool>,
    pub italic: Option<bool>,
    pub size:   Option<f64>,   // em pontos
}

impl StyleDelta {
    pub const fn empty() -> Self {
        Self { bold: None, italic: None, size: None }
    }
}

/// Nó da cadeia de estilos.
#[derive(Debug, Clone)]
struct StyleNode {
    delta: StyleDelta,
    parent: Option<Arc<StyleNode>>,
}

/// Lista ligada imutável de deltas de estilo.
///
/// Clone é O(1) — apenas o Arc do nó de topo é clonado.
/// Leitura é O(N) onde N é a profundidade da cadeia (tipicamente < 10).
///
/// Equivalente simplificado de `StyleChain` do Typst original.
/// Suporta `#set text(bold: true)` e herança em blocos aninhados.
#[derive(Debug, Clone)]
pub struct StyleChain(Option<Arc<StyleNode>>);

impl StyleChain {
    /// Cadeia vazia — usa os valores por defeito do motor.
    pub const fn empty() -> Self {
        StyleChain(None)
    }

    /// Cria uma cadeia com os valores por defeito do motor.
    /// bold: false, italic: false, size: 11.0pt (paridade com Typst)
    pub fn default_chain() -> Self {
        let root = StyleNode {
            delta: StyleDelta {
                bold:   Some(false),
                italic: Some(false),
                size:   Some(11.0),
            },
            parent: None,
        };
        StyleChain(Some(Arc::new(root)))
    }

    /// Cria uma nova cadeia que herda desta e aplica `delta` por cima.
    /// Custo: O(1) — cria um novo Arc.
    pub fn push(&self, delta: StyleDelta) -> Self {
        let node = StyleNode {
            delta,
            parent: self.0.clone(),
        };
        StyleChain(Some(Arc::new(node)))
    }

    /// Resolve `bold` percorrendo a cadeia até ao primeiro delta que o define.
    pub fn bold(&self) -> bool {
        self.resolve_bool(|d| d.bold).unwrap_or(false)
    }

    /// Resolve `italic`.
    pub fn italic(&self) -> bool {
        self.resolve_bool(|d| d.italic).unwrap_or(false)
    }

    /// Resolve `size` em pontos.
    pub fn size(&self) -> f64 {
        self.resolve_f64(|d| d.size).unwrap_or(11.0)
    }

    fn resolve_bool(&self, f: impl Fn(&StyleDelta) -> Option<bool>) -> Option<bool> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = f(&n.delta) {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    fn resolve_f64(&self, f: impl Fn(&StyleDelta) -> Option<f64>) -> Option<f64> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = f(&n.delta) {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }
}

/// Conversão para `TextStyle` plano — usado pelo layout e export actuais
/// enquanto a migração completa não está feita.
impl From<&StyleChain> for crate::entities::layout_types::TextStyle {
    fn from(chain: &StyleChain) -> Self {
        crate::entities::layout_types::TextStyle {
            bold:   chain.bold(),
            italic: chain.italic(),
            size:   chain.size(),
        }
    }
}
```

**Expor em `entities/mod.rs`**:
```rust
pub mod style_chain;
pub use style_chain::{StyleChain, StyleDelta};
```

---

## Tarefa 3 — Integrar `StyleChain` em `eval`

### 3a — Adicionar `style_chain` a `EvalContext`

```rust
// Em eval_context.rs
use crate::entities::style_chain::StyleChain;

pub struct EvalContext<'world, W: TrackedWorld> {
    // ... campos existentes ...
    /// Cadeia de estilos activa. Começa com os defaults do motor.
    /// Actualizada por `#set` rules durante eval.
    pub styles: StyleChain,
}

impl<'world, W: TrackedWorld> EvalContext<'world, W> {
    pub fn new(world: &'world W) -> Self {
        Self {
            // ... campos existentes ...
            styles: StyleChain::default_chain(),
        }
    }
}
```

### 3b — Tratar `Expr::SetRule` em `eval_expr`

Localizar o arm de `SetRule` (ou a sua ausência) em `eval_expr`.

O `SetRule` no AST tem: `target` (nome da função, ex: `"text"`) e
`args` (argumentos com nomes, ex: `bold: true`).

```rust
Expr::SetRule(set) => {
    // Extrair o target — deve ser "text" para suporte neste passo.
    // Outros targets (par, page, etc.) ficam para passos futuros.
    let target = set.target().to_untyped().text();

    if target != "text" {
        // Targets desconhecidos: ignorar silenciosamente por agora.
        // Registar como comportamento intencional (não é Err).
        return Ok(Value::None);
    }

    // Extrair argumentos nomeados do SetRule.
    // A estrutura exacta depende do AST — ver diagnóstico.
    let mut delta = StyleDelta::empty();

    for arg in set.args().items() {
        match arg {
            Arg::Named(named) => {
                let key = named.name().get();
                let val = eval_expr(ctx, scopes, named.expr())?;
                match key.as_str() {
                    "bold" => {
                        if let Value::Bool(b) = val {
                            delta.bold = Some(b);
                        }
                    }
                    "italic" => {
                        if let Value::Bool(b) = val {
                            delta.italic = Some(b);
                        }
                    }
                    "size" => {
                        if let Value::Length(l) = val {
                            delta.size = Some(l.abs.to_pt());
                        }
                    }
                    _ => { /* propriedade desconhecida — ignorar */ }
                }
            }
            _ => {}
        }
    }

    ctx.styles = ctx.styles.push(delta);
    Ok(Value::None)
}
```

**Nota**: `#set` é uma statement, não uma expressão que retorna valor.
`Value::None` é o retorno correcto.

**Nota sobre scoping**: no Typst original, `#set` dentro de um bloco
`{ }` afecta apenas esse bloco. Implementar scoping de styles agora
seria excessivo — `ctx.styles` é global ao eval neste passo. Registar
como DEBT menor em DEBT.md se necessário.

### 3c — Capturar o estilo em `Content::Text` no momento da produção

O `EvalContext` (e a `ctx.styles` que contém) existe apenas durante a
fase de eval. A fase de layout processa `Content` depois de eval ter
terminado — não tem acesso ao `EvalContext`. Portanto, o estilo activo
**tem de ser capturado dentro dos nós de `Content`** no momento em que
são produzidos, não passado separadamente ao layout.

**Passo 3c.1 — Adicionar `style` a `Content::Text`**

Em `entities/content.rs`, a variante `Text` precisa de transportar o
`TextStyle` activo no momento da sua criação:

```rust
// ANTES
pub enum Content {
    Empty,
    Text(EcoString),
    // ...
}

// DEPOIS
pub enum Content {
    Empty,
    Text(EcoString, TextStyle),   // estilo capturado em eval
    // ...
}
```

`TextStyle` já é `Clone` e é pequeno (3 campos). O custo de clonar
por nó de texto é aceitável nesta fase.

**Parar. Verificar quantos sítios em eval.rs e layout.rs constroem
`Content::Text` antes de alterar a definição. Reportar a lista antes
de alterar o enum.**

**Passo 3c.2 — Actualizar todos os construtores de `Content::Text`**

Em `eval.rs`, cada sítio que produz `Content::Text(s)` passa a ser:

```rust
Content::Text(s, TextStyle::from(&ctx.styles))
```

O estilo é resolvido no momento da produção — se `ctx.styles` mudar
depois (por um `#set` subsequente), os nós já criados não são afectados.
Isso é o comportamento correcto: o estilo no momento da escrita, não
o estilo no final do documento.

**Passo 3c.3 — Actualizar `layout.rs` para ler o estilo do nó**

Em `layout.rs`, o arm que processa `Content::Text` passa de:

```rust
// ANTES — usa TextStyle fixo passado externamente
Content::Text(s) => {
    // usava text_style do caller
}

// DEPOIS — lê o estilo do nó
Content::Text(s, style) => {
    // usa style directamente
    let pt_size = style.size;
    // ...
}
```

**Passo 3c.4 — Actualizar `export.rs` se necessário**

Se `export.rs` acede a `Content::Text` directamente (não via `FrameItem`),
aplicar o mesmo padrão. Verificar no diagnóstico.

**Nota sobre `Content::Space` e outros nós sem texto**: não precisam de
transportar estilo — o layout usa o estilo do `Text` adjacente ou os
defaults do motor.

---

## Tarefa 4 — Actualizar `DEBT.md`

```markdown
### DEBT-1 — StyleChain — PARCIALMENTE RESOLVIDO

**Resolvido no Passo 30**:
- `StyleChain` implementada em `entities/style_chain.rs`
- `StyleDelta { bold, italic, size }` como delta de herança
- `#set text(bold:, italic:, size:)` avaliado em `eval_expr`
- `EvalContext::styles: StyleChain` — cadeia activa durante eval
- `TextStyle::from(&StyleChain)` — bridge para layout/export actuais

**Divergência intencional**:
- `#set` é global ao eval (não tem scoping por bloco) — DEBT menor
- Apenas `text` como target suportado — outros targets ignorados silenciosamente
- StyleChain não integrada com `#show` rules (Passo futura)

**Pendente**:
- Scoping de `#set` por bloco `{ }`
- Propriedades adicionais (fill, font-family, weight numérico, etc.)
- `#show` rules
- Paridade total com o sistema de styles do original
```

---

## Tarefa 5 — Testes

```rust
// ── StyleChain — testes directos ─────────────────────────────────────────

#[test]
fn style_chain_defaults() {
    let chain = StyleChain::default_chain();
    assert_eq!(chain.bold(),   false);
    assert_eq!(chain.italic(), false);
    assert_eq!(chain.size(),   11.0);
}

#[test]
fn style_chain_push_herda() {
    let base = StyleChain::default_chain();
    // Só muda bold — italic e size devem ser herdados
    let child = base.push(StyleDelta { bold: Some(true), italic: None, size: None });
    assert_eq!(child.bold(),   true);
    assert_eq!(child.italic(), false);  // herdado
    assert_eq!(child.size(),   11.0);   // herdado
}

#[test]
fn style_chain_push_multiplos_niveis() {
    let base  = StyleChain::default_chain();
    let mid   = base.push(StyleDelta { bold: Some(true), italic: None, size: None });
    let child = mid.push(StyleDelta { bold: None, italic: None, size: Some(14.0) });
    // bold herdado de mid, size de child, italic do root
    assert_eq!(child.bold(),   true);
    assert_eq!(child.italic(), false);
    assert_eq!(child.size(),   14.0);
}

#[test]
fn style_chain_clone_e_o1() {
    // Clone não deve clonar a cadeia inteira — apenas o Arc do topo
    let base  = StyleChain::default_chain();
    let chain = base.push(StyleDelta { bold: Some(true), italic: None, size: None });
    let clone = chain.clone();
    assert_eq!(clone.bold(), true);
    // Não há forma directa de testar O(1) em Rust, mas o teste confirma
    // que o clone funciona correctamente
}

#[test]
fn text_style_from_style_chain() {
    let chain = StyleChain::default_chain()
        .push(StyleDelta { bold: Some(true), italic: None, size: Some(14.0) });
    let ts = TextStyle::from(&chain);
    assert_eq!(ts.bold,   true);
    assert_eq!(ts.italic, false);
    assert_eq!(ts.size,   14.0);
}

// ── #set text() via eval ──────────────────────────────────────────────────

#[test]
fn eval_set_text_bold() {
    let world = MockWorld::new("#set text(bold: true)");
    let src = world.source(world.main()).unwrap();
    // eval deve completar sem erro
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "set text bold falhou: {:?}", result);
}

#[test]
fn eval_set_text_size() {
    let world = MockWorld::new("#set text(size: 14pt)");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "set text size falhou: {:?}", result);
}

#[test]
fn eval_set_target_desconhecido_ignora() {
    // #set par() não está implementado — deve ser ignorado, não dar erro
    let world = MockWorld::new("#set par(leading: 1em)");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "set par desconhecido deve ser ignorado");
}

#[test]
fn eval_set_e_content_combinados() {
    // #set seguido de conteúdo — o eval deve processar ambos
    let world = MockWorld::new("#set text(bold: true)\nOlá mundo");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok());
}

#[test]
fn estilo_capturado_no_momento_da_producao() {
    // Texto antes de #set usa estilo anterior; texto depois usa estilo novo.
    // Verifica que o estilo é capturado em cada Content::Text no momento
    // da produção, não aplicado globalmente no final.
    let world = MockWorld::new(
        "antes\n#set text(bold: true)\ndepois"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    // Percorrer o Content do módulo e verificar que os dois nós Text
    // têm estilos diferentes.
    // A forma exacta de aceder ao Content depende da API de Module.
    // No mínimo, confirmar que eval não dá Err.
    // Inspecção mais detalhada via layout se necessário.
    let _ = m; // substituir por asserção quando API de Module permitir
}
```

---

## Verificação final

```bash
cargo test -p typst-core 2>&1 | tail -5
cargo test -p typst-infra 2>&1 | tail -5
cargo build 2>&1 | tail -5
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found

# Confirmar StyleChain em L1
grep -rn "std::fs\|std::net\|std::env\|std::time" \
  01_core/src/entities/style_chain.rs
# Deve retornar vazio — sem I/O de sistema

# Confirmar que TextStyle plano ainda existe (bridge mantida)
grep -n "pub struct TextStyle" 01_core/src/entities/layout_types.rs

# Confirmar styles em EvalContext
grep -n "styles.*StyleChain\|StyleChain.*styles" \
  01_core/src/rules/eval_context.rs

# Confirmar SetRule em eval_expr
grep -n "SetRule\|set_rule" 01_core/src/rules/eval.rs
```

Critérios de conclusão:
- `StyleChain` e `StyleDelta` em `entities/style_chain.rs`, camada L1 ✓
- `StyleChain::default_chain()`, `push()`, `bold()`, `italic()`, `size()` implementados ✓
- `TextStyle::from(&StyleChain)` implementado como bridge ✓
- `Content::Text` transporta `TextStyle` — `Content::Text(EcoString, TextStyle)` ✓
- Todos os construtores de `Content::Text` em eval passam `TextStyle::from(&ctx.styles)` ✓
- `layout.rs` lê o estilo do nó `Content::Text(_, style)` — não de variável externa ✓
- `Expr::SetRule` arm em `eval_expr` — `text` target processa `bold`, `italic`, `size`; outros targets ignorados ✓
- `style_chain_defaults`, `style_chain_push_herda`, `style_chain_push_multiplos_niveis` passam ✓
- `text_style_from_style_chain` passa ✓
- `eval_set_text_bold`, `eval_set_text_size`, `eval_set_target_desconhecido_ignora` passam ✓
- DEBT-1 marcado como parcialmente resolvido em `DEBT.md` ✓
- Zero violations ✓
- Testes não regridem (394 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- `Content::Text` já transportava `TextStyle` ou foi adicionado neste passo?
- Quantos construtores de `Content::Text` existiam e em que ficheiros?
- `Expr::SetRule` tinha arm em `eval_expr` ou estava ausente?
- `export.rs` acedia a `Content::Text` directamente — precisou de ser actualizado?

**Da implementação:**
- Se scoping de `#set` por bloco foi necessário ou pode ficar para depois
- Número final de testes e zero violations confirmado

**DEBT-1 parcialmente resolvido. Go para Passo 31 — DEBT-2: closures lazy capture.**
