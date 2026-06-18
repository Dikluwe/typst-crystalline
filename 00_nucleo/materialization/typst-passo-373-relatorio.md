# Relatório P373 — F-5b fatia (2) ATERROU: transporte aninhado + de-bake do render `#set text`

> **Desfecho.** Escopo **A** (transporte + fatia 2 num lote). O bug do transporte (single-wrap, P372)
> foi corrigido (wraps aninhados por escopo léxico) e a **fatia 2** (de-bake do render `#set text`/
> `#set par` pelo canal `custom`) aterrou. Duas correções **emergiram da medição** (ADR-0108): os
> **resolvers da chain** passaram a ler o `custom` (para o render chegar a todo o layout, não só ao
> merge arm), e a **baseline 12-vs-11** foi corrigida (init de `self.style` da chain). **Pipeline
> COMPLETO verde.** O item (1) da auditoria P362 fecha em **4/4** (fonte única do render). DEBT-61
> fechado.

**HEAD**: pós-P371. **Branch**: Tekt. **L0**: `f_fronteira_e1.md` §3a.13 (realizado) + §3a.14
(Realização P373), hashes sincronizados, lint 0/0.

---

## O que aterrou

### 1. Transporte aninhado (corrige o bug P372, observável independente do F-5b)
`eval/mod.rs` `eval_markup`: substituído o `wrap_start` único (single-wrap-final-collapse) por
**boundary-tracking + reverse-fold**. Cada fronteira de `#set` (mudança do `custom` vs o estado
corrente) regista `(parts.len(), Styles do delta que ESTE #set introduziu)`; no fim, fold de dentro
para fora produz o aninhamento `Styled(A, [X, Styled(B, [Y])])` — espelho do `styled_with_map`
por-`#set` recursivo do vanilla. **Content-preserving:** `#set` único → 1 wrap (= antes); chaves
diferentes → mesmo efeito; mesma-chave → correto por-segmento (a correção).
**Prova isolada:** `font_wiring_segunda_font_diferente_ambas_embebidas` (2 fonts embebidas).

### 2. Fatia 2 — de-bake do render
- `Content::Text(EcoString, TextStyle)` → `Content::Text(EcoString)` (cascata ~90 sítios).
- `#set text(<campo>)` e `#set par(leading)` → canal `custom` (`"text.<campo>"`/`"par.leading"`,
  Value canónico) em `eval_set_rule`, em vez do `StyleDelta` tipado assado no node. Validação/erro-hard
  de `lang`/`font` preservada (ADR-0052/0053); só a **saída** muda (typed → custom).
- **`PartialEq` do `Content::Text`** passa a comparar só o texto (render saiu do node) — mais fiel ao
  vanilla (`TextElem` ghost fields).
- `morph_canon` **inalterado**: o `custom`-Styled é `is_semantically_empty` → desce → `#set text X ==
  X` (transparente). **α (gate duro): caso 2 + rede +11 verdes** — não reabriu.

### 3. Achado emergente A — resolvers da chain leem o `custom`
A premissa "decodificar no merge arm basta" era **falsa** (medido pelo `09-cidfont`): código que lê
`self.style.font`/`.size` **fora** do merge arm (margem, medição, `space_width`) não via o `#set
text`. Correção: os resolvers de `StyleChain` (`bold/italic/size/weight/tracking/leading/lang/font`)
consultam, **no mesmo nó e antes de subir** (top-wins exato), o campo tipado **E** o `custom`. Assim
`TextStyle::from(&chain)` — e o `self.style` lido em todo o layout — reflete o `#set text` como o
antigo delta tipado refletia. O `custom` continua transparente à morfologia.

### 4. Achado emergente B — baseline 12-vs-11 (decisão do dono: corrigir a raiz)
A fatia 2 expôs uma inconsistência **pré-existente**: o `Layouter` inicializava `self.style` de
`font_size`=**12.0** (`DEFAULT_FONT_SIZE`, geometria), mas o texto resolve a **11.0** (default da
chain, ADR-0039); `space_width()` (= `self.style.size`) ficava a 12 em docs não-embrulhados, 11 nos
embrulhados — visível só no espaço-líder. O transporte da fatia 2 embrulha o corpo do `#set` num
`Styled`, expondo o **0.6pt** no `09-cidfont`. **Correção (ADR-0039, fonte única):** `self.style`
inicial deriva da chain (`TextStyle::from(&default_chain())` = 11), mantendo `font_size_pt`=12 para
geometria. **Propagou a 4 goldens** (`02`/`03`/`07`/`09`), todos a **mesma** correção sub-pixel
(espaço-líder 12→11, = tamanho do texto). **Os 4 goldens foram regenerados (aprovação do dono)** —
**divergência mecânica consciente (ADR-0107):** a paridade é com a língua; os bytes do PDF divergem
de propósito e o novo output é mais consistente. `01-markup-plain` (sem espaço-líder) intocado.

---

## Gates (pipeline COMPLETO)

```
typst-core (RUST_MIN_STACK=33554432): 2747 passed, 0 failed  (α/caso 2 + rede +11 + morph verdes)
03_infra:        472 passed  (font_wiring 2-fonts CORRIGIDO + 9 goldens p307b verdes)
02_shell + wiring: 47 passed
crystalline-lint .: ✓ 0 violations
```

**Testes atualizados (de-bake):** `morfologia_eq_ignora_render_na_chain` (render agora no
`custom`-Styled, não no node); helpers `texto_bold_contendo`/`styled_bold_*` leem o `custom`
(`text.bold`/`text.weight`).

**Goldens regenerados:** `02-markup-heading`, `03-text-styling`, `07-multi-feature`, `09-cidfont`
(byte-diff = só posições; mesmo tamanho; correção sub-pixel do espaço-líder).

---

## Linhagem / dívida

- **L0:** `f_fronteira_e1.md` §3a.14 (Realização P373) + §3a.13 (marcado realizado). Hashes
  sincronizados (`--fix-hashes`), lint 0/0.
- **DEBT-61 FECHADO** (P371 fatia 1 + P373 fatia 2). Item (1) da auditoria P362: **4/4** (fonte única
  do render).

## Fora de escopo
Marco G; DEBT-59; DEBT-60. (A fatia 2, antes prevista para o P374, foi feita aqui no escopo A.)
