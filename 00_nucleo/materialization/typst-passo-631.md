---
# P631 — `boxed.rs` usa o helper partilhado `layout_sub_frame`

> **Passo:** 631
> **Data:** 2026-07-09
> **Foco:** P628 identificou que `boxed.rs` tem alinhamento RTL "próprio" — chama `align_current_line_rtl()` isoladamente sobre os itens do body, em vez de usar o parâmetro `align_rtl` do helper partilhado, porque `boxed.rs` é inline (avança `cursor_x` do pai, não `cursor_y`) e isola os itens dentro da `current_line` do pai antes de os mover. Este passo introduz uma variante do helper que cobre este caso, reduzindo a duplicação de save/restore de estado que hoje existe em `boxed.rs`.
> **Tipo:** Refactor. Sem mudança de comportamento pretendida — a mesma disciplina de P629 aplica-se aqui.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P628 (sonda e plano), P629 (helper `layout_sub_frame`/`SubLayoutRegion` já criado, no módulo `sub_frame.rs`), P630-esclarecimento (mapa confirmado antes de avançar).

---

## Contexto

`boxed.rs` tem uma característica que os outros consumidores de `layout_sub_frame` não têm: é **inline**. Uma `#box()` não começa uma linha nova — continua na posição horizontal onde o texto do pai já ia, e o conteúdo dentro da caixa é isolado da `current_line` do pai antes de ser movido para o sítio certo. Isto obriga `boxed.rs` a fazer o seu próprio save/restore de estado à volta da chamada a `layout_content`, e a chamar `align_current_line_rtl()` manualmente depois, em vez de deixar o helper tratar disso.

---

## Sonda mínima

### Confirmar o código actual de `boxed.rs`

```bash
grep -n "align_current_line_rtl\|current_line\|layout_content" 01_core/src/rules/layout/boxed.rs
```

Confirmar exactamente que save/restore está a ser feito à mão, para desenhar a variante do helper que o substitua sem mudar o resultado.

### Critério de fecho da sonda

- [ ] Código actual de `boxed.rs` mapeado, linha a linha, na parte relevante.
- [ ] Confirmado que a variante proposta (abaixo) cobre exactamente o mesmo comportamento.

---

## Implementação

### Sub-passo A — sem mudança de comportamento

Não aplicável da mesma forma que P629 (não há um "mover primeiro" aqui, porque não existe ainda um helper inline a mover) — mas manter a mesma disciplina de verificação apertada: implementar a variante, testar exaustivamente antes de trocar `boxed.rs` para a usar.

### Adicionar `layout_sub_frame_inline` a `sub_frame.rs`

```rust
// Esboço, a confirmar contra a estrutura real:
pub(super) fn layout_sub_frame_inline(
    &mut self,
    content: &Content,
    region: SubLayoutRegion,
) -> (f64, Vec<FrameItem>) {
    // Isola current_line do pai antes de chamar layout_content,
    // corre o sub-layout, isola os itens produzidos,
    // aplica align_current_line_rtl() se region.align_rtl,
    // restaura current_line do pai, devolve os itens isolados
    // para o caller mover para onde precisar.
}
```

### `boxed.rs` migrado

Substituir o save/restore manual e a chamada isolada a `align_current_line_rtl()` pela nova variante.

### Critério de fecho da implementação

- [ ] `layout_sub_frame_inline` criada em `sub_frame.rs`.
- [ ] `boxed.rs` migrado para a usar.
- [ ] Código manual de save/restore removido de `boxed.rs`, não deixado como código morto ao lado.

---

## Validação

```bash
cargo test -p typst-core boxed
cargo test -p typst-core p625
cargo test -p typst-core p624
cargo test --workspace
crystalline-lint .
```

**Zero mudanças de snapshot** é o resultado esperado. Se houver alguma, seguir o mesmo protocolo de P629 — investigar antes de aceitar, não regenerar automaticamente.

Repetir especificamente o teste visual de P624/P625 (`#box(width: 150pt)` com texto árabe) para confirmar que o alinhamento continua correcto depois da migração:

```bash
cat > /tmp/p631-box-arabe.typ <<'EOF'
#box(width: 150pt)[
  #set text(lang: "ar", dir: rtl, size: 30pt)
  الكتاب على الطاولة
]
EOF
./target/release/typst /tmp/p631-box-arabe.typ /tmp/p631.pdf
mutool draw -o /tmp/p631.png -r 150 /tmp/p631.pdf
```

Comparar com a imagem já obtida em P625.

---

## Critério de fecho do passo

- [ ] Sonda mínima completa.
- [ ] `layout_sub_frame_inline` criada e testada isoladamente antes de `boxed.rs` a usar.
- [ ] `boxed.rs` migrado, código duplicado removido.
- [ ] Teste visual de RTL em box confirmado, igual ao resultado de P625.
- [ ] Zero mudanças de snapshot inesperadas.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p631.md`, com hash do commit.
- [ ] Prompt L0 actualizado com a nova variante.

---

## Próximo passo

Conforme o plano de P628: avaliação de `columns.rs` — decidir se merece um helper de nível superior próprio, ou se fica fora da unificação por ter responsabilidades genuinamente diferentes (particionamento e ordenação regional, não só colocação de texto).
