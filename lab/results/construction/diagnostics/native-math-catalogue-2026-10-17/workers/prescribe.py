from pathlib import Path
import json,gzip,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));cases=json.load(open(out/'batch-schema.json'))['cases'];r=json.load(gzip.open(out/'before-batch.json.gz'));assert r['sources_unchanged'];assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in b['hashes'].items());branches=[c for c in cases if c['new_bridge']]
measure='''

## Identidade matemática nativa — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado; pin `9f2b6e8715237cb086899a42873660fe744622e8`.
`00_nucleo/diagnosticos/native-math-catalogue-2026-10-17/` fixa o estado
exato em before.json.gz: UTC, diff HEAD --stat e hashes de todos os consumers,
L0 e configuração protegidos. before-measurement.json.gz é reutilização
verificada da matriz final da etapa anterior: fontes e binário Tekt têm
identidade de hash com o baseline atual (baseline-reuse.json), não uma nova
execução. A matriz cobre os oito perfis de features e 3956 casos de funções;
26 rotas math ainda são recusadas em where/kind pelo Tekt e aceitas pelo pin.
before-batch.json.gz mede 227 casos detalhados de campos, exclusões, alias
parcial e counter, com comandos/binários/saídas e seus hashes.

batch-schema.json conserva blocos e SHA-256 das fontes primárias do pin,
file:line e declarações de ABI do Tekt; typst-macros/src/elem.rs:410–421
exclui internal/external de field_id. As anotações elem destas 26 funções
contêm Mathy e não Locatable. Isto é semântica/forma pública da linguagem,
não igualdade mecânica de Func nem inferência de intenção por render.

### Responsabilidade exclusiva e decisão

'''
bridge_rows='\n'.join('- `'+c['module']+'::'+c['actual_callback']+'`'+(' como `'+c['callback']+'`' if c['callback']!=c['actual_callback'] else '') for c in branches)
texts={
'01_core/src/compiler/stdlib/structural/math.rs':'''Este owner conserva a montagem estática existente e acrescenta somente
reexports diretos, limitados a `pub(in crate::compiler)`, das nativas abaixo.
Nenhum wrapper/algoritmo/schema é movido ao hub, nenhum binding é adicionado.
O alias interno class_for_metadata evita tocar o reexport público condicional
de teste já existente no parent. Os nomes completos efetivos são os da lista:

'''+bridge_rows+'''

Accent/cancel e mat já têm pontes próprias e não são duplicados aqui.
''',
'01_core/src/compiler/stdlib/structural/mod.rs':'''Este owner propaga diretamente as 23 pontes internas declaradas no owner
`00_nucleo/prompts/compiler/stdlib/structural/math.md`, de `math` ao hub
estrutural, com `pub(in crate::compiler)`. Reexportar os mesmos símbolos,
sem wrappers, schema local, lookup por nome ou mudança nos reexports públicos
atuais, inclusive o native_math_class público somente em cfg(test).
''',
'01_core/src/compiler/stdlib/mod.rs':'''A fachada propaga diretamente as 23 pontes internas declaradas no owner
`00_nucleo/prompts/compiler/stdlib/structural/math.md`, de `structural` a
`compiler::stdlib`, com `pub(in crate::compiler)`. Não copiar schema/validação,
criar wrapper, alterar bindings, mudar API externa ou resolver a divergência
histórica de visibilidade de native_flush neste lote.
''',
'01_core/src/compiler/stdlib/structural/math/limits.rs':'''Somente a visibilidade Rust interna de native_math_scripts e
native_math_limits passa de pub(super) para pub(in crate::compiler), para
permitir reexport direto pelo parent. Parâmetros, retorno, corpos, diagnósticos,
ordem de validação e presença explícita permanecem literalmente preservados.
Nenhuma das duas funções vira API externa; nenhuma assinatura pública é
adicionada/modificada. Este adendo autoriza essas duas novas pontes internas,
preservando as pontes existentes descritas no contrato anterior.
'''}
rows='\n'.join('| `'+c['callback']+'` | `'+c['name']+'` | '+', '.join('`'+f+'`' for f in c['fields'])+' | `'+c['source']+':'+str(c['line'])+'` |' for c in cases)
texts[scope['sources'][0]]='''Completar a tabela estática native_element_fields com os 26 callbacks
abaixo, usando fn_addr_eq tipada à ABI existente de quatro argumentos. Todas
as entradas têm locatable=false. Reconhecer a mesma função sob alias; não
usar Func.name/PartialEq, tipo de Content retornado, strings de registry nem
execução do constructor. A tabela é exclusiva deste owner; as fachadas só
transportam os callbacks originais por reexport interno direto.

| Callback | Nome público | Campos selecionáveis, na ordem do pin | Fonte no pin |
|---|---|---|---|
'''+rows+'''

Excluir num-deparenthesized e denom-deparenthesized internos de frac e gap
external de mat; row-gap/column-gap são selecionáveis. Não retirar o parâmetro
constructor gap de mat: parâmetro externo e campo de seleção são superfícies
diferentes. Label permanece campo comum; filtros preservam valores sem cast.
Preservar page/math.op com engine, equation, as quinze identidades do lote
anterior e todas as entradas históricas. Math underline/overline continuam
identidades diferentes das funções globais homônimas.
'''
accept='''
### Aceitação e limites

Correção interna de tabela/composição em fluxo contínuo ADR-0127. Nenhum campo,
trait, assinatura Rust externa, compatibilidade, default ou fase muda. L0
antes dos testes/código; RED funcional por identidade ausente, GREEN dos 26
casos com filtros e exclusões; controles de With, homônimo e helpers que não
são elementos. Validar antes/depois toda a matriz, preservando observáveis
fora das identidades autorizadas e aliases efetivos (accent/cancel globais
cristalinos incluídos, sem crédito de paridade se não existem no pin).

O sucesso de where/kind não certifica getters, síntese, indexação, show,
layout ou counter. Os controles counter permanecem explícitos e a sua
admissão separada não é alterada neste lote. As quinze rotas restantes de
formatos/HTML/PDF têm investigação própria; a colisão de callback dos formatos
não é resolvida por nomes. FG1/RM1/SY1/RG1 preservam escopos e gates.

Verificar ownership/V26 antes do resselo restrito aos cinco pares; build,
suíte completa, fmt, Clippy/Rustdoc estritos, linhagem e diagnóstico global
sem nova violação, sob MemoryMax12GiB/High10GiB/swap0/jobs1 sequenciais.
'''
for source,body in texts.items():
 p=Path(scope['owners'][source]);p.write_text(p.read_text()+measure+body+accept)
scope['mode']='26 existing math native metadata, 23 internal direct bridges and two internal visibility widenings only; ADR-0127 continuous';(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n');(out/'l0-before-tests.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),owners={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in scope['owners'].values()},sources_still_baseline=all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==b['hashes'][p] for p in scope['sources'])),indent=2)+'\n')
print('Prescribed five owners, bridges',len(branches))
