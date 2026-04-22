# Passo 62 — Motor de Figuras e Auto-Numeração (Fechamento do DEBT-10)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — Onde a nova variante será injectada.
- `01_core/src/rules/introspect.rs` — Onde o contador de figuras será gerido.
- `01_core/src/rules/layout/` — A nova estrutura de submódulos do Passo 61.

Pré-condição: `cargo test` — 625 L1 + 119 L3 + 50 parity, zero violations.
A decomposição de `layout.rs` e a TOC estão operacionais.

---

## Contexto

Em Typst, `#figure(body, caption: ...)` confere ao seu conteúdo um bloco
destacável, um contador independente (plano, não hierárquico) e a capacidade
de ser referenciado no texto via `@label`. O contador de figuras avança
automaticamente — sem intervenção manual do utilizador.

Este passo fecha o DEBT-10: com `Content::Figure` implementado, o sistema de
introspecção rastreia as figuras na Passagem 1, resolve as suas labels, e o
Layouter desenha o bloco com a legenda numerada na Passagem 2.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar se Figure já existe como vestígio no Content
grep -rn "Figure" 01_core/src/entities/content.rs | head -10

# 2. Verificar se o sistema de activação de numeração tem um nó
#    equivalente ao SetHeadingNumbering para figuras, ou se usa outro mecanismo
grep -n "SetHeadingNumbering\|numbering_active" \
  01_core/src/rules/eval.rs | head -10

# 3. Ver o estado actual dos braços em layout/mod.rs para confirmar
#    onde inserir o novo braço Figure
grep -n "Content::" 01_core/src/rules/layout/mod.rs | tail -20

# 4. Verificar se figure.rs já existe na pasta layout/
ls -l 01_core/src/rules/layout/
```

Reportar o output completo antes de continuar. A resposta à questão 2 é
crítica: se não existir um nó equivalente a `SetHeadingNumbering` para
figuras, a flag `numbering_active["figure"]` tem de ser activada por outro
mecanismo — provavelmente via `SetRule` no `eval.rs`, ou definida como
`true` por defeito no `CounterState`.

---

## Tarefa 1 — Prompt L0 (L0)

Criar o ficheiro de especificação antes de qualquer código:

`00_nucleo/prompts/rules/layout_figure.md`:

```markdown
# L0 — Layout: Figuras e Legendas

## Módulo
`01_core/src/rules/layout/figure.rs`

## Propósito
Encapsula o braço `Content::Figure` do Layouter. Responsável por desenhar
o corpo da figura e, se existir, a legenda (caption) numerada.

## Regras de negócio
- O contador visual avança em `step_flat("figure")` se `numbering_active["figure"]`.
- O prefixo da legenda segue o formato "Figura N: ".
- O corpo (`body`) é desenhado primeiro, seguido do prefixo e do `caption`.
- Figura sem caption não desenha prefixo numérico.
- Não escreve em `resolved_labels` — isso é responsabilidade de `introspect.rs`.
- A dupla contagem (introspecção + layout) é intencional: a Passagem 1 rastreia
  o estado final do documento; a Passagem 2 desenha os números iterativamente.

## Critérios de verificação
- Figura numerada com caption → prefixo "Figura 1: " antes do texto da legenda.
- Figura sem caption → sem prefixo numérico.
- Duas figuras numeradas → prefixos "Figura 1: " e "Figura 2: " correctos.
```

```bash
git add 00_nucleo/prompts/rules/layout_figure.md
crystalline-lint --fix-hashes .
```

---

## Tarefa 2 — Mecanismo de Activação de Numeração (L1)

O diagnóstico revelará se existe um nó `SetFigureNumbering` ou equivalente.
Se não existir, há duas opções:

**Opção A confirmada:** inicializar `numbering_active["figure"]` como `true` em
`CounterState::new()`. O método `is_numbering_active()` mantém-se estritamente
agnóstico — não pode conter lógica específica de domínio ("figure", "heading")
porque `CounterState` é infraestrutura genérica. A paridade com o Typst é
garantida no construtor, não no método de consulta:

```rust
impl CounterState {
    pub fn new() -> Self {
        let mut s = Self::default();
        // Figuras são numeradas por defeito — paridade com o Typst original.
        // O método is_numbering_active() não conhece esta regra; o construtor sim.
        s.numbering_active.insert("figure".to_string(), true);
        s
    }
}

// is_numbering_active permanece agnóstico:
pub fn is_numbering_active(&self, key: &str) -> bool {
    self.numbering_active.get(key).copied().unwrap_or(false)
}
```

Se algum código usar `CounterState::default()` em vez de `CounterState::new()`,
as figuras não serão numeradas — esse é o comportamento correcto para um estado
não inicializado. O construtor `new()` é o ponto de entrada canónico.

Registar DEBT-14 em `00_nucleo/DEBT.md`:

```markdown
### DEBT-14 — SetRule para figure(numbering: ...) (Passo 62)
A numeração de figuras está activa por defeito (Opção A). O utilizador não
consegue desactivá-la com `#set figure(numbering: none)` até que o braço
de SetRule para "figure" seja adicionado ao eval.rs, produzindo um nó
equivalente a SetHeadingNumbering. Quando implementado, as figuras sem
numeração cujas labels forem referenciadas mostrarão o fallback "@label"
(comportamento intencional — ver Tarefa 4 do Passo 62).
```

---

## Tarefa 2b — Registo de `figure()` na stdlib (L1)

**Esta tarefa é pré-requisito dos testes L3.** Se o `eval.rs` não reconhecer
`figure` como função nativa, o pipeline L3 falhará com "unknown function" antes
de chegar ao motor de layout.

Em `01_core/src/rules/stdlib.rs` (ou onde as funções nativas estão mapeadas),
registar `figure` no mesmo padrão das outras funções nativas:

```rust
// Na função que constrói o scope da stdlib:
scope.define("figure", Value::Func(Func::native(native_figure)));
```

Implementação da função nativa:

```rust
/// Função nativa `figure(body, caption: ...)`.
///
/// Extrai o argumento posicional obrigatório (body) e o argumento nomeado
/// opcional (caption), e produz Content::Figure.
fn native_figure(args: &[Value], named: &IndexMap<EcoString, Value>) -> SourceResult<Value> {
    // Argumento posicional: body (obrigatório)
    let body = match args.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(other) => Content::text(other.to_display_string()), // coerção simples
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "figure() requer um argumento posicional (body)".to_string(),
        )]),
    };

    // Argumento nomeado: caption (opcional)
    // Coerção defensiva: se o utilizador escrever caption: "string" em vez de
    // caption: [markup], o valor é Value::Str — não engolir silenciosamente.
    // Value::None interceptado explicitamente: caption: none → ausência de legenda,
    // não a string literal "none" no PDF.
    let caption = named.get("caption").and_then(|v| match v {
        Value::Content(c) => Some(Box::new(c.clone())),
        Value::Str(s)     => Some(Box::new(Content::text(s.as_str()))),
        Value::None       => None,
        other             => Some(Box::new(Content::text(other.to_display_string()))),
    });

    Ok(Value::Content(Content::Figure {
        body:    Box::new(body),
        caption,
    }))
}
```

**Nota sobre a assinatura:** ajustar os tipos dos parâmetros (`args`, `named`)
conforme a interface real das funções nativas no projecto — o padrão exacto
depende de como `native_figure` será registado (por ex: se a stdlib usa
`&Args` em vez de `&[Value]` + `&IndexMap`). Seguir o padrão das funções
nativas existentes em `stdlib.rs`.

Após adicionar, correr `cargo test` — o teste de integração das funções
nativas existentes deve continuar a passar.

---

## Tarefa 3 — `Content::Figure` (L1)

Em `01_core/src/entities/content.rs`, adicionar a variante (se não existir):

```rust
/// Elemento com numeração própria e legenda opcional.
Figure {
    body:    Box<Content>,
    caption: Option<Box<Content>>,
},
```

Actualizar `plain_text()`:

```rust
Content::Figure { body, caption } => {
    let body_text = body.plain_text();
    let cap_text  = caption.as_ref()
        .map(|c| c.plain_text())
        .unwrap_or_default();
    // Inserir separador apenas se ambas as partes têm conteúdo — evita
    // espaços pendurados quando body ou caption são nós sem texto puro
    // (ex: imagem futura que devolve "").
    match (body_text.is_empty(), cap_text.is_empty()) {
        (false, false) => format!("{} {}", body_text, cap_text),
        (false, true)  => body_text,
        (true,  false) => cap_text,
        (true,  true)  => String::new(),
    }
},
```

Actualizar `is_empty()`:

```rust
// Uma figura não está vazia se tiver body OU caption com conteúdo.
// Ex: corpo vazio com caption válida → não é vazio (imprime "Figura N: Legenda").
Content::Figure { body, caption } =>
    body.is_empty() && caption.as_ref().map_or(true, |c| c.is_empty()),
```

Adicionar `Content::Figure` ao braço terminal do `walk()` em `introspect.rs`
enquanto o braço real ainda não existe (para manter exaustividade):

```rust
| Content::Figure { .. } => {},
```

Correr `cargo test` — deve compilar sem warnings de exaustividade.

---

## Tarefa 4 — Introspecção das Figuras (L1)

Registar DEBT-15 em `00_nucleo/DEBT.md` antes de continuar:

```markdown
### DEBT-15 — Campo `kind` em Content::Figure (Passo 62)
A chave "figure" está hardcoded em step_flat("figure") tanto na introspecção
como no layout. No Typst original, #figure aceita um argumento `kind`
(ex: image, table, code), e cada kind tem contador próprio — "Tabela 1" e
"Figura 1" são independentes. Com a implementação actual, tabelas e gráficos
partilham o mesmo contador. Resolução: adicionar campo `kind: String` (default
"figure") a Content::Figure e usar step_flat(&kind) em vez da string fixa.
```

Em `01_core/src/rules/introspect.rs`, substituir o braço terminal por um
braço real para `Content::Figure`:

```rust
Content::Figure { body, caption } => {
    // Avançar o contador apenas se a figura tiver legenda — figuras sem caption
    // não consomem um número da sequência (evita dessincronização: "Figura 1",
    // [gap silencioso], "Figura 3").
    if state.is_numbering_active("figure") && caption.is_some() {
        state.step_flat("figure");
    }
    // Descer na árvore — pode haver Labels dentro do corpo ou da legenda.
    walk(body, state);
    if let Some(cap) = caption {
        walk(cap, state);
    }
},
```

### Actualizar o braço `Labelled` na introspecção

O braço `Content::Labelled` em `introspect.rs` precisa de formatar a
referência de uma figura. Adicionar o case `Content::Figure` ao match
interno:

```rust
Content::Labelled { target, label } => {
    walk(target, state);

    let resolved_text = match &**target {
        Content::Heading { .. } =>
            state.format_hierarchical("heading")
                .map(|n| format!("Secção {}", n)),

        Content::Equation { .. } => {
            let n = state.get_flat("equation");
            if n > 0 { Some(format!("Equação ({})", n)) } else { None }
        },

        Content::Figure { caption, .. } => {
            let n = state.get_flat("figure");
            // Se a figura tem caption e está numerada, resolver para "Figura N".
            // Se não tem caption (não consome contador — ver walk()), inserir
            // string vazia: a introspecção deve sempre registar a label encontrada.
            // String vazia → Ref resolve para nada (sem texto visível, sem "@label"
            // falso). Padrão consistente com títulos não numerados do Passo 61.
            if n > 0 && state.is_numbering_active("figure") && caption.is_some() {
                Some(format!("Figura {}", n))
            } else {
                Some(String::new())  // Label registada mas sem prefixo numérico
            }
        },

        _ => None,
    };

    // Registar sempre: a presença da chave em resolved_labels indica que a label
    // existe no documento. A ausência indica que a label não foi encontrada.
    // Não usar unwrap_or_default() para não misturar os dois casos.
    if let Some(text) = resolved_text {
        state.resolved_labels.insert(label.clone(), text);
    }
},
```

---

## Tarefa 5 — Submódulo `figure.rs` (L1)

Adicionar `pub mod figure;` em `layout/mod.rs` (junto com os outros `pub mod`).

Substituir o braço terminal `Content::Figure { .. } => {}` no Layouter por
delegação:

```rust
Content::Figure { body, caption } => figure::layout_figure(self, body, caption),
```

Criar `01_core/src/rules/layout/figure.rs`:

**Verificação prévia obrigatória — confirmar `flat.clear()` no pipeline:**
Antes de escrever `figure.rs`, confirmar que `layout/mod.rs` limpa os contadores
planos antes de iniciar o layout físico:

```bash
grep -n "flat.clear\|flat\.clear" 01_core/src/rules/layout/mod.rs | head -5
```

Se essa linha não existir ou tiver sido removida, a primeira figura na Passagem 2
receberá o número `N+1` em vez de `1` (porque a introspecção já avançou o contador
até `N`). Restaurar antes de continuar:

```rust
// Em layout/mod.rs, dentro de layout():
l.counter.flat.clear();        // ← obrigatório: reiniciar contadores planos
l.counter.hierarchical.clear(); // ← idem para hierárquicos
// NÃO limpar resolved_labels nem headings_for_toc — injectados da introspecção
```

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_figure.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-04-13

use crate::entities::content::Content;
// Importar Layouter e FontMetrics — ajustar o path conforme a estrutura real

/// Renderiza uma figura com legenda opcional.
///
/// A dupla contagem é intencional: a introspecção conta na Passagem 1 para
/// resolver labels; o Layouter conta aqui na Passagem 2 para gerar os prefixos
/// visuais iterativamente na ordem correcta.
pub fn layout_figure<M: FontMetrics>(
    layouter: &mut Layouter<M>,
    body:     &Content,
    caption:  &Option<Box<Content>>,
) {
    // 1. Desenhar o corpo da figura.
    layouter.layout_node(body, layouter.style);

    // 2. Avançar o contador visual apenas se a figura tiver legenda —
    // mesma regra da introspecção (Passagem 1) para manter sincronização.
    // Figuras sem caption não consomem número da sequência.
    let is_numbered = layouter.counter.is_numbering_active("figure") && caption.is_some();
    if is_numbered {
        layouter.counter.step_flat("figure");
    }

    // 3. Desenhar a legenda, se existir.
    if let Some(cap) = caption {
        layouter.layout_node(&Content::Linebreak, layouter.style);

        if is_numbered {
            let n = layouter.counter.get_flat("figure");
            // Agrupar prefixo e legenda num único nó para garantir que ficam
            // na mesma linha — dois layout_node consecutivos podem introduzir
            // uma quebra de linha indesejada entre "Figura 1: " e o texto.
            // ATENÇÃO DEBT-13: clonar `cap` aqui agrava o risco de duplicação
            // de side-effects (CounterUpdate dentro da legenda dispara duas vezes).
            // Ver DEBT-13 em DEBT.md — requer mecanismo de AST read-only futuro.
            let caption_block = Content::Sequence(vec![
                Content::text(format!("Figura {}: ", n)),
                *cap.clone(),
            ]);
            layouter.layout_node(&caption_block, layouter.style);
        } else {
            layouter.layout_node(cap, layouter.style);
        }
    }
}
```

---

## Tarefa 6 — Testes

**Aviso DEBT-13:** não usar `CounterUpdate` ou `CounterDisplay` dentro do
`body` de uma `Figure` nos testes deste passo — os resultados seriam
não-determinísticos enquanto DEBT-13 não estiver resolvido.

### Testes L1 — Introspecção de figuras

```rust
#[test]
fn introspect_resolve_label_de_figura() {
    use crate::entities::label::Label;
    use crate::rules::introspect::introspect;

    // Nota: numeração de figuras é true por defeito (Opção A) ou activada
    // pelo mecanismo confirmado no diagnóstico. Ajustar se necessário.
    let content = Content::Sequence(vec![
        Content::Labelled {
            label:  Label("fig1".to_string()),
            target: Box::new(Content::Figure {
                body:    Box::new(Content::text("Um gráfico")),
                caption: Some(Box::new(Content::text("Evolução"))),
            }),
        },
    ]);

    let state = introspect(&content);
    assert_eq!(
        state.resolved_labels.get(&Label("fig1".to_string())).map(|s| s.as_str()),
        Some("Figura 1"),
        "label de figura deve resolver para 'Figura 1'"
    );
}

#[test]
fn introspect_duas_figuras_contadores_independentes() {
    use crate::entities::label::Label;
    use crate::rules::introspect::introspect;

    let content = Content::Sequence(vec![
        Content::Labelled {
            label:  Label("f1".to_string()),
            target: Box::new(Content::Figure {
                body:    Box::new(Content::text("A")),
                caption: Some(Box::new(Content::text("Legenda A"))),
            }),
        },
        Content::Labelled {
            label:  Label("f2".to_string()),
            target: Box::new(Content::Figure {
                body:    Box::new(Content::text("B")),
                caption: Some(Box::new(Content::text("Legenda B"))),
            }),
        },
    ]);

    let state = introspect(&content);
    assert_eq!(
        state.resolved_labels.get(&Label("f1".to_string())).map(|s| s.as_str()),
        Some("Figura 1")
    );
    assert_eq!(
        state.resolved_labels.get(&Label("f2".to_string())).map(|s| s.as_str()),
        Some("Figura 2")
    );
}
```

### Testes L1 — Layout de figuras

```rust
#[test]
fn layout_figure_com_caption_tem_prefixo() {
    use crate::rules::introspect::introspect;
    use crate::rules::layout::layout;

    let content = Content::Figure {
        body:    Box::new(Content::text("Gráfico")),
        caption: Some(Box::new(Content::text("Resultados"))),
    };

    let state = introspect(&content);
    let doc = layout(&content, state);
    let text = doc.plain_text();

    assert!(text.contains("Gráfico"),    "corpo da figura deve aparecer");
    assert!(text.contains("Figura 1:"),  "prefixo numérico deve aparecer");
    assert!(text.contains("Resultados"), "legenda deve aparecer");
}

#[test]
fn layout_figure_sem_caption_sem_prefixo() {
    use crate::rules::introspect::introspect;
    use crate::rules::layout::layout;

    let content = Content::Figure {
        body:    Box::new(Content::text("Diagrama")),
        caption: None,
    };

    let state = introspect(&content);
    let doc = layout(&content, state);
    let text = doc.plain_text();

    assert!(text.contains("Diagrama"),    "corpo deve aparecer");
    assert!(!text.contains("Figura 1:"), "sem caption, sem prefixo");
}

#[test]
fn layout_ref_para_figura_resolve_corretamente() {
    use crate::entities::label::Label;
    use crate::rules::introspect::introspect;
    use crate::rules::layout::layout;

    let content = Content::Sequence(vec![
        Content::Labelled {
            label:  Label("fig1".to_string()),
            target: Box::new(Content::Figure {
                body:    Box::new(Content::text("Gráfico")),
                caption: Some(Box::new(Content::text("Legenda"))),
            }),
        },
        Content::text(" — ver "),
        Content::Ref { target: Label("fig1".to_string()) },
    ]);

    let state = introspect(&content);
    let doc = layout(&content, state);
    let text = doc.plain_text();

    assert!(text.contains("Figura 1"),
        "Ref para figura deve resolver para 'Figura 1': {:?}", text);
    assert!(!text.contains("@fig1"),
        "não deve usar fallback @fig1: {:?}", text);
}
```

### Testes L3 — Pipeline completo

```rust
#[test]
fn pipeline_figure_com_ref_gera_pdf() {
    // Usar bloco de texto em vez de image() — Content::Image pode não estar
    // implementado ainda e mascararia o sucesso do motor de figuras.
    let (world, _dir) = world_from_str(
        "#figure(\n  [Gráfico de Barras],\n  caption: [Resultados]\n) <fig1>\n\
         Como mostrado na @fig1."
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty(), "PDF com figure e ref não deve estar vazio");
}

#[test]
fn pipeline_figure_sem_ref_nao_causa_panico() {
    let (world, _dir) = world_from_str(
        "#figure(\n  [Conteúdo],\n  caption: [Legenda simples]\n)"
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty());
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
- [ ] Prompt L0 `layout_figure.md` criado e registado antes do código.
- [ ] `numbering_active["figure"] = true` em `CounterState::new()` (Opção A).
- [ ] DEBT-14 registado (SetRule para `#set figure(numbering: ...)`).
- [ ] DEBT-15 registado (campo `kind` hardcoded como "figure").
- [ ] `native_figure()` registada na stdlib; `cargo test` passa com teste simples
  de `#figure([Teste])` antes de avançar para o layout.
- [ ] `native_figure()` usa coerção defensiva para `caption` — não engole
  `Value::Str` ou outros tipos silenciosamente.
- [ ] `Content::Figure { body, caption }` adicionado ao enum.
- [ ] `cargo check` imediato após adicionar a variante — seguir erros de
  exaustividade em todos os ficheiros antes de continuar.
- [ ] `plain_text()` defensivo (sem espaços pendurados).
- [ ] `is_empty()` verifica body E caption.
- [ ] `walk()` em `introspect.rs` tem braço real para `Figure`.
- [ ] Braço `Labelled` verifica `caption.is_some()` antes de resolver "Figura N".
- [ ] `layout/figure.rs` criado com `pub(super)`.
- [ ] `pub mod figure;` declarado em `layout/mod.rs`.
- [ ] Dupla contagem documentada como intencional no código.
- [ ] DEBT-10 marcado como **encerrado** em `00_nucleo/DEBT.md`.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `Content::Figure` já existia como vestígio ou foi adicionado de raiz.
- Qual opção de activação de numeração foi usada (A ou B), e porquê.
- Se `layout/figure.rs` exigiu ajustes de visibilidade além de `pub(super)`.

**Da implementação:**
- Se o braço `Labelled` em `introspect.rs` para `Figure` devolveu `None`
  para figuras sem numeração — e se isso causou o fallback `@label` no PDF.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 63:**
- **GO — DEBT-12 (números de página na TOC):** com DEBT-10 encerrado, Passo 63
  implementa a terceira passagem que alimenta os números de página de volta à
  introspecção para a TOC.
- **GO — DEBT-13 (efeitos colaterais duplicados na TOC):** se o uso de
  contadores em títulos se revelar frequente nos testes do corpus, Passo 63
  implementa o mecanismo de congelamento de AST antes de avançar.
- **NO-GO — `Content::Figure` quebrou exaustividade do `walk`:** se o
  compilador reportar padrões não cobertos; Passo 63 corrige antes de avançar.
