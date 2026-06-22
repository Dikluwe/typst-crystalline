# Passo 347d — a Revocation é função de linguagem ou mecanismo interno?

> **A pergunta única.** O P347c revelou a `Style::Revocation` do vanilla (o que torna o
> auto-casamento de text rules limitado). A decisão do modelo de terminação do cristalino
> (α: ponto-fixo morfológico) depende de **uma** coisa: a Revocation é **função da
> linguagem** (o autor a escreve no `.typ` → é superfície que documentos reais tocam →
> **respeitar**, α carrega-a) ou **mecanismo interno** da realização (o motor a insere
> sozinho, o usuário nunca a escreve → é mecânica → pela ADR-0107 o cristalino a
> **substitui** pelo seu modelo, sem dever paridade)? Este passo responde isso e **só
> isso**. Não escreve a recursão, não escolhe o lote.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P347d (confirmar livre).
**Pré-condição**: P347c fechado (Revocation localizada: `Style::Revocation(RecipeIndex)`,
`foundations/styles.rs:225`; uso em `typst-realize/src/lib.rs:1223-1259`). HEAD pós-P346
na `tekt`; suíte **2723** / **3242**, lint 0/0, árvore limpa. Se não bater, parar.
**Tipo**: **medição de superfície** — content-preserving estrito: **zero código, zero
teste, zero `.rs`/`.toml`, zero mudança na árvore**. Read-only na fonte do `lab/` (e, se
útil e read-only, no histórico da `main` sob a quarentena do P347c). Termina na **TRAVA**
com o veredito. **NÃO** escrever a recursão; **NÃO** escolher o formato do lote do α.
**Objetivo**: classificar a Revocation em **LINGUAGEM** (há caminho do código de usuário
que a produz) ou **INTERNA** (só o motor a cria), com evidência `file:line`.
**Fontes**: `lab/typst-original/` — `foundations/styles.rs` (a definição de
`Style::Revocation`), `typst-realize/src/lib.rs:1223-1259` (o uso), e os pontos onde uma
`Revocation` é **construída** (quem a cria). A doc/histórico da `main` só se a fonte for
ambígua e sob a quarentena read-only do P347c. Relatórios P347c (a descrição), ADR-0107
(o critério: linguagem = paridade; mecânica = diverge).

---

## A medição (a única pergunta, em três sondas)

### S1 — quem CONSTRÓI uma `Style::Revocation`?
Da fonte (`git grep`/leitura no `lab/`): achar **todos** os pontos onde uma
`Style::Revocation(...)` é criada. Para cada um, classificar a origem:
- **construída pelo motor** (dentro de `typst-realize`/`typst-library`, no caminho de
  aplicação de show rules, sem passar por valor de usuário) → indício **INTERNA**;
- **construída a partir de valor/chamada de usuário** (uma função de stdlib, um método de
  `content`/`style`, uma conversão de um `Value` que o autor produz) → indício
  **LINGUAGEM**.
Citar `file:line` de cada ponto de construção.

### S2 — há porta da LINGUAGEM para ela?
Procurar uma API exposta ao autor que produza a revogação:
- uma função na stdlib (`typst-library/src/.../*.rs` com `#[func]`) cujo nome/efeito seja
  revogar/desativar uma recipe;
- um método em `content` ou `style` chamável do `.typ` (ex.: algo como `.revoke()` ou um
  campo de `#show`);
- sintaxe de `#show` que gere a revogação a partir do que o autor escreve.
Se **existe** → **LINGUAGEM** (o autor pode escrevê-la; documentos reais podem depender).
Se **não existe** (nenhuma `#[func]`/método/sintaxe a produz; ela só nasce no motor) →
**INTERNA**.

### S3 — confirmação por uso (opcional, se a `main` ajudar, read-only)
Se S1/S2 forem ambíguos: procurar nos **testes** do vanilla (`tests/`) ou em exemplos se
algum **código de usuário** invoca a revogação diretamente (não o motor). Um teste que
escreve a revogação em `.typ` confirma LINGUAGEM; só testes do comportamento de
terminação (sem escrever revogação) reforçam INTERNA. (Não trocar de branch; `git show`/
`grep` read-only.)

---

## TRAVA — o veredito (parar; não escrever a recursão)

Emitir, com `file:line`:
- **LINGUAGEM** — há caminho do código de usuário que produz a Revocation (S1/S2 com a
  API/sintaxe citada). → **respeitar**: o lote do α **carrega** a Revocation como recurso
  de compatibilidade (o autor pode usá-la; α termina por morfologia **e** honra a
  revogação explícita quando escrita). 
- **INTERNA** — a Revocation só nasce no motor; nenhuma porta de usuário (S1 só
  motor-construído, S2 sem API). → pela ADR-0107 é **mecânica**: o cristalino **não** a
  reproduz; o modelo α (ponto-fixo morfológico + detecção de ciclo) **substitui** o que a
  Revocation fazia, por outro caminho, sem dever paridade. Registrar como mecânica
  divergente consciente.
- **AMBÍGUO** — se a fonte não decide (raro): registrar o que pende e o que confirmaria
  (a doc de usuário do Typst, que eu — o assistente — posso buscar na web por fora).

Isto **decide** se o lote do α leva ou não a Revocation — não escolhe α/β/γ (α já é a
inclinação do dono; este passo só dimensiona o que α inclui). **Parar** para o dono.

---

## Verificação (gates)
```
content-preserving: zero código/teste/.rs/.toml. Árvore da `tekt` intacta (git status
  limpo). Suíte 2723/3242 não re-rodada. Quarentena: só leitura (grep/show), sem
  checkout/merge.
lint: crystalline-lint . = 0/0.
evidência: cada ponto de construção da Revocation com file:line; a API de usuário (se
  houver) com file:line; o veredito LINGUAGEM/INTERNA sustentado. Inferências marcadas.
  Zero "~".
fronteira: não escreveu a recursão; entregou o veredito e parou.
```

---

## O que NÃO fazer
- **Não escrever a recursão** nem nenhum código.
- **Não escolher α/β/γ** — o dono já inclina para α; este passo só decide se α inclui a
  Revocation.
- **Não tratar "a Revocation existe no código do vanilla" como prova de que é linguagem**
  — existir no motor é INTERNA; só uma porta de usuário a torna LINGUAGEM. A distinção é o
  objeto do passo (ADR-0107: existir na implementação ≠ ser linguagem).
- **Não trocar de branch nem trazer código da `main`** — quarentena read-only do P347c.
- **Não estimar com "~"**.

---

## Relatório (`typst-passo-347d-relatorio.md`)
- S1: os pontos de construção da `Style::Revocation` (`file:line`), cada um classificado
  motor vs usuário.
- S2: a porta de linguagem — existe uma `#[func]`/método/sintaxe que a produz? `file:line`
  ou a ausência confirmada.
- S3 (se usado): uso em testes/exemplos do vanilla — código de usuário ou só motor.
- **VEREDITO**: LINGUAGEM (α carrega a Revocation) / INTERNA (α a substitui) / AMBÍGUO (o
  que confirmaria; o assistente busca a doc por fora).
- **Mapa de filtro (campo):** o lugar lógico — "antes de decidir o que o modelo de
  terminação do cristalino inclui, o projeto separou o que da Revocation é **linguagem**
  (respeitar) do que é **mecânica** (substituir) — ADR-0107 aplicada a uma feature
  concreta do vanilla; e a tentação de 'respeitar tudo que existe no Typst' foi medida,
  não assumida" — com o rastro (P347c achou a Revocation; o dono inclinou a respeitá-la
  por ser 'função do Typst'; P347d mede se é função de linguagem ou de motor).
- Item: `content→elements` aponta para o Marco G (P346).
```
