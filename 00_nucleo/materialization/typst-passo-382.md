# Passo 382 — atomização do layout, fatia math (final): Equation + variantes → `rules/math/layout/`

> **O que faz.** Fatia **final** do `layout_content` (564 linhas, pós-P381): os arms **math** —
> `Equation` + o arm agrupado de **16 variantes** — que, ao contrário de todos os anteriores,
> **descem a `rules/math/layout/`** (path próprio do subsistema de math), **não** a `rules/layout/`
> flat. **Por isso a forma NÃO é assumida**: a Fase A **abre medindo a convenção interna do
> subsistema `rules/math/layout/`** (como ele já se organiza) antes de decidir o formato — a forma B
> das fatias anteriores (free function em `rules/layout/<elem>.rs`) pode **não** ser a convenção
> certa lá dentro (Trava 1: terreno novo, medir antes de fixar). Move as variantes math na forma que
> a medição apontar, mantendo as não-metas (ADR-0109): `match` exaustivo, despacho estático,
> `entities/` intacto. **Após este passo, o `layout_content` fica só máquina** (Sequence/Styled/
> Dynamic/SetPage) **+ no-ops/displays** — a atomização do **layout** fecha (os elementos de domínio,
> incl. math, atomizados). **Content-preserving**: a lógica move, não muda. **L0 commitado ANTES de
> mover** (mecanismo externo). **Trava de L0**, **commit ao fim**, **não emenda o seguinte**
> (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P382 (confirmar livre).
**Pré-condição**: P381 fechado (elementos de domínio não-math atomizados; `layout_content` 564;
−1293 acumulado; 43 unidades). Suíte verde, lint **0/0**, `content→elements` 68 (não-gate). HEAD
pós-P381. Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: atomização (content-preserving). A **rede de caracterização (+11, P331)** + os testes de
math são o oráculo. A métrica é de **leitura**, não a lente.

---

## Fase A — medir a convenção do subsistema math PRIMEIRO (a fonte vence; `file:line`)

A diferença desta fatia: o destino é `rules/math/layout/`, um subsistema com a **sua própria
organização** — não o `rules/layout/` flat das fatias anteriores. **Não assumir a forma B.**

1. **Reler** a ADR-0109 (as **não-metas** valem aqui também — exaustivo, estático, `entities/`), as
   **Travas anti-deriva**, e o P381 (o arm math agrupado, `@858`/`@868`).
2. **Medir o subsistema `rules/math/layout/`**, com `file:line`: como ele já está organizado — há um
   `layout_equation`/`layout_node` que recebe a `StyleChain` por parâmetro (visto no P360:
   `equation.rs:37` `layout_equation(body, &self.style)`)? Como os nós math já são despachados lá
   dentro? Qual a **convenção** de onde a lógica de cada variante vive? **A forma de atomizar sai
   daqui** — pode ser delegar ao subsistema que já existe, não criar arquivos flat novos.
3. **Medir os arms math no `layout_content`** (`@858`/`@868`, Equation + 16 variantes): o que cada um
   faz — já é só uma ponte para `rules/math/layout/`, ou tem lógica própria no `layout_content` a
   mover? Se já é ponte fina, "atomizar" pode ser só confirmar a delegação; se tem lógica, mover para
   o subsistema.
4. **Decidir a forma** (medida, não assumida): delegar ao `rules/math/layout/` existente / criar
   arquivos no subsistema seguindo a convenção dele / outra forma que a fonte mostrar. Confirmar que
   mantém `match` exaustivo, despacho estático, `entities/` intacto.
5. Se a medição mostrar que os arms math **já estão atomizados** (são pontes finas para o
   subsistema), o passo **confirma e fecha** — registra que a fatia math não tinha lógica a mover, e
   o `layout_content` já está no mínimo. **Não inventar movimento que não há** (a Trava 6: não
   concluir "tem trabalho" a priori).

---

## Limites duros (ADR-0109)

- **`match` exaustivo MANTIDO**; **despacho ESTÁTICO** (sem `dyn`); **`entities/` não tocado**.
- **A forma sai da medição do subsistema math** — não impor a forma B se a convenção do
  `rules/math/layout/` for outra.
- **Não tocar a máquina (Sequence/Styled/Dynamic/SetPage)** — fica.
- **Content-preserving** — a rede de caracterização + os testes de math passam **sem alteração**; se
  virar, a lógica mudou ao mover → **investigar, não mascarar**.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.

---

## Estágios

### Estágio L0 — desenho (com a convenção medida) + Trava + **commit do L0** (PARA aqui)
Registrar no L0 (`rules/atomizacao_elementos.md`) a convenção do subsistema math medida, a forma
escolhida, e o que resta após o passo (a máquina + os displays [a-decidir]; e que a próxima frente é
o `introspect.rs`). Sincronizar hashes. **Commitar o L0 agora** (mecanismo externo). **TRAVA**: para
no chat — a medição do subsistema + a forma + os hashes para o dono aprovar. Nenhum arm movido antes.

### Estágio 1 — mover/confirmar a fatia math (após aprovação)
Per o desenho: mover a lógica math do `layout_content` para o subsistema `rules/math/layout/` (na
convenção dele), ou confirmar a delegação se já for ponte fina. Arms magros no núcleo.
Content-preserving.

### Estágio Teste
- A **rede de caracterização (+11)** e os **testes de math** passam **sem alteração**.
- **Leitura**: o `layout_content` encolhe ao mínimo (só máquina + no-ops/displays); registrar o
  acumulado desde 1857 e que a atomização do **layout fecha** aqui.
- Confirmar: `match` exaustivo, despacho estático, `entities/` intacto; a máquina não tocada; o
  α/caso 2/caso 4/flag/F-5b intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes, **commitar**: `git add -A && git commit -m "Passo 382 — atomização fatia math
(layout fechado: só máquina resta)"`. Árvore limpa e commitada.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): inalterada em número (content-preserving) — rede +11 + testes de
  math sem asserção virada.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (leitura — ADR-0109):
  - a lógica math vive no subsistema rules/math/layout/ (na convenção dele); layout_content no mínimo.
  - após o passo, layout_content = SÓ máquina (Sequence/Styled/Dynamic/SetPage) + no-ops/displays.
  - a atomização do LAYOUT fecha (elementos de domínio + math atomizados).
  - content-preserving: comportamento idêntico (rede + testes de math).

NÃO-METAS: match exaustivo (0 wildcards); despacho estático (0 dyn); entities/ intacto
  (content→elements = 68 não-gate); forma = a convenção do subsistema math (medida).

FORA (não tocada): a máquina do layouter.
INTACTOS: α/caso 2, morph ==/morph_canon, caso 4, flag P350c, F-5b; os 3 numbering; o #set de props.
lente (instrumento): content→elements = 68 inalterado.
perf: medir.
L0: atomizacao_elementos.md + hash COMMITADO ANTES de mover, Trava aprovada.
commit: L0 (Estágio L0) + fatia (fecho); árvore limpa.
```

---

## Válvula declarada

Se a medição mostrar que os arms math **já são pontes finas** para o subsistema (já atomizados), o
passo **fecha confirmando** — sem movimento inventado (Trava 6). Se houver lógica a mover e ela não
couber num lote, fatiar (Equation num, as 16 variantes noutro). Se mover exigir tocar a máquina ou o
`entities/`, **parar e reportar**.

---

## O que NÃO fazer

- **Não assumir a forma B** — a forma sai da convenção do subsistema `rules/math/layout/` medida.
- **Não inventar movimento** se os arms math já forem pontes finas (Trava 6 — auditar, não assumir).
- **Não usar `dyn`/wildcard** nem tocar `entities/` (ADR-0109).
- **Não tocar a máquina** (Sequence/Styled/Dynamic/SetPage), o α/caso 2, o `morph_canon`/`==`, o
  caso 4, a flag, o F-5b.
- **Não mudar comportamento ao mover** — content-preserving; se a rede/math virar, investigar.
- **Não deixar L0/doc não-commitado** entre estágios (mecanismo externo).
- **Não pular a Trava**; **não emendar o passo seguinte** (Trava 5).
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-382-relatorio.md` + resumo no chat)

A **convenção do subsistema `rules/math/layout/` medida** (`file:line`) e a forma escolhida; a
medição dos arms math (lógica própria vs ponte fina); o L0 commitado (hash) e a Trava aprovada; a
fatia math movida/confirmada (`file:line`); a paridade (rede +11 + testes de math sem alteração); a
**métrica de leitura** (o `layout_content` no mínimo; o acumulado desde 1857; **a atomização do
layout fechada**); as não-metas; a confirmação de que a máquina ficou e o α/caso 2/caso 4/flag/F-5b
intactos; a lente (= 68, não-gate); a perf; **os commits**; `git status` limpo; lint 0/0; o caveat
de stack. **A nota de que o LAYOUT está atomizado (só máquina resta); a próxima frente é o
`introspect.rs` (43 arms), e os displays counter/state são [a-decidir].** Termina aqui — não emenda
o seguinte.

## Fora de escopo (confirmado)

A **máquina do layouter** (fica); a atomização do **`introspect.rs`** (próxima frente, após o
layout); os **displays counter/state** ([a-decidir], fronteira); a **varredura do projeto** e os
**crates** (depois); Marco G/desacoplamento (descartado); DEBT-59; DEBT-60.
