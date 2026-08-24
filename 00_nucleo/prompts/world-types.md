# world_types — stubs opacos para contratos de World
Hash do Código: b8367659

**Camada**: L1 — entities
**Criado em**: 2026-03-22
**Arquivos gerados**: `01_core/src/entities/world_types.rs`

---

## Contexto

`World` trait em `01_core/contracts/world.rs` usa tipos que ainda
não foram migrados de `lab/typst-original/`: `Library`, `FontBook`,
`Source`, `Bytes`, `Font`, `Datetime`, `FileResult`.

Estes são **stubs opacos** — newtypes com interior mínimo que
permitem que `World` compile agora. O interior muda nos passos
seguintes sem alterar a interface de `World`.

**Stubs não são placeholders descartáveis** — são o padrão Opção C
aplicado a tipos bloqueantes. O contrato público (o tipo em si)
é definitivo; a representação interna é provisória.

### Destino de cada tipo

| Tipo | Stub agora | Substituído em |
|------|------------|---------------|
| `Bytes` | `Vec<u8>` | Passo 5 (infra) |
| `Font` | `Vec<u8>` | Passo 5 (infra) |
| `Library` | `Scope` real (`global`) | **P772n** (era `()` opaco desde a criação — substituído em P772n) |
| `FontBook` | `()` opaco | Passo 5 (infra) |
| `Datetime` | campos primitivos | Passo 4 ou permanente |
| `FileResult<T>` | `Result<T, FileError>` | permanente |
| `FileError` | enum com String | permanente |
| `Source` | `FileId + String` | Passo 4 (parse) |

---

## Restrições Estruturais

- Zero dependências externas
- Interiores privados (opacidade garantida)
- `FileError` usa `thiserror`
- Sem `Default` nos stubs opacos de `()` — não faz sentido semântico
- `Source` stub tem campos `pub` mínimos para o `World` trait compilar

---

## Tipos

```rust
/// Conteúdo binário de um ficheiro. Interior provisório.
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new(data: Vec<u8>) -> Self { Self(data) }
    pub fn as_slice(&self) -> &[u8] { &self.0 }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

/// Fonte tipográfica carregada. Opaca até Passo 5.
pub struct Font(Vec<u8>);

/// Biblioteca de valores e funções do Typst — âmbito base do documento.
/// **P772n**: materializada com um `Scope` real (`global`). `Library::new()`
/// continua a existir (produz `global` vazio) — usado por dezenas de mocks
/// de `World` em testes não relacionados com eval/stdlib, que não precisam
/// de uma stdlib real. `Library::with_global(scope)` é o construtor usado
/// pelo bootstrap real do avaliador (`eval/mod.rs`, `eval/modules.rs`).
pub struct Library {
    pub global: Scope,
}

/// Catálogo de fontes com metadados. Opaco até Passo 5.
pub struct FontBook(());

/// Data e hora para o método today() de World.
/// P843 (F5): data passa a ser OPCIONAL — o constructor `datetime(...)`
/// do vanilla aceita só-hora (`Datetime::Time`). Pelo menos um dos dois
/// é `Some` (invariante dos construtores `new_date`/`new_time`/
/// `new_datetime`/`from_parts`).
pub struct Datetime {
    date: Option<time::Date>,
    time: Option<time::Time>,
}

/// Erro de acesso a ficheiro.
#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("file not found")]
    NotFound,
    #[error("access denied")]
    AccessDenied,
    #[error("{0}")]
    Other(String),
}

/// Resultado de operação de ficheiro.
pub type FileResult<T> = Result<T, FileError>;

/// Ficheiro de texto carregado. Stub mínimo até parse() migrar no Passo 4.
pub struct Source {
    pub id:   FileId,
    pub text: String,
}
```

### P1146 — accessor ordinal (condicionado ao gate ADR-0127)

A fonte ratificada e as sondas coincidentes nos dois binários medem
`datetime.ordinal(datetime(year: 2024, month: 2, day: 29)) == 60`,
`datetime.ordinal(time-only) == none` e 365 para 2023-12-31. Os accessors
`year/month/weekday/day/hour/minute/second` já existem na entidade; `ordinal`
é o único ausente.

Após aprovação do gate, o contrato público acrescenta:

```rust
pub fn ordinal(&self) -> Option<u16>;
```

Ele delega a `time::Date::ordinal` quando `date` existe e retorna `None` para
time-only. Não altera campos, representação, igualdade ou invariantes.

---

## Critérios de Verificação

```
Dado Bytes::new(vec![1, 2, 3])
Quando as_slice() for chamado
Então retorna &[1, 2, 3]

Dado Bytes::new(vec![])
Quando is_empty() for chamado
Então true

Dado FileError::NotFound
Quando Display for chamado
Então "file not found"

Dado FileError::Other("custom".to_string())
Quando Display for chamado
Então "custom"

Dado Source { id: FileId::from_raw(1), text: "hello".into() }
Quando text for acedido
Então "hello"

Dado que FontBook é opaco
Quando instanciado
Então compila sem erros
Quando inspeccionado por fora do módulo
Então o interior não é acessível

Dado Library::new()
Quando global for acedido
Então Scope vazio (len() == 0)

Dado Library::with_global(scope) com "calc" definido
Quando global.get("calc") for chamado
Então Some(&Value dessa binding)
```

---

## Nota sobre Library e FontBook

`FontBook(())` continua intencionalmente opaco — não constrói instâncias
úteis neste passo.

`Library` (**P772n**) deixou de ser opaca: tem um campo público `global:
Scope`. `Library::new()` continua a existir e a produzir um `global` vazio
— preserva compatibilidade com as dezenas de mocks de `World` em testes
que não exercitam eval/stdlib e só precisam de satisfazer a assinatura de
`fn library(&self) -> &Library`. O bootstrap real do avaliador usa
`Library::with_global(scope)`, onde `scope` é a stdlib + cores predefinidas
+ `std` + `text` + elementos de utilizador — tudo o que antes era
achatado directamente em `Scopes.top` (`eval/mod.rs`/`eval/modules.rs`),
tornando-o indistinguível de bindings normais do documento e por isso
mutável sem erro (causa raiz de P772l §2.3 / P772n).

`Scopes<'a>.base: Option<&'a Library>` (`rules/scopes.md`) é o único
consumidor: `Scopes::get` consulta `base.global` como último recurso
(leitura); `Scopes::get_mut` **nunca** consulta `base` — é exactamente
essa ausência estrutural que torna os bindings de `global` imutáveis,
sem precisar de nenhum campo `kind`/`BindingKind` em `Binding`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação inicial — stubs para World trait (ADR-0005) | world_types.rs |
| 2026-07-16 | P772n — `Library` materializada com `global: Scope` real (era `()` opaco desde a criação; "Passo 4" nunca completado, a stdlib era achatada em `Scopes.top` em vez de usar `Library`). Fecha a causa raiz de P772l §2.3 (`cannot_mutate_constant`). | world_types.rs |
