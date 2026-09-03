# P1294 — relatório de execução bloqueada

## Natureza e autoridade

Este é um relatório diagnóstico, legível por humanos, produzido por solicitação
expressa do dono em `2026-09-02T22:24:38-03:00`. Ele **não** é o
`p1294-final-report.md` previsto para o caminho aprovado do Passo 1294, não é um
certificado e não altera o veredito congelado de S3.

O relatório foi redigido depois do commit
`76fb7336311bdb6497456ab5fdc0a8ce355ff39b`, na branch `Tekt`. A árvore estava
limpa imediatamente antes desta escrita. Este arquivo não pertence à cadeia
selada, não é pinado pelo manifesto, pelo selo ou pelo recibo e não pode ser
usado como prova de conclusão do passo.

Regime: execução segregada por capacidades e artefatos, sem alegação de
isolamento técnico de leitura.

## Entradas congeladas

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1294.md` | `10d6393ad9615f0ed38bc7ad8192d167a5bf91388e52cf38a1acb4271b1cc2e4` |
| `00_nucleo/diagnosticos/p1294-sanitization-manifest.json` | `0d3f24c9ac3f5691d4f07c1658440b58134bb9e94fd5d7e47f42ff84bedf7c41` |
| `00_nucleo/diagnosticos/p1294-terminal-seal.json` | `352ad6184b4c25c3ec5136ff56cca7ec8600885f69f89d248506fa698d4546a0` |
| `00_nucleo/diagnosticos/p1294-verification-receipt.json` | `0d31aa9b1e31440552c1d52f571923b40d68f7c65027bd093417c906c04535d1` |

O bloco `sanitized_terminal` possui `7.870` bytes canônicos e SHA-256
`6238a84f546d7b15c18b64f6a9402008d2b4c59d719f189ef6f828aaf21d8868`.
O veredito autoritativo do recibo é `BLOCKED`; `certificate_authorized` e
`report_authorized` são ambos `false`.

## Sanitização documental confirmada

A medição reproduziu que o bloco terminal histórico possui `4.588` bytes
canônicos e SHA-256
`077559cc1632cdd684532c2dec1638a9f57277b806b8263e519cadad7e50f045`,
exatamente igual ao digest declarado pelo metadado. A alegação
`dcb5a0911cb9e9f7c1ecd8cb89a1697b8a058972b92bfd052be95ef24d5ff394`
foi refutada e não foi promovida a digest real.

O manifesto P1293 histórico foi preservado byte a byte. A leitura por pares de
objeto detectou exatamente duas ocorrências de
`$.contract_reopening_attach_ic_b.replacement_serial_seal`. Em ordem textual,
os valores medem `864` e `581` bytes canônicos e possuem SHA-256
`181f102cb6c57b4a3d33e0ec3891f6e53e5683eee5a9f9d13140ca8a458c6679`
e `029fcf801a8ff169b2aba3024873fed28d5b45949247f104ba87135c39e50fb2`.
Nenhum parser `last-wins` foi aceito como prova e nenhum artefato P1293 foi
normalizado retroativamente.

## Gates aprovados

- Integridade do inventário: `74/74`, zero ausentes e zero divergentes; digest
  canônico `33fe3dcd9cd287b24a82e255f7dffd2728cc76aab969ad5b049d35bbcf7b9c42`.
- Seis artefatos finais P1293 e os oráculos P1292/P1293: byte a byte intactos.
- `cargo fmt --all -- --check`, `crystalline-lint .`, V3, V4, V5, V7, V13,
  V14, V15, V26 e `--fix-hashes --dry-run`: aprovados; o dry-run imprimiu
  `Nothing to fix`.
- Contratos protegidos nas duas ordens: P1292 `11/11` e P1293 `11/11` em
  todas as quatro execuções.
- Evidência de mutação preservada, sem rerun: dois baselines verdes, 31
  mutações forward e 31 reverse, score `1.0`, zero sobreviventes, zero
  `Unknown` e ordem estável.
- Superfície default: `111/99/12`; HTML: `115/104/11`; zero missing, zero
  unverified, zero `Unknown` e zero divergências entre objetos de resultado.

## Bloqueios

S3 registrou quatro gates bloqueantes:

1. A primeira execução de `cargo test --workspace -q` passou, mas a segunda
   terminou com código `101`. O teste
   `p1137_watch_dependencias_recuperacao_e_filtro` não satisfez a condição em
   20 segundos. Uma execução verde não apaga essa falha.
2. `cargo build --release` passou, porém produziu o binário
   `10fd9cf488be69cfc35796f4535e3efab71a970be874d5bbfee194e7c4460151`,
   diferente da testemunha congelada
   `c527b4111493f444d66e44e6d38d4823b2d09515c73d92eda1b9c4a2a86bff90`.
3. A superfície default não foi byte a byte idêntica. A única diferença foi
   `$.binaries.crystalline.sha256`, com o mesmo par de hashes do item 2.
4. A superfície HTML não foi byte a byte idêntica pela mesma e única diferença.

Totais do recibo: uma repetição workspace falha, uma divergência de hash do
binário, duas comparações byte a byte de superfície falhas, duas diferenças de
caminho JSON, zero diferenças nos objetos de resultado, zero `Unknown` e zero
sobreviventes.

## Análise de proveniência do hash do binário

Medição anterior à decisão: o recibo final P1293 registra as medições no HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, com working tree não commitada,
entre `2026-09-02T20:10:38.679661-03:00` e
`2026-09-02T20:21:26.103426-03:00`. O P1294 executou os gates no baseline já
commitado `5079a0cdfaedc406b7c036ac61e40d7f6a1264d6`; o binário reconstruído declarou
`typst 0.15.1 (5079a0cd)`.

Inferência: a testemunha `c527b411...` pertence ao estado anterior, enquanto a
reconstrução `10fd9cf4...` incorpora o commit posterior. Isso explica também as
duas diferenças derivadas nas superfícies, mas não transforma os gates em
sucesso. A inferência seria refutada por uma reprodução limpa do hash
`c527b411...` no commit `5079a0cd` com o mesmo toolchain, ou por evidência de que
o hash do commit não participa dos bytes do binário.

O timeout do teste temporal é um bloqueio independente; a análise do binário
não o absolve nem o apaga.

## Veredito e próximos passos

O P1294 permanece **não concluído**. Não existe certificado P1294 e o caminho
reservado `p1294-final-report.md` permanece ausente. Este relatório não autoriza
nenhum dos dois.

Para prosseguir, uma nova cadeia deve preservar o recibo bloqueado e:

1. diagnosticar e corrigir, em passo próprio, a instabilidade do teste P1137;
2. definir uma testemunha de build reproduzível vinculada ao commit realmente
   verificado;
3. executar uma nova sequência S3 sem sobrescrever a evidência vermelha desta
   tentativa.

Claim proporcional: a sanitização documental foi materializada e auditada até
um recibo bloqueado. Não se alega equivalência funcional geral, isolamento
técnico de leitura nem cumprimento operacional perfeito.

`PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED`
