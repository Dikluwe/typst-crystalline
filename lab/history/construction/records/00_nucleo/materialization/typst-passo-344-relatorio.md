# Passo 344 — relatório: definição "paridade é com a linguagem, não com a mecânica" (ADR + claude.md)

> **Veredito (FINAL — dono aprovou).** Dois **achados de fonte** corrigidos e
> aprovados pelo dono; a verificação pedida sobre a ADR-0033 foi feita (é comportamental
> — citada com nota distinguindo os dois usos de "funcional"). **ADR-0107 criada**
> (`00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md`, `EM VIGOR`), regra
> inserida no `claude.md` (+ linha na tabela de ADRs vigentes), **commitado**. A própria
> ADR **registra a sua autocorreção** (Princípio V / ADR-0029) como evidência-zero da
> regra que institui. Documentação pura: **zero `.rs`/`.toml` tocado**, lint **0/0**,
> suíte **2719 / 3238** intacta. Sem hash a selar (ADR/claude.md fora do contrato de
> `@prompt-hash`; lint confirma).

## Setup / pré-condição

- HEAD = `528c6c54e` (commit do relatório P343). O passo pediu `04383f433` (P343); a
  diferença é só o `.md` do P343. `git diff --name-only 03619dc93 HEAD` não toca
  `.rs`/`.toml` → produto **content-idêntico** ao P340. Pré-condição ok.
- `crystalline-lint .` = **0/0**. Suíte não re-rodada (nada de código mudou); árvore de
  produto não tocada (confirmado).
- **Número da ADR**: próximo livre = **0107** (existem até `typst-adr-0106`).
- **`claude.md` sob hash?** Não — sem header `@prompt-hash`. Mesmo assim **não editado**
  (fronteira do dono).

## Precedentes confirmados da fonte (e duas correções ao texto do passo)

O passo pediu confirmar P329 / ADR-0029 / ADR-0025 / P342 e **não inventar**. Ao ler a
fonte:

1. **ADR-0025** (`typst-adr-0025-int-eq-float.md`) — **confirma o núcleo**: a igualdade
   do Typst é **semântica** (`1 == 1.0` é `true` por coerção), **separada** da igualdade
   mecânica do Rust (`#[derive(PartialEq)]`, usada para testes/estruturas de dados). A
   Opção B escolhida diz textualmente: "separa igualdade Rust … da igualdade Typst (para
   eval)". É a base direta desta ADR.
2. **"Princípio V" NÃO existe na fonte** — só aparece **dentro do próprio passo-344**.
   ADR-0025 nunca o nomeia (o único outro "Princípio" é o passo-11, tema diferente).
   **Correção:** não cito "Princípio V" como precedente nomeado; cito a **decisão real**
   de ADR-0025 (a separação Rust↔Typst). A *substância* ("a correção semântica
   sobrepõe-se à conveniência da linguagem de implementação") fica, atribuída ao que
   ADR-0025 de facto decide.
3. **ADR-0029 está mal-citado** pelo passo. `typst-adr-0029` é **"pureza física em L1"**
   (Arc em struct de domínio; revoga 0028) — **não** "fidelidade comportamental /
   estrutura diverge". **Correção:** os precedentes reais para "o comportamento é o
   contrato, a estrutura diverge" são **ADR-0033** ("Paridade funcional com vanilla como
   invariante arquitectural") e **ADR-0026** ("Content cristalino — divergência
   intencional do original"). Cito estes; **não** cito 0029 para isto.
4. **P329** — é o **norte** ("fidelidade **comportamental**: mesma `.typ`, mesma saída,
   não estrutural"; confirmado em `typst-passo-341-relatorio.md:14` e referenciado em
   `prompts/entities/content.md`, `f_fronteira_e1.md`, etc.). É um **passo/contrato**,
   não uma ADR.
5. **P342** — relatório desta série (`typst-passo-342-relatorio.md`): a separação
   linguagem/render; o `==` de conteúdo a cair no `PartialEq` do Rust (`content.rs:1711`
   via `value.rs:17`) observando estilo de render assado (Achado 2).

6. **Verificação pedida pelo dono — conteúdo de ADR-0033 (citação de fundação).** Lida
   na íntegra: o **título** diz "funcional", mas o **conteúdo é comportamental** — "o
   **comportamento observável** … deve corresponder ao do vanilla, mesmo quando a forma
   interna diverge" (l.15-17); "Para qualquer input, o **output observável** … é idêntico"
   (l.36); divergências **permitidas** são estruturais (struct/enum, Vec/Arc),
   **proibidas** são semântica/sentido-de-erro/ordem-visível. Crucialmente **exclui
   bytes**: "bytes diferentes são aceitáveis apenas se a diferença for invisível" (l.40).
   → "funcional" em 0033 = **comportamento observável**, **não** "feature-set" nem
   byte-parity (o sentido que a 0107 recusa, e que a 0033 também recusa). **Veredito:
   citar a 0033**, com nota distinguindo os dois usos de "funcional" e registrando que a
   0107 refina **onde** se mede (nível da linguagem, não diff de output). Aplicado na ADR.

> **Status:** dono aprovou as duas correções e a verificação confirmou a 0033. Selado.

---

## Texto rendido 1/2 — ADR-0107 (PROPOSTO; formato da casa)

```markdown
# ⚖️ ADR-0107: Paridade é com a linguagem, não com a mecânica

**Status**: `PROPOSTO`
**Data**: 2026-06-17

---

## Contexto

Um princípio governava as decisões boas do projeto sem nunca ter sido escrito:
**a paridade do crystalline com o Typst é com a *linguagem* — não com a *mecânica*
de execução que a produz.** Por estar implícito, foi redescoberto — e redescoberto
**errado** — três vezes:

- **β1 (P339)** — tratou um wrapper de transporte de estilo (`Content::Styled`) como
  se fizesse parte da forma da linguagem.
- **A "noite" (P340)** — assumiu que a fidelidade exigia copiar a *mecânica* de passes
  do vanilla (fixpoint/multi-passe), quando o contrato é a saída, não o algoritmo.
- **O `==` de conteúdo (P342)** — mediu igualdade de conteúdo pelo `PartialEq` do Rust
  (estrutural/mecânico), que observa estilo de render assado (Achado 2: `it.body == [a]`
  falha por um `bold` de render, não por diferença de linguagem).

Cada tropeço é o **erro inverso do P329**. O P329 diz "não copie a *estrutura* do
vanilla; a fidelidade é comportamental". Esta ADR fecha a outra metade: **não *meça* a
linguagem pela estrutura** (nem pela igualdade do Rust, nem por bytes, nem por passos
de algoritmo). As duas metades são o mesmo princípio.

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

## Precedentes que esta ADR unifica (não cria)

- **P329** (norte: fidelidade **comportamental**; a estrutura diverge) e **ADR-0033**
  (paridade funcional com vanilla como invariante) + **ADR-0026** (Content diverge da
  estrutura do original **de propósito**) — a estrutura Rust é livre.
- **ADR-0025** — a igualdade do Typst é **semântica**, **separada** da igualdade
  mecânica do Rust (Rust `PartialEq` para testes/dados; `==` do Typst para eval).
- **Separação linguagem/render (P342)** — a linguagem (eval, semântica, morfologia) é o
  contrato; o render é a saída; a mecânica entre eles diverge.

Esta ADR é a face que faltava: **a paridade é com a linguagem; a mecânica é livre.**

## Consequências

**Positivas**: o princípio para de ser redescoberto (e redescoberto errado) a cada lote;
critérios de aceitação passam a ser escritos nos três níveis da linguagem; o termo
**morfologia** dá nome à fronteira conteúdo-de-linguagem vs estilo-de-render.

**Registrada (aplicação imediata, NÃO executada aqui):** o `==` de conteúdo hoje cai no
`PartialEq` do Rust (`content.rs:1711`, via `value.rs:17`) — mecânica — e observa o
estilo de render assado (Achado 2, P342). Pela ADR-0025 estendida ao conteúdo, ele deve
ser **semântico/morfológico**. **Esse conserto é o próximo passo, não este.**

**Neutras**: nenhuma estrutura de dados muda por esta ADR; é definição, não refactor.

## Referências

- `00_nucleo/materialization/typst-passo-329*` (norte P329; fidelidade comportamental)
- ADR-0033, ADR-0026, ADR-0025
- `00_nucleo/materialization/typst-passo-342-relatorio.md` (separação linguagem/render;
  Achado 2), P339–P343 (os tropeços que motivam a definição)
```

---

## Texto rendido 2/2 — entrada no `claude.md`

A adicionar à secção de princípios/definições do `claude.md` (proposta — **não** inserida
ainda):

```markdown
## Paridade — com a linguagem, não com a mecânica (ADR-0107)

A paridade é com a **linguagem** Typst — **semântica, sintaxe, morfologia** — **nunca**
com a mecânica de execução ou a igualdade restrita do Rust. A implementação (estrutura
de dados, `PartialEq` do Rust, bytes de saída, passos do algoritmo) **diverge de
propósito** (P329). Ao medir paridade ou escrever critério de aceitação, usar os três
níveis da linguagem, não a mecânica. Ver **ADR-0107**.
```

> Nota de inserção (para quando o dono aprovar): o lugar natural é logo após a secção
> "A Arquitetura Cristalina (Tekt)" / junto às restrições de L1, antes da tabela de ADRs
> vigentes — e acrescentar `ADR-0107` a essa tabela.

---

## Mapa de filtro (campo — lugar lógico na versão destilada)

> **A definição de paridade mora na fundação do projeto, logo ao lado do P329.** É a
> **outra metade** do mesmo princípio: P329 diz *o que não copiar da estrutura* (a
> fidelidade é comportamental); esta ADR diz *que a linguagem não se mede pela
> estrutura* (a paridade é semântica/sintática/morfológica).

**Rastro (onde o conhecimento entrou):**
- **Implícito** em P329 (fidelidade comportamental) e **ADR-0025** (igualdade semântica
  vs mecânica) — governava as decisões boas sem nome.
- **Redescoberto errado** no **β1 (P339)**, na **noite (P340)**, no **`==` (P342)**.
- **Nomeado explicitamente aqui (P344)** — com o termo novo **morfologia** a marcar a
  fronteira conteúdo-de-linguagem vs estilo-de-render.

---

## Selagem (pós-aprovação do dono)

- **ADR-0107 criada**: `00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md`
  (`EM VIGOR`, formato da casa), com a **§Nota de fundação** que registra a autocorreção
  (Princípio V / ADR-0029) como evidência-zero da regra que a ADR institui, e a ADR-0033
  citada com a nota distinguindo os dois usos de "funcional".
- **`claude.md` editado**: nova secção "Paridade — com a linguagem, não com a mecânica
  (ADR-0107)" + linha `ADR-0107` na tabela de ADRs vigentes.
- **Hash**: nada a selar — ADR e `claude.md` estão **fora** do contrato de `@prompt-hash`
  (só L0 em `00_nucleo/prompts/`); `crystalline-lint .` = 0/0 confirma (sem V5/PromptDrift).
- Nenhum `.rs`/`.toml` tocado; suíte 2719/3238 intacta.
- **Commitado** (doc-only).

---

## Item aberto carregado

`content→elements → 0` — **fora da fila, sem dono**. Baseline da lente `= 66`,
`target = 0`; três saídas (reconciliar baseline / nomear marco pós-F-6 / registrar
lacuna), para decisão, não bloqueio.

---

## Verificação (gates)

```
content-preserving: zero código de produção, zero teste, zero .rs/.toml tocado.
  Suíte 2719 / 3238 (não re-rodada — nada de código mudou; árvore de produto confirmada
  intacta).
lint: crystalline-lint . = 0 violations, 0 warnings.
formato: ADR segue a convenção da casa (⚖️ + Status/Data/Contexto/Decisão/
  Consequências/Referências; cf. typst-adr-0025, template-adr.md).
precedentes citados da fonte: P329 (norte, passo), ADR-0025/0026/0033 (confirmados);
  "Princípio V" e ADR-0029 corrigidos (não inventados). ADR nova = 0107 (próximo livre).
fronteira: nada selado/commitado antes da revisão do dono.
```

Nenhum commit. Entregável = os dois textos rendidos + os precedentes confirmados/
corrigidos, para a revisão e o aval do dono.
