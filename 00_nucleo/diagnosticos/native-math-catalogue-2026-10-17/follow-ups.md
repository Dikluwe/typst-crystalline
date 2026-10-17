# Pendências depois do catálogo matemático

A matriz registra oito perfis e 3956 casos de funções do registry efetivo; não certifica todos os campos associados a Type ou comportamento de render/query/show. before.json.gz e final-state.json.gz fixam HEAD, árvore não commitada, diffstat e hashes; as coletas bilaterais conservam todos os comandos e saídas.

## Rotas ainda omitidas

- `std.format.bundle`
- `std.format.html`
- `std.format.html.elem`
- `std.format.html.frame`
- `std.format.pdf`
- `std.format.pdf.artifact`
- `std.format.pdf.attach`
- `std.format.png`
- `std.format.svg`
- `std.html`
- `std.html.elem`
- `std.html.frame`
- `std.pdf`
- `std.pdf.artifact`
- `std.pdf.attach`

## Próximas correções

- Auditoria preparatória em next-format-source-evidence.json.gz e next-format-pinned-schema.json.gz conserva fontes, hashes e nove declarações de schemas do pin, sem decidir arquitetura. O L0 histórico de FuncRepr o mostra público enquanto o código atual é pub(crate): conferir o contrato antes de assumir que ampliar a representação dispensa gate.
- Formatos raiz compartilham native_format no Tekt; reconhecer somente pelo ponteiro atual ou pelo nome criaria identidades falsas. Medir e definir a identidade interna no L0 antes de materializar. HTML/PDF e subfunções exigem schemas próprios e aliases efetivos preservados.
- Counter/bare selector têm admissões separadas, inclusive listas antigas de funções. As 26 sondas de counter continuam divergentes; não confundir metadados com suporte de contador/indexação.
- Os 29 negativos de campo desconhecido/internal/external têm mensagem correta, mas a âncora do diagnóstico ainda diverge do pin. São 26 desconhecidos mais dois campos internos de frac e gap external de mat.
- Revisar campos/capacidades das entradas históricas; as 26 funções aqui corrigidas não certificam toda a tabela. Helpers sqrt/abs/norm/round continuam funções não-elemento.
- Permanecem os 117 V16 anteriores. V19/V20 seguem notes preservadas; nenhuma supressão nova foi adicionada.
- FG1/RM1/SY1/RG1 e demais frentes anteriores continuam abertas. TS1/equality foram preservados; RG1 continua pendente de ratificação. Nenhuma API pública ou publicação é autorizada por este lote.

Correções internas de tabela/paridade seguem ADR-0127 em fluxo contínuo. Mudança pública, fase, default ou compatibilidade requer a proposta concreta correspondente. O objetivo global continua ativo.
