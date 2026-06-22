# Passo 342 — Auditoria da separação linguagem/render (medição; sem mudança de comportamento)

> **Propósito.** Verificar, da fonte, se a separação comportamental
> **linguagem → render** (contrato P329) se sustenta nos pontos onde as
> divergências recentes batem — e diagnosticar o Achado 2 (igualdade de
> conteúdo) como **vazamento render→linguagem** ou **semântica de linguagem**.
> Não conserta nada.

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P342 (confirmar livre).
**Pré-condição**: P341/P341b (spikes de medição, content-preserving) sem commit
de produto; HEAD = `03619dc93` (último commit do P340). Suíte **2719**
(`typst-core --lib`) / **3238** (workspace), lint 0/0, árvore limpa. Se não bater,
parar.
**Tipo**: **auditoria / medição** — content-preserving estrito: **zero código de
produção, zero teste alterado, zero ficheiro de produto tocado**. Termina na
**TRAVA** com o mapa da separação, para decisão do dono. **Não** consertar a
igualdade, **não** construir o multi-passe, **não** decidir o conserto.
**Objetivo**: produzir (1) o **mapa da fronteira** — para cada ponto onde a
camada de linguagem e a de render se tocam, se a fronteira está limpa ou se há
vazamento, com direção (`render→linguagem` ou `linguagem→render`) e `file:line`;
e (2) o **diagnóstico do Achado 2** — a causa medida de `it.body == [a]` divergir,
classificada em vazamento vs semântica.
**Fontes**: `f_fronteira_e1.md` (§3a.8 β1, §3b modelo de valor), `style.md`,
o L0 do modelo de conteúdo, o contrato P329, os registros do F-3 (a regra "só
conta se observável na linguagem"; o gatilho (c)), o recon eval→layout da noite
(`engine.styles` morre na fronteira; só a árvore `Content` cruza), os relatórios
P341/P341b, `lab/typst-original/` (vanilla como referência do que é
observável-na-linguagem; compilar onde a leitura for ambígua).

---

## A separação, definida operacionalmente (os predicados da auditoria)

Contrato P329: a **linguagem** (o `.typ` + a semântica de avaliação) e a **saída
renderizada** são o contrato; o **mecanismo** entre as duas diverge à vontade.
Regra do F-3: uma divergência só é bug de fidelidade se é **observável na
linguagem e muda a saída**; mecanismo interno diverge livre.

Mapear cada item ao seu lado (parte da auditoria é confirmar esta classificação
contra o código, não assumi-la):
- **Camada de linguagem**: parser; avaliação (`eval/mod.rs`, `eval/rules.rs`); o
  modelo de conteúdo (`entities/content.rs`, o enum `Content`, os módulos de
  elemento); a semântica de valor (`Value`, o `==`/`Eq` sobre `Content`/`Value`);
  o significado de `#show`/`#set` (`rules.rs` `intercept_content`/
  `apply_show_rules`); a superfície de `query`/introspecção.
- **Camada de render**: o layout (`layout/mod.rs`); a realização-como-preparo-de-
  render; a saída visível (frames / `plain_text` / PDF).
- **O caso ambíguo a julgar**: o `Content::Styled` do β1 — vive no modelo de
  conteúdo (estrutura de linguagem), mas existe para **transportar estilo ao
  render**. A auditoria decide se ele vaza para a observação da linguagem.

---

## Fase A — inventário da fronteira (`file:line`, sem código)

Enumerar os pontos onde as duas camadas se tocam e registrar, em cada um, **o que
cruza e quem lê o quê**:
1. A fronteira **eval→layout**: o que cruza (o recon da noite achou: só a árvore
   `Content`; `engine.styles` morre). Reconfirmar e listar.
2. A definição de **igualdade**: onde `PartialEq`/`Eq` de `Content` e de `Value`
   são definidos (`file:line`); de que campos a igualdade depende.
3. **`query`/introspecção**: o que o walk observa do `Content` (já se sabe que
   ignora os styles do `Styled`, `introspect.rs:1204` — reconfirmar).
4. **Matching de `#show`**: por seletor (`rules.rs:88-176`) — confirmar que não
   usa igualdade de árvore.
5. **Acesso a campo na linguagem**: como `it.body` (e os outros campos expostos a
   closures de `#show`) é materializado — ele carrega wrapper/proveniência?

### (cross-check estrutural — opcional, se a lente estiver à mão)
Rodar a lente (`tekt-cargo-dsm@98d8f9e`) e verificar a **direção** das arestas na
fronteira: render (layout) deve depender do modelo de conteúdo (linguagem), não o
contrário. Listar qualquer aresta `linguagem→render` (ex.: `content`/`eval` →
`layout`) como suspeita de vazamento estrutural, para casar com o achado
semântico. (Nota: `content`↔`style` já está no megaciclo de 90 — não confundir o
acoplamento estrutural já conhecido com o vazamento **comportamental** que esta
auditoria mede.)

---

## Fase B — os testes de vazamento (medição; sem código de produção)

Para cada predicado, medir e classificar **limpo** vs **vazamento (+direção)**:

1. **Igualdade de conteúdo não depende de artefato de render.** Construir o par
   mínimo do Achado 2: `it.body` (corpo de `= a`) vs `[a]` (bloco fresco).
   Rastrear, da fonte, **por que diferem** no `Eq`:
   - se diferem por um campo de **wrapper β1 (`Styled`)** ou de **proveniência de
     parse** (span, origem) que um lado tem e o outro não → **vazamento
     render→linguagem** (a igualdade da linguagem está observando estrutura de
     transporte/parse-mecanismo);
   - se diferem por uma definição de igualdade de conteúdo **estrutural** que
     simplesmente não normaliza analisado≡construído como o vanilla → **semântica
     de linguagem** (não é vazamento; é o modelo de valor diferindo do vanilla).
   Medir o que o **vanilla** considera igual aqui (compilar; é a referência do que
   a linguagem promete). Registrar a causa com `file:line`.
2. **O wrapper β1 não é observável pela linguagem.** Para cada operação de
   linguagem — `==`, `query`, matching de `#show`, acesso a campo (`it.body`),
   `plain_text`, `map_*` — verificar se o `Content::Styled` do transporte é
   visível. Qualquer **sim** é vazamento render→linguagem. (A fatia 1 mediu
   `PartialEq=false` para o wrapper; o ponto aqui é se algum caminho de **linguagem**
   o expõe, não só a igualdade estrutural de teste.)
3. **A fronteira eval→layout não realimenta.** Confirmar que nada da camada de
   render volta a influenciar decisão observável na avaliação.
4. **A realização eager é transformação de conteúdo, não leitura de render.**
   Confirmar que o `#show` eager (`intercept_content`) opera sobre `Content`
   (linguagem) e não lê estado de layout.

---

## TRAVA ARQUITETURAL — checkpoint com o mapa

Emitir:
- **O mapa da separação**: cada ponto da fronteira → limpo / vazamento (+direção)
  + `file:line`.
- **O diagnóstico do Achado 2**: vazamento render→linguagem **ou** semântica de
  linguagem, com a causa medida e a comparação com o vanilla.
- **Recomendação de onde o conserto cai** (sem executá-lo):
  - se **vazamento**: o conserto é tirar o artefato de render da igualdade da
    linguagem (ex.: a igualdade ignora o wrapper β1 / a proveniência) — e isso
    pode também reabrir a decisão do wrapper β1 (o gatilho que o P339 registrou),
    agora na camada certa;
  - se **semântica de linguagem**: o conserto é o modelo de valor de conteúdo
    passar a comparar como a linguagem promete (medido contra o vanilla) — fora do
    render e fora do transporte.
- E o que o mapa diz sobre a recursão (P341b): se a causa de m1 é a igualdade
  (camada de linguagem), o guard por-regra (mecanismo de render) só se remede
  **depois** do conserto da igualdade.

Parar aqui. Nenhum código de produção, nenhum teste alterado.

---

## Verificação (gates)

```
content-preserving: zero código de produção, zero teste alterado, zero ficheiro
  de produto tocado. Suíte inalterada: 2719 / 3238.
lint: crystalline-lint . = 0 violations, 0 warnings.
medição reproduzível: comandos de compilação do vanilla + as corridas do
  cristalino registrados; binários do P341.
lente (se usada): só leitura; nada de produto muda.
```

---

## O que NÃO fazer

- **Não consertar a igualdade** nem nada — auditoria só mede.
- **Não alterar nenhum teste**; não construir o multi-passe; não tocar o guard.
- **Não reabrir o wrapper β1 nesta corrida** — só registrar se o mapa o implica;
  a decisão é do dono no checkpoint.
- **Não confundir acoplamento estrutural já conhecido** (o megaciclo content↔style)
  **com vazamento comportamental** — a auditoria mede o segundo.
- **Não estimar com "~"** — cada veredito do mapa é medido.

---

## Relatório (`typst-passo-342-relatorio.md`)

- O mapa da separação (a tabela ponto × veredito × direção × `file:line`).
- O diagnóstico do Achado 2 (vazamento vs semântica), com a causa e o vanilla.
- A recomendação de camada para o conserto, e o que isso implica para a recursão
  do P341b e para o gatilho do wrapper β1.
- Item aberto carregado: `content→elements → 0` (fora da fila, sem dono — as três
  saídas), para decisão, não bloqueio.
