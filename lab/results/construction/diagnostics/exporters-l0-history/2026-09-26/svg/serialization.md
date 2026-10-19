---
Hash do Código: a65dc621
prompt: infra/export/svg/serialization
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/export/svg/serialization

## Medição anterior à decisão

Fonte anterior `03_infra/src/export/svg.rs:1975-1982, 1988-2001`, HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, capturada em `2026-09-26T09:29:07-03:00`. Proveniência exata: `00_nucleo/diagnosticos/svg-atomization.md`. O texto histórico conserva medições e gates anteriores; não é nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo `03_infra/src/export/svg/serialization.rs`. Possuir color_to_css, fmt_num e fmt_pair. Manter conversão sRGB/alpha, arredondamento de números a nove casas com fator 1_000_000_000, fast-path inteiro e ryu. Não alterar representação numérica, escaping do XmlWriter ou formato dos offsets adaptativos (estes pertencem a paints).

Atomização estrutural reversível em L3: fluxo contínuo ADR-0127, sem mudança de contrato público, comportamento por defeito, fase ou compatibilidade. Referências ao monólito no texto histórico abaixo indicam a fonte anterior; a implementação específica passa a pertencer somente a este consumer. Não ampliar alegações de paridade nem interpretar mecanicamente novos IDs/bytes como paridade de linguagem.
