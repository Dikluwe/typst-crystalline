# Relatório de Paridade — Passo 624

**Data:** 2026-07-09  
**Commit de implementação:** `bec2a9ac8`  
**Hash do L0 `rules/layout.md`:** `faf0ea7a` (ficheiros filhos); `layout/mod.rs` mantém `9c9b7122`  

---

## 1. Objetivo

Confirmar se `grid.rs`, `placement.rs`, `columns.rs` e `boxed.rs` já usam as
funções únicas de P593 (`text_width`, `line_content_right`) ou se ainda têm
"conta própria" de largura de texto.

---

## 2. Revisão ficheiro a ficheiro

| Ficheiro | Calcula largura de texto própria? | Observação |
|----------|-----------------------------------|------------|
| `grid.rs` | Não directamente | Delega em `measure_content_constrained` e `layout_sub_frame_with_width`. |
| `placement.rs` | Não | Usa `layout_sub_frame_with_width` para body e posiciona items. |
| `columns.rs` | Não | Manipula `cursor_x` / `width` da region; não mede texto. |
| `boxed.rs` | Não directamente | Delega em `measure_content_constrained` para body. |

Conclusão da revisão: **nenhum dos quatro ficheiros calcula largura de texto
por conta própria**. No entanto, `grid.rs` e `boxed.rs` chamam
`measure_content_constrained`, que **tinha** cálculo próprio de largura para
`Content::Text` usando `advance` directo em vez de `text_width`.

---

## 3. A conta própria encontrada

Local: `01_core/src/rules/layout/mod.rs:1323`

```rust
Content::Text(text) => {
    ...
    let space_w = self.metrics.advance(" ", self.style.size, &self.style).0;
    for word in text.split_whitespace() {
        let word_w = self.metrics.advance(word, self.style.size, &self.style).0;
        ...
    }
}
```

Este braço estimava larguras com `advance`, ignorando o shaping disponível em
`text_width` desde P593. Como `measure_content_constrained` é usado por
`Grid`, `Box`, `Block`, `Pad`, `Stack` e `Sequence`, a inconsistência podia
afectar qualquer estrutura que media texto em scripts contextuais antes do
layout real.

---

## 4. Testes com árabe em tabela, colunas e caixa

### 4.1 Tabela

```typst
#set text(font: "DejaVu Sans")
#table(
  columns: 1,
  [#set text(lang: "ar", size: 30pt); الكتاب على الطاولة],
)
```

| Implementação | Largura da linha | Observação |
|---------------|------------------|------------|
| Cristalino (antes e depois) | 243.34 pt | Texto numa linha; sem divergência de largura. |
| Vanilla | 243.33 pt | Texto numa linha. |

Diferença residual: posicionamento da borda da tabela (vanila desenha borda;
cristalino ainda não) — fora do escopo de P624.

### 4.2 Colunas

```typst
#set page(columns: 2)
#set text(lang: "ar", size: 30pt)
الكتاب على الطاولة
```

| Implementação | Posição | Observação |
|---------------|---------|------------|
| Cristalino | Coluna esquerda | Diferença de direção/ordem de colunas. |
| Vanilla | Coluna direita | RTL coloca o bloco na coluna direita. |

Larguras das palavras idênticas (54 / 108 / 126 pt). A divergência é de
layout RTL em colunas, não de cálculo de largura.

### 4.3 Caixa (`box`)

```typst
#box(width: 150pt)[
  #set text(lang: "ar", size: 30pt)
  الكتاب على الطاولة
]
```

| Implementação | Alinhamento | Observação |
|---------------|-------------|------------|
| Cristalino | Esquerda | Texto LTR-like dentro do box. |
| Vanilla | Direita | Texto RTL dentro do box. |

As quebras ocorrem nas mesmas posições; larguras das palavras idênticas. A
diferença é de alinhamento RTL dentro do box.

---

## 5. Decisão

**Unificar `measure_content_constrained` para usar `text_width`.**

Embora os testes visuais de P624 não tenham exposto divergência de largura
(coincidiram por causa do input ou do caminho de layout real), a função
`measure_content_constrained` claramente duplicava a lógica de medição de
largura de palavra em vez de reutilizar a fonte única de verdade de P593.

A correção previne divergências futuras em estruturas que dependem desta
medição antecipada (grids com colunas auto, boxes com clipping, blocks não
quebráveis, etc.).

---

## 6. Implementação

Ficheiro alterado: `01_core/src/rules/layout/mod.rs:1323`

```rust
Content::Text(text) => {
    ...
    let space_w = self
        .metrics
        .text_width(" ", self.style.size, &self.style)
        .0;

    for word in text.split_whitespace() {
        let word_w = self
            .metrics
            .text_width(word, self.style.size, &self.style)
            .0;
        ...
    }
}
```

O Prompt L0 `00_nucleo/prompts/rules/layout.md` foi actualizado para
documentar que `measure_content_constrained` deve usar `text_width`.

---

## 7. Validação

```bash
cargo build --release --workspace   # ok
cargo test --workspace              # ok — todos os testes passam
crystalline-lint .                  # ✓ No violations found
```

---

## 8. Notas

- Os quatro ficheiros originais da pergunta (`grid.rs`, `placement.rs`,
  `columns.rs`, `boxed.rs`) não tinham cálculo de largura próprio.
- A única conta própria relevante estava no helper partilhado
  `measure_content_constrained`, que agora está unificado com P593.
- As diferenças visuais observadas em tabela/colunas/box (bordas,
  direcção de colunas, alinhamento RTL) não são de cálculo de largura e
  ficam registadas fora do escopo deste passo.
