from pathlib import Path
import datetime,hashlib,json,re
R=Path.cwd();P=R/'00_nucleo/materialization/incremental-prioridade6-2026-10-08';C=P/'candidates'
np='00_nucleo/prompts/_nuclei/watch-source-generation.toml'
nucleus='''tekt = 1
kind = "nucleus"
id = "watch-source-generation"
title = "Reuso sintático validado por edição"

[[claims]]
id = "exact-current-source"
level = "must"
statement = "Um hit sintático exige leitura atual bem-sucedida, caminho canônico físico, FileId atribuído pelo World corrente e texto UTF-8 integral iguais; hash ou metadata não bastam."

[[claims]]
id = "fresh-world"
level = "must"
statement = "IDs/mapas/dependências/bytes/inputs/plugins/clock e caches restantes continuam por World; a sessão nunca restaura IDs ou dispensa resolução e registro atuais."

[[claims]]
id = "syntax-only"
level = "must-not"
statement = "Não guardar Module, Func, efeito, diagnóstico emitido, frame, Location, fonte ou transiente na sessão; somente Source markup física imutável, sem mudar fase ou avaliação."

[[claims]]
id = "bounded-lifetime"
level = "must"
statement = "A sessão pertence a uma chamada watch, sem estado global/disco/unsafe, com LRU limitada a 8 entradas, 1 MiB de texto agregado e 512 KiB por entrada; orçamento não promete RSS."
'''
n=C/np;n.parent.mkdir(parents=True,exist_ok=True);n.write_text(nucleus)
h=hashlib.sha256(n.read_bytes());h.update(b'\0TEKT-NUCLEUS-DEPS-V1\0');pin=h.hexdigest()
evidence='''### Medição anterior à decisão

00_nucleo/diagnosticos/incremental-prioridade6-2026-10-08/before.json.gz,
measurement.json.gz, control-measurement.json.gz e count.json registram árvore,
HEAD, UTC, comandos e releases. Source::new reconstrói CST; World novo constrói
Source main/import a cada edição. O import mínimo já absorve recursos no eval_ms;
não atribuir toda a métrica a parsing. O contrato causal de Source/id/caminho vem
dos owners existentes; o reuso é mecânica e precisa preservar observáveis de língua.
Hipótese de benefício refutada por custo, diagnóstico, efeito, stale source,
geometria ou identificação diferente. Não inferir intenção upstream.

### Contrato proposto — gate ADR-0127 pendente

'''
parts=[('03_infra/src/world.rs','00_nucleo/prompts/infra/system-world.md','''Exportar SourceSession e oferecer new_with_source_session(root: impl Into<PathBuf>,
main_path: impl AsRef<Path>, session: Arc<SourceSession>) -> Result<Self,SystemWorldError>.
Existing new e transientes conservam assinatura/identidade/erros. Configurar sessão
antes da primeira leitura eager e carregar main na política normal; guardar somente
Arc opcional privada e transportá-la a slots físicos. World restante sempre fresco.
Não incorporar FontCatalogSession nesta sessão nem alterar seu contrato.
'''),('03_infra/src/world/cache.rs','00_nucleo/prompts/infra/system-world/cache.md','''SourceSlot físico pode transportar Arc opcional privada da sessão. Após read_to_string
normal, fornecer caminho canônico/id/texto completos ao owner source_session;
sem sessão, usar Source::new vigente. Registrar IDs/deps normalmente. OnceLock de
cada World mantém seu snapshot/error; nenhum erro de leitura persiste na sessão.
Synthetic/from_source/code/expression permanecem fora do cache inter-World.
Conservar interfaces privadas existentes quando isso evitar reparos externos.
'''),('03_infra/src/world/source_session.rs','00_nucleo/prompts/infra/system-world/source_session.md','''Possuir SourceSession pública opaca com new() sem I/O, campos privados e métodos
internos de comparação/retensão. Guardar apenas Source markup imutável por caminho
canônico físico e FileId corrente, validar texto integral sem Source Eq/hash como
prova. Não atribuir IDs. Manter no máximo 8 entradas LRU, 1 MiB total de texto e
512 KiB/entrada; oversize devolve parsing normal sem retenção. Uso de Mutex por
instância em L3, zero globals/unsafe/disco/clock. Parsing usa Source::new de L1.
Concorrência pode repetir trabalho; nenhum retorno usa Source de texto/id/path
não validados. Não guardar outros produtos ou erros de leitura; dados CST com
errors/warnings seguem reemissão normal pelo eval, sem cache do Sink.
Testes locais devem incluir reuso em Worlds reais, recusa por texto/id/path,
limites/evicção, transientes, erro/recuperação e concorrência. Memória real não é
limitada pelo orçamento de texto: medir RSS. Sem comprovado ganho de produto,
retirar conexão produtiva e conservar evidência negativa.
'''),('04_wiring/src/compile.rs','00_nucleo/prompts/wiring/compile.md','''Transportar Arc opcional SourceSession por helper observado privado, junto da
sessão de fontes existente. APIs públicas compile/observed conservam assinatura
e delegam sem sessão sintática. Para physical input com sessão, usar constructor
new_with_source_session antes da leitura eager; sem sessão/transiente, caminho
existente. Não executar validação ou lógica de cache em L4. Preservar erros,
inputs/font_paths/plugins/cert/targets/staging/deps/timers e ordem de diagnóstico.
'''),('04_wiring/src/watch.rs','00_nucleo/prompts/wiring/watch.md','''Criar Arc<SourceSession> uma vez por run_watch fora do loop, independente da
FontCatalogSession já ratificada. Passar clones ao helper privado de compile em
cada ciclo; World continua novo. A chamada watcher delimita lifetime/retensão.
Preservar arm-before-publication, staging/HTTP/último sucesso, error recovery,
watch deps/fallback, evict(10) e polling100ms. Não criar nova flag ou modo público.
''')]
rows=[]
for consumer,owner,contract in parts:
 target=R/owner;old=target.read_text() if target.exists() else '# L0 — infra/system-world/source_session\nHash do Código: PENDENTE\n\nNúcleos Tekt:\n'
 old=old.replace('Núcleos Tekt:\n',f'Núcleos Tekt:\n- {np} sha256:{pin}\n',1)
 old+='\n## Reuso sintático de watch — lote A\n\n'+evidence+contract+'''\nAceitação: RED→GREEN, contagem de produção separada de tempos, três rodadas CLI
pareadas contra release P5, startup/RSS e controles de invalidação/identidade/
efeitos/Unicode/diagnóstico/assets/fontes/geométricas. Resselo apenas deste owner,
build/suítes pertinentes e lint zero erros; não fechar incrementalidade completa.
'''
 draft=C/owner;draft.parent.mkdir(parents=True,exist_ok=True);draft.write_text(old)
 sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None
 rows.append({'consumer':consumer,'owner':owner,'candidate':str(draft.relative_to(R)),'candidate_sha256':sha(draft),'base_l0_sha256':sha(target),'base_consumer_sha256':sha(R/consumer)})
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
manifest={'state':'awaiting_concrete_ratification','utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'base_release_sha256':sha(R/'target/release/typst'),'gate':'ADR-0127 §2.1: SourceSession/new e SystemWorld::new_with_source_session públicos','scope':'Lote A sintático somente; B avaliação e C layout fora do pacote','contract':{'path':str((P/'contract.md').relative_to(R)),'sha256':sha(P/'contract.md')},'owners':rows,'nucleus':{'target':np,'candidate':str(n.relative_to(R)),'raw_sha256':sha(n),'effective_sha256':pin}}
(P/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n');print('manifest',sha(P/'manifest.json'),'nucleus',pin)
