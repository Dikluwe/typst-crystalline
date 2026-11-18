# Revisão de fronteiras dos contratos — 2026-11-13

Revisão documental autorizada pelo usuário. Sem integração de candidatas, mudança de API, algoritmo, comportamento ou fases. Escopo: ADRs 0029/0030/0107 e L0s de Region, layout_types, pipeline paginado/contextual, reconciliação e núcleo compilation-metric-cache. Não constitui auditoria de todo o corpus.

A evidência precede a revisão: ADR-0030:71 proibia genericamente cópias em caminhos críticos; Region justificava Vec pela propriedade de um Layouter; layout_types chamava Text de permanente; reconciliation preservava agrupamento e compensação estimado/real; paged e o núcleo restringiam a retenção ao mesmo World/compilação. A auditoria prefix-identity-audit demonstra reúso de preparação sem retenção geral do resultado físico entre compilações, mas não atribui todo o custo a esses contratos nem prova a intenção histórica de facilitar construção.

Alterações:

- Pureza, responsabilidade da camada e validade de cache são perguntas distintas. ADR-0030 não manda converter todo Vec em Arc nem torna toda cópia um erro de linguagem. Memoização, por si só, não determina camada; I/O, imports e dependências continuam restritos.
- A regra tipográfica do ADR-0029 não se estende automaticamente aos tipos Frame/FrameItem/Region explicitamente excluídos; leia fidelidade com ADR-0107. Liberdade mecânica também não dispensa contratos e validação.
- As restrições de retenção continuam valendo para os handles emprestados, stores e experimentos atuais. Não são uma proibição universal de um produto imutável futuro com contrato próprio de dependências, efeitos, invalidação, memória e fallback.
- Text permanente significa suporte ao carrier e fallback, não obrigação eterna de rematerializá-lo em toda compilação. A reconciliação é algoritmo atual com limites conhecidos, não uma exigência da linguagem de sempre corrigir posições depois.
- Isolamento de gerações continua obrigatório. Compartilhar armazenamento imutável não equivale a compartilhar estado mutável nem certifica reúso semântico.

Ainda não especificado/materializado: produto físico persistente entre edições, diário completo de dependências, substituição da reconciliação ou nova ordem de fases. Esta revisão não declara essas alternativas corretas ou mais rápidas. Também não torna provisória toda cláusula mecânica: o contrato atual permanece obrigatório até uma revisão específica e validada.

Proveniência e validação: `lab/results/construction/diagnostics/contract-scope-review-2026-11-13/` conserva HEAD/diffstat/hashes anteriores, textos antes da edição, logs de grafo/selagem/lint e receipt.json. O primeiro pin usou SHA bruto e foi rejeitado; o hash efetivo foi corrigido antes da selagem. O grafo e o lint completo passaram. As únicas alterações Rust desta revisão são seis selos @prompt-hash, comprovados contra hashes anteriores; testes de execução não foram repetidos para alteração exclusivamente documental. Alterações preexistentes foram preservadas.
