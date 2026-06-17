# ⚖️ ADR-0107: Paridade é com a linguagem, não com a mecânica

**Status**: `EM VIGOR`
**Data**: 2026-06-17

---

## Contexto

Um princípio governava as decisões boas do projeto sem nunca ter sido escrito:
**a paridade do crystalline com o Typst é com a *linguagem* — não com a *mecânica*
de execução que a produz.** Por estar implícito, foi redescoberto — e redescoberto
**errado** — três vezes:

- **β1 (P339)** — tratou um wrapper de transporte de estilo (`Content::Styled` de
  numbering local) como se fizesse parte da forma da linguagem.
- **A "noite" (P340)** — assumiu que a fidelidade exigia copiar a *mecânica* de passes
  do vanilla (fixpoint/multi-passe), quando o contrato é a saída, não o algoritmo.
- **O `==` de conteúdo (P342)** — mediu igualdade de conteúdo pelo `PartialEq` do Rust
  (estrutural/mecânico), que observa estilo de render assado (Achado 2: `it.body == [a]`
  falha por um `bold` de render, não por diferença de linguagem).

Cada tropeço é o **erro inverso do P329**. O P329 diz "não copie a *estrutura* do
vanilla; a fidelidade é comportamental". Esta ADR fecha a outra metade: **não *meça* a
linguagem pela estrutura** (nem pela igualdade do Rust, nem por bytes de saída, nem por
passos de algoritmo). As duas metades são o mesmo princípio.

---

## Decisão

A paridade do crystalline com o Typst é definida em **três níveis da linguagem**:

- **Semântica** — o que a construção **significa**. Ex.: `1 == 1.0` é `true`
  (ADR-0025); o `==` de conteúdo compara o conteúdo **pelo que ele é**, não pela sua
  estrutura de dados.
- **Sintaxe** — como a construção se **escreve** (a gramática `.typ`).
- **Morfologia** — *(termo introduzido por esta ADR; não usado antes no projeto)* — a
  **forma do conteúdo enquanto objeto da linguagem**: o texto, a estrutura de markup, o
  estilo **semântico** (`*bold*`, `_italic_`) — **distinta** do estilo **resolvido de
  render** que a implementação assa ou deriva.
  - Fronteira por exemplo: em `= a`, a morfologia de `it.body` é o texto **"a"**; o
    **bold** do heading é estilo de **render**, **não** faz parte da morfologia. Logo
    `it.body` e `[a]` têm a **mesma morfologia** e a linguagem **deve** tratá-los como
    iguais.

**O que NÃO é paridade (é mecânica; diverge de propósito — P329 / ADR-0033 / ADR-0026):**

- a igualdade restrita do Rust (`#[derive(PartialEq)]` / comparação estrutural sobre as
  structs);
- os bytes exatos da saída renderizada;
- os passos do algoritmo (ordem de passes, eager vs multi-passe, …);
- a forma da estrutura de dados interna (qual variante, que campos, que wrappers).

**Como a paridade se mede.** Por semântica/sintaxe/morfologia da construção — **não**
por diff de bytes de saída nem por um `==` mecânico devolver o mesmo booleano. O
**vanilla compilado é oráculo da *semântica* quando a fonte é ambígua** (gatilho de 2º
nível), **não** oráculo de bytes. Medir paridade pela mecânica confunde implementação
com linguagem.

---

## Nota de fundação — esta ADR foi submetida à própria regra ao nascer

O primeiro rascunho desta ADR (no P344) citou como precedentes um **"Princípio V"** e a
**ADR-0029**. A verificação contra a fonte — a mesma disciplina que esta ADR institui —
refutou ambos:

- **"Princípio V" não existe** no projeto: aparecia apenas dentro do texto do passo que
  redigia esta ADR; a ADR-0025 nunca o nomeia. Corrigido para citar a **decisão real** de
  ADR-0025 (a separação entre a igualdade do Rust e a igualdade do Typst).
- **ADR-0029 é "pureza física em L1" (Arc)**, não fidelidade comportamental. O precedente
  comportamental correto é **ADR-0033** (+ **ADR-0026** para a divergência estrutural de
  `Content`), com **P329** como o **norte** — que é um passo, não uma ADR.

Registra-se isto **não como rodapé envergonhado, mas como primeira evidência**: a
definição de paridade foi testada contra a deriva no momento de nascer — confiar na
narrativa em vez da fonte — e resistiu, porque foi medida pela fonte. É o caso-zero da
regra desta ADR aplicada a si mesma.

---

## Precedentes que esta ADR unifica (não cria)

- **P329** — o **norte** (passo, não ADR): a fidelidade é **comportamental** (mesma
  `.typ` → mesma saída), não estrutural; a estrutura Rust é livre.
- **ADR-0033** — "Paridade funcional com vanilla como invariante arquitectural".
  Estabelece que o **comportamento observável** (output visível, sentido de erro, ordem
  visível) é o contrato e a **forma interna diverge** — é o precedente comportamental que
  esta ADR herda. **Nota sobre o nome (verificada na fonte, P344):** o conteúdo de
  ADR-0033 é **comportamental**, não "funcionalidade" no sentido que esta ADR recusa —
  ela própria **exclui bytes**: "bytes diferentes são aceitáveis apenas se a diferença
  for invisível". Os dois usos de "funcional": em 0033 = **comportamento observável** (o
  que a linguagem faz); o sentido que esta ADR rejeita seria "igualar pela mecânica/bytes"
  — que 0033 também rejeita. Esta ADR **refina onde se mede**: no nível da linguagem
  (semântica/sintaxe/morfologia), não por diff do output observável. As duas ADRs
  concordam — 0033 vê o contrato pelo lado do output observável; 0107 pelo lado da
  linguagem.
- **ADR-0026** (+ R1) — `Content` diverge da estrutura do original **de propósito** (enum
  vs vtable): a forma de dados é livre.
- **ADR-0025** — a igualdade do Typst é **semântica** (`1 == 1.0`), **separada** da
  igualdade mecânica do Rust (`PartialEq` derivado, para testes/estruturas de dados; `==`
  do Typst, para eval).
- **Separação linguagem/render (P342)** — a linguagem (eval, semântica, morfologia) é o
  contrato; o render é a saída; a mecânica entre eles diverge.

Esta ADR é a face que faltava: **a paridade é com a linguagem; a mecânica é livre.**

---

## Consequências

**Positivas**: o princípio para de ser redescoberto (e redescoberto errado) a cada lote;
critérios de aceitação passam a ser escritos nos três níveis da linguagem; o termo
**morfologia** dá nome à fronteira conteúdo-de-linguagem vs estilo-de-render.

**Registrada (aplicação imediata, NÃO executada nesta ADR):** o `==` de conteúdo hoje cai
no `PartialEq` do Rust (`content.rs:1711`, via `value.rs:17`) — mecânica — e observa o
estilo de render assado (Achado 2, P342). Pela ADR-0025 estendida ao conteúdo, ele deve
ser **semântico/morfológico**. **Esse conserto é um passo seguinte, não esta ADR.**

**Neutras**: nenhuma estrutura de dados muda por esta ADR; é definição, não refactor.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Deixar o princípio implícito (status quo) | Zero trabalho | Redescoberto errado a cada lote (β1, noite, `==`) |
| Definir paridade pelo **output** (bytes/render) | Teste simples | Confunde linguagem com render; falha morfologia (`it.body == [a]`); é a mecânica que diverge de propósito |
| **Definir paridade pela linguagem (semântica/sintaxe/morfologia)** | **Nomeia o contrato real; unifica P329/0033/0026/0025/P342; dá o critério de medição** | **Exige o termo novo "morfologia" e disciplina ao escrever critérios** |

---

## Referências

- **P329** — norte da fidelidade comportamental (`00_nucleo/materialization/typst-passo-329*`;
  reafirmado em `typst-passo-341-relatorio.md`, `prompts/entities/content.md`,
  `f_fronteira_e1.md`).
- **ADR-0033** — Paridade funcional com vanilla como invariante (comportamento observável).
- **ADR-0026** / **ADR-0026-R1** — `Content` como enum; divergência estrutural intencional.
- **ADR-0025** — `Int == Float`; igualdade semântica do Typst separada da mecânica do Rust.
- **P342** — `00_nucleo/materialization/typst-passo-342-relatorio.md` (separação
  linguagem/render; Achado 2; `==` caindo no `PartialEq` do Rust).
- **P339–P343** — os tropeços (β1, noite, `==`, inventário do bake) que motivam a definição.
- **P344** — passo que redigiu esta ADR (e a nota de fundação acima).
