# Passo 376 — Atomização dos elementos (ADR-0110): mover a lógica para o arquivo do elemento

> **O que faz.** Primeira aplicação da **ADR-0110** (atomização): move a lógica de **layout /
> introspect / hub** de cada elemento — hoje amontoada nos `match` monolíticos do núcleo
> (`layout/mod.rs` 1857 linhas/59 arms; `introspect.rs` 43 arms; o hub de `content.rs`) — para o
> **arquivo do elemento** (`heading.rs`, `figure.rs`, …), de modo que cada elemento fique **legível
> sozinho**. O `match` no núcleo **permanece exaustivo**, com os corpos a **delegar**
> (`Content::Heading(h) => h.layout(ctx)`); a **jump table** e os **imports** ficam. **NÃO** usa
> `dyn`/vtable/PropMap, **NÃO** zera `content→elements` (a métrica da lente é irrelevante —
> ADR-0110), **NÃO** perde exaustividade. **Content-preserving**: a lógica **muda de arquivo, não
> muda de comportamento** (paridade pela rede de caracterização). Começa por **gravar a ADR-0110 +
> o trecho do `claude.md`**. Escopo: os **elementos do arco F** primeiro; a varredura do projeto e a
> decisão de crates ficam para **depois** (decisão do dono, P376). **Design-first**: L0 + Trava
> antes de mover código. **Commita ao terminar** e **não emenda o seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P376 (confirmar livre).
**Pré-condição**: F fechado (P373 commitado); a medição do P375 (a infra de delegação do hub já
existe; o layout/introspect são bespoke e atomizáveis por delegação). Suíte verde, lint **0/0**.
HEAD pós-P375. Lente `tekt-cargo-dsm` disponível (instrumento; **não é gate** — ADR-0110). Caveat de
stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: atomização — **content-preserving** (a lógica move para o arquivo do elemento; o
comportamento é idêntico). A **rede de caracterização (+11, P331)** é o oráculo de paridade. A
métrica é de **leitura** (tamanho do monólito ↓, lógica do elemento no arquivo do elemento), **não**
a lente.

---

## Estágio 0 — gravar a ADR-0110 + o `claude.md` (antes da medição)

Gravar `00_nucleo/adr/typst-adr-0110-atomizacao.md` (a definição + as não-metas + a forma canónica +
o erro histórico corrigido). Adicionar o **trecho da ADR-0110 ao `claude.md`** (o bloco "Atomização
= … NÃO é zerar content→elements / NÃO usa vtable / NÃO remove o match exaustivo"). Isto fixa o
critério antes de qualquer movimento — para a IA (e este passo) não derivar para o desacoplamento.

---

## Fase A — medir o monólito + desenhar a delegação (a fonte vence; `file:line`)

1. **Reler** a ADR-0110 (a forma canónica), a ADR-0026/0105 (enum sem vtable, exaustividade), as
   **Travas anti-deriva**, e o P375 (os 59 arms de layout / 43 de introspect bespoke).
2. **Medir os monólitos**, com `file:line`: para `layout/mod.rs` (o maior — 1857 linhas, 59 arms) e
   `introspect.rs` (43 arms), e o hub de `content.rs` (se ainda houver lógica gorda além da
   delegação), listar **por elemento** quanta lógica vive no monólito (linhas) e o que ela lê (os
   campos concretos `h.level`, `e.caption`, etc.) — para a delegação levar a lógica **inteira**, não
   um pedaço.
3. **Desenhar a forma da delegação** (a forma canónica da ADR-0110): o método no elemento
   (`impl HeadingElem { fn layout(&self, ctx) {…} }`) e o arm magro no núcleo
   (`Content::Heading(h) => h.layout(ctx)`). Medir o que o método precisa receber (o `ctx`/engine/
   chain) para a lógica funcionar igual de dentro do arquivo do elemento. **A forma sai da medição**
   (Trava 1) — confirmar que nada da lógica depende de estar fisicamente no `match` (ex.: acesso a
   estado local do layouter que precise ser passado).
4. **Confirmar as não-metas** (ADR-0110): o `match` continua exaustivo (sem wildcard); o despacho
   continua estático (sem `dyn`); os imports ficam (`content→elements` inalterado — registrar como
   fato, não gate). Se a medição mostrar que mover a lógica **exige** `dyn` ou wildcard, **parar e
   reportar** (seria violar a ADR-0110).
5. **O escopo e a ordem**: quais elementos atomizar primeiro (os do arco F / os de maior lógica no
   monólito), e se cabe num lote ou fatia por elemento/grupo (válvula).

---

## Limites duros (da ADR-0110)

- **Manter o `match` exaustivo** (sem wildcard) — a garantia do compilador fica.
- **Despacho estático** — sem `dyn`/vtable/PropMap. Se a lógica exigir despacho dinâmico para mover,
  **parar e reportar**.
- **Imports ficam** — `content→elements` não é gate; não tentar reduzi-lo (isso é desacoplamento,
  fora da ADR-0110).
- **Content-preserving** — a lógica move, não muda. A rede de caracterização passa **sem alteração**;
  qualquer asserção que mude é sinal de que a lógica mudou ao mover → **investigar, não mascarar**.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag P350c, o F-5b (fechado).

---

## Estágios

### Estágio L0 — desenho (com a medição) + Trava (PARA aqui para o dono)
Com os monólitos medidos e a forma da delegação desenhada, registrar no L0 (`f_fronteira_e1.md` ou
um entity de layout/atomização) a forma canónica aplicada, os elementos do lote, e a confirmação das
não-metas (exaustividade/estático/imports). Sincronizar hashes. **TRAVA**: para no chat — a medição
+ o desenho + o escopo (quais elementos) + os hashes para o dono aprovar. Nenhum código movido antes.

### Estágio 1 — mover a lógica (após aprovação)
Por elemento do lote: a lógica de layout (e introspect, se aplicável) **muda** do `match` para um
método no arquivo do elemento (`impl XElem { fn layout/walk(…) }`); o arm do núcleo vira a delegação
de uma linha. **Aditivo-neutro**: o comportamento é idêntico; só o endereço da lógica muda.

### Estágio Teste
- A **rede de caracterização (+11)** e a suíte passam **sem alteração** — content-preserving (a
  lógica move, não muda). Qualquer asserção que mude → investigar (a lógica mudou ao mover).
- **Leitura (a métrica da ADR-0110)**: registrar o tamanho do monólito antes/depois (`layout/mod.rs`
  encolhe X linhas) e que a lógica do elemento agora vive no arquivo do elemento.
- Confirmar: o `match` exaustivo (sem wildcard), o despacho estático (sem `dyn`), os imports
  (`content→elements` inalterado), e o α/caso 2/caso 4/flag intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes, **commitar**: `git add -A && git commit -m "Passo 376 — atomização dos
elementos (ADR-0110): lógica de layout/introspect para os arquivos dos elementos"`. Árvore limpa e
commitada. (Para na Trava → commita só a ADR-0110 + L0; reverte por contradição → commita só o
relatório.)

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): inalterada em número (content-preserving) — a rede +11 e a suíte
  passam SEM asserção virada. Se alguma virar, a lógica mudou ao mover → investigar.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (a métrica é de LEITURA — ADR-0110):
  - o monólito encolhe (layout/mod.rs −X linhas; a lógica de cada elemento move para o seu arquivo).
  - cada elemento atomizado é legível sozinho (a sua lógica de layout vive no seu arquivo).
  - content-preserving: comportamento idêntico (rede de caracterização).

NÃO-METAS confirmadas (ADR-0110): match exaustivo MANTIDO (sem wildcard); despacho ESTÁTICO (sem
  dyn/vtable); imports FICAM (content→elements inalterado — registrar, NÃO é gate).

INTACTOS: α/caso 2, morph ==/morph_canon, caso 4, flag P350c, F-5b (fechado), os 3 numbering, o #set
  de props de usuário.
lente (instrumento, NÃO gate): content→elements registrado como fato (inalterado — a atomização não
  o move); a métrica real é o tamanho do monólito.
perf (critério 4): a delegação por método é inlinável → sem regressão esperada; medir antes/depois.
L0 (critério 5): ADR-0110 + claude.md gravados (Estágio 0); o entity de atomização + hash
  sincronizado ANTES de mover código, Trava aprovada.
commit: árvore limpa e commitada ao fim.
```

---

## Válvula declarada

Se a medição achar que os 59 arms de layout (+ 43 de introspect) não cabem num lote, **fatiar por
elemento ou grupo** (ex.: os elementos de maior lógica primeiro; ou por família — heading/par, math,
figure/grid). Registrar a fatia e o número. O `match` magro pode coexistir com arms ainda-gordos
durante a transição (cada arm migra independente — a exaustividade fica intacta o tempo todo).

---

## O que NÃO fazer

- **Não usar `dyn`/vtable/PropMap** nem remover a exaustividade (ADR-0110 — seria desacoplamento).
- **Não tentar reduzir `content→elements`** — não é a meta; os imports ficam.
- **Não mudar comportamento ao mover** — content-preserving; se a rede virar, investigar.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.
- **Não pular a Trava** (ADR + medição + L0 + hash do dono antes de mover código).
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no commit; a decisão é do dono.
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-376-relatorio.md` + resumo no chat)

A ADR-0110 + o `claude.md` gravados (Estágio 0); a **medição dos monólitos** (`layout/mod.rs`
1857 linhas / 59 arms, `introspect.rs` 43 arms — quanto por elemento, `file:line`); a **forma da
delegação** desenhada; o L0 com hash e a Trava aprovada; a lógica movida por elemento (`file:line`
do método novo no arquivo do elemento + o arm magro no núcleo); a **paridade** (rede de
caracterização sem alteração); a **métrica de leitura** (o monólito encolheu X linhas; cada elemento
legível sozinho); a confirmação das **não-metas** (match exaustivo, despacho estático, imports
inalterados); a prova de que o α/caso 2/caso 4/flag/F-5b ficaram intactos; a lente (registrada como
fato, não gate); a perf; **o commit de fecho** (hash); `git status` limpo; lint 0/0; o caveat de
stack. **Termina aqui — não emenda o seguinte.**

## Fora de escopo (confirmado — decisão do dono)

A **varredura do projeto inteiro** (todo monólito, não só os de elemento) — **depois** de finalizar
os elementos; a **decisão de crates** (criar crates novos nas camadas) — **depois** (este lote move
para arquivos dentro dos crates atuais); o **Marco G / desacoplamento** (zerar `content→elements` via
`dyn`) — **descartado** (massaroca de plugin-system, ADR-0110 não-meta 3); DEBT-59; DEBT-60.
