from pathlib import Path
import json,gzip,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));plan=json.load(open(out/'projection-plan.json'));assert json.load(open(out/'before-field-probe.json'))['differences']==48;assert json.load(open(out/'before-probe-result.json'))['results'][1]['codes']==[];assert all(Path(p).read_text()==b['files'][p] for p in scope['sources']);n=Path(scope['nuclei'][0]);n.write_text('''tekt = 1
kind = "nucleus"
id = "content-projection-exhaustiveness"
title = "Complementos nominais de projeções Content"

[[claims]]
id = "closed-domain"
level = "must"
statement = "Cada projeção auditada de Content enumera nominalmente os casos tratados e seu complemento atual, para que uma variante futura exija decisão do compilador. Nenhum wildcard de descarte substitui esse domínio fechado."

[[claims]]
id = "uniform-complement"
level = "may"
statement = "Alternativas com a mesma projeção negativa podem compartilhar corpo: false, None, Span::detached ou delegação integral ao hash existente. O owner prescreve a responsabilidade e cobertura de cada grupo V19, sem suprimir a nota."

[[claims]]
id = "guard-refusal"
level = "must"
statement = "Quando um caso positivo tem guard, a recusa conserva o caminho anterior e sua variante aparece no complemento nominal. Campos string desconhecidos permanecem abertos; não confundir esgotamento de Content com enumeração de todos os nomes de campo."

[[claims]]
id = "static-boundary"
level = "must-not"
statement = "Não introduzir registry, vtable, I/O, estado global ou classificador paralelo para substituir os matches estáticos e seus owners. O reparo nominal não altera algoritmos ou contratos públicos; exceções semânticas medidas precisam de prescrição própria no owner."
''');effective=hashlib.sha256(n.read_bytes()+b'\x00TEKT-NUCLEUS-DEPS-V1\x00').hexdigest()
common='''

## V16 — projeções explícitas de Content (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/tekt-v16-content-views-2026-10-17/before.json.gz registra
UTC/status/diff HEAD --stat, fontes/L0 e 2129 hashes conferidos. contexts-before.json
mapeia os seis V16 por AST/linhas; projection-plan.json guarda o domínio real
Content (120 variantes) e os complementos por função. before-probe-result.json
conserva 17280 controles simbólicos de padrões e guards: o mutante Content
futuro compila nos seis locais (zero E0004 selecionados). É mecânica de proteção
de domínio, sem nova prova de paridade ou de leitura por IA. Payloads são
opacos no modelo; ele não substitui controles reais dos algoritmos.

before-field-probe.json usa a rlib atual e o programa field-control.rs com hash,
argv, UTC e source freeze: 48 de 84 leituras diferem quando Source, Located,
ShowRecipe ou Realized estão dentro de Styled/Styled aninhado. Fora de Styled,
os quatro já preservam campos. fields.rs:10–13 reconhece só Equation/Styled;
seu guard recusa os carriers antes da delegação. O núcleo source-origin já
exige transparência; o owner fields já exige body/visual dos outros três.
É invariância de campos da língua, não exigência de estrutura Rust vanilla:
esses carriers são internos e não têm syntax própria no pin. Não inferir
intenção do defeito; refutam a correção perda de precedência, ausência/none,
campos indevidos em não-equações, mudança do visual para original ou execução
de receitas. O pin de linguagem permanece 9f2b6e8715237cb086899a42873660fe744622e8;
esta medição metamórfica não é declarada comparação bilateral nova.

### Responsabilidade exclusiva

{owned}

A lista nominal por função, em ordem da declaração vigente, é derivável das
variantes não cobertas incondicionalmente pelos braços anteriores; guardar
Styled no complemento se o guard puder recusar. Nos campos, variantes com
nome literal ou guard conservam também o caso de campo desconhecido; variantes
já delegadas para qualquer f ficam fora do complemento. O domínio string
continua aberto. O núcleo pinado projection-exhaustiveness partilha esta regra;
o owner continua exclusivo e source-origin conserva sua obrigação vigente.

### Aceitação e limites

L0 antes de testes/código; fluxo contínuo ADR-0127, sem campo/assinatura/trait
público, compatibilidade, default ou fase nova. RED funcional de fields antes
de código, sem usar falhas de montagem de sonda como RED. Após o reparo, os 84
controles reais de carriers devem coincidir; testar precedência de estilos e
não-equações (Realized consulta visual, não original). A sonda estrutural deve
passar de zero para seis E0004 selecionados ao adicionar uma variante somente
no modelo descartável. Seus 17280 casos simbólicos conservam a classificação,
exceto os 96 casos do helper que passam a reconhecer os quatro carriers;
cada variante de cada complemento V19 deve ser exercitada separadamente.
Essa prova não é equivalência de payloads: exige replay dos corpos anteriores,
regressão real e invariância dos algoritmos não alterados. Não adicionar
matriz produtiva que apenas espelhe o match.

Esperado: remover exatamente seis V16, acrescentar seis V19 nominais uniformes
prescritos, preservar os demais diagnósticos por fonte/regra/severidade/mensagem
e não acrescentar V20 ou supressão. Grafo V15/V26 antes de resselo dos cinco
pares. Build, testes focais e workspace, Clippy all-targets -D warnings/-D
unfulfilled_lint_expectations, Rustdoc estrito, fmt, linhagem estrita/reversa,
replay, source freeze e diff --check. Pesados sequenciais MemoryMax12GiB,
High10GiB/swap0/jobs1. Sem commit/push/merge/release. As frentes de linguagem,
contratos pendentes e o objetivo global não são encerrados por este lote.
'''
owned={
'emptiness':'Em Content::is_empty, preservar todos os braços atuais e substituir somente o fallback false por seu complemento nominal de 63 variantes. Essas variantes conservam a classificação estrutural não vazia; não mudar vacuidade de elementos, matemática, texto vazio ou container neste lote.',
'fields':'Em styled_equation, reconhecer Source/ShowRecipe/Located pelo body e Realized pelo visual, recursivamente, sem executar receitas nem mudar Equation/Styled existentes. As outras 114 variantes retornam None nominalmente. Em get_field, manter todos os braços/guards/valores e trocar apenas o wildcard final pelo complemento nominal de 94 variantes com campo desconhecido. Preservar os quatro overrides de equação e a precedência outer-first já vigente; não generalizar campos de equação para Styled de outro tipo. Acrescentar somente dois testes funcionais locais de carriers/precedência/negativos antes do código.',
'morphology':'Em morph_canon, conservar a transformação e toda política de tabela/termi­nação. Substituir somente o None final pelo complemento nominal de 112 variantes, incluindo Styled quando o guard semântico recusa. Não alterar formas canônicas, estilos, source spans, recipes ou walkers.',
'source':'Em source_span, manter Source, TableCell e Table nos braços atuais; as outras 117 variantes retornam Span::detached explicitamente. Não mudar source_body, assentamento de primeira origem ou alcance de span através de outros carriers.',
'rust_hash':'Em Hash::hash, preservar a transparência dos quatro carriers e as especializações MathCases/MathMatrix; as outras 114 variantes delegam nominalmente ao mesmo hash_content(self).hash(state). Não alterar algoritmo, discriminantes, PartialEq, hashes, cache ou identidade da linguagem.'}
for source,owner in scope['owners'].items():
 p=Path(owner);s=p.read_text();needle='Núcleos Tekt:\n';a=s.index(needle)+len(needle);z=s.index('\n\n',a);pins=s[a:z].splitlines()+['- '+str(n)+' sha256:'+effective];s=s[:a]+'\n'.join(sorted(pins))+s[z:];p.write_text(s+common.format(owned=owned[Path(source).stem]))
(out/'l0-record.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),effective=effective,owners={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in scope['owners'].values()}),indent=2)+'\n');print('Five L0 and nucleus prescribed before tests/code')
