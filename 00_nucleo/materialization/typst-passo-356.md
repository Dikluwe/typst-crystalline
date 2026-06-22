# Passo 356 — caso 1, lacuna (i): show-set + func (o exemplo canônico do doc)

> **O que faz.** Conserta a **lacuna (i)** medida no recon do P355
> (`f-recon-composicao-passo-355.md`): `#show heading: set text(...)` **+** `#show heading: it
> => …` no mesmo elemento. **Hoje o crystalline perde o show-set** — o func roda primeiro, seu
> output não casa o seletor de heading, e o show-set nunca entra. O **vanilla** dobra o show-set
> na chain (`map.apply`, `lib.rs:458-464`) e aplica o func **sob** a chain aumentada (`chained =
> styles.chain(&map)`, `lib.rs:357`); o resultado é o **exemplo canônico do doc**
> (`styling.md`, *Show rules*: "composable", "good practice", 4× `#show heading`). O conserto é
> **contido** em `apply_show_rules`/`apply_all` e **NÃO reabre o α** (o show-set é
> `Transformation::Style`, fora do guard/morph; é ordem show-set-vs-func, não recursão). **Honra
> a composição como LÍNGUA** — não declara divergência (a lição do P355: composição same-kind é
> língua documentada, não mecânica). **NÃO conserta a lacuna (ii)** (múltiplos func same-kind —
> A2/A3, fatia seguinte) e **não** se apresenta como "composição funciona" inteira.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P356 (confirmar livre). *(O diagnóstico do contador `1.1`≠`0.1`, DEBT-60,
desloca para o P357.)*
**Pré-condição**: P355 fechado — a ADR-0109 e as edições de L0 revertidas; o recon de
reconciliação produzido (`f-recon-composicao-passo-355.md`); a composição classificada como
**LÍNGUA pela fonte** (`styling.md`). HEAD no estado committed pós-revert (`1eb216303`, pós-P353).
Suíte **2733/0**, lint **0/0**, árvore limpa. Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo
não bater, parar e reportar.
**Tipo**: caso 1, lacuna (i) — **paridade** (honra a composição documentada). **NÃO
content-preserving**: o combo show-set+func passa de "show-set perdido" para "show-set aplica sob
o func", casando o vanilla/doc. O combo tem **0 testes hoje** (recontagem P354/P355), então
**nenhuma asserção existente muda**; os testes do lote são novos (o exemplo do doc).

---

## Leituras da Fase A (a fonte vence; `file:line`)

1. **A REFERÊNCIA DA LINGUAGEM — leitura obrigatória** (a lição do P355): `lab/typst-original/
   docs/reference/language/styling.md`, secção *Show rules*. O exemplo canônico com 4× `#show
   heading` (show-set + func), "keeping styling composable", "good practice". É a fonte que
   classifica o combo como **língua** e define o comportamento-alvo. **O P355 quebrou por não
   ler isto; aqui é gate, não opção.**
2. **ADR-0107 e ADR-0108** — reler e aplicar. A composição é língua (não mecânica); a aceitação
   é ao nível da língua (o show-set aplica sob o func, observável), com o vanilla **e o doc**
   como oráculo.
3. **O mecanismo do vanilla** (leitura da quarentena, nunca importar), confirmar `file:line`:
   - `typst-realize/src/lib.rs:458-464`: `Transformation::Style(t) => { map.apply(t); continue }`
     — o show-set **dobra na chain** sem consumir o passe.
   - `lib.rs:357`: `chained = styles.chain(&map)` — a chain aumentada.
   - `lib.rs:341`: a 1ª func vira `step` e aplica **sob** `chained` — logo o show-set está
     **ativo quando o func realiza**.
4. **O crystalline hoje**: `rules/eval/rules.rs` — `apply_show_rules`/`apply_all`. Onde o func
   roda **antes** do show-set e o show-set checa o output **pós-func** (a causa medida: o output
   não casa o seletor → show-set perdido).
5. **As travas a confirmar verdes**: os testes de show-set do P352 (múltiplos show-set já
   compõem via `collapse`/`rules.rs:272`) e os de caso 2 (`p348_*`). Confirmar 0 testes no combo
   show-set+func hoje.

**Re-confirmar o desenho do conserto contra os sítios reais ANTES do código** (o desenho vem do
recon, mas a Fase A o verifica): dobrar as show-set casadas sobre o **nó original**, aplicar o
func **sob** a chain aumentada, embrulhar o output do func no `Styles` (espelha `chained`). Se a
medição contradisser o desenho do recon, a fonte vence e reporta (precedente P351/P352/P355).

---

## Limites duros

- **Não reabrir o α / caso 2.** O show-set é `Transformation::Style`, fora do guard/morph; o
  conserto é de **ordem show-set-vs-func**, não de recursão. O `morph_canon`/`==` (P345) e o
  `p348` ficam intactos.
- **Não consertar a lacuna (ii)** (múltiplos func same-kind). É A2/A3, **fatia seguinte**, com a
  decisão do dono. Fica **aberta e nomeada** — não silenciada.
- **Não apresentar como "composição funciona" inteira.** A (ii) continua divergente; o teste e o
  relatório **declaram** isso (a lição do S5b: o conserto parcial não pode mascarar o buraco
  maior).
- **Não regredir os múltiplos show-set** (já compõem, P352).
- **Não tocar o caso 4, a flag P350c, o DEBT-59 nem o Marco G** (`edges content→elements` = 66).

---

## Estágios

### Estágio L0 — a ordem correta (Trava; PARA aqui para o dono)
Atualizar `entities/show.md`: a regra de ordem **show-set dobra na chain antes do func; o func
aplica sob a chain aumentada** (com cross-reference ao `styling.md` e ao `lib.rs:357,458-464`).
Sincronizar o hash. **TRAVA**: para no chat para a aprovação do dono antes de qualquer `.rs`.

### Estágio 1 — o conserto (após aprovação)
Em `apply_show_rules`/`apply_all` (`rules/eval/rules.rs`): quando há show-set **e** func casando
o mesmo elemento, **coletar os `Styles` das show-set casadas** sobre o nó original, **aplicar o
func sob a chain aumentada**, e **embrulhar o output do func num `Content::Styled`** carregando
esses styles (espelha `chained = styles.chain(&map)` do vanilla). O show-set deixa de checar o
output pós-func; passa a estar ativo **quando** o func realiza.

### Estágio Teste
- **Novo (o exemplo canônico do doc)**: `#show heading: set text(...)` ⨁ `#show heading: it =>
  [X:]+it.body` → o `"X:"` recebe o estilo do show-set (renderiza como o vanilla/doc). Antes:
  show-set perdido.
- **Verde (não regredir)**: os múltiplos show-set do P352 continuam compondo.
- **Documenta a (ii) ainda divergente**: um teste marca que **múltiplos func same-kind** ainda
  aplicam uma só (referência ao recon), explicitamente como lacuna aberta — não como paridade.
- Confirmar por construção/teste que o α / caso 2, o caso 4, o `morph_canon`/`==` e a flag P350c
  ficaram intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2733 + os testes novos do combo. ZERO asserção existente
  alterada (0 testes no combo show-set+func hoje).
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável, nível da língua — ADR-0107/0108; oráculo = vanilla + styling.md):
  - o exemplo canônico do doc (show-set + func) renderiza com o estilo do show-set aplicado sob
    o func — paridade com o vanilla.
  - múltiplos show-set continuam compondo (P352 verde).
  - múltiplos func same-kind: DIVERGÊNCIA ABERTA E DECLARADA (não apresentada como paridade).

INTACTOS (confirmar): α / caso 2 (p348 verde), caso 4 (P340), morph ==/morph_canon (P345), flag
  P350c, DEBT-59, Marco G (edges content→elements = 66).

lente (critério 3): edges(content→elements::*) = 66 INALTERADO; edges(elemento→elemento) = 0.
perf (critério 4): antes = o baseline da mesma sessão do P353 (1.1991 s ± 0.0742); reportar o
  depois — o conserto corre só quando há show-set+func casados; caminho sem isso, ~nulo; afirmar
  só o medido.
L0 (critério 5): show.md com a ordem correta + hash sincronizado ANTES do código, Trava cumprida
  (aprovação do dono registrada).
```

---

## O que NÃO fazer

- **Não reabrir o α / caso 2** (o conserto é show-set-vs-func, não recursão).
- **Não consertar a lacuna (ii)** (múltiplos func — A2/A3, fatia seguinte com decisão do dono).
- **Não apresentar o conserto como composição completa** — a (ii) fica declarada como aberta.
- **Não regredir os múltiplos show-set** (P352).
- **Não pular a Trava** (show.md + hash do dono antes de qualquer `.rs`).
- **Não importar a quarentena.** `lab/` é leitura de semântica/doc, nunca import.

---

## Relatório (`typst-passo-356-relatorio.md` + resumo no chat)

A leitura do `styling.md` (o exemplo canônico) e do mecanismo do vanilla com `file:line`; o L0
(`show.md`, ordem correta) com hash e a Trava aprovada; o conserto em `apply_show_rules`/`apply_all`
com `file:line`; os testes novos (o exemplo do doc renderiza certo; a (ii) documentada como
aberta); a prova de que α / caso 2, caso 4, morph `==` e a flag ficaram intactos, e que os
múltiplos show-set não regrediram; os números da lente (edges 66) e da perf (antes/depois);
`git status` limpo por estágio fora de `lab/` e docs; lint 0/0; o caveat de stack.

## Fora de escopo (confirmado)

A lacuna (ii) — **múltiplos func same-kind** (A2/A3, fatia seguinte, com a demanda medida e a
decisão do dono); o diagnóstico do contador `1.1`≠`0.1` / DEBT-60 (P357); o de-bake do F-5
(limpeza sem demanda, adiado no P353); F-6 (3 folhas); Marco G; flag na CLI (DEBT-59).
