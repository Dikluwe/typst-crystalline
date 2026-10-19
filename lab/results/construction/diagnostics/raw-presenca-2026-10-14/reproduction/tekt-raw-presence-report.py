from pathlib import Path
import json,gzip,hashlib,datetime,base64,difflib
out=Path('00_nucleo/diagnosticos/raw-presenca-2026-10-14');package=Path('00_nucleo/materialization/raw-presenca-2026-10-14')
def read(n):
 with gzip.open(out/n,'rt') as f:return json.load(f)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=read('before-source.json.gz');candidate=read('candidate-final.json.gz');suite=json.loads((out/'workspace-result.json').read_text());checks=json.loads((out/'followup-summary.json').read_text());cmp=json.loads((out/'comparison.json').read_text())
assert suite['exit']==0 and suite['sources_unchanged'] and all(sha(n)==v['sha256'] for n,v in candidate['files'].items())
assert sha(package/'manifest.json')=='b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45'
manifest=json.loads((package/'manifest.json').read_text());assert all(sha(package/n)==v for n,v in manifest['candidates'].items())
patch=''.join(''.join(difflib.unified_diff(before['files'].get(n,{}).get('text','').splitlines(True),v['text'].splitlines(True),fromfile=n,tofile=n)) for n,v in candidate['files'].items());(out/'current.patch').write_text(patch)
repro=out/'reproduction';repro.mkdir(exist_ok=True)
for n in ['setup','tests','implement','probe','render','compare','seal','validate','suite','report']:
 p=Path('/tmp/tekt-raw-presence-'+n+'.py');(repro/p.name).write_bytes(p.read_bytes())
(repro/'README.txt').write_text('Scripts de registro da execução, não comandos idempotentes. setup/tests/implement/seal são históricos de mutação: NÃO executar sobre a árvore já corrigida. probe/render criam diretórios novos. Preservar manifesto aprovado e confirmar hashes de candidate-final antes das validações. A primeira fixture CLI usou Location::from_raw privado; foi corrigida para Locator::next. O teste de encoders passou a usar json(bytes(...))/yaml(bytes(...)), confirmado no pin. A declaração do núcleo foi movida antes das seções e os hashes para o preâmbulo. Todos os estados finais e patches constam no arquivo de evidências.\n')
entries=[]
for p in sorted(list(out.rglob('*'))+list(package.rglob('*'))):
 if p.is_file() and p.name!='evidence.json.gz':entries.append(dict(path=str(p),sha256=sha(p),base64=base64.b64encode(p.read_bytes()).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:archive=json.load(f)
assert all(hashlib.sha256(base64.b64decode(x['base64'])).hexdigest()==x['sha256'] for x in archive['entries'])
text=f'''# Raw: presença lexical aprovada e materializada — 2026-10-14

A mudança aprovada está implementada: omissão de `block`/`lang` permanece distinta
de `block: false` e `lang: none`. O contrato Rust acrescenta `RawElem.set_fields: u8`,
`RAW_SET_BLOCK = 1` e `RAW_SET_LANG = 2`; `Content::raw` conserva a assinatura.
Construções externas por struct literal precisam informar o novo campo, conforme
avisado na proposta. RawLine, fase, render e casts legados não foram alterados.

## Proveniência e autorização

HEAD `{before['head'].strip()}`, working tree não commitado. A lista exata de alterações
`git diff HEAD --stat`, status, UTC, fontes e SHA-256 está em before-source.json.gz,
red-state-final.json.gz, candidate-final.json.gz e workspace-pre.json.gz. Não houve
commit, push ou publicação. Fontes mantiveram os mesmos hashes durante validação
completa. Suíte: {suite['start']} → {suite['end']}.

Manifesto aprovado `{sha(package/'manifest.json')}`; resposta humana
`Aprovar a proposta`, registrada em materialization/raw-presenca-2026-10-14/approval-receipt.json.
O manifesto e os candidatos permanecem byte a byte idênticos aos aprovados. A cópia
canônica atualiza a ratificação e posiciona os metadados no formato exigido pelo
linter. Encoding/CLI tinham avançado após a proposta; suas correções foram preservadas.

Oráculo: vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489,
SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
`before/results.json.gz` e `after/results.json.gz` registram argv, saídas, fonte,
UTC, HEAD/estado e SHA-256 de cada binário. Não se usa a versão textual como prova.

## Medição e decisão

Na fonte inicial, RawElem raw.rs:21–25 só conservava valores; native_raw
markup.rs:78–136 perdia presença. O novo getter retorna somente valores conhecidos;
Content delega, has/at/fields distinguem Set/Unset/Undeclared, igualdade e observação
incluem presença, e CLI/encoders usam a projeção comum. A falha era do modelo de
código e seus consumidores; a especificação aprovada agora legitima a correção.
Não é uma classificação estatística das falhas do projeto inteiro.

O corpus bilateral contém {cmp['after']['comparisons']} comparações de caso/formato,
incluindo controles. Coincidências semânticas: {cmp['before']['matches']} antes,
{cmp['after']['matches']} depois. As quatro diferenças restantes são dois casos
legados em dois formatos: raw() aceita texto ausente e block:none é aceito como
omissão no Tekt; o vanilla rejeita. Estão fora deste gate, sem crédito de paridade.
JSON/YAML são comparados como valores; a ordem text/block/lang tem teste próprio.
Erros comparam mensagem causal; saídas integrais permanecem disponíveis.

As cinco amostras PDF (inline, bloco, markup, Bool/Auto e show) têm pixels idênticos
antes/depois e passaram qpdf; hashes de pixels, fontes e textos estão em comparison.json
mais render-before/render-after. Isso comprova preservação nesses controles, não
paridade universal de render. A amostra markup também foi inspecionada visualmente.

## Testes, linhagem e limites

- Dez testes focados passaram: métodos/campos, aliases/Source, igualdade,
  JSON/YAML, diagnósticos, markup/Bool/Auto, cópias/mapas, construtor Rust,
  observação e CLI/query com snapshot presente/ausente.
- Seis regressões unitárias foram observadas em RED antes da implementação;
  os encoders também têm RED bilateral. Falhas de fixture foram preservadas:
  Location::from_raw é privado em L2; decode não existe em json/yaml. Foram corrigidas
  para APIs reais, sem modificar esperados para acomodar o código.
- Suíte completa: **{suite['passed']} passaram, {suite['failed']} falharam,
  {suite['ignored']} ignorados**. A tentativa anterior terminou com status 143 durante
  compilação, causa não estabelecida; não foi contada como teste aprovado/reprovado.
- Build offline/locked, rustdoc com -D warnings, all-target check, formato,
  diff check e linhagem V1/V5/V15/V26 estrita passaram, assim como reverse lineage.
- Núcleo raw-field-presence: pin `{manifest['nucleus_pin']}`; nove owners 1:1
  atualizados, resselo restrito. Os bloqueios iniciais de posicionamento de
  metadados foram corrigidos e conservados nos logs.
- Clippy permanece com {checks['clippy_after']} avisos no inventário `--cap-lints warn`;
  isso **não** aprova o gate CI `-D warnings`. Linter completo: zero erros,
  {checks['architecture']['warning']} warnings e {checks['architecture']['note']} notes.
  Uma nova nota V19 aponta o match text/block/lang com três alternativas;
  os três campos são cobertos pelos testes. Não houve aumento de warnings.

CI remoto/release, demais lacunas de produção e transporte de tiling permanecem
abertos. Este lote não certifica o projeto inteiro para produção.

## Evidências preservadas

[evidence.json.gz](raw-presenca-2026-10-14/evidence.json.gz) contém {len(entries)}
entradas com caminho, SHA-256 e bytes em base64, relidas e verificadas. Inclui o
pacote aprovado, recibo, estados, patch, comandos, logs, medições, imagens e scripts
históricos; scripts de mutação não devem ser reaplicados à árvore já corrigida.
'''
(out.with_suffix('.md')).write_text(text)
print(dict(report=str(out.with_suffix('.md')),archive_entries=len(entries),archive_sha256=sha(out/'evidence.json.gz'),suite=suite['passed'],checks=checks))
