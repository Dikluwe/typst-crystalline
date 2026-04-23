# Diagnóstico: padrão `<T<'static> as Validate>::Constraint` em `Tracked` recursivo

**Tipo vanilla**: `Route<'a>` (e qualquer `#[comemo::track]` auto-referente).
**Localização vanilla**: `lab/typst-original/crates/typst-library/src/engine.rs:258`.
**Data do diagnóstico**: 2026-04-22
**Contexto**: Passo 92 — descoberta durante integração estrutural de `Route<'a>`.

**Natureza**: registo factual do padrão descoberto ao encadear
`Tracked<Self>` recursivamente em `#[comemo::track]`, com a versão
`comemo 0.4.0` usada no cristalino. Este ficheiro não decide —
documenta para materializações futuras.

---

## 1. Contexto

Tipos `#[comemo::track]` expõem métodos através de um proxy `Tracked`.
Quando o próprio tipo tem um campo do tipo `Tracked<Self>` (auto-referência
— típico em estruturas de lista ligada como `Route<'a>`), surge um
problema de inferência do parâmetro `Constraint` que quebra a
covariância necessária para encadear lifetimes.

---

## 2. Manifestação

`01_core/src/entities/world_types.rs` — forma que **não compila** no
encadeamento recursivo (versão inicial do Passo 90):

```rust
pub struct Route<'a> {
    outer: Option<Tracked<'a, Self>>,  // Constraint inferida por Self
    ...
}
```

Ao tentar `Route::extend(route)` num `eval_expr<'r>(..., route: Tracked<'r, Route<'r>>)`
recursivo (Passo 92), o compilador reportava:

```
error: lifetime may not live long enough
   ...
    = note: requirement occurs because of the type `Route<'_>`, which makes the generic argument `'_` invariant
    = note: the struct `Route<'a>` is invariant over the parameter `'a`
```

Forma que **compila** (Passo 92, resolução):

```rust
pub struct Route<'a> {
    outer: Option<Tracked<'a, Self, <Route<'static> as Validate>::Constraint>>,
    ...
}
```

---

## 3. Explicação

`comemo 0.4.0` define `Tracked<'a, T, C = <T as Validate>::Constraint>`
onde `C` é o tipo de constraint associado a `T`. Quando `T` tem um
parâmetro de lifetime `'a`, o `C` inferido por omissão depende desse
`'a` — tornando `Tracked<'a, Route<'a>>` **invariante** em `'a`. Isto
impede qualquer narrowing/widening da lifetime, incluindo o necessário
para `Route::extend`.

A solução documentada pelo próprio `comemo` (docstring de `Tracked` em
`track.rs:120-146`) é especificar explicitamente o `Constraint` usando
um `'static` artificial: `<T<'static> as Validate>::Constraint`. Como
todas as constraints em `comemo` são `'static` (ver `Validate::Constraint: 'static`),
a escolha de `'static` é inócua e apenas informa o compilador que o
`Constraint` não depende de `'a`. O `Tracked<'a, T>` torna-se
**covariante** em `'a` e o encadeamento compila.

O vanilla usa o mesmo pattern com o seu `Track::Call` (equivalente
semântico ao `Validate::Constraint` do `comemo 0.4.0` — APIs diferentes,
mesmo propósito).

---

## 4. Quando aplicar

Indicadores para usar este padrão em tipos L1 do cristalino:

- Tipo `#[comemo::track]` com campo `Tracked<Self>` (lista ligada
  de tracked — ex.: `Route`).
- Tipo `#[comemo::track]` com campo `Tracked<T<'a>>` e lifetime
  dependente que precisa ser narrowing-compatible.
- Erros de compilação ao chamar `T::extend(tracked)` com mensagens
  sobre `'a` invariante e "requirement occurs because of the type
  `T<'_>`".

Se o tipo não tem auto-referência via `Tracked` (ex.: `Traced` do
Passo 88, que guarda apenas `Option<Span>`), o pattern **não é
necessário** — a forma com `Constraint` default funciona.

---

## 5. Referências

- `01_core/src/entities/world_types.rs:186` — `Route::outer` no
  cristalino (uso concreto do padrão).
- `lab/typst-original/crates/typst-library/src/engine.rs:258` —
  `Route::outer` no vanilla (`<Route<'static> as Track>::Call`;
  vanilla usa a API `Track::Call` em vez de `Validate::Constraint`).
- `comemo 0.4.0` — `src/track.rs:120-146` — docstring de `Tracked`
  que documenta o padrão com exemplo `Chain<'a>`.
- `00_nucleo/materialization/typst-passo-92.md` — passo onde foi
  descoberto e aplicado.

---

## 6. Candidatos futuros

Tipos do projecto que, quando materializados, vão provavelmente
precisar deste padrão:

- **`Engine<'a>`** (stub em `world_types.rs`) — tem campo
  `route: Route<'a>`, pelo que pode herdar o requisito indirectamente;
  se expuser métodos tracked sobre si próprio com auto-referência, o
  pattern será directo.
- Outros tipos `#[comemo::track]` com estrutura de lista ligada
  (ex.: futura cadeia de `StyleChain<'a>` se for tracked) — aplicar
  o mesmo padrão.
- Tipos que, durante materialização, produzam o erro "invariant
  over the parameter" ao encadear `Tracked` — resposta directa:
  adicionar `<T<'static> as Validate>::Constraint`.
