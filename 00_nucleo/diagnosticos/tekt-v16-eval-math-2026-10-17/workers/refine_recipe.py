from pathlib import Path
import json,gzip,hashlib,datetime,shutil
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();v=json.load(open(out/'validation-result.json'));assert v['commands'][0]['exit']==101;assert all(sha(p)==h for p,h in v['source_hashes'].items());assert '4 passed; 1 failed;' in (out/'validation-controls.log').read_text();(out/'attempt-recipe').mkdir()
for name in ['validation-controls.log','validation-result.json','validation-resources.json']:shutil.move(str(out/name),str(out/'attempt-recipe'/name))
source=Path(scope['sources'][1]);owner=Path(scope['owners'][1]);s=source.read_text();addition='''

### Refinamento medido — contexto de ShowRecipe em raiz Table

A primeira validação deste lote (attempt-recipe/validation-controls.log) mede
4 controles passando e 1 falhando. Source direto/aninhado e nas duas ordens
com Located deixam de publicar hidden; ShowRecipe(Source(Table)) ainda publica
uma heading ocultada. O diagnóstico anterior da raiz não abrangia o transporte
do contexto em ShowRecipe: esse ramo de walk_at delega a walk e reinicializa
forced_location=None e shallow_table=false. Estado/UTC/hashes e a falha estão
preservados antes deste refinamento; não se altera o RED original.

Antes do código, acrescentar controle recipe-plain à matriz de carriers e teste
Located(ShowRecipe(Source(Table))) com visual Heading. A Table conserva a
Location explícita, continua única e não publica o heading hidden; visual shown
permanece consultável uma vez. Confirmar RED adicional sem enfraquecer controles.

Trocar exclusivamente a delegação do ramo ShowRecipe de walk para walk_at,
passando os mesmos acumuladores, lang, chain, label_from_parent e os valores
recebidos forced_location/shallow_table. Não observar/alocar Location do carrier,
não aplicar callbacks/recipes, não trocar chain por estilos capturados nem
mudar reserva ou assinatura. O corpo continua projetado uma vez na mesma fase.
É reparo privado de contexto da identidade já preparada, equivalente à regra
vigente de Source; sem API pública, default novo ou compatibilidade nova.
Esse refinamento complementa c.source_body de table_root; não o substitui.

Replay autoriza essas duas expressões privadas e os controles novos, sem outras
mudanças. Os seis testes selecionados devem passar após o reparo; cinco novos
são deste owner e um do owner de eval. Suíte workspace esperada cresce em seis
testes sobre o estado de entrada, sem alterar os casos antigos ou os sete
ignorados. Preservar build/fmt/Clippy/Rustdoc/linhagem/reversa/SARIF e 12 GiB.
''';owner.write_text(owner.read_text()+addition)
old='''        Content::ShowRecipe(e) => {
            walk(
                &e.body,
                locator,
                tags,
                intr,
                auto_label_counter,
                lang,
                chain,
                label_from_parent,
            );
            return;
        }''';new=old.replace('            walk(', '            walk_at(').replace('                label_from_parent,\n','                label_from_parent,\n                forced_location,\n                shallow_table,\n');assert s.count(old)==1
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source':str(source),'owner':str(owner),'L0_first':True,'expected_l0':owner.read_text(),'source_before_tests':s,'old':old,'new':new}
(out/'recipe-refinement.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print('Recipe-context refinement prescribed before tests/code')
