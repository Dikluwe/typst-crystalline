# ADR-0109 — Atomização: definição, e a sua distinção de desacoplamento

**Estado:** aceite (decisão do dono).
**Contexto da decisão:** arco F (P331–P375). Substitui qualquer uso anterior de "atomização" que a
tenha confundido com a métrica `content→elements` da lente ou com desacoplamento de imports.
**Histórico de numeração:** o P376 gravou esta ADR como `0110` sem varrer `00_nucleo/adr/` (número
inventado); o P377 corrigiu para **0109** (o primeiro livre — o maior em uso era 0108).

---

## Decisão

**Atomização** é distribuir a lógica e os dados de arquivos **monolíticos** para arquivos **menores
de responsabilidade única**, cada um **legível e compreensível de forma independente**.

O critério de sucesso é de **leitura**: consigo entender uma unidade (um elemento, uma feature)
abrindo **só o arquivo dela**, sem caçar a sua lógica espalhada por arquivos do núcleo.

A métrica da atomização é, portanto:
- o **tamanho** dos arquivos monolíticos (deve encolher);
- o **número de responsabilidades** por arquivo (deve tender a uma);
- a **localidade**: a lógica de uma unidade vive no arquivo da unidade — **na sua camada**
  (a lógica de *render* na camada de render, não no arquivo do *struct* de dados; ver a forma B).

---

## O que a atomização NÃO é (as não-metas — registadas para impedir a deriva)

1. **NÃO é zerar `content→elements` (a métrica da lente).** Essa métrica conta **imports** (quem
   importa quem). Atomizar é sobre **onde a lógica mora**, não sobre quem importa quem. O núcleo
   pode continuar importando os elementos e chamá-los; isso **não** prejudica a leitura independente
   (`heading.rs` ser importado por `content.rs` não torna `heading.rs` menos legível).
2. **NÃO usa despacho dinâmico** (vtable / `Arc<dyn>` / PropMap-reificação). O `match` estático
   sobre o enum **permanece**, e com ele a **jump table O(1)** e a **exaustividade do compilador**.
3. **NÃO é desacoplamento** (substituir elementos sem recompilar o núcleo, carregar de plugin
   externo, fronteira de compilação núcleo-sem-elementos). Isso é uma propriedade de
   plugin-system/substituibilidade em runtime — **não é objetivo deste projeto**.
4. **NÃO troca garantia estática por separação.** A separação obtém-se **mantendo** o `match`
   exaustivo, com os **corpos dos arms delegando** a uma free function da feature
   (`Content::Heading(h) => heading::layout(self, h)`). O `match` fica magro e verboso (uma linha
   por elemento); a lógica gorda muda para o arquivo da feature **na camada de render**. A
   verbosidade do `match` é um preço **aceite** — não pesa em runtime e é trivial de ler (todos os
   arms iguais na forma).

---

## A forma canónica (como atomizar um elemento) — Opção B

> **A forma é a B (free function na camada de render).** A Opção A (lógica no arquivo do *struct*,
> `impl XElem { fn layout }`) foi **medida e rejeitada** — ver a secção seguinte.

Antes:
```rust
// rules/layout/mod.rs (monolítico)
match content {
    Content::Heading(h) => { /* 44 linhas de layout de heading */ }
    Content::Figure(f)  => { /* 34 linhas de layout de figura */ }
    // … 59 arms bespoke, 1857 linhas …
}
```
Depois:
```rust
// rules/layout/mod.rs (magro — match exaustivo preservado, corpos de 1 linha)
match content {
    Content::Heading(h) => heading::layout(self, h),
    Content::Figure(f)  => figure::layout_figure(self, /* … */),
    // … 59 arms, 1 linha cada — exaustividade e jump table intactas …
}
// rules/layout/heading.rs (a lógica de render atomizada, legível sozinha)
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    h:        &HeadingElem,
) {
    // as 44 linhas, aqui — acede ao estado privado do Layouter por ser
    // módulo DESCENDENTE de rules::layout (sem import reverso, sem pub(crate)).
}
```

- O enum `Content` **fica fechado**, com as variantes (modelo da ADR-0026/0105-D).
- O `match` **fica sem wildcard** (exaustivo) — o compilador continua a pegar o elemento esquecido.
- O despacho **fica estático** (jump table) — sem ganho nem perda algorítmica; **sem vtable** (a
  ADR-0026 é satisfeita, não emendada).
- A lógica de render de cada elemento **muda para o seu arquivo na camada de render**
  (`rules/layout/<elem>.rs`), como **free function** que recebe `&mut Layouter`. Segue os 6
  precedentes do repo (`figure.rs`, `image.rs`, `grid.rs`, `placement.rs`, `outline.rs`,
  `references.rs`).

---

## A Opção A rejeitada (registada para a IA não a reintroduzir)

A **Opção A** põe a lógica de render no arquivo do *struct* do elemento
(`entities/elements/heading.rs`, `impl HeadingElem { fn layout(&self, ctx) }`). O **P376 mediu** que
ela **exige três coisas** que vão contra a arquitetura:

1. **Import reverso `entities → rules::layout::Layouter`** — o arquivo de **dados** passa a importar
   o **render**. Cria um **ciclo de módulos** `entities ↔ rules` e inverte a direção dado→render
   (contra a separação que a ADR-0107 pressupõe).
2. **Alargar a visibilidade do `Layouter`** (`style`/`regions`/`font_size_pt`/`flush_line`/… de
   privados para `pub(crate)`) — alarga a superfície de API.
3. **Threading de genéricos** `Layouter<'a, M, S>` no método do struct.

A **Opção B evita os três** (o método de render vive na camada de render; acede ao `Layouter`
privado por **descendência de módulo**, não por import reverso). Por isso a forma canónica é a **B**.
**Não reintroduzir a A** em nome de "abrir só o arquivo do struct" — a leitura independente obtém-se
na camada de render, sem o acoplamento dado→render.

---

## Relação com as outras ADRs

- **ADR-0026** (enum sem vtable): **satisfeita.** A atomização não introduz vtable; é o "enum linear
  sem vtable" que a 0026 prescreve, só com a lógica movida para free functions da camada de render.
- **ADR-0105** (modelo D, exaustividade cl.3): **satisfeita e reforçada.** A exaustividade do `match`
  é **mantida** (não há despacho dinâmico a repor com teste/lint).
- **ADR-0107** (paridade com a linguagem; separação dado/render): a forma B **respeita** a separação
  (render fica na camada de render); a forma A a **violaria** (dado importando render). A atomização
  é estrutural, não muda comportamento (content-preserving).

---

## O erro histórico que esta ADR corrige (registado para a IA não o repetir)

No P346, a métrica `content→elements` da lente (alvo 0) foi promovida a "Marco G" e tratada como se
fosse o objetivo de atomização. Era confusão de dois significados: a lente mede **imports**
(desacoplamento); o objetivo é **localidade da lógica** (atomização). O P361/P375 mediram que zerar
`content→elements` **reloca** o acoplamento (não o elimina) e exige despacho dinâmico (vtable, que a
0026 rejeita) + perda de exaustividade. **Nada disso serve à atomização** como aqui definida. A
atomização obtém-se **sem** tocar a métrica da lente, **sem** despacho dinâmico, **sem** perder
exaustividade.

---

## Trecho para o `claude.md` (para a IA não viajar)

> **Atomização** (ADR-0109) = mover a lógica de arquivos monolíticos para o arquivo da unidade dona,
> **na sua camada** (cada elemento/feature legível sozinho). **NÃO** é zerar `content→elements`,
> **NÃO** é desacoplamento de imports, **NÃO** usa vtable/`dyn`/PropMap, **NÃO** remove o `match`
> exaustivo. A **forma é a B**: o `match` no núcleo fica magro (corpos delegam a uma free function
> `<elem>::layout(self, e)` em `rules/layout/<elem>.rs`); a lógica de render muda para o arquivo da
> feature na camada de render (acede ao `Layouter` por descendência de módulo, **sem** import
> reverso `entities→rules`, **sem** `pub(crate)`). A **Opção A** (lógica no arquivo do struct) está
> **rejeitada** — cria o acoplamento dado→render. O `match` exaustivo, a jump table e os imports
> **ficam**. A métrica da lente (`content→elements`) é **irrelevante** — não a use como gate. Se um
> plano propuser despacho dinâmico, "zerar `content→elements`", ou a Opção A em nome de atomização,
> **isso é a deriva que esta ADR proíbe** — pare e separe os significados.

---

## Escopo de aplicação (decisão do dono)

- **Primeiro os elementos** que o arco F trabalhou (atomizar layout de cada elemento para o seu
  arquivo na camada de render). **P376** materializou a fatia **containers** (Block/Boxed/Stack/Pad;
  monólito −581 linhas, forma B, content-preserving). **P377+** seguem as fatias restantes.
- A **varredura do projeto inteiro** (todo monólito, não só os de elemento) fica para **depois de
  finalizar** os elementos.
- A **decisão de crates** (criar crates novos dentro das camadas para agrupamentos) fica para
  **depois** — começar movendo para **arquivos dentro dos crates atuais**.
