from pathlib import Path
import json,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/place-origem-local-2026-10-14')
p=Path('00_nucleo/prompts/compiler/layout/tests.md')
# Owner path comes from the source header, not a guessed alias.
import re
p=Path(re.search(r'^//! @prompt (.+)$',Path('01_core/src/compiler/layout/tests.rs').read_text(),re.M).group(1))
s=p.read_text()+'''
### Retificação de cinco expectativas de baseline — medição antes da decisão

`layout-attempt3.log` conserva cinco falhas, todas com diferença de top-edge7.7pt em FixedMetrics. P898 Place raiz (tests.rs:20900–21010) já inclui `+top` ao medir FrameItem::Text.pos, mas P908 aninhado usa a fórmula do topo do frame sem a baseline interna. `baseline-expanded/results.json.gz` e `candidate-expanded/results.json.gz` reproduzem fontes anteriores exatas, restauram o candidato por hash e comparam o pin: Align bottom auto passa bbox de y12.75 para20pt, igual ao vanilla; candidatos com dy=-5 tornam o marcador visível para comparação. Align horizon após bloco alto passa y212.75 para220pt, mas vanilla tem233.25pt: ainda falta o espaçamento de bloco13.2pt, pendência distinta, não fechada por esta retificação. Os casos mistos também têm diferença residual registrada. Fonte upstream distribui o frame pela altura e conserva a baseline interna; não igualar baseline de texto ao topo geométrico.

Corrigir P898 Align bottom/horizon e P908 Place Align/Align duplo/Transform para medir a baseline como topo + text top-edge, preservando altura final e testes de finitude/dy. P908 mantém seu controle interno de scope Parent não flutuante criado diretamente em Rust; isso não autoriza tal construção pela linguagem, que continua rejeitada. Não declarar paridade universal de Rotate/Align. Adicionar teste de restauração de origem entre subframes e checkpoint; os testes de geometria existentes seguem sem relaxamento.
''';p.write_text(s)
p=Path('01_core/src/compiler/layout/tests.rs');s=p.read_text()
old='''        (y - margin).abs() < 0.01,
        "único bloco bottom-aligned deve ficar na margem: y={:.4} margin={:.4}",''';new='''        (y - margin - FixedMetrics.text_edges(style.size, &style).0 .0).abs() < 0.01,
        "topo do texto bottom-aligned deve ficar na margem; y mede a baseline: y={:.4} margin={:.4}",''';assert s.count(old)==1;s=s.replace(old,new)
old='''        (short_y - cursor_after_tall).abs() < 0.01,
        "'short' (horizon-aligned, último do fluxo) deve ficar logo após o bloco alto: y={:.4} esperado={:.4}",''';new='''        (short_y - cursor_after_tall
            - FixedMetrics.text_edges(short_style.size, short_style).0 .0).abs() < 0.01,
        "topo de 'short' deve seguir o bloco; y mede a baseline: y={:.4} topo esperado={:.4}",''';assert s.count(old)==1;s=s.replace(old,new)
old='    let expected_y = page.height - margin - content_h + dy;';assert s.count(old)==3;s=s.replace(old,'    // FrameItem::Text.pos is the baseline, not the aligned frame top.\n    let expected_y = page.height - margin - content_h + dy + top.val();')
idx=s.rfind('\n}')
s=s[:idx]+'''
    #[test]
    fn place_origin_nested_frames_and_checkpoint_restore_parent_reference() {
        use comemo::Track;
        let intr = crate::entities::introspector::TagIntrospector::empty();
        let mut layouter = Layouter::new(
            FixedMetrics,
            crate::entities::image_sizer::NullImageSizer,
            11.0,
            (&intr as &dyn crate::entities::introspector::Introspector).track(),
        );
        layouter.is_sub_frame = true;
        layouter.sub_frame_origin_y = 13.0;
        let checkpoint = layouter.capture_flow_suffix_checkpoint();
        let region = super::super::sub_frame::SubLayoutRegion {
            origin_x: 0.0, width: 60.0, height: Some(40.0),
            align_rtl: false, unconstrained_height: false,
        };
        let (_, items, _, _, _) = layouter.layout_sub_frame_at_geometric_origin(
            &Content::text("child"), region,
        );
        assert!(!items.is_empty());
        assert_eq!(layouter.sub_frame_origin_y, 13.0);
        layouter.sub_frame_origin_y = 29.0;
        layouter.restore_flow_suffix_checkpoint(checkpoint);
        assert_eq!(layouter.sub_frame_origin_y, 13.0);
        assert!(layouter.is_sub_frame);
    }
'''+s[idx:];p.write_text(s)
# Clarify that all returned body coordinates, including fixups, use geometric zero.
p=Path('01_core/src/compiler/layout/placement.rs');s=p.read_text();start=s.index('        // **Distinção física vs lógica (eixo Y)**',s.index('pub(super) fn layout_place'));end=s.index('        let insertion_base',start);s=s[:start]+'''        // Body items and orphaned fixups share geometric zero. Translate both
        // by the same target_y; no font ascender is embedded in that origin.
'''+s[end:];p.write_text(s)
