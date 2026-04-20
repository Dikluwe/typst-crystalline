# Passo 71 — Suporte a Imagens e Acesso a Ficheiros (DEBT-24)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/func.rs` — Assinatura actual de `NativeFunc`.
- `01_core/src/rules/eval.rs` — Onde `apply_func` chama `native.call` e onde
  `EvalContext` está definido.
- `01_core/src/entities/content.rs` — Onde `Content::Image` será adicionado.
- `01_core/src/rules/stdlib.rs` — Todas as funções nativas actuais, para
  actualizar as assinaturas.

Pré-condição: `cargo test` — 694 L1 + 125 L3, zero violations.
DEBT-20 e DEBT-23 encerrados. `active_guards` funcional.

---

## Contexto

Para processar `#image("caminho.png")` surgem três problemas novos:

**Acesso ao disco (DEBT-24):** as funções nativas recebem apenas `&Args` e não
têm acesso ao `World` para ler ficheiros. A assinatura de `NativeFunc` precisa
de ser alterada para receber `&mut EvalContext`.

**Gestão de memória:** `Content` é clonado frequentemente. Guardar `Vec<u8>`
directamente faria cada clone copiar todos os bytes da imagem. A solução é
`Arc<Vec<u8>>` — os clones partilham a mesma alocação.

**Dimensões físicas:** se a crate `image` não estiver disponível em L1, o
layouter usa dimensões fixas temporárias. Não instalar a crate só para este
passo.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Verificar se a crate 'image' existe no Cargo.toml de L1
grep -n "^image\|^image " 01_core/Cargo.toml

# 2. Confirmar a assinatura actual de NativeFunc
grep -n "pub struct NativeFunc\|pub call" 01_core/src/entities/func.rs -A 4 | head -15

# 3. Confirmar a linha exacta onde native.call é invocada em eval.rs
grep -n "native.call\|\.call)(" 01_core/src/rules/eval.rs | head -10

# 4. Confirmar se World tem um método de leitura de ficheiros
grep -n "fn file\|fn read\|fn asset" 01_core/src/entities/world.rs 2>/dev/null \
  || grep -rn "fn file\|fn read\|fn asset" 01_core/src/ | head -10

# 5. Listar todas as funções nativas actuais (para actualizar assinaturas)
grep -n "^fn native_" 01_core/src/rules/stdlib.rs
```

Reportar o output completo antes de continuar.

O diagnóstico 1 determina se as dimensões da imagem podem ser lidas ou se são
mockadas. O diagnóstico 4 determina o nome exacto do método a usar em
`ctx.world` — se não existir, a Tarefa 0 adiciona-o antes de qualquer outra
coisa. O diagnóstico 5 lista todas as funções a actualizar na Tarefa 2.

---

## Tarefa 0 — Método de leitura de ficheiros no trait `World` (L1)

Se o diagnóstico 4 confirmar que o trait `World` não tem método de leitura de
ficheiros, adicioná-lo antes de qualquer outra tarefa:

```rust
// Em 01_core/src/entities/world.rs (ou onde World está definido):

/// Lê o conteúdo de um ficheiro pelo caminho relativo à raiz do projecto.
/// Retorna os bytes do ficheiro ou um erro se o ficheiro não existir.
fn file(&self, path: &str) -> Result<std::sync::Arc<Vec<u8>>, String>;
```

Implementar em `MockWorld` (para testes):

```rust
// MockWorld precisa de um campo para ficheiros simulados:
pub struct MockWorld {
    // ... campos existentes ...
    files: std::collections::HashMap<String, std::sync::Arc<Vec<u8>>>,
}

impl MockWorld {
    /// Regista um ficheiro simulado para uso nos testes.
    pub fn add_file(&mut self, path: &str, data: Vec<u8>) {
        self.files.insert(path.to_string(), std::sync::Arc::new(data));
    }
}

impl World for MockWorld {
    fn file(&self, path: &str) -> Result<std::sync::Arc<Vec<u8>>, String> {
        self.files.get(path)
            .map(|v| std::sync::Arc::clone(v))
            .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
    }
}
```

Implementar em `SystemWorld` (L3, em `01_infra/src/world.rs`):

```rust
impl World for SystemWorld {
    fn file(&self, path: &str) -> Result<std::sync::Arc<Vec<u8>>, String> {
        let full_path = self.root.join(path);
        std::fs::read(&full_path)
            .map(std::sync::Arc::new)
            .map_err(|e| format!("erro ao ler '{}': {}", path, e))
    }
}
```

Se o trait `World` usa `comemo::Tracked`, verificar se `file()` precisa de ser
anotado com `#[comemo::memoize]` ou similar — seguir o padrão dos outros
métodos existentes.

---

## Tarefa 1 — Alterar a assinatura de `NativeFunc` (L1)

Em `01_core/src/entities/func.rs`, alterar o campo `call`:

```rust
// ANTES
pub call: fn(&Args) -> SourceResult<Value>,

// DEPOIS
pub call: fn(&mut crate::rules::eval_context::EvalContext<'_>, &Args) -> SourceResult<Value>,
```

Ajustar o caminho de `EvalContext` conforme o diagnóstico 3 — pode ser
`crate::rules::eval::EvalContext` se não existir um ficheiro separado.

Em `eval.rs`, na linha onde `native.call` é invocada (diagnóstico 3):

```rust
// ANTES
FuncRepr::Native(native) => (native.call)(&args),

// DEPOIS
FuncRepr::Native(native) => (native.call)(ctx, &args),
```

**Após esta alteração, o compilador vai falhar em todas as funções nativas
de `stdlib.rs`.** Isso é esperado — a Tarefa 2 corrige todas de uma vez.

---

## Tarefa 2 — Actualizar assinaturas das funções nativas (L1)

Em `01_core/src/rules/stdlib.rs`, adicionar `_ctx: &mut EvalContext<'_>` como
primeiro parâmetro em **todas** as funções nativas existentes. A lista exacta
vem do diagnóstico 5. Exemplo do padrão:

```rust
// ANTES
fn native_upper(args: &Args) -> SourceResult<Value> { ... }

// DEPOIS
fn native_upper(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> { ... }
```

Funções a actualizar (completar com a lista do diagnóstico 5):
- `native_upper` → `_ctx: &mut EvalContext<'_>`
- `native_lower` → `_ctx: &mut EvalContext<'_>`
- `native_replace` → `_ctx: &mut EvalContext<'_>`
- `native_assert` → `_ctx: &mut EvalContext<'_>`
- `native_figure` → `_ctx: &mut EvalContext<'_>`
- `native_heading` (sentinela, se existir) → `_ctx: &mut EvalContext<'_>`
- `native_strong` → `_ctx: &mut EvalContext<'_>`
- `native_emph` → `_ctx: &mut EvalContext<'_>`
- `native_raw` → `_ctx: &mut EvalContext<'_>`
- Qualquer outra função listada pelo diagnóstico 5.

O prefixo `_` no nome do parâmetro suprime o warning de variável não usada
sem remover a capacidade de aceder ao contexto no futuro.

Após actualizar todas as funções, `cargo check` deve compilar sem erros.

---

## Tarefa 3 — `Content::Image` (L1)

Em `01_core/src/entities/content.rs`, adicionar a variante ao enum:

```rust
/// Imagem carregada do disco. Arc para que clones do AST partilhem
/// a mesma alocação de memória sem copiar os bytes.
///
/// `width` e `height` preservam a intenção semântica do utilizador
/// (ex: `#image("f.png", width: 50pt)`). O layouter pode ignorá-los
/// neste passo (placeholder 100×100), mas o AST não descarta a informação.
Image {
    path:   String,
    data:   std::sync::Arc<Vec<u8>>,
    width:  Option<Value>,
    height: Option<Value>,
},
```

Actualizar os métodos afectados:

```rust
// plain_text: imagem não tem texto legível
Content::Image { .. } => String::new(),

// is_empty (se existir): imagem ocupa espaço, não é vazia
Content::Image { .. } => false,

// map_content: terminal — sem filhos Content
Content::Image { .. } => self.clone(),

// map_text: terminal — sem texto a transformar
Content::Image { .. } => self.clone(),

// materialize_time (Passo 66): terminal — sem CounterDisplay a congelar
Content::Image { .. } => self.clone(),
```

**Aviso de exaustividade:** se o compilador emitir `non_exhaustive_patterns`
nalgum `match` de `Content`, adicionar `Content::Image { .. } => self.clone()`
(ou o equivalente para o método em questão) ao grupo de terminais. Nunca
usar `_ =>`.

**Layouter (DEBT-24b) — obrigatório neste passo:** ao adicionar `Content::Image`
ao enum, o `match` principal do layouter (`layout_content` ou equivalente em
`01_core/src/rules/layout/mod.rs`) também deixará de compilar. Adicionar o
braço com um placeholder de dimensões fixas:

```rust
Content::Image { .. } => {
    // DEBT-24b: dimensões reais da imagem não suportadas neste passo.
    // Placeholder de 100×100 pontos para não bloquear o layout.
    // Passo 72 implementará leitura real de dimensões via crate image (L3)
    // ou heurística baseada nos bytes (L1).
    let width  = Pt(100.0);
    let height = Pt(100.0);
    // Inserir um FrameItem ou avançar o cursor pelo espaço ocupado —
    // seguir o padrão dos outros nós visuais já existentes no layouter.
    layouter.advance_block(width, height);
},
```

Se o layouter não tiver `advance_block`, usar o padrão mais próximo dos
braços existentes (ex: `Content::Figure`, `Content::Space`). O objectivo
é que o compilador aceite a variante e o layout não entre em panic — as
dimensões correctas são trabalho do Passo 72.

Registar em `DEBT.md`:

```markdown
### DEBT-24b — Dimensões reais de imagem (Passo 71)
Content::Image no layouter usa placeholder 100×100 pt.
Resolução: Passo 72 adiciona leitura de dimensões reais via bytes
do Arc<Vec<u8>> (heurística de cabeçalho PNG/JPEG) ou via crate image em L3.

### DEBT-25 — Resolução de caminhos relativos na stdlib (Passo 71)
SystemWorld::file resolve caminhos relativos à raiz do projecto (`root.join(path)`).
Isto está incorrecto quando a função é invocada a partir de um ficheiro fonte
que não está na raiz — ex: `capitulos/intro.typ` com `#image("foto.png")`
procura `root/foto.png` em vez de `root/capitulos/foto.png`.
Resolução: EvalContext precisa de saber o FileId ou caminho do ficheiro
actualmente em avaliação. World::file deve receber esse contexto para
resolver o caminho relativo ao ficheiro fonte, não à raiz do projecto.

### DEBT-26 — PartialEq exaustivo em Content::Image (Passo 71)
Content deriva PartialEq. Arc<Vec<u8>> comparado com == desreferencia e
compara byte a byte — O(N) para imagens grandes. Em documentos com múltiplas
imagens ou no algoritmo de fixpoint (Passo 65), isto pode paralisar a thread.
Resolução: criar PtrEqArc<T> que implementa PartialEq via Arc::ptr_eq
(comparação de endereço, O(1)). Substituir Arc<Vec<u8>> por PtrEqArc<Vec<u8>>
em Content::Image em Passo 72 ou 73.
```

---

## Tarefa 4 — `native_image` na stdlib (L1)

Em `01_core/src/rules/stdlib.rs`:

```rust
fn native_image(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    // Apenas "width" e "height" são aceites como named args neste passo.
    // Outros named args geram erro explícito.
    for key in args.named.keys() {
        if key.as_str() != "width" && key.as_str() != "height" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em image(): '{}'", key),
            )]);
        }
    }

    let path = match args.items.first() {
        Some(Value::Str(s)) => s.to_string(),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("image() requer string com o caminho, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "image() requer 1 argumento posicional (caminho do ficheiro)".to_string(),
        )]),
    };

    // Ler o ficheiro através do World. O método exacto depende do diagnóstico 4.
    // World::file já retorna Arc<Vec<u8>> — O(1) clone do ponteiro, não cópia dos bytes.
    let data = match ctx.world.file(&path) {
        Ok(arc) => arc,
        Err(msg) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("image(): não foi possível ler '{}': {}", path, msg),
        )]),
    };

    // Extrair dimensões opcionais. Ignoradas pelo layouter neste passo
    // (placeholder 100×100), mas preservadas no AST para o Passo 72.
    let width  = args.named.get("width".into()).cloned();
    let height = args.named.get("height".into()).cloned();

    Ok(Value::Content(Content::Image { path, data, width, height }))
}
```

Registar no scope:

```rust
scope.define("image", Value::Func(Func::native("image", native_image)));
```

**Nota sobre o layouter:** a variante `Content::Image` chegará ao layouter sem
dimensões. O layouter deve usar um tamanho fixo padrão (ex: 100×100 pontos)
até que a leitura de dimensões seja implementada. Registar como DEBT-24b se
ainda não estiver registado.

---

## Tarefa 5 — Testes

### Testes L1 — `native_image`

```rust
#[test]
fn eval_image_le_ficheiro_para_content() {
    let mut world = MockWorld::new("#image(\"teste.png\")");
    world.add_file("teste.png", vec![1, 2, 3, 4]);

    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();

    let content = module.content().unwrap();
    match content {
        Content::Image { path, data, width, height } => {
            assert_eq!(path, "teste.png");
            assert_eq!(data.len(), 4);
            assert!(width.is_none(), "sem width esperado");
            assert!(height.is_none(), "sem height esperado");
        },
        other => panic!("Esperado Content::Image, obtido {:?}", other),
    }
}

#[test]
fn eval_image_ficheiro_inexistente_gera_erro() {
    let world = MockWorld::new("#image(\"nao_existe.png\")");
    // MockWorld sem add_file — ficheiro não existe.
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err(),
        "image() com ficheiro inexistente deve gerar Err");
    let err = result.unwrap_err();
    assert!(err[0].message.contains("nao_existe.png"),
        "Mensagem de erro deve incluir o caminho: {:?}", err[0].message);
}

#[test]
fn eval_image_rejeita_named_arg_invalido() {
    let mut world = MockWorld::new("#image(\"a.png\", bla: 1)");
    world.add_file("a.png", vec![0]);
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err());
    assert!(result.unwrap_err()[0].message.contains("bla"));
}

#[test]
fn content_image_arc_partilhado_em_clone() {
    // Confirmar que clonar Content::Image não copia os bytes.
    let data = std::sync::Arc::new(vec![1u8, 2, 3, 4]);
    let c1 = Content::Image {
        path:   "a.png".to_string(),
        data:   data.clone(),
        width:  None,
        height: None,
    };
    let c2 = c1.clone();

    // Arc::ptr_eq confirma que ambos apontam para a mesma alocação.
    if let (Content::Image { data: d1, .. }, Content::Image { data: d2, .. }) = (&c1, &c2) {
        assert!(std::sync::Arc::ptr_eq(d1, d2),
            "Clone de Content::Image deve partilhar a mesma alocação Arc");
    } else {
        panic!("Clones não são Content::Image");
    }
}
```

### Teste L3 — Pipeline com imagem

```rust
#[test]
fn pipeline_image_sem_panic() {
    // Confirmar que um documento com #image() passa pelo pipeline completo
    // sem panic, mesmo que o layouter use dimensões fixas.
    let root = criar_dir_temporario();
    std::fs::write(root.join("teste.png"), &[1u8, 2, 3, 4]).unwrap();

    let world = SystemWorld::new(&root, "main.typ", &[]);
    // Escrever o ficheiro principal
    std::fs::write(root.join("main.typ"), "#image(\"teste.png\")").unwrap();

    let src = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    // Não deve entrar em panic — as dimensões fixas são aceitáveis.
    assert!(!doc.pages.is_empty());
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
- [ ] Método `file()` adicionado ao trait `World`, `MockWorld` e `SystemWorld`
  (se não existia — confirmar com diagnóstico 4).
- [ ] `NativeFunc.call` actualizado para receber `&mut EvalContext<'_>`.
- [ ] Chamada em `eval.rs` actualizada para passar `ctx`.
- [ ] Todas as funções nativas em `stdlib.rs` recebem `_ctx: &mut EvalContext<'_>`
  como primeiro parâmetro (lista completa do diagnóstico 5).
- [ ] `Content::Image { path, data: Arc<Vec<u8>> }` adicionado ao enum.
- [ ] `plain_text`, `is_empty`, `map_content`, `map_text`, `materialize_time`
  actualizados com o braço `Content::Image`. Sem `_ =>`.
- [ ] Layouter actualizado com braço `Content::Image { .. }` de placeholder
  100×100 pt. DEBT-24b registado em `DEBT.md`.
- [ ] `World::file` retorna `Arc<Vec<u8>>` — não `Vec<u8>`. `native_image`
  não cria `Arc` (recebe-o directamente do `World`).
- [ ] `native_image` implementada com validação estrita de named args e leitura
  via `ctx.world.file()`.
- [ ] `image` registado no scope da stdlib.
- [ ] Testes `eval_image_le_ficheiro_para_content` e
  `eval_image_ficheiro_inexistente_gera_erro` passam.
- [ ] Teste `content_image_arc_partilhado_em_clone` confirma que `Arc::ptr_eq`
  é verdadeiro após clone.
- [ ] `Content::Image` tem quatro campos: `path`, `data`, `width: Option<Value>`,
  `height: Option<Value>`. Braços `{ .. }` nos métodos terminais continuam válidos.
- [ ] `native_image` extrai `width` e `height` de `args.named` e guarda-os no nó.
- [ ] DEBT-25 registado em `01_core/DEBT.md` (resolução de caminhos relativos).
- [ ] DEBT-26 registado em `01_core/DEBT.md` (`PartialEq` exaustivo em `Arc`).
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se o trait `World` já tinha método de leitura de ficheiros ou foi necessário
  adicionar `file()`.
- Nome exacto do método usado (`file`, `read`, `asset`, ou outro).
- Lista completa das funções nativas actualizadas na Tarefa 2.

**Da implementação:**
- Se o compilador identificou `match` de `Content` em falta para `Image` —
  e em quais ficheiros.
- Se o layouter precisou de ser actualizado para não entrar em panic com
  `Content::Image` (dimensões fixas ou ignorar o nó).
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 72:**
- **GO — dimensões reais de imagem:** se a crate `image` puder ser adicionada
  ao L1 ou L3, Passo 72 lê as dimensões reais e passa-as ao layouter.
- **GO — exportação de imagem para PDF:** o layouter já tem o `Arc<Vec<u8>>`;
  Passo 72 usa a biblioteca PDF para emitir o stream XObject.
- **NO-GO — `file()` não implementado no SystemWorld:** se o teste L3 entrar
  em panic; Passo 72 completa a implementação antes de avançar para exportação.
