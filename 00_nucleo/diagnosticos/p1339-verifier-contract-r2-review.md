# P1339 — revisão focal independente do contrato r2

Verificador `/root/p1311_review`, 2026-09-10 03:13:41 UTC; executado sem
atestação de isolamento. Estado de origem: HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado,
conforme inventário e hashes em `p1339-verifier-intake-r1.json`.
Nenhuma edição de produto, L0 ou oráculos; zero matrizes completas pré-selo.

## Evidência

- Contrato r1 preservado: `e062fb551fcffd78d21c1ae2a0a185aa376dd790c3e0825cec045a7f1e65b509`.
- Contrato r2: `fb10967e8c22e5350a8c864b874cf282b5011c6dbc14b5c3eacc4f103b6aab71`.
- Desenho r2: `5efe3cb39fcf932b1db6d45ba672c8cd34563bee9dfb3fb456f7dca822c1bbe5`.
- Suplemento do adapter: `08d0be7f19894c111130d0c791df6804cd1e7d89d2b8d6f418702be88af2125d`.

Leitura do desenho e de todas as diferenças estruturais entre os contratos;
verificação SHA-256 de cada entrada de `input_sha256` r2 sem divergência.
Comparação exata confirma rotas, obrigações públicas, predicados W01–W10
(excluída apenas a descrição operacional `requires` e novo `phase_mapping`),
539 IDs históricos, mutantes, budget, hashes L0, perfis, ordens, scope-out e
preservação global inalterados.

## Conclusão focal

**Aceito o redesenho contratual por fases**, não o selo ou o produto. A revisão
resolve a precondição circular registrada em
`p1339-verifier-phase-precondition-r1.md`: F01–F10 têm definição integral
congelada antes do candidato e execução obrigatória real na fase final.
`NotDue` é apenas agenda, sem crédito de execução e não substitui Unknown.
Todos os vinte mutantes permanecem devidos na fase C. A distinção entre
`context_reads_valid_for == false` e a variante privada real `Unproven`
está explícita e verificável por binding test-only.

A decomposição dos três casos mistos mantém conjuntamente produtor protegido,
consumidor autorizado e predicado composto previamente congelado. Ainda cabe
auditar os fixtures concretos e o ledger de deltas: esta revisão não admite
qualquer saída mista por conveniência nem altera medições históricas.

O suplemento do adapter permite somente tradução mecânica dos fixtures e
predicados independentes. Antes do selo, serão exigidos seus hashes, fontes
concretas/templates completos, inventário exaustivo de carriers e binding
fechado; JSON genérico com nomes de obrigações não é teste. A implementação
da relação sob teste nunca pode morar no adapter. O selo também deve pinar
`p1339-implementation-authorities.json` e este suplemento prospectivo.

Permanecem pendentes oráculos/registro negativos canônicos, cobertura por
cláusula, auditoria dos adapters, M12 compilado com grafo real, execuções de
discriminação e determinismo. Nenhum GO de implementação é emitido aqui.
