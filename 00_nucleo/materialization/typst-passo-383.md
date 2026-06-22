# Passo 383 — atomização do `introspect.rs` (a outra camada de "elementos primeiro")

> **O que faz.** Atomiza o `introspect.rs` — os **43 arms** do walk — a segunda camada onde os
> elementos de domínio têm lógica amontoada num monólito (a primeira, o layout, fechou no P382).
> Quando esta fechar, os elementos estarão atomizados **nas duas camadas** = o escopo "elementos
> primeiro" completo. **A Fase A ABRE medindo a convenção do `introspect.rs`** — como o walk se
> organiza, se há precedente de um arm que já delega — **antes de fixar a forma**. A lição do P382
> (math): a forma certa **sai da camada de destino**, não se herda das fatias anteriores; o
> introspect pode ter convenção própria (não a forma B do layout flat, nem o `impl Layouter` do
> math). **Mede de passagem se os displays counter/state** (`CounterDisplay`/`StateDisplay` —
> [a-decidir] desde o P379) **se resolvem nesta frente**, por serem **adaptadores de introspeção** —
> o introspect é a camada deles. **Separa elementos de domínio (atomizáveis) da máquina do walk**
> (recursão/fixpoint/orquestração — fica, como a máquina do layouter ficou). **Content-preserving**:
> a lógica move, não muda. `match` exaustivo, despacho estático, `entities/` intacto. **L0 commitado
> ANTES de mover** (mecanismo externo). **Trava de L0**, **commit ao fim**, **não emenda o seguinte**
> (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P383 (confirmar livre).
**Pré-condição**: P382 fechado (atomização do layout completa — `layout_content` 545, só máquina +
pontes + no-ops; 44 unidades). Suíte verde, lint **0/0**, `content→elements` 68 (não-gate). HEAD
pós-P382. Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: atomização (content-preserving). A **rede de caracterização (+11, P331)** + os testes de
introspeção/fixpoint são o oráculo. A métrica é de **leitura**, não a lente. **Camada nova (walk) —
a forma sai da medição, não herdada do layout/math.**

---

## Fase A — medir a convenção do walk PRIMEIRO + classificar os arms (a fonte vence; `file:line`)

**Antes de fixar a forma:**

1. **Reler** a ADR-0109 (atomização: lógica no arquivo dono, sem despacho dinâmico, exaustividade
   mantida), as **Travas anti-deriva**, e os P379/P382 (o introspect-chain do P363; a lição "a forma
   sai da camada").
2. **Medir a convenção do `introspect.rs`**, `file:line`: como o walk se organiza (`walk` +
   `extract_payload` + `populate_intr` + `materialize_time` + os pontos `:156/:176/:422/:828`); se
   algum arm **já delega** (a forma a seguir); se há um método/free-function central; como o
   introspect-chain (P363) threada a `StyleChain` (a forma do parâmetro).
3. **Classificar os 43 arms** (como o P379 fez no layout), `file:line`:
   - **[elemento de domínio]** — a lógica de introspeção de um elemento (extrair payload, contar,
     emitir tag) que é separável para o arquivo do elemento / um arquivo por-elemento na convenção do
     walk;
   - **[máquina do walk]** — recursão, fixpoint, orquestração (desce nos filhos, gere o
     introspector) — **fica**, não é elemento;
   - **[adaptador de introspeção]** — os displays counter/state ([a-decidir]): medir se atomizam
     naturalmente aqui (são plumbing da própria camada) ou se ficam como máquina;
   - **[no-op]** — arms vazios.
   Marcar [medido]/[inferido].
4. **Decidir a forma a partir do walk** (não impor a do layout/math): o arm magro delega na convenção
   medida. Se a lógica de introspeção de um elemento já vive noutro lugar e só o despacho está no
   `match`, a fatia é mover o despacho.
5. **Confirmar as não-metas** (ADR-0109): `match` exaustivo (sem wildcard); despacho estático (sem
   `dyn`); `entities/` não tocado. Se mover exigir `dyn`/wildcard ou tocar `entities/`, **parar e
   reportar**.
6. **O escopo/fatias**: quantos arms são [elemento de domínio] (a atomizar) vs [máquina]/[no-op]
   (ficam); se cabe num lote ou fatia (válvula) — e se os displays counter/state entram (resolvendo
   o [a-decidir]) ou ficam como máquina.

---

## Limites duros (ADR-0109)

- **`match` exaustivo MANTIDO** (sem wildcard); **despacho ESTÁTICO** (sem `dyn`); **`entities/` não
  tocado** (`content→elements` inalterado, não-gate).
- **A forma sai da medição do walk** — **não** impor a forma B do layout nem o `impl Layouter` do
  math se o walk tiver convenção própria.
- **Máquina do walk fica** (recursão/fixpoint/orquestração) — não é elemento; não forçar.
- **Não tocar o introspect-chain (P363)** de forma que mude o que ele entrega — a atomização move a
  lógica, não altera o threading da chain.
- **Content-preserving** — a rede + os testes de introspeção/fixpoint passam **sem alteração**; se
  virar, a lógica mudou ao mover → **investigar, não mascarar**.
- **Não tocar** o α/caso 2 (o fixpoint morfológico — cuidado: o introspect tem o seu fixpoint;
  confirmar que não é o α), o `morph_canon`/`==`, o caso 4, a flag, o F-5b.

---

## Estágios

### Estágio L0 — desenho (com a convenção medida) + Trava + **commit do L0** (PARA aqui para o dono)
Registrar no L0 (`rules/atomizacao_elementos.md` ou um entity do introspect) a convenção do walk
medida, a classificação dos 43 arms (domínio/máquina/adaptador/no-op), a forma escolhida, o destino
dos displays counter/state, e o escopo/fatias. Sincronizar hashes. **Commitar o L0 agora** (mecanismo
externo). **TRAVA**: para no chat — a convenção + a classificação + a forma + os hashes para o dono
aprovar. Nenhum arm movido antes.

### Estágio 1 — mover a fatia (após aprovação)
Os arms [elemento de domínio] ficam magros, delegando na convenção medida; a lógica de introspeção
muda para o arquivo do elemento / por-elemento. A máquina e os no-ops ficam. Content-preserving.

### Estágio Teste
- A **rede de caracterização (+11)** + os **testes de introspeção/fixpoint** passam **sem alteração**.
- **Leitura**: o `introspect.rs` (o walk) encolhe X linhas; quantos arms de domínio atomizados;
  quantos ficam (máquina/no-op).
- Confirmar: `match` exaustivo, despacho estático, `entities/` intacto; o introspect-chain não
  alterado; o α/caso 2/caso 4/flag/F-5b intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes, **commitar**: `git add -A && git commit -m "Passo 383 — atomização do
introspect.rs: lógica de introspeção dos elementos para os arquivos dos elementos"`. Árvore limpa e
commitada.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): inalterada em número (content-preserving) — rede +11 + testes de
  introspeção/fixpoint sem asserção virada. Se virar, investigar.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (leitura — ADR-0109):
  - os arms [elemento de domínio] do walk legíveis nos seus arquivos, na convenção medida.
  - o introspect.rs encolhe; o walk fica com máquina (recursão/fixpoint) + no-ops + adaptadores (se
    ficarem).
  - content-preserving: comportamento idêntico (rede + testes de introspeção).

NÃO-METAS: match exaustivo (0 wildcards); despacho estático (0 dyn); entities/ intacto
  (content→elements = 68 não-gate); forma = a do walk (medida, não imposta).

FORA (não tocada): máquina do walk (recursão/fixpoint/orquestração); o introspect-chain (P363, não
  alterado no que entrega).
INTACTOS: α/caso 2, morph ==/morph_canon, caso 4, flag P350c, F-5b; os 3 numbering; o #set de props.
lente (instrumento): content→elements = 68 inalterado.
perf: medir.
L0: + hash COMMITADO ANTES de mover, Trava aprovada.
commit: L0 (Estágio L0) + fatia (fecho); árvore limpa.
```

---

## Válvula declarada

Se os arms [elemento de domínio] do walk não couberem num lote, fatiar (por família, como no layout).
Se a medição mostrar que a maioria dos arms é [máquina] (o walk é mais orquestração que lógica
por-elemento), a atomização do introspect é **pequena** — registrar isso (não inflar para parecer
grande). Os displays counter/state: se forem máquina, ficam; se forem adaptadores atomizáveis,
entram — a medição decide.

---

## O que NÃO fazer

- **Não impor a forma B (layout) nem o `impl Layouter` (math)** — medir a convenção do walk primeiro.
- **Não usar `dyn`/wildcard** nem remover a exaustividade (ADR-0109).
- **Não tocar `entities/`** nem reduzir `content→elements`.
- **Não tocar a máquina do walk** (recursão/fixpoint) nem alterar o que o introspect-chain entrega.
- **Não mudar comportamento ao mover** — content-preserving; se a rede/introspeção virar, investigar.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.
- **Não deixar L0/doc não-commitado** entre estágios (mecanismo externo).
- **Não pular a Trava**; **não emendar o passo seguinte** (Trava 5).
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-383-relatorio.md` + resumo no chat)

A **convenção do walk medida** (`file:line`: como se organiza, se há precedente de delegação, como o
introspect-chain threada); a **classificação dos 43 arms** (domínio/máquina/adaptador/no-op, com
`file:line` e [medido]/[inferido]); a **forma escolhida a partir da convenção**; o destino dos
**displays counter/state** (atomizados aqui ou ficam — resolvendo o [a-decidir]); o L0 commitado
(hash) e a Trava aprovada; a fatia movida (`file:line` + arms magros); a paridade (rede +11 + testes
de introspeção sem alteração); a **métrica de leitura** (o `introspect.rs` −X; quantos arms de
domínio atomizados); as não-metas; a confirmação de que a máquina/introspect-chain ficaram intactos
e o α/caso 2/caso 4/flag/F-5b idem; a lente (= 68, não-gate); a perf; **os commits** (L0 + fatia);
`git status` limpo; lint 0/0; o caveat de stack. **A nota de que, com o introspect atomizado, os
elementos estão atomizados nas duas camadas — "elementos primeiro" completo; a frente seguinte é a
varredura do projeto (decisão do dono).** Termina aqui — não emenda o seguinte.

## Fora de escopo (confirmado)

A **máquina do walk** (recursão/fixpoint/orquestração — fica); a **máquina do layouter**
(Sequence/Styled/Dynamic/SetPage — fica); a **varredura do projeto inteiro** (após "elementos
primeiro" completo — decisão do dono); os **crates** (depois); Marco G/desacoplamento (descartado);
DEBT-59; DEBT-60.
