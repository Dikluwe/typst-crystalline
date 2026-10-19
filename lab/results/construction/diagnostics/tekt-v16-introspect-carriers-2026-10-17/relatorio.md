# Relatório — introspecção I1: carriers, rótulos e ações

Etapa concluída. V16 **109 → 107**, V19 **1094 → 1096 notas**, V20 **873 → 873 notas**. Foram corrigidas duas perdas de informação causadas por Source/carriers, sem silenciar o débito da travessia bibliográfica restante.

## Alterações

A coleta de chaves e a conversão de referências visitam Source, ShowRecipe e Located pelo body e Realized pelo visual. A reconstrução modifica só esse campo, preservando span, Location, original e captura de recipe. A coleta não chama Dynamic; a conversão mantém o comportamento Dynamic já existente. Não executa recipes. O fast path sem chaves permanece idêntico.

compute_labelled reconhece Source por delegação ao mesmo helper com Introspector, Location e idioma intactos. Heading, Equation block e Figure mantêm sua projeção direta. Styled não é desmontado; outros alvos continuam sem associação inventada. Esse texto é compatibilidade e não substitui o caminho numérico autoritativo de referência.

O fallback de label tem 117 variantes nominais, incluindo Equation quando block recusa. O match de ação automática de ElementPayload tem 17 alternativas negativas explícitas (12 variantes e as recusas bool de Heading/Figure/Table/Equation). Os corpos ativos, profundidade, eventos, stores, callbacks e ordem não mudaram. As duas notas V19 permanecem visíveis e prescritas nos L0.

## Limite explícito I1/I2

A bibliografia continua **incompleta** para outros containers estáticos. I2-bibliography-static-coverage completará sua auditoria e travessia (células raw, headers/footers, Repeat/Columns e demais corpos). O wildcard de collect_bib_keys permanece V16 e o passthrough legado de convert_refs continua registrado como dívida; não foram chamados de terminais legítimos nem silenciados. O L0 inicial declara essa divisão. Este lote não redefine o objetivo global nem certifica bibliografia/referências universais.

## Evidência

- Sonda sobre tipos reais: o algoritmo original reconstruído perde a ordem nos **12 cenários**; o código vigente preserva **k2,k1 nos 12**, com verificação adicional de citation_order público. São quatro carriers em três posições: raiz, bibliografia e referências. O baseline executa os algoritmos originais extraídos do snapshot, conforme a metodologia abaixo; não é uma medição da API pública antiga na rlib atual. Os artefatos before/after-citation-probe.json conservam programa/rlib/source freeze com hashes, argv, UTC e stdout. A rlib medida permaneceu idêntica até o fechamento.
- RED funcional: dois testes falharam e um controle passou antes da implementação. Source perdia Seção 1, Equação (1) e Figura 1 no mesmo Location; as três projeções agora coincidem com o alvo direto. Os controles preservam original oculto de Realized, referência desconhecida, dados de carrier e limites de Styled.
- Os 290 controles simbólicos mantêm a classificação, salvo os dois casos Source do modelo de label. O modelo mantém nomes/formas de enums e condições bool, com payloads opacos; não é prova de seus valores ou paridade. A variante fictícia passa de zero para dois E0004. Replay exato dos corpos e regressão real completam a prova.
- Workspace: **8926 aprovados, 0 falhas, 7 ignorados**. Build, fmt, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito/quatro índices, V1/V5/V15/V26, reverse lineage, replay e diff --check passaram. SARIF preservou o V16 bibliográfico e todos os demais diagnósticos por fonte/regra/severidade/mensagem; sem V20 novo.
- Recursos sequenciais: MemoryMax12GiB/High10GiB/swap0/jobs1. Pico **10.004955 GiB** (10742738944 bytes), eventos {'low': 0, 'high': 193, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}. Sem OOM/OOM kill.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Baseline `2026-10-17T19:27:59.007463+00:00`, fechamento `2026-10-17T19:56:26.852157+00:00`. before.json.gz registra textos/L0/status/diff HEAD --stat e 2130 hashes conferidos; final-state.json.gz congela o estado final. Três pares source/L0, sem campo/assinatura pública, default, fase ou dependência nova. O pin source-origin foi consumido nos dois owners pertinentes sem alterar o núcleo. L0 antes de testes/código, fluxo interno ADR-0127. Linter 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA-256 e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2.

A transparência de carriers é uma prova metamórfica exigida pelo contrato já ratificado; esses carriers não têm syntax vanilla. Não se inventa igualdade de representação Rust. O alvo de linguagem permanece o commit 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

O primeiro GREEN identificou uma fixture Located inválida: duas referências distintas recebiam o mesmo Location. A identidade produtiva foi preservada; o teste passou a usar um Locator por árvore com prefixo reservado. O baseline válido executa os algoritmos originais extraídos de before.json.gz sobre tipos reais da rlib, com adaptação apenas de imports; o observer estrito de Cite confere sua ordem e o GREEN também confere citation_order público. Essa reconstrução é distinguida da medição pública inicial arquivada em located-fixture-attempt.

Uma montagem inicial da sonda cortou os campos de Figure numa chave de doc comment; E0026 foi corrigido antes de contar controles, sem classificá-lo como RED funcional (probe-assembly-attempt). Três imports de fixture irrelevantes foram retirados conforme o gate Clippy, sem alterar assertivas; o replay inclui esse ajuste. Os testes não foram relaxados.

O inventário lista os 107 V16 restantes e follow-ups.md preserva I2 e as demais frentes. Pacotes TS1 aprovado e RG1 pendente conferidos pela entrega. Objetivo global ativo; sem commit/push/merge/release.
