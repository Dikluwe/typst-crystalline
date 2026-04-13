# Prompt L0 — `entities/span`
Hash do Código: 1f5a1ef5

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/span.rs`
**Criado em**: 2026-03-22 (Passo 1)
**Atualizado em**: 2026-04-12 (restauro — expandido com layout de bits, dois sabores, Spanned<T> e critérios completos)
**ADRs relevantes**: nenhum ADR dedicado; contrato fundamental de rastreabilidade de todo o compilador

---

## Contexto e Objetivo

Para emitir mensagens de erro úteis ou para permitir introspecção (ex: clicar
num elemento do PDF e saltar para o código fonte), o motor precisa de mapear
cada nó da AST e cada elemento de `Content` de volta à sua localização no
texto original. O `Span` é a estrutura de dados que guarda essa coordenada.

`Span` ocupa **8 bytes** e é `Copy` — pode estar em cada nó da AST sem
explodir o consumo de memória. A otimização de null pointer (`NonZeroU64`)
garante que `Option<Span>` também ocupa 8 bytes.

---

## Dois Sabores de Span

```
Layout interno do u64:
| 16 bits (FileId) | 48 bits (number ou range codificado) |
```

### Numbered Span (para ficheiros Typst)
Além do `FileId`, codifica um **número único** por nó AST (range 2..2^47).
Em vez de guardar bytes que se deslocam ao editar, cada nó recebe um número
estável durante a edição incremental. O `Source` mantém a tabela de
mapeamento `número → byte range`.

### Raw Range Span (para ficheiros não-Typst)
Codifica directamente `Range<usize>` com até 2^23 por extremidade (límite de
8 MiB — suficiente para ficheiros de texto). Usado por imagens, binários e
assets externos que não têm AST.

### Detached Span
O valor reservado `1u64` representa um span sem ficheiro — usado por nós
gerados sinteticamente pelo compilador (ex: `Span::detached()`).

---

## Restrições Estruturais

- Camada **L1**: zero I/O. Apenas `NonZeroU64` e `FileId` (ambos L1).
- `Span` é `Copy + Clone + Eq + PartialEq + Hash`.
- `from_number` e `number()` são `pub(crate)` — usados apenas por `SyntaxNode`
  e pelo parser (Passo 4) no mesmo crate.
- Não guarda directamente a string do ficheiro — apenas coordenadas/índices.

---

## Instrução

### `Span`

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span(NonZeroU64);

impl Span {
    // Construtores
    pub const fn detached() -> Self                           // valor reservado = 1
    pub const fn from_range(id: FileId, range: Range<usize>) -> Self  // satura em 2^23
    pub const fn from_raw(v: NonZeroU64) -> Self              // round-trip

    pub(crate) const fn from_number(id: FileId, number: u64) -> Option<Self>
    // number deve estar em FULL = 2..2^47; retorna None fora do range

    // Leitores
    pub const fn is_detached(self) -> bool                   // self.0.get() == 1
    pub const fn id(self) -> Option<FileId>                  // 16 bits mais altos
    pub const fn range(self) -> Option<Range<usize>>         // só para raw range spans
    pub const fn into_raw(self) -> NonZeroU64

    pub(crate) const fn number(self) -> u64                  // 48 bits mais baixos

    // Utilitários
    pub fn or(self, other: Self) -> Self   // retorna other se self for detached
    pub fn find(iter: impl IntoIterator<Item = Self>) -> Self // 1º não-detached
}
```

### `Spanned<T>`

Envolve qualquer valor com a sua localização na fonte.

```rust
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Spanned<T> {
    pub v: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub const fn new(v: T, span: Span) -> Self
    pub const fn detached(v: T) -> Self       // Span::detached()
    pub const fn as_ref(&self) -> Spanned<&T>
    pub fn map<F, U>(self, f: F) -> Spanned<U>
}

impl<T: Debug> Debug for Spanned<T>  // delega o Debug ao campo v
```

---

## Critérios de Verificação

```
// Detached
Span::detached().is_detached()         = true
Span::detached().id()                  = None
Span::detached().range()               = None

// Numbered span
let file = FileId::from_raw(NonZeroU16::new(5).unwrap());
let span = Span::from_number(file, 10).unwrap();
span.id()     = Some(file)
span.number() = 10
span.range()  = None           // não é raw range

// from_number fora do range
Span::from_number(file, 0) = None
Span::from_number(file, 1) = None   // reservado para DETACHED

// Raw range (saturação em 2^23 = 8388608)
let span = Span::from_range(file, 177..233);
span.id()   = Some(file)
span.range() = Some(177..233)

Span::from_range(file, 0..0).range()            = Some(0..0)
Span::from_range(file, 0..8388607).range()      = Some(0..8388607)

// or()
Span::detached().or(b) = b          // se self é detached, retorna other
b.or(Span::detached()) = b          // se self não é detached, retorna self

// find()
Span::find([Span::detached(), span_b, span_c]).number() = span_b.number()

// Spanned
let s = Spanned::new(42u32, Span::detached());
s.v    = 42
s.span = Span::detached()

let doubled = Spanned::detached(10u32).map(|v| v * 2);
doubled.v    = 20
doubled.span = Span::detached()
```

---

## Resultado Esperado

- `01_core/src/entities/span.rs` com `Span`, `Spanned<T>` e testes co-localizados
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/span.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação — Passo 1: Span(NonZeroU64), Spanned<T> | `span.rs` |
| 2026-04-12 | Restauro — expandido com layout de bits, dois sabores (numbered/raw range), `pub(crate)`, critérios completos | `span.md` |
