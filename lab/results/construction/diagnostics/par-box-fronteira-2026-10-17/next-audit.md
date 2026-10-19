# Próximas auditorias

Etapa atual: geometria não fracionária e admissão regional certificada nos corpora do relatório. Nenhuma lista abaixo conta como implementação concluída.

1. NativeText posicional/relativo: medir combinações de tamanho/fill/body, argumentos nomeados e aliases antes de alterar constructor/StyleChain. Evidência prévia no lote paragraph; tamanho posicional já diverge. Não implementar apenas o caso absoluto e descartar em.
2. Box fracionário, composição B: `empty-fr-box` conserva baseline divergente e backing de largura zero. Ler o L0 boxed e o alocador de frações antes de propor transporte; medir corpo, prefixo/sufixo, clipping, paint e posições, incluindo nested/repeat. A etapa presente não elimina essa dívida.
3. SVG multipágina: o corpus regions-before usa o template `{p}`. Tekt grava nome literal e a página inicial, enquanto o pin expande o template e exporta todas. Delimitar contrato CLI e classificação ADR-0127; preparar revisão se houver quebra pública de compatibilidade. Os PDFs vetorizados de todas as páginas certificam a geometria da etapa atual, sem certificar o exportador SVG multipágina.
4. V16 restantes: usar inventario-v16-restante.md e respectivos L0; classificar intenção/semântica antes de enumerar variantes, projetar tipos ou prescrever exceção. Manter V19 por política comum comprovada, sem chamar execução de um braço de cobertura de todas as alternativas.
5. Coleta externa de decoração e regiões com reservas/colunas/transações: a nova admissão de linha conserva seus protocolos. Medir suas fronteiras antes de estender a certificação.

RG1 continua dependente da confirmação já solicitada. Não reimplementar o contrato público nem sobrescrever as correções independentes de power, paragraph/par e Par/Box ao retomar a proposta.
