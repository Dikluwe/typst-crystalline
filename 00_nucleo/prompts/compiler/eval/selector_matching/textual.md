# L0 — compiler/eval/selector_matching/textual
Hash do Código: c34e3678

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/compiler/eval/selector_matching.rs:370-481`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/selector_matching/textual.rs`. Casar seletores textuais e produzir intervalos e substituições para Text e Regex, preservando offsets UTF-8, ordem e a exclusão de matches vazios. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.


## Seletores de show — exaustividade e agrupamento, 2026-10-16

### Medição anterior à decisão

01_core/src/compiler/eval/selector_matching/textual.rs:13 (is_text_rule, latent-predicate); 01_core/src/compiler/eval/selector_matching/textual.rs:26 (textual_selector_matches, emitted-v16); 01_core/src/compiler/eval/selector_matching/textual.rs:54 (textual_match_ranges, emitted-v16). O enum em 01_core/src/entities/show.rs:63-96 contém Text, NodeKind, NativeElement, DynKind, Regex, Where, And, Or e Label. Fonte/linhas e funções atuais foram confrontadas com os avisos SARIF da revisão pinada e com o inventário AST anterior, verificando presença literal dos matches na árvore atual. Proveniência: lab/results/construction/diagnostics/tekt-v16-show-selectors-2026-10-16/before.json.gz conserva HEAD, working tree não commitado, UTC, hashes/fontes, status e git diff HEAD --stat; classification.json distingue avisos emitidos de predicado latente. Classificação: padrões/exaustividade são mecânica; seleção, texto, efeitos e erro de linguagem permanecem conforme os contratos vigentes. Não há nova medição de paridade ou inferência de ganho de leitura/performance.

### Decisão interna

is_text_rule torna o predicado matches! existente em match explícito: Text/Regex true, NodeKind/NativeElement/DynKind/Where/And/Or/Label false. Isso fecha uma decisão latente que V16 não emite. textual_selector_matches usa esse mesmo grupo não textual no fallback false, conservando corpos Text e Regex. textual_match_ranges usa grupo Text/NodeKind/NativeElement/DynKind/Where/And/Or/Label vazio: incluir Text porque o guard anterior falha no padrão vazio. Não mudar o guard, offsets, ordem, coleta nem funções de splice. Teste S2: nove variantes, texto UTF-8 com duas ocorrências nos offsets [0,3] e [4,7], misses, Text vazio e Regex de match vazio; Where/And/Or textuais não se tornam regras textuais dedicadas.

Alternativas com o mesmo resultado conservam um corpo compartilhado. Grupos explícitos informam a política vigente e exigem decisão quando Selector cresce; não duplicar corpos para zerar V19. V19 permanece informativo, sem afirmar déficit de cobertura ou fator de multiplicação de testes. Guard e políticas recursivas merecem controles próprios; uma execução do corpo não prova todas as alternativas. Nenhum allow/expect, exceção, ajuste de severidade ou helper genérico para esconder V16.

### Aceitação e limites

Fluxo contínuo ADR-0127: nenhuma assinatura/campo/trait público, default de produto, compatibilidade ou fase nova. Não reabrir propostas históricas deste owner. L0 antes dos testes/código e resselo restrito bilateral. Escrever os controles no módulo e confirmar o comportamento anterior; o RED estrutural é a ausência de E0004 nos seis pontos na cópia real do core com uma variante sentinela acrescentada apenas ao enum copiado. Após nominalização, os seis pontos devem exigir E0004 nessa cópia, sem stubs nem mutação da fonte produtiva. Os cinco V16 selecionados devem desaparecer da emissão, preservando diagnósticos externos e notas V19. Reexecutar testes reais, build, formato, Clippy estrito, Rustdoc e linhagem direta/reversa sob 12 GiB, swap zero e um job Cargo. Não declarar gate global verde enquanto V16 externos permanecerem.
