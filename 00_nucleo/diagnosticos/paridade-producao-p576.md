# P576 — Alinhamento de parágrafo com `#set text(dir: rtl)`

**Status**: `PARCIAL` — implementação funciona para texto 100% RTL; **P577
encontrou regressão confirmada** (sobreposição visual de glifos) quando a
linha RTL contém uma run de direcção mista (dígitos, texto latino
embutido) — exactamente o caso do documento de referência usado desde
P563/P567/P569/P574. Ver `00_nucleo/diagnosticos/paridade-producao-p577.md`.
Não fechar como concluído até um passo dedicado corrigir essa interacção.  
**Data**: 2026-07-05  
**Scope**: `01_core/src/entities/value.rs`, `01_core/src/entities/dir.rs`, `01_core/src/entities/layout_types.rs`, `01_core/src/entities/style_chain.rs`, `01_core/src/engine/eval/mod.rs`, `01_core/src/engine/eval/rules.rs`, `01_core/src/engine/eval/repr.rs`, `01_core/src/engine/layout/text.rs`, `01_core/src/engine/layout/cursor.rs`, `01_core/src/engine/layout/mod.rs`.  
**Commit da implementação**: *(a registar após commit)* — L0 dos prompts já commitado em `76f73904c`; a implementação L1 continua por commitar.

---

## 1. Pergunta

A sequência RTL está quase completa: ordem das palavras dentro da linha (P562/P564/P567) e espaços entre palavras (P569) já funcionam. Falta o alinhamento de parágrafo: um texto árabe começava sempre na margem esquerda, mesmo quando deveria começar à direita com `#set text(dir: rtl)`. Este passo implementa essa ligação.

---

## 2. Medições

### 2.1 Estado antes da implementação

- `01_core/src/engine/eval/rules.rs:931` — o arm `target == "text"` tratava `bold`, `italic`, `size`, `fill`, `weight`, `tracking`, `lang`, `font`; `dir` caía no warn de propriedade não suportada.
- `01_core/src/engine/eval/mod.rs:1196-1203` — `left`/`center`/`right`/`start`/`end`/`top`/`horizon`/`bottom` já eram `Value::Align` no escopo global; `ltr`/`rtl`/`ttb`/`btt` ainda não existiam.
- `01_core/src/engine/layout/mod.rs:499-501` — `cursor_x` e `line_start_x` inicializavam-se em `margin`, fixando a origem LTR.
- Sonda: texto árabe sem `dir:` começava à esquerda no vanilla e no cristalino; com `dir: rtl` o vanilla começava à direita, o cristalino emitia `unknown variable: rtl`.

### 2.2 Estado após a implementação

| Teste | Input | Resultado visual |
|-------|-------|------------------|
| RTL simples | `#set text(dir: rtl, lang: "ar", size: 20pt)`<br>`مرحبا بالعالم` | Texto começa à **direita** (paridade vanilla). |
| LTR explícito | `#set text(dir: ltr, lang: "ar", size: 20pt)`<br>`الكتاب على الطاولة` | Texto começa à **esquerda**. |
| Documento misto | Árabe `dir: rtl` + `#v(1em)` + latim `dir: ltr` | Árabe alinhado à direita, latim alinhado à esquerda. |
| Regressão P569 | `/tmp/p574-longo.typ` (sem `dir:`) | Output `pdftotext` idêntico ao estado anterior. |

Nota: a separação de parágrafos por linha em branco não força `flush_line` no cristalino (comportamento pré-existente fora do scope de P576); para o teste misto usou-se `#v(1em)` como separador explícito.

---

## 3. Implementação

### 3.1 `Value::Dir(Dir)`

- `01_core/src/entities/value.rs:149` — nova variante `Value::Dir(Dir)`.
- `01_core/src/entities/value.rs:219` — `type_name()` devolve `"direction"`.
- `01_core/src/engine/eval/repr.rs:77` — `repr()` devolve `"ltr"`/`"rtl"`/`"ttb"`/`"btt"`.

### 3.2 Identificadores `ltr` / `rtl` / `ttb` / `btt` no escopo global

`01_core/src/engine/eval/mod.rs:1205-1210` — define os quatro identificadores como `Value::Dir(Dir::*)`, seguindo o mesmo padrão dos identificadores de alinhamento.

### 3.3 `#set text(dir: ...)` reconhecido

`01_core/src/engine/eval/rules.rs:1073-1078` — o arm `"dir"` aceita `Value::Dir(Dir)` e empurra `text.dir` na `StyleChain`.

### 3.4 `TextStyle.dir` transportado até ao layout

- `01_core/src/entities/layout_types.rs:141` — novo campo `pub dir: Option<Dir>`.
- `01_core/src/entities/style_chain.rs:624` — `From<&StyleChain>` inicializa `dir: None`.
- `01_core/src/engine/layout/text.rs:61-64` e `:133` — lê `text.dir` da chain e funde no `TextStyle` efectivo.

### 3.5 Alinhamento RTL no Layouter

`01_core/src/engine/layout/cursor.rs:156-184` — novo método `align_current_line_rtl()`. Quando um item da linha corrente tem `style.dir == Some(Dir::RTL)`, calcula o offset até à margem direita e desloca todos os items da linha.

Pontos de chamada:
- `01_core/src/engine/layout/cursor.rs:232` — `flush_line()` alinha antes de comitar a linha.
- `01_core/src/engine/layout/mod.rs:1150` — `finish()` alinha a última linha do documento, que não passa por `flush_line`.

---

## 4. Decisão / Classificação

| Questão | Decisão | Base de medição |
|---------|---------|-----------------|
| Adicionar `Value::Dir(Dir)`? | **Sim** | Necessário para suportar `dir: rtl` como identificador (paridade sintática); `Dir` já existe em L1. |
| Onde controlar o alinhamento? | **Layouter (L1)** | A passagem L3 `layout_bidi` continua a reordenar palavras; o alinhamento da linha inteira é feito no `flush_line`/`finish`, que têm acesso ao estado da página. |
| `ltr`/`rtl`/`ttb`/`btt` no escopo global? | **Sim** | Mesmo padrão de `left`/`center`/`right` (`eval/mod.rs:1196-1203`). |
| Scope-out? | **ttb/btt** | Escrita vertical continua fora de scope, conforme `infra/layout_bidi.md`. |
| Separar parágrafos por linha em branco? | **Fora de scope** | Bug pré-existente: linha em branco não força `flush_line`. Documento misto validado com `#v(1em)`. |

---

## 5. Inferências e riscos

1. **Inferência**: a detecção automática de RTL (`unicode-bidi`) não deve decidir o alinhamento de parágrafo; só o `dir:` explícito o faz, tal como no vanilla.  
   **O que a refutaria**: vanilla alinhar à direita texto árabe sem `dir:` — não observado.

2. **Risco baixo**: o alinhamento é aplicado no flush/finish, o que significa que o layout interno continua LTR. Em documentos RTL longos com quebras de linha, cada linha é alinhada à direita individualmente; a quebra de linha ainda é calculada com base na largura LTR. Para a maioria dos casos árabes simples, isto é visualmente aceitável; casos complexos podem precisar de reflow RTL no futuro.

3. **Risco a monitorar**: o bug de separação de parágrafos por linha em branco pode tornar documentos mistos com `dir:` diferentes numa única linha. Não é introduzido por P576, mas afecta o critério de fecho literal do passo.

4. **Risco materializado (P577)**: o risco 2 acima ("cada linha é alinhada
   individualmente") previa o problema certo sem o testar. P577 confirmou
   que o documento de referência padrão da sequência RTL (com um `42`
   embutido) produz **sobreposição visual de glifos** — `flush_line()`
   parece ser chamado duas vezes para o que devia ser uma única linha
   quando há uma run de direcção mista, e cada chamada alinha
   independentemente à margem direita. Ver
   `00_nucleo/diagnosticos/paridade-producao-p577.md` para a medição
   completa e a tabela de posições.

---

## 6. Estado dos testes

```bash
cargo build --release --bin typst
crystalline-lint .
cargo test --workspace
```

- `crystalline-lint .` → `✓ No violations found`.
- `cargo test --workspace` → confirmado em P577: **4208 passed, 0 failed,
  8 ignored**. (Este relatório tinha deixado o resultado como "em
  background" — corrigido pela regra de proveniência de medição escrita
  depois de P575.)

---

## 7. Conclusão

`#set text(dir: rtl)` está implementado e produz alinhamento à direita para parágrafos árabes 100% RTL, com paridade visual ao vanilla. A propriedade `dir` é reconhecida em `#set text(...)`, os identificadores `ltr`/`rtl`/`ttb`/`btt` estão no escopo global, e o Layouter aplica o alinhamento na hora de fechar cada linha.

**Revisão pós-P577**: a sequência RTL **não** está completa. P577 mediu o
documento de referência padrão da sequência (com um `42` embutido, usado
desde P563) e encontrou sobreposição visual de glifos — o alinhamento
funciona para texto RTL puro mas quebra quando a linha mistura direcções.
Este passo fica com estado `PARCIAL`; a correcção é objecto de um passo
dedicado (ver recomendação em `paridade-producao-p577.md`).
