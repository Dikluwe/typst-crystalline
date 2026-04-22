# ⚖️ ADR-0026: Content cristalino — divergência intencional do original

**Status**: `IMPLEMENTADO`
**Revisto por**: ADR-0026-R1
**Nota**: decisão original continua em vigor (Content como enum
em L1). ADR-0026-R1 refina a forma interna de `Content::Sequence`
para `Arc<[Content]>` — ver esse ADR para detalhes da
implementação corrente.
**Data**: 2026-03-27

---

## Contexto

O `Content` original em `typst-library` usa:
- `pub struct Content(raw::RawContent)` com vtable unsafe
- Trait `NativeElement` gerada por proc macros `#[elem]`
- Arc manual customizado (fat pointer com ref counting próprio)
- `StyledElem` como wrapper para aplicação de `Styles`

Replicar esta implementação em L1 exigiria:
1. `typst_macros` como dependência de L1 (crate de proc macros)
2. Código `unsafe` em L1 (violação do princípio de domínio puro)
3. Toda a cadeia `NativeElement` → `Styles` → `StyleChain` antes de
   ter sequer texto básico a funcionar

A alternativa é um enum linear declarativo que representa os mesmos
conceitos sem a metaprogramação.

---

## Decisão

`Content` cristalino diverge intencionalmente da implementação do original.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Empty,
    Text(EcoString),
    Space,
    Sequence(Vec<Content>),
    // Variantes futuras — adicionadas com ADR quando necessário
}
```

**A paridade funcional é mantida**: para o mesmo input Typst, o
cristalino produz o mesmo texto de output. **A paridade de implementação
não é um objectivo** — o original usa vtable, o cristalino usa enum.

Esta decisão é coberta pela "Autorização de Divergência" estabelecida
no Passo 18: quando a metaprogramação do original comprometeria a
clareza arquitectural de L1, o cristalino diverge e documenta.

---

## Implicações futuras

Quando Heading, Strong, Emph, e outros elementos de markup forem
adicionados, cada um será uma nova variante do enum:

```rust
pub enum Content {
    // ...
    Heading { level: u8, body: Box<Content> },
    Strong(Box<Content>),
    Emph(Box<Content>),
    // etc.
}
```

Quando `Styles` real migrar, a variante `Styled` poderá ser adicionada:

```rust
Styled(Box<Content>, Styles),  // ADR separada quando Styles migrar
```

O enum pode crescer linearmente sem vtable, mantendo a arquitectura clara.

---

## O que esta ADR não decide

- Quando `Heading`, `Strong`, `Emph` serão adicionados ao enum
- Se `Content` deve eventualmente ter `Arc` para clone O(1) de sequências
  grandes (registado como candidato em DEBT.md)
- Quando e se `Styles` real migra para L1

---

## Consequências

**Positivas**: L1 sem `unsafe`, sem proc macros, sem `typst_macros`;
`Content` é um tipo Rust idiomático testável com `assert_eq!`; pipeline
end-to-end funciona com texto simples.

**Negativas**: divergência de representação interna; se o original
adicionar variantes a `NativeElement`, a sincronização é manual.

**Neutras**: a fronteira funcional (mesmo output de texto para o mesmo
input) é testada pelos testes de pipeline end-to-end.

---

## Referências

- Passo 18 — diagnóstico de Content (vtable, proc macros, Arc manual)
- ADR-0016 — adiamento de eval() e estratégia typst-library
