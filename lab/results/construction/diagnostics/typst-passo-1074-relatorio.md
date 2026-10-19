# Relatório de Execução — Passo 1074: Heading Força `italic=false` em vez de Herdar — Achado #9 do P1031

**Data**: 2026-08-18
**Passo**: 1074 — Heading Força `italic=false` em vez de Herdar (Achado #9 do P1031)
**Gate**: `ADR-0127` (Classificação: Mudança de Comportamento por Defeito / Paridade com a Linguagem Typst)
**Status**: CONCLUÍDO COM ÊXITO (Herança completa de `TextStyle` comprovada no PDF via `pdffonts`, paridade com Vanilla Typst)

---

## 1. Contexto e Motivação (Achado #9 do P1031)

No Vanilla Typst (`crates/typst-library/src/model/heading.rs:277-285`), o elemento `Heading` define exclusivamente `out.set(TextElem::weight, FontWeight::BOLD)` e escala de tamanho (`TextElem::size = Em(scale)`). O estilo de itálico não é modificado pelo cabeçalho, herdando normalmente o estilo de texto ativo no contexto envolvente (`#set text(style: "italic")`).

No Crystalline ([`01_core/src/compiler/layout/heading.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/heading.rs)), o layout inicializava `layouter.style` com:
```rust
layouter.style = TextStyle {
    bold: true,
    italic: false,
    size: heading_size,
    ..TextStyle::default()
};
```
Isso causava duas divergências semânticas:
1. Forçava `italic: false`, quebrando a herança de `#set text(style: "italic")`.
2. O uso de `..TextStyle::default()` descartava silenciosamente todos os outros estilos ativos no contexto envolvente (`fill`, `font`, `lang`, `dir`, `tracking`, `top_edge`, `bottom_edge`, etc.).

---

## 2. Auditoria e Esclarecimento de `heading_level` (§3 do L0)

### 2.1 Origem de `heading_level: Option<u8>`
O campo `pub heading_level: Option<u8>` **não foi criado neste passo**. Ele é um campo pré-existente da struct `TextStyle` em [`01_core/src/entities/layout_types.rs:175-176`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/layout_types.rs#L175-L176), introduzido no Passo 99 (ADR-0038/0039 forward-compat):
```rust
/// Nível de heading — ADR-0038/0039 forward-compat.
pub heading_level: Option<u8>,
```
Devido ao uso anterior de `..TextStyle::default()`, esse metadado era sempre colapsado para `None`. Ao refatorar para `..prev.clone()`, o nível ativo `Some(*level)` passa a ser explicitamente registrado no `TextStyle` durante o layout do heading.

### 2.2 Tabela Completa de Campos de `TextStyle`

| Campo | Tipo | Origem | Comportamento no Heading Pós-P1074 | Justificativa |
| :--- | :--- | :--- | :--- | :--- |
| `bold` | `bool` | Core | Forçado `true` | Canônico Typst (`FontWeight::BOLD`) |
| `size` | `Pt` | Core | `heading_size` (escala) | Canônico Typst (`Em(1.4)` / `Em(1.2)` / `Em(1.0)`) |
| `heading_level` | `Option<u8>` | ADR-0038/P99 | Preenchido `Some(*level)` | Metadado do nível do cabeçalho |
| `italic` | `bool` | Core | **Herdado de `prev.italic`** | Paridade Vanilla (P1074 / Achado #9) |
| `fill` | `Option<Color>` | ADR-0038/P99 | **Herdado de `prev.fill`** | Preserva cores ativas (`#set text(fill: ...)`) |
| `font` | `Option<FontList>` | Core | **Herdado de `prev.font`** | Preserva famílias de fonte ativas |
| `lang` | `Option<Lang>` | Core | **Herdado de `prev.lang`** | Preserva localização ativa |
| `dir` | `Option<Dir>` | P576 | **Herdado de `prev.dir`** | Preserva direção do texto (`ltr`/`rtl`) |
| `weight` | `Option<u16>` | P136/P289 | **Herdado de `prev.weight`** | Preserva pesos explícitos |
| `tracking` | `Option<Length>` | P136 | **Herdado de `prev.tracking`** | Preserva espaçamento entre letras |
| `top_edge` / `bottom_edge` | `Option<TextEdge>` | P762/P837 | **Herdados de `prev`** | Preserva alinhamento de bordas de linha |
| `subscript` / `superscript` | `bool` | P448 | **Herdados de `prev`** | Preserva sub/sobrescritos |
| `highlight` (+ configs) | `Option<Color>` | P449/P471 | **Herdados de `prev`** | Preserva realce de fundo |
| `baseline_offset` | `Length` | P448 | **Herdado de `prev`** | Preserva offset vertical de baseline |

---

## 3. Alteração Implementada (§1 e §2 do L0)

Em [`01_core/src/compiler/layout/heading.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/heading.rs):

```rust
    let prev = layouter.style.clone();
    layouter.style = TextStyle {
        bold: true,
        size: heading_size,
        heading_level: Some(*level),
        ..prev.clone()
    };
```

---

## 4. Medição Diferencial e Auditoria no PDF Gerado (§4 e §5 do L0)

Executado o repro real nos dois binários (`/usr/local/bin/typst` vs `target/release/typst`) para documentos com e sem itálico envolvente:

* **Documento Normal**: `= Cabecalho`
* **Documento Itálico**: `#set text(style: "italic")` + `= Cabecalho`

### 4.1 Inspeção de Fontes Embutidas no PDF (`pdffonts`)

| Binário / Caso | Normal (`= Cabecalho`) | Com `#set text(style: "italic")` | Comportamento Observado |
| :--- | :--- | :--- | :--- |
| **Vanilla Typst** | `LibertinusSerif-Bold-Identity-H` | `LibertinusSerif-BoldItalic-Identity-H` | Seleciona variante BoldItalic |
| **Crystalline (Antes)** | `AAAAAA+LibertinusSerif-Bold` | `AAAAAA+LibertinusSerif-Bold` ❌ | Ignorava itálico envolvente |
| **Crystalline (Pós-P1074)** | `AAAAAA+LibertinusSerif-Bold` | `AAAAAA+LibertinusSerif-BoldItalic` ✅ | **Paridade idêntica com Vanilla** |

### 4.2 Inspeção do Operador de Fonte no PDF Descomprimido

* **Heading Normal (`= Cabecalho`)**:
  ```text
  BT
  /F1 15.4 Tf
  [ <0001> 0 <0002> 0 <0003> -10 <0005> -7 <0004> 0 <0002> 0 <0007> 0 <0006> 0 <0008> 0 ] TJ
  ET
  ```
  *(Onde `/F1` referencia o Object 5: `AAAAAA+LibertinusSerif-Bold`)*

* **Heading com Itálico (`#set text(style: "italic")` + `= Cabecalho`)**:
  ```text
  BT
  /F1 15.4 Tf
  [ <0001> 0 <0002> 0 <0003> -10 <0005> -7 <0004> 0 <0002> 0 <0007> 0 <0006> 0 <0008> 0 ] TJ
  ET
  ```
  *(Onde `/F1` agora referencia o Object 5: `AAAAAA+LibertinusSerif-BoldItalic`)*

---

## 5. Validação Final

* **Teste Unitário em Memória**: `p1074_heading_inherits_italic_and_other_styles` em `01_core/src/compiler/layout/tests.rs` (valida `bold: true`, `italic: true` e herança de `fill: Color::Srgb`).
* `crystalline-lint .`: APROVADO (0 erros, 0 avisos de drift).
* `cargo test --workspace`: APROVADO (5.947 testes, 100% PASS).
