# Passo 359 — fechar o DEBT-60: contador de heading (`1.1`≠`0.1`) + supplement "Secção"

> **O que faz.** Fecha o **DEBT-60** — duas anomalias de paridade que a probe do P353 mediu, com
> o **vanilla 0.14.2 como oráculo**: **(a)** o contador de heading diverge — `= A` / confinado `==
> B` / `== C` dá **`1.1`** no crystalline vs **`0.1`** no vanilla (um heading de nível-1 sem
> numbering ativo **avança** o contador no crystalline e **não** no vanilla); **(b)** o `#outline`
> emite um **supplement "Secção"** que o vanilla não emite. **Diagnóstico-primeiro com Trava**: o
> Estágio A **separa** as duas (podem ter **raízes diferentes**), mede cada uma contra o vanilla
> **e a referência da linguagem**, e localiza a causa com `file:line` — **só então**, com a causa
> na mão e a aprovação do dono, aplica a correção. **A forma do conserto NÃO é pré-escrita** —
> nasce da medição (a lição P351–P355: dimensionar de memória erra; medir primeiro acerta). Se
> forem duas raízes, parte em dois consertos/lotes; se uma, fecha num. **Não toca** a F-realização
> fechada, o loop α / caso 2, o `morph_canon`/`==` nem a flag P350c.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P359 (confirmar livre).
**Pré-condição**: P358 fechado (caso 1 e F-realização fechados; suíte **2736**; lint **0/0**;
lente 66). HEAD pós-P358, árvore limpa. Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo não
bater, parar e reportar.
**Tipo**: correção de paridade, **diagnóstico-primeiro**. O Estágio A é medição (read-only,
probes revertidas); a correção é **content-preserving-em-direção-ao-vanilla** (o output passa a
casar o oráculo; as asserções que mudarem são as que hoje afirmam o valor divergente, justificadas
contra o vanilla). **A forma e o tamanho do conserto saem da medição**, não deste enunciado.

---

## Estágio A — diagnóstico (a fonte vence; `file:line`; Trava no fim)

**Read-only**: ler + probes descartáveis (compiladas e revertidas; árvore limpa). Medir e separar:

1. **ADR-0107 e ADR-0108** — reler. A paridade do contador/outline é **semântica** (o número e o
   supplement são conteúdo de linguagem que o leitor vê), com o vanilla **e a referência** como
   oráculo. Não pré-classificar a causa nem o tamanho.
2. **A referência da linguagem** (o gate da lição P355): `lab/typst-original/docs/reference/` —
   o que a referência diz sobre (a) quando um heading **avança** o contador (numbering ativo vs
   inativo; o nível-1 sem numbering conta ou não?) e (b) o **supplement** de heading no `#outline`
   (existe um default? "Section"/"Secção"? de onde vem?). É a fonte que define o alvo.
3. **Sub-anomalia (a) — o contador**: medir no vanilla o caso exato da probe (`= A` / `#[ #set
   heading(numbering:"1.") ; == B ] ` / `== C`) e variações (numbering ativo no topo; sem
   confinamento) → registrar os números do vanilla. No crystalline, localizar **onde o contador de
   heading avança** (`introspector.formatted_counter_at` / o stepping de nível) com `file:line`, e
   **por que** `A` conta aqui e não no vanilla (a regra de avanço considera numbering ativo? o
   confinamento? o nível?).
4. **Sub-anomalia (b) — o supplement "Secção"**: localizar **onde** o `#outline` do crystalline
   monta a entrada e **de onde** vem o "Secção" (supplement default? string embutida? montagem da
   entrada do outline?) com `file:line`, e o que o vanilla faz no mesmo ponto.
5. **A pergunta que decide o tamanho**: as duas anomalias têm a **mesma raiz** (ex.: um único
   ponto de numbering/supplement) ou **raízes diferentes**? **Medir, não assumir.** Marcar medido
   vs inferido e o que refutaria.
6. **Confirmar o que NÃO regride**: a correção do contador/outline **não** pode tocar a
   F-realização (caso 1–4), o α / caso 2, o `morph_canon`/`==`, a flag P350c nem o Marco G — medir
   se os pontos de conserto estão fora desses caminhos.

**TRAVA**: o Estágio A termina no chat. Entregar: os números do vanilla (a) e (b); a causa de cada
um com `file:line`; **se é uma raiz ou duas**; a forma proposta do conserto (um lote, ou dois;
content-preserving-vs-vanilla); os testes que virariam de divergência para paridade; e qualquer L0
a tocar. **Nenhum `.rs` de produto antes da aprovação do dono.**

---

## Estágio B — a correção (após aprovação; a forma vem do Estágio A)

Materializar o que o Estágio A desenhou. As regras, **independentes do conteúdo exato**:

- **Se duas raízes**: dois consertos isoláveis (ou dois lotes, se a Fase A medir que não cabem
  juntos) — o contador e o supplement separados, cada um com seu teste de paridade.
- **Se uma raiz**: um conserto, com os dois sintomas cobertos por testes.
- Cada conserto é **paridade contra o vanilla**: o output passa a casar o oráculo; a referência da
  linguagem confirma o alvo. As asserções que mudam são as que hoje afirmam o valor divergente
  (`1.1`, "Secção"), justificadas uma a uma.
- **L0 primeiro** onde aplicável (a regra de avanço do contador / o supplement), com hash
  sincronizado e a Trava cumprida.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Limites duros

- **Não tocar a F-realização (casos 1–4), o loop α / caso 2, o `morph_canon`/`==` nem a flag
  P350c.** O DEBT-60 é contador/outline; se a medição mostrar que a causa mora num desses
  caminhos, **parar e reportar** (é re-escopo, não conserto silencioso).
- **Não assumir uma raiz nem duas.** O Estágio A mede; o tamanho do lote vem da medição.
- **Não pré-escrever a forma do conserto.** Ela nasce do Estágio A.
- **Não tocar o caso 4, o Marco G, o de-bake do F-5 nem o DEBT-59.**

---

## Verificação (gates)

```
Estágio A: read-only (zero .rs/.toml de produto; probes revertidas; árvore limpa; suíte 2736 não
  re-rodada). Saída = diagnóstico + Trava.
Estágio B (após aprovação):
  build: limpo por estágio.
  suíte (RUST_MIN_STACK=33554432): 2736 ± as asserções que afirmavam o valor divergente (1.1,
    "Secção"), agora paridade — justificadas vs vanilla; reportar quais.
  lint: crystalline-lint . = 0/0.
  ACEITAÇÃO (observável; oráculo = vanilla 0.14.2 + referência):
    - (a) o contador: `= A` / confinado `== B` / `== C` → casa o número do vanilla (`0.1`, ou o
      que o Estágio A medir como correto).
    - (b) o outline: sem o supplement "Secção" que o vanilla não emite (ou o supplement correto,
      conforme a referência).
  INTACTOS: F-realização (1–4), α / caso 2, caso 4, morph ==/morph_canon, flag P350c, Marco G
    (edges content→elements = 66).
  lente: edges(content→elements::*) = 66; edges(elemento→elemento) = 0.
  perf: antes = baseline da mesma sessão; depois reportado (esperado ~nulo; afirmar só o medido).
  L0: o(s) ponto(s) de regra editados e hashes sincronizados ANTES do código, Trava cumprida.
```

---

## O que NÃO fazer

- **Não escrever a correção antes do diagnóstico.** O Estágio A para na Trava; o conserto vem
  depois, com a forma medida.
- **Não medir as duas anomalias como uma** sem confirmar a raiz (a deriva que o arco evita).
- **Não assumir "Secção" é só uma string** nem "o contador é só um off-by-one" — medir a causa.
- **Não tocar a F-realização, o α / caso 2, a flag P350c, o Marco G nem o DEBT-59.**
- **Não importar a quarentena.** `lab/` é leitura de semântica/doc, nunca import.

---

## Relatório (`typst-passo-359-relatorio.md` + resumo no chat)

O Estágio A: os números do vanilla (a)/(b), a causa de cada um com `file:line`, **uma raiz ou
duas**, e a Trava aprovada; o Estágio B: o(s) conserto(s) com `file:line`, os testes que viraram
de divergência para paridade (com a justificativa vs vanilla), o L0 tocado com hash; a prova de
que a F-realização, o α / caso 2, o caso 4, o morph `==`, a flag e o Marco G ficaram intactos; os
números da lente (edges 66) e da perf (antes/depois); `git status` limpo por estágio fora de
`lab/` e docs; lint 0/0; o caveat de stack.

## Fora de escopo (confirmado)

O de-bake do F-5 (adiado P353); F-6 (3 folhas, DEBT-58); Marco G; a flag na CLI (DEBT-59);
A2/A3 do caso 1 (declinados P358, só o gatilho); qualquer toque no α / `morph_canon` / `==` ou na
flag de diagnóstico.
