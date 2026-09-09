# Passo 1327 — avisar quando o bare import não tem efeito

## O problema medido

O próximo item elegível da fila P1322 é `module-bare-import-warning`.
P1326 encerrou o diagnóstico de fields de closures; `math-symbol-binding-warning`
continua pendente de identidade suficiente, conforme a revisão P1325, e não
é reaberto aqui. Não é necessário distinguir origem de módulos neste recorte.

No baseline P1326, `import std` e `let named = std; import named` preservam
binding e valor, mas omitem `this import has no effect`, presente no vanilla
ratificado upstream/main `a51e02804`. A medição nova está em
`00_nucleo/diagnosticos/p1327-baseline.json` (SHA-256
`f8cee37f7f3556db93f935deb977790a0a13ddd232639334e3d3931cf8b504f8`),
com HEAD, working tree não commitado, diff/stat, binários, horários e saídas.
Fonte: `01_core/src/compiler/eval/modules.rs:207-219` e
`lab/typst-original/crates/typst-eval/src/import.rs:82-103`.

## Entrega delimitada

1. Atualizar primeiro os L0 `compiler/eval/modules.md` e `compiler/eval/tests.md`.
2. Congelar testes A/B independentes e sucessores somente das seis observações
   históricas P1305/P1306 que exigiam silêncio. Medir o corpus antes de C.
3. Demonstrar RED sem erros de build e implementar aviso somente após resolução
   e validação de bare Module cuja fonte é Ident, sem as/lista. Preservar binding.
4. Exigir GREEN, build, suíte workspace, fmt, lint geral e V5/V15/V26, linhagem
   recíproca conferida e observação CLI integral normal/repetida/invertida.
5. Registrar resultado e dívidas em `00_nucleo/diagnosticos/p1327-final-report.md`,
   com revisão e recibo de encerramento. Não commitar nem iniciar outro passo.

Owner produtivo: `01_core/src/compiler/eval/modules.rs`; test-only:
`01_core/src/compiler/eval/tests.rs`. Preservar todas as mudanças anteriores.
O passo coordena; somente os dois L0 legitimam código. Correção diagnóstica
de paridade em fluxo contínuo ADR-0127, sem API/default/fase novos.

## Fronteiras e prova

Aviso sobre o Ident inteiro, mensagem exata, severidade warning, sem hints/trace.
Aliases e módulos ordinários não são exceções. Erro posterior conserva o aviso.
Field, literal, as, items e wildcard não recebem aviso; falhas anteriores e
bare dinâmico continuam erros. Rename redundante, tipos importáveis adicionais,
ordem/trace de import e math identity ficam abertos, sem falsa paridade.

Regime Tekt A/B: root especifica/implementa e integra testes congelados;
autor de testes recebe L0 e interfaces/test-only, nunca o candidato; revisor
não edita o que julga. Ambiente compartilhado, sem atestação técnica de
isolamento e sem selo de refinamento. Unknown obrigatório bloqueia conclusão.
Duas revisões sem ganho na mesma causa reabrem a análise. Temporários em
`/tmp/p1327-target.k9Mq0s`, cópia sem hardlinks do cache anterior.

## Sucessão R2 — apresentação de erro e warning em eval

O C1 passou nos seis testes novos e no workspace, mas a comparação CLI
congelada encontrou 24 diferenças de ordem dos mesmos blocos: warning→erro
no cristalino, erro→warning no vanilla. A insuficiência prevista do owner
set foi portanto refutada, não escondida por mudança de oráculo.
Prova e causalidade: `00_nucleo/diagnosticos/p1327-review-cli-failure.md` e
`00_nucleo/diagnosticos/p1327-ab-verdict.json` (Violated histórico).

Acrescentar primeiro a obrigação L0 em `00_nucleo/prompts/wiring.md`,
owner `04_wiring/src/main.rs`, somente ordem de apresentação em `run_eval`.
Congelar manifesto R2, preservar o C1 e os testes/oráculos anteriores,
medir controles independentes com warning preexistente e erro, e só então
materializar a composição erro→warnings. Não alterar compile/query/drain,
formatter, Source, serialização ou avaliação. Correção contínua ADR-0127;
não acrescenta API/default/fase. Reexecutar CLI integral e gates finais no
C2. O relatório deve explicar a falha C1 e a ampliação causal, não declarar
que o primeiro owner bastou. Nenhuma prova histórica é sobrescrita.
