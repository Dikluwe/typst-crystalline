# Prompt L0 — entities/sides
Hash do Código: d5d8273f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/sides.rs`
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0037
(coesão por domínio), ADR-0061 (Layout Fase X roadmap; primeira
aplicação no Passo 156C)

## Contexto

`Sides<T>` é um tipo geométrico genérico que agrupa quatro
valores indexados por lado (`left` / `top` / `right` / `bottom`).
Materializado no Passo 156C (sub-passo 1 da Fase 1 Layout, per
ADR-0061) como suporte estrutural para `Content::Pad
{ padding: Sides<Length> }`.

Réplica simplificada de
`lab/typst-original/crates/typst-library/src/layout/sides.rs`,
reduzida ao essencial para o consumer cristalino actual. Vanilla
adiciona helpers para `relative_to`, `sum_by_axis`, `map_corners`,
etc. que não são necessários enquanto o único consumer for `pad`.
Quando outros consumers (PageConfig refino com `Sides<Length>`
para margens, Stroke com `Sides<Stroke>` para bordas) forem
materializados, este L0 ganha extensões mínimas conforme
necessário.

Diagnóstico prévio: ausente — entrada nova adicionada em P156C
(modelo "tudo-num-passo" análogo ao `entities/sides.md` vanilla).

## Interface pública

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sides<T> {
    pub left:   T,
    pub top:    T,
    pub right:  T,
    pub bottom: T,
}

impl<T> Sides<T> {
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self;
}

impl<T: Clone> Sides<T> {
    pub fn uniform(value: T) -> Self;
}

impl<T: Default> Default for Sides<T>;
```

## Semântica

- **Ordem dos campos**: `left`, `top`, `right`, `bottom`. Não
  é a ordem CSS (`top`, `right`, `bottom`, `left`); segue
  modelo vanilla typst que usa `left`/`top`/`right`/`bottom`.
- **`new(...)`**: construtor com cada lado independente.
- **`uniform(v)`**: clona o valor para os quatro lados. Útil
  para padding simétrico (`Sides::uniform(Length::pt(10.0))`).
- **`Default`**: cada lado a `T::default()`. Para `Sides<Length>`
  resulta em `Length::ZERO` em todos os lados.
- **`Copy`** derivado quando `T: Copy` (caso comum:
  `Sides<Length>`, `Sides<f64>`, `Sides<bool>`).
- **`Eq`** derivado para suportar comparação exacta em testes;
  só requer `T: Eq` (não satisfeito por `Length` em virtude
  do `f64`, mas o derive genérico não restringe a clientes
  que não invoquem `Eq`).

## Invariantes

- Sem ordem implícita entre lados — cada lado é independente.
- Sem validação de valores — `T` pode ser qualquer tipo.
- Sem mutação após construção — fields públicos mas todos os
  consumidores actuais constroem novos `Sides` em vez de mutar.

## Consumers actuais

- `Content::Pad { padding: Sides<Length> }` (Passo 156C).

## Consumers planeados (ADR-0061 + ADR-0060 Fase 3 refino)

- `PageConfig::margin: Sides<Length>` (era escalar; refino Fase 3
  Layout per ADR-0061).
- `Content::Block { inset: Sides<Length>, ... }` (Fase 2 Layout).
- `Content::Box { inset: Sides<Length>, ... }` (Fase 2 Layout).
- `Stroke<Sides<Length>>` para bordas refinadas (futuro).

## Sobre paridade

Vanilla expõe `Sides<T>` com helpers ricos (rotação por `Dir`,
soma por eixo, mapeamento por canto). O cristalino mantém
forma mínima até consumer concreto exigir extensão (per
ADR-0061 Fase 1 = mínimo viável; ADR-0054 perfil graded
aceita aproximação).

`Side` enum (Left/Top/Right/Bottom) e métodos derivados
(`inv`, `next_cw`, `axis`) são scope-out neste passo. Adicionar
quando primeiro consumer de iteração indexada (e.g. layout
de borda côncava) o exigir.
