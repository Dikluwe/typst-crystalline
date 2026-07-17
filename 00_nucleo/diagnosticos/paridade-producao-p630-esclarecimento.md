# Relatório — Passo 630 (P630): Esclarecimento `place.rs` vs `placement.rs` e `cursor.rs:650`

**Data:** 2026-07-09  
**Commit base da verificação:** `0ef7b303c` (fecho de P629)  
**Foco:** P629 alterou `place.rs:44` e `cursor.rs:650` para a nova assinatura `layout_sub_frame`, mas a sonda de P628 só tinha mapeado `grid.rs` e `placement.rs`. Verificar se isto é uma omissão real da sonda ou uma discrepância de enquadramento.

---

## Resumo executivo

Não é uma omissão real da sonda de P628.

- `place.rs` e `placement.rs` são ficheiros distintos com papéis complementares; apenas `placement.rs` é um dos "cinco caminhos" mapeados por P628.
- `cursor.rs:650` é o sub-layout dos corpos de nota de rodapé — um caso já conhecido e legítimo, não um novo caminho de layout.
- O refactor de P629B acabou por tocar em todos os call-sites de `layout_sub_frame_with_width` (incluindo os dois acima), mesmo não estando no plano escrito de P629. A mudança foi mecânica e sem alteração de comportamento.

Conclusão: o mapa de P628 mantém-se válido; pode prosseguir-se com a migração de `boxed.rs` conforme planeado.

---

## Verificações efectuadas

### 1. `place.rs` vs `placement.rs`

```bash
ls -la 01_core/src/engine/layout/place.rs 01_core/src/engine/layout/placement.rs
```

Resultado: são ficheiros diferentes.

- **`01_core/src/engine/layout/place.rs`** (3.251 bytes): entrypoint do elemento `PlaceElem`, atomizado conforme ADR-0109. Contém a free function `layout()` que decide entre `float: true` (sub-frame + buffer de floats) e `float: false` (delega em `layout_place`).
- **`01_core/src/engine/layout/placement.rs`** (8.958 bytes): contém os métodos `layout_align()` e `layout_place()` da impl `Layouter`. É um dos cinco caminhos mapeados por P628 e consome `layout_sub_frame` directamente.

Relação: `place.rs` chama `layouter.layout_sub_frame(...)` no ramo `float: true` e `layouter.layout_place(...)` no ramo `float: false`; `layout_place` vive em `placement.rs` e também chama `layout_sub_frame`. São camadas diferentes da mesma feature (`#place`), não caminhos independentes.

### 2. `cursor.rs:650`

```bash
sed -n '640,660p' 01_core/src/engine/layout/cursor.rs
```

Contexto (linhas 643–650):

```rust
let combined = Content::sequence(vec![
    Content::text(format!("[{}] ", n)),
    (*body).clone(),
]);
let (h, items) = self.layout_sub_frame(
    &combined,
    super::sub_frame::SubLayoutRegion {
        origin_x: 0.0,
        width: avail_w,
        height: None,
        align_rtl: true,
        unconstrained_height: true,
    },
);
```

Esta chamada está dentro da função `flush_footnotes` (comentário na linha 560: "flush dos footnote bodies pendentes no rodapé da página actual"). Cada `body` é o conteúdo de uma nota de rodapé; o sub-layout isola o corpo numa região de largura fixa antes de o posicionar bottom-up no rodapé.

Trata-se de um **sub-layout legítimo e já identificado** — não é um novo caminho de layout ao nível dos cinco mapeados por P628.

### 3. O que P629B alterou de facto

```bash
git show 6c3813369 --stat
```

O commit `6c3813369` (P629B — refactor `layout_sub_frame_with_width` para `SubLayoutRegion`) tocou em 15 ficheiros, incluindo:

- `01_core/src/engine/layout/cursor.rs`
- `01_core/src/engine/layout/place.rs`
- `01_core/src/engine/layout/placement.rs`
- `01_core/src/engine/layout/grid.rs`
- `01_core/src/engine/layout/sub_frame.rs`

Ou seja, P629B migrou **todos** os call-sites existentes de `layout_sub_frame_with_width`, não apenas os dois que constavam no texto do passo (`grid.rs` e `placement.rs`). As alterações em `cursor.rs` e `place.rs` foram mecânicas: substituição da assinatura antiga pela struct `SubLayoutRegion`, sem mudança de semântica.

---

## Por que não é omissão da sonda de P628

A sonda de P628 operava ao nível dos **caminhos de layout de alto nível**:

| Caminho | Ficheiro(s) principal(is) |
|---|---|
| Fluxo principal | `mod.rs`, `cursor.rs` |
| Grid | `grid.rs` |
| Place / Align | `placement.rs` |
| Columns | `columns.rs` |
| Boxed | `boxed.rs` |

`place.rs` não é um caminho independente: é o arquivo da feature `PlaceElem` (atomização ADR-0109) que delega em `placement.rs`. O seu uso de `layout_sub_frame` é consequência do uso em `placement.rs`, não um novo padrão.

`cursor.rs:650` é um sub-layout pontual dentro do flush de footnotes — semelhante a outros sub-layouts já encapsulados em `placement.rs` e `grid.rs`. P628 não tinha o objetivo de listar todos os call-sites de `layout_sub_frame_with_width`; tinha o objetivo de mapear onde há duplicação de lógica de "colocar texto num espaço" entre caminhos de alto nível.

---

## Decisão

- **Não actualizar o mapa de P628.** O mapa continua correcto no seu enquadramento (caminhos de alto nível).
- **Registar que P629B foi mais abrangente do que o plano escrito de P629.** O texto do passo mencionava apenas `grid.rs` e `placement.rs`, mas a execução mecânica correctamente atualizou também `place.rs` e `cursor.rs` porque partilhavam a mesma função deprecada. Não houve regressão.
- **Prosseguir com `boxed.rs`** (próximo passo da sequência de unificação), conforme proposto em P628.

---

## Critérios de fecho

- [x] Confirmado que `place.rs` e `placement.rs` são ficheiros diferentes e qual a relação entre eles.
- [x] Confirmado que `cursor.rs:650` faz o sub-layout dos corpos de nota de rodapé — caso legítimo, não omissão da sonda.
- [x] Mapa de P628 mantido; não é necessária correção.
- [x] Relatório curto em `00_nucleo/diagnosticos/paridade-producao-p630-esclarecimento.md`.
