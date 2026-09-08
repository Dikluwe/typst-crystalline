# P1317 — revisão independente pré-candidato

Revisor: `/root/p1317_review`, sessão iniciada com tarefa limitada, sem histórico
de implementação herdado. Regime A/B **executado sem atestação de isolamento**:
o workspace é compartilhado; a separação de escrita é uma disciplina operacional.
Somente `00_nucleo/diagnosticos/p1317-review-*` é gravável por este papel.
Fonte, L0, testes, oráculos e recibos julgados são somente leitura.
Skill `tekt-materializacao-segregada` e suas duas referências lidas integralmente;
ADRs 0107, 0108, 0127 e 0129 lidas. A pesquisa em `00_nucleo/adr/` não encontrou
ADR local de materialização segregada. Pastas materialization/context não lidas.

## Medição e entradas

HEAD `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado
herdando P1315/P1316. Em 2026-09-08T14:57:57Z, `git diff HEAD --stat` registrou:

```text
 00_nucleo/prompts/compiler/stdlib/loading.md | 182 ++++++++++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs       | 213 +++++++++++++++++++++++++--
 2 files changed, 380 insertions(+), 15 deletions(-)
```

Medição P1317 SHA-256
`92b04d68105d573523c673c1bcd0fb64ec2ffe57d17788c79742ce0450ee2fdf`;
focal SHA-256 `2a995791a497c9579fa8854d0531eebc74428b71afe92d59f249ec6143fa4fe0`.
Ambos os hashes foram conferidos por `sha256sum`. Eles identificam fontes,
estado anterior, argv, saídas e instante. L0 inteiro lido, SHA-256 observado
`75a92fc68daac14ab093e1b6838a9401aea17efe575acdebbc971491cda15e71`;
resselo canônico posterior poderá mudar somente metadata, sujeito à revisão.

Baseline `/tmp/p1316-target.1c6HK7/release/typst`, SHA-256 registrado
`1178fcde18dee54cb6b5c0feadcc79c8066346e0bb005060db7a2217a072e8ba`.
Vanilla `/usr/local/bin/typst`, SHA-256 registrado
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
upstream ratificado `a51e02804`.

Fonte vanilla `loading/csv.rs:138–157` define separadamente a causa Utf8
`file is not valid UTF-8`; `diag.rs:845–925` acrescenta posição/path e escolhe
origem. No cristalino, `loading.rs:955–965` entrega Utf8 ao fallback Display.
O focal com dados de mesma largura chega a Utf8 nos dois produtos; o focal
com linha curta e byte inválido chega a UnequalLengths nos dois. A intenção
da causa vem do formatter explícito; copiar antecipadamente uma validação
UTF-8 mudaria o erro vencedor e violaria a obrigação.

## Decisão

**GO de escopo, condicionado aos gates pré-patch e finais.** Correção interna
de paridade em mapeamento de erro existente, abrangida pelo fluxo contínuo
ADR-0127. Não há assinatura, entidade, trait, opção, fase ou compatibilidade
nova. O observável é mensagem diagnóstica, exceção expressa ADR-0108; não é
requisito de mecânica do parser. O owner permanece loading.rs ↔ loading.md.

A cláusula P1317 substitui expressamente apenas a preservação textual Utf8
das revisões anteriores. Mantém spans Bytes P1316, decoder/Path/Str detached,
ordinal P1315 e demais mensagens. O novo texto propaga a Path/Str pelo decoder;
isso precisa de teste real via World/arquivo, pois sondas iniciais de arquivo
inexistente só provaram I/O. As sondas com concatenação Bytes também não
provaram parsing no baseline e foram corrigidas pelo focal literal.

Parsing+excesso segue dívida de precedência: se parsing legado vencer por
Utf8, muda sua causa textual; não se exige igualdade com vanilla que rejeita
excesso antes. Sufixos de localização e diagnósticos Path/Str continuam dívida.

Pendente: RED local genuíno, A/B congelado antes de candidato, controles de
Unicode válido/UnequalLengths/opções/outros loaders, Path/Str via World,
origens distintas/With/Args/map, preservação dos testes e artefatos P1315/P1316,
replays normal/repeat/reverse e gates build/lint. Sem alegação de paridade CSV
geral, selo de refinamento ou isolamento técnico.
