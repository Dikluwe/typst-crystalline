# Passo 60 — Motor de Introspecção (Duas Passagens)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/layout.rs` — Braços actuais para `Labelled`, `Ref`,
  `Heading`, `CounterUpdate`, `SetHeadingNumbering`.
- `01_core/src/entities/counter_state.rs` — `CounterState` com
  `resolved_labels: HashMap<Label, String>`.
- `01_core/src/rules/mod.rs` — Onde o pipeline é orquestrado (se existir
  uma função `compile` ou equivalente).

Pré-condição: `cargo test` — 613 L1 + 116 L3 + 50 parity, zero violations.
O Passo 59 está operacional com resolução backward.

---

## Contexto

O sistema actual resolve referências em single-pass: uma `Ref` só é resolvida
se a `Label` correspondente apareceu *antes* no documento. Para resolver
referências para a frente (`@conclusao` antes do título `= Conclusão
<conclusao>`), o layouter precisa de saber o estado final dos contadores
*antes* de começar a gerar os elementos visuais.

A solução é uma pré-passagem analítica separada do layout físico:

1. **Passagem 1 — Introspecção:** percorre `Content` sem gerar FrameItems,
   apenas avançando contadores e populando `resolved_labels`. É leve —
   sem métricas de fonte, sem alocação de frames.
2. **Passagem 2 — Layout:** o Layouter físico arranca com `resolved_labels`
   já populado. O braço `Ref` consulta o mapa e encontra sempre a entrada
   (backward e forward).

O módulo `introspect.rs` encapsula a Passagem 1, mantendo `layout.rs`
focado exclusivamente na geometria visual.

Esta separação também resolve DEBT-11 parcialmente: a extracção de
`introspect.rs` é o primeiro passo para decompor `layout.rs` em
submódulos com responsabilidades claras.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Localizar onde layout() é chamada no pipeline
grep -rn "pub fn layout\b" 01_core/src/rules/ | head -10
grep -rn "layout(" 03_infra/src/integration_tests.rs | head -5

# 2. Confirmar assinatura actual de layout()
grep -n "^pub fn layout" 01_core/src/rules/layout.rs | head -5

# 3. Verificar se existe módulo introspect
ls -l 01_core/src/rules/introspect.rs 2>/dev/null || echo "não existe"

# 4. Confirmar que Content::Styled existe e qual o nome do campo filho
grep -n -A 3 "Styled" 01_core/src/entities/content.rs | head -15
```

Reportar o output completo antes de continuar. A resposta à questão 2 é
crítica: se `layout()` já aceita um `CounterState` (do Passo 57), a
mudança de assinatura pode ser apenas adicionar o dicionário pré-calculado;
se não aceita, a mudança é maior. A resposta à questão 4 determina o nome
do campo filho em `Content::Styled` — pode ser `child`, `body`, `content`,
ou `target`. O `walk()` em `introspect.rs` usa esse nome; se errar, o
compilador rejeita com erro de campo desconhecido.

---

## Tarefa 1 — Prompt L0 (L0)

Antes de qualquer código, criar o documento de especificação para que o
linter não dispare V7 quando o ficheiro `introspect.rs` for criado.

Criar `00_nucleo/prompts/rules/introspect.md`:

```markdown
# L0 — Motor de Introspecção (`rules/introspect.rs`)

## Módulo
`01_core/src/rules/introspect.rs`

## Propósito
Pré-passagem analítica sobre `Content`. Constrói o `CounterState`
completo (incluindo `resolved_labels`) antes do layout físico arrancar.
Permite resolver referências para a frente (forward refs).

## Regras de negócio

### O que a introspecção faz
- Percorre `Content` recursivamente via `walk()`.
- Avança contadores (`step_hierarchical`, `step_flat`) nos mesmos
  nós onde o Layouter o faria.
- Regista `resolved_labels` para cada `Labelled` encontrado.
- Intercede em `SetHeadingNumbering` e `CounterUpdate` para replicar
  os side-effects de estado.

### O que a introspecção NÃO faz
- Não acede a `FontMetrics`.
- Não aloca `Frame`, `FrameItem`, ou `PagedDocument`.
- Não produz output visual de nenhum tipo.

### Isolamento
A função pública `introspect(content: &Content) -> CounterState`
é pura: dado o mesmo `Content`, retorna sempre o mesmo `CounterState`.
Não tem estado global.

## Interface pública
```rust
pub fn introspect(content: &Content) -> CounterState;
```

## Critérios de verificação
- `Labelled` após `Heading` → `resolved_labels` contém a chave.
- `Labelled` antes de `Heading` → `resolved_labels` contém a chave
  (porque walk percorre o target antes de registar).
- `CounterUpdate { action: Update(5) }` → `flat["equation"] == 5`.
- `SetHeadingNumbering { active: true }` → `is_numbering_active("heading") == true`.
- Dois documentos independentes → estados independentes (sem partilha).
```

```bash
git add 00_nucleo/prompts/rules/introspect.md
```

---

## Tarefa 2 — Módulo `introspect.rs` (L1)

Criar `01_core/src/rules/introspect.rs`:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-04-12

use crate::entities::{
    content::Content,
    counter_state::{CounterAction, CounterState},
};

/// Pré-passagem analítica sobre `Content`.
///
/// Percorre a árvore completa uma vez, avançando contadores e populando
/// `resolved_labels`, sem realizar nenhum cálculo visual.
///
/// O `CounterState` retornado é passado ao Layouter como estado inicial,
/// garantindo que todas as referências — incluindo para a frente — estão
/// resolvidas antes do primeiro FrameItem ser gerado.
pub fn introspect(content: &Content) -> CounterState {
    let mut state = CounterState::new();
    walk(content, &mut state);
    state
}

fn walk(content: &Content, state: &mut CounterState) {
    match content {
        Content::Sequence(seq) => {
            for item in seq { walk(item, state); }
        },

        Content::Heading { level, body } => {
            state.step_hierarchical("heading", *level as usize);
            walk(body, state);
        },

        Content::Equation { block, body } => {
            if *block && state.is_numbering_active("equation") {
                state.step_flat("equation");
            }
            walk(body, state);
        },

        // Figure — adicionar quando Content::Figure existir (DEBT-10)
        // Content::Figure { body, caption } => { ... }

        Content::Labelled { target, label } => {
            // Walk no target primeiro — garante que o contador já avançou.
            walk(target, state);

            // Registar com o valor actual do contador.
            let resolved_text = match &**target {
                Content::Heading { .. } =>
                    state.format_hierarchical("heading")
                        .map(|n| format!("Secção {}", n)),
                Content::Equation { .. } => {
                    let n = state.get_flat("equation");
                    if n > 0 { Some(format!("Equação ({})", n)) } else { None }
                },
                _ => None,
            };
            if let Some(text) = resolved_text {
                state.resolved_labels.insert(label.clone(), text);
            }
        },

        Content::SetHeadingNumbering { active } => {
            state.numbering_active.insert("heading".to_string(), *active);
        },

        Content::CounterUpdate { key, action } => {
            match action {
                CounterAction::Step => {
                    if key == "heading" {
                        state.step_hierarchical("heading", 1);
                    } else {
                        state.step_flat(key);
                    }
                },
                CounterAction::Update(val) => {
                    state.update_flat(key, *val);
                },
            }
        },

        // Nós que contêm filhos — delegar recursivamente.
        Content::Strong(body)
        | Content::Emph(body) => walk(body, state),

        Content::Styled { child, .. } => {
            // ATENÇÃO: o nome do campo depende do diagnóstico.
            // Se o compilador rejeitar `child`, substituir pelo nome real
            // confirmado no diagnóstico 4 (pode ser `body`, `content`, ou `target`).
            walk(child, state);
        },

        // Nós com múltiplos filhos que podem conter Labels — percorrer todos.
        // Se Content::Table ou Content::Grid existirem, adicionar aqui:
        // Content::Table { rows, .. } => {
        //     for row in rows { for cell in row { walk(cell, state); } }
        // },
        // Uma Label dentro de uma célula de tabela que não seja percorrida
        // pela introspecção nunca será descoberta — DEBT implícito a registar.

        // Terminais e nós sem efeito em contadores — ignorar.
        Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::Ref { .. }
        | Content::CounterDisplay { .. }
        | Content::MathFrac { .. }
        | Content::MathAttach { .. }
        | Content::MathRoot { .. }
        | Content::MathDelimited { .. }
        | Content::MathMatrix { .. }
        | Content::MathCases { .. }
        | Content::MathAlignPoint
        | Content::Linebreak
        | Content::Raw { .. }
        | Content::ListItem(_)
        | Content::EnumItem { .. }
        | Content::Link { .. } => {},
    }
}
```

**Aviso de exaustividade:** a lista de terminais deve cobrir todas as
variantes de `Content`. Se o compilador emitir um aviso de
`non_exhaustive_patterns`, adicionar as variantes em falta ao braço
terminal. Não usar `_ => {}` como wildcard silencioso.

**Antes de compilar**, adicionar ao bloco de diagnósticos:

```bash
# Listar todas as variantes de Content para verificar cobertura do walk
grep -n "^\s*[A-Z][A-Za-z]*\b" 01_core/src/entities/content.rs | head -40
```

Se aparecerem variantes com filhos que não estejam no `walk` como braço
recursivo (ex: `Table`, `Grid`, `Metadata`), adicioná-las antes de
compilar. Uma variante de container não coberta faz a introspecção
ignorar silenciosamente todas as Labels dentro dela — bug difícil de
detectar em runtime.

Registar o módulo em `01_core/src/rules/mod.rs`:

```rust
pub mod introspect;
```

---

## Tarefa 3 — Actualizar o Pipeline (L1)

### 3a — Nova assinatura de `layout()`

A função `layout()` passa a aceitar um `CounterState` pré-calculado como
estado inicial. Isto substitui a criação interna de `CounterState::new()`:

```rust
/// Layout com estado de introspecção pré-calculado.
///
/// O `initial_state` deve ser produzido por `introspect::introspect(content)`
/// antes desta chamada. O Layouter usa apenas `resolved_labels` da introspecção
/// — os restantes campos do estado (contadores, flags de numeração) são
/// reconstituídos de novo durante o layout físico, porque a introspecção
/// "gastou" os contadores até ao fim do documento e o layout precisa de os
/// "gastar" de novo na ordem correcta para gerar os prefixos visuais.
pub fn layout(content: &Content, initial_state: CounterState) -> PagedDocument {
    let mut l = Layouter::new(/* métricas actuais */);
    // Iniciar com estado limpo e injectar apenas o mapa de labels resolvidas.
    // NÃO copiar hierarchical, flat, nem numbering_active — serão reconstruídos
    // nó a nó pelo Layouter, exactamente como no single-pass.
    // numbering_active será reposto pelos nós SetHeadingNumbering encontrados
    // durante o layout, na ordem correcta do documento.
    l.counter.resolved_labels = initial_state.resolved_labels;
    l.layout_content(content);
    l.finish()
}
```

**Porquê não copiar `numbering_active`:** a introspecção activou as flags
na ordem em que encontrou os `SetHeadingNumbering`. O Layouter também
encontrará esses nós na mesma ordem e activará as flags no momento certo.
Copiar as flags pré-activadas faria com que headings antes do `#set`
também fossem numerados — comportamento incorrecto.

**Porquê não copiar `hierarchical` e `flat`:** a introspecção avançou os
contadores até ao fim do documento (ex: heading counter = `[3]`). O Layouter
precisa de começar em `[]` e avançar nó a nó para saber que o primeiro heading
é `1`, o segundo é `2`, etc. Copiar os valores finais faria os prefixos
numéricos começar errados.

### 3b — Manter `layout_with_state` para testes

A função `layout_with_state` do Passo 57 continua útil para testes
unitários que precisam de injectar estado específico:

```rust
pub fn layout_with_state(content: &Content, state: CounterState) -> PagedDocument {
    // Sem reinicialização de contadores — o caller controla o estado.
    let mut l = Layouter::new(/* métricas actuais */);
    l.counter = state;
    l.layout_content(content);
    l.finish()
}
```

### 3c — Limpeza do braço `Labelled` no Layouter

O braço `Content::Labelled` no Layouter do Passo 59 inseria em
`resolved_labels` durante o layout visual. Com a pré-passagem, essa
inserção é redundante — o mapa já está populado. Remover a inserção
para evitar duplicação e potencial inconsistência entre as duas passagens:

```rust
Content::Labelled { target, label: _ } => {
    // Label registada pela introspecção — o Layouter apenas faz
    // o layout transparente do target.
    // NÃO inserir em resolved_labels aqui.
    self.layout_node(target, self.style)
},
```

### 3d — Orquestrar as duas passagens

Se existir uma função de composição (ex: em `rules/mod.rs`, em L3, ou
nas integration tests), actualizar para usar o novo pipeline:

```rust
// Padrão de uso (L3 ou testes):
let content = eval(&source, ...)?;

// Passagem 1 — Introspecção
let initial_state = introspect::introspect(&content);

// Passagem 2 — Layout físico
let document = layout::layout(&content, initial_state);
```

Se a orquestração acontece directamente nas integration tests, actualizar
`compile_to_pdf` em L3 para seguir este padrão.

---

## Tarefa 4 — Testes

### Testes L1 — Introspecção isolada

```rust
#[test]
fn introspect_popula_label_forward() {
    use crate::entities::{counter_state::CounterState, label::Label};
    use crate::rules::introspect::introspect;

    // Ref antes do Labelled — forward reference
    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::Ref { target: Label("conclusao".to_string()) },
        Content::Labelled {
            label:  Label("conclusao".to_string()),
            target: Box::new(Content::heading(1, Content::text("Conclusão"))),
        },
    ]);

    let state = introspect(&content);
    assert!(
        state.resolved_labels.contains_key(&Label("conclusao".to_string())),
        "introspect deve popular resolved_labels mesmo para forward refs"
    );
    assert_eq!(
        state.resolved_labels.get(&Label("conclusao".to_string())).map(|s| s.as_str()),
        Some("Secção 1")
    );
}

#[test]
fn introspect_counter_update_e_aplicado() {
    use crate::entities::counter_state::{CounterAction, CounterState};
    use crate::rules::introspect::introspect;

    let content = Content::Sequence(vec![
        Content::CounterUpdate {
            key:    "equation".to_string(),
            action: CounterAction::Update(5),
        },
    ]);

    let state = introspect(&content);
    assert_eq!(state.get_flat("equation"), 5);
}

#[test]
fn introspect_dois_conteudos_independentes() {
    use crate::entities::label::Label;
    use crate::rules::introspect::introspect;

    let content_a = Content::Labelled {
        label:  Label("a".to_string()),
        target: Box::new(Content::heading(1, Content::text("A"))),
    };
    let content_b = Content::Ref { target: Label("a".to_string()) };

    let state_a = introspect(&content_a);
    let state_b = introspect(&content_b);

    assert!(state_a.resolved_labels.contains_key(&Label("a".to_string())));
    assert!(!state_b.resolved_labels.contains_key(&Label("a".to_string())),
        "estados de introspecção devem ser independentes");
}
```

### Testes L1 — Pipeline duas passagens

```rust
#[test]
fn pipeline_duas_passagens_resolve_forward_ref() {
    use crate::entities::{counter_state::CounterState, label::Label};
    use crate::rules::{introspect::introspect, layout::layout};

    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        // Ref ANTES do Labelled — seria fallback @conclusao no single-pass
        Content::text("Ver a "),
        Content::Ref { target: Label("conclusao".to_string()) },
        Content::text(". "),
        Content::Labelled {
            label:  Label("conclusao".to_string()),
            target: Box::new(Content::heading(1, Content::text("Conclusão"))),
        },
    ]);

    // Passagem 1
    let initial_state = introspect(&content);
    assert!(initial_state.resolved_labels.contains_key(&Label("conclusao".to_string())));

    // Passagem 2
    let doc = layout(&content, initial_state);
    let text = doc.plain_text();

    assert!(
        text.contains("Secção 1"),
        "forward ref deve resolver para 'Secção 1': {:?}", text
    );
    assert!(
        !text.contains("@conclusao"),
        "não deve usar fallback com duas passagens: {:?}", text
    );
}
```

### Testes L3 — Pipeline completo

```rust
#[test]
fn pipeline_forward_ref_resolve_no_pdf() {
    let (world, _dir) = world_from_str(
        "#set heading(numbering: \"1.\")\n\
         Ver a @conclusao.\n\
         = Conclusão <conclusao>"
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty());
    // Verificação heurística — "Sec" pode ter encoding diferente no stream
    let pdf_str = String::from_utf8_lossy(&pdf);
    assert!(
        !pdf_str.contains("@conclusao"),
        "forward ref não deve aparecer como fallback no PDF"
    );
}

#[test]
fn pipeline_backward_ref_continua_a_funcionar() {
    // Regressão: garantir que backward refs não partiram com a mudança.
    let (world, _dir) = world_from_str(
        "#set heading(numbering: \"1.\")\n\
         = Metodologia <metodo>\n\
         De acordo com a @metodo."
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty());
    assert!(!String::from_utf8_lossy(&pdf).contains("@metodo"));
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
- [ ] Prompt L0 `introspect.md` criado, com `git add`, antes de qualquer
  código.
- [ ] `introspect.rs` criado com o header de linhagem correcto.
- [ ] `pub mod introspect;` registado em `rules/mod.rs`.
- [ ] `walk()` cobre todas as variantes de `Content` sem wildcard silencioso.
- [ ] `layout()` aceita `CounterState` como parâmetro; copia apenas
  `resolved_labels` para o Layouter; todos os outros campos do estado
  (contadores, flags de numeração) são reconstruídos nó a nó durante
  o layout físico.
- [ ] Braço `Labelled` no Layouter simplificado — remove inserção em
  `resolved_labels` (agora feita pela introspecção).
- [ ] Forward refs resolvem no teste L1 de duas passagens.
- [ ] Testes de regressão para backward refs continuam a passar.
- [ ] DEBT-10 marcado como encerrado em `01_core/DEBT.md`.
- [ ] DEBT-11 anotado em `01_core/DEBT.md` com a nota de que `introspect.rs`
  é o primeiro passo da decomposição de `layout.rs`.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `layout()` já aceitava `CounterState` ou se a mudança de assinatura
  quebrou call sites em L3.
- Se `Content::Styled` existe — e qual o nome exacto do campo filho
  (`child`, `body`, `content`, ou `target`).
- Se existem variantes container de `Content` além de `Styled` e `Strong`
  (ex: `Table`, `Grid`, `Metadata`) — e se foi necessário adicioná-las
  ao `walk`.
- Quantas variantes de `Content` existem actualmente (total confirmado).

**Da implementação:**
- Se a reinicialização de `hierarchical` e `flat` em `layout()` foi
  suficiente ou se surgiram efeitos secundários inesperados.
- Se a limpeza do braço `Labelled` no Layouter causou alguma regressão.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 61:**
- **GO — decomposição de `layout.rs`** (DEBT-11): com `introspect.rs`
  criado, o próximo passo natural é extrair `layout/references.rs` e
  `layout/counters.rs` como submódulos, ficando `layout.rs` como
  orquestrador. O prompt L0 de cada submódulo substitui as secções
  correspondentes do `layout.md` actual.
- **GO — Tabela de Conteúdos**: com `resolved_labels` global e forward
  refs resolvidas, Passo 61 gera uma TOC como `Content::Sequence` de
  `Ref` pré-resolvidas.
- **NO-GO — assinatura de `layout()` quebrou L3**: se a mudança de
  assinatura criou incompatibilidades não previstas em L3; Passo 61
  resolve as interfaces antes de avançar.
