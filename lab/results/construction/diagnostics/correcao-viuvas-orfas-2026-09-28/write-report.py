from pathlib import Path
import json,hashlib
P=Path('00_nucleo/diagnosticos/correcao-viuvas-orfas-2026-09-28');root=Path.cwd();r=json.loads((P/'final-receipt.json').read_text());out=P.with_suffix('.md')
def link(label,path):return '['+label+']('+str(root/path)+')'
t='''# Proteção regional de viúvas e órfãs — 2026-09-28

Viúvas e órfãs passam a influenciar a admissão das linhas nos parágrafos preparados com métricas verticais uniformes e páginas de largura constante. Os parâmetros positivos ativam proteção; zero e negativos desativam. A implementação usa os custos do escopo compartilhado correto, preserva overrides/restauração e não altera as quebras já selecionadas.

## Medição antes da decisão

O estado anterior transportava e validava `widow/orphan`, mas os emissores não reservavam espaço para grupos de linhas. Os L0 e o núcleo de custos declaravam explicitamente a paginação fora do escopo. Essa limitação contratual legitimava o adiamento; a inspeção não permite atribuir motivação aos autores.

O pin vanilla continua `9f2b6e8715237cb086899a42873660fe744622e8`. A fonte `lab/typst-original/crates/typst-layout/src/flow/collect.rs:196-229` usa comparação estrita com zero, e não penalidades graduais. Protege as primeiras duas linhas por orphan; protege as últimas duas por widow quando o índice da penúltima é pelo menos dois; com exatamente três linhas e ambas as proteções, agrupa as três. A fonte `flow/distribute.rs:343-364` só avança um grupo por essa necessidade quando cabe na próxima região. Não copiamos as estruturas Frame/Child do upstream como obrigação mecânica.

Foram medidas 102 entradas antes de implementar, com Noto Sans versionada, dimensões e parâmetros explícitos: BEFORE e EXTRA. A fonte e os binários anteriores estão identificados em INITIAL. No controle `page-65-100%-0%-False`, o Tekt deixava três linhas e uma, enquanto o vanilla distribuía duas e duas. Custos zero coincidiam. Os controles de três linhas, grupo maior que a região, leading, bottom-edge, configuração padrão, constructor, show-set, inline e restauração distinguem os caminhos relevantes.

Quatro controles adicionais de escopo positivo no primeiro/no meio do fluxo foram acrescentados depois da implementação, sem medição Tekt anterior: ADDITIONAL. Não são contados como melhorias com baseline. Os resultados finais dos 106 inputs estão em AFTER e AFTER_EXTRA. A comparação usa palavras, caixas e páginas completas com tolerância de 0,0005 pt; grupos XML do extrator não são contados como linhas físicas.

## Correção e fronteiras de responsabilidade

O novo owner privado PAGINATION concentra certificação vertical, agrupamento e admissão, sob L0_PAGINATION. Calcula a extensão necessária com arestas e leading recebidos das métricas. Só avança se o grupo não cabe na região atual, existe material nela e ele cabe na próxima página, cujas margens são resolvidas pelo mecanismo existente. Um grupo maior que a página conserva o progresso ordinário: não gera loop ou página branca artificial.

O emissor consulta essa admissão antes de emitir cada linha escolhida. Não refaz seleção nem acrescenta justificação. `text_costs::prevention` resolve os dois controles separadamente, reutilizando a tabela canônica de campos. Os valores dois/três e o teste positivo são regras medidas na fonte pinada, não números calibrados pelas sondas. Não há novos campos, traits, flags, assinaturas públicas ou fase de pipeline; o fluxo contínuo ADR-0127 aplicou L0 antes de código, RED→GREEN e resselo restrito.

A estratégia implícita permite a preparação simples necessária à proteção padrão dos parágrafos com duas ou mais linhas. Parágrafos de uma linha mantêm o percurso anterior quando não há seleção explicitamente solicitada. Wrappers integrais Styled entram normalmente pelo owner de estilos antes da tentativa: não são promovidos a partir de glyphs. A iteração reabilita o planejamento na fronteira explícita anterior a Parbreak/Pagebreak, com linha atual sem material; prefixos inline já materiais continuam bloqueados. Cada parágrafo precisa ser certificado inteiro antes de sua emissão.

Os invariantes de custos continuam compartilhados por núcleo pinado. Há 14 pares L0/consumer e um núcleo alterados, totalizando 29 arquivos produtivos/L0 nesta rodada. REVIEW não encontrou alteração produtiva/L0 fora do escopo. Alterações anteriores extensas da working tree foram preservadas.

## Refutadores e correções intermediárias

O RED inicial está em `red.log`. O controle FixedMetrics de duas linhas tinha altura 45 pt e não distinguia proteção: a observação mostrou baselines 28,6 e 43,45 pt. A fixture passou a usar 40 pt, sem mudar a distribuição esperada; `fixture-red.log` confirma falha com o emissor anterior, e `green.log` registra os cinco controles novos aprovados. Não tratamos a expectativa inicial equivocada como defeito do produto.

A primeira ampliação do planejamento padrão falhou em três testes existentes de invisibilidade de Metadata/State/Func (`core-first.log`). O ajuste restringiu a ampliação aos parágrafos com múltiplas linhas, conservando as expectativas dos testes.

FIRST preserva 14 falhas da primeira comparação completa: a tentativa anterior à entrada do wrapper integral lia custos/métricas exteriores. A tentativa passou a ser deferida para o escopo correto. SECOND preserva quatro falhas restantes: os dois controles positive-first adicionais e as regressões costs/restore e overhang/restore. A reentrada só diante de Pagebreak era insuficiente; a fronteira Parbreak resolveu o primeiro parágrafo, mantendo prefixos e conteúdo opaco protegidos.

O selo de Sequence também expôs uma diferença entre o script de resselo e a metadata canônica: `Hash do Código` estava depois da primeira linha em branco. O linter preservava essa linha como corpo do prompt. A metadata foi movida para o cabeçalho e o escopo foi resselado; CANONICAL registra a fonte do algoritmo do linter. Não foi usado fix global.

## Validação final e proveniência

Estado: HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitada. RECEIPT registra UTC TIME, status completo incluindo untracked, `git diff HEAD --stat`, SHA-256 de todas as fontes produtivas/L0 e artefatos. As fontes finais são idênticas durante build, testes, lint e as sondas finais.

| Verificação | Resultado |
|---|---|
| Core | CORE passaram; zero falhas; dois ignorados |
| Infra | INFRA passaram; zero falhas |
| Build dev/release | passaram |
| Crystalline lint | zero erros; WARN warnings; NOTES notes |
| V5/V6/V15/V26 nos owners afetados | nenhuma |
| Entradas bilaterais | INPUTS: 106 novas + 295 regressões |
| Asserções | CHECKS passaram; zero falhas |
| Novos controles admitidos | AGREEMENTS geometrias coincidentes |
| Melhorias com baseline anterior | IMPROVEMENTS novas geometrias; CLOSED regressão antes aberta agora coincidente |

ACCEPTANCE enumera cada obrigação. Casos anteriormente abertos no corpus continuam sendo controles de preservação, exceto a geometria `regression-composition/paragraph-pages`, agora coincidente no input medido; isso não fecha todos os contextos de paginação.

Oráculo: árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário `lab/typst-original/target/release/typst`, SHA-256 `VANILLA`. Tekt final: `target/release/typst`, SHA-256 `TEKT`. Tekt anterior: `5fd64d361e3c4c387cbf9149660e9e7f7c624f8aeabaf46bbba8d5a2c02d2d98`. Não se usa string de versão como identidade; não houve commit ou publicação nesta rodada.

## Limites atuais

A proteção não está certificada para colunas/células/subframes, floats, notas pendentes, transações regionais ativas, métricas verticais misturadas, baseline offsets, ou o conteúdo recusado pela preparação global. Altura automática não exige avanço por proteção. Os cinco controles `footnote`, `columns`, `tracking`, `mixed-size` e `linebreak` conservam o comportamento anterior; os dois últimos já coincidem na geometria medida, mas não ampliam a certificação do algoritmo. Notas/colunas/tracking continuam divergências conhecidas, sem regressão nesta rodada.

O aviso de set text(costs:) agora declara suporte limitado, em vez de ausência total de implementação. Não foi criada API de Sink no constructor. Custos não finitos e desempenho não foram certificados. Texto copiado do hífen discricionário no PDF, Unicode completo, estilos dentro da palavra, admissão de Par aninhado e show par funcional permanecem frentes próprias. Não há alegação de estado da arte.
'''
replacements={
 'BEFORE':link('before-pdf',P/'before-pdf/proveniencia.json'),'EXTRA':link('before-extra',P/'before-extra/proveniencia.json'),'INITIAL':link('initial.json',P/'initial.json'),'ADDITIONAL':link('additional-scopes',P/'additional-scopes/proveniencia.json'),'AFTER_EXTRA':link('after-extra',P/'after-extra/proveniencia.json'),'AFTER':link('after-pdf',P/'after-pdf/proveniencia.json'),'L0_PAGINATION':link('L0 paragraph_pagination','00_nucleo/prompts/compiler/layout/paragraph_pagination.md'),'PAGINATION':link('paragraph_pagination.rs','01_core/src/compiler/layout/paragraph_pagination.rs'),'REVIEW':link('scope-review',P/'scope-review.json'),'FIRST':link('first-round/acceptance',P/'first-round/acceptance.json'),'SECOND':link('second-round/acceptance',P/'second-round/acceptance.json'),'CANONICAL':link('seal-canonicalization',P/'seal-canonicalization.md'),'RECEIPT':link('recibo final',P/'final-receipt.json'),'ACCEPTANCE':link('acceptance.json',P/'acceptance.json'),'TIME':r['utc'],'CORE':f"{r['tests']['core']['passed']:,}".replace(',','.'),'INFRA':f"{r['tests']['infra']['passed']:,}".replace(',','.'),'WARN':str(r['lint']['warnings']),'NOTES':f"{r['lint']['notes']:,}".replace(',','.'),'INPUTS':str(r['bilateral_inputs']),'CHECKS':f"{r['checks']:,}".replace(',','.'),'AGREEMENTS':str(r['geometry_agreements_certified_new_controls']),'IMPROVEMENTS':str(len(r['geometry_improvements_with_baseline'])),'CLOSED':str(len(r['regression_geometry_newly_matching'])),'VANILLA':r['binary_sha256']['vanilla'],'TEKT':r['binary_sha256']['tekt']}
# Replace longer tokens first; placeholders are never substituted inside generated links.
import re
t=re.sub('|'.join(sorted(replacements,key=len,reverse=True)),lambda m:replacements[m[0]],t);out.write_text(t)
r['report_sha256']=hashlib.sha256(out.read_bytes()).hexdigest();(P/'final-receipt.json').write_text(json.dumps(r,ensure_ascii=False,indent=2)+'\n');print(str(out.resolve()))
