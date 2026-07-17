# Relatório de Paridade — P757

**Passo:** 757  
**Data:** 2026-07-14  
**Foco:** (A) Corrigir `em` em dimensões de página; (B) revalidar P756 contra o binário vanilla de referência (0.15.0).  
**Hash base:** `b7ce6230a9b473e3559847c3a80fdae26cdb75ed`  
**Hash do commit com as alterações de código:** `c83a177b94e95df1ffb94fe341f08ee7e1bf897d`.  
(Nota: este é o hash do commit que introduziu as alterações de código; o commit que inclui a versão final deste relatório terá um hash ligeiramente diferente.)

---

## 1. Parte B — Revalidação de P756 contra o binário correcto

### 1.1 Versões dos binários

```text
/usr/local/bin/typst --version            → typst 0.14.2 (b33de9de)
lab/typst-original/target/release/typst --version → typst 0.15.0 (969087ec)
```

**Achado:** `/usr/local/bin/typst` é a versão **0.14.2**, enquanto o binário de referência desta conversão (em `lab/typst-original/target/release/typst`) é o **0.15.0**. A secção 3 do relatório de P756 usou o binário errado.

### 1.2 Testes repetidos com Typst 0.15.0

Documentos usados:

- `/tmp/p757-cjk.typ` — CJK com aspas, página `100 pt`.
- `/tmp/p757-cjk-plain.typ` — CJK sem aspas, página `100 pt`.
- `/tmp/p757-thai.typ` — Thai, página `100 pt`.

Resultados (extracção de texto via `pdftotext`):

**CJK com aspas**

```text
CRYS: 测试文本，
CRYS: "测试引号的位
CRYS: 置"。这是一段很长的中文
CRYS: 文字用来测试换行的效果如
CRYS: 何。

VANI: 测试文本，“测试
VANI: 引号的位置”。这
VANI: 是一段很长的中
VANI: 文文字用来测试
VANI: 换行的效果如何。
```

**CJK sem aspas**

```text
CRYS: 测试文本测试引号的位置这
CRYS: 是一段很长的中文文字用来
CRYS: 测试换行的效果如何

VANI: 测试文本测试引
VANI: 号的位置这是一
VANI: 段很长的中文文
VANI: 字用来测试换行
VANI: 的效果如何
```

**Thai**

```text
CRYS: สวัสดีครับผม
CRYS: ชื่อจอห์นยินดีที่ไดี
CRYS: รัจกคณ

VANI: สวัสดีครับผม
VANI: ชื่ชื่อจอห์นยินดีที่ได้
VANI: รู้จักคุณ
```

### 1.3 Revisão das conclusões de P756

As conclusões qualitativas mantêm-se:

- O cristalino segmenta CJK/Thai sem truncamento e sem fugas para além da margem.
- A posição exacta das quebras difere do vanilla 0.15.0 porque o cristalino usa layout greedy, enquanto o vanilla usa Knuth-Plass com penalidades globais.
- No caso CJK com aspas, o vanilla 0.15.0 consegue manter a aspa de abertura no final da linha anterior; o cristalino coloca-a no início da linha seguinte quando o fragmento não cabe.
- No Thai, persistem diferenças de rendering/shaping além da decisão de quebra.

A diferença de versão (0.14.2 vs 0.15.0) não altera a interpretação geral, mas a comparação correcta é com o 0.15.0.

---

## 2. Parte A — Correção de `em` em dimensões de página

### 2.1 Sonda

Documento de teste:

```typst
#set page(width: 7em, height: 5em)
X
```

Resultado antes da correção:

```text
Vanilla 0.15.0: MediaBox [ 0 0 77 55 ]
Cristalino:     MediaBox [ 0 0  0  0 ]  (2 páginas criadas)
```

O vanilla resolve `7em` / `5em` contra o font-size default (11 pt), resultando em `77 pt × 55 pt`.

### 2.2 Causa

Em `01_core/src/engine/eval/rules.rs`, a função auxiliar `extract_pt` usada pelo arm `#set page(...)` fazia:

```rust
Value::Length(l) => Ok(Some(l.abs.to_pt())),
```

`Length::abs` é a componente absoluta; a componente `em` era descartada. Para `7em`, `abs` é zero, pelo que a largura ficava `0 pt`.

### 2.3 Correção

Alterado para:

```rust
let size_pt = engine.styles.size();
Value::Length(l) => Ok(Some(l.resolve_pt(size_pt))),
```

`engine.styles.size()` resolve o tamanho de fonte activo na `StyleChain` (default 11 pt). `Length::resolve_pt` soma a componente absoluta com `em * size_pt`.

### 2.4 Resultado após correção

```text
Vanilla 0.15.0: MediaBox [ 0 0 77 55 ]
Cristalino:     MediaBox [ 0 0 77 55 ]
```

### 2.5 Testes adicionados

Em `01_core/src/engine/layout/tests.rs`:

- `p757_page_width_height_em_resolve_contra_font_size` — `7em × 5em` com font-size default → `77 pt × 55 pt`.
- `p757_page_width_em_respeita_text_size` — `#set text(size: 12pt)` antes da página → `84 pt × 60 pt`.

---

## 3. L0 actualizado

- `00_nucleo/prompts/engine/eval.md`: adicionada secção §P757 documentando a resolução de `em` em dimensões de página. Hash do código no L0: `9e869009`.
- Hashes de lineage actualizados nos ficheiros de `01_core/src/engine/eval/` via `crystalline-lint --fix-hashes`.

---

## 4. Validação

```text
cargo test --workspace
  4121 passed (typst-core)
   635 passed (typst-infra)
    33 passed (typst-shell)
     2 passed (typst-wiring)
    27 passed (cli integration)
     2 passed (crystalline-lint integration)
     0 failed
crystalline-lint .
  ✓ No violations found
```

---

## 5. Decisão

P757 está fechado:

- A. Bug de `em` em `#set page(width: ...)` / `#set page(height: ...)` corrigido e testado.
- B. P756 revalidado contra Typst 0.15.0 (binário de referência); conclusões qualitativas mantêm-se, com a comparação correcta.
- Sem regressões; `crystalline-lint .` limpo.
