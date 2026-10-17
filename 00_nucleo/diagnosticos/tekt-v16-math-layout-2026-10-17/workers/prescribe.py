from pathlib import Path
import json,gzip,hashlib,re,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());variants=json.load(open(out.parent/'tekt-v16-eval-math-2026-10-17/scope.json'))['variants'];frame=json.load(open(out.parent/'tekt-frame-items-2026-10-16/classification.json'))[0];names=list(dict.fromkeys(frame['covered']+frame['complement']));assert len(names)==11;variants['FrameItem']={n:'FrameItem::'+n+' { .. }' for n in names};variants['MathStyleKind']={n:'MathStyleKind::'+n for n in ['Plain','SansSerif','Chancery','Roundhand','Fraktur','Monospace','DoubleStruck','Script','SScript','Display','Inline']}
rows=sorted(json.load(open(out/'all-contexts.json')),key=lambda r:(r['source'],r['pattern_start_byte']));inventory={(r['source'],r['line']):r['enum'] for r in json.load(open(out/'wildcard-inventory.json'))};assert len(rows)==36
for i,r in enumerate(rows,1):
 r['id']='L'+str(i).zfill(2);r['optional_content']=r['source'].endswith('/attach.rs') and r['line']==262;en='Content' if r['optional_content'] else inventory[(r['source'],r['line'])];r['enum']=en;r['unconditional']=[];r['guarded']=[];assert r['pattern_source']=='_'
 for a in r['arms']:
  if a['pattern']=='_':continue
  names=re.findall(r'\b'+en+r'::(\w+)',a['pattern']);assert names and all(n in variants[en] for n in names),(r['id'],a)
  r['guarded' if ' if ' in a['pattern'] else 'unconditional']+=names
 r['complement']=[n for n in variants[en] if n not in r['unconditional']];r['replacement']=' | '.join(variants[en][n] for n in r['complement'])
 r['reason']='Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.'
 if en=='FrameItem':r['reason']='O coletor procura somente a espécie de item explicitada; as demais não são candidatos. Conservar os asserts de presença/quantidade e não confundir None com sucesso de layout.'
 if en=='MathStyleKind':r['reason']='Setters de glyph não alteram MathSize; somente Display/Inline/Script/SScript o fazem. O corpo vazio é o no-op desse eixo independente.'
 if en=='Value':r['reason']='Somente Relative/Ratio/Length representam size nesta fronteira. Os demais conservam None; isso não autoriza descartar erros de validação do produtor.'
 if r['optional_content']:r['reason']='Option None retorna None via ?; Some inspeciona Content normalizado e o complemento mantém None. Não criar padrão profundo Some(Content alternativo).'
 if r['guarded']:r['reason']+=' Variantes sob guard permanecem no complemento para conservar a condição recusada.'
new_source='01_core/src/compiler/math/layout/source_transparency_tests.rs';new_owner='00_nucleo/prompts/compiler/math/layout/source-transparency-tests.md';scope={'sources':b['sources']+[new_source],'owners':b['owners']+[new_owner],'existing_sources':b['sources'],'existing_owners':b['owners'],'new_sources':[new_source],'new_owners':[new_owner],'targets':rows,'variants':variants,'emitted':18,'latent':18,'initial_counts':{'V16:error':170,'V19:note':989,'V20:note':870}}
functional={
'accent.rs':'Normalizar Source por empréstimo nos scrutinees de extract_base_char, one_char e atom_identity. Em dotless_base, transformar o body normalizado e reaplicar a primeira origem com os helpers existentes source_span/with_source_span; conservar explicit, size, dotless, campos, estilos e código dos demais braços. Isso torna i/j dotless também sob Source e conserva uma ocorrência textual base+marcas, sem usar a string para medir desenho.',
'attach.rs':'No início de layout_attach, usar uma referência local base.source_body() e manter layout da base uma vez. No loop de unwrapped_base, normalizar b a cada iteração antes do match; conservar a precedência de wrappers e as condições limits/inline/block atuais. Normalizar o scrutinee dos dois matches sobre MathOp.text e a recursão content_is_text_like. Reescrever somente a projeção extract_char: let c = c?; match c.source_body() com os dois casos atuais e complemento nominal None. Não aceitar Text como novo script elegível, não inferir presença por largura/tinta e não apagar Source do carrier armazenado. Nenhum novo campo/algoritmo de limite ou mudança de fórmulas/kerns/shifts.',
'matrix.rs':'Normalizar cell.source_body() na classificação de split_cell_on_align_point e item.source_body() na detecção do MathAlignPoint. Cada parte derivada conserva sua origem própria ou recebe a origem comprovada da célula pelo helper existente, nunca span fabricado. Fallback conserva cell.clone() inteiro. edge_node inspeciona o body Source por empréstimo para achar a primeira/última folha; assinaturas, células originais, plano, alinhamento, gaps e augment permanecem.',
'spacing.rs':'Normalizar content.source_body() nos predicados is_textual_spaced_item/is_bare_pipe e cada child.source_body() na elegibilidade de Sequence textual. Não mudar classes, regras de gap, prioridade zero explícito, promoção Vary, fatores ou tamanho de fonte. Restabelecer somente a classificação que Source não pode alterar.',
'sequence_composition.rs':'Normalizar Source nas leituras já existentes de AlignPoint/Linebreak, espaço fraco, próximo irmão material e MathText pontuacional. Na vista pontuacional derivada, reaplicar a origem comprovada pelo helper existente. A vista de spacing reconhece também Source(Text) e transporta sua origem ao produzir o MathText transitório. Não modificar os nós autorais, remover espaço explícito ou reutilizar texto extraível como geometria. Conservar a distinção de whitespace terminal sem material seguinte e espaço fraco entre materiais.',
'stretch.rs':'Normalizar Source na elegibilidade leaf_char; conservar os casts e a seleção de eixo/variante/assembly/size. Isso não amplia a antiga elegibilidade leaf-only para corpos compostos nem redefine o contrato completo de native axis: o débito separado math-stretch-composite continua aberto e deve completar anexos/composições com métricas e medição próprias. Não ratificar esse fallback parcial como default normativo ou prometer paridade completa.',
'underover.rs':'Normalizar Source nos scrutinees de piece_char, nested_spreader, base_char_for_attachment e na classificação da base de layout_underover_with_attach. Preservar seleção de suporte/anotação, identidade pública, estilos, fórmulas de attachment/limites e translação. Um suporte Source de um caractere não vira anotação em tamanho de script.',
'mod.rs':'Registrar apenas o módulo privado source_transparency_tests, com a condição interna #![cfg(test)] no novo consumer e owner exclusivo, conforme a composição existente. No código produtivo, normalizar transformed.source_body() em layout_equation_measured para preservar line_count/first/last baseline; normalizar node e next em should_tighten_text_frame e content em single_math_leaf_char. flatten_math_sequence_nodes classifica node/children pelo body Source, preserva a Sequence de markup atômica e abre só sequências técnicas já elegíveis; todo fragmento derivado mantém origem própria ou a primeira origem comprovada do container por source_span/with_source_span. Não clonar fragmentos já originados nem acrescentar trabalho de reoriginação no caminho sem span. Preservar a transformação de glyph uma vez, baselines, callbacks, contexto, árvore autoral, API e fases.'}
base=f"HEAD `{b['head']}`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `00_nucleo/diagnosticos/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes."
measurement='''source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.'''
accept='''L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.'''
receipt={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'owners':{},'code_unchanged':True}
for source,owner in zip(b['sources'],b['owners']):
 p=Path(owner);s=p.read_text();assert s==b['files'][owner];name=Path(source).name;target_rows=[r for r in rows if r['source']==source];append='\n\n## Exaustividade nominal e origem matemática — 2026-10-17\n\n### Medição anterior à decisão\n\n'+base+'\n\n'+measurement+'\n\n### Prescrição exclusiva\n\n'
 if name in functional:
  append+=functional[name]+'\n'
  pin='- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67'
  if pin not in s:
   if 'Núcleos Tekt:\n' in s:s=s.replace('Núcleos Tekt:\n','Núcleos Tekt:\n'+pin+'\n',1)
   else:s=s.replace(s.splitlines()[1],s.splitlines()[1]+'\n\nNúcleos Tekt:\n'+pin,1)
 else:append+='Manutenção somente do helper/teste ou do eixo MathSize descrito abaixo. Não ampliar travessia ou reescrever os cenários existentes deste owner.\n'
 append+='\nTrocar os fallbacks enumerados pelo complemento nominal atual dos braços **incondicionais**, sem alterar corpo/guard/ordem; variantes guardadas continuam no complemento para quando a condição falha. A projeção Option de attach é a única exceção estrutural: None retorna por ?, Some passa ao match nominal de Content normalizado, sem padrão profundo.\n'
 for r in target_rows:append+=f"\n- **{r['id']}** — `{r['source']}:{r['line']}`, `{r['function']}`, {r['enum']}: excluir somente `{', '.join(r['unconditional']) or '(nenhuma variante inteira)'}`; guardados `{', '.join(r['guarded']) or '(nenhum)'}`. {r['reason']}\n"
 append+='\n### Aceitação e limites\n\n'+accept+'\n';p.write_text(s+append);receipt['owners'][owner]=p.read_text()
p=Path(new_owner);assert not p.exists();p.write_text('''# L0 — math/layout/source-transparency-tests
Hash do Código: 00000000

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/math/layout/source_transparency_tests.rs`

## Medição anterior à decisão

'''+base+'\n\n'+measurement+'''

## Contrato exclusivo

Possuir apenas regressões puras de transparência Source para o layout matemático.
Usar MathLayouter pela entrada pública e FontMetrics determinístico, com advance,
arestas, TopAccentAttachment, kerns assinados e variantes horizontais declaradas.
Não carregar fonte, World, relógio, arquivo ou rede. Manter a fixture atribuível;
ela não demonstra seleção de fonte real ou paridade de exportação inteira.

Os 23 pares da análise suplementar são casos separados. Comparar geometria
(width/ascent/descent, baselines, posições e tamanhos de texto/glyph) com 1e-9,
texto/caractere observável e identidade extraível por ocorrência, sem exigir bytes,
GID, pointer, formato privado de frame ou igualdade Rust de Content. O observador
conhece Text/TextShaped/Glyph, TextReplacement e Semantic; os seis demais tipos
atuais de FrameItem são Unknown e falham explicitamente, sem fabricar observação.
Text/TextShaped partilham observação visual; os seis tipos recusados partilham
a mesma falha, justificando só esses dois grupos V19. Não ignorar item desconhecido.

Conservar controles positivos independentes: limits inline empilha e scripts fica
lateral; o mock fornece variantes extensíveis com avanços distintos; a matriz
retém alinhamento de coluna por ampersand; espaço fraco interno permanece e nas
bordas colapsa. Source não altera identidade extraível do acento atômico ou do
acento aninhado, nem dotless i/j. Nunca usar identity text para medir glyphs.

Adicionar controle de equação display com duas linhas: Source conserva line_count
2 e primeira/última baseline. Adicionar controle de flatten: cada fragmento retém
origem própria ou recebe origem comprovada do container, sem substituir origem
já presente e sem alterar a árvore autoral. Um controle negativo de observador
com TextSeparator deve falhar Unknown. A transformação dotless terá seu teste
causal e de origem co-localizado no owner de accent; não exportar helper só para
este consumer nem compartilhar propriedade do código produtivo.

## Aceitação

'''+accept+'\n');receipt['owners'][new_owner]=p.read_text();receipt['code_unchanged']=all(sha(p)==h for p,h in b['hashes'].items() if p not in scope['owners']);assert receipt['code_unchanged'];(out/'scope.json').write_text(json.dumps(scope,ensure_ascii=False,indent=2)+'\n');(out/'l0-before-code.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'L0_first':True,'owners':len(scope['owners']),'original_decisions':len(rows),'emitted':18,'functional_source_pairs':23,'code_changed':False}))
