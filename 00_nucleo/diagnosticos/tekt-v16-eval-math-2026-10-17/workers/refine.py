from pathlib import Path
import json,gzip,hashlib,re,datetime
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));candidate=json.load(gzip.open(out/'candidate-state.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in candidate['source_hashes'].items());assert candidate['counts']=={'V16:error':170,'V19:note':988,'V20:note':872}
p=Path(scope['owners'][0]);addition='''

### Refinamento medido de prime_count — V20, 2026-10-17

candidate-state.json.gz/candidate-architecture.log do lote
tekt-v16-eval-math-2026-10-17 preservam o candidato, estado/UTC/hashes e a medição:
170 V16, 988 V19 e 872 V20. O complemento aninhado de Present(Content alternativo)
acrescentou um V20 à profundidade já existente do braço Present(MathPrimes).
A implementação direta foi, portanto, refutada pelo critério de não acrescentar
profundidade; isso não é falha funcional ou motivo para silenciar V20.

Superseder somente a forma do caso prime_count: manter assinatura e substituir
o match aninhado por dois matches rasos. O externo distingue Present(prime_content)
de Omitted | ExplicitNone; o interno retorna Some(elem.count) para MathPrimes e
None para o complemento nominal das outras 119 variantes atuais de Content.
Sem guard, captura @, clone, if-let, helper/API novo ou alteração dos demais
69 locais, fixtures/asserções e ordem. Esses dois complementos V19 são legítimos
porque compartilham somente o mesmo None em cada domínio. O conteúdo não é
validado duas vezes nem percorrido: só se lê count quando há MathPrimes.

A sonda mantém os 70 locais originais e acrescenta o match interno auxiliar:
variante MathAttachSlot nova exige revisão do externo e Content nova exige
revisão do interno. O resultado esperado é 71 E0004 selecionados no GREEN,
zero no RED inicial, sem outros erros. Comparar árvores por ordinal, descontando
somente o novo scrutinee prime_content. SARIF esperado do lote final: 52 V16
removidos, 71 V19 prescritos novos, um V20 antigo de prime_count removido e
nenhum V20 novo. Todos os outros diagnósticos preservados. Replay autoriza
somente esta reestruturação local adicional, conforme receita literal no recibo
de refinamento. Os gates de aceitação e limite de 12 GiB anteriores permanecem.
''';p.write_text(p.read_text()+addition)
source=Path(scope['sources'][0]);s=source.read_text();a=s.index('    fn prime_count(');opening=s.index('{',a);z=opening+1;depth=1
while depth:depth+=(s[z]=='{')-(s[z]=='}');z+=1
old=s[a:z];sig=s[a:opening];assert sig.strip()=='fn prime_count(slot: &MathAttachSlot) -> Option<usize>',sig
complement=' | '.join(v for n,v in scope['variants']['Content'].items() if n!='MathPrimes')
new=sig+'''{
        match slot {
            MathAttachSlot::Present(prime_content) => match prime_content {
                Content::MathPrimes(elem) => Some(elem.count),
                '''+complement+''' => None,
            },
            MathAttachSlot::Omitted | MathAttachSlot::ExplicitNone => None,
        }
    }'''
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'measurement':candidate['counts'],'source':str(source),'owner':str(p),'expected_l0':p.read_text(),'old':old,'new':new,'source_before':s,'L0_first':True,'code_unchanged':source.read_text()==candidate['files'][str(source)]}
(out/'refinement.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print('Refinement L0 saved before code; nominal group now delegates to shallow Content match')
