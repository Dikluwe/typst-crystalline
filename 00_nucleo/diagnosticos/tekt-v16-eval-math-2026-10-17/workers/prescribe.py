from pathlib import Path
import json,gzip,re,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());variants=json.load(open(out.parent/'tekt-v16-introspect-origins-2026-10-16/scope.json'))['variants'];variants['MathAttachSlot']={'Omitted':'MathAttachSlot::Omitted','ExplicitNone':'MathAttachSlot::ExplicitNone','Present':'MathAttachSlot::Present(_)'}
rows=sorted(json.load(open(out/'all-contexts.json')),key=lambda r:r['pattern_start_byte']);assert len(rows)==70
inventory={r['line']:r for r in json.load(open(out/'wildcard-inventory.json'))}
for i,r in enumerate(rows,1):
 r['id']='M'+str(i).zfill(2);en=inventory[r['line']]['enum'];r['enum']=en;r['guarded']=[];r['unconditional']=[];assert r['pattern_source']=='_'
 for a in r['arms']:
  if a['pattern']=='_':continue
  names=re.findall(r'\b'+en+r'::(\w+)',a['pattern']);assert names and all(n in variants[en] for n in names)
  if ' if ' in a['pattern']:r['guarded']+=names
  else:r['unconditional']+=names
 r['complement']=[n for n in variants[en] if n not in r['unconditional']];r['special_primes']=r['line']==12007
 if r['special_primes']:
  assert r['arms'][0]['pattern']=='MathAttachSlot::Present(Content::MathPrimes(elem))';r['unconditional']=[];r['complement']=['Omitted','ExplicitNone','Present'];inside=' | '.join(v for n,v in variants['Content'].items() if n!='MathPrimes');r['replacement']='MathAttachSlot::Omitted | MathAttachSlot::ExplicitNone | MathAttachSlot::Present('+inside+')';r['reason']='Omitted/ExplicitNone e Present de Content diferente de MathPrimes não fornecem contagem; o payload nominal mantém revisão futura dos dois enums.'
 else:
  r['replacement']=' | '.join(variants[en][n] for n in r['complement'])
  if en=='MathAttachSlot':r['reason']='Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.'
  elif 'panic!' in r['arm_source']:r['reason']='Variantes que não satisfazem a asserção falham com o panic anterior, incluindo Quote sem attribution quando o guard falha; nunca aceitar ausência.'
  elif r['function']=='count':r['reason']='Somente Par soma uma ocorrência e os wrappers explicitados recursam; as demais variantes contribuem zero neste contador de teste.'
  elif r['function'] in ['go','break_policy_custom_values']:r['reason']='Coleta de propriedade na travessia já especificada; demais variantes não acrescentam itens e a herança/ordem permanecem.'
  else:r['reason']='Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.'
 if r['guarded']:r['reason']+=' As variantes do guard permanecem no complemento para conservar a falha da condição.'
assert sum(r['status']=='emitted-warning' for r in rows)==52
scope={'sources':['01_core/src/compiler/eval/tests.rs','01_core/src/compiler/introspect/walk.rs'],'owners':['00_nucleo/prompts/compiler/eval/tests.md','00_nucleo/prompts/compiler/introspect/walk.md'],'variants':variants,'targets':rows,'initial_counts':{'V16:error':222,'V19:note':918,'V20:note':871},'emitted':52,'latent':18}
(out/'scope.json').write_text(json.dumps(scope,ensure_ascii=False,indent=2)+'\n')
base=f"HEAD `{b['head']}`, working tree não commitado. UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `00_nucleo/diagnosticos/tekt-v16-eval-math-2026-10-17/before.json.gz`. AST do parser pinado em contexts.json/all-contexts.json/wildcard-inventory.json; definições atuais por hash. A suíte anterior passou no mesmo estado executável, revalidado por hashes antes de decidir. Sem inferir intenção upstream ou benefício cognitivo da IA a partir do lint."
addition='\n\n## Helpers matemáticos e estruturais — exaustividade V16, 2026-10-17\n\n### Medição anterior à decisão\n\n'+base+'\n\nO inventário mede 52 V16 emitidos e 18 fallbacks nominais não emitidos (sete Content e onze MathAttachSlot). Guards em leaf/MathFrac/Quote e o payload Present(MathPrimes) são refinamentos: não representam cobertura incondicional dessas variantes. A propriedade pretendida é revisão diante de enum novo, sem alterar linguagem ou asserção.\n\n### Prescrição exclusiva e V19 justificado por caso\n\nPara cada caso abaixo, trocar somente o padrão `_` pelo complemento nominal **atual** das variantes cobertas incondicionalmente pelos braços anteriores. Content tem 120 variantes; MathAttachSlot tem Omitted/ExplicitNone/Present(Content). Não remover do complemento variantes atendidas somente sob guard. Manter guards, corpos, ordem de busca/slots, any/find_map, callbacks, retornos, erros, fixtures e esperados integralmente. Não trocar por if-let/matches!, não adicionar allow/expect, não alterar limiar. Cada grupo V19 continua informativo; não é prova de executar todas as alternativas.\n'
for r in rows:addition+=f"\n- **{r['id']}**, `01_core/src/compiler/eval/tests.rs:{r['line']}`, `{r['function']}`, {r['enum']}, {'emitido' if r['status']=='emitted-warning' else 'latente'}: excluir somente `{', '.join(r['unconditional']) or '(nenhuma variante inteira)'}`; manter sob fallback `{', '.join(r['guarded']) or '(sem guard)'}`. {r['reason']}\n"
addition+='''

O caso prime_count substitui o catch-all por Omitted | ExplicitNone | Present
com complemento nominal de Content menos MathPrimes dentro do payload. Não
usar Present(_) como novo catch-all interno: a sonda deve exigir revisão diante
de Content novo e de MathAttachSlot novo. Os demais slots Present(Content)
continuam delegando a seus helpers. As asserções Quote/Label continuam falhando
com o mesmo texto; não converter qualquer erro em None ou falso sucesso.

### Aceitação e escopo

Correção interna de testes ADR-0127 contínuo, sem API pública, fase, default,
compatibilidade ou dependência nova. RED estrutural dos 52 avisos e sonda em
workspace descartável compilando lib cfg(test): zero E0004 nos 70 locais antes;
após, E0004 em todos com sentinelas Content/MathAttachSlot. Testar também
controles de guard recusado (leaf e fração lined) e asserções Quote existentes,
sem alterar seus esperados. Replay dos spans conserva tudo exceto padrões,
rustfmt, metadados e selos. SARIF diferencial completo: 52 V16 removidos,
notas V19 dos grupos prescritos, V20 e demais diagnósticos preservados após
mapear linhas. Não exigir remoção de V16 alheios para declarar este lote fechado.
Suíte workspace completa, build, fmt, Clippy all-targets -D warnings e
-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices HTML,
V1/V5/V15/V26 estrito, reversa e diff. Limite 12 GiB, swap zero, um job Cargo,
execução pesada sequencial, fontes congeladas e proveniência completa.
'''
p=Path(scope['owners'][0]);assert p.read_text()==b['files'][str(p)];p.write_text(p.read_text()+addition)
addition='\n\n## Raiz Table com origem transparente — 2026-10-17\n\n### Medição anterior à decisão\n\n'+base+'''

source-table-before.json registra a sonda externa ligada à lib real pela API
pública introspect: Realized(Table(Heading hidden), Text replacement) retorna
zero headings; a mesma origem com Source retorna um heading ocultado. O núcleo
content/source-origin já exige transparência e contribuição única; o contrato
raso de Table consumida já proíbe indexar children/caption ocultados. A diferença
observável é query(heading), não igualdade Rust. As seis sondas CLI da etapa
tekt-v16-eval-helpers-2026-10-17 passaram, portanto eram evidência insuficiente
para refutar o defeito do carrier construído pelo contrato público. Inferência
causal: table_root recursa Located/ShowRecipe mas classifica Source como false,
escolhendo o walk profundo indevido. Refutar por índice diferente, perda de
visual, identidade/ordem/counter errado ou efeitos duplicados após normalizar.
A primeira tentativa externa usou helper crate-private e falhou E0624; é erro
de montagem da sonda, nunca RED funcional. A construção pública posterior mede
o defeito e tem receita/rlib/binário/saída/UTC preservados.

### Prescrição exclusiva vigente

No helper local table_root do ramo Realized, trocar somente o scrutinee
`match c` por `match c.source_body()`. Desembrulhar Source por empréstimo antes
de classificar Table, Located e ShowRecipe; preservar todos os braços nominais,
corpos e ordem existentes. Não classificar todo Source como Table, não retirar
o carrier armazenado, não mover fase, não percorrer corpos semânticos ocultados,
não alterar reserva, forced_location, shallow_table, tag própria, assinaturas,
payloads, queries, synthesis ou comportamento de outro elemento. Source que
contém Heading continua não-Table e conserva a travessia original e visual.
Supersede a exclusão de table_root no lote anterior somente para esse reparo
interno da transparência já vigente, sem contrato público/default novo.

### Testes antes do reparo e aceitação

Adicionar neste consumer quatro testes puros pela API pública de introspecção:
Table crua consumida conserva tag própria e zero heading ocultado; Source único,
aninhado e nas duas ordens com Located e dentro de ShowRecipe não publica o
heading ocultado e conserva uma tabela; visual Heading permanece consultável
e Location explícita da Table se conserva; Source(Heading) não-Table conserva
origem e visual. RED real antes de mudar scrutinee; GREEN depois, sem mudar
expectativas. Preservar os controles Source de contexto anteriores e demais
testes. O reparo altera uma seleção privada de travessia, na mesma fase, para
restaurar contrato já ratificado: ADR-0127 contínuo. Resselo dos dois pares
somente após grafo V15/V26 válido. Repetir a sonda pública com 0/0, as seis
sondas CLI e os gates workspace/Clippy/Rustdoc/build/fmt/linhagem/reversa/SARIF.
Nenhum V20 ou diagnóstico adicional; 12 GiB, swap zero, um job e sem publicação.
'''
p=Path(scope['owners'][1]);assert p.read_text()==b['files'][str(p)];p.write_text(p.read_text()+addition)
(out/'l0-before-code.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'owners':{p:Path(p).read_text() for p in scope['owners']},'productive_sources_unchanged':all(sha(p)==h for p,h in b['hashes'].items() if p not in scope['owners'])},ensure_ascii=False,indent=2)+'\n');print(json.dumps({'L0_first':True,'structural_decisions':len(rows),'functional_Source_fix':True,'code_changed':False}))
