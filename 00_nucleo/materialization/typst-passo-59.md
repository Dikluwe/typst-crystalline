# Passo 59 — Contadores Automáticos e Resolução de Referências (Single-Pass)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/counter_state.rs` — `CounterState` genérico com
  `hierarchical`, `flat`, `numbering_active` do Passo 58.
- `01_core/src/rules/layout.rs` — Braços actuais para `Heading`, `Labelled`,
  `Ref`, `CounterUpdate`, `SetHeadingNumbering`.
- `01_core/src/entities/label.rs` — A entidade `Label` usada como chave de
  resolução.
- `01_core/src/entities/content.rs` — Confirmar se `Equation` e `Figure`
  existem e quais os seus campos.

Pré-condição: `cargo test` — 609 L1 + 115 L3 + 50 parity, zero violations.
O `CounterState` genérico está operacional e `MethodCall` é interceptado.

---

## Contexto

Para que `@intro` seja convertido em "Secção 1.1" no PDF, o Layouter precisa
de um mapa em memória que associe cada `Label` ao texto formatado pelo
contador no momento em que o nó foi encontrado.

O paradigma Single-Pass do Cristalino funciona da seguinte forma:

1. **Registo:** o Layouter encontra `Labelled { target, label }`, inspecciona
   o tipo de `target`, formata o contador actual, e guarda no dicionário
   `resolved_labels: HashMap<Label, String>`.
2. **Resolução:** o Layouter encontra `Ref { target }`, consulta o dicionário.
   Se a chave existir (referência para trás), desenha o texto resolvido.
   Se não existir (referência para a frente), desenha o fallback `@nome`.
3. **Auto-numeração:** `Equation` e `Figure` avançam os seus contadores planos
   automaticamente se a numeração estiver activa para a chave correspondente.

A limitação do Single-Pass (referências para a frente não resolvem) é
registada como DEBT-10 e endereçada quando o motor de introspecção de duas
passagens for implementado.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Verificar campos de Equation e Figure no Content
grep -n -A 5 "Equation {" 01_core/src/entities/content.rs
grep -n -A 5 "Figure {"   01_core/src/entities/content.rs

# 2. Confirmar que Label deriva Hash (necessário para HashMap<Label, String>)
grep -n "derive\|Hash\|Eq\|PartialEq" 01_core/src/entities/label.rs | head -10

# 3. Ver o braço Labelled actual no Layouter
grep -n -A 8 "Content::Labelled" 01_core/src/rules/layout.rs | head -15

# 4. Ver o braço Ref actual no Layouter
grep -n -A 5 "Content::Ref" 01_core/src/rules/layout.rs | head -10

# 5. Verificar se o prompt L0 de layout existe
ls -l 00_nucleo/prompts/rules/layout*.md 2>/dev/null
```

Reportar o output completo antes de continuar. As respostas às questões 1 e 2
são as mais críticas: se `Figure` não existe em `Content`, a Tarefa 4 fica
restrita a `Equation`; se `Label` não deriva `Hash`, o `HashMap<Label, String>`
não compila sem uma linha adicional de `derive`.

---

## Tarefa 1 — Criar o Prompt L0 (L0)

Antes de qualquer código, criar o ficheiro de especificação para que o
linter não dispare V7 (OrphanPrompt) quando o ficheiro do módulo for
actualizado.

Criar `00_nucleo/prompts/rules/layout_references.md`:

```markdown
# L0 — Layout: Referências e Contadores Automáticos

## Módulo
`01_core/src/rules/layout.rs` — secção de resolução de labels e auto-numeração.

## Regras de negócio

### Resolução Single-Pass
O Layouter executa numa única passagem. O `CounterState` acumula
`resolved_labels: HashMap<Label, String>` à medida que avança.

- **Labelled**: não tem presença visual. Side-effect: insere no dicionário
  o texto formatado do contador actual (ex: `Label("intro") → "Secção 1.1"`).
- **Ref**: consulta o dicionário. Encontrou → desenha o texto resolvido.
  Não encontrou → fallback literal `@nome` (DEBT-10: referências para a frente).

### Auto-numeração
- `Equation { block: true, .. }`: se `numbering_active["equation"]` for
  verdadeiro, avança `step_flat("equation")` antes de desenhar e adiciona
  o número formatado `(N)` à direita da equação.
- `Figure { .. }`: se `numbering_active["figure"]` for verdadeiro, avança
  `step_flat("figure")` antes de desenhar e prefixa a caption com
  `"Figura N: "`.

### Limitação conhecida (DEBT-10)
Referências para a frente (a label aparece depois da Ref no documento)
não são resolvidas nesta passagem — exigem o motor de introspecção de
duas passagens (Passos 60+).

## Critérios de verificação
- `Labelled(Heading, label)` → `resolved_labels` contém a chave após layout.
- `Ref(label)` para trás → plain_text contém o texto resolvido.
- `Ref(label)` para a frente → plain_text contém `@nome` (não panic).
- `Equation { block: true }` numerada → número aparece no PDF.
```

Depois de criar o ficheiro:

```bash
git add 00_nucleo/prompts/rules/layout_references.md
crystalline-lint --fix-hashes .
```

---

## Tarefa 2 — Dicionário `resolved_labels` no `CounterState` (L1)

Em `01_core/src/entities/counter_state.rs`, adicionar o campo ao struct:

```rust
use crate::entities::label::Label;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct CounterState {
    pub hierarchical:    HashMap<String, Vec<usize>>,
    pub flat:            HashMap<String, usize>,
    pub numbering_active: HashMap<String, bool>,
    /// Mapa de labels para o texto resolvido na passagem actual.
    /// Chave: Label; Valor: texto formatado (ex: "Secção 1.1", "Figura 2").
    /// DEBT-10: apenas referências para trás são resolvidas.
    pub resolved_labels: HashMap<Label, String>,
}
```

**Pré-condição:** `Label` deve derivar `Hash` e `Eq`. Confirmar com o
diagnóstico. Se não derivar:

```rust
// Em label.rs — adicionar à lista de derives:
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);
```

---

## Tarefa 3 — Registo de Labels no Layouter (L1)

Em `01_core/src/rules/layout.rs`, substituir o braço `Content::Labelled`
actual (que apenas fazia layout transparente) pela versão com registo:

```rust
Content::Labelled { target, label } => {
    // Inspecionar o tipo de alvo para determinar o prefixo correcto.
    // O registo acontece com o valor do contador *antes* de este avançar
    // — o heading já avançou o seu contador ao ser processado.
    let resolved_text = match &**target {
        Content::Heading { .. } => {
            self.counter.format_hierarchical("heading")
                .map(|num| format!("Secção {}", num))
        },
        Content::Equation { .. } => {
            self.counter.get_flat("equation")
                .checked_sub(0)  // get_flat devolve 0 se não existir
                .filter(|&n| n > 0)
                .map(|num| format!("Equação ({})", num))
        },
        Content::Figure { .. } => {
            let num = self.counter.get_flat("figure");
            if num > 0 { Some(format!("Figura {}", num)) } else { None }
        },
        _ => None,
    };

    // Arquivar se conseguimos formatar um número.
    if let Some(text) = resolved_text {
        self.counter.resolved_labels.insert(label.clone(), text);
    }

    // Layout transparente — a label não tem presença visual.
    self.layout_node(target, self.style)
},
```

**Nota sobre a ordem:** o Layouter processa os nós sequencialmente. Quando
encontra `Labelled { target: Heading, .. }`, o braço `Content::Heading` ainda
não foi executado para este nó — será executado dentro de `layout_node(target)`.
Por isso, o registo deve acontecer *depois* de `layout_node(target)` se o
contador só avança dentro do braço do Heading. Ajustar a ordem conforme o
comportamento confirmado pelo diagnóstico.

A forma mais segura é chamar `layout_node` primeiro e depois ler o contador:

```rust
Content::Labelled { target, label } => {
    // Layout primeiro — garante que o contador foi avançado pelo braço do alvo.
    self.layout_node(target, self.style);

    // Registar depois com o valor actualizado.
    let resolved_text = match &**target {
        Content::Heading { .. } =>
            self.counter.format_hierarchical("heading")
                .map(|num| format!("Secção {}", num)),
        Content::Equation { .. } => {
            let n = self.counter.get_flat("equation");
            if n > 0 { Some(format!("Equação ({})", n)) } else { None }
        },
        Content::Figure { .. } => {
            let n = self.counter.get_flat("figure");
            if n > 0 { Some(format!("Figura {}", n)) } else { None }
        },
        _ => None,
    };
    if let Some(text) = resolved_text {
        self.counter.resolved_labels.insert(label.clone(), text);
    }
},
```

---

## Tarefa 4 — Resolução de Referências (L1)

Substituir o braço `Content::Ref` actual (que apenas desenhava `@nome`):

```rust
Content::Ref { target } => {
    let display_text = match self.counter.resolved_labels.get(target) {
        Some(text) => text.clone(),
        // Fallback visual limpo para referências para a frente — DEBT-10.
        None => format!("@{}", target.0),
    };
    self.layout_node(&Content::text(display_text), self.style)
},
```

---

## Tarefa 5 — Auto-numeração de Equações e Figuras (L1)

Esta tarefa depende do resultado do diagnóstico sobre os campos de `Equation`
e `Figure`. Se estas variantes não existirem em `Content` ainda, registar
em DEBT-10 e pular para a Tarefa 6.

### Se `Content::Equation { block, body, .. }` existe

No braço do Layouter para `Content::Equation`:

```rust
Content::Equation { block, body } => {
    // Auto-numeração apenas em equações de bloco (display math).
    let is_numbered = *block
        && self.counter.is_numbering_active("equation");

    if is_numbered {
        self.counter.step_flat("equation");
    }

    // Layout da equação existente (MathLayouter).
    // [ manter código de layout matemático existente ]

    // Se numerada, acrescentar o número alinhado à direita da linha.
    if is_numbered {
        let n = self.counter.get_flat("equation");
        let num_text = format!("({})", n);
        // DEBT: alinhamento à direita real requer conhecimento da largura
        // da página — por agora, acrescentar inline como texto separado.
        self.layout_node(&Content::text(num_text), self.style);
    }
},
```

### Se `Content::Figure { body, caption }` existe

```rust
Content::Figure { body, caption } => {
    // Figuras são numeradas por defeito se a flag estiver activa.
    let is_numbered = self.counter.is_numbering_active("figure");

    if is_numbered {
        self.counter.step_flat("figure");
    }

    self.layout_node(body, self.style);

    if let Some(cap) = caption {
        if is_numbered {
            let n = self.counter.get_flat("figure");
            let prefix = format!("Figura {}: ", n);
            self.layout_node(&Content::text(prefix), self.style);
        }
        self.layout_node(cap, self.style);
    }
},
```

---

## Tarefa 6 — Testes

### Testes L1 — Resolução backward

```rust
#[test]
fn layout_ref_para_tras_resolve_secao() {
    use crate::entities::counter_state::CounterState;
    use crate::entities::label::Label;
    use crate::rules::layout::layout_with_state;

    let mut state = CounterState::new();
    state.numbering_active.insert("heading".to_string(), true);

    let content = Content::Sequence(vec![
        Content::Labelled {
            label:  Label("intro".to_string()),
            target: Box::new(Content::heading(1, Content::text("Introdução"))),
        },
        Content::text(" Como vimos em "),
        Content::Ref { target: Label("intro".to_string()) },
    ]);

    let doc = layout_with_state(&content, state);
    let text = doc.plain_text();
    assert!(
        text.contains("Secção 1"),
        "Ref para trás deve resolver para 'Secção 1', obtido: {:?}", text
    );
}

#[test]
fn layout_ref_para_frente_usa_fallback() {
    use crate::entities::counter_state::CounterState;
    use crate::entities::label::Label;
    use crate::rules::layout::layout_with_state;

    let content = Content::Sequence(vec![
        // Ref aparece antes da Label — referência para a frente
        Content::Ref { target: Label("conclusao".to_string()) },
        Content::Labelled {
            label:  Label("conclusao".to_string()),
            target: Box::new(Content::heading(1, Content::text("Conclusão"))),
        },
    ]);

    let doc = layout_with_state(&content, CounterState::new());
    let text = doc.plain_text();
    assert!(
        text.contains("@conclusao"),
        "Ref para a frente deve usar fallback '@conclusao', obtido: {:?}", text
    );
}

#[test]
fn layout_resolved_labels_nao_interfere_entre_documentos() {
    // Dois layouts independentes não devem partilhar estado.
    use crate::entities::counter_state::CounterState;
    use crate::entities::label::Label;
    use crate::rules::layout::{layout, layout_with_state};

    let mut state = CounterState::new();
    state.numbering_active.insert("heading".to_string(), true);

    let content_a = Content::Labelled {
        label:  Label("sec".to_string()),
        target: Box::new(Content::heading(1, Content::text("A"))),
    };
    let _ = layout_with_state(&content_a, state);

    // Segundo layout com state limpo — não deve ter "sec" resolvida
    let content_b = Content::Ref { target: Label("sec".to_string()) };
    let doc_b = layout(&content_b);
    assert!(
        doc_b.plain_text().contains("@sec"),
        "Estado do layout anterior não deve vazar para o seguinte"
    );
}
```

### Testes L3 — Pipeline completo

```rust
#[test]
fn pipeline_ref_backward_resolve_no_pdf() {
    let (world, _dir) = world_from_str(
        "#set heading(numbering: \"1.\")\n\
         = Metodologia <metodo>\n\
         De acordo com a @metodo..."
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty());
    let pdf_str = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_str.contains("Sec"),  // "Secção" pode ter encoding no PDF
        "PDF deve conter o prefixo da referência resolvida"
    );
}

#[test]
fn pipeline_ref_forward_nao_causa_panico() {
    // Referência para a frente não deve causar panic — apenas fallback.
    let (world, _dir) = world_from_str(
        "Ver a @conclusao\n= Conclusão <conclusao>"
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty(), "PDF deve ser gerado mesmo com forward ref");
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
- [ ] Prompt L0 `layout_references.md` criado e registado.
- [ ] `Label` deriva `Hash` + `Eq`.
- [ ] `resolved_labels: HashMap<Label, String>` em `CounterState`.
- [ ] Braço `Labelled` regista o texto resolvido *depois* de layoutar o target.
- [ ] Braço `Ref` consulta `resolved_labels` e usa fallback se não encontrar.
- [ ] `Equation` e `Figure` avançam contadores se `numbering_active` for
  verdadeiro (ou anotado em DEBT-10 se as variantes não existirem).
- [ ] Referências para trás resolvem; referências para a frente não causam
  panic.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `Content::Equation` e `Content::Figure` existem e quais os seus campos.
  Se não existirem, anotar em DEBT-10 e indicar em que passo serão adicionadas.
- Se `Label` já derivava `Hash` e `Eq` ou precisou de ser alterada.
- Se o prompt L0 `layout_references.md` foi criado de raiz ou expandiu um
  ficheiro existente.

**Da implementação:**
- Se o registo no braço `Labelled` ficou antes ou depois de `layout_node`
  (e porquê — depende de quando o contador do Heading avança).
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 60:**
- **GO — motor de introspecção duas passagens**: referências backward
  funcionam; Passo 60 implementa a pré-passagem que resolve `resolved_labels`
  antes do layout real, eliminando DEBT-10 para referências forward.
- **GO — Tabela de Conteúdos**: com `resolved_labels` operacional, Passo 60
  pode gerar uma TOC como sequência de `Ref` que resolvem para os títulos
  numerados.
- **NO-GO — Label sem Hash**: se a adição de `Hash` a `Label` criou conflitos
  de derive com outros campos da struct; Passo 60 resolve antes de avançar.
