# Passo 347d — a Revocation é função de linguagem ou mecanismo interno? — **VEREDITO: INTERNA**

> **Veredito: INTERNA.** A `Style::Revocation` do vanilla é **mecanismo interno da
> realização**, não função de linguagem: é **construída só pelo motor** (um único ponto,
> `typst-realize/src/lib.rs:1301`, no caminho de aplicação de text rules, a partir de
> estado interno `m.id`), **não tem porta de usuário** (zero `#[func]`/método/cast/sintaxe
> que a produza em todo o `lab/`), e o **próprio comentário** a descreve como dispositivo
> interno ("*Disables a specific show rule recipe … the only place **we** need it*"). Pela
> ADR-0107 (existir na implementação ≠ ser linguagem), o cristalino **não lhe deve
> paridade**: o modelo α (ponto-fixo morfológico + detecção de ciclo) **substitui** o que a
> Revocation fazia, por outro caminho, sem carregá-la. Read-only: árvore da `tekt` intacta,
> lint 0/0, suíte 2723/3242 não re-rodada.

## Pré-condição
P347c fechado (Revocation localizada). Branch `Tekt`, zero modificações rastreadas, lint
0/0. Quarentena read-only mantida (só `grep`/leitura no `lab/`; sem checkout/merge).

## S1 — quem CONSTRÓI uma `Style::Revocation`?
**Um único ponto de construção em todo o `lab/`:**
- `typst-realize/src/lib.rs:1301` — `let revocation = Style::Revocation(m.id).into();`
  Dentro de `typst-realize` (o **motor**), no caminho de aplicação de **text rules**
  (regex). Contexto (`:1295-1304`): o motor aplica o recipe (`m.recipe.apply(...)`) a um
  trecho casado, produz `output`, e **encadeia** `Style::Revocation(m.id)` na chain do
  output antes de `visit(output)` — para o **mesmo** recipe (`m.id`) não re-casar o próprio
  output. `m.id`/`m.recipe`/`m.styles` são **estado interno** da realização (não vêm de
  valor de usuário). → **construída pelo motor: indício INTERNA.**

(A definição `Style::Revocation(RecipeIndex)` está em `foundations/styles.rs:225`; os
acessores `property()`/`recipe()`/etc. devolvem `None`/`false` para ela — não é uma
propriedade exposta.)

## S2 — há porta da LINGUAGEM para ela?
**Não.** Busca em **todo** `lab/typst-original/crates/` por
`#[func]`/`fn .*revoke`/`.revoke`/`cast`/`FromValue`/`IntoValue` relacionados a
revocation: **zero resultados**. Não há função de stdlib, método de `content`/`style`, nem
sintaxe de `#show` que produza uma `Style::Revocation`. `RecipeIndex` (o payload) é um
índice interno de recipe, não um tipo que o autor constrói. → **sem porta de usuário:
INTERNA.**

**Evidência direta da intenção (o comentário da definição, `styles.rs:220-225`):**
> "Disables a specific show rule recipe. Note: This currently only works for regex recipes
> since it's the only place **we** need it for the moment. Normal show rules use guards
> directly on elements instead."

— "*Disables a … recipe*" + "*the only place **we** need it*" (we = implementadores do
motor) + "*Normal show rules use guards directly on elements*": descreve um **dispositivo
interno** da realização (para o caso regex), explicitamente do ponto de vista do motor,
**não** uma feature que o autor escreve.

## S3 — uso em testes do vanilla (código de usuário ou só motor?)
Busca em `lab/.../tests/` por `revoke`/`revocation`: o **único** hit é
`tests/suite/model/par.typ:339` — `#let revoke = metadata("revoke")`, uma **variável de
usuário chamada "revoke"** usada como sentinela de `metadata` para comparar `bibliography.title`
(`:341-342`). **Nada a ver** com `Style::Revocation` — é homônimo. **Nenhum teste escreve
uma revogação** em código `.typ`. → reforça **INTERNA** (nem os testes a expõem como
linguagem).

## VEREDITO — **INTERNA**
A `Style::Revocation` é **mecanismo interno da realização** (S1: só motor-construída, um
ponto; S2: nenhuma porta de usuário; S3: nenhum uso de usuário; comentário a declara como
dispositivo do motor). Pela **ADR-0107**, é **mecânica** — existir na implementação do
vanilla **não** a torna linguagem.

**Implicação (decide o que α inclui; não escolhe α/β/γ):** o lote do α **não carrega** a
Revocation. O modelo α (ponto-fixo morfológico + detecção de ciclo) **substitui** o papel
que a Revocation+guard cumprem (limitar o auto-casamento), por outro caminho, **sem dever
paridade** e **sem** expor uma revogação ao autor (porque o vanilla também não a expõe).
Registrar como **mecânica divergente consciente**.

**Nota de coerência:** o papel concreto da Revocation é limitar **text rules** regex
(apply-once). No cristalino, text rules já são **passe único** (`rules.rs:175-181`,
`map_text` substitui uma vez, sem revisitar) — então o que a Revocation evita (re-casar o
próprio output de uma regex rule) **já não ocorre** no modelo cristalino. α trata a
recursão de **element** rules (heading etc.); a Revocation (regex) é ortogonal e interna.
*(Inferência marcada: a ortogonalidade text-vs-element é leitura da fonte; o ponto do
veredito — INTERNA — independe dela.)*

## Verificação (gates)
```
content-preserving: zero código/teste/.rs/.toml. Árvore da `tekt` intacta (git status de
  produto limpo). Suíte 2723/3242 não re-rodada. Quarentena: só grep/leitura, sem
  checkout/merge.
lint: crystalline-lint . = 0/0.
evidência: construção única em lib.rs:1301 (motor); ausência de #[func]/método/cast
  confirmada em todo crates/; comentário styles.rs:220-225; S3 hit homônimo identificado.
  Inferências marcadas. Zero "~".
fronteira: não escreveu a recursão; não escolheu α/β/γ; entregou o veredito e parou.
```

## Mapa de filtro (campo)
**Lugar lógico:** antes de decidir o que o modelo de terminação do cristalino inclui, o
projeto separou o que da Revocation é **linguagem** (respeitar) do que é **mecânica**
(substituir) — **ADR-0107 aplicada a uma feature concreta** do vanilla; a tentação de
"respeitar tudo que existe no Typst" foi **medida, não assumida**, e a medição disse
**mecânica**. **Rastro:** P347c achou a Revocation e o dono inclinou a respeitá-la por ser
"função do Typst"; P347d mede e conclui que é **função do motor**, não da linguagem —
então α a substitui, não a carrega.

## Item carregado
`content→elements` aponta para o **Marco G** (P346) — não volta como órfão.
```
