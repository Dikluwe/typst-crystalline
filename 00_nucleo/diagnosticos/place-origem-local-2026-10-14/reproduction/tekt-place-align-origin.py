from pathlib import Path
import gzip,json,datetime,hashlib
out=Path('00_nucleo/diagnosticos/place-origem-local-2026-10-14')
p=Path('00_nucleo/prompts/compiler/layout/placement.md');s=p.read_text();s+='''
### Controle Align: compensação acidental identificada antes da correção

`green-attempt1.log` registra a primeira implementação: os três testes vermelhos passam, mas Align(Box(Place(shape))) desloca y15 para6.2 em FixedMetrics (origem de ascender8.8). `placement.rs` compõe Align legado retirando sub_origin_y; `boxed.rs:467–477` e `sub_frame.rs:300–320` já colocam a caixa geométrica na origem zero após acomodar o ascent da linha. Antes, o strut indevido de Place compensava essa subtração. O controle bilateral `before/results.json.gz` estabelece y15 para este caso no pin; não autoriza preservar erros que se cancelam.

Compor também o body de Align em origem geométrica, sem subtrair ascender. Manter decisões flow_aligned (largura preenchida, gaps de parágrafo), alinhamentos, recomposição e fixups. Texto deve instalar a própria baseline. Aceitação inclui controles de Align com texto, formas, caixas, células, altura automática e suíte completa; qualquer regressão exige medição, não reescrever expectativas para o resultado novo. Isso remove convenção interna de origem para esse caller, sem mudar API pública ou fase.
''';p.write_text(s)
p=Path('01_core/src/compiler/layout/placement.rs');s=p.read_text();old='''        let (sub_h, sub_items, sub_deco, orphaned_align_x, orphaned_align_y) =
            if flow_aligned {
                self.layout_sub_frame_at_geometric_origin(body, region)
            } else {
                self.layout_sub_frame(body, region)
            };
        self.explicit_line_alignment = saved_alignment;
        let sub_origin_y = if flow_aligned {
            0.0
        } else {
            self.metrics.vertical_metrics(self.style.size, &self.style).0 .0
        };''';assert old in s;s=s.replace(old,'''        let (sub_h, sub_items, sub_deco, orphaned_align_x, orphaned_align_y) =
            self.layout_sub_frame_at_geometric_origin(body, region);
        self.explicit_line_alignment = saved_alignment;
        let sub_origin_y = 0.0;''');p.write_text(s)
