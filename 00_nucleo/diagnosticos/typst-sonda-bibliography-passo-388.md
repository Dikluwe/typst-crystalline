# Sonda de viabilidade — `bibliography`/`cite` (Passo 388) — PASSO PARADO

**Tipo:** Sonda de viabilidade (read-only; ADR-0108). **Não materializa código.**
**Data:** 2026-06-22. **HEAD:** `08fed76d3` (pós-P387).
**Resultado:** o Passo 388 **parou por decisão do dono** — a premissa do passo (materializar
`bibliography`/`cite` greenfield sobre hayagriva, com o runtime de introspecção como ponto
fraco) é **refutada pela medição**. Este documento é o único artefacto; nenhum L0/ADR/código
foi escrito.

> **Por que sonda-primeiro (§2 do passo).** O passo mandava medir o runtime antes de codar e
> **derivar** o escopo da medição, não assumir. A medição derivou um escopo nulo para "Fase 1
> sobre hayagriva": quase tudo já existe, e o que falta é divergência **deliberada**, não dívida.

---

## 1. O que a sonda mediu (estado real, com `file:line`)

`bibliography`/`cite` **já está funcionalmente materializado** (P159A-G + P181D-H, auditado
P258). Não é greenfield.

| Componente | Estado | Evidência |
|------------|--------|-----------|
| Variants `Content` | `Content::Cite { key, supplement, form }` + `Content::Bibliography { entries, title }` | `content.rs:753-801` |
| `cite(key, supplement:, form:)` | materializado | `structural.rs:940-977` |
| `bibliography(entries, title:)` | materializado; entries = `Array<Dict>` inline | `structural.rs:887-915`; `extract_bib_entries` `743-861` |
| `BibEntry` | 16 campos (4 obrig. + 12 opc.); ~70-75% de `hayagriva::Entry` | `bib_entry.rs:81-101` |
| Formas de citação | 4: `Normal`/`Prose`/`Author`/`Year` | `citation_form.rs:36-42`; render `layout/cite.rs:19-46` |
| **Numeração** | atribuída em **ordem de aparição** (`[N]` na forma Normal) | `introspect.rs:498-500`; `bib_store.rs` |
| `BibStore` | sub-store (P181); `entries: Vec`, `numbers` | `entities/bib_store.rs` |
| Layouter consumer | resolve forma por lookup; render | `layout/cite.rs`, `layout/bibliography.rs` |
| Introspector integration | `bib_entry_for_key`/`bib_number_for_key`; Cite indexado em `kind_index[Citation]` | `introspector.rs:524`; P162/P181D-H |

## 2. O runtime de introspecção — **forte**, não fraco (refuta §2/§8 do passo)

O passo temia que o substrato 2 (coleta cross-document) fosse o ponto fraco. A medição diz o
contrário:

- `query_by_kind(kind) -> Vec<Location>` devolve em **ordem de aparição** (walk monotónico).
  `introspector.rs:393-394` + push em ordem de walk `introspect.rs:421-424`; teste confirma
  (`introspector.rs:700-705`).
- `query_by_label(label) -> Option<Location>` — resolução de label estável. `introspector.rs:397`.
- Numeração 1-based em ordem de aparição já existe (`introspect.rs:498-500`).

Ou seja, **os recursos que a Fase 1 numérica precisaria (coleta ordenada, numeração) já
existem**. O que o passo classificaria como "graded por falta de runtime" não se aplica.

## 3. O gap real medido — e por que NÃO é o que o passo propõe

O que falta vs vanilla **é divergência deliberada (ADR-0054), não dívida não-planeada**:

- **Sem parser `.bib`/CSL externo (hayagriva).** Input é `Vec<BibEntry>` literal (dict inline);
  `bib_entry.rs` tem um parser BibTeX-**like** próprio (413 LoC), sem hayagriva. `structural.rs:884`.
- **Sem CSL styling (citationberg; APA/MLA/IEEE/…).** Só as 4 formas hardcoded. `style` é
  scope-out (`structural.rs:933`).
- **Bibliografia não filtra por citados.** `layout/bibliography.rs:25` lista **todas** as entries
  fornecidas; não usa `query_by_kind(Citation)` para listar só as citadas. `content.rs:799` marca
  "sem validação cross-reference" / ADR-0017.

**Decisão documentada que o passo contraria.** ADR-0062 (hayagriva) está **`PROPOSTO`**, com
deferimento explícito; DEBT-55 (atualizado P258, 2026-05-15): cumprido "via paridade manual
P159A-G, **sem dependência crate hayagriva real**"; promoção a hayagriva "**diferida até consumer
real exigir CSL styling completo**". **Nenhum consumer-trigger** desse tipo é estabelecido no
Passo 388. Cablar hayagriva agora reverteria uma decisão deliberada sem o gatilho que ela nomeou.

## 4. Conclusão e por que o passo parou

A "Fase 1 sobre hayagriva" do Passo 388 propõe materializar precisamente o que P258 **adiou de
propósito**, sob uma premissa (greenfield + runtime fraco) que o código **refuta**:

1. As variants, 4 formas, numeração, coleta e layout **já existem**.
2. O runtime de introspecção **suporta** coleta ordenada e numeração.
3. O que falta (parser externo hayagriva + CSL completo) é **scope-out deliberado** ADR-0054,
   adiado por DEBT-55/ADR-0062 **até consumer exigir** — gatilho ausente no passo.

Por ADR-0108 (medir antes de decidir; distinguir intenção de comportamento; desconfiar do
enquadramento), proceder cablaria hayagriva contra uma decisão documentada, sem o trigger. O dono
**parou o passo** para re-planear o P388 a partir deste estado real.

## 5. Insumos para re-planear o P388 (se/quando for retomado)

Caso um futuro passo queira mexer aqui, os **gaps genuínos pequenos** (dentro do modelo-próprio,
sem reverter a divergência) que a sonda isolou:

- **(a) Filtrar a bibliografia por citados** — `layout/bibliography.rs` passar a usar
  `query_by_kind(Citation)` para listar só as chaves citadas (o runtime já dá a coleta). É o gap
  mais "real" e barato; não precisa de hayagriva.
- **(b) Forma author-data plena / desambiguação** — refino das 4 formas existentes.
- **(c) Cablar hayagriva** — só com o consumer-trigger que DEBT-55/ADR-0062 nomeiam; é a reversão
  grande da divergência ADR-0054, não um refino.

Estes são **candidatos**, não reservas (política "sem novas reservas"); a decisão é do dono.

---

## Referências
- DEBT-55 (atualizado P258) — bibliography via paridade manual; hayagriva diferida.
- ADR-0062 (`PROPOSTO`) — autorização hayagriva, com deferimento.
- ADR-0066 (`PROPOSTO`) — runtime de introspecção (parte materializada P205B+C).
- `inventario-bib-state.md` (P180) — inventário precedente do estado bib.
- Série P159A-G — materialização manual; P181 — `BibStore`.
- ADR-0108 — medir antes de decidir (a regra que travou o passo).
