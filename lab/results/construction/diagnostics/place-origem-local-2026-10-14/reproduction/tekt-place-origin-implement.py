from pathlib import Path
import json,gzip,hashlib,datetime,re
out=Path('00_nucleo/diagnosticos/place-origem-local-2026-10-14')
src='01_core/src/compiler/stdlib/layout/place.rs';owner='00_nucleo/prompts/compiler/stdlib/layout/place.md'
with gzip.open(out/'native-before.json.gz','wt') as f:json.dump({p:{'text':Path(p).read_text(),'sha256':hashlib.sha256(Path(p).read_bytes()).hexdigest()} for p in [src,owner]},f)
owners=json.loads((out/'owners.json').read_text());owners[src]=owner;(out/'owners.json').write_text(json.dumps(owners,indent=2))
p=Path(owner);s=p.read_text();i=s.index('\n',s.index('Hash do Código:'));s=s[:i+1]+'\nNúcleos Tekt:\n- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:2baf472bb540640d571ae618aa218e5f2b166839a2070f8a4a907ec65d7ded20\n'+s[i+1:]
s+='''
## Preservar omissão vertical — 2026-10-14

### Medição antes da decisão

`01_core/src/compiler/stdlib/layout/place.rs:42–48` converte omissão em `Some(Top)`. As sondas `00_nucleo/diagnosticos/place-origem-local-2026-10-14/{before,extra-before}/results.json.gz` guardam HEAD, árvore suja, diffstat, UTC, binários e SVGs contra o pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8: após texto A, alinhamento omitido ancora na baseline; Top explícito ancora na região. `lab/typst-original/crates/typst-layout/src/flow/distribute.rs:703–715` distingue o eixo explícito do offset de flow. Trata-se de geometria de linguagem, não de igualdade Rust. Os testes place_origin_pending_line_and_explicit_top_are_distinct reproduzem a divergência antes da correção.

### Decisão e aceitação

Usar o Option existente para preservar v=None quando o eixo vertical é omitido; manter o alinhamento explícito. Não adicionar campo ou alterar assinatura. Correção de paridade em fluxo contínuo ADR-0127, sem novo modo, flag ou fase. Layout decide a âncora segundo o núcleo place-coordinate-origin. Preservar clearance, scope, validação e floats; validar testes de linha pendente, Top explícito, geometria, floats e suíte completa. A etapa B de âncora após fluxo já comitado permanece aberta no diagnóstico de origem local.
''';p.write_text(s)
def edit(path,old,new,count=1):
 p=Path(path);s=p.read_text();assert s.count(old)==count,(path,old,s.count(old));p.write_text(s.replace(old,new))
base='01_core/src/compiler/layout/'
edit(base+'mod.rs','    pub(super) is_sub_frame: bool,','    pub(super) is_sub_frame: bool,\n    /// Vertical origin expected when recomposing this sub-frame into its parent.\n    pub(super) sub_frame_origin_y: f64,')
edit(base+'mod.rs','            is_sub_frame: false,','            is_sub_frame: false,\n            sub_frame_origin_y: 0.0,')
edit(base+'sub_frame.rs','        let saved_is_sub_frame = self.is_sub_frame;','        let saved_is_sub_frame = self.is_sub_frame;\n        let saved_sub_frame_origin_y = self.sub_frame_origin_y;')
edit(base+'sub_frame.rs','        let start_y = self.regions.current.cursor_y.0;','        let start_y = self.regions.current.cursor_y.0;\n        self.sub_frame_origin_y = start_y;')
edit(base+'sub_frame.rs','        self.is_sub_frame = saved_is_sub_frame;','        self.is_sub_frame = saved_is_sub_frame;\n        self.sub_frame_origin_y = saved_sub_frame_origin_y;')
edit(base+'checkpoint.rs','    is_sub_frame: bool,','    is_sub_frame: bool,\n    sub_frame_origin_y: f64,')
edit(base+'checkpoint.rs','            is_sub_frame: self.is_sub_frame,','            is_sub_frame: self.is_sub_frame,\n            sub_frame_origin_y: self.sub_frame_origin_y,')
edit(base+'checkpoint.rs','        self.is_sub_frame = checkpoint.is_sub_frame;','        self.is_sub_frame = checkpoint.is_sub_frame;\n        self.sub_frame_origin_y = checkpoint.sub_frame_origin_y;')
p=Path(base+'placement.rs');s=p.read_text();a=s.index('        let geometric_origin = !(');b=s.index('        let region = ',a);s=s[:a]+s[b:];a=s.index('            if geometric_origin {',a);b=s.index('\n        // **P904',a);s=s[:a]+'            self.layout_sub_frame_at_geometric_origin(body, region);\n'+s[b:];a=s.index('        let in_sub_frame = self.is_sub_frame;',a);b=s.index('\n        // **P897',a);section=s[a:b].replace('(cx, cy, cell.width, cell.height, cy)','(cx, cy, cell.width, cell.height, 0.0)').replace('(0.0, 0.0, avail_w_page, avail_h_page, 0.0)','(0.0, self.sub_frame_origin_y, avail_w_page, avail_h_page, 0.0)').replace('sub_origin_y,','0.0,');s=s[:a]+section+s[b:];p.write_text(s)
p=Path(base+'place.rs');s=p.read_text();a=s.index('        // P1292 v12');b=s.index('        let item_start = ',a);s=s[:a]+'''        // Omitted vertical alignment follows the pending line baseline. The
        // geometric body supplies its own text ascent; explicit Top stays at
        // the region origin. Preserve the parent's recomposition convention.
        let follows_pending_line = effective_alignment.v.is_none()
            && super::html_frame::items_have_material(
                &layouter.regions.current.current_line,
            );
        let in_sub_frame = layouter.is_sub_frame;
        let pending_line_delta = follows_pending_line.then(|| {
            layouter.regions.current.cursor_y.0 - layouter.sub_frame_origin_y
        });
        let saved_margin_top = layouter.page_config.margin.top;
        if !in_sub_frame && follows_pending_line {
            layouter.page_config.margin.top = layouter.regions.current.cursor_y.0;
        }

'''+s[b:];s=s.replace('            if let Some(top_edge) = semantic_top_edge {\n                let delta = top_edge.0;','            if let Some(delta) = pending_line_delta {');p.write_text(s)
edit(src,'    // Default "top-left" para Place, "left" (default Align2D) para Align.','    // Preserve an omitted vertical axis: it follows flow, unlike explicit Top.')
edit(src,'            v: Some(crate::entities::layout_types::VAlign::Top),','            v: None,')
