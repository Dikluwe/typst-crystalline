# P1314 — revisão final independente

**Veredito: `PASS_SCOPED`. Nenhum achado acionável pendente no recorte P1314.**

As opções CSV agora validam cada ocorrência causal, conservam a origem do
valor inválido e só permitem que a última ocorrência vença quando todas
forem válidas. O candidato respeita o L0 congelado e suas preservações.
Isso não fecha paridade geral CSV nem as dívidas excluídas de Symbol,
parsing, missing/excesso e precedência de named desconhecido.

## Evidência e proveniência

Revisor `/root/p1314_review`, conferência final 2026-09-08T12:48:44.626Z.
HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado,
índice sem mudanças. Recibo próprio `p1314-review-evidence.json`, SHA-256
`6c0f091da3eef84294af36a6b8b0dd47027a86ff2d410080e3b983a697067751`.
Contém diff/stat, identidades, hashes de entradas e resultados recontados.

Reprodução somente leitura no host, que contém o namespace RAM registrado:

```sh
node 00_nucleo/diagnosticos/p1314-review-check.cjs
```

Script SHA-256
`2eee4e567d0470139c7bc642c59eb7e79f9a371e212b90787e5bd47a204ba22d`.
O script não corrige os materiais examinados; compara observações completas,
exige chaves únicas/completas e falha ao encontrar divergência. O snapshot
de evidência foi escrito separadamente por apply_patch.

Identidades finais verificadas fisicamente:

- Owner: `1e911a53392ce900e482cf020973d5c78136977c032045fdc4f66e10ea8ec83a`.
- L0 integral: `6b717ccd9ddd4784407ca2b1d052f25c63c78198062748f449538a6ff85a0097`;
  pin normativo do freeze permanece
  `21231170b7092331ae09fafc656210bb94829658d822a15291de6ea0eb28dbfe`.
- Candidato RAM: `cf9b997ac399cfb95cf15063cc417484ed28a75be18f6e7d55a28300d792114f`.
- Freeze: `41ac0730e01cd31cd9660b0e229e37d5e486a2c114927f3ac49c3475733756b9`.

## Revisão do código e da cadeia

O owner anterior foi reconstruído do HEAD e do diff integral registrado no
baseline P1314; seu SHA-256 coincide com o pin P1313. Os testes anteriores
permanecem literalmente intactos, assim como produção anterior ao recorte
CSV. A inspeção do restante confirma preservação do cast da fonte, rejeição
prévia de named desconhecido, leitura e decoder.

O helper privado percorre occurrences por referência e filtra um nome de
cada vez. Cada cast interrompe na primeira falha e atribui value_span dessa
ocorrência ao diagnóstico. O fallback Args sintético consulta named e mantém
erro detached. O cast completo de delimiter termina antes do de row-type;
nenhum cast admitido, default ou mensagem foi ampliado pelos casts privados
extraídos. Não há clone do carrier, seleção de fixture/texto de erro, novo
campo, trait, assinatura pública, owner, dependência ou fase de pipeline.

Bytes continuam sendo decodificados depois das opções, sem World. Path/Str
só chegam à leitura após validação completa. Os testes locais atacam origem
sintética/detached e World proibido; seu RED flagra leitura indevida quando a
opção inválida anterior é descartada. Essas propriedades não são atribuídas
exclusivamente à observação CLI.

O freeze antecede o patch autorizado por `p1314-review-prepatch.md`.
Seus 24 pins permanecem intactos. A mudança final de L0 é somente o metadado
Hash do Código permitido pelo pin normativo. A comparação física de 8.083
arquivos encontra alterações P1314 somente em loading.rs e loading.md;
os 252 artefatos anteriores pinados continuam intactos. Os pareceres prévios
não foram reescritos. A tentativa operacional de patch rejeitada por formato
não constitui revisão de resultado nem mudança no contrato.

## Gates e recontagens independentes

Os números seguintes pertencem ao estado e recibo acima. RED local registra
quatro falhas reais e dois controles aprovados; GREEN tem seis aprovados.
GREEN, build, workspace, lint, fmt, diff-check e preview final de linhagem
têm estado de fonte before/after igual ao diff final auditado. Os metadados
de executável predecessor em gates anteriores ao build não foram confundidos
com o candidato final.

Workspace release: **6.645 aprovados, zero falhas, três ignorados**. Os seis
testes P1314 passam nesse estado final; ignorados são os doctests antigos de
layout/introspector. Não houve necessidade de repetir a suíte. Build e gates
arquiteturais encerram em zero; lint registra zero erros, 240 warnings e
1.136 infos. Preview final responde `Nothing to fix`.

Recompus as 936 expectativas do freeze diretamente das medições de origem,
incluindo a política explícita Symbol; recomparei as 2.808 observações do
candidato nas ordens normal/repeat/reverse. Todas passam, nenhuma Unknown.
As 344 expectativas que distinguiam baseline do resultado exigido estão
satisfeitas. Os casos históricos mantêm cwd e expressões originais, sem
normalização de stdout/stderr para esconder diferenças.

| Replay contra P1313 | Preservados | Deltas congelados |
|---|---:|---:|
| P1310 | 1.036 | 0 |
| P1311 | 220 | 0 |
| P1312 | 276 | 4 |
| P1313, ordem normal A/B | 524 | 52 |
| P1308 | 1.982 | 0 |

Os quatro deltas P1312 são csv-delimiter nos quatro perfis. Os 52 deltas
P1313 são exatamente os 13 ids de opções predeclarados nos quatro perfis;
foram recontados das execuções A/B existentes, sem execução redundante ou
mudança do oráculo histórico. P1308 conserva o resultado contra seu oráculo
original: 1.938 Preserved, 44 Violated e zero Unknown. Essas 44 diferenças
precedem P1314; não foram rebatizadas como replay histórico inteiramente
verde. Todos os outros replays também têm zero Unknown.

## Limites do veredito

Regime A/B executado sem atestação de isolamento técnico. O testador recebeu
contexto novo, congelou expectativas antes do candidato e não leu produção
P1314 nem testes locais; a exposição incidental ao diff histórico embutido
no baseline foi declarada. O revisor não editou os artefatos que julgou.
Filesystem compartilhado não atesta separação de capacidades. Não há selo
completo, mutation score ou equivalência funcional geral.

O relatório de implementação pode encerrar sua seção pendente com esses
resultados e referenciar este parecer. A atualização de status do passo
tático não altera entrada normativa congelada. Esta autorização de fecho
documental não permite alterar source, L0 normativo, casos ou oráculos sem
nova revisão. Sem stage, commit ou push.
