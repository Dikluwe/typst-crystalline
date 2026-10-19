# Relatório — V16 de MathSize/ssty

Etapa concluída. V16 **117 → 115**, V19 **1086 → 1088 notas**, V20 **873 → 873 notas**. Os dois fallbacks de ssty foram tornados nominais. O gate global continua bloqueado pelos 115 V16 restantes; esta etapa não declara o objetivo global concluído.

## Correção e decisão

Em 03_infra/src/font_metrics/math.rs::ssty_level_of e 03_infra/src/shaper/shaping.rs::ssty_level_for_subrun, Display e Text agora compartilham explicitamente None. Script conserva Some(1), ScriptScript conserva Some(2). Os gates anteriores de math, math_text_item e elegibilidade permanecem byte a byte. Shaping continua usando o predicado canônico; sua seleção de fontes, cache, features, glifos e emissão não mudou.

Dois pares source/L0 e o novo núcleo fonts/ssty-math-size.toml pinado compartilham a obrigação. L0 foi escrito antes da materialização. É manutenção interna em fluxo contínuo ADR-0127, sem alteração de MathSize, TextStyle, contrato público, default ou fase. O replay permite somente os complementos nominais, o import privado/prefixos de MathSize e o hash de linhagem; todos os outros arquivos protegidos foram conferidos. Não se alega melhoria medida de leitura por IA nem nova paridade com o vanilla.

As duas notas V19 são intencionais: ambos os tamanhos não solicitam ssty e são exercitados separadamente. Permanecem visíveis. Uma futura variante não recebe None silenciosamente: o compilador exige revisão. Não houve supressão nem alteração de severidade/configuração.

## Evidências

- RED estrutural anterior: before-guard-result.json falha porque o mutante MathSize::TektSstyFuture compila nos dois matches, com zero E0004 selecionados. Os controles atuais já passam; não houve falha funcional nova inventada.
- GREEN: a mesma sonda rejeita a variante futura com exatamente dois E0004, um em cada função auditada. Os 224 resultados dos quatro tamanhos e dos gates de math/TextItem/elegibilidade permanecem idênticos. Métricas cobre 32 entradas; shaping cobre 192 com 12 textos (vazio, ASCII, Unicode, números/decimal, combinações e não elegíveis).
- A sonda copia as funções e a declaração reais; modela somente os três campos de TextStyle lidos. Isso prova a proteção estrutural local e seus valores de controle, sem substituir shaping integral. O enum produtivo não foi alterado nem a árvore real mutada pela sonda.
- Testes reais focalizados ssty passaram. A suíte completa registra 8921 aprovados, 0 falhas e 7 ignorados, sem novo teste produtivo que apenas espelhe o match.
- Build, fmt, Clippy workspace/all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito e quatro índices, V1/V5/V15/V26, reverse lineage, replay e diff --check passaram. O SARIF diferencial confirma somente os dois V16 retirados e os dois V19 prescritos; demais diagnósticos preservados por fonte/regra/severidade/mensagem, sem V20 novo.
- Recursos sequenciais: MemoryMax12GiB, MemoryHigh10GiB, swap zero, Cargo jobs1. Pico 9.316467 GiB (10003480576 bytes); eventos {'low': 0, 'high': 0, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}. Sem alcance de MemoryMax, OOM ou OOM kill.

## Proveniência e continuidade

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Baseline `2026-10-17T18:36:27.221710+00:00`, fechamento `2026-10-17T18:50:36.744490+00:00`. before.json.gz conserva textos/fontes/L0/status/diff HEAD --stat e os 2128 hashes verificados da etapa import-syntax. final-state.json.gz congela 2129 arquivos, status/diffstat e binários. As sondas guardam rustc, argv, programas, saídas e hashes; validation-result.json registra comandos, UTC e source freeze. O linter é o pin 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA-256 e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2. O alvo vanilla é o commit 9f2b6e8715237cb086899a42873660fe744622e8; o binário tem SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. É proveniência do pin, sem alegar medição nova de paridade deste reparo mecânico.

inventario-v16-restante.md organiza os 115 locais restantes. follow-ups.md preserva as frentes de linguagem e propostas pendentes. Os pacotes TS1 aprovado e RG1 pendente são conferidos pela entrega. Sem commit, push, merge ou release.
