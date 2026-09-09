# Reabertura focal do leitor de preservação

O leitor R0 reconstruiu flags antes da expressão, mas `p1335-sentinels-r3.py:63–76`
congela expressão antes de `--features`. Isso gerou 27 violações de identidade
nos controles de localização não default, sem alteração real dos bytes ou flags.
O R0 e seu relatório permanecem. O sucessor `p1335-review-preservation-ledger-r1.py`
corrige somente a ordem exata do argv; não normaliza comando, stderr ou spans.

O controle de autorização é mais estrito que um allowlist de IDs: para as 32
transições P1325/P1326, o leitor reconstrói mensagem e span do field a partir da
expressão e compara os três canais integrais. Para as 12 preservações relocadas,
exige 36 observações atuais no path histórico, binário pinado e igualdade literal.
Isso não declara fechados os campos `table.cell.body` ainda divergentes.
