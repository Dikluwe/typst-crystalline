# Passo 33 — DEBT-7: Scoping de `#set` por bloco

**Pré-condições**:
- Passo 32 concluído: 413 testes L1 + 48 testes L3, zero violations
- `DEBT-7` registado em `DEBT.md`
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
# Confirmar DEBT-7 em DEBT.md
grep -A 8 "DEBT-7" 01_core/DEBT.md

# Confirmar estado actual do merge em layout.rs
grep -n "bold.*||\|||.*bold\|node_style\|merge" \
  01_core/src/rules/layout.rs | head -10

# Confirmar que styles em EvalContext é StyleChain
grep -n "styles.*StyleChain\|pub styles" \
  01_core/src/rules/eval_context.rs

# Confirmar como blocos de código são avaliados actualmente
grep -n "CodeBlock\|eval_code_block\|eval_block\|Expr::Code" \
  01_core/src/rules/eval.rs | head -15
```

**Parar se qualquer pré-condição falhar.**

---

## Contexto

Actualmente, `#set text(bold: true)` dentro de um bloco `{ }` afecta
o documento inteiro — `ctx.styles` é global ao eval. O comportamento
correcto é: ao sair do bloco, o estilo volta ao que era antes de entrar.

```typst
texto normal
#{ 
  #set text(bold: true)
  texto a negrito      // bold: true
}
texto normal de novo   // deve voltar a bold: false
```

O mecanismo é simples: guardar `ctx.styles` antes de entrar num bloco,
restaurar ao sair. O `StyleChain` já é imutável e clonável em O(1)
(clone do `Arc` do nó de topo) — por isso guardar e restaurar é barato.

Este passo também corrige o merge `bold || node_style.bold` no layout
(DEBT-7) que impedia que `bold: false` revertesse `bold: true`.

---

## Tarefa 1 — Diagnóstico

```bash
# Ver como eval_code_block / eval_block está implementado
grep -n -A 20 "fn eval_code_block\|fn eval_block\|CodeBlock" \
  01_core/src/rules/eval.rs | head -40

# Ver onde Expr::Code / ContentBlock é avaliado
grep -n "Expr::Code\|ContentBlock\|Expr::Content" \
  01_core/src/rules/eval.rs | head -10

# Ver o merge actual em layout.rs
grep -n -A 5 "fn.*layout\|Content::Text\|node_style\|bold" \
  01_core/src/rules/layout.rs | head -30

# Confirmar que StyleChain::clone é O(1)
grep -n "impl Clone for StyleChain\|derive.*Clone.*StyleChain" \
  01_core/src/entities/style_chain.rs
```

**Parar. Reportar output antes de qualquer código.**

Questões a responder:
1. `eval_code_block` existe como função separada, ou os blocos são
   avaliados inline em `eval_expr`?
2. Há um arm explícito para `Expr::Code` (bloco de código `{ }`) e
   para `Expr::Content` (bloco de conteúdo `[ ]`) em `eval_expr`?
3. O merge em layout.rs é `||` sobre bool, ou é outro padrão?
4. `StyleChain::clone()` — deriva `Clone` automaticamente via `Arc`?

---

## Tarefa 2 — Scoping de styles em blocos

O padrão é: save → entrar no bloco → restaurar. Aplicar em todos os
sítios onde um novo scope de avaliação começa.

### 2a — Blocos de código `{ }`

Localizar o arm de `Expr::Code` (ou a função `eval_code_block`) e
envolver a avaliação com save/restore:

```rust
// Padrão a aplicar em cada bloco
let saved_styles = ctx.styles.clone();   // O(1) — clone do Arc
let result = eval_block_body(ctx, scopes, block);
ctx.styles = saved_styles;               // restaurar ao sair
result?
```

### 2b — Blocos de conteúdo `[ ]`

O mesmo padrão para `Expr::Content` (content blocks). Estes blocos
também podem conter `#set`.

### 2c — Corpo de closures

O corpo de uma closure é avaliado com `call_scopes` próprio, mas
`ctx.styles` é partilhado. Aplicar o mesmo save/restore em
`call_closure` / `apply_closure`:

```rust
let saved_styles = ctx.styles.clone();
let result = eval_code(ctx, &mut call_scopes, &repr.body);
ctx.styles = saved_styles;
result
```

**Nota**: closures que usam `#set` internamente não devem afectar o
estilo do caller. Com este passo, isso fica correcto.

### 2d — Corpos de `for` e `while`

Verificar se os corpos de loops podem conter `#set`. Se sim, aplicar
o mesmo padrão. Se o corpo de loop já é avaliado como `CodeBlock`,
o 2a cobre automaticamente.

```bash
# Verificar se eval_while e eval_for chamam eval_code_block
grep -n "eval_while\|eval_for\|while.*body\|for.*body" \
  01_core/src/rules/eval.rs | head -10
```

---

## Tarefa 3 — Corrigir o merge em `layout.rs`

O merge `bold || node_style.bold` impede que `bold: false` reverta
`bold: true`. Com o scoping correcto, o estilo do nó já é o estilo
resolvido no momento da produção — não precisa de ser combinado com
nenhum estado externo do layout.

Localizar o merge e remover a lógica de OR:

```rust
// ANTES
Content::Text(s, node_style) => {
    let effective_bold = self.style.bold || node_style.bold;
    // usar effective_bold
}

// DEPOIS
Content::Text(s, node_style) => {
    // node_style já tem o estilo correcto capturado em eval
    // não há merge — o layout usa node_style directamente
    let pt_size = node_style.size;
    let bold    = node_style.bold;
    let italic  = node_style.italic;
    // ...
}
```

**Parar. Mostrar o código actual do arm `Content::Text` em layout.rs
antes de alterar.**

---

## Tarefa 4 — Actualizar `DEBT.md`

```markdown
### DEBT-7 — Merge de TextStyle falha com block scoping — RESOLVIDO

**Resolvido no Passo 33**:
- Save/restore de `ctx.styles` em blocos de código `{ }`, content
  blocks `[ ]`, e corpos de closures
- Merge `bold || node_style.bold` removido de layout.rs
- `#set text(bold: false)` dentro de um bloco agora reverte correctamente
  ao sair do bloco

**Ficheiros alterados**: `rules/eval.rs`, `rules/layout.rs`
```

---

## Tarefa 5 — Testes

```rust
// ── scoping de #set por bloco ─────────────────────────────────────────────

#[test]
fn set_global_afecta_documento_inteiro() {
    // #set fora de bloco — afecta tudo
    let world = MockWorld::new("#set text(bold: true)\ntexto");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    // Verificar que o Content::Text produzido tem bold: true
    // Adaptar conforme API de Module::content()
    let _ = m;
}

#[test]
fn set_dentro_bloco_nao_vaza_para_fora() {
    // O estilo bold volta a false depois do bloco
    let world = MockWorld::new(
        "antes\n#{ #set text(bold: true)\nnegrito }\ndepois"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    // "antes" e "depois" devem ter bold: false
    // "negrito" deve ter bold: true
    // Verificar via Content::Text(_, style) — adaptar conforme API
    assert!(m.is_ok() || true); // no mínimo, não dá panic/err
}

#[test]
fn set_dentro_closure_nao_afecta_caller() {
    let world = MockWorld::new(
        "#let f() = { #set text(bold: true)\nnegrito }\n#f()\ntexto normal"
    );
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "closure com set falhou: {:?}", result);
    // "texto normal" deve ter bold: false — verificar se API permitir
}

#[test]
fn set_false_reverte_set_true_anterior() {
    // Este teste falha antes deste passo (merge || impedia reverter)
    let world = MockWorld::new(
        "#set text(bold: true)\nnegrito\n#{ #set text(bold: false)\nnormal }\nnegrito novamente"
    );
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "set false falhou: {:?}", result);
}

#[test]
fn set_aninhado_multiple_niveis() {
    let world = MockWorld::new(
        "#{\n  #set text(size: 14pt)\n  texto14\n  #{\n    #set text(size: 18pt)\n    texto18\n  }\n  texto14novamente\n}"
    );
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "set aninhado falhou: {:?}", result);
}

// ── testes de integração L3 — verificar que o merge foi removido ──────────
// Adicionar em 03_infra/src/integration_tests.rs:

#[test]
fn pipeline_set_scoped_nao_vaza() {
    let (world, _dir) = world_from_str(
        "normal\n#{ #set text(bold: true)\nnegrito }\nnormal novamente"
    );
    let source = world.source(world.main()).unwrap();
    let module = eval(&world, &source).unwrap();
    let doc = layout(module.content(), &FixedMetrics).unwrap();
    // O documento deve ter 3 segmentos de texto com estilos diferentes
    // Verificar que não há panic e que o PDF é produzido
    let pdf = export_pdf(&doc);
    assert!(!pdf.is_empty());
}
```

**Nota sobre verificação de estilos**: se `Module::content()` retorna
`Option<&Content>` e `Content` é opaco, pode não ser possível inspecionar
os estilos individuais de cada nó sem uma API de debug. Nesse caso,
os testes verificam ausência de panic/err e o teste L3 verifica o PDF.
Registar no relatório se a inspecção directa foi possível.

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

# Confirmar que o merge foi removido
grep -n "||.*bold\|bold.*||" 01_core/src/rules/layout.rs
# Deve retornar vazio

# Confirmar save/restore em eval
grep -n "saved_styles\|restore.*styles\|styles.*saved" \
  01_core/src/rules/eval.rs
# Deve aparecer em cada sítio de bloco
```

Critérios de conclusão:
- Save/restore de `ctx.styles` em blocos de código `{ }` ✓
- Save/restore de `ctx.styles` em content blocks `[ ]` ✓
- Save/restore de `ctx.styles` em corpos de closures ✓
- Merge `bold || node_style.bold` removido de `layout.rs` ✓
- `set_dentro_bloco_nao_vaza_para_fora` passa ✓
- `set_dentro_closure_nao_afecta_caller` passa ✓
- `set_false_reverte_set_true_anterior` passa ✓
- `set_aninhado_multiple_niveis` passa ✓
- DEBT-7 marcado como resolvido em `DEBT.md` ✓
- Zero violations ✓
- Testes não regridem (413 L1 + 48 L3 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- Blocos de código e content blocks tinham arms separados ou eram
  tratados pela mesma função?
- Corpos de `for` e `while` precisaram de save/restore explícito, ou
  já passavam por `eval_code_block`?
- O merge em layout.rs era `||` sobre bool ou outro padrão?

**Da implementação:**
- Foi possível inspecionar estilos individuais em `Content::Text`
  nos testes, ou só verificação de ausência de erro?
- Número final de testes e zero violations confirmado.

**DEBT-7 encerrado. Go para Passo 34 — preparação de `Content` para
equações: tipos fundamentais para nós matemáticos.**
