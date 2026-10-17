from pathlib import Path
import json,gzip,hashlib,tarfile,collections,datetime,subprocess
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
package=json.load(open(out/'package-result.json'));pub=json.load(open(out/'publication-result.json'));post=json.load(open(out/'post-pin-result.json'));before=json.load(gzip.open(out/'before.json.gz'));review=json.load(gzip.open(out/'review-state.json.gz'))
assert pub['remote_exact_commit_verified'] and pub['pin_applied'] and post['diagnostics_identical_to_review']
assert all(sha(p)==h for p,h in post['source_hashes'].items())
old_manifest=(out/'manifest-sha256.json').read_bytes();approved=json.loads(old_manifest)
assert hashlib.sha256(old_manifest).hexdigest()=='fd4f7669549b54e80f80ca2cfc781f32c0e3179f5db28f1e07a2d0b607db8068'
assert all(sha(out/p)==h for p,h in approved.items())
assert not (out/'approved-review.tar.gz').exists()
with tarfile.open(out/'approved-review.tar.gz','w:gz') as archive:
 for path in approved:archive.add(out/path,arcname=path,recursive=False)
 archive.add(out/'manifest-sha256.json',arcname='manifest-sha256.json',recursive=False)
(out/'review-manifest-sha256.json').write_bytes(old_manifest)
rows=json.loads((out/'post-pin-architecture.log').read_text())['runs'][0]['results'];pending=[x for x in rows if x['ruleId']=='V16'];assert len(pending)==244
lines=['# V16 pendentes após o pin None/prelude','','Estado medido no working tree não commitado, HEAD `'+before['project_head']+'`, pin local `'+package['commit']+'`. Post-pin-result.json conserva comandos, UTC e hashes. São 244 emissões abertas; a classificação funcional/arquitetural requer leitura dos respectivos L0.','','O próximo lote medido reúne quatro decisões de Value em delimitadores/vetores e presença de stroke de matriz, três owners. Planejamento em `../tekt-v16-math-value-2026-10-16/`; nenhuma alteração produtiva desse lote foi aplicada.','','| Consumer | Quantidade |','|---|---:|']
counts=collections.Counter(x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./') for x in pending)
for path,n in sorted(counts.items()):lines.append('| ['+path+']('+str(root/path)+') | '+str(n)+' |')
lines+=['','## Ocorrências medidas','','| Consumer e linha | Diagnóstico |','|---|---|']
for row in pending:
 physical=row['locations'][0]['physicalLocation'];path=physical['artifactLocation']['uri'].removeprefix('./');line=physical['region']['startLine'];message=' '.join(row['message']['text'].split()).replace('|','\\|')
 lines.append('| ['+path+':'+str(line)+']('+str(root/path)+':'+str(line)+') | '+message+' |')
lines+=['','## Pendências não emitidas','','T027 (eval/mod.rs) e T158/T160 (layout/realized.rs) continuam abertos conforme [inventário anterior](../tekt-v16-value-projections-2026-10-16/latent-v16.json); seus fontes permanecem com os mesmos hashes. Some(_) e outros catchalls internos dos nove matches None exigem análise própria. V19 informativo não é lista automática de defeitos ou prova de cobertura.']
(out/'lista-pendencias.md').write_text('\n'.join(lines)+'\n')
resources={p.name:json.load(open(p)) for p in out.glob('*-resources.json')};peak=max(int(x['memory_after']['memory.peak']) for x in resources.values())
for r in resources.values():
 assert r['memory_before']['memory.max']=='12884901888' and r['memory_before']['memory.swap.max']=='0'
 assert 'max 0\n' in r['memory_after']['memory.events'] and 'oom 0\n' in r['memory_after']['memory.events'] and 'oom_kill 0\n' in r['memory_after']['memory.events']
report=(out/'relatorio.md').read_text()
report=report.replace('Estado: candidato implementado e validado; publicação e pin aguardam autorização.','Estado: etapa concluída. Branch publicada com autorização do usuário; pin aplicado e revalidado no working tree do Typst. Objetivo geral de correções continua aberto.')
report=report.replace('O candidato `'+package['commit']+'`','O commit publicado `'+package['commit']+'`')
old='O pin ativo ainda é `'+package['base_commit']+'` e continua emitindo 253; o efeito acima foi medido com o candidato, ainda local.'
new='O pin nos arquivos locais do CI agora é `'+package['commit']+'`; a validação após aplicação confirmou os mesmos 244 V16 e demais diagnósticos idênticos. Não houve commit/push do Typst, portanto não se afirma execução de um workflow remoto deste projeto.'
assert old in report;report=report.replace(old,new)
report=report.replace('Não houve alteração no código/configuração/prompt produtivo do Typst nesta etapa, nem no checkout canônico do linter.','O código e os prompts produtivos do Typst permaneceram intactos. No consumidor, somente `.github/workflows/ci.yml` e `tools/ci/README.md` receberam as quatro substituições de pin aprovadas. O checkout canônico do linter permaneceu intacto; o commit foi preparado em cópia isolada e publicado na branch aprovada.')
old='[proposed-pin.patch](proposed-pin.patch) prepara as quatro referências em `.github/workflows/ci.yml` e `tools/ci/README.md`, sem aplicar pin não publicado. A autorização anterior cobria somente publicar e53de6f; falta autorizar esta nova branch/commit. Não houve push, merge ou release. Após autorização, publicar só a branch, confirmar o commit remoto e então aplicar o pin; não fazer merge/release nem commit/push do Typst.'
new='O usuário autorizou a revisão de manifesto SHA-256 `fd4f7669549b54e80f80ca2cfc781f32c0e3179f5db28f1e07a2d0b607db8068`. A [revisão aprovada congelada](approved-review.tar.gz) e seu [manifesto original](review-manifest-sha256.json) preservam exatamente os artefatos analisados. [publication-result.json](publication-result.json) comprova push da branch, conferência por ls-remote e fetch independente do commit e árvore no GitHub, antes de aplicar as quatro referências do [patch aprovado](proposed-pin.patch). [post-pin-result.json](post-pin-result.json) comprova revalidação após aplicação. Não houve merge, release ou commit/push do Typst. A branch publicada está em [Dikluwe/tekt-linter](https://github.com/Dikluwe/tekt-linter/tree/'+package['branch']+'). Os binários de validação são identificados por caminho e SHA-256 nos registros; não foi substituído o binário genérico instalado no sistema.'
assert old in report;report=report.replace(old,new)
report=report.replace('244 V16 reais/pendentes no candidato','244 V16 ainda a analisar/corrigir')
report+='\nA [lista completa de pendências](lista-pendencias.md) conserva todas as 244 emissões por arquivo/linha, além de T027 e T158/T160 latentes. O fechamento desta etapa não declara encerrado o objetivo geral.\n'
(out/'relatorio.md').write_text(report)
(out/'review.md').write_text('# Revisão aprovada e publicada — V16 None/prelude\n\nO usuário autorizou publicar `'+package['branch']+'`, commit `'+package['commit']+'`, e aplicar o pin. A execução foi concluída sem merge/release ou commit/push do Typst.\n\n[Relatório final](relatorio.md) · [revisão original imutável](approved-review.tar.gz) · [publicação e prova remota](publication-result.json) · [validação após pin](post-pin-result.json).\n\nO manifesto da revisão aprovada tem SHA-256 `fd4f7669549b54e80f80ca2cfc781f32c0e3179f5db28f1e07a2d0b607db8068` e foi preservado em review-manifest-sha256.json. O manifesto final inclui também os registros da publicação.\n')
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':before['project_head'],'status':subprocess.check_output(['git','status','--porcelain=v1'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'source_hashes':post['source_hashes'],'review_approved_manifest_sha256':sha(out/'review-manifest-sha256.json'),'approved_review_archive_sha256':sha(out/'approved-review.tar.gz'),'published_commit':package['commit'],'published_tree':package['tree'],'branch':package['branch'],'publication':pub,'post_pin':post,'resources':resources,'peak_bytes':peak,'peak_gib':peak/1024**3,'linter_suite':review['tests'],'stage_complete':True,'goal_scope_incomplete':True,'remaining_v16_emitted':244,'latent_ids_preserved':['T027','T158','T160'],'no_typst_product_source_change':True,'no_typst_commit_or_push':True,'no_merge_or_release':True}
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
manifest={str(p.relative_to(out)):sha(p) for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest-sha256.json'}
(out/'manifest-sha256.json').write_text(json.dumps(manifest,indent=2)+'\n')
assert all(sha(out/p)==h for p,h in manifest.items())
print(json.dumps({'stage':'complete','active_local_pin':package['commit'],'remaining_v16':244,'v19':888,'v20':871,'peak_gib':peak/1024**3,'manifest_files':len(manifest),'goal_scope_incomplete':True}),flush=True)
