# P621 — `tracking` com árabe e devanágari

**Data da medição:** 2026-07-09  
**Commit base:** `eabb7b762308f3b03b1c84593157de0a1b85330d`  
**Passo:** P621  
**ADR-0108 em vigor:** medição precede a decisão.

## Resumo executivo

Implementou-se o aplicamento de `text.tracking` nos avanços reais dos glifos durante o shaping (`03_infra/src/shaper.rs`). A correção é no shaper, não no export PDF: aplicar tracking via operador PDF `Tc` desalinha o layout RTL, porque `fix_line_positions_page` redistribui as posições usando `width_real` calculado a partir dos `x_advance` dos glifos.

A medição visual confirma que o tracking é aplicado em árabe e devanágari, mas a paridade visual fica limitada por uma divergência pré-existente: quebras de parágrafo não são respeitadas no cristalino para RTL, pelo que as duas linhas de teste são renderizadas numa única linha visual.

## Decisão técnica

### Por que no shaper, não no export

- O layout reserva largura via `FontMetrics::text_width`, que adiciona `tracking × (n_chars − 1)` à largura base (`01_core/src/engine/layout/metrics.rs:91-104`).
- `fix_line_positions_page` (`03_infra/src/shaper.rs`) redistribui posições x dentro de cada linha usando a largura real dos runs (`width_real − width_est`).
- Se o tracking só estiver no PDF (`Tc`), `width_real` não o inclui, e o layout RTL/LTR fica desalinhado face à largura reservada.
- Solução: aumentar `x_advance` de cada glifo no shaper, para que `width_real` já inclua tracking.

### Como é aplicado

```rust
let tracking_fu = style.tracking
    .map(|t| {
        let pt = t.resolve_pt(style.size.val());
        (pt * candidate.units_per_em as f64 / style.size.val()).round() as i32
    })
    .unwrap_or(0);
```

Para cada glifo, o tracking é adicionado ao `x_advance` apenas quando o glifo seguinte pertence a um cluster de caracteres diferente. Isto evita partir ligaduras/conjuntos em scripts como o devanágari. O último glifo de cada sub-run nunca recebe tracking.

## Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `03_infra/src/shaper.rs` | Tracking aplicado nos `x_advance` dos glifos, com cluster-aware spacing. Teste `p621_tracking_aumenta_x_advance` adicionado. |
| `00_nucleo/prompts/infra/shaper.md` | Secção §P621 documentando a decisão e fórmula. |
| `03_infra/src/export/stream.rs` | Revertida tentativa anterior de aplicar `Tc` em `emit_shaped_pdf`; testes P621 removidos deste ficheiro. |
| `00_nucleo/prompts/infra/export/stream.md` | Removida secção §P621 obsoleta sobre `Tc` no export. |
| `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md` | Tracking+árabe/devanágari movido para "Corrigido"; quebras de parágrafo RTL registadas como divergência conhecida. |

## Verificação mecânica

```bash
cargo test --workspace
```

Resultado: **605 passed; 0 failed; 5 ignored**.

```bash
crystalline-lint .
```

Resultado: **✓ No violations found**.

## Testes visuais

Documentos de teste:

```typst
// árabe
#set text(lang: "ar", font: "DejaVu Sans", size: 24pt)
مرحبا بالعالم

#set text(tracking: 5pt)
مرحبا بالعالم

// devanágari
#set text(lang: "sa", font: "Noto Sans Devanagari", size: 24pt)
नमस्ते संसार

#set text(tracking: 5pt)
नमस्ते संसार
```

Comandos usados:

```bash
lab/typst-original/target/release/typst compile doc.typ vanilla.pdf
./target/release/typst doc.typ cristalino.pdf
mutool draw -o img.png -r 150 pdf
```

### Observações

- **Árabe:** o cristalino aplica tracking visível entre os caracteres. No entanto, as duas linhas (sem e com tracking) são renderizadas na mesma linha visual, porque quebras de parágrafo não são respeitadas para RTL.
- **Devanagari:** o cristalino aplica tracking entre clusters de caracteres. A mesma limitação de quebra de parágrafo faz com que as duas linhas apareçam concatenadas.
- Teste isolado sem tracking (`p621-arabe-twolines.typ`) confirmou que a concatenação das duas linhas ocorre independentemente de tracking: é uma divergência pré-existente.

## Decisão

1. **Aplicar tracking no shaper** é a arquitetura correcta para este sistema; fechar com P621.
2. **Quebras de parágrafo em RTL** são uma divergência separada, anterior a P621, e não devem bloquear o fecho deste passo.
3. A paridade de tracking está ao nível da linguagem (avanços reais dos glifos incluem tracking); a renderização visual exacta continua dependente do layout de parágrafos.

## Proveniência da medição

- Commit base: `eabb7b762308f3b03b1c84593157de0a1b85330d`
- Binário vanilla: `lab/typst-original/target/release/typst`
- Binário cristalino: `./target/release/typst`
- Documentos temporários: `/tmp/p621-arabe-tracking.typ`, `/tmp/p621-sanscrito-tracking.typ`, `/tmp/p621-arabe-twolines.typ`
- Data/hora aproximada: 2026-07-09.
