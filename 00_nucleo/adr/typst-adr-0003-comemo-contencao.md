# ⚖️ ADR-0003: comemo e Hierarquia de Contenção — Coexistência

**Status**: `IDEIA — não implementar ainda`
**Data**: 2026-03-22
**Contexto**: migração Typst → Arquitetura Cristalina

---

## Contexto

ADR-0001 autoriza `comemo` em `[l1_allowed_external]` durante a
migração (Opção C — pragmática primeiro, isolamento depois).

ADR-0002 propõe uma hierarquia de contenção com `InteractionScope`
declarado por nível como mecanismo primário de optimização de layout.

A questão que este ADR regista: **qual a relação entre os dois?**

---

## comemo é invalidação, contenção é escopo

Os dois mecanismos resolvem problemas diferentes e complementares:

**`comemo`** — responde à pergunta: *"quando um input muda, o que
precisa de ser recomputado?"*

Rastreia acessos via `Tracked<T>`, detecta quais valores foram
consultados durante uma computação, e invalida selectivamente
quando esses valores mudam. É um mecanismo de **invalidação
reactiva**.

**Hierarquia de contenção** — responde à pergunta: *"quando
optimizo este nível, quantos outros níveis posso afectar?"*

Declara o escopo máximo de influência de cada decisão de layout,
reduzindo o espaço de busca de forma estrutural. É um mecanismo
de **delimitação de escopo**.

---

## Quando usar cada um

```
Mudança de fonte → comemo invalida o que dependia da fonte
                 → contenção define quais níveis são afectados
                   pelo novo layout com a nova fonte

Edição de texto num parágrafo → comemo invalida o layout desse parágrafo
                               → ParagraphLevel com InteractionScope::SiblingNegotiation
                                 sabe que só precisa de renegociar com vizinhos
                                 → comemo invalida apenas esses vizinhos
```

A hierarquia de contenção informa o `comemo` sobre o escopo de
propagação de invalidação. Com a contenção declarada, `comemo`
não precisa de explorar o documento inteiro para saber o que
invalidar — o escopo já está declarado na hierarquia.

---

## Manter comemo quando a hierarquia não for usada

O Typst cristalino deve manter `comemo` funcional independentemente
da hierarquia de contenção ser adoptada ou não:

1. **Compatibilidade retroativa**: a `World` trait pública usa
   `comemo::Tracked<dyn World>` — qualquer código que implementa
   `World` (ex: `typst-cli`, integrações externas, pacotes do
   ecossistema Typst) depende desta assinatura. Quebrar isso
   quebraria toda a cadeia de compatibilidade. A migração cristalina
   não pode impor esta ruptura.

2. **Compatibilidade funcional**: documentos que não usam a hierarquia explícita
   continuam a beneficiar da invalidação incremental de `comemo`

3. **Migração gradual**: a hierarquia pode ser adoptada por nível,
   sem quebrar o comportamento incremental existente

4. **Fallback**: se um nível não declara `InteractionScope`, o
   comportamento é o actual do Typst — `comemo` faz a invalidação
   sem restrição de escopo

A hierarquia de contenção é um refinamento opcional sobre `comemo`,
não um substituto.

---

## Modelo de coexistência

```rust
pub struct LayoutLevel {
    /// Escopo de interacção declarado.
    /// None = comportamento actual (comemo sem restrição de escopo)
    interaction_scope: Option<InteractionScope>,

    /// comemo rastreia acessos independentemente do scope
    children: Vec<LayoutLevel>,
}
```

Quando `interaction_scope` é `None`, o engine usa o pipeline
actual do Typst com `comemo`. Quando é `Some(scope)`, o engine
restringe a propagação de invalidação ao escopo declarado —
potencialmente saltando recomputações que o `comemo` teria
explorado.

---

## Isolamento de comemo em L3 (revisão do ADR-0001 Opção B)

O ADR-0001 deixou em aberto o isolamento de `comemo` em L3 para
o Passo 10. Este ADR clarifica como isso ficaria:

```rust
// 01_core/contracts/incremental.rs
/// Contrato de rastreio de acessos para invalidação incremental.
/// L1 usa este contrato — não comemo directamente.
pub trait Trackable {
    type Tracked<'a>: Copy where Self: 'a;
    fn track(&self) -> Self::Tracked<'_>;
}

// 03_infra/incremental/comemo_impl.rs
/// Implementação de Trackable usando comemo.
/// comemo fica confinado a L3.
```

A `World` trait em L1 passaria a usar `Trackable` em vez de
`comemo::Tracked<dyn World>` directamente. L3 implementa
`Trackable` para `comemo`.

Esta é a Opção B do ADR-0001 — registada aqui como forma concreta
de a implementar quando chegar o momento.

---

## Estado: IDEIA

Regista a relação conceptual entre os dois mecanismos e a forma
de os fazer coexistir. Não tem plano de implementação — depende
de ADR-0002 estar estável e de experiência concreta com o pipeline
cristalino.

---

## Referências

- ADR-0001 — autorização de `comemo` e Opção B (isolamento em L3)
- ADR-0002 — hierarquia de contenção e `InteractionScope`
- comemo: https://github.com/typst/comemo
- `lab/typst-original/crates/typst-eval/` — uso actual de `Tracked<dyn World>`
