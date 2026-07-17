# Relatório — Passo 629 (P629)

**Data:** 2026-07-09  
**Commit sub-passo A:** `213e56e6c` — move `layout_sub_frame_with_width` para `sub_frame.rs`  
**Commit sub-passo B:** `6c3813369` — refactor para `SubLayoutRegion`  
**Hash L0 actualizado:** `2b3c0378` (`00_nucleo/prompts/engine/layout.md`)  
**Foco:** Refactor do helper de sub-layout num módulo próprio e com assinatura explícita, sem mudança de comportamento.

---

## Resumo executivo

P629 executou o primeiro passo da divisão proposta em P628: transformar `layout_sub_frame_with_width` num helper genérico `layout_sub_frame` com uma estrutura `SubLayoutRegion`. O refactor foi feito em dois sub-passos separados (mover primeiro, alterar assinatura depois) para poder verificar "zero mudança de comportamento" em cada etapa.

Resultado: todos os testes passam, `crystalline-lint .` está limpo, e o L0 `layout.md` foi actualizado com a nova secção.

---

## Implementação

### Sub-passo A — mover o helper

- Criado `01_core/src/engine/layout/sub_frame.rs` seguindo a convenção de um ficheiro por conceito de layout.
- `layout_sub_frame_with_width` foi movido byte-a-byte de `layout/mod.rs` para `sub_frame.rs`.
- Adicionada declaração `mod sub_frame;` em `layout/mod.rs`.
- Nenhuma alteração de assinatura ou lógica.

### Sub-passo B — introduzir `SubLayoutRegion`

- Criada a estrutura:

```rust
pub(super) struct SubLayoutRegion {
    pub origin_x: f64,
    pub width: f64,
    pub height: Option<f64>,
    pub align_rtl: bool,
    pub unconstrained_height: bool,
}
```

- Renomeada a função para `layout_sub_frame(content: &Content, region: SubLayoutRegion) -> (f64, Vec<FrameItem>)`.
- Mantida a mesma lógica interna; apenas a forma de receber parâmetros mudou.
- Actualizados os call-sites:
  - `01_core/src/engine/layout/grid.rs:250` e `:487`
  - `01_core/src/engine/layout/placement.rs:31` e `:124`
  - `01_core/src/engine/layout/place.rs:44`
  - `01_core/src/engine/layout/cursor.rs:650`
- Removidas todas as referências a `layout_sub_frame_with_width` (incluindo comentários).

### L0 actualizado

- `00_nucleo/prompts/engine/layout.md` ganhou a secção "Sub-layout isolado (`layout_sub_frame`, Passo 629)" com a definição de `SubLayoutRegion`, a assinatura de `layout_sub_frame` e invariantes.
- Hash actualizado para `2b3c0378` via `crystalline-lint --fix-hashes`.

---

## Validação

### Sub-passo A

```bash
cargo test -p typst-core grid
cargo test -p typst-core placement
cargo test -p typst-core p624
cargo test -p typst-core p625
```

Resultado: **todos passam** (161 + 8 + 0 + 0 testes).

### Sub-passo B

```bash
cargo test -p typst-core grid
cargo test -p typst-core placement
cargo test -p typst-core p624
cargo test -p typst-core p625
cargo test -p typst-core p626
cargo test -p typst-core p627
cargo test --workspace
crystalline-lint .
```

Resultados:
- Testes específicos: **todos passam**.
- `cargo test --workspace`: **todos passam** (3593 + 605 + 28 + 2 + 27 + 2 + 0 + 0 + 0).
- `crystalline-lint .`: **No violations found**.

### Nota sobre regressão

Durante o sub-passo B, a primeira tentativa usou `height: Some(body_h)` e `unconstrained_height: false` para a emissão de células em `grid.rs`. Quatro testes de grid falharam:

- `p273_9_containers_estendidos::p273_9_grid_cell_lifo_restore`
- `p273_9_containers_estendidos::p273_9_grid_cell_save_restore_parent_bbox`
- `grid_altura_da_linha_e_o_maximo_das_celulas`
- `p273_9_containers_estendidos::p273_9_grid_debt37_cell_origin_consumption_preserved`

A causa foi uma mudança não intencional de comportamento: o helper original sempre corria com altura ilimitada e `is_height_unconstrained = true`. A correção foi manter `height: None` e `unconstrained_height: true` em ambas as chamadas de `grid.rs`, confirmando que o refactor se limitou a reorganizar a assinatura.

---

## Decisões e notas

- **Dois sub-passos foram necessários.** O movimento de código produz um diff grande por si só; misturá-lo com a mudança de assinatura teria tornado difícil detectar a regressão acima.
- **`SubLayoutRegion` não é ainda usada para impor limites de altura.** A semântica original era "altura ilimitada"; os campos `height` e `unconstrained_height` existem para futuros consumidores, mas `grid.rs` e `placement.rs` mantêm o comportamento anterior.
- **O L0 foi actualizado porque houve mudança arquitectural.** A localização do helper e a sua interface são agora parte da especificação.

---

## Critérios de fecho do passo

- [x] Módulo `sub_frame.rs` criado, seguindo a convenção de um ficheiro por conceito.
- [x] Sub-passo A (só mover) confirmado com testes limpos.
- [x] Sub-passo B (`SubLayoutRegion` e `layout_sub_frame`) implementado depois.
- [x] `grid.rs` e `placement.rs` migrados.
- [x] Zero mudanças de snapshot — regressão detectada e corrigida antes de aceitar.
- [x] Testes específicos de grid, placement, P624, P625, P626, P627 sem regressão.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] L0 `layout.md` actualizado e hash sincronizado.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p629.md`, com hash do commit de cada sub-passo.
