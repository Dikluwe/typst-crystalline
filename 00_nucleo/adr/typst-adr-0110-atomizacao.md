# ADR-0110 — Atomização: definição, e a sua distinção de desacoplamento

**Estado:** aceite (decisão do dono).
**Contexto da decisão:** arco F (P331–P375). Substitui qualquer uso anterior de "atomização" que a
tenha confundido com a métrica `content→elements` da lente ou com desacoplamento de imports.

---

## Decisão

**Atomização** é distribuir a lógica e os dados de arquivos **monolíticos** para arquivos **menores
de responsabilidade única**, cada um **legível e compreensível de forma independente**.

O critério de sucesso é de **leitura**: consigo entender uma unidade (um elemento, uma feature)
abrindo **só o arquivo dela**, sem caçar a sua lógica espalhada por arquivos do núcleo.

A métrica da atomização é, portanto:
- o **tamanho** dos arquivos monolíticos (deve encolher);
- o **número de responsabilidades** por arquivo (deve tender a uma);
- a **localidade**: a lógica de uma unidade vive no arquivo da unidade.

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
   exaustivo, com os **corpos dos arms delegando** a métodos do elemento (`Content::Heading(h) =>
   h.layout(ctx)`). O `match` fica magro e verboso (uma linha por elemento); a lógica gorda muda
   para o arquivo do elemento. A verbosidade do `match` é um preço **aceite** — não pesa em runtime
   e é trivial de ler (todos os arms iguais na forma).

---

## A forma canónica (como atomizar um elemento)

Antes:
```rust
// layout/mod.rs (monolítico)
match content {
    Content::Heading(h) => { /* 50 linhas de layout de heading */ }
    Content::Figure(f)  => { /* 40 linhas de layout de figura */ }
    // … 59 arms bespoke, 1857 linhas …
}
```
Depois:
```rust
// layout/mod.rs (magro — match exaustivo preservado, corpos de 1 linha)
match content {
    Content::Heading(h) => h.layout(ctx),
    Content::Figure(f)  => f.layout(ctx),
    // … 59 arms, 1 linha cada — exaustividade e jump table intactas …
}
// heading.rs (a lógica atomizada, legível sozinha)
impl HeadingElem { fn layout(&self, ctx) { /* as 50 linhas, aqui */ } }
```

- O enum `Content` **fica fechado**, com as variantes (modelo da ADR-0026/0105-D).
- O `match` **fica sem wildcard** (exaustivo) — o compilador continua a pegar o elemento esquecido.
- O despacho **fica estático** (jump table) — sem ganho nem perda algorítmica; **sem vtable** (a
  ADR-0026 é satisfeita, não emendada).
- A lógica de cada elemento **muda para o arquivo do elemento**.

---

## Relação com as outras ADRs

- **ADR-0026** (enum sem vtable): **satisfeita.** A atomização não introduz vtable; é o "enum linear
  sem vtable" que a 0026 prescreve, só com a lógica movida para os métodos do elemento.
- **ADR-0105** (modelo D, exaustividade cl.3): **satisfeita e reforçada.** A exaustividade do `match`
  é **mantida** (não há despacho dinâmico a repor com teste/lint).
- **ADR-0107** (paridade com a linguagem): ortogonal — a atomização é estrutural, não muda
  comportamento (content-preserving).

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

> **Atomização** (ADR-0110) = mover a lógica de arquivos monolíticos para o arquivo da unidade dona
> (cada elemento/feature legível sozinho). **NÃO** é zerar `content→elements`, **NÃO** é
> desacoplamento de imports, **NÃO** usa vtable/`dyn`/PropMap, **NÃO** remove o `match` exaustivo. A
> forma: o `match` no núcleo fica magro (corpos delegam a `elem.metodo()`); a lógica gorda muda para
> o arquivo do elemento. O `match` exaustivo, a jump table e os imports **ficam**. A métrica da
> lente (`content→elements`) é **irrelevante** para a atomização — não a use como gate. Se um plano
> propuser despacho dinâmico ou "zerar `content→elements`" em nome de atomização, **isso é a deriva
> que esta ADR proíbe** — pare e separe os dois significados.

---

## Escopo de aplicação (decisão do dono, P376)

- **Primeiro os elementos** que o arco F trabalhou (atomizar layout/introspect/hub-logic de cada
  elemento para o arquivo do elemento).
- A **varredura do projeto inteiro** (todo monólito, não só os de elemento) fica para **depois de
  finalizar** os elementos.
- A **decisão de crates** (criar crates novos dentro das camadas para agrupamentos) fica para
  **depois** — começar movendo para **arquivos dentro dos crates atuais**.
