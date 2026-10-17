# Passo — leitura seletiva do catálogo

Documento de execução, não L0.

Concluído experimento sem mudanças produtivas: seleção de tabelas, análise de bytes, redução do prefixo post mantendo validação, coleta pareada por face e seis testes em debug/release. Relatório e recibos em `00_nucleo/diagnosticos/catalogo-seletivo-2026-10-05.md` e pasta homônima.

Antes de integração: definir limites/fallback/erros e atualização do L0 de aquisição, com helper proprietário 1:1 e classificação ADR-0127. Não usar o buffer parcial como fonte final, não alterar defaults silenciosamente, não ressellar owners por causa deste diagnóstico. Validar CLI, seleção de fontes, texto/geometria/PDF. Coverage::from_ranges permanece uma frente separada pendente de ratificação.
