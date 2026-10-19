# Relatório P355 — caso 1: Trava da ADR-0109 → medição refuta → revert → reconciliação

> **Desfecho.** O P355 ia materializar o rumo (B) do P354 (declarar a composição same-kind como
> divergência mecânica, via **ADR-0109** + L0, depois um conserto de ordem). Na **Trava** de selagem,
> o dono aplicou a ADR-0108 à própria ADR: a classificação **língua-vs-mecânica** que autoriza
> divergir **só pode vir da fonte, medida**. Ela estava **inferida**. A medição a **refutou**:
> **composição same-kind é LÍNGUA documentada**. A ADR-0109 + edições de L0 foram **REVERTIDAS**, o
> rumo (B) abandonado, e abriu-se um **recon de reconciliação** (`f-recon-composicao-passo-355.md`).
> **Nenhum `.rs` de produto escrito.** Suíte **2733/0**; lint **0/0**; árvore limpa.

**HEAD**: pós-P353 (1eb216303). **Branch**: Tekt. Caveat `RUST_MIN_STACK=33554432`.

---

## 1 — Estágio ADR/L0 (Trava) — o que foi escrito e depois revertido

Conforme o molde do P355, escrevi a declaração **antes** de qualquer `.rs` e parei na Trava:
- **ADR-0109** (`typst-adr-0109-…md`, `PROPOSTA`): decisão "uma regra efetiva, não acumula";
  classificação "acumulação = mecânica de realização (guard por-instância)"; custo "B1, 0 testes,
  raro"; gatilho de reabertura; alternativas A1/A2/A3.
- **L0**: `show.md` (secção "Composição same-kind: divergência consciente" + invariante de ordem
  innermost-first) e `f_fronteira_e1.md` §3b.6 (caso 1 = divergência declarada). Hashes sincronizados
  (`crystalline-lint --fix-hashes`: `show.rs`→238523ab; declarantes de `f_fronteira_e1`→efb2e7c4);
  lint 0/0. Parei no chat para aprovação.

## 2 — A Trava do dono (ADR-0108 aplicada à ADR)

O dono **não selou no escuro**. Pergunta central: *"a acumulação de N regras same-kind no vanilla é
mecânica de realização, ou é semântica que o usuário do Typst conhece e usa?"* — porque a ADR-0107
exige que classificar algo como **mecânica** venha **da fonte**, e a cadeia P347 já pegara o erro
**inverso** (quase respeitar como língua algo mecânico; aqui o risco é tratar como mecânica algo que
é língua). *"0 testes hoje" ≠ "0 documentos reais"* (a armadilha do `it.body == [a]`).

**Concessão honesta (registrada):** a classificação era **inferida**. Eu medira o **mecanismo** (o
guard por-instância produz a acumulação — `lib.rs:472-474`) e estendera a classificação "mecânica" do
**papel de terminação** (P347d) para o **papel de composição**, **sem medir a intenção**.

## 3 — A medição de intenção (a fonte; `file:line`) — REFUTA a classificação

- **Doc de referência** (`lab/typst-original/docs/reference/language/styling.md`, *Show rules*):
  **promove** múltiplas regras same-kind — exemplo com **4× `#show heading`** (3 show-set + 1
  transform) e *"This is good practice because now these rules can still be overridden by later
  show-set rules, keeping styling composable."* → **composição same-kind = intenção da língua.**
- **Mecanismo vanilla** (`typst-realize/src/lib.rs`): num `verdict`, todas as show-set matching
  dobram na chain (`map.apply(transform); continue`, `:458-464`, sem consumir o passe) e a 1ª func
  vira `step`, aplicada sob `chained = styles.chain(&map)` (`:341,357`) → as show-set ficam **ativas
  quando a func realiza**. O guard é a **implementação**; a composição é a **intenção**.

**Veredito:** a classificação "mecânica" da ADR-0109 é o **erro inverso-P347d**, refutado pela fonte.
Composição same-kind é **língua**; divergir quebraria documentos reais (o **exemplo canônico** do
doc). A ADR não podia ser selada.

## 4 — O crystalline hoje (probes P355, revertidas)

| caso | crystalline | vanilla/doc | veredito |
|---|---|---|---|
| múltiplos **show-set** same-kind | **compõe ✓** — `set text(bold)` ⨁ `set text(weight:700)` → um `Content::Styled{bold, weight}` (P352, `rules.rs:272`) | compõe | **paridade** |
| **show-set + func** (exemplo do doc) | **show-set PERDIDO ✗** — `set text(bold)` ⨁ `it=>[X:]+it.body` → `"X:T"` com "X:" não-bold (o func roda 1º; o output não casa heading; o show-set não entra) | show-set na chain, func sob ela | **diverge** |
| múltiplos **func** same-kind | uma efetiva ✗ (P354) | acumula | **diverge** |

## 5 — Ação: revert + reconciliação

- **Revertido**: ADR-0109 (apagada), `show.md`, `f_fronteira_e1.md`, e os hash-bumps (`show.rs`,
  `element_registry.rs`, `dynamic.rs`, `test_callout.rs`) → estado committed (P353). Lint 0/0;
  drift 0.
- **Recon de reconciliação** aberto: `00_nucleo/diagnosticos/f-recon-composicao-passo-355.md`, com as
  duas lacunas:
  - **(i) show-set + func** — conserto de ordem **contido**, **não reabre o α** (show-set é
    `Transformation::Style`, fora do guard/morph): dobrar a show-set sobre o **nó original**, aplicar
    o func, embrulhar o output do func no `Styles` (espelha `chained` do vanilla). Sítios: só
    `apply_show_rules`/`apply_all`. Baixo risco; 0 testes no combo; testes P352 ficam verdes.
  - **(ii) múltiplos func** — A2 (por-`(RuleId, morph_canon)`, diverge na ordem) ou A3 (por-instância,
    reabre o α) + a sub-melhoria de ordem innermost-first. **Fatia seguinte**, decisão A2/A3 do dono.
- **Recomendação**: materializar a **lacuna (i)** (exemplo canônico do doc; baixo risco; honra a
  composição documentada) como o próximo lote; adiar a (ii) medindo a demanda.

## 6 — Lições registradas

- **ADR-0108 aplicada a uma ADR**: a classificação língua-vs-mecânica que autoriza divergir é a única
  coisa do projeto que **nunca** pode vir de raciocínio — e foi pega inferida aqui. O freio do dono
  na Trava funcionou (reduz ≠ elimina, ADR-0108 §resíduo).
- **Erro inverso-P347d nomeado**: tratar como mecânica algo que é língua (composição). A medição da
  **intenção** (doc + design), não do mecanismo, é o que classifica.
- **S5b**: o conserto de ordem (P355 Estágio 1) **nunca** podia ser apresentado como "composição
  funciona" — e, medido, ele nem era o conserto certo (o combo show-set+func não é ordem-de-func, é a
  ordem show-set-vs-func). A declaração (ADR) teria mascarado um buraco maior.

## 7 — Gates

```
build/suíte: 2733/0 (nenhum código de produto tocado; probes revertidas).
lint: crystalline-lint . = 0/0; drift 0 (tudo de volta ao committed).
ADR-0109: revertida (apagada). L0 (show.md, f_fronteira_e1.md): revertidos ao committed.
INTACTOS: loop α/caso 2, caso 4, morph ==/morph_canon, flag P350c, Marco G — não tocados.
saída: f-recon-composicao-passo-355.md (a reconciliação; veredito: composição é língua).
árvore limpa fora de docs/tools; caveat de stack respeitado nas probes.
```

## Fora de escopo / próximo

A implementação da lacuna (i) (show-set+func) — próximo lote, com Trava de L0 (`show.md` com a ordem
correta) + testes do exemplo do doc. A lacuna (ii) (múltiplos func, A2/A3) — fatia seguinte. O
diagnóstico do contador `1.1`≠`0.1` (DEBT-60) — P356.
