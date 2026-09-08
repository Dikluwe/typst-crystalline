# Passo 1314 — erros e ocorrências das opções CSV

**Estado: implementado e validado — PASS_SCOPED em 2026-09-08.**

Fechamento em `00_nucleo/diagnosticos/p1314-final-report.md`, com evidência
e revisão independentes. Workspace: 6.645 testes passam; A/B: 2.808
comparações passam; lint sem erros. Limites preservados estão no relatório.
Sem stage, commit ou push.

## Medição e resultado pretendido

O diagnóstico da execução P1313 mantém parsing e opções CSV como dívidas
distintas. A medição fresca `00_nucleo/diagnosticos/p1314-measurement.json`
(SHA-256 `660fd77c95fe243e22590036b03dc5e364b0e3a107888592b35c4d166d557626`)
mostra dois defeitos locais nas opções: erros sem origem do valor e validação
apenas da última ocorrência depois de With/Args. Por exemplo, uma opção
`delimiter: "ab"` pré-ligada é ignorada quando outra válida a sobrepõe;
o vanilla rejeita a primeira inválida e mostra sua origem.

Duplicatas escritas diretamente já são rejeitadas antes da função nos dois
produtos. Não confundir esse caso com ocorrências transportadas por With/Args.
Baseline, fontes, executáveis, horários e estado não commitado estão nos
recibos `p1314-baseline.json` e `p1314-measurement.json` em diagnósticos.

## Escopo

L0 proprietário: `00_nucleo/prompts/compiler/stdlib/loading.md`, seção P1314.
Consumer único: `01_core/src/compiler/stdlib/loading.rs`.
Validar todas as ocorrências de delimiter, depois todas de row-type, em sua
ordem causal; a última válida vence somente quando nenhuma conversão falha.
Erros apontam ao value_span da ocorrência que falhou, sem inventar origem
para Args sintético/detached. Defaults, casts admitidos e mensagens continuam.

Preservar unknown-named antes da fonte, cast da fonte antes das opções,
opções antes de leitura/parse, diagnósticos de parsing e I/O, Symbol fonte,
suporte Bytes, read, encoders e os demais decoders. Delimiter Symbol mantém
rejeição e mensagem cristalinas com origem corrigida: não alegar paridade
dessa coerção. Não criar entidades, trait, assinatura pública, modo ou fase.

## Execução

1. L0 primeiro, revisão ADR-0127 e ownership V15/V26. Correção interna de
   paridade em consumer existente segue fluxo contínuo; nova dúvida real para.
2. A/B independente a partir do L0 congelado, sem ler patch/testes locais;
   revisor distinto confere políticas antes do candidato. Sem atestação de
   isolamento técnico, pois filesystem compartilhado; sem selo completo.
3. RED local real para origem, primeiro inválido, último válido e ausência
   de I/O; patch mínimo no owner e GREEN. Build em target RAM dedicado.
4. Quatro perfis e normal/repeat/reverse; controles de fonte/parsing/leitura,
   wrappers e carriers. Replays históricos imutáveis: declarar previamente
   os deltas de opções, nunca corrigir os oráculos anteriores.
5. Build/testes workspace, fmt, lint, diff/check, resselo e revisão final.
   Relatório substantivo em diagnósticos: resultados e dívidas, com proveniência.

Unknown obrigatório, crash, timeout ou fixture inválida bloqueiam. Até duas
revisões focais sem progresso pela mesma causa; depois reexaminar o desenho.
O passo é tático, não L0: registrar seu histórico, mas não usá-lo como fonte
normativa congelada que impeça atualizar o status da execução.

Escritas: este passo, L0/owner, diagnósticos p1314-* e temporários dedicados.
Não ler passos históricos/context; não stage, commit, push ou escrever P1315.
