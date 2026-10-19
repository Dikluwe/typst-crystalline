# Relatório — sintaxe de import/include e contexto herdado

Etapa concluída. Fontes importadas e incluídas são validadas antes da avaliação: sintaxe inválida produz diagnóstico causal em vez de panic ou execução parcial. Módulos importados conservam as features e o target do chamador.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Baseline `2026-10-17T17:50:11.339184+00:00`, fechamento `2026-10-17T18:33:31.215338+00:00`. before.json.gz, red-state.json.gz, validation-result.json e final-state.json.gz preservam fontes/L0, hashes, UTC, status e git diff HEAD --stat. Os 2125 hashes da etapa anterior foram conferidos no bootstrap; 2128 estão congelados no fechamento. Arquivos de sondas preservam argv, fixtures, saídas completas e SHA-256 dos binários. Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

## Alterações e L0

Quatro pares consumer/L0: compiler/eval/mod.rs, modules.rs, novo syntax_diagnostics.rs e tests.rs para a expectativa legada. O núcleo eval/source-validation.toml está pinado nos quatro owners. L0 foi escrito antes dos testes e da implementação; correção interna/de paridade em fluxo ADR-0127. Assinaturas, reexports e definições públicas foram comparados com o baseline; sem nova API Rust pública, default ou mudança de fase. Integridade dos arquivos fora do escopo foi conferida.

O helper puro converte erros sintáticos em SourceDiagnostic, classifica explicitamente os cinco SyntaxErrorKind existentes e mantém mensagens e todos os hints em ordem. NoTextWithinStars/Underscores continuam avisos não fatais, com span do markup delimitador e deduplicação pelo Sink existente. Documento principal, expressão, import e include compartilham essa política; nenhum catch_unwind produtivo foi adicionado.

Import e include validam a fonte depois da resolução e da verificação de ciclo, antes de executar código. Erros preservam a fonte real e acumulam a trilha de importação/inclusão aninhada. O contexto local do import herda full_error, features e target; escopo, estilos e show-rules continuam isolados conforme o contrato anterior. A construção de documento em módulo importado passa a funcionar no target Bundle, sem habilitá-la nos targets Paged/Html.

## Validação

- RED funcional: os oito testes novos falharam antes da implementação, sem erro de compilação. GREEN cobre sintaxe inválida nos três modos de import, include string/path, fontes de pacote em World de memória, aninhamento, prioridade do erro sintático sobre efeitos, avisos/deduplicação, oito perfis de features, três targets e erro de execução com trace.
- Corpus bilateral: 58 casos principais e 25 de contexto, total 83. Classificação observada: {'language-parity': 56, 'explicit-residual': 12, 'disabled-feature-rejection-parity': 14, 'bundle-html-semantic-parity': 1}; 73 resultados Tekt mudaram. Resultados brutos e lista individual em observables-result.json. Os 12 resíduos são três classes: diagnóstico de string não fechada (cinco rotas), diagnóstico de hexadecimal inválido (cinco rotas) e morfologia de include válido (duas rotas). Rejeição com feature desligada é contabilizada à parte da execução com feature ligada.
- Comparação mantém mensagens, hints, spans e traces completos; desconta apenas o aviso Tekt preexistente de arquivo vazio por literal exato. Diferenças residuais de parser/morfologia permanecem explícitas. Bundle compara presença de child.html e todos os eventos HTML (elementos, atributos e texto); bytes e hashes das saídas também são preservados como evidência auxiliar.
- Suíte workspace: 8921 aprovados, 0 falhas, 7 ignorados. Build, formato, Clippy all-targets estrito, Rustdoc estrito/quatro índices, V1/V5/V15/V26, reverse lineage e diff --check passaram.
- Tekt global: {'V16:error': 117, 'V19:note': 1086, 'V20:note': 873}. Nenhum erro/aviso novo ou severidade relaxada. A nota V19 da classificação dos três tipos de erro migrou do hub para o novo helper com a mesma mensagem; uma nota V19 nova está apenas no helper de teste que confere o caminho comum a Import/Include, ambos exercitados pelos testes e pelas sondas de stderr. Essa cobertura decorre do L0 modules.md, que exige traces nos dois caminhos, pacote/path e aninhamento. Não há supressão. Os demais diagnósticos foram preservados por arquivo, regra, severidade e mensagem. O gate global continua bloqueado pelos V16 já existentes.
- RAM: MemoryMax 12 GiB, MemoryHigh 10 GiB, swap zero e Cargo jobs=1, execuções sequenciais. Pico medido 10.002991 GiB (10740629504 bytes); eventos {'low': 0, 'high': 969, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}. Logs de recursos acompanham cada comando.

## Limites e continuação

A primeira suíte completa encontrou uma expectativa antiga de trace vazio. A fixture exata confirmou no pin, nos quatro perfis, que ela deve exigir Import("routes/error.typ") com span 2..36 da expressão. A expectativa foi atualizada preservando todas as demais assertivas e passou a conferir também caminho e span do trace. O L0 tests.md foi atualizado antes do teste; scope-extension.json.gz registra os textos/hashes originais. A primeira execução e seu source freeze estão em legacy-expected-attempt.

A comparação inicial e o primeiro diferencial ficaram preservados em initial-comparison. A primeira comparação só descontava o aviso de arquivo vazio no fim do stderr; a revisão retira exatamente um bloco literal desse aviso em qualquer posição e mantém todos os demais bytes. O primeiro diferencial exigia apenas a nota movida; a revisão identificou e justificou a nota adicional no teste, sem alterar fonte, severidade ou configuração. observables-reviewed-resources.json registra essa conferência separada.

Este lote fecha a validação de fontes antes de avaliar e a herança de contexto medidas. Não afirma paridade universal de parser, imports, HTML ou bundle. As diferenças remanescentes estão em observables-result.json e follow-ups.md. O objetivo global continua ativo. Os pacotes TS1 aprovado e RG1 pendente são verificados sem alteração na conferência de entrega. Sem commit/push/merge/release.
