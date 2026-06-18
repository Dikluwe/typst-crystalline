# F-recon lacuna (ii) — múltiplos func same-kind: A2 vs A3 vs fechar (P357)

> **Tipo**: recon read-only (probes vanilla 0.14.2 compiladas; zero código de produto, zero L0;
> suíte não re-rodada; árvore limpa; `RUST_MIN_STACK=33554432`). Mede a **demanda** e a **ordem
> exata** da lacuna (ii) — múltiplos `#show` **func** de mesmo kind sobre um elemento — e o **custo
> de A3**, para o dono decidir A2 / A3 / **fechar**. **Não implementa, não decide.**
>
> **Veredito medido (resumo):** a premissa "o vanilla acumula N func same-kind" — herdada do P354/B1
> — **não se sustenta quando medida**. O B1 do spike era **show-set** (fold de estilo) = **lacuna
> (i), já feita no P356**. Para **func** (transformacional), o vanilla **não acumula**: aplica **um**
> (output muda de kind/embrulha) ou **erra** (output do mesmo kind). A única divergência real é de
> **ordem** (qual func único vence). → **recomendação: fechar a lacuna (ii)** com um **conserto de
> ordem barato** (innermost-first); **A2 e A3 declinados**.

**Pré-condição**: P356 fechado (lacuna (i) feita; suíte 2736; lint 0/0). HEAD pós-P356 (37adaf867).

---

## 1 — A demanda, medida contra a referência (o gate da lição do P355)

Varredura de **todos** os `#show` func-form em `docs/reference/`, `docs/tutorial/`, `docs/guides/`:
- `styling.md:107` `#show heading: it => block[…]` — **1 func** (+ 3 show-set; o exemplo canônico da
  **lacuna (i)**, já feita). `styling.md:153` `#show "Project": smallcaps` — selector de **texto**.
- `guides/tables.md:733/761/794` `#show table.cell: it => …` — 3 ocorrências em **blocos de exemplo
  distintos** (1 func cada). `tutorial/3-advanced.md:536` `#show heading: smallcaps` — 1 func.
  `tutorial/2-formatting.md:244` `#show "ArtosFlow": name => box[…]` — selector de **texto**.

**Medido: ZERO exemplos** de **2+ func same-kind sobre o mesmo elemento** (a acumular), em toda a
referência/tutorial/guias. Cada func é o **único** do seu seletor. A promoção "composable / good
practice" do `styling.md` é **explicitamente** sobre **show-set** (`set align`, `set text` —
overridable), que é a **lacuna (i)**. **A demanda da (ii) é marginal — agora medida, não suposta.**

## 2 — A ordem/comportamento exato do vanilla 0.14.2 (probes descartáveis)

| caso | vanilla 0.14.2 | crystalline | leitura |
|---|---|---|---|
| **output muda de kind** — `it=>[A:]+it.body` ⨁ `it=>[B:]+it.body`, `= T` | **`"B:T"`** (só o **innermost** / última-declarada; o output é Sequence → a outra regra não re-casa) | **`"A:T"`** (só a 1ª declarada) | **um** func aplica em ambos; divergência = **ORDEM** (innermost vs first), **não** acumulação |
| **output do mesmo kind** — `it=>heading[A #it.body]` ⨁ `it=>heading[B #it.body]`, `= T` | **ERRO** `maximum show rule depth exceeded` (output re-casa → recursão de instâncias frescas) | erro (teto-64) / α | **nenhum** acumula limpo — ambos **erram** |
| **output embrulha** — `it=>underline(it)` ⨁ `it=>emph(it)`, `= T` | termina (`"T"`); o outer vira underline → as heading-rules param | termina | **não há acumulação func observável** a reconciliar |

**Medido (refuta a premissa):** o vanilla **não tem** um padrão comum e funcional de "N func same-kind
acumulam". O caso `output muda de kind` (o mais comum, ex. o `it=>block[…]` do doc) aplica **um**; o
`output mesmo kind` **erra**. A acumulação limpa do B1 (final = última-aplicada, 3 passes) era
**show-set** (fold no `map`/chain — `lib.rs:458-464`), ou seja **lacuna (i)** (P356). **A lacuna (ii),
como "acumulação de func", é um artefacto de medição** (B1 conflado show-set com func).

## 3 — O espaço de desenho, re-confirmado (medido vs inferido)

- **A1** (guard por-`RuleId`-uma-vez): **inviável** (medido P354) — a regra guardada não re-aplica ao
  output recursivo → regride o caso 2 (`p348` a→b→c pára em b).
- **A2** (guard por-`(RuleId, morph_canon)`): forçaria acumulação de func. **Mas não há o que
  acumular**: no caso comum (muda de kind) basta a ordem; no caso mesmo-kind o **vanilla erra** —
  A2 faria o crystalline **acumular onde o vanilla erra** → **diverge mais**, não menos. **Pior que
  declinar** para a (ii).
- **A3** (guard por-instância do vanilla): paridade exata de instância — **mas o caso mesmo-kind é
  exatamente o que o vanilla ERRA**, e o P348 já divergiu disso de propósito (α: `it=>[=Z]` converge
  para Z onde o vanilla erra — divergência consciente, ADR-0107). A3 **reabriria** essa decisão selada.
- **A4 (emergente da medição)**: a divergência real é **só de ordem** (qual func único vence). O
  **conserto barato** — inverter a iteração de `node_rules` para **innermost-first** (última-declarada
  primeiro) — casa o `"B:T"` do vanilla, **sem tocar o α**, e **fecha** a única divergência real.
  (É a "sub-melhoria barata" que o P354/P355 já anteciparam.)

## 4 — O custo de A3, medido (o preço na frente)

A3 substitui o modelo **α** (ponto-fixo morfológico, P348) pela terminação por **identidade de
instância** do vanilla. Toca o loop inteiro `apply_show_rules`/`apply_all` (`rules/eval/rules.rs:125-301`,
~176 linhas; **16** usos de `morph_canon`/`MAX_SHOW_RULE_DEPTH`/`active_guards`) e **reabre o P348 +
P350c**, pondo em risco/flip **5 testes do caso 2**:
- `p348_show_recursao_converge_para_ponto_fixo` (`:693`) — a→b→c por morfologia,
- `p348_show_recursao_o_inf_converge_divergencia_consciente` (`:708`) — a divergência `o_inf`,
- `p348_show_recursao_ciclo_erra_com_mensagem_vanilla` (`:722`) — ciclo → erro,
- `f3s2_show_callout_anti_recursao_termina` (`:657`),
- `show_rule_nao_recursiva_sem_stack_overflow` (`:3042`),
+ a **flag de erro completo (P350c)**, construída sobre o caminho α (`full_error`/histórico de
morfologias). **Custo alto** — reabre uma divergência consciente **selada** — para comprar paridade
**num caso que o próprio vanilla erra**. **Não justificado.**

## 5 — Recomendação (marcada) — a DECISÃO é do dono

**Recomendo FECHAR a lacuna (ii)** (ramo "não é feature" do mapa do P357), porque a medição mostra
que **múltiplos func same-kind não acumulam no vanilla** (aplicam um / erram) e **não há demanda**
(0 exemplos na referência). Concretamente:
1. **Conserto barato de ordem** — inverter `node_rules` para **innermost-first** (última-declarada
   vence), casando o `"B:T"` do vanilla no subconjunto de **um-func-efetivo**. **Não toca o α.** É a
   única divergência **real** e fica em **paridade**. (O teste `multiplos_func_same_kind_ainda_diverge_
   lacuna_ii` do P356 vira de "A:T (diverge)" para "B:T (paridade)".)
2. **Declarar** o residual: 2+ func same-kind **mesmo-kind-returning** → ambos (vanilla e crystalline)
   **erram** por recursão — não é acumulação, é o caso 2 (já tratado pelo α/teto). Registrar, não
   reconciliar.
3. **Declinar A2 e A3**: A2 acumularia onde o vanilla erra (diverge mais); A3 reabre o α/P348 selado
   (custo §4) por demanda nula. **ADR-0107/0108**: não reabrir o caso 2 sem demanda de ordem exata
   medida — e a medição diz que não há.

**Gatilho de reabertura** (se algum dia): um padrão real (pacote/doc) onde 2+ func same-kind
**precisem** acumular com ordem exata → vira teste de paridade contra o vanilla → A2/A3 nessa hora.

**A escolha (fechar+ordem / A2 / A3) é do dono.** O lote seguinte (o conserto de ordem, ou nada)
nasce da decisão. Nenhum código/L0 tocado; probes revertidas; árvore limpa; lint inalterado; suíte
não re-rodada.

---

## 6 — DECISÃO DO DONO (P357): **fechar + conserto de ordem (innermost-first)**

Escolhido **fechar a lacuna (ii)** com o **conserto de ordem barato**. O lote seguinte materializa:
1. **Inverter a iteração de `node_rules`** (`rules/eval/rules.rs`) para **innermost-first**
   (última-declarada vence), casando o `"B:T"` do vanilla no subconjunto de **um-func-efetivo**.
   **Não toca o α** (regra única = no-op; cross-kind inalterado). O teste
   `multiplos_func_same_kind_ainda_diverge_lacuna_ii` (P356) **vira de divergência para paridade**
   (passa a asserir `"B:T"`).
2. **Declarar o residual** (2+ func same-kind **mesmo-kind-returning** → ambos erram por recursão =
   caso 2, já tratado pelo α/teto) no L0, com o gatilho de reabertura para A2/A3.
3. **A2 e A3 declinados** (A2 acumularia onde o vanilla erra; A3 reabre o α/P348 selado por demanda
   nula). ADR-0107/0108.

Este recon (P357) **fecha** aqui; o conserto de ordem + L0 (Trava) é o lote seguinte.
