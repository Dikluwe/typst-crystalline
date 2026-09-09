# P1323 — completar o warning experimental HTML

## Resultado e limite

Implementados os três hints e a terminação de parágrafo do warning HTML,
conforme o vanilla ratificado upstream `a51e02804`. A emissão continua no
mesmo ponto, condicionada à feature HTML, em stderr. Nenhuma API, flag,
serialização, fase do pipeline ou formatter foi introduzido. A mudança
produtiva limita-se a `04_wiring/src/main.rs`, legitimada primeiro por
`00_nucleo/prompts/wiring.md`.

O recorte é o envelope textual sem cores. Não declara paridade geral HTML,
ANSI, PDF ou da linguagem. Os outros débitos do diagnóstico P1322 continuam
abertos; controles de preservação não concedem crédito de paridade a eles.
Veredito independente final: **aprovado neste recorte**, conforme
`p1323-review-final.md` e `p1323-review-final-audit.json`.

## Proveniência reproduzível

Todas as medições são da working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`. Cada recibo de comando contém UTC,
argv, target, HEAD, status, diff integral/estatística, inventário produtivo
antes/depois e hash do binário. Não usar apenas `typst --version` como origem.

- Vanilla `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Baseline `/tmp/p1322-target.Ir19xI/release/typst`: SHA-256
  `756b1c85a5879ea0afb79fc35184525aebfafa6ccb926ae86679a868691f99fa`.
- Candidato `/tmp/p1323-target.9NrOxY/release/typst`: SHA-256
  `f0b251746274d537c63929ab357b0e84e0e0955c7986d58621d83a0ce3a4e5ee`.
- Manifesto: `p1323-manifest.json`; baseline fresco: `p1323-baseline.json`.
  Corpo normativo congelado em `p1323-obligation-freeze.json`, SHA-256 do
  corpo `33ca7a522ae37d8fc9e206c6f5cc592638adbf6d182b5eac189c138e197b4926`.
  A exclusão é apenas a linha recíproca `Hash do Código:`.

As alterações anteriores em `loading.md`, `loading.rs`, `call_dispatch.md`
e `call_dispatch.rs` foram preservadas. No estado final, `git diff HEAD
--stat` inclui esses quatro arquivos e o par wiring deste passo:

```text
00_nucleo/prompts/compiler/eval/call_dispatch.md |  53 ++-
00_nucleo/prompts/compiler/stdlib/loading.md     | 198 +++++++++-
00_nucleo/prompts/wiring.md                      |  56 ++-
01_core/src/compiler/eval/call_dispatch.rs       |  92 ++++-
01_core/src/compiler/stdlib/loading.rs           | 466 ++++++++++++++++++++---
04_wiring/src/main.rs                            |  30 +-
6 files changed, 837 insertions(+), 58 deletions(-)
```

Os novos documentos/testes auxiliares P1323 são não rastreados e identificados
pelos recibos/fechamento. Nenhum commit, staging ou push foi realizado.

## Verificação do produto

O teste unitário independente foi integrado antes da correção. Uma preparação
mecânica extraiu o literal antigo para constante sem mudar bytes de saída;
os quatro perfis do baseline foram preservados (`p1323-seam-preservation.json`).
O RED falhou na asserção real, exit 101 (`p1323-unit-red.json`), não por erro de
compilação ou ausência de testes. Após a implementação, o mesmo teste passou
(`p1323-unit-green.json`); o snippet congelado não foi alterado.

| Gate | Medição / recibo |
|---|---|
| Build workspace release locked | exit 0, `p1323-candidate-build.json`, 2026-09-08T23:25:11.126216Z |
| Testes workspace release locked | 6.667 passaram, 0 falharam, 3 ignorados, `p1323-workspace-tests.json`, 2026-09-08T23:26:29.330635Z |
| Formatação | exit 0, `p1323-fmt.json` |
| Lint completo | 0 erros, 240 warnings, 1.138 infos, `p1323-final-lint.json`, 2026-09-08T23:30:49.043083Z |
| V5/V15/V26 com fail-on warning | exit 0, sem violations, `p1323-final-lineage-r2.json` |
| Diff check final | exit 0, `p1323-final-diff-check-r2.json` |

O lint completo não está livre de avisos. Seu dry-run de resselo devolveu
`Nothing to fix` mesmo com metadado recíproco desatualizado. A inspeção do
algoritmo canônico em `tekt-linter/03_infra/hash_writer.rs` e `prompt_io.rs`,
com cálculo independente do revisor, determinou `Hash do Código: 136cabde`.
Foi corrigida somente essa linha (`p1323-reciprocal-correction.json`), sem
alterar corpo normativo, fonte compilada, binário ou testes. O hash efetivo
do L0/header é `dd1ca444`, com os três pins dos Núcleos preservados.

## A/B: resultados e correção do comparador

A suíte congela casos HTML habilitado/desabilitado, serializações, erro HTML,
PDF/PNG/SVG, query, eval e help, em ordem normal, repetida e reversa.
O gate sem feature mede ausência/rejeição, não sucesso HTML. A calibração
do warning rejeitou seis alterações do observável e preservou o controle
positivo; o caso deliberadamente opaco permaneceu Unknown. São alterações
de cópias do observável, não mutantes do código produtivo.

O primeiro recibo `p1323-ab-process-receipt.json` NÃO passou: o controle PDF
falhou por hash integral e instabilidade entre repetições do próprio baseline.
Os PDFs daquela execução eram efêmeros e foram removidos pelo runner; isso é
uma limitação real, não recuperada pelo recibo. Não se atribui retroativamente
causa comprovada a esses arquivos indisponíveis.

O focal sucessor reteve novas amostras baseline/candidato e mostrou variação
somente no payload de `xmpMM:InstanceID`; texto, página, caixas, fontes,
imagens e render permaneceram iguais. Evidência:
`p1323-ab-pdf-focal-receipt.json`. `--document-id` fixa DocumentID, mas não
InstanceID. O qpdf acrescenta seu próprio ID ao reexportar, portanto seu
arquivo reescrito não foi usado para normalizar o PDF original.

R2 preserva todos os bytes raw, exceto o payload base64 estritamente válido
de 16 bytes do único InstanceID. Mantém hash bruto, localização e valor
excluído; ausência, duplicação e formato opaco viram Unknown. A calibração
rejeitou mudanças de MediaBox, fonte e DocumentID. O comparador não ignora
metadados em geral nem compara bytes PDF com vanilla. Suíte e runner R1
permanecem intactos; a revisão vive em `p1323-ab-frozen-suite-r2.json` e
`p1323-ab-cli-r2.py`. A execução R2 iniciada em
`2026-09-08T23:32:35.377774Z`, registrada em
`p1323-ab-process-receipt-r2.json`, produziu 45 checks de warning e 45 de
preservação `Preserved`, com 60 comparações de ordem estáveis. São os mesmos
15 casos em três ordens, não 90 casos independentes. Os PDFs R2 estão retidos;
o parecer independente aceitou a revisão focal e esses resultados, sem
transformar o recibo R1 em aprovação.

## Execução e segregação

A skill `tekt-materializacao-segregada` orientou o regime A/B proporcional:
root redigiu intenção/L0 e implementou; `/root/p1323_tests` escreveu os testes
a partir de L0/vanilla sem ler código candidato; `/root/p1323_review` auditou
sem editar o que julgou. O ambiente é compartilhado: execução sem atestação
técnica de isolamento, sem selo de refinamento ou equivalência funcional geral.
A inspeção focal dos PDFs pelo autor dos testes usou a skill PDF.

A tentativa autorizada de target na RAM encontrou ENOSPC durante a cópia,
antes de compilar: a visão de espaço do sandbox divergia da visão do host.
Somente a cópia incompleta desta tentativa foi movida de
`/dev/shm/p1323-target.M0xPMi` para `/tmp/p1323-ram-copy-failed.M0xPMi`,
preservando-a. O build usou `/tmp/p1323-target.9NrOxY`, cópia sem hardlinks.
O recibo `p1323-target-fallback.json` sucede apenas a logística RAM do plano
e manifesto; não altera a obrigação. Temporários de outros passos não foram
removidos. Nenhuma alteração do exportador PDF foi necessária.
