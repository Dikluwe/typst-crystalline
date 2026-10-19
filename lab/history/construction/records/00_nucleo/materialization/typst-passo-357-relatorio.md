# Relatório P357 — Medição da lacuna (ii): múltiplos func same-kind (A2 vs A3 vs fechar)

> **Tipo**: recon read-only (probes vanilla 0.14.2 via CLI em `/tmp`; zero código de produto, zero
> L0; suíte não re-rodada; árvore limpa; `RUST_MIN_STACK=33554432`). Termina numa **decisão proposta
> ao dono** (como P354/P355). Saída detalhada: `00_nucleo/diagnosticos/f-recon-lacuna-ii-passo-357.md`.
>
> **Veredito.** A premissa "o vanilla acumula N func same-kind" (herdada do P354/B1) **não se
> sustenta quando medida**. **Decisão do dono: FECHAR a lacuna (ii) + conserto de ordem
> (innermost-first); A2/A3 declinados.**

**HEAD**: pós-P356 (37adaf867). **Branch**: Tekt. **Pré-condição**: P356 fechado (suíte 2736, lint 0/0).

---

## 1 — Demanda, medida contra a referência (o gate da lição do P355)

Varredura de **todos** os `#show` func-form em `docs/reference/`, `docs/tutorial/`, `docs/guides/`:
**ZERO exemplos** de **2+ func same-kind** sobre o mesmo elemento (a acumular). Cada func é o único
do seu seletor (`styling.md:107` 1 func + 3 show-set; `tables.md:733/761/794` 1 func cada em blocos
distintos; `tutorial/3-advanced.md:536` 1 func; os de texto à parte). A promoção "composable / good
practice" do `styling.md` é **explicitamente** sobre **show-set** = **lacuna (i)** (feita no P356).
**Demanda da (ii): marginal — medida, não suposta** (a regra que o P355 quebrou, agora honrada).

## 2 — Comportamento exato do vanilla 0.14.2 (probes)

| caso | vanilla | crystalline | leitura |
|---|---|---|---|
| output muda de kind — `it=>[A:]+body` ⨁ `it=>[B:]+body`, `= T` | **`"B:T"`** (um, innermost/última) | **`"A:T"`** (um, 1ª) | divergência = **ORDEM**, não acumulação |
| output mesmo kind — `it=>heading[…]` ×2 | **ERRO** `maximum show rule depth exceeded` | erro/α | **nenhum acumula** — ambos erram (= caso 2) |
| output embrulha — `underline`/`emph` | termina (`"T"`) | termina | nada a reconciliar |

**Refuta a premissa**: o B1 do spike (acumulação, "última-aplicada vence", 3 passes) era **show-set**
(fold no `map`/chain, `lib.rs:458-464`) = lacuna (i). Para **func**, o vanilla **não acumula**:
aplica um (muda de kind/embrulha) ou erra (mesmo kind). **A lacuna (ii) como "acumulação de func" é
um artefacto de medição** (B1 conflado).

## 3 — Espaço de desenho (medido) + custo de A3

- **A1** (por-`RuleId`-uma-vez): **inviável** — regride o caso 2.
- **A2** (por-`(RuleId, morph_canon)`): acumularia func **onde o vanilla erra** (caso mesmo-kind) →
  **diverge mais**; no caso comum (muda de kind) basta a ordem. Pior que declinar.
- **A3** (por-instância): paridade exata, mas o caso mesmo-kind **é o que o vanilla erra** e o P348 já
  divergiu disso de propósito (α; ADR-0107). **Custo medido**: substitui o α no loop
  `apply_show_rules`/`apply_all` (`rules.rs:125-301`, ~176 linhas; 16 usos de
  `morph_canon`/`MAX_SHOW_RULE_DEPTH`/`active_guards`); reabre P348 + P350c; flip de **5 testes do
  caso 2** (`p348_show_recursao_converge_para_ponto_fixo:693`, `p348_..._o_inf_converge:708`,
  `p348_..._ciclo_erra:722`, `f3s2_show_callout_anti_recursao_termina:657`,
  `show_rule_nao_recursiva_sem_stack_overflow:3042`) + a flag de erro completo. **Alto, por demanda nula.**
- **A4 (emergente)**: a divergência real é **só de ordem** — o conserto barato (innermost-first)
  casa o `"B:T"` do vanilla **sem tocar o α**.

## 4 — Decisão do dono e o lote seguinte

**Decisão (P357): FECHAR a lacuna (ii) + conserto de ordem (innermost-first); A2/A3 declinados.**
O lote seguinte (não este recon) materializa:
1. Inverter a iteração de `node_rules` (`rules/eval/rules.rs`) → **innermost-first** (última-declarada
   vence; casa o `"B:T"` do vanilla no subconjunto de um-func-efetivo). **Não toca o α** (regra única
   = no-op; cross-kind inalterado).
2. O teste `multiplos_func_same_kind_ainda_diverge_lacuna_ii` (P356) **vira de divergência para
   paridade** (passa a asserir `"B:T"`).
3. **Declarar** o residual no L0 (`show.md`): 2+ func same-kind **mesmo-kind-returning** → ambos
   erram por recursão (= caso 2, já tratado pelo α/teto), com o **gatilho de reabertura** para A2/A3
   se uma demanda real de ordem/acumulação exata surgir.

## 5 — Gates / estado

```
read-only: nenhum código de produto, nenhum L0 tocado; probes via CLI/vanilla em /tmp (revertidas
  por construção — nada escrito no repo). Suíte NÃO re-rodada (mantém-se 2736 do P356).
lint: crystalline-lint . = 0/0 (inalterado).
INTACTOS: α/caso 2, caso 4, morph ==/morph_canon, flag P350c, Marco G — não tocados (read-only).
saída: f-recon-lacuna-ii-passo-357.md (a medição + a decisão §6). árvore limpa fora de docs.
caveat de stack respeitado nas probes (vanilla debug).
```

## Fora de escopo / próximo

A **implementação** do conserto de ordem (innermost-first) + L0 (Trava) — lote seguinte. O
diagnóstico do contador `1.1`≠`0.1` / DEBT-60 (P358). O de-bake do F-5 (adiado P353); F-6; Marco G;
flag na CLI (DEBT-59); qualquer toque no α / `morph_canon` / `==` ou na flag.
