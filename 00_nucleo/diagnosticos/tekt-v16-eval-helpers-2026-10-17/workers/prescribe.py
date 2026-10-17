from pathlib import Path
import json,gzip,hashlib,re,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items())
rows=sorted(json.load(open(out/'contexts.json')),key=lambda x:x['line'])[:11];assert [r['line'] for r in rows]==[129,494,538,566,571,632,645,771,941,963,1098]
variants=json.load(open(out.parent/'tekt-v16-introspect-origins-2026-10-16/scope.json'))['variants'];assert len(variants['Content'])==120 and len(variants['Value'])==39
reasons=['Projeção visual legada remove apenas Located/Realized/ShowRecipe; demais nós não são substituídos pelo adapter.','Busca da primeira heading com numbering herdado; demais variantes não dão resultado nem abrem nova recursão.','Coleta de headings na ordem atual; folhas excluídas não acrescentam itens.','Valor diferente de Str/None conserva o padrão de numeração herdado, sem validá-lo como novo padrão.','Busca de Equation com numbering herdado; os demais nós não produzem resultado.','Busca delimitadores de Matrix/Vec na travessia atual; demais variantes não fornecem delimitador.','Busca limits do MathOp na travessia atual; demais variantes não fornecem limits.','Coleta numbering de Figure com estilo e Label atuais; demais nós não acrescentam itens.','Detector de Dynamic nas travessias atuais; as demais variantes não satisfazem esse detector.','Busca custom em Styled/Sequence com preferência e curto-circuito atuais; demais nós não fornecem propriedade.','Busca custom em Styled/Sequence, preservando primeira chave e ordem; demais nós não fornecem propriedade.']
for i,r in enumerate(rows):
 assert r['pattern_source']=='_' and all(' if ' not in a['pattern'] for a in r['arms'])
 enum='Value' if r['line']==566 else 'Content';handled=re.findall(r'\b'+enum+r'::(\w+)', ' '.join(a['pattern'] for a in r['arms']));assert len(handled)==len(set(handled));assert all(n in variants[enum] for n in handled)
 r.update(id='E'+str(i+1).zfill(2),enum=enum,handled=handled,complement=[n for n in variants[enum] if n not in handled],reason=reasons[i]);r['replacement']=' | '.join(variants[enum][n] for n in r['complement'])
scope={'sources':['01_core/src/compiler/eval/tests.rs'],'owners':['00_nucleo/prompts/compiler/eval/tests.md'],'variants':variants,'targets':rows,'initial_counts':{'V16':233,'V19':907,'V20':871},'deferred_test_helpers':52}
(out/'scope.json').write_text(json.dumps(scope,ensure_ascii=False,indent=2)+'\n')
p=Path(scope['owners'][0]);s=p.read_text();assert s==b['files'][str(p)]
addition='''

## Helpers de projeção e estilos — V16, 2026-10-17

### Medição anterior à decisão

`01_core/src/compiler/eval/tests.rs:129,494,538,566,571,632,645,771,941,963,1098`
contém onze braços abertos auditados por AST, sem guards nesses matches.
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
UTC, diff HEAD --stat exato, fontes, configuração, hashes e parser pinado
estão em `00_nucleo/diagnosticos/tekt-v16-eval-helpers-2026-10-17/before.json.gz`,
`contexts.json` e `audit-recipe.json`. A enumeração atual mede 120 variantes
Content e 39 Value; o snapshot antes do código preserva as definições por hash.
A suíte anterior passou no mesmo estado executável, confirmado pelos hashes.
São helpers cfg(test), não decisões novas da linguagem. Não se infere intenção
upstream nem benefício de leitura por IA a partir de uma nota do linter.

### Prescrição e justificativa de V19 por decisão

Trocar exclusivamente o padrão `_` de cada braço abaixo pelo complemento
nominal **atual** do enum indicado em relação às variantes atendidas nos demais
braços. Usar o prefixo em escopo, payload `_`/`{ .. }` adequado e grupo plano,
sem capturar binding novo. O complemento integral e seus spans estão em
scope.json; enumerações futuras exigem nova decisão pelo compilador.
'''
for r in rows:
 addition+=f"\n- **{r['id']}**, linha {r['line']}, `{r['function']}`, {r['enum']}: excluir do complemento `{', '.join(r['handled'])}`. {r['reason']} O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.\n"
addition+='''

Conservar byte a byte os demais tokens Rust após normalização por rustfmt:
corpos, guards, pattern payloads anteriores, chamadas, early returns, ordem,
short-circuit, estilos herdados e todas as fixtures/asserções existentes.
Não normalizar Source nem ampliar travessia neste lote; os helpers têm escopo
específico e a projeção semântica preservada continua separada do adapter visual.
Não converter match em if-let/matches!, não adicionar allow/expect, não mudar
limiares ou grupos preexistentes para calar V19. Esta nota é informativa e deve
continuar visível. Nenhum teste ou resultado ausente passa a ser aceito.

### Aceitação e limites

Manutenção interna de testes, ADR-0127 contínuo: sem contrato público, entidade,
default, fase, dependência ou compatibilidade nova. RED estrutural: onze V16
no parser pinado e sentinelas Content/Value sem E0004 nesses onze matches;
GREEN: V16 removidos e E0004 nos onze após inserir uma variante sentinela em
cada enum numa cópia descartável do workspace com cfg(test) compilado.
Não alegar RED funcional nem nova paridade: o comportamento é preservado.
Conferir replay dos padrões com corpos imutáveis, enum/assinaturas intactos,
SARIF diferencial completo (onze V16 a menos, onze V19 de complementos a mais,
V20 inalterado), suíte workspace com os mesmos esperados, build, fmt, Clippy
workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc
-D warnings e quatro índices HTML, V1/V5/V15/V26 estrito e linhagem reversa.
Resselo somente deste par. Medições com fontes congeladas, proveniência completa,
MemoryMax=12 GiB, MemorySwapMax=0, Cargo um job e comandos pesados sequenciais.
Os outros 52 V16 neste consumer e Source(Table) permanecem fora da materialização
desta seção; investigar cada um antes de prescrever correção. Não publicar release.
'''
p.write_text(s+addition)
(out/'l0-before-code.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'owner':str(p),'expected_text':p.read_text(),'sha256':sha(p),'code_unchanged':all(sha(x)==h for x,h in b['hashes'].items() if x!=str(p))},ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'L0_first':True,'targets':len(rows),'group_sizes':[len(r['complement']) for r in rows]}))
