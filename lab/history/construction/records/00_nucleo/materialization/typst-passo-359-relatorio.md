# Relatório P359 — DEBT-60 (contador `1.1`≠`0.1` + "Secção"): Stage A + Stage B PAROU (re-escopo)

> **Desfecho.** O Stage A (diagnóstico) mediu as duas anomalias e localizou as causas. O Stage B
> (correção) **parou e reverteu**: a correção de **(a)** (gatear o avanço do contador de heading em
> `numbering_active`) tem um **raio de explosão de ~20 testes** porque o **P335 (Lote F-2 S5)
> tornou o contador de heading INCONDICIONAL por decisão deliberada**, e ~20 testes de
> introspect/layout encodam isso. Não é um conserto isolado — é **re-escopo** de uma decisão
> arquitetural prévia. Por limite duro ("se a causa mora num caminho protegido/decidido → parar e
> reportar, não conserto silencioso"), reverti o Stage B. **Suíte de volta a 2736; lint 0/0.**
> **Nenhum código P359 commitado.**

**HEAD**: pós-P356 (37adaf867) + P357/P358 não-commitados. Caveat `RUST_MIN_STACK=33554432`.

---

## Stage A — diagnóstico (medido, `file:line`)

**Duas raízes distintas** (medido):

### (a) Contador — `heading.rs:91` + `introspect.rs:529-538`
- **Vanilla 0.14.2 (oráculo, 2 probes):** um heading **só avança o contador quando numbering
  ativo**. `= A`(sem)/`==B`(num)/`==C` → B = `0.1.` (A não avança L1). `…num`/`=A`(=1)/`=B`(none)/
  `=C` → "1. A"/"B"/"**2.** C" (B não-numerado **não** consome — C foi 1→2). Fonte: numbering
  counter "remains unset if numbering is None" (`model/heading.rs:140-143`).
- **Crystalline:** `to_payload` emite `counter_update: Step` **incondicional** (`heading.rs:91`); o
  `introspect` aplica `apply_hierarchical_at("heading", …)` **sem gate** (`introspect.rs:534`).
  Figure gateia por `is_counted`, Equation por `numbering_active` — **Heading não gateia**.

### (b) Outline "Secção" — `outline.rs:57`
- **Vanilla:** o `prefix` do outline formata o numbering e **só** acrescenta supplement para
  figure/equation, **não** heading (`model/outline.rs:123-124`) → mostra só o número.
- **Crystalline:** `layout_outline` usa `Content::reference(label)` → renderiza "Secção {n}"
  (cross-reference, correta no corpo, errada no outline).
- **Raiz independente de (a)**, MAS o **número** que o outline deve mostrar depende de (a) estar
  certo (senão mostra "1.1" sem "Secção" — meio-conserto).

---

## Stage B — PAROU: (a) é re-escopo, não conserto

Implementei (a) [gate `numbering_active` no payload + `introspect`] e (b) [número via
`formatted_counter_at`, sem "Secção"]. Resultado da suíte: **20 testes FALHARAM** — todos de
**contador/introspect/ref**, nenhum do outline.

**A causa (medida no teste `introspector_consistencia_heading`, `introspect.rs`):**
> `// Lote F-2 S5 (P335): marcador removido — contador de heading **incondicional**.`
> Cria headings via `Content::heading` (não-numerados) e asserta `formatted_counter("heading") ==
> "1.2.1"`.

O **P335 tornou o contador de heading incondicional por decisão deliberada** (+ gate no **display**,
não no **step**). ~20 testes (introspect fixpoint, ref-resolution "Secção", paridade de pipelines)
encodam "heading não-numerado avança o contador". Gatear o step (a) reverte essa decisão e flipa os 20.

**Equivalência observável (medida).** A arquitetura P335 (contador incondicional + gate no display)
é **observavelmente igual** ao vanilla em **todos os casos comuns** (all-numbered: os números batem;
all-unnumbered: nada exibe, contador interno irrelevante). **Diverge só no caso misto/confinado** —
exatamente o DEBT-60: um heading não-numerado avança o L1 que um heading numerado depois exibe
(`1.1` vs `0.1`). É um caso **raro** (numbering confinado com headings não-numerados antes).

**Confissão de método (ADR-0108).** O Stage A localizou `heading.rs:91` como causa mas **não checou**
que o contador incondicional era uma decisão **deliberada do P335** com ~20 testes dependentes — o
Stage A disse "nenhum teste existente afirma o valor divergente", e estava **errado**. A medição que
faltou: grep dos testes que asserem o contador de heading. O Stage B a fez (ao falhar) — e parou.

---

## A decisão é do dono (re-escopo)

(a) não é fechável sem reverter a decisão P335 (contador incondicional) e atualizar ~20 testes.
Opções:
- **(1) Re-escopar (a) num lote próprio**: gatear o step em `numbering_active` + atualizar os ~20
  testes (que asserem o valor incondicional — divergente) para refletir "heading não-numerado não
  avança" + ADR/L0 documentando a reversão do P335. Substancial; toca o coração do introspect.
- **(2) Aceitar a divergência (manter DEBT-60 (a))**: a divergência é só no caso misto/confinado
  (raro); a arquitetura P335 é observavelmente igual ao vanilla no resto. Manter DEBT-60 (a) aberto
  como divergência consciente medida (custo: o `1.1`≠`0.1` no caso confinado).
- **(3) Fazer só (b)**: remover o "Secção" do outline (contido a `outline.rs`, sem raio de
  explosão). MAS o número mostrado seria o do contador incondicional (`1.1`), não o vanilla
  (`0.1`) — meio-conserto (tira "Secção", mantém o número divergente). Por S5b, **não recomendo
  sem (a)** (mascara o número errado).

**Recomendação marcada:** **(2) ou re-escopar (1) deliberadamente** — não fazer (3) sozinho (S5b).
A divergência (a) é rara e a arquitetura P335 é uma escolha consciente; revertê-la é um lote próprio
com ADR, não um conserto de DEBT. A escolha é do dono.

**Estado:** Stage B revertido; suíte 2736; lint 0/0; nenhum código P359; DEBT-60 permanece aberto.
**Intactos:** F-realização, α/caso 2, caso 4, morph `==`, flag P350c, Marco G — não tocados.

---

## Stage A — fecho com as 2 medições que faltavam (a pedido do dono: selar na medição, não na aposta)

### Medição 1 — numbering confinado é idiomático? (decide aceitar-(a) vs re-escopar)
Varredura de **todos** os `set heading(numbering:)` em `docs/` (reference + tutorial): **6
ocorrências, TODAS document-wide top-level** (`styling.md:23,104`; `context.md:78,106`;
`2-formatting.md:181,200`). **ZERO confinadas** a bloco/escopo. → **numbering de heading confinado
NÃO é idiomático** — o caso DEBT-60 (a) (numbering confinado com heading não-numerado antes) é
**construído, não da linguagem**. Reverter o P335 (~20 testes, coração do introspect) por um canto
que a linguagem não promove pagaria caro por **demanda nula** (anti-ADR-0107). → **aceitar a
divergência (a)** é a opção **medida-correta** (não a aposta "raro").

### Medição 2 — o supplement "Secção" (b) é independente de (a)?
**Sim quanto ao raio de explosão** (probe (b) só em `outline.rs`, revertida): suíte **2736/0** — a
correção (b) **não** flipa nenhum teste do contador (a). MAS a probe revelou **dois factos**:
1. **Lacuna de teste:** a minha probe (b) trocou `reference(label)` por
   `formatted_counter_at(query_by_label(label))` e o outline passou a mostrar **"A B" — SEM número
   nenhum** (vanilla: "1. A"/"1.1. B"), e a **suíte passou 2736/0**. **Nenhum teste asserta o número
   no outline** — foi por isso que tanto o "Secção" quanto o desaparecimento do número passaram. (A
   `query_by_label` devolve `None` para as auto-labels do TOC — não estão no registry consultável.)
2. **(b) é independente de (a) mas NÃO é trivial.** O número do TOC vem **embutido** no
   `resolved_text` `"Secção {n}"` que o `reference(label)` renderiza — "Secção" (rótulo) e "{n}"
   (número) vêm **juntos** por esse caminho, e o `resolved_text` é **partilhado** com as referências
   de corpo (`@heading` → "Secção 1.1", correto). Para o outline mostrar **"{n}" sem "Secção"** é
   preciso uma **fonte de número independente** (a `Location` do heading → `formatted_counter_at`),
   o que exige **fiar a location/número na entrada do TOC** (`headings_for_toc`/`HeadingForToc`),
   contido ao **caminho do TOC** (NÃO ao contador (a)). + um **teste do número no outline** (a lacuna).

**Conclusão das medições:** (a) → **aceitar a divergência** (confinado não-idiomático, medido). (b)
→ **lote isolado limpo** (independente de (a)), mas requer fiar o número no TOC + fechar a lacuna de
teste — **não** o one-liner que o recon supôs, e **não** um meio-conserto (o supplement e o número
são eixos distintos, como o dono apontou).

---

## DECISÃO DO DONO + Stage B executado

**Decisão (P359): aceitar (a) + fazer (b) como lote isolado.**

### (a) — ACEITE como divergência consciente medida (DEBT-60 atualizado)
Registrado na **DEBT-60**: causa = P335 (contador incondicional, deliberado); raio ~20 testes;
demanda **nula** (numbering confinado não-idiomático, medido); arquitetura P335 observavelmente igual
ao vanilla fora do caso misto/confinado. Gatilho de reabertura registrado. **Não tocado o contador.**

### (b) — FEITO (contido a `compute_heading_auto_toc`; mais simples que o recon supôs)
A medição 2 supôs "fiar a Location/número na entrada do TOC". A fonte mostrou um caminho **mais
limpo**: a auto-label `auto-toc-{n}` é **TOC-específica** (não partilhada com refs de corpo), e o seu
`resolved_text` é produzido por `compute_heading_auto_toc` (`introspect.rs:453`). **Fix = uma linha
lá**: `format!("Secção {}", n)` → `format!("{}.", n)` (número, sem supplement; `{n}.` espelha o corpo,
`mod.rs:720`). O outline (`reference(auto-toc-label)`) passa a renderizar `"1."`/`"1.1."` — **sem
"Secção"**, e o nº == o nº do corpo == vanilla. `compute_labelled` (refs de corpo `@heading`) fica
**intacto** ("Secção {n}", correto no corpo) — labels distintas.

**Observável (vanilla 0.14.2 oráculo):** doc-wide numbering / `#outline()` → crystalline **"1. A" /
"1.1. B"** = vanilla. "Secção" eliminado.

**Testes:** 3 asserções do `resolved_text` da auto-toc (que afirmavam `"Secção 1"`) **viraram** para
`"1."`/`"2."` (justificadas vs vanilla; eram o valor divergente):
`heading_auto_toc_walk_emite_tag…`, `consumer_c4_recebe_some_para_auto_toc_label`,
`p191b_compute_heading_auto_toc…`. **+1 novo** `layout_outline_mostra_numero_sem_supplement_seccao`
(assere o número no outline + ausência de "Secção") — **fecha a lacuna de teste** que escondia ambas
as anomalias.

### Gates (todos verdes)
```
build: workspace limpo. suíte: 2736 → 2737 (+1 teste de outline; 3 asserções viradas em lugar,
  justificadas vs vanilla). Demais crates: 472/24/2/21, 0 falhas.
lint: crystalline-lint . = 0/0.
ACEITAÇÃO: (b) outline mostra o número sem "Secção" (paridade vanilla); (a) divergência aceite (DEBT-60).
INTACTOS: F-realização, α/caso 2, caso 4, morph ==/morph_canon, flag P350c, Marco G — não tocados
  (mudança em compute_heading_auto_toc / introspect, fora desses caminhos).
lente: content→elements = 66 INALTERADO; elemento→elemento = 0.
perf (mesma sessão): depois 0.7673 s ± 0.0256 (≈ P358 0.78; mudança de formato de string, sem regressão).
L0: introspect.md (auto-toc resolved_text) + layout_outline.md (número da entrada) + DEBT-60
  atualizado; hashes sincronizados (introspect.rs 15b1586c, outline.rs 2d2e3feb).
```

### Honestidade (ADR-0108)
O Stage A inicial errou ("nenhum teste flipa"); o Stage B pegou (20 falhas → re-escopo de (a)). O
dono insistiu em **medir** antes de decidir 1 vs 2 (numbering confinado idiomático?) e em **separar**
(b) de (a) — ambas as medições mudaram o resultado: (a) aceite por demanda nula medida; (b) revelou-se
limpo MAS a 1ª tentativa (query_by_label) **apagou o número** e a suíte **não pegou** (lacuna de
teste) — o novo teste fecha-a. **(b) feito; (a) registrado como divergência medida.**
