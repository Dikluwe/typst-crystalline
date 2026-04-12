# Passo 57 — Sistema Base de Contadores (`CounterState`)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — Variante `Heading` e as novas variantes
  `Labelled` / `Ref` do Passo 56.
- `01_core/src/rules/layout.rs` — Estrutura actual do Layouter e o braço
  `Content::Heading`.
- `01_core/src/rules/eval.rs` — Verificar se `SetRule` para `heading` já é
  interceptado ou cai no wildcard.

Pré-condição: `cargo test` — 589 L1 + 111 L3 + 50 parity, zero violations.
A fundação de Labels e Refs do Passo 56 está operacional.

---

## Contexto

Um contador rastreia um valor numérico ao longo do documento. O caso mais
imediato é a numeração de secções. Quando o documento contém:

```typst
= Introdução
== Motivação
= Conclusão
```

a numeração esperada é `1`, `1.1`, `2`. O layouter actual desenha headings
sem qualquer numeração porque não tem estado entre nós.

Este passo não implementa o motor de introspecção completo do Typst (que
depende de `comemo` e de duas passagens de layout). Implementa a versão
cristalina: um `CounterState` que viaja com o Layouter numa única passagem,
suficiente para numeração sequencial sem referências para a frente.

### O que já existe no parser

O parser já reconhece a sintaxe relacionada com contadores:

- `#set heading(numbering: "1.1")` — gera um nó `SetRule` no AST.
- `counter(heading).get()` — gera nós `FuncCall` e `MethodCall` no AST.

O `eval` actual trata estas construções com o wildcard `_ => Ok(Value::None)`
ou como `SetRule` genérico sem efeito no numerador. Este passo intercede
nesses dois pontos.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Estrutura actual de Content::Heading
grep -A 5 "Heading" 01_core/src/entities/content.rs | head -15

# 2. Como o braço Heading está implementado no layouter
grep -n -A 15 "Content::Heading" 01_core/src/rules/layout.rs | head -25

# 3. Verificar se SetRule para "heading" é interceptado no eval
grep -n "SetRule\|set.*heading\|numbering" 01_core/src/rules/eval.rs | head -15

# 4. Verificar se counter() ou context() caem no wildcard ou geram erro
grep -n "counter\|context\b" 01_core/src/rules/eval.rs | head -10

# 5. Verificar a assinatura actual da função pública layout()
grep -n "^pub fn layout" 01_core/src/rules/layout.rs | head -5
```

Reportar o output completo antes de continuar. As respostas às questões
3 e 5 são as mais críticas: determinam se a assinatura de `layout()` muda
de forma breaking e se `SetRule` já tem uma estrutura de dados aproveitável.

---

## Tarefa 1 — `CounterState` em L1

Criar `01_core/src/entities/counter_state.rs`:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter_state.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-04-12

/// Estado de contadores que viaja com o Layouter durante uma passagem.
///
/// Cristalino diverge do Typst original aqui: o original resolve contadores
/// em duas passagens com `comemo` (para suportar referências para a frente).
/// Esta implementação usa uma única passagem — suficiente para numeração
/// sequencial de headings.
///
/// DEBT-10: Resolver contadores em duas passagens com estado global quando
/// o motor de introspecção completo for implementado (Passos 60+).
#[derive(Debug, Clone, Default)]
pub struct CounterState {
    /// Níveis activos de heading. `[1, 2]` representa a secção 1.2.
    heading: Vec<usize>,
    /// Se a numeração de headings está activa.
    /// Activada por `#set heading(numbering: "1.1")` ou equivalente.
    pub heading_numbering: bool,
}

impl CounterState {
    pub fn new() -> Self { Self::default() }

    /// Avança o contador para o nível indicado.
    ///
    /// - Se `level` for maior que o comprimento actual: preenche com zeros
    ///   até `level - 1` e adiciona 1.
    /// - Se `level` for igual ao comprimento: incrementa o último elemento.
    /// - Se `level` for menor que o comprimento: trunca e incrementa.
    ///
    /// Exemplos:
    /// - `[]` + level 1 → `[1]`
    /// - `[1]` + level 2 → `[1, 1]`
    /// - `[1, 1]` + level 1 → `[2]`
    /// - `[1, 2]` + level 2 → `[1, 3]`
    pub fn step_heading(&mut self, level: usize) {
        let level = level.max(1);
        self.heading.truncate(level);
        if self.heading.len() < level {
            self.heading.resize(level - 1, 0);
            self.heading.push(1);
        } else {
            // len() == level após truncate
            if let Some(last) = self.heading.last_mut() {
                *last += 1;
            }
        }
    }

    /// Retorna a string formatada do nível actual.
    /// Retorna `None` se o vector estiver vazio.
    ///
    /// Exemplos: `[1]` → `"1"`, `[1, 2]` → `"1.2"`.
    pub fn format_heading(&self) -> Option<String> {
        if self.heading.is_empty() {
            None
        } else {
            Some(self.heading.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join("."))
        }
    }
}
```

Registar o módulo em `01_core/src/entities/mod.rs`:

```rust
pub mod counter_state;
```

Criar o prompt L0 em `00_nucleo/prompts/entities/counter_state.md` com
os critérios de verificação da Tarefa 4.

---

## Tarefa 2 — Intercepção no `eval`

### 2a — `#set heading(numbering: ...)` activa a numeração

No braço de `SetRule` em `eval.rs`, identificar o target `"heading"` e o
argumento `numbering`. Quando presente e não `none`, produzir um
`Content::SetHeadingNumbering { active: true }` (nova variante simples) ou,
se o padrão `SetRule` já propaga para `StyleChain`, adicionar a flag
`heading_numbering` ao `EvalContext` para que o layouter a leia.

A decisão depende do diagnóstico: usar a abordagem que minimize a
perturbação da `StyleChain` existente. Se não for claro, preferir uma
variante de `Content` nova e documentar como DEBT-10.

**Nota de segurança — tipagem do argumento `numbering`:** No Typst, o
argumento `numbering` aceita tanto `String` (`"1.1"`, `"I.a"`) como uma
closure. A extracção do valor deve fazer pattern matching exclusivamente
sobre `Value::Str`. Qualquer outro tipo (closure, `Value::None`, ou outro)
deve ser tratado como "numeração inactiva" sem panic:

```rust
// Correcto — defensivo contra closures e outros tipos
let active = match args.get("numbering") {
    Some(Value::Str(_)) => true,
    _ => false,  // None, closure, ou qualquer outro tipo → ignorar
};
```

Nunca usar `.unwrap()` ou `.expect()` no valor de `numbering`.

### 2b — `counter(heading).get()` e `context { ... }`

No wildcard do eval (ou no braço de `FuncCall`), interceptar chamadas ao
padrão `counter(heading)`:

```rust
// Fallback para counter(heading).get() e context { counter(heading).display() }
// enquanto o motor de introspecção completo não existe.
// Produz Content::CounterDisplay { kind: "heading" }.
```

`Content::CounterDisplay` é uma nova variante mínima:

```rust
// Em content.rs:
CounterDisplay {
    kind: String, // "heading" por agora; "figure", "equation" em passos futuros
},
```

Actualizar `plain_text()` (retornar string vazia — o número só existe em
layout) e `is_empty()` (retornar `false`).

---

## Tarefa 3 — Propagação do estado no Layouter

### 3a — Adicionar `CounterState` ao Layouter

A struct do Layouter recebe um campo adicional:

```rust
pub struct Layouter<M: FontMetrics> {
    // ... campos existentes ...
    pub counter: CounterState,
}
```

Se a função pública `layout(content: &Content) -> PagedDocument` existir,
a sua assinatura não muda — o `CounterState` começa com `CounterState::new()`
e é gerido internamente:

```rust
pub fn layout(content: &Content) -> PagedDocument {
    let mut l = Layouter::new(/* métricas actuais */);
    // counter já está em Layouter::new() com CounterState::new()
    l.layout_content(content);
    l.finish()
}
```

Se o Layouter for uma struct anónima ou a função `layout` não tiver uma
struct explícita, adicionar o `CounterState` como campo local da função
raiz de iteração e passar por referência mutável nas chamadas recursivas.

### 3b — Braço `Content::Heading`

```rust
Content::Heading { level, body } => {
    self.counter.step_heading(*level as usize);

    let scale = heading_scale(*level);
    let heading_size = self.font_size_pt * scale;
    let prev = self.style;
    self.style = TextStyle { bold: true, italic: false, size: heading_size };

    if self.cursor_x > MARGIN { self.flush_line(); }

    // Prefixo numérico — apenas se numbering estiver activo
    if self.counter.heading_numbering {
        if let Some(num_str) = self.counter.format_heading() {
            // Usar layout_node com Content::text, não layout_word directamente.
            // layout_word pode não existir como método público — a API estável
            // para inserir texto programático é sempre Content::text → layout_node.
            self.layout_node(&Content::text(format!("{}. ", num_str)), self.style);
        }
    }

    self.layout_content(body);
    self.flush_line();
    self.style = prev;
},
```

### 3c — Braço `Content::CounterDisplay`

```rust
Content::CounterDisplay { kind } => {
    let text = if kind == "heading" {
        self.counter.format_heading()
            .unwrap_or_else(|| "0".to_string())
    } else {
        // Fallback para outros tipos — passos futuros
        format!("counter({})", kind)
    };
    self.layout_node(&Content::text(text), style)
},
```

### 3d — Braço `Content::SetHeadingNumbering`

Se a Tarefa 2a criar esta variante em vez de propagar via `StyleChain`, o
layouter precisa de a consumir sem desenhar nada. Sem este braço, o match
cai no wildcard (se existir) ou causa um compile error por variante não
coberta:

```rust
// Em layout_content:
Content::SetHeadingNumbering { active } => {
    self.counter.heading_numbering = *active;
    // Não desenha nada — apenas actualiza o estado do layouter.
},
```

### 3e — Braço `Content::Labelled`

O Layouter do Passo 56 já tem este braço. Confirmar que o `CounterState`
não precisa de ser actualizado aqui (correcto — a Label é metainformação,
não avança contadores).

---

## Tarefa 4 — Testes

### Testes L1 — `CounterState` isolado

```rust
#[test]
fn step_heading_nivel_1_inicial() {
    let mut s = CounterState::new();
    s.step_heading(1);
    assert_eq!(s.format_heading(), Some("1".to_string()));
}

#[test]
fn step_heading_dois_niveis() {
    let mut s = CounterState::new();
    s.step_heading(1);
    s.step_heading(2);
    assert_eq!(s.format_heading(), Some("1.1".to_string()));
}

#[test]
fn step_heading_nivel_2_apos_nivel_2() {
    let mut s = CounterState::new();
    s.step_heading(1);
    s.step_heading(2);
    s.step_heading(2);
    assert_eq!(s.format_heading(), Some("1.2".to_string()));
}

#[test]
fn step_heading_volta_ao_nivel_1() {
    let mut s = CounterState::new();
    s.step_heading(1);
    s.step_heading(2);
    s.step_heading(1);
    assert_eq!(s.format_heading(), Some("2".to_string()));
}

#[test]
fn step_heading_tres_niveis_sequencia_completa() {
    let mut s = CounterState::new();
    s.step_heading(1); // [1]
    s.step_heading(2); // [1, 1]
    s.step_heading(3); // [1, 1, 1]
    s.step_heading(2); // [1, 2]
    s.step_heading(1); // [2]
    assert_eq!(s.format_heading(), Some("2".to_string()));
}

#[test]
fn format_heading_vazio_retorna_none() {
    let s = CounterState::new();
    assert_eq!(s.format_heading(), None);
}
```

### Testes L1 — Layout com numeração

```rust
#[test]
fn layout_heading_sem_numbering_nao_tem_prefixo() {
    // Por defeito, heading_numbering é false — não deve aparecer "1."
    let doc = layout(&Content::heading(1, Content::text("Intro")));
    let text = doc.plain_text();
    assert!(!text.contains("1."), "sem numbering activo, não deve haver prefixo numérico");
    assert!(text.contains("Intro"));
}

#[test]
fn layout_heading_com_numbering_tem_prefixo() {
    use crate::entities::counter_state::CounterState;
    use crate::rules::layout::layout_with_state;

    let mut state = CounterState::new();
    state.heading_numbering = true;
    let content = Content::Sequence(vec![
        Content::heading(1, Content::text("Intro")),
        Content::heading(2, Content::text("Motivação")),
        Content::heading(1, Content::text("Conclusão")),
    ]);
    let doc = layout_with_state(&content, state);
    let text = doc.plain_text();
    assert!(text.contains("1."), "H1 deve ter prefixo '1.'");
    assert!(text.contains("1.1"), "H2 deve ter prefixo '1.1'");
    assert!(text.contains("2."), "segundo H1 deve ter prefixo '2.'");
}
```

Se `layout_with_state` não existir ainda, criá-la como variante de `layout()`
que aceita um `CounterState` inicial:

```rust
pub fn layout_with_state(content: &Content, state: CounterState) -> PagedDocument {
    let mut l = Layouter::new(/* métricas actuais */);
    l.counter = state;
    l.layout_content(content);
    l.finish()
}
```

### Testes L3 — Pipeline completo

```rust
#[test]
fn pipeline_heading_numeracao_por_defeito_sem_prefixo() {
    // Sem #set heading(numbering: ...), o PDF não deve ter "1."
    let (world, _dir) = world_from_str("= Introdução\n== Motivação");
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty());
    // Verificação textual: "Introdução" e "Motivação" devem aparecer;
    // prefixo numérico não deve aparecer sem set rule explícito.
}

#[test]
fn pipeline_heading_numeracao_activa() {
    let (world, _dir) = world_from_str(
        "#set heading(numbering: \"1.1\")\n= Introdução\n== Motivação\n= Conclusão"
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty());
    let pdf_str = String::from_utf8_lossy(&pdf);
    // "1." deve aparecer no stream do PDF como prefixo do primeiro heading
    assert!(pdf_str.contains("1."), "H1 deve ter prefixo numérico no PDF");
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
- [ ] `CounterState` criado em `01_core/src/entities/counter_state.rs` com
  header de linhagem.
- [ ] `step_heading` correcta para todos os padrões de progressão de nível.
- [ ] `format_heading` retorna `None` para estado inicial e string com pontos
  para estados não-vazios.
- [ ] Layouter tem campo `counter: CounterState`.
- [ ] Braço `Content::Heading` chama `step_heading` antes de desenhar.
- [ ] Prefixo numérico aparece apenas quando `heading_numbering == true`.
- [ ] `Content::CounterDisplay` adicionado ao enum e tratado no layouter.
- [ ] DEBT-10 registado em `01_core/DEBT.md`.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `SetRule` já tinha estrutura de dados para o target `"heading"` ou se foi
  necessário criá-la.
- Se a assinatura de `layout()` mudou (breaking change) ou se o `CounterState`
  ficou interno ao Layouter sem expor-se na API pública.
- Se `counter(heading).get()` já caía no wildcard ou tinha tratamento parcial.

**Da implementação:**
- Se `layout_with_state` foi necessária para os testes ou se outra abordagem
  foi usada.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 58:**
- **GO — numeração de figuras e equações**: headings numerados funcionam;
  Passo 58 adiciona `Counter` genérico para `figure` e `equation`, com
  `CounterDisplay` a resolver o tipo correcto.
- **GO — `#counter(heading).step()`**: se o eval já intercepta chamadas
  a `counter`, Passo 58 liga `MethodCall` `step()` / `update()` ao
  `CounterState` passado por contexto.
- **NO-GO — duas passagens necessárias**: se testes de referência para a
  frente (ex: `@intro` resolvendo para "Secção 1" antes do heading aparecer)
  são críticos para o passo seguinte, Passo 58 resolve o estado global antes
  de expandir o inventário de contadores.
