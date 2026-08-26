# Prompt L0 — entities/dir
Hash do Código: bc025298

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/dir.rs`
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0037
(coesão por domínio), ADR-0061 (Layout Fase X roadmap;
primeira aplicação no Passo 156I).

## Contexto

`Dir` é um enum com 4 variantes cardinais usado como atributo
`dir` em `Content::Stack`. Materializado no Passo 156I
(Fase 2 Layout sub-passo 3, ADR-0061) como infraestrutura
genérica reusável análoga a `Parity` (P156E) e `Sides<T>`
(P156C).

Réplica simplificada de
`lab/typst-original/crates/typst-library/src/layout/dir.rs::Dir`.
Vanilla expõe `Smart<Dir>` em alguns sítios (default
auto-determinado por bidi engine); cristalino usa `Dir`
directo com default `TTB` (stack vertical).

> **Fonte de paridade (P1031)** — a referência ao ficheiro fonte do vanilla existia, mas
> sem citação da documentação. Doc comment `#[ty]` do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/layout/dir.rs:6-20`, publicado em
> `typst.app/docs/reference/layout/direction/`:
>
> *"The four directions into which content can be laid out. Possible values are:
> - `{ltr}`: Left to right.
> - `{rtl}`: Right to left.
> - `{ttb}`: Top to bottom.
> - `{btt}`: Bottom to top.
>
> These values are available globally and also in the direction type's scope, so you can
> write either of the following two:*
>
> ```example
> #stack(dir: rtl)[A][B][C]
> #stack(dir: direction.rtl)[A][B][C]
> ```
>
> **Citação literal** para as quatro variantes cardinais, para os nomes de superfície
> (`ltr`/`rtl`/`ttb`/`btt`) e para o uso como argumento `dir` de `stack` — as três
> afirmações do contexto acima. Nota adicional que este L0 não regista: os valores existem
> **tanto** no escopo global **como** em `direction.*`.
>
> **Divergência declarada, não medida**: o default `TTB` do cristalino contra o
> `Smart<Dir>` do vanilla é escolha de representação com efeito potencial no observável
> (a auto-determinação por bidi). Fica marcada como **inferência** de que não afecta os
> casos suportados; refutável por medição de `#stack` sem `dir` explícito em texto RTL.
> Não medida neste passo.

## Interface pública

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir { LTR, RTL, TTB, BTT }

impl Dir {
    pub fn is_horizontal(self) -> bool;
    pub fn is_vertical(self) -> bool;
    pub fn is_reverse(self) -> bool;
}

impl Default for Dir {
    fn default() -> Self { Self::TTB }
}
```

## Semântica

- `LTR` (left-to-right): texto ocidental, stack horizontal forward.
- `RTL` (right-to-left): árabe, hebraico, stack horizontal reverse.
- `TTB` (top-to-bottom): default stack vertical (forward).
- `BTT` (bottom-to-top): raro; equações empilhadas inversas.

- `is_horizontal()` ↔ `LTR | RTL`.
- `is_vertical()`   ↔ `TTB | BTT`.
- `is_reverse()`    ↔ `RTL | BTT` (direcção negativa face ao
  sistema de coordenadas Y-down/X-right).

## Invariantes

- 4 variantes apenas — sem `Auto` interno (vanilla `Smart<Dir>`
  simplificado para `Dir` directo per padrão "Smart→Option/
  default" — mas neste caso usamos Dir directo com Default
  porque Dir tem default natural `TTB`).
- `Copy` derivado: trivial, sem heap.
- `Eq` derivado: comparação exacta.
- Métodos puros sem side-effects.

## Consumers actuais

- `Content::Stack { children, dir, spacing }` (Passo 156I).
- `Value::Dir(Dir)` em runtime (Passo 576), expondo `ltr`, `rtl`, `ttb`, `btt`
  como valores de primeira classe no escopo global do eval.

## Consumers planeados

- `text.dir` no `#set text(...)` para alinhamento de parágrafo RTL (P576).
- `Content::Columns { dir, ... }` quando Fase 3 Layout
  materializar columns (per DEBT-56).

## Sobre paridade

Vanilla usa `Smart<Dir>` em `PageElem` para auto-determinação
por lang; cristalino usa `Dir` directo com `Default::default() ==
TTB` (mais idiomática Rust).

`is_horizontal`/`is_vertical`/`is_reverse` métodos derivados
oferecem queries comuns sem expor pattern-match a callers
(encapsulamento simples).
