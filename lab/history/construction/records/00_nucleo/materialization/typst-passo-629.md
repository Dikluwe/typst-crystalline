---
# P629 — Refactor do helper existente para `SubLayoutRegion`

> **Passo:** 629
> **Data:** 2026-07-09
> **Foco:** P628 propôs esta divisão como o primeiro passo, o mais seguro: transformar `layout_sub_frame_with_width` numa função que recebe uma estrutura `SubLayoutRegion` explícita, sem mudar nenhum comportamento. Afecta `grid.rs` e `placement.rs`, os dois caminhos que já usam o helper.
> **Tipo:** Refactor. Sem mudança de comportamento pretendida — qualquer diferença de output é regressão, não intencional.
> **Tamanho:** S–M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P628 (sonda e plano), P579/580 (onde `layout_sub_frame_with_width` foi criado).

---

## Implementação

### Módulo novo

Criar `01_core/src/engine/layout/sub_frame.rs`, seguindo a convenção já estabelecida no projecto de um ficheiro por conceito de layout (`grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs`). Deixar isto dentro de `mod.rs` iria contra essa convenção e tornaria o ficheiro, já grande, ainda maior.

### Duas mudanças separadas, não uma só, para manter a verificação clara

Este refactor mistura duas coisas que, se feitas ao mesmo tempo, tornam mais difícil confirmar "zero mudança de comportamento" — mover código de sítio produz um `diff` grande por si só, que pode esconder uma alteração real por engano no meio do ruído do movimento.

**Sub-passo A — só mover, sem alterar nada:**

Mover `layout_sub_frame_with_width` tal como está, byte a byte, para `sub_frame.rs`. Sem renomear, sem mudar assinatura, sem tocar na lógica interna. Confirmar `cargo test --workspace` e zero mudanças de snapshot antes de continuar para o sub-passo B.

**Sub-passo B — só depois, trocar a assinatura:**

Com o código já no novo ficheiro, e só depois de confirmado que a mudança de sítio não alterou nada, introduzir `SubLayoutRegion`:

```rust
pub(super) struct SubLayoutRegion {
    pub origin_x: f64,
    pub width: f64,
    pub height: Option<f64>,
    pub align_rtl: bool,
    pub unconstrained_height: bool,
}
```

Renomear a função para `layout_sub_frame(content: &Content, region: SubLayoutRegion) -> (f64, Vec<FrameItem>)`, mantendo a mesma lógica interna.

### Chamadas actualizadas

- `grid.rs`: construir `SubLayoutRegion` com os parâmetros já usados hoje (`align_rtl: true`), chamando através do módulo novo.
- `placement.rs`: construir `SubLayoutRegion` com os parâmetros já usados hoje (`align_rtl: false`, `unconstrained_height: true`), chamando através do módulo novo.

### Critério de fecho da implementação

- [ ] `layout_sub_frame` criada, com a mesma lógica interna, sem alteração de comportamento.
- [ ] `grid.rs` e `placement.rs` actualizados para a nova assinatura.
- [ ] Nenhuma função antiga (`layout_sub_frame_with_width`) deixada por trás sem uso — remover se ficar órfã.

---

## Validação

Esta é a parte mais importante deste passo, dado que é um refactor sem mudança de comportamento pretendida.

```bash
cargo test --workspace
```

Nenhum teste deve mudar de resultado. Se algum snapshot P307b mudar, isso é sinal de que o refactor introduziu uma diferença não intencional — investigar antes de regenerar o snapshot, não regenerar automaticamente como se fosse esperado.

```bash
crystalline-lint .
```

Repetir os testes específicos de grid e placement já existentes (incluindo os de RTL de P624/P625):

```bash
cargo test -p typst-core grid
cargo test -p typst-core placement
cargo test -p typst-core p624
cargo test -p typst-core p625
```

---

## Critério de fecho do passo

- [ ] Módulo `sub_frame.rs` criado, seguindo a convenção de um ficheiro por conceito.
- [ ] Sub-passo A (só mover) confirmado com testes limpos, antes de avançar para o sub-passo B.
- [ ] Sub-passo B (`SubLayoutRegion` e `layout_sub_frame`) implementado depois, não ao mesmo tempo.
- [ ] `grid.rs` e `placement.rs` migrados.
- [ ] **Zero mudanças de snapshot** em qualquer um dos dois sub-passos — se houver alguma, investigada antes de aceitar.
- [ ] Testes específicos de grid, placement, P624, P625 sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p629.md`, com hash do commit de cada sub-passo, não só do fecho.
- [ ] Prompt L0 correspondente actualizado com a nova interface e a localização no módulo novo.
