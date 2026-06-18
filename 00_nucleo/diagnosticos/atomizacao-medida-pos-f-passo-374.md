# Atomização medida pós-F (recon read-only, em valores) — Passo 374

> **O que é.** Retrato **em valores** da atomização após o F fechar (P373: item (1) 4/4, DEBT-61
> fechado). Separa **(a) a atomização que o F entregou, medida pelo PRINCÍPIO** (fonte única, com
> `file:line`) de **(b) as métricas da lente** `tekt-cargo-dsm` (instrumento, **não gate do F**).
> **Read-only:** zero código de produto, zero L0. Tudo `[medido]`.

**Estado da árvore (registrado).** HEAD = `7031b23d9` (P372-revert). **As mudanças do P373 estão na
árvore de trabalho, não commitadas** — a medição da Parte (a) é feita **na árvore de trabalho** (o
estado P373, que é o que este passo quer); a lente (Parte b) analisa o source da árvore via Cargo.
Os números refletem o P373. (A pré-condição "árvore limpa" não está satisfeita — P373 por commitar;
sem impacto na medição read-only, registrado por transparência.)

**Lente.** `lente` (crate `lente_app`), repo `/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm`,
commit **`98d8f9e`** (`feat: --comparar a nível de item — trilha 0075→0078`). Comando:
`RUST_MIN_STACK=33554432 lente --pacote typst-core --estrutura --so-referencia --filtrar-stdlib`
(EXIT 0). Baseline de comparação: `baseline-estrutural-lente-passo-338.md`.

---

## Parte (a) — a atomização que o F entregou (medida pelo PRINCÍPIO)

**Esta é a atomização real do F** — contagem de estruturas com `file:line`, não um número da lente.

| # | O que o F removeu/colapsou | Antes | Depois | Evidência (HEAD/árvore) |
|---|---|---|---|---|
| 1a | `HeadingElem.numbering_active` (campo assado) | campo | **ausente** | `heading.rs:34` (comentário "removido"); sem campo na struct |
| 1b | `EquationElem.numbering_active` (campo do **elemento**) | campo | **ausente** | `equation.rs:29` (comentário "removido"); o gate vive na chain |
| 1c | `FigureElem.numbering` (campo assado) | campo | **ausente** | `figure.rs:28` (comentário); sem campo na struct |
| 1d | `TextStyle` assado em `Content::Text` | `Text(EcoString, TextStyle)` | **`Text(EcoString)`** | `content.rs:126` |
| | **Caminhos duplos (item (1) da auditoria P362)** | **4** | **0** | **fonte única do render — 4/4** |
| 2 | Arms de `morph_canon` por-elemento (numbering) | N arms | **1 arm transparente** | `content.rs:2139` `Styled if is_semantically_empty => desce` subsume o transporte; sem arms de numbering |
| 3 | ABI `figure_numbering: Option<&str>` (parâmetro/campo real) | 141 ocorr. / 16 fich. (P365) | **0** | só 3 refs **em comentário** (`engine.rs:62`, `func.rs:75`, `rules.rs:437`); 0 param/campo |
| 4 | Distinção semântica `strong`/`emph`/`#set text` | colapsados (`Styled[Bold]`, P101) | **variantes distintas** | `content.rs:147` `Strong(Arc<StrongElem>)`, `:150` `Emph(Arc<EmphElem>)`; teste `f5b_strong_distinto_de_set_text_bold` `:2386` |
| 5 | Canal de estilo | disperso (assado + chain) | **único: `StyleDelta.custom`** | numbering (`heading/equation/figure.numbering`) + user-props (P368, `format!("{}.{}")` `rules.rs:512`) + render `#set text`/`par` (P373, 9 chaves `text.*`/`par.leading`) |

**Nota (1b).** O campo saiu do **elemento** `EquationElem` (o caminho duplo). O enum
`ElementPayload::Equation` mantém um `numbering_active: false` **neutro** (`element_payload.rs:154`),
sobrescrito da chain (`custom("equation.numbering")`) no walk de emissão — placeholder, **não** gate
assado. O caminho duplo (elemento↔chain) está fechado.

**Leitura da Parte (a):** o F entregou **4 caminhos duplos → 0** (fonte única do render), o **ABI
partilhado 141 → 0**, os arms de `morph_canon` colapsados a **1**, a **distinção semântica
restaurada** (fidelidade ADR-0107) e o **canal de estilo unificado** no `custom`. **Esta é a
atomização que o F perseguiu — e cumpriu.**

---

## Parte (b) — as métricas da lente (INSTRUMENTO, não gate do F)

| Métrica | Baseline (P338) | **Atual (P373, árvore)** | Δ | Nota |
|---|---|---|---|---|
| `edges(elemento → elemento)` | 0 | **0** | 0 | **Atomização estrita perfeita** — elementos independentes, sem import cruzado. Mantido o arco todo. |
| `edges(content → elements::*)` | 66 | **68** | **+2** | **Alvo do Marco G, NÃO gate do F** (P346, órfão da lente). |
| `edges(elements::* → content)` | 65 | 67 | +2 | acoplamento inverso (contexto) |
| Total arestas (so-referência) | 675 | 688 | +13 | crescimento do arco (variantes + dynamic + outros) |

**Decomposição do delta `content→elements` (66 → 68, +2) — medido, não inferido:**
- **+2 = `strong` + `emph`** (presentes no conjunto atual de targets; `content.rs` importa ambos os
  módulos novos). **Pré-P371 não existiam** como módulos de elemento (colapsados em `Styled`,
  Passo 101) → não estavam nos 66 do P338. **São variantes PRESCRITAS** pela ADR-0026 (enum fechado,
  `Strong(Box)`/`Emph(Box)` distintos) / ADR-0105 (modelo D). **Não é regressão de atomização** — é o
  modelo prescrito.
- **de-bake do P373: 0 nesta métrica.** O de-bake removeu um **campo** (`TextStyle` de
  `Content::Text`) e moveu render para o `custom` — **não** mexeu nos imports `content → elements::*`.
  A métrica que o de-bake move é a **Parte (a)** (caminhos duplos), não esta.

**Ressalva dupla (a lição central do arco):**
1. `content→elements` é o alvo do **Marco G** (desacoplar o núcleo dos nativos → 0), **não gate do
   F**. O F nunca perseguiu este número (P346: órfão da lente).
2. O `66→68` **subiu por desenho** (variantes prescritas), **não por piora**. O alvo 0 é trabalho
   **separado e não-F** (Marco G; P361 mediu ser decisão de modelo α-vtable vs β-PropMap, grande).

---

## A leitura honesta (não confundir as duas)

- **A atomização do F (Parte a)** melhorou de forma **real e estrutural**: caminhos duplos **4→0**,
  ABI **141→0**, `morph_canon` **N→1 arm**, distinção semântica restaurada, canal **único**. Medida
  pelo princípio (fonte única), com `file:line`.
- **A lente (Parte b)** mostra `elemento→elemento = 0` (atomização estrita intacta) e
  `content→elements = 68` (que **sobe por desenho**, e cujo alvo 0 é o **Marco G, fora do F**).
- **O erro do arco** seria ler o `66→68` como "o F piorou a atomização". **Não é:** o F não toca essa
  métrica (é Marco G); o +2 é variante prescrita; a atomização do F é a Parte (a), que melhorou.

---

## Gates (read-only)

```
Nenhum código de produto, nenhum L0 escrito. Lente rodada (instrumento, EXIT 0, commit 98d8f9e).
Suíte NÃO re-rodada (P373 já a deixou verde). Árvore: P373 por commitar (registrado); lente sem
artefactos no repo (só /tmp). Lint inalterado. RUST_MIN_STACK=33554432.
Saída: este ficheiro. Termina aqui — não emenda o passo seguinte (Trava 5). A decisão é do dono.
```

## Fora de escopo (confirmado)
Marco G (decisão de modelo, não-F); DEBT-59; DEBT-60; qualquer código ou L0.
