from pathlib import Path
import json,gzip,hashlib,base64,datetime,subprocess
base=Path('00_nucleo/diagnosticos/transformacoes-2026-10-14/png-afim');base.mkdir(exist_ok=False)
paths=['03_infra/src/export/render.rs','00_nucleo/prompts/infra/export/render.md']
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest(),base64=base64.b64encode(Path(p).read_bytes()).decode()) for p in paths})
with gzip.open(base/'before.json.gz','wt') as f:json.dump(r,f)
p=Path(paths[1]);p.write_text(p.read_text()+'''
## Contorno de glifos sob matriz afim — correção medida

### Evidência anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/refuters-third/results.json.gz`
registra fontes, comandos, HEAD, diffstat/UTC e hashes dos binários. O controle
reflow-wrap-rotate coincide em SVG, mas diverge no PNG; PDF foi observado também
em refuters-pdf-first. `03_infra/src/export/render.rs:402–405` calcula ppem com sy
e usa apenas sx/sy para a origem; kx/ky são ignorados e a forma permanece uniforme.
Fonte pinada `typst-render/src/text.rs:53–91` rasteriza contornos com a matriz
completa quando a rasterização uniforme não representa a transformação.
Isso distingue posição/forma da linguagem de bytes de PNG e estratégia de cache.
A falha é interna, dentro do transporte já aprovado; não cria API nem muda fase.

### Responsabilidade exclusiva

Glifos TextShaped já suportados mantêm avanço, offsets, fonte preparada e alpha.
Quando a matriz inclui rotação, shear, escala não uniforme ou reflexo, rasterizar
o contorno da mesma face com a matriz completa: transformação do grupo, origem
do glifo, escala de unidades da fonte e inversão do eixo vertical, nessa ordem.
Conservar a máscara acumulada em coordenadas do dispositivo. Caminho uniforme
positivo pode continuar pixglyph; tamanhos grandes podem usar contorno. Fechar
contornos abertos inclusive CFF2 antes da rasterização. Glifo sem contorno não
fabrica tinta; esta correção não certifica fontes coloridas/bitmap, Glyph direto,
Text sem shaping ou suporte integral de matemática.

### Refutadores

Glifo assimétrico com rotação de 90 graus, escala não uniforme, shear e reflexão:
comparar posição e distribuição de cobertura com a transformação geométrica da
mesma figura. Máscara deve recortar também esse caminho sem afetar irmãos.
Controles uniformes preservam o comportamento anterior; corpo fonte→PNG deve
acompanhar a geometria medida em SVG/PDF e a referência pinada, sem realinhamento.
''')
p=Path(paths[0]);p.write_text(p.read_text()+'''

#[cfg(test)]
mod affine_glyph_regressions {
    use super::*;

    fn draw(ts: sk::Transform, mask: Option<&sk::Mask>) -> sk::Pixmap {
        let bytes = include_bytes!("../../fixtures/fonts/NotoSans-Regular.ttf");
        let face = ttf_parser::Face::parse(bytes, 0).unwrap();
        let mut canvas = sk::Pixmap::new(180,180).unwrap();
        render_glyph(&mut canvas, State { transform: ts, mask, ..State::new(1.0) },
            &face, face.glyph_index('L').unwrap(), 30.0,50.0,30.0,
            sk::Color::from_rgba8(20,40,80,160));
        canvas
    }

    fn centroid(canvas: &sk::Pixmap) -> (f64,f64) {
        let (mut mass,mut x,mut y)=(0.0,0.0,0.0);
        for py in 0..canvas.height() {
            for px in 0..canvas.width() {
                let alpha=canvas.pixel(px,py).unwrap().alpha() as f64;
                mass+=alpha;x+=(px as f64+0.5)*alpha;y+=(py as f64+0.5)*alpha;
            }
        }
        assert!(mass>0.0,"transformed glyph must retain its ink");
        (x/mass,y/mass)
    }

    #[test]
    fn affine_glyph_preserves_ink_position_under_rotation_scale_and_reflection() {
        let (x,y)=centroid(&draw(sk::Transform::identity(),None));
        for ts in [
            sk::Transform::from_row(0.0,1.0,-1.0,0.0,140.0,0.0),
            sk::Transform::from_scale(2.0,1.0),
            sk::Transform::from_row(1.0,0.0,0.4,1.0,0.0,0.0),
            sk::Transform::from_row(1.0,0.0,0.0,-1.0,0.0,140.0),
        ] {
            let (actual_x,actual_y)=centroid(&draw(ts,None));
            let expected_x=x*ts.sx as f64+y*ts.kx as f64+ts.tx as f64;
            let expected_y=x*ts.ky as f64+y*ts.sy as f64+ts.ty as f64;
            assert!((actual_x-expected_x).abs()<1.0 && (actual_y-expected_y).abs()<1.0,
                "{ts:?}: ({actual_x},{actual_y}) vs ({expected_x},{expected_y})");
        }
    }

    #[test]
    fn affine_glyph_outline_obeys_device_clip_and_keeps_alpha() {
        let ts=sk::Transform::from_row(0.0,1.0,-1.0,0.0,140.0,0.0);
        let plain=draw(ts,None);
        let mut mask=sk::Mask::new(180,180).unwrap();
        for y in 0..180 { for x in 0..100 { mask.data_mut()[y*180+x]=255; } }
        let clipped=draw(ts,Some(&mask));
        let mut retained=false;let mut removed=false;
        for y in 0..180 { for x in 0..180 {
            let before=plain.pixel(x,y).unwrap();let after=clipped.pixel(x,y).unwrap();
            if x<100 { assert_eq!(before,after);retained|=after.alpha()>0; }
            else { assert_eq!(after.alpha(),0);removed|=before.alpha()>0; }
            assert!(after.alpha()<=160);
        } }
        assert!(retained && removed,"the clip must cut actual transformed ink");
    }
}
''')
