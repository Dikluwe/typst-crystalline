# Próximo lote independente: calc.abs

Triagem, sem alteração produtiva. Baseline: before.json.gz/audit-state.json.gz desta auditoria, HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado e fontes verificadas por SHA. O SARIF integral da etapa Par/Box, da mesma árvore produtiva, contém V16 em calc/abs.rs:68: `wildcard _ => delega decisão para other.type_name()`.

Consumer: `01_core/src/compiler/stdlib/calc/abs.rs`; owner: `00_nucleo/prompts/compiler/stdlib/calc/abs.md`. O ramo interno nomeia Str como string e Bool como boolean; o restante delega ao nome canônico do Value. A admissão externa já rejeita essas famílias por tipo e preserva seus spans. A classificação é formatação de diagnóstico, não aceitação numérica nem perda de resultado por default neutro.

A regra V16 está cobrando decisão local explícita antes da delegação. Não há prova aqui de defeito no linter e não se autoriza relaxar a política global. O candidato de lote é enumerar as famílias Value que compartilham o nome canônico no próprio ramo, com grupo V19 prescrito pelo L0. Str/Bool mantêm aliases específicos. Uma nova variante deve obrigar a decidir seu vocabulário no consumer.

Antes de código: atualizar o L0 proprietário em fluxo contínuo ADR-0127; medir chamadas bilaterais (incluindo aliases, overflow, dimensional/mixed-length, extras e erros); escrever testes. Se a mudança preservar comportamento, declarar esse fato e usar o RED estrutural de variante futura como prova da exigência de exaustividade, sem inventar um defeito funcional. A sonda usa o match produtivo extraído e stubs mínimos, com controle atual compilando e variante futura produzindo E0004; não compartilha o target Cargo nem é prova de paridade de runtime.

Sem supressão de V16. V19 só é justificado pela mesma política explícita de nome canônico para todas as alternativas; executar um braço não comprova cobertura individual. Validar resultados/diagnósticos de linguagem, Clippy, selos, suíte, Rustdoc e diferencial SARIF sob12GiB. TS1/RG1 e seus arquivos protegidos não entram nesse lote. Esta triagem ainda não é uma correção implementada nem crédito de redução de avisos.
