from pathlib import Path
import json,shutil,datetime
out=Path(__file__).resolve().parents[1];receipt=json.load(open(out/'l0-before-code.json'));parent='00_nucleo/prompts/compiler/math/layout/_comum.md';owner='00_nucleo/prompts/compiler/math/layout/source-transparency-tests.md';note='''

### Marcadores Source no grid — complemento medido em 2026-10-17

source-markers-before.json confirma: Source(Linebreak) transforma duas linhas
em uma (a/b na mesma baseline); Source(MathAlignPoint) conserva duas linhas,
mas desloca 1/2 em relação à coluna de 333/4. Preservar esse RED antes do reparo.
No owner pai, needs_grid_layout e partition_grid também classificam os marcadores
por node.source_body(), sem mudar separadores, linhas/células vazias ou ordem.
No braço de conteúdo de partition_grid, reter o clone original com sua origem;
usar complemento nominal plano se isso eliminar o binding normalizado anterior,
sem captura @ ou suppressão. A nova origem nunca fabrica Location.
O novo consumer de regressão acrescenta dois testes públicos display, para
Linebreak e MathAlignPoint Source, exigindo as mesmas baselines e posições.
Os 27 controles anteriores não mudam: o total passa a 29, com RED esperado
5 passando/24 falhando. Nenhum novo contrato, fase, default ou API pública.
'''
for p in [parent,owner]:Path(p).write_text(Path(p).read_text()+note);receipt['owners'][p]=Path(p).read_text()
receipt['marker_supplement_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(out/'l0-before-code.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n');p=Path('01_core/src/compiler/math/layout/source_transparency_tests.rs');before=p.read_text();addition='''
#[test]
fn v16_math_source_linebreak_marker_keeps_two_rows() {
    let plain = seq(vec![text("a"), Content::linebreak(), text("b")]);
    let source = seq(vec![text("a"), src(Content::linebreak(), true), text("b")]);
    let expected = observe(&plain, true); assert_eq!(expected.line_count, 2);
    equivalent(&expected, &observe(&source, true));
}
#[test]
fn v16_math_source_align_marker_keeps_two_aligned_columns() {
    let make = |on| seq(vec![text("1"), src(Content::math_align_point(), on), text("2"),
        Content::linebreak(), text("333"), src(Content::math_align_point(), on), text("4")]);
    let expected = observe(&make(false), true); assert_eq!(expected.line_count, 2);
    equivalent(&expected, &observe(&make(true), true));
}
''';p.write_text(before+addition);t=json.load(open(out/'tests-before-code.json'));shutil.copyfile(out/'tests-before-code.json',out/'tests-original27.json');t['marker_before']=before;t['marker_addition']=addition;t['new_source_text']=p.read_text();t['expected_files'][str(p)]=p.read_text();t['new_tests']=29;(out/'tests-before-code.json').write_text(json.dumps(t,ensure_ascii=False,indent=2)+'\n')
for name in ['baseline-functional-red.log','baseline-result.json']:
 shutil.copyfile(out/name,out/('original27-'+name))
p=out/'workers/baseline.py';s=p.read_text().replace('5 passed; 22 failed;','5 passed; 24 failed;').replace('22 fail','24 fail');p.write_text(s);print('Two grid-marker regressions prescribed and written before implementation')
