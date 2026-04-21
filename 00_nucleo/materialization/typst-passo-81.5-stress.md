# Passo 81.5 — Stress de Composição Geométrica (Gate para Passo 82)

## Propósito

Este passo não introduz funcionalidade nova. É um gate de qualidade que
valida a integridade espacial da composição de três sistemas implementados
nos Passos 80–81:

- **Grid** (Passo 80) — resolução de colunas com `available_width()`.
- **Transform** (Passo 78) — grupos com matriz `cm` e espaço local.
- **PageConfig dinâmico** (Passo 81) — snapshots imutáveis e `SetPage`.

O perigo não é um crash. É um **vazamento de contexto geométrico silencioso**:
o Grid calcula `available_width` com as dimensões da página A4 em vez da
página activa, ou uma Transform inverte Y assumindo 841.89pt quando a página
só tem 300pt. O resultado é um PDF válido mas visualmente errado.

**O Passo 82 só começa quando todos os critérios deste passo estiverem verdes.**

---

## Pré-condição

`cargo test` — todos os testes dos Passos 80 e 81 a passar, zero violations.
`doc.pages[0].width == 595.28`, `doc.pages[1].width == 400.0`,
`doc.pages[2].width == 200.0` conforme o Passo 81.

---

## Ficheiro de teste de stress

Criar `tests/stress_81_5.typ` (ou equivalente na estrutura de testes L3):

```typst
// Página 1: A4 padrão (595.28 × 841.89) — nunca alterada explicitamente
Texto introdutório na primeira página.

#set page(width: 400pt, height: 300pt, margin: 20pt)

// Página 2: Layout customizado com Grid + Transform aninhados
#grid(
  columns: (1fr, 2fr),
  [
    #transform(translate(5pt, 10pt))[A]
  ],
  [
    Este texto deve quebrar linha porque a célula 2fr tem largura limitada.
  ],
  [
    #rect(width: 100pt, height: 50pt)
  ],
  [
    #rect(width: 80pt, height: 30pt)
  ],
)

#set page(width: 200pt, height: 200pt, margin: 5pt)

// Página 3: Página quadrada mínima
Fim.
```

---

## Testes a implementar em `03_infra/tests/integration_tests.rs`

### Fase 1 — Invariantes de estado (macro)

```rust
#[test]
fn stress_81_5_tres_paginas_com_snapshots_correctos() {
    let doc = compilar_stress_81_5(); // helper que compila o ficheiro acima

    assert_eq!(doc.pages.len(), 3,
        "SetPage deve criar exactamente 2 quebras de página");

    // Página 1: A4 padrão
    assert!((doc.pages[0].width  - 595.28).abs() < 0.01,
        "Página 1 deve preservar A4 width (595.28pt)");
    assert!((doc.pages[0].height - 841.89).abs() < 0.01,
        "Página 1 deve preservar A4 height (841.89pt)");

    // Página 2: 400×300pt
    assert_eq!(doc.pages[1].width,  400.0,
        "Página 2 deve ter width = 400pt do SetPage");
    assert_eq!(doc.pages[1].height, 300.0,
        "Página 2 deve ter height = 300pt do SetPage");

    // Página 3: 200×200pt
    assert_eq!(doc.pages[2].width,  200.0);
    assert_eq!(doc.pages[2].height, 200.0);
}
```

### Fase 2 — Geometria do Grid na página 2

```rust
#[test]
fn stress_81_5_grid_usa_available_width_da_pagina_activa() {
    let doc = compilar_stress_81_5();
    let items = &doc.pages[1].items;

    // available_width = 400 - 2*20 = 360pt; total_fr = 3.
    // Col 0 (1fr) = 120pt; Col 1 (2fr) = 240pt.
    let margin = 20.0;

    // O item "A" vive dentro de uma Transform(translate(5, 10))
    // dentro da célula 0,0 do grid.
    // Posição X absoluta: margin + offset_célula(0) + translate_x = 20 + 0 + 5 = 25pt.
    // Posição Y absoluta: margin + offset_linha(0) + translate_y = 20 + 0 + 10 = 30pt.
    let item_a = items.iter()
        .find(|(_, item)| matches!(item, FrameItem::Text(t) if t.text.contains("A")))
        .expect("Item 'A' deve existir na página 2");

    let expected_x = margin + 5.0; // 25pt
    let expected_y = margin + 10.0; // 30pt

    assert!((item_a.0.x.0 - expected_x).abs() < 0.5,
        "Transform dentro de Grid deve respeitar coordenadas da página 2. \
         Esperado x={:.1}, obtido x={:.1}", expected_x, item_a.0.x.0);

    assert!((item_a.0.y.0 - expected_y).abs() < 0.5,
        "Translate Y deve ser relativo ao topo da página 2 (300pt), não A4. \
         Esperado y={:.1}, obtido y={:.1}", expected_y, item_a.0.y.0);
}
```

### Fase 3 — Row height avança cursor correctamente

```rust
#[test]
fn stress_81_5_row_height_e_maximo_da_linha() {
    let doc = compilar_stress_81_5();
    let items = &doc.pages[1].items;

    // O rect de 50pt está na célula (0,1) — segunda linha, primeira coluna.
    // Deve estar abaixo do item "A" (que está na linha 0).
    let rect_50 = items.iter()
        .find(|(_, item)| {
            matches!(item, FrameItem::Shape { height, .. } if (*height - 50.0).abs() < 0.1)
        })
        .expect("Rect de 50pt deve existir na página 2");

    let item_a = items.iter()
        .find(|(_, item)| matches!(item, FrameItem::Text(t) if t.text.contains("A")))
        .expect("Item 'A' deve existir na página 2");

    assert!(rect_50.0.y.0 > item_a.0.y.0,
        "Linha 1 do grid deve estar abaixo da linha 0. \
         rect y={:.1}, item_a y={:.1}", rect_50.0.y.0, item_a.0.y.0);
}
```

### Fase 4 — Anti-regressão: nenhum item excede os limites da página 2

```rust
#[test]
fn stress_81_5_nenhum_item_excede_limites_da_pagina_2() {
    // Esta é a asserção mais importante do passo.
    // Se qualquer cálculo de margem, grid ou transform usou a constante A4,
    // algum item terá coordenadas fora dos limites físicos da página 2.
    let doc = compilar_stress_81_5();
    let items = &doc.pages[1].items;

    for (pos, item) in items {
        assert!(pos.x.0 <= 400.0,
            "Item {:?} excede largura da página 2 (400pt): x={:.1}",
            item, pos.x.0);
        assert!(pos.y.0 <= 300.0,
            "Item {:?} excede altura da página 2 (300pt): y={:.1}. \
             Possível causa: inversão Y usou 841.89pt (A4) em vez de 300pt.",
            item, pos.y.0);

        // Verificar recursivamente itens dentro de Groups (Transforms).
        if let FrameItem::Group { items: sub_items, .. } = item {
            for (sub_pos, _) in sub_items {
                let abs_x = pos.x.0 + sub_pos.x.0;
                let abs_y = pos.y.0 + sub_pos.y.0;
                assert!(abs_x <= 400.0,
                    "Item transformado excede largura da página 2: abs_x={:.1}", abs_x);
                assert!(abs_y <= 300.0,
                    "Item transformado excede altura da página 2: abs_y={:.1}. \
                     Possível causa: Transform usou page_height global em vez de snapshot.",
                    abs_y);
            }
        }
    }
}
```

### Fase 5 — PDF tem três MediaBox distintos e correctos

```rust
#[test]
fn stress_81_5_pdf_tem_tres_mediabox_distintos() {
    let doc = compilar_stress_81_5();
    let pdf = export_pdf(&doc);
    let pdf_str = String::from_utf8_lossy(&pdf);

    assert!(pdf_str.contains("[0 0 595.28 841.89]"),
        "PDF: página 1 deve ter MediaBox A4");
    assert!(pdf_str.contains("[0 0 400.00 300.00]"),
        "PDF: página 2 deve ter MediaBox 400×300pt");
    assert!(pdf_str.contains("[0 0 200.00 200.00]"),
        "PDF: página 3 deve ter MediaBox 200×200pt");

    // Verificar que não existe um MediaBox híbrido — sinal de vazamento catastrófico.
    // Ex: [0 0 400.00 841.89] significa que a largura foi actualizada mas a altura ficou A4.
    assert!(!pdf_str.contains("[0 0 400.00 841.89]"),
        "PDF: MediaBox híbrido detectado — height da página 2 vazou para A4");
    assert!(!pdf_str.contains("[0 0 595.28 300.00]"),
        "PDF: MediaBox híbrido detectado — width da página 2 ficou em A4");

    // Contagem exacta: exactamente 3 MediaBox.
    let count = pdf_str.matches("/MediaBox").count();
    assert_eq!(count, 3,
        "PDF deve ter exactamente 3 /MediaBox, encontrou {}", count);
}
```

---

## Tabela de invariantes

| Invariante | Teste que a cobre | Falha típica |
|---|---|---|
| Isolamento de width | Fase 2 — posição X do item A | `available_width()` ainda lê constante A4 |
| Isolamento de height | Fase 4 — nenhum item com Y > 300pt | `new_page()` ou Transform usam `page_height` global |
| Snapshot de Page | Fase 1 — `pages[1].width == 400` depois de mudar para 200 | `Page` guarda referência em vez de valor |
| Inversão Y no PDF | Fase 5 — sem MediaBox híbrido | Exportador usa variável global `PAGE_HEIGHT = 841.89` |
| Reset de cursor | Fase 1 implícita — página 3 tem conteúdo | `new_page()` não resetou cursor |
| Transform local | Fase 2 — Translate(5, 10) resulta em (25, 30) global | Transform aplica offset antes do merge do Grid |

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de aprovação — todos obrigatórios antes de avançar para o Passo 82:
- [ ] `stress_81_5_tres_paginas_com_snapshots_correctos` passa.
- [ ] `stress_81_5_grid_usa_available_width_da_pagina_activa` passa.
- [ ] `stress_81_5_row_height_e_maximo_da_linha` passa.
- [ ] `stress_81_5_nenhum_item_excede_limites_da_pagina_2` passa — **este
  é o critério mais importante: zero itens fora dos limites físicos**.
- [ ] `stress_81_5_pdf_tem_tres_mediabox_distintos` passa.
- [ ] Nenhum MediaBox híbrido no PDF.
- [ ] Zero violações no linter e no clippy.
- [ ] Contagem total de testes ≥ 884 (indicativo do Passo 81).

---

## Ao terminar, reportar

- Quais fases falharam na primeira execução e qual foi a causa — documenta
  os vazamentos de contexto encontrados para referência futura.
- Se a Fase 4 encontrou itens fora dos limites: indicar qual FrameItem e
  qual coordenada, para rastrear o local exacto do vazamento.
- Número total de testes após o passo e zero violations confirmados.

**Go para o Passo 82** quando todos os critérios estiverem verdes e o
relatório confirmar zero vazamentos de contexto geométrico.
