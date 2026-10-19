# Passo 373 — transporte do `#set`: análise + correção do colapso same-key (+ o que couber junto)

> **O que faz.** Analisa e corrige o **transporte do P368**, que a fatia (2) do F-5b revelou
> bloqueado: hoje ele é **single-wrap-final-collapse** — múltiplos `#set` da mesma chave num corpo
> embrulham a cauda inteira num único `Content::Styled` com o valor **final** colapsado, então
> `#set text(font: A)\nOlá\n#set text(font: B)\nAdeus` dá font **B** nos dois (medido em
> `font_wiring…`, só 1 de 2 fonts embebidas). **É um bug de paridade observável**, vivo **agora**,
> independente do F-5b — afeta o `#set text` que os usuários escrevem. A correção: o `eval_markup`
> passa a **aninhar** os wraps (cada `#set` embrulha a sua própria cauda, fiel ao escopo léxico —
> espelho do `styled_with_map` por-`#set` do vanilla) ou carregar por-segmento, conforme a medição.
> **A Fase A mede TUDO que cabe junto na mesma correção** (Trava 1) — os **três consumidores** do
> `custom` (numbering, `#set` de props de usuário P368, render `#set text`) e a premissa "inerte em
> layout" que o P372 mediu **falsa** (`tracking`/`leading` são consumidos; `leading` vem por `#set
> par`) — **antes de fixar o escopo**. Inclui a **fatia (2) do F-5b no mesmo lote SE** a Fase A
> medir que ela assenta limpa sobre o transporte corrigido (o desenho dela já está medido e passou
> no `typst-core`, P372); **senão fatia**. **Gate do α duro** (aninhar os wraps não pode reabrir o
> caso 2). **Content-preserving para o existente**; corrige o bug observável rumo ao vanilla
> (declarado, S5b). **Não toca** o caso 4, a flag P350c, o Marco G, nem a distinção de tipo da fatia
> (1). **Design-first**: L0 + Trava antes do código. **Commita ao terminar** e **não emenda o
> seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P373 (confirmar livre).
**Pré-condição**: P372 revertido (a fatia 2 medida como bloqueada pelo transporte; nenhum `.rs`
funcional; o L0 `§3a.13 ⛔` registra o blocker). P371 de pé (fatia 1: strong/emph variantes; α
intacto). Suíte **2747**, lint **0/0**, lente **68/0**, árvore limpa. HEAD pós-P371 (o revert do
P372 restaurou). Lente `tekt-cargo-dsm` disponível (registrar versão/commit). Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: correção de paridade do transporte (**observável** — o same-key colapsa rumo ao vanilla
que escopa) + **possivelmente** a fatia (2) do F-5b junto (se couber). A **rede de caracterização
(+11)** e o **pipeline completo** (`03_infra`, `font_wiring`, os golden PDF) são o oráculo — o P372
mediu que o `typst-core` sozinho **não** pega o bug; este lote roda o pipeline completo.

---

## Fase A — a análise + medir tudo que cabe junto (a fonte vence; `file:line`)

**Antes de fixar o escopo:**

1. **Reler** ADR-0107 (paridade com a linguagem — o escopo léxico do `#set` é semântica), as
   **Travas anti-deriva**, o relatório do P372 (as 3 medições: a premissa inerte falsa; o blocker
   single-wrap; os golden PDF), o L0 `§3a.13 ⛔`, e o desenho da fatia (2) que passou no `typst-core`.
2. **O transporte hoje** (`eval/mod.rs` `eval_markup`), com `file:line`: como o snapshot do canal
   `custom` é tirado e como o `wrap_start`/`Content::Styled` embrulha a cauda. Confirmar o
   **single-wrap-final-collapse** e localizar exatamente onde o colapso descarta o escopo léxico.
3. **O oráculo vanilla**: como o vanilla trata múltiplos `#set` da mesma chave num corpo
   (`styled_with_map` por-`#set`, escopo léxico — `lab/typst-eval/markup.rs`). O alvo da correção
   sai daqui.
4. **Os três consumidores do `custom`** — medir o impacto da correção em cada um, `file:line`:
   - **numbering** (P364/P365): o aninhamento muda o que o numbering vê? (era por-elemento; provável
     no-op, confirmar);
   - **`#set` de props de usuário** (P368): o aninhamento conserta o same-key aqui também? (o mesmo
     bug late aqui — `#set badge(x:A)...#set badge(x:B)`);
   - **render `#set text`** (a fatia 2): sobre o transporte aninhado, o desenho da fatia 2 (já medido,
     passou no `typst-core`) assenta limpo?
5. **A premissa "inerte em layout" (refutada no P372)**: confirmar quais campos do `#set text`/`#set
   par` são **consumidos** no layout (`tracking`/`leading` são; `leading` vem por `#set par`) — para
   o de-bake da fatia 2 (se entrar) decodificar **todos** do `custom`, não só um subconjunto.
6. **O que cabe junto** (Trava 1 — medir o escopo, não assumir): a correção do transporte é o núcleo;
   a fatia (2) entra **no mesmo lote se** assentar limpa sobre o transporte corrigido e o pipeline
   completo ficar verde; **senão** a fatia (2) é o P374. Os golden PDF do P372 (#3) — medir se a
   regressão era **só** efeito do bug do font (#2) e some com a correção, ou se há outra causa.
7. **O α** (gate duro): o aninhamento dos wraps muda o que o `morph_canon`/α-fixpoint vê
   (`rules.rs:229`)? Provável não (o `custom` é transparente; aninhar muda a topologia dos wraps, não
   a morfologia) — **confirmar** e declarar o que será rodado (caso 2 + rede +11).

**A Fase A decide o escopo:** ou **(A) transporte + fatia (2) num lote** (se couber, pipeline verde),
ou **(B) transporte só** (P373) e a fatia (2) no P374. **A medição decide, não a preferência.**

---

## Limites duros

- **Rodar o pipeline COMPLETO**, não só o `typst-core` — o P372 mostrou que o `typst-core` verde
  esconde o bug. O `03_infra` (`font_wiring`, os golden PDF) e a rede +11 são parte da aceitação.
- **O gate do α**: o caso 2 + a rede +11 verdes. Se o aninhamento (ou a fatia 2, se entrar) reabrir o
  caso 2, **parar e reverter**.
- **Content-preserving para o existente** (numbering, user-props): o aninhamento não pode mudar o
  output deles. A correção é **só** o same-key que colapsava (o bug observável); declarado.
- **Não tocar a fatia (1)** (strong/emph variantes), o caso 4, a flag P350c, o Marco G.
- **Não fixar o escopo (A ou B) de memória** — a Fase A mede se a fatia (2) cabe.

---

## Estágios

### Estágio L0 — análise + desenho + escopo (A ou B) + Trava (PARA aqui para o dono)
Registrar a análise do transporte (`file:line`), o alvo vanilla (aninhamento por escopo), o impacto
nos 3 consumidores, a premissa inerte corrigida, e o **escopo medido (A ou B)**. Registrar no L0
`f_fronteira_e1.md` (atualizar o `§3a.13 ⛔` para a correção). Sincronizar hashes. **TRAVA**: para no
chat — a análise + o desenho + o escopo (A/B) + os hashes para o dono aprovar. Nenhum código antes.

### Estágio 1 — a correção do transporte (após aprovação)
O `eval_markup` passa a **aninhar** os wraps (cada `#set` embrulha a sua cauda, escopo léxico) ou
carregar por-segmento, per o desenho. Os 3 consumidores leem o `custom` correto por escopo.

### Estágio 2 — (só se escopo A) a fatia (2) do F-5b sobre o transporte corrigido
O `#set text`/`#set par` → `custom` (todos os campos, incl. os consumidos); o layout lê do `custom`;
o `TextStyle` assado do `Content::Text` removido. (O desenho já passou no `typst-core`, P372.)

### Estágio 3 — os gates (o número)
Rodar o **pipeline completo**: `typst-core` (2747) + `03_infra` (`font_wiring`, os golden PDF) + o
caso 2 (α) + a rede +11. **Todos verdes.** O `font_wiring` agora embebe **2** fonts (o bug corrigido);
os golden PDF — se regridem só pelo bug do font, agora batem; se há outra causa, **parar e reportar**.

### Estágio Teste
- **Novo**: `#set text(font: A)\nX\n#set text(font: B)\nY` → X font A, Y font B (escopo léxico, 2
  fonts embebidas) — a correção do bug, com o vanilla como oráculo.
- **Não-regressão**: numbering, `#set` user-props, a rede +11, o caso 2 — sem alteração.
- **Se escopo A**: `#set text X ≠ X` (a fatia 2); o `TextStyle` assado removido (fonte única).
- As asserções que viram (o bug do font; e, se A, o `#set text X == X`), **declaradas** (S5b).

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes (pipeline completo + α), **commitar**: `git add -A && git commit -m "Passo 373 —
correção do transporte same-key (escopo léxico)[ + F-5b fatia 2]"`. Árvore limpa e commitada. (Para
na Trava → commita só o L0; reverte por α/pipeline → commita só o relatório.)

---

## Verificação (gates)

```
build: limpo por estágio.
PIPELINE COMPLETO (não só typst-core): typst-core 2747(+novos) + 03_infra (font_wiring ×2 VERDE,
  golden PDF) + caso 2 (α) + rede +11. RUST_MIN_STACK=33554432.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável; oráculo = vanilla 0.14.2 + pipeline):
  - same-key: #set text(font:A) X #set text(font:B) Y → X=A, Y=B (escopo léxico; 2 fonts embebidas).
  - não-regressão: numbering, #set user-props, rede +11 inalterados.
  - se escopo A: #set text X ≠ X (fatia 2); TextStyle assado removido (fonte única do render).

GATE DO α (duro): caso 2 + rede +11 verdes. Se reabrir, REVERTER.

INTACTOS (confirmar): fatia (1) (strong/emph); caso 4, flag P350c, Marco G; os 3 numbering; o #set
  de props de usuário (corrigido para same-key, mas o comportamento single-set inalterado).

lente (instrumento): content→elements = 68 (registrar; se a fatia 2 entrar e remover o caminho
  assado, registrar o delta). Não é gate.
perf (critério 4): antes = baseline da mesma sessão; depois reportado (o aninhamento pode mudar a
  contagem de wraps — medir).
L0 (critério 5): f_fronteira_e1.md (§3a.13 → correção) + hash sincronizado ANTES do código, Trava
  aprovada.
commit: árvore limpa e commitada ao fim.
```

---

## Válvula declarada

Escopo **B** (a Fase A mede que a fatia 2 não cabe limpa sobre o transporte corrigido): este lote é
**só** a correção do transporte (P373); a fatia (2) é o **P374**. Registrar a medição que levou a B.
Se a própria correção do transporte não couber (ex.: o aninhamento toca o numbering de forma larga),
fatiar a correção.

---

## O que NÃO fazer

- **Não validar só no `typst-core`** — rodar o pipeline completo (o erro do P372).
- **Não deixar o aninhamento (ou a fatia 2) reabrir o caso 2** (gate duro do α — reverter).
- **Não fixar o escopo (A/B) de memória** — medir (Trava 1).
- **Não mudar o output do numbering / user-props single-set** (content-preserving para o existente).
- **Não tocar a fatia (1), o caso 4, a flag P350c, o Marco G.**
- **Não pular a Trava** (análise + L0 + hash do dono antes do código).
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no commit; a decisão é do dono.
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-373-relatorio.md` + resumo no chat)

A **análise do transporte** (o single-wrap-final-collapse, `file:line`) e o **alvo vanilla**
(aninhamento por escopo); o **impacto medido nos 3 consumidores**; a premissa inerte corrigida; o
**escopo decidido (A ou B)** com a medição que o justifica; o L0 com hash e a Trava aprovada; a
correção do transporte (e, se A, a fatia 2) com `file:line`; **os gates do pipeline COMPLETO** (o
`font_wiring` com 2 fonts, os golden PDF, o caso 2/α — os números, não "passou"); a **lista das
asserções que viraram** (o bug do font; e se A, o `#set text X == X`), justificadas; a confirmação
de que a fatia (1), o caso 4, a flag e o Marco G ficaram intactos; a lente (delta); a perf; **o
commit de fecho** (hash); `git status` limpo; lint 0/0; o caveat de stack. **Se escopo A: a nota de
que o F-5b FECHA, o item (1) vai a 4/4, o F fica completo sem divergência (DEBT-61 fecha). Se B: a
nota de que falta a fatia (2) — P374.** Termina aqui — não emenda o seguinte.

## Estado do F após o P373 (a registrar, conforme o escopo)

- **Escopo A**: o F está completo pelos princípios sem divergência registrada (extensibilidade
  P368/P369; atomização dos 4 duplos; distinção strong/emph/text P371/P372-via-373; o transporte
  correto). DEBT-61 fecha.
- **Escopo B**: o transporte está correto (o bug de paridade same-key fechado, beneficiando
  numbering/user-props/render); falta a fatia (2) do F-5b (P374) para o item (1) ir a 4/4.

Os débitos **não-F** permanecem: DEBT-59 (flag CLI), DEBT-60 (contador), o **Marco G** (decisão de
modelo α/β, fora do F).

## Fora de escopo (confirmado)

A fatia (1) (feita, P371); o **Marco G** (não-F); DEBT-59 (flag CLI); DEBT-60 (contador); qualquer
toque no caso 4, no Marco G ou na flag. (A fatia 2 está **dentro** se a Fase A medir escopo A.)
