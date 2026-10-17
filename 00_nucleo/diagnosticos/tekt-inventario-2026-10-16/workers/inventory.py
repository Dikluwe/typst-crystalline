from pathlib import Path
import json,csv,io,re,hashlib,datetime,collections,subprocess
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-inventario-2026-10-16';validation=root/'00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
v=json.loads((validation/'validation-details.json').read_text());assert any(c['label']=='architecture' and c['exit']==0 for c in v['commands'])
assert all(sha(p)==h for p,h in v['source_hashes'].items())
sarif=json.loads((validation/'validation-architecture.log').read_text());results=sarif['runs'][0]['results'];warnings=[r for r in results if r['level']=='warning'];notes=[r for r in results if r['level'] not in ['warning','error']];assert not any(r['level']=='error' for r in results)
rows=[]
for i,r in enumerate(warnings,1):
 loc=r['locations'][0]['physicalLocation'];p=loc['artifactLocation']['uri'].removeprefix('./');line=loc['region']['startLine'];t=Path(p).read_text();m=re.search(r'^//! @prompt (.+)$',t,re.M);assert m,p;owner=m[1];lines=t.splitlines()
 rows.append(dict(id=f'T{i:03}',rule=r['ruleId'],source=p,line=line,column=loc['region'].get('startColumn',1),owner=owner,source_sha256=sha(p),owner_sha256=sha(owner),test_file='/tests/' in p or p.endswith('/tests.rs'),message=r['message']['text'],context='\n'.join(lines[max(0,line-3):line+2]),status='Auditar L0 e tipo/observável antes de prescrever alteração'))
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=v['head'],working_tree=True,validation=str(validation/'validation-details.json'),validation_sha256=sha(validation/'validation-details.json'),sarif_sha256=sha(validation/'validation-architecture.log'),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),counts=dict(collections.Counter(r['rule'] for r in rows)),notes=len(notes),note_rules=dict(collections.Counter(r['ruleId'] for r in notes)),linter_provenance=str(validation/'linter-provenance.json'),linter_provenance_sha256=sha(validation/'linter-provenance.json'),rows=rows)
(out/'inventory.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
f=io.StringIO();fields=['id','rule','source','line','owner','test_file','message','status'];w=csv.DictWriter(f,fieldnames=fields);w.writeheader();w.writerows({k:r[k] for k in fields} for r in rows);(out/'inventory.csv').write_text(f.getvalue())
counts=record['counts'];byowner=collections.Counter(r['owner'] for r in rows)
text=f'''# Inventário Tekt após encerramento Clippy

**{len(rows)} avisos**, sem erros; {len(notes)} notas separadas, que não entram na contagem de avisos. Medição SARIF da validação {v['start']} até o gate arquitetural; fontes/L0 conferidos por SHA-256 sem alterações. HEAD `{v['head']}`, working tree não commitado. [inventory.json]({out}/inventory.json) guarda UTC, hashes, status, diffstat e contexto de cada ocorrência; [CSV]({out}/inventory.csv).

| Ordem sugerida | Regra | Avisos | Auditoria necessária |
|---|---|---:|---|
| 1 | V18 | {counts.get('V18',0)} | Cardinalidade e bytes: comparar ranges com decisões de domínio equivalentes, preservando spans/saídas |
| 2 | V17 | {counts.get('V17',0)} | Guards compostos: preservar precedência, bindings, curto-circuito e fallthrough |
| 3 | V16 | {counts.get('V16',0)} | Wildcards de domínio: ler tipo/L0 e decidir variantes nominais, sem adotar enums futuros silenciosamente |
| 4 | V12 | {counts.get('V12',0)} | Tipos/impl em L4: separar composição produtiva de infraestrutura de testes antes de mover contratos |

A implementação pinada do linter mantém V16 visível mesmo com citações/exceções; não propor anotações como solução. As {len(notes)} notas cobrem delegações de fallback V16, alternativas OR V19 e profundidade V20; são conservadas por regra em notes.json e não foram descartadas ou reclassificadas como correções fechadas. Flags/rules globais não serão rebaixadas para fechar contagem. A coluna test_file é somente classificação pelo caminho; não prova que um item fora desses caminhos seja produtivo ou que um teste esteja isento de arquitetura.

V12 foi localizado em um enum de produção (`04_wiring/src/main.rs::Published`) e 18 declarações em arquivos de teste. Isso exige distinguir papel de composição e harness de testes antes de decidir realocação ou regra; não é autorização para isentar testes automaticamente.

Este inventário é diagnóstico, não L0 nem autorização de código. A lista contém {len(byowner)} owners; cada correção exige ler o owner atual, prescrever nele a alteração e resselar somente seu consumer. Mudanças públicas seguem ADR-0127; internas seguem fluxo contínuo com testes do observável.

| Caso | Regra | Local | L0 | Evidência |
|---|---|---|---|---|
'''
for r in rows:
 msg=r['message'].replace('|','\\|').replace('\n',' ');text+=f'| {r["id"]} | {r["rule"]} | [{r["source"]}:{r["line"]}]({root/r["source"]}:{r["line"]}) | [{r["owner"]}]({root/r["owner"]}) | {msg} |\n'
(out/'lista.md').write_text(text);(out/'notes.json').write_text(json.dumps(notes,ensure_ascii=False,indent=2)+'\n')
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256'))
print(json.dumps(dict(warnings=len(rows),rules=counts,owners=len(byowner),notes=len(notes))))
