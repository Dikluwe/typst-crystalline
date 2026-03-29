# Passo 18 — Content mínimo e início do pipeline visual

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0016-adiamento-eval-typst-library.md`
- `lab/typst-original/crates/typst-library/src/foundations/content.rs` (ou equivalente)
- `lab/typst-original/crates/typst-layout/src/lib.rs`

Pré-condição: `cargo test` — 295 testes (273 L1 + 22 L3), zero violations.

Este passo é o início da fase visual do compilador. O motor lógico
(eval()) está completo. O objectivo agora é fazer texto fluir por
todas as camadas: parse → eval → Content → layout (stub).

**Invariante de L1**: `Content` é puramente declarativo. Não desenha,
não mede, não renderiza. Apenas representa a estrutura do documento.
Qualquer operação que precise de métricas de fonte ou I/O pertence a L3.

**Autorização de divergência**: se a implementação original de `Content`
usar metaprogramação pesada (vtable, proc macros, `NativeElement` trait)
de forma indissociável de tipos não migrados, o cristalino diverge
intencionalmente. Um enum linear com variantes declarativas é
arquitecturalmente superior a replicar a metaprogramação do original
neste passo.

---

## Tarefa 1 — Diagnóstico de Content

**Parar aqui. Este diagnóstico determina o âmbito do passo.**

```bash
# Localizar Content
find lab/typst-original/crates/typst-library/src \
  -name "content*.rs" -o -name "content" -type d | sort

# Estrutura interna — enum, struct com vtable, ou outra?
grep -n "^pub struct Content\|^pub enum Content\|^struct Content\b" \
  lab/typst-original/crates/typst-library/src/foundations/content.rs 2>/dev/null \
  || grep -rn "^pub struct Content\b" \
     lab/typst-original/crates/typst-library/src/ | head -5

grep -rA 25 "^pub struct Content\b" \
  lab/typst-original/crates/typst-library/src/ | head -30

# NativeElement — vtable dinâmica?
grep -rn "^pub trait NativeElement\|dyn NativeElement\|NativeElement\b" \
  lab/typst-original/crates/typst-library/src/ | head -10

# Styles — é campo de Content? É separável?
grep -n "styles\|Styles" \
  lab/typst-original/crates/typst-library/src/foundations/content.rs 2>/dev/null \
  | head -15

# Como texto simples ("Hello") se torna Content no original
grep -rn "TextElem\|text.*content\|Content::new\|elem.*text" \
  lab/typst-original/crates/typst-eval/src/ | head -20

# Arc interno em Content?
grep -n "Arc\|Rc\b" \
  lab/typst-original/crates/typst-library/src/foundations/content.rs 2>/dev/null \
  | head -10

# Assinatura de layout() e deps
grep -n "^pub fn layout\|^pub fn typeset\|^pub fn layout_document" \
  lab/typst-original/crates/typst-layout/src/lib.rs 2>/dev/null | head -5
grep -n "^pub fn layout" -A 10 \
  lab/typst-original/crates/typst-layout/src/lib.rs 2>/dev/null | head -15
```

### Questões críticas a responder

1. **Vtable vs enum**: Content usa `Box<dyn NativeElement>` / proc macro
   gerada, ou é um enum de variantes concretas?
2. **Styles em Content**: `Content` contém `Styles` directamente em cada
   nó, ou Styles é uma camada separada aplicada em layout()?
3. **Arc**: Content usa Arc internamente? Importante para clone O(1) ao
   montar sequências.
4. **TextElem**: qual a estrutura mínima para criar texto? É uma struct
   com campo `text: EcoString`?
5. **layout() deps**: layout() requer `Introspection`, `Engine`, ou
   apenas `Content` + `World`?

---

## Tarefa 2 — Decisão de representação de Content

### Opção B — Enum (preferida se estrutura permitir)

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Empty,
    Text(EcoString),           // TextElem mínimo
    Space,                     // SpaceElem
    Sequence(Vec<Content>),    // sequência de elementos
    // Variantes futuras (não implementar sem ADR):
    // Styled(Box<Content>, Styles),   // precisa Styles real
    // Heading { level: u8, body: Box<Content> },
    // Strong(Box<Content>),
    // Emph(Box<Content>),
    // Raw { text: EcoString, lang: Option<EcoString> },
    // Elem(Arc<dyn NativeElement>),   // vtable — Passo 20+
}
```

**Usar Opção B se**: Styles não é campo obrigatório de cada nó de
Content (pode ser aplicado em layout()), e o número de variantes
necessárias neste passo é pequeno.

### Opção C — Struct com Arc (se vtable necessária)

Se o diagnóstico confirmar que Content requer vtable para funcionar
com layout(), criar um tipo minimalista:

```rust
pub struct Content(Arc<ContentInner>);

enum ContentInner {
    Empty,
    Text(EcoString),
    Space,
    Sequence(Vec<Content>),
    // Elem(Arc<dyn NativeElement>),  // adiado
}
```

`Arc<ContentInner>` dá clone O(1) para sequências grandes — importante
quando layout() percorre a árvore repetidamente.

### Opção D — Content diverge intencionalmente do original

Se o original usa metaprogramação (proc macros `#[elem]`, `NativeElement`
trait gerada) de forma que replicar seria trazer toda a complexidade
de `typst_macros` para L1:

→ Ignorar a representação interna do original.
→ Usar Opção B (enum) independentemente do que o original faz.
→ Documentar divergência em ADR-0026.

**Esta autorização é explícita e intencional.** O objectivo é um
compilador cristalino funcional, não uma réplica da metaprogramação
do original.

---

## Tarefa 3 — Implementar Content mínimo

**Após confirmar a opção com o diagnóstico.**

```rust
// 01_core/src/entities/content.rs
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-27

use ecow::EcoString;  // ADR-0024 — clone O(1) para texto

impl Content {
    pub fn text(s: impl Into<EcoString>) -> Self {
        Self::Text(s.into())
    }

    pub fn empty() -> Self { Self::Empty }

    pub fn sequence(parts: Vec<Content>) -> Self {
        match parts.len() {
            0 => Self::Empty,
            1 => parts.into_iter().next().unwrap(),
            _ => Self::Sequence(parts),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
            || matches!(self, Self::Sequence(v) if v.is_empty())
    }

    /// Extrai texto plano recursivamente — para verificação em testes.
    pub fn plain_text(&self) -> String {
        match self {
            Self::Empty       => String::new(),
            Self::Text(s)     => s.to_string(),
            Self::Space       => " ".to_string(),
            Self::Sequence(v) => v.iter().map(|c| c.plain_text()).collect(),
        }
    }
}
```

### Adicionar a entities/mod.rs

```rust
pub mod content;
```

### Adicionar Value::Content

```rust
// Em value.rs:
Content(crate::entities::content::Content),

// type_name:
Self::Content(_) => "content",

// From:
impl From<crate::entities::content::Content> for Value {
    fn from(c: crate::entities::content::Content) -> Self { Self::Content(c) }
}
```

### Prompt L0

**Criar**: `00_nucleo/prompts/entities/content.md`

Documentar:
- Content como tipo puramente declarativo (não desenha, não mede)
- Variantes iniciais e razão de cada uma
- Variantes futuras como comentários com bloqueante
- Divergência do original se Opção D foi escolhida
- Critérios de verificação

---

## Tarefa 4 — eval_markup produz Content

```rust
// Em rules/eval.rs — substituir eval_markup:

fn eval_markup(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    use SyntaxKind::*;
    let mut parts: Vec<Content> = Vec::new();

    for child in node.children() {
        if child.kind().is_trivia() { continue; }

        match child.kind() {
            Text => parts.push(Content::text(child.text().as_str())),
            Space | Parbreak => parts.push(Content::Space),
            _ => {
                if let Some(expr) = Expr::from_untyped(child) {
                    match eval_expr(expr, scopes, ctx)? {
                        Value::Content(c) => parts.push(c),
                        Value::Str(s)     => parts.push(Content::text(s.as_str())),
                        Value::None       => {}   // ignorar
                        _                 => {}   // outros tipos → ignorar em markup
                    }
                }
            }
        }
    }

    Ok(Value::Content(Content::sequence(parts)))
}
```

### Exposição do Content no Module

Para que `eval()` retorne o Content do documento (não apenas os
bindings), adicionar um campo a `Module`:

```rust
// Em entities/module.rs — expandir ModuleInner:
struct ModuleInner {
    name:    String,
    scope:   Scope,
    content: Option<Content>,  // conteúdo produzido por eval()
}

impl Module {
    pub fn content(&self) -> Option<&Content> {
        self.0.content.as_ref()
    }
}
```

E em `eval()`:

```rust
pub fn eval(...) -> SourceResult<Module> {
    let mut ctx = EvalContext::new(world);
    let global = make_stdlib();
    let mut scopes = Scopes::new(&global);
    scopes.push();

    let content_val = eval_markup(source.root(), &mut scopes, &mut ctx)?;
    let content = match content_val {
        Value::Content(c) => Some(c),
        _ => None,
    };

    let top = scopes.pop().unwrap_or_default();
    let mut module = Module::new(source.id().into_raw().get().to_string(), top);
    module.set_content(content);
    Ok(module)
}
```

---

## Tarefa 5 — layout() esqueleto em L1

```bash
# Confirmar assinatura real antes de implementar
grep -n "^pub fn layout_document" -A 8 \
  lab/typst-original/crates/typst-layout/src/lib.rs 2>/dev/null
```

```rust
// 01_core/src/rules/layout.rs
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-27

use crate::entities::content::Content;

/// Resultado de layout — placeholder para PagedDocument.
/// Substituído quando Frame e Page migrarem.
pub struct LayoutResult {
    pub pages: Vec<PageResult>,
}

pub struct PageResult {
    pub plain_text: String,  // apenas para verificação em testes
}

impl LayoutResult {
    pub fn plain_text(&self) -> String {
        self.pages.iter().map(|p| p.plain_text.as_str()).collect::<Vec<_>>().join("\n")
    }
}

/// Layout stub — não posiciona, apenas verifica se Content tem texto.
///
/// Implementação real requer métricas de fonte (FontBook) e será
/// atraída para L3 quando Frame e Page migrarem.
pub fn layout(content: &Content) -> LayoutResult {
    LayoutResult {
        pages: vec![PageResult {
            plain_text: content.plain_text(),
        }]
    }
}
```

---

## Tarefa 6 — Teste de Pipeline End-to-End

O "Hello World" visual que valida toda a cadeia:

```rust
#[test]
fn pipeline_completo_texto_simples() {
    // Parse → Eval → Content → Layout
    let world = MockWorld::new("Hello world");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();

    let content = module.content().expect("eval deve produzir Content");
    assert!(!content.is_empty());
    assert!(content.plain_text().contains("Hello"));
    assert!(content.plain_text().contains("world"));

    let result = layout(content);
    assert!(!result.plain_text().is_empty());
}

#[test]
fn pipeline_interpolacao_variavel() {
    // #let x = "Mundo"; Olá #x
    let world = MockWorld::new("#let x = \"Mundo\"\nOlá #x");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();

    let content = module.content().expect("Content deve existir");
    let text = content.plain_text();
    assert!(text.contains("Olá"), "texto estático deve estar presente: {:?}", text);
    assert!(text.contains("Mundo"), "variável interpolada deve estar presente: {:?}", text);
}

#[test]
fn pipeline_documento_vazio() {
    let world = MockWorld::new("");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    // Documento vazio → Content::Empty ou Sequence([])
    let content = module.content();
    if let Some(c) = content {
        assert!(c.is_empty());
    }
    // Sem pânico — pipeline robusto para input vazio
}

#[test]
fn pipeline_apenas_codigo_sem_markup() {
    // Apenas código → sem Content visível, mas sem erro
    let world = MockWorld::new("#let x = 42");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    assert_eq!(module.scope().get("x"), Some(&Value::Int(42)));
    // Content pode ser vazio — é correcto
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão:
- Diagnóstico de Content documentado (opção escolhida + razão) ✓
- `Content::text("hello").plain_text() == "hello"` ✓
- `Value::Content(...)` existe no enum ✓
- `eval("Hello world")` → `module.content()` contém texto ✓
- `eval("#let x = \"Mundo\"\nOlá #x")` → content tem "Olá" e "Mundo" ✓
- `layout(content).plain_text()` não é vazio para texto real ✓
- `layout()` compila em L1 sem deps de L3 ✓
- Zero violations ✓
- Testes não regridem (295 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico (crítico para o Passo 19):**
- Estrutura real de Content no original — vtable ou outra
- Se Styles é campo directo de cada nó ou camada separada
- Opção escolhida (B, C, ou D) e razão
- Se Content usa Arc internamente no original
- Assinatura real de layout() e deps não migradas

**Da implementação:**
- Se `Module::content` foi adicionado sem breaking changes
- Se `eval_markup` passou a produzir Content para texto puro
- Se interpolação `#x` dentro de markup funciona
- Número total de testes e zero violations

**Go/No-Go para o Passo 19:**
- **GO — layout() real**: pipeline end-to-end funciona com Content mínimo;
  Passo 19 implementa `Frame` e `Page` para produzir `PagedDocument`
  com posicionamento básico de texto
- **GO — Styles mínimo**: se Content útil requer Styles para show/set rules;
  Passo 19 migra `Styles` subset antes de avançar layout()
- **NO-GO — Content vtable indissociável**: vtable e tipos não migrados
  impedem Content mínimo; Passo 19 é análise arquitectural de como
  separar Content da sua metaprogramação
