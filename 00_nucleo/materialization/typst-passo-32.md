# Passo 32 — DEBT-6: Cobertura via TrackedWorld real

**Pré-condições**:
- Passo 31 concluído: 410 testes, zero violations
- `DEBT-6` visível em `DEBT.md`
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
# Confirmar DEBT-6 em DEBT.md
grep -A 8 "DEBT-6" 00_nucleo/DEBT.md

# Confirmar estrutura actual de eval_for_test
grep -n "eval_for_test\|fn eval_for_test\|MockWorld" \
  01_core/src/rules/eval.rs | head -20

# Confirmar que o merge bold || node_style.bold está registado como DEBT
grep -n "bold.*merge\|merge.*bold\|node_style.*bold\|DEBT.*bold\|scoping.*set" \
  00_nucleo/DEBT.md
# Se não aparecer, registar antes de continuar (ver Tarefa 0)

# Confirmar que #let f(n) = ... funciona (sintaxe do Passo 31)
grep -n "LetBindingKind\|Closure.*param\|params.*closure" \
  01_core/src/rules/eval.rs | head -10
```

**Parar se qualquer pré-condição falhar.**

---

## Tarefa 0 — Registar DEBT do merge bold (se não registado)

Se o `grep` acima não encontrar entrada para o merge `bold || node_style.bold`,
adicionar em `DEBT.md` antes de qualquer outro trabalho:

```markdown
### DEBT-7 — Merge bold em layout (Passo 30) — BAIXA

**Impacto**: quando `#set text(bold: false)` for scoped por bloco, o
`node_style.bold` no layout vai sobrescrever o estilo acumulado,
ignorando que o `#set` foi revogado ao sair do bloco.

**Condição de activação**: só é problema quando scoping de `#set` por
bloco for implementado. Enquanto `#set` for global ao eval, não há
divergência visível.

**Quando resolver**: junto com o scoping de `#set` por bloco.
**Ficheiros**: `01_core/src/rules/layout.rs`
```

---

## Contexto

DEBT-6 regista que `eval_for_test` usa `MockWorld` — um mundo artificial
que não passa pelo mecanismo de tracking real (`TrackedWorld`). O blind
spot é de **cobertura**, não de bug em `eval_for_test`: os testes de L1
nunca exercitam o caminho de código que será usado em produção.

**O que este passo faz**:
1. Audita o parsing de `#let f(x) = ...` — a sintaxe foi usada no Passo 31
   sem testes de parser explícitos. Adicionar esses testes agora.
2. Actualiza DEBT-2 com a divergência eager confirmada.
3. Regista DEBT-7 (merge bold).
4. Fecha DEBT-6: testes de integração em L3 com `SystemWorld` real.

**O que este passo não faz**: alterar `eval_for_test`. Continua a existir
para testes unitários rápidos de L1. A solução para DEBT-6 é *adicionar*
cobertura em L3, não modificar o que existe em L1.

---

## Tarefa 1a — Auditoria do parser: `#let f(x) = ...`

O teste `closure_recursiva_funciona` do Passo 31 usou a sintaxe
`#let fib(n) = ...`. Verificar se há testes explícitos para esta estrutura
no parser.

```bash
# Procurar testes de parser para LetBinding com parâmetros
grep -rn "let_binding\|parse_let\|LetBinding\|#let.*(.*).*=" \
  01_core/src/rules/parse.rs 2>/dev/null | grep -i "test\|#\[" | head -10

# Ver a estrutura AST gerada para #let f(x) = x
# (confirmar que gera LetBinding com Closure, não LetBinding com Init)
grep -n "LetBindingKind\|LetBinding\|Closure" \
  01_core/src/entities/ast/code.rs | head -20
```

**Se não houver testes de parser para esta estrutura**, adicionar em
`01_core/src/rules/parse.rs` dentro de `#[cfg(test)]`:

```rust
#[test]
fn parse_let_funcao_com_parametros() {
    // #let f(x, y) = x + y deve gerar LetBinding com Closure no body
    use crate::rules::parse::parse;
    let node = parse("#let f(x, y) = x + y");
    // Verificar que não há erros de parse
    assert!(
        node.errors().is_empty(),
        "parse de #let f(x,y) gerou erros: {:?}", node.errors()
    );
    // Verificar que a estrutura é LetBinding (não outro nó)
    // A forma exacta depende da API de SyntaxNode — adaptar conforme necessário
}

#[test]
fn parse_let_funcao_sem_parametros() {
    use crate::rules::parse::parse;
    let node = parse("#let f() = 42");
    assert!(node.errors().is_empty());
}

#[test]
fn parse_let_funcao_recursiva() {
    use crate::rules::parse::parse;
    let node = parse("#let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }");
    assert!(node.errors().is_empty());
}
```

**Parar. Reportar se os testes existiam ou foram adicionados.**

---

## Tarefa 1 — Diagnóstico

```bash
# Ver MockWorld em detalhe
grep -n "struct MockWorld\|impl MockWorld\|impl World for MockWorld\|impl TrackedWorld" \
  01_core/src/rules/eval.rs | head -20

# Ver SystemWorld — já implementa TrackedWorld?
grep -n "impl TrackedWorld for SystemWorld\|impl World for SystemWorld" \
  03_infra/src/world.rs

# Ver testes existentes em L3
ls 03_infra/src/
grep -rn "#\[test\]\|#\[cfg(test)\]" 03_infra/src/ | head -20

# Ver como SystemWorld é construído
grep -n "SystemWorld::new\|fn new" 03_infra/src/world.rs | head -10

# Ver eval() pública — é chamável de L3?
grep -n "pub fn eval\b" 01_core/src/rules/eval.rs
```

**Parar. Reportar output antes de qualquer código.**

Questões a responder:
1. `MockWorld` implementa `TrackedWorld` directamente ou há um wrapper?
2. `SystemWorld` já implementa `TrackedWorld`, ou só `World`?
3. Há testes em `03_infra` ou é o primeiro?
4. `eval()` pública aceita qualquer `impl TrackedWorld` ou está hardcoded para `MockWorld`?

---

## Tarefa 2 — Garantir que `eval()` aceita qualquer `TrackedWorld`

A assinatura pública de `eval` deve ser genérica sobre o world:

```rust
// Em 01_core/src/rules/eval.rs

pub fn eval<W: TrackedWorld>(
    world: &W,
    source: &Source,
) -> SourceResult<Module> {
    let mut ctx = EvalContext::new(world);
    let mut scopes = Scopes::new(make_stdlib());
    eval_markup(&mut ctx, &mut scopes, source.root())
}
```

Se a assinatura actual já é genérica, confirmar e avançar.
Se está hardcoded para `MockWorld` ou outro tipo concreto, corrigir aqui.

---

## Tarefa 3 — Testes de integração em L3

Criar `03_infra/src/integration_tests.rs` (ou adicionar a um ficheiro
de testes existente se já houver):

```rust
// 03_infra/src/integration_tests.rs
// Testes de integração: pipeline completo via SystemWorld real.
// Estes testes exercitam o caminho de código de produção (DEBT-6).

#[cfg(test)]
mod integration {
    use std::path::PathBuf;
    use typst_core::rules::eval::eval;
    use typst_core::rules::layout::layout;
    use crate::export::{export_pdf, export_pdf_with_font};
    use crate::world::SystemWorld;

    /// Cria um SystemWorld com um ficheiro .typ temporário contendo `src`.
    fn world_from_str(src: &str) -> (SystemWorld, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let main = dir.path().join("main.typ");
        std::fs::write(&main, src).unwrap();
        let world = SystemWorld::new(
            dir.path().to_path_buf(),
            main,
            vec![],
        ).unwrap();
        (world, dir)
    }

    #[test]
    fn pipeline_texto_simples() {
        let (world, _dir) = world_from_str("Olá, mundo!");
        let source = world.source(world.main()).unwrap();
        let module = eval(&world, &source).unwrap();
        let doc = layout(module.content(), &typst_core::rules::layout::FixedMetrics).unwrap();
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn pipeline_export_pdf_helvetica() {
        let (world, _dir) = world_from_str("Texto simples.");
        let source = world.source(world.main()).unwrap();
        let module = eval(&world, &source).unwrap();
        let doc = layout(module.content(), &typst_core::rules::layout::FixedMetrics).unwrap();
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
        // PDF começa com %PDF-
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_export_pdf_com_fonte_real() {
        // Requer pelo menos uma fonte no sistema.
        // Se não houver fontes, o teste é ignorado via early return.
        let (world, _dir) = world_from_str("Texto com fonte real.");
        let source = world.source(world.main()).unwrap();
        let module = eval(&world, &source).unwrap();
        let doc = layout(module.content(), &typst_core::rules::layout::FixedMetrics).unwrap();

        // Tentar obter dados de fonte real
        if let Some(font_data) = world.book().families().next()
            .and_then(|(_, infos)| infos.first())
            .and_then(|info| world.font(info.index))
            .map(|f| f.data().clone())
        {
            let pdf = export_pdf_with_font(&doc, &font_data);
            assert!(!pdf.is_empty());
            assert_eq!(&pdf[..5], b"%PDF-");
        }
        // Se não há fontes, o teste passa silenciosamente
    }

    #[test]
    fn pipeline_com_set_text_bold() {
        let (world, _dir) = world_from_str("#set text(bold: true)\nTexto a negrito.");
        let source = world.source(world.main()).unwrap();
        let module = eval(&world, &source).unwrap();
        let doc = layout(module.content(), &typst_core::rules::layout::FixedMetrics).unwrap();
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn pipeline_com_closures() {
        let src = "#let saudacao(nome) = \"Olá, \" + nome\n#saudacao(\"Mundo\")";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = eval(&world, &source).unwrap();
        let doc = layout(module.content(), &typst_core::rules::layout::FixedMetrics).unwrap();
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn pipeline_eval_retorna_err_em_sintaxe_invalida() {
        let (world, _dir) = world_from_str("#let x = ");  // incompleto
        let source = world.source(world.main()).unwrap();
        // Pode ser Err de parse ou de eval — ambos são aceitáveis
        // O importante é não entrar em panic
        let _ = eval(&world, &source);
    }
}
```

**Nota sobre `tempfile`**: verificar se `tempfile` já é dependência de
`03_infra`. Se não for, adicionar em `Cargo.toml` de `03_infra` como
`dev-dependency`:

```toml
[dev-dependencies]
tempfile = "3"
```

Uma ADR não é necessária para `dev-dependencies` que não entram em
produção — mas registar no relatório.

---

## Tarefa 4 — Expor `module.content()` se necessário

Os testes de integração chamam `module.content()`. Verificar se `Module`
já expõe o `Content` produzido pelo eval:

```bash
grep -n "fn content\|pub.*content\|Content" 01_core/src/entities/module.rs
```

Se não existir, adicionar:

```rust
impl Module {
    pub fn content(&self) -> &Content {
        &self.0.content
    }
}
```

Reportar se `ModuleInner` já tem um campo `content: Content`.

---

## Tarefa 5 — Actualizar `DEBT.md`

```markdown
### DEBT-6 — eval_for_test coverage blind spot — RESOLVIDO

**Resolvido no Passo 32**:
- Testes de integração em `03_infra/src/integration_tests.rs`
- Pipeline completo exercitado: SystemWorld → eval → layout → export_pdf
- `eval()` pública confirmada como genérica sobre `TrackedWorld`
- `eval_for_test` mantida para testes unitários rápidos de L1

**Cobertura adicionada**:
- `pipeline_texto_simples`: eval + layout via SystemWorld
- `pipeline_export_pdf_helvetica`: export com fallback Helvetica
- `pipeline_export_pdf_com_fonte_real`: export com fonte do sistema
- `pipeline_com_set_text_bold`: StyleChain via pipeline real
- `pipeline_com_closures`: closures via pipeline real
```

Actualizar também DEBT-2 com o resultado confirmado do Passo 31:

```markdown
### DEBT-2 — Closures eager vs lazy capture — PARCIALMENTE RESOLVIDO

[... texto existente ...]

**Confirmado no Passo 31**: captura via snapshot de `IndexMap` —
`#let x = 1; #let f() = x; #let x = 2; f()` retorna `1` (não `2`).
O snapshot é uma cópia independente, não uma referência partilhada.
Divergência semântica documentada com o original. Não bloqueante.
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

# Confirmar que testes de integração estão em L3, não L1
grep -rn "SystemWorld" 01_core/src/ 2>/dev/null
# Deve retornar vazio — SystemWorld não deve aparecer em L1

# Confirmar que eval() é genérica
grep -n "pub fn eval" 01_core/src/rules/eval.rs
```

Critérios de conclusão:
- `eval()` pública é genérica sobre `impl TrackedWorld` ✓
- `03_infra/src/integration_tests.rs` criado com 6 testes de pipeline ✓
- `tempfile` adicionado como `dev-dependency` de `03_infra` se necessário ✓
- `module.content()` exposto em `Module` ✓
- DEBT-6 marcado como resolvido em `DEBT.md` ✓
- Testes de parser para `#let f(x) = ...` existem ou foram adicionados ✓
- DEBT-2 actualizado com divergência eager confirmada ✓
- Zero violations ✓
- Testes não regridem (410 base + novos) ✓
- `SystemWorld` não aparece em `01_core/src/` ✓

---

## Ao terminar, reportar

**Da auditoria do parser:**
- `#let f(x) = ...` já tinha testes de parser, ou foram adicionados neste passo?
- A estrutura AST gerada é `LetBinding` com `Closure` no body — confirmar.

**Do diagnóstico e implementação do DEBT-6:**
- `eval()` já era genérica ou foi necessário alterar?
- `SystemWorld` já implementava `TrackedWorld` ou só `World`?
- `Module` já tinha `content()` ou foi adicionado?
- `tempfile` já era dependência ou foi adicionada?

**Da implementação:**
- Algum teste de integração falhou por razão diferente do esperado?
- Número final de testes (L1 + L3) e zero violations confirmado.

**DEBT-6 encerrado. Todos os DEBTs do roadmap inicial (1–6) resolvidos
ou em progresso documentado. Go para Passo 33 — próxima fase do roadmap.**
