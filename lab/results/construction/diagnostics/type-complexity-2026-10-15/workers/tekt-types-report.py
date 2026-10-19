from pathlib import Path
import gzip,json,hashlib,difflib,csv,datetime,collections
root=Path.cwd();out=root/'00_nucleo/diagnosticos/type-complexity-2026-10-15';b=json.load(gzip.open(out/'before.json.gz'))
exceptions={'C022':('SYM_GROUPS','Tabela estática nome/base/factory documentada; manter a forma explícita.'),'C024':('pub fn native_fn_addr','Ponteiro de função nativa com assinatura explícita; evitar uma nova API somente para abreviação.'),'C128':('const ACCESSIBLE','Composição estática de standards e labels existentes em L4; manter a tabela local.')}
records={'C006':('struct EvaluatedPass','EvaluatedPass: nomes para valor, escopo, estilos bibliográficos, metadados e recipes.'),'C013':('struct RowCell','RowCell: nomes para identidade, conteúdo, coluna e spans, conservando empréstimos.'),'C093':('fn parse_tiff_orientation','TiffReaders: leitores u16/u32 nomeados; seleção II/MM compartilhada.'),'C094':('fn parse_tiff_dpi','TiffReaders: leitores u16/u32 nomeados; seleção II/MM compartilhada.'),'C096':('struct PreparedBundle','PreparedBundle privado ao pipeline: bodies, introspector e document_starts.'),'C097':('struct PagedCompilation','PagedCompilation privado ao pipeline: document e introspector.')}
cases=[]
for c in b['cases']:
 p=c['path'];lines=(root/p).read_text().splitlines()
 if c['id'] in exceptions:
  anchor,action=exceptions[c['id']];status='exceção pontual';reason='Contrato já explícito; #[expect] nominal com reason e gate unfulfilled_lint_expectations.'
 elif c['id'] in records:
  anchor,action=records[c['id']];status='corrigido';reason='Campos semânticos substituem posições anônimas, sem mudar dados ou execução.'
 else:
  anchor='';action='Reutilizar o alias público FontKey já existente; expansão de tipo idêntica.';status='corrigido';reason='Um nome canônico substitui 21 repetições do mesmo tipo de fontes.'
 if anchor:line=next(i for i,t in enumerate(lines,1) if anchor in t)
 else:
  # Find the original occurrence by its ordered index among the warnings in the same file.
  peers=[x for x in b['cases'] if x['path']==p and x['id'] not in exceptions and x['id'] not in records]
  occurrences=[i for i,t in enumerate(lines,1) if 'fonts:' in t and 'FontKey' in t]
  line=occurrences[peers.index(c)]
 cases.append(dict(id=c['id'],status=status,path=p,before_line=c['line'],after_line=line,owner=c['owner'],action=action,reason=reason))
assert len(cases)==30 and len({c['id'] for c in cases})==30
(out/'cases.json').write_text(json.dumps(cases,ensure_ascii=False,indent=2)+'\n')
with (out/'cases.csv').open('w') as f:
 w=csv.DictWriter(f,fieldnames=list(cases[0]));w.writeheader();w.writerows(cases)
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(v['text'].splitlines(True),(root/p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p,v in sorted(b['files'].items())))
rows=['| Caso | Situação | Código atual | L0 proprietário | Decisão |','|---|---|---|---|---|']
for c in cases:rows.append(f"| {c['id']} | {c['status']} | [{c['path']}:{c['after_line']}]({root/c['path']}:{c['after_line']}) | [{c['owner']}]({root/c['owner']}) | {c['action']} |")
(out/'lista.md').write_text('# Tratamento dos 30 casos de type_complexity\n\nMedição de origem e localizações anteriores: before.json.gz. As linhas abaixo apontam para a implementação atual. O relatório descreve a validação e a proveniência.\n\n'+'\n'.join(rows)+'\n')
print(json.dumps(dict(cases=len(cases),statuses=dict(collections.Counter(c['status'] for c in cases))),ensure_ascii=False))
