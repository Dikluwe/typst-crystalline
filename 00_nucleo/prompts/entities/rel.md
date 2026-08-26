# Prompt L0 — `entities/rel`
Hash do Código: ffffffff

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/rel.rs`
**Criado em**: 2026-06-25 (P469)
**Atualizado em**: 2026-06-25
**ADRs relevantes**: ADR-0029 (`Length`), ADR-0117 Cláusula 4 (tipo puro)

---

## Contexto

`Rel<T>` modela um valor **relativo a um contexto** mais um offset absoluto.
Neste passo materializa-se apenas `Rel<Length>`; o tipo é genérico para
permitir `Rel<Abs>` ou outras bases no futuro.

Exemplos Typst:
- `50%` → `Rel { rel: 0.5, abs: Length::ZERO }`
- `100% - 1em` → `Rel { rel: 1.0, abs: Length::em(-1.0) }`
- `50% + 2cm` → `Rel { rel: 0.5, abs: Length::cm(2.0) }`

A resolução para um comprimento absoluto requer o contexto de layout
(largura/altura do container) e é responsabilidade de consumers em Trilha 7.

---

## Decisão — `Rel<T>` genérico, instanciado para `Length`

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rel<T> {
    pub rel: f64,   // fração do contexto (0.5 = 50%)
    pub abs: T,     // offset absoluto
}
```

Traits genéricos (quando `T` os suporta):
- `Default`
- `Add`, `Sub`, `Neg`
- `Mul<f64>`, `Div<f64>`

Instanciação para `Length`:
- `Rel::<Length>::zero()`
- `Rel::<Length>::from_percent(pct: f64)`
- `Rel::<Length>::resolve(&self, context: Length) -> Length`
- `Rel::<Length>::is_abs_zero()`

Operações mistas:
- `Rel<Length> + Length` e `Length + Rel<Length>` → `Rel<Length>`
- `Rel<Length> - Length` → `Rel<Length>`

---

## Fronteiras e scope-out

- `Rel<Abs>` e `Rel<Fr>` — fora de escopo; tipo genérico prepara terreno.
- Resolução em layout (`block(width: 50%)`) — Trilha 7.
- Comparação `<`/`>` de `Relative` — requer contexto; scope-out.
- `==` estrutural entre dois `Rel<T>` — suportado via `PartialEq` derive.

---

## Interface pública

```rust
impl<T: Default> Rel<T> {
    pub fn from_percent(pct: f64) -> Self;
}

impl Rel<Length> {
    pub fn zero() -> Self;
    pub fn resolve(&self, context: Length) -> Length;
    pub fn is_abs_zero(&self) -> bool;
}

impl<T: Add<Output=T>> Add for Rel<T>;
impl<T: Sub<Output=T>> Sub for Rel<T>;
impl<T: Neg<Output=T>> Neg for Rel<T>;
impl<T: Mul<f64, Output=T>> Mul<f64> for Rel<T>;
impl<T: Div<f64, Output=T>> Div<f64> for Rel<T>;

impl Add<Length> for Rel<Length>;
impl Add<Rel<Length>> for Length;
impl Sub<Length> for Rel<Length>;
impl Sub<Rel<Length>> for Length;
```

---

## Critérios de Verificação

```rust
Rel::<Length>::from_percent(50.0).rel == 0.5
Rel::<Length>::from_percent(50.0).abs.is_zero()

let r = Rel::<Length>::from_percent(50.0) + Length::cm(2.0);
r.resolve(Length::cm(10.0)) == Length::cm(7.0)

let r = Rel::<Length>::from_percent(50.0) * 2.0;
r.rel == 1.0 && r.abs.is_zero()

Length::cm(1.0) + Rel::<Length>::from_percent(50.0)
    == Rel::<Length>::from_percent(50.0) + Length::cm(1.0)
```

---

## Resultado Esperado

- `01_core/src/entities/rel.rs` com `Rel<T>` genérico e instanciação para `Length`.
- Testes co-localizados cobrindo construção, operações e resolução.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-25 | Criação — P469 `Rel<Length>` | `rel.rs` |
