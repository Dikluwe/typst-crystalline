# Passo 344 — Definição: a paridade é com a linguagem, não com a mecânica (ADR + claude.md)

> **O que muda.** Este passo deixou de ser o de-bake. Ele registra como **definição
> explícita** um princípio que já governava as decisões boas (P329, ADR-0025) e que
> eu redescobri errado três vezes (β1, a noite, o `==`): **a paridade do crystalline
> é com a linguagem Typst — semântica, sintaxe, morfologia — não com a mecânica de
> execução que as produz.** Medir paridade pela mecânica (igualdade restrita do Rust,
> bytes de saída, passos de algoritmo) é confundir implementação com linguagem — o
> erro inverso do P329. Documentação pura: zero código, zero teste.

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P344 (confirmar livre).
**Pré-condição**: HEAD = `04383f433` (commit do relatório P343); produto
content-idêntico ao P340. Suíte **2719** (`typst-core --lib`) / **3238**
(workspace), lint 0/0, árvore limpa. Se não bater, parar.
**Tipo**: **definição / documentação** — content-preserving estrito: **zero código
de produção, zero teste, zero ficheiro de produto (`.rs`/`.toml`) tocado**. Toca só
uma ADR nova + o `claude.md`. **Termina na fronteira de revisão do dono**: redige,
apresenta o texto, e **para** antes de `--fix-hashes`/commit — uma definição do que
é paridade é forte demais para selar sem o olho do dono.
**Objetivo**: redigir a ADR que define paridade e a regra operacional curta no
`claude.md`, para o princípio parar de ser redescoberto (e redescoberto errado) a
cada lote. Não é decisão de desenho nova — é a razão implícita virando explícita,
o mesmo movimento do "L0 descreve o estado atual".
**Fontes**: ADR-0029/P329 (fidelidade comportamental; a estrutura diverge),
**ADR-0025** (a igualdade do Typst é semântica, separada da igualdade mecânica do
Rust; Princípio V — a correção semântica do motor sobrepõe-se à conveniência da
linguagem de implementação), relatório P342 (a separação linguagem/render; o `==`
de conteúdo a cair no `PartialEq` do Rust em vez de uma comparação semântica), os
relatórios P339–P343 (os três tropeços que motivam a definição). Convenção de ADR
do repo (ler uma ADR existente para o formato).

---

## O conteúdo a materializar (a substância; o agente rende no formato do repo)

### ADR-00NN — "Paridade é com a linguagem, não com a mecânica" (numerar livre)

**Estado**: PROPOSTO neste passo; EM VIGOR após a revisão do dono.

**Princípio.** A paridade do crystalline com o Typst é com a **linguagem**, definida
em três níveis:
- **Semântica** — o que a construção **significa** (ex.: `1 == 1.0` é verdadeiro; o
  `==` de conteúdo compara o conteúdo pelo que ele é, não pela sua estrutura de dados).
- **Sintaxe** — como a construção se **escreve** (a gramática `.typ`).
- **Morfologia** — a **forma do conteúdo** da linguagem: o que o conteúdo *é* como
  objeto da linguagem (o texto, a estrutura de markup, o estilo **semântico** como
  `*bold*`), distinta do estilo **resolvido de render** que a implementação assa ou
  deriva. **`morfologia` é um termo introduzido por esta ADR** (não usado antes no
  projeto) — defini-lo aqui pela primeira vez, explicitamente, com a fronteira:
  - exemplo: em `= a`, a morfologia de `it.body` é o texto "a"; o **bold** do heading
    é estilo de render, **não** faz parte da morfologia. Por isso `it.body` e `[a]`
    têm a **mesma morfologia** e a linguagem deve tratá-los como iguais.

**O que NÃO é paridade (é mecânica; diverge de propósito — P329):**
- a igualdade restrita do Rust (`PartialEq` derivado/estrutural sobre as structs);
- os bytes exatos da saída renderizada;
- os passos do algoritmo (ordem de passes, eager vs multi-passe, etc.);
- a forma da estrutura de dados interna (qual variante, que campos, que wrappers).

**Como a paridade se mede.** Por semântica/sintaxe/morfologia da construção — não
por diff de saída nem por um `==` mecânico devolver o mesmo booleano. O vanilla
compilado é **oráculo da semântica quando a fonte é ambígua** (o gatilho de 2º
nível), não oráculo de bytes. Medir paridade pela mecânica confunde implementação
com linguagem e é o **erro inverso do P329** (P329 diz "não copie a estrutura"; este
diz "não meça a linguagem pela estrutura").

**Precedentes que esta ADR unifica (não cria):**
- **P329 / ADR-0029** — a fidelidade é comportamental; a estrutura Rust diverge.
- **ADR-0025** — a igualdade do Typst é semântica, separada da igualdade mecânica do
  Rust; a semântica sobrepõe-se à conveniência da implementação (Princípio V).
- **Separação linguagem/render (P342)** — a linguagem (eval, semântica, morfologia)
  é o contrato; o render é a saída; a mecânica entre eles diverge.
  Esta ADR é a face que faltava: as três acima são o mesmo princípio — **a paridade
  é com a linguagem; a mecânica é livre.**

**Consequência registrada (aplicação imediata, não executada aqui):** o `==` de
conteúdo hoje cai no `PartialEq` do Rust (`content.rs:1711`, via `value.rs:17`), que
é mecânica — observa o estilo de render assado (Achado 2, P342). Pela ADR-0025
terminada para o conteúdo, ele deve ser **semântico/morfológico**. Esse conserto é o
**próximo passo**, não este.

### `claude.md` — regra operacional (curta, aponta para a ADR)

Adicionar à secção de princípios/definições do `claude.md`:

> **Paridade.** A paridade é com a **linguagem** Typst — semântica, sintaxe,
> morfologia — **nunca** com a mecânica de execução ou a igualdade restrita do Rust.
> Implementação (estrutura, igualdade do Rust, bytes de saída, passos do algoritmo)
> diverge de propósito (P329). Ao medir paridade ou escrever critério de aceitação,
> usar os três níveis da linguagem, não a mecânica. Ver ADR-00NN.

---

## Fronteira de parada — revisão do dono

Redigir a ADR e a entrada do `claude.md`, **apresentar o texto rendido no chat**, e
**parar**. Não rodar `--fix-hashes`, não commitar, até o dono revisar e aprovar o
texto. (Fronteira do P339: definição forte é selada pelo dono, não pelo agente.)
Após a aprovação — num passo seguinte ou na continuação — selar o hash (se a ADR/o
claude.md entram no contrato de hash do repo) e commitar.

---

## Verificação (gates)

```
content-preserving: zero código de produção, zero teste, zero .rs/.toml tocado.
  Suíte inalterada: 2719 / 3238 (não precisa re-rodar — nada de código mudou; só
  confirmar que a árvore de produto não foi tocada).
lint: crystalline-lint . = 0 violations, 0 warnings (a ADR/claude.md não devem
  introduzir violação de linhagem; se o claude.md está sob hash, NÃO selar antes da
  revisão — é a fronteira).
formato: a ADR segue a convenção de uma ADR existente do repo (ler uma antes).
precedentes citados com referência real (P329/ADR-0029, ADR-0025, P342) — não
  inventar número de ADR; confirmar os números na fonte.
```

---

## O que NÃO fazer

- **Não tocar código** — nenhum `.rs`/`.toml`. Esta é a definição, não o conserto.
- **Não consertar o `==` de conteúdo** — é o próximo passo (a ADR-0025 terminada
  para o conteúdo); aqui só se **registra** que ele é a consequência.
- **Não selar o hash nem commitar antes da revisão do dono** — é a fronteira.
- **Não inventar precedentes nem números de ADR** — confirmar P329/ADR-0025/P342 na
  fonte; numerar a ADR nova pelo próximo livre.
- **Não expandir o escopo** — só a ADR + a regra no claude.md. O de-bake e o `==`
  semântico são lotes seguintes.

---

## Relatório (`typst-passo-344-relatorio.md`)

- O texto rendido da ADR e da entrada do `claude.md` (para a revisão do dono).
- Os números de ADR/precedente confirmados da fonte.
- **Mapa de filtro (campo):** o lugar lógico desta ADR na versão destilada — "a
  definição de paridade mora na fundação do projeto, logo ao lado do P329; é a outra
  metade do mesmo princípio (P329 diz o que não copiar da estrutura; esta diz que a
  linguagem não se mede pela estrutura)" — com o rastro: implícito em P329 e
  ADR-0025; redescoberto errado no β1 (P339), na noite (P340), no `==` (P344 antigo);
  nomeado explicitamente aqui.
- Confirmação de que parou na fronteira (nada selado/commitado) à espera da revisão.
- Item aberto carregado: `content→elements → 0` (fora da fila, sem dono).
```
