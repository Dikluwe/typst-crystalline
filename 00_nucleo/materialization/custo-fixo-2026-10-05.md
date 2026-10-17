# Execução — auditoria do custo fixo e compressão de cobertura

Continuação autorizada pelo usuário. Primeiro revalidar inicialização/fontes/exportação no estado atual, contar aquisição/reuso e separar tempos aninhados. Depois comparar uma correção interna concreta.

A auditoria encontrou domínio do catálogo somente nos controles com texto e apontou compressão de cobertura como parcela relevante. Uma sonda escalar independente comprovou codificação igual e redução de custo antes de editar o L0. O owner entities/font_book foi atualizado primeiro; materialização exclusivamente em Coverage::from_codepoints, sem API/unsafe/política nova. As provas semânticas existentes já eram GREEN; a hipótese de custo foi validada por benchmark, sem inventar RED funcional ou testes ordinários de relógio.

Suítes core/infra, build release, lint/ownership e controles PDF/fontes concluídos. Resultados, limites e estado exato no [diagnóstico](../diagnosticos/custo-fixo-2026-10-05.md). Diferenças preexistentes de paridade contra vanilla registradas e não misturadas à otimização. Sem commit nesta sessão.
