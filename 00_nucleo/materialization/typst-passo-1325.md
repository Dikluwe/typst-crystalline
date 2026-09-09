# Passo 1325 — âncora de campo ausente em dicionário, conteúdo raw e float

## Medição e escolha

Continuação a partir do fechamento P1324 em
`00_nucleo/diagnosticos/p1324-closure.json`, preservando a árvore não commitada.
O baseline fresco `00_nucleo/diagnosticos/p1325-baseline.json` registra HEAD,
diff/stat integral, binários pinados, horários e saídas. A referência continua
upstream `a51e02804`, não uma tag de versão.

A prioridade seguinte da fila P1322, `math-symbol-binding-warning`, foi
reaberta: nome `math` e `Module.content() == None` não provam builtin.
`eval/modules.rs:105–114` também constrói módulos importados sem conteúdo.
O recibo `00_nucleo/diagnosticos/p1325-review-math-identity-preflight.md`
refuta a suficiência do guard proposto; não é prova de impossibilidade geral.
Identidade nativa e owners necessários ficam pendentes, sem guard improvisado.

O próximo cohort elegível, `nonmodule-field-span`, foi medido novamente:
mensagens de Dict, Content raw e Float coincidem nos campos ausentes focais,
mas o destaque inclui o acesso inteiro. `[x].text` expôs débito de lookup
separado; ele permanece erro, com apenas a âncora corrigida. Não ocultar esse
residual como paridade nem expandir este passo para disponibilidade.

## Implementação autorizada

1. Atualizar primeiro o proprietário L0
   `00_nucleo/prompts/compiler/eval/bindings/field_access.md`, com sucessão
   explícita das antigas proteções de span total apenas neste recorte.
2. Congelar baseline, escopo e L0 normativo; revisar gates V5/V15/V26 e resselo.
3. Autor independente, com contexto novo, escreve testes derivados do L0,
   sem ler o owner ou a solução. Integrar testes formatados e demonstrar RED.
4. Alterar somente a seleção de span de Dict, Content raw e Float no owner
   `01_core/src/compiler/eval/bindings/field_access.rs`. Não alterar lookup.
5. GREEN, build, testes workspace, fmt, lint zero violations, linhagem;
   matriz bilateral normal/repetida/inversa e revisão independente.
6. Relatório substantivo em `00_nucleo/diagnosticos/p1325-final-report.md`,
   incluindo limites e débitos; fechamento reproduzível sem commit.

Regime: ensaio A/B da skill tekt-materializacao-segregada, com autor de testes
e revisor separados. Ambiente compartilhado: sem atestação técnica de
isolamento, sem selo de refinamento. Unknown não vale sucesso. Até duas
revisões sem ganho na mesma falha exigem reabrir desenho. Não repetir corpus
completo antes de o focal passar. Temporários no target dedicado
`/tmp/p1325-target.KQl8cD`, cópia sem hardlinks do cache anterior; sem limpeza.

ADR-0127 contínua: correção localizada de paridade diagnóstica, não mudança
pública/default/fase. Excluídos LocatedContent, outros tipos, gates e erros
de método/callee. Preservar os seis arquivos produtivos externos ao par,
todos os artefatos históricos e testes anteriores. Não stage/commit/push.
Este passo coordena; apenas o Prompt L0 legitima a implementação.
