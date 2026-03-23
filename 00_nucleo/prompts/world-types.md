# world_types — stubs opacos para contratos de World

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
| `Library` | `()` opaco | Passo 4 (eval) |
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

/// Biblioteca de valores e funções do Typst. Opaca até Passo 4.
pub struct Library(());

/// Catálogo de fontes com metadados. Opaco até Passo 5.
pub struct FontBook(());

/// Data e hora para o método today() de World.
pub struct Datetime {
    pub year:  i32,
    pub month: u8,
    pub day:   u8,
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

Dado que Library e FontBook são opacos
Quando instanciados
Então compilam sem erros
Quando inspeccionados por fora do módulo
Então o interior não é acessível
```

---

## Nota sobre Library e FontBook

`Library(())` e `FontBook(())` são intencionalmente opacos e
não constroem instâncias úteis neste passo. O seu propósito
é apenas satisfazer as assinaturas de `World`. Nos testes
de `World`, usar mocks que retornam `&Library` via referência
a um campo da struct de mock — não construir `Library` directamente.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação inicial — stubs para World trait (ADR-0005) | world_types.rs |
