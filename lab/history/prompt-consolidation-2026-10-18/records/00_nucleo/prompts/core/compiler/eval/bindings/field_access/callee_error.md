# L0 — compiler/eval/bindings/field_access/callee_error
Hash do Código: a39f4be1

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/bindings/field_access/callee_error.rs`. No HEAD `ba5a4c7d305ed1444660d9eb80862f51b417efb6` e árvore limpa, `01_core/src/compiler/eval/bindings/field_access.rs:1052-1164` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-field-access-2026-09-30-before.json.gz`.

## Contrato

Construir os diagnósticos de chamadas inválidas de campos, chaves de dicionário e métodos inexistentes. Preservar as assinaturas e semântica de `00_nucleo/prompts/core/compiler/eval/bindings/field_access.md`; despacho estático na mesma camada L1, sem I/O nem estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de acesso a campos; `crystalline-lint .` passa com ownership e hashes corretos. Não inferir ganho de desempenho.


## Projeções de Value — exaustividade nominal, 2026-10-16

### Medição anterior à decisão

01_core/src/compiler/eval/bindings/field_access/callee_error.rs:18 (element_or_type_with_name, emitted-v16). O enum atual em 01_core/src/entities/value.rs:59–195 tem 39 variantes materializadas; o catálogo e forma de payload estão no inventário desta etapa, não na tabela histórica de variantes futuras. before.json.gz em lab/results/construction/diagnostics/tekt-v16-value-projections-2026-10-16 conserva HEAD, working tree não commitado, UTC, status/diff HEAD --stat, hashes e os textos originais dos owners/consumers. contexts.json foi extraído do AST atual conferido com o SARIF pinado; classification.json distingue nove V16 emitidos e oito projeções latentes. Forma Rust/exaustividade é mecânica; nomes, conversões, empréstimos e observáveis de linguagem mantêm o contrato vigente. Não há nova medição de paridade ou desempenho.

### Decisão interna

element_or_type_with_name conserva Content e LocatedContent como element com elem_name original; para as outras 37 variantes Value, nomear explicitamente o grupo que devolve type e delega à mesma vanilla_type_name(value). Não alterar os diagnósticos/hints/spans de field_callee_error nem a tabela do owner error_formatting. A entidade consultada não perde identidade/snapshot para essa projeção de nome. Teste C5: Content e LocatedContent com heading e Location legítima, boolean/integer/string e nomes genéricos none/auto/float/array; preservar nomes longos desta rota.

As variantes que compartilham resultado conservam um corpo comum. A lista nominal exige decisão se Value cresce; não duplicar corpos para remover V19. Notas V19 permanecem informativas e registram alternativas escritas, não testes executados, combinações alcançáveis ou déficit N×. Controles positivos/negativos e limites são escolhidos pelo comportamento, sem tratar a execução de um braço como prova de cada alternativa. Nenhum allow/expect ou relaxamento de severidade/exceção para ocultar V16.

### Gate e aceitação

Fluxo contínuo ADR-0127: nenhuma assinatura/trait/campo público, nova variante produtiva, default de produto, quebra de compatibilidade ou fase diferente. L0 antes de testes e implementação, resselo bilateral restrito e V1/V5/V15/V26 válidos. Escrever controles no módulo, confirmar comportamento antes e depois; RED estrutural é ausência de E0004 nos 17 pontos selecionados numa cópia do core com variante sentinela acrescentada somente ao Value copiado. GREEN exige E0004 em cada um dos 17 pontos, sem stubs, e remoção exata dos nove V16 selecionados com os demais diagnósticos preservados. Build, suíte workspace, Clippy estrito, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job Cargo. O gate global permanece aberto enquanto os V16 externos existirem. O falso positivo do parser sobre None explícito é investigação separada e não será escondido neste consumer.
