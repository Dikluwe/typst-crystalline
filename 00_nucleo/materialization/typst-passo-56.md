# Passo 56 — Fundação de Introspecção: Labels (`<label>`) e Referências (`@ref`)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — Ciclo principal de avaliação de expressões e blocos.
- `01_core/src/entities/content.rs` — Enumeração `Content`.
- `01_core/src/entities/ast/expr.rs` (ou equivalente) — Definição do parser para a sintaxe `<label>` e `@ref`.

Pré-condição: `cargo test` — 585 L1 + 110 L3 + 50 parity, zero violations.
O motor matemático está fechado (Passos 36–55).

---

## Contexto

Para iniciar a implementação de estado (contadores, índices, etc.), o sistema
precisa de identificar univocamente blocos de conteúdo. A sintaxe do Typst
permite anexar uma etiqueta a um elemento colocando-a imediatamente a seguir
(ex: `= Introdução <intro>`). Para referenciar, usa-se a sintaxe de arroba
(ex: `Conforme visto na secção @intro`).

Neste passo inaugural, não vamos implementar o motor de resolução de referências
(isso requer o sistema de paginação e estado global). Vamos **apenas estender
a AST e o `eval`** para que o sistema compreenda a sintaxe, guarde a Label na
memória, e passe com sucesso pela fase de construção de `Content` sem perder
a informação.

---

## Diagnósticos obrigatórios antes de codificar

Confirmar como o parser do Typst traduz etiquetas para a AST e como a associação
ao nó precedente é estruturada.

```bash
# 1. Definições de Label e Ref na AST original
grep -n "Label" lab/typst-original/crates/typst-syntax/src/ast.rs | head -10
grep -n "Ref"   lab/typst-original/crates/typst-syntax/src/ast.rs | head -10

# 2. Como o Cristalino representa Labels/Refs actualmente
grep -n "Label\|Ref" 01_core/src/entities/ast/expr.rs | head -10

# 3. Localizar o loop de avaliação de markup
grep -n "fn eval_markup" 01_core/src/rules/eval.rs | head -5
```

Reportar o output completo antes de continuar. O diagnóstico confirmará
se o parser agrupa a `<label>` como filho do nó precedente ou como nó
irmão independente — a lógica de associação na Tarefa 2 depende desta
distinção.

---

## Tarefa 1 — Representação da Label no Domínio (L1)

Em `01_core/src/entities/content.rs` (ou num novo ficheiro
`01_core/src/entities/label.rs` se preferir isolamento):

### 1a — Criar a estrutura `Label`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String); // String simples — ADR-0015 proíbe EcoString em L1
```

### 1b — Adicionar variantes ao `Content`

Aviso arquitectural: adicionar `Option<Label>` a cada variante existente
(`Heading`, `Equation`, etc.) é um anti-padrão que polui a estrutura interna
de cada nó e obriga a actualizar todos os match arms. A abordagem correcta é
uma variante de embrulho que anexa metadados a qualquer nó sem tocar nas
variantes existentes.

```rust
// Em Content:

Labelled {
    target: Box<Content>,
    label: Label,
},
Ref {
    target: Label,
    // Em passos futuros, o motor de introspection substituirá este nó
    // pelo texto resolvido (ex: "Secção 1"). Por agora, é um placeholder.
},
```

Actualizar `plain_text()` e `is_empty()` para as novas variantes:
- `Labelled`: delegar em `target.plain_text()` / `target.is_empty()`.
- `Ref`: retornar `format!("@{}", target.0)` / `false`.

---

## Tarefa 2 — Intercepção e Modificação no `eval`

O desafio de avaliar Labels em Typst é que operam de forma "retroactiva" a
nível semântico mas são sequenciais no AST. O parser gera tipicamente um nó
`Heading` seguido de um nó `Label` como irmãos — não como pai/filho.

Em `01_core/src/rules/eval.rs`:

### 2a — Mapeamento simples

Adicionar braços na função principal de avaliação (nome confirmado pelo
diagnóstico) para `Expr::Label` e `Expr::Ref`:

- `Expr::Ref(ref_node)` → `Content::Ref { target: Label(nome) }`.
- `Expr::Label(label_node)` → retornar o nome como `Label`; a associação
  acontece em 2b, não aqui directamente.

### 2b — Associação retroactiva

Na função que itera sobre os nós de markup (nome confirmado pelo diagnóstico),
adicionar a lógica de embrulho:

1. Manter o array de partes produzidas no loop (`parts: Vec<Content>`).
2. Quando o nó actual for do tipo `Label`, em vez de o adicionar como elemento
   independente:
   - Remover o último elemento de `parts`.
   - Envolvê-lo na variante `Content::Labelled`.
   - Empurrar o resultado de volta para `parts`.
3. Se `parts` estiver vazio quando uma `Label` aparecer (label sem nó
   precedente no mesmo bloco), ignorar silenciosamente ou emitir diagnóstico
   — o diagnóstico confirmará o comportamento do original.

---

## Tarefa 3 — Layout de Pass-Through (L1)

No motor de layout (`01_core/src/rules/layout.rs` ou equivalente):

### 3a — Transparência do `Labelled`

A `Label` é metainformação pura. Não tem presença visual no fluxo de layout:

```rust
Content::Labelled { target, .. } => self.layout_node(target, style),
```

### 3b — Fallback visual do `Ref`

Enquanto não existe motor de cross-reference, desenhar a etiqueta literalmente
para confirmar que a informação sobreviveu ao pipeline:

```rust
Content::Ref { target } => {
    // Temporário até termos estado global (Passos 56+)
    self.layout_node(&Content::text(format!("@{}", target.0)), style)
},
```

---

## Tarefa 4 — Testes Unitários e de Integração

### Testes L1 (`eval.rs` ou ficheiro de testes equivalente)

```rust
#[test]
fn eval_label_anexa_ao_bloco_anterior() {
    // `= Título <meu_label>` deve gerar Content::Labelled envolvendo Content::Heading
    let world = MockWorld::new("= Título <meu_label>");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let content = module.content().expect("deve ter content");
    assert!(matches!(content, Content::Labelled {
        target,
        label: Label(ref s),
    } if matches!(**target, Content::Heading { .. }) && s == "meu_label"),
        "esperado Labelled(Heading), obtido: {:?}", content);
}

#[test]
fn eval_ref_gera_content_ref() {
    // `@meu_label` deve gerar Content::Ref { target: Label("meu_label") }
    let world = MockWorld::new("@meu_label");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let content = module.content().expect("deve ter content");
    assert!(matches!(content, Content::Ref { target: Label(ref s) } if s == "meu_label"),
        "esperado Ref(meu_label), obtido: {:?}", content);
}
```

### Testes L3 (`integration_tests.rs`)

```rust
#[test]
fn pipeline_introspeccao_labels_refs_gera_pdf() {
    // Pipeline completo: eval + layout + export sem pânico.
    // O PDF deve conter o texto literal "@intro" como prova do fallback.
    let (world, _dir) = world_from_str("= Introdução <intro>\nIsto é uma referência: @intro");
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty(), "PDF não deve estar vazio");
    // Verificação heurística: o fallback "@intro" deve aparecer no stream PDF
    let pdf_str = String::from_utf8_lossy(&pdf);
    assert!(pdf_str.contains("@intro"), "PDF deve conter o texto de fallback @intro");
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
- [ ] Estrutura `Label` criada em L1.
- [ ] Variantes `Labelled` e `Ref` adicionadas a `Content`.
- [ ] `plain_text()` e `is_empty()` actualizados para as duas variantes.
- [ ] A função de avaliação de markup associa uma `Label` ao nó precedente.
- [ ] O layouter atravessa `Labelled` de forma transparente, sem pânico.
- [ ] O layouter produz o fallback literal `@nome` para `Ref`.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Nome real da função que itera sobre nós de markup (se não for `eval_markup`).
- Se o parser agrupa `<label>` como filho do nó precedente ou como nó irmão.
- Se `Expr::Label` e `Expr::Ref` já existiam na AST cristalina ou precisaram
  de ser adicionados.

**Da implementação:**
- Se `parts.last_mut()` foi suficiente ou foi necessária outra abordagem de
  associação (ex: o diagnóstico revelou que a label vem como filho e não irmão).
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 57:**
- **GO — contadores**: `Label` e `Ref` funcionam no pipeline; Passo 57
  implementa o sistema de contadores (`Counter`) que associa valores numéricos
  a Labels por tipo de nó (ex: headings numerados automaticamente).
- **NO-GO — associação retroactiva bloqueada**: o parser não expõe a Label
  como irmão mas como contexto não alcançável pelo eval; Passo 57 resolve
  a barreira de acesso à AST antes de continuar.
