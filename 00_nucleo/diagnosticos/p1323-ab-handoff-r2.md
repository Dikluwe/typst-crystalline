# P1323 — sucessão focal do comparador PDF

Manifesto inalterado: `p1323-manifest.json`, SHA-256
`0ffe55374fa98a5b6ddc40284a52fef616069b38517f582cf79fa52e577cd260`.
Regime A/B sem atestação técnica de isolamento. Autor `/root/p1323_tests`.
Sem leitura de código baseline, código candidato ou patch. Nesta reabertura,
foram lidas saídas públicas do produto; a correção do observador foi derivada
da variação reproduzida no baseline, antes de nova execução integral.

R1 fica integralmente preservada. Seu comparador exigia hash integral do PDF
entre builds cristalinos e em repetições. `--document-id` não tornou esses
bytes determinísticos. Os PDFs efêmeros de R1 foram apagados pelo runner;
portanto não é possível identificar retrospectivamente quais bytes variaram
naquelas amostras. A evidência abaixo prova a causa somente nas amostras
novas, produzidas com os mesmos binários/fixture/argv fixados em R1.

## Medição focal anterior à revisão

Recibo `p1323-ab-pdf-focal-receipt.json`, SHA-256
`b5d898a7a6d5267970a3dea8b55404416341595649287b5c92d4563da44e1b42`,
registra a medição iniciada em `2026-09-08T23:28:35.638423+00:00`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado e
diff stat exato. O focal executou baseline duas vezes e candidato duas vezes,
com o único caso `preserve-pdf` fixado em R1. Cada artefato foi inspecionado
por pdfinfo, pdffonts, pdfimages, pdftotext, qpdf e pdftoppm. A skill PDF foi
usada para inspeção; não houve autoria de conteúdo PDF nem edição dos PDFs.

As quatro saídas têm 5788 bytes, uma página A4, MediaBox
`[0 0 595.2756 841.8898]`, texto `Hello, parity.`, fonte incorporada
`AAAAAA+LibertinusSerif-Regular` e nenhuma imagem. Datas e DocumentID são
iguais. Os PNGs Poppler têm o mesmo SHA-256
`9e3d6e78a2aa2f329786a8a908cd564ff22bbcca5de6847a8923b455871d230e`;
o representante foi inspecionado visualmente, com texto legível e sem recorte.

O diff dos bytes originais, cuja prova completa está na suite R2, mostra
diferenças exclusivamente no payload `<xmpMM:InstanceID>`, intervalo
zero-based `[4900,4924)`. Esse payload tem 24 caracteres base64 canônicos
representando 16 bytes; os primeiros 22 caracteres variam e `==` permanece.
O comando `--document-id` fixa `xmpMM:DocumentID`, não InstanceID. qpdf adiciona
um `/ID` próprio ao reexportar para QDF; esse campo não existe nos PDFs
originais e não foi excluído pelo comparador.

Os PDFs/PNGs/QDFs focais estão retidos em
`p1323-fixtures/ab-pdf-focal-y_gbowo8/`; não foram limpos.

## Comparador sucessor congelado

`p1323-ab-cli-r2.py` altera somente a comparação PDF e a retenção de
artefatos. Casos, argv, warning, fixtures, snippet unitário e demais
comparadores são herdados de R1 sem alteração. Todos os bytes originais PDF
participam da comparação, exceto o payload do único InstanceID canônico
reconhecido. O recibo preserva também SHA-256 raw, tamanho, offset e valor
excluído. Ausência, duplicação, encoding não suportado ou PDF opaco produzem
`Unknown`. Isto é um observador estreito para a fixture medida, não um
normalizador genérico nem prova de paridade PDF com vanilla.

A calibração PDF usa cópias do observável: controles idêntico e alteração
isolada do InstanceID são `Preserved`; alterações de MediaBox, nome de fonte
e DocumentID são `Violated`; ausência do envelope, envelope duplicado e bytes
opacos são `Unknown`. Reaplicação em ordem reversa mantém os resultados.
Não são mutantes produtivos. O ganho desta revisão foi identificar uma fonte
de volatilidade já presente no baseline e discriminá-la de três mudanças
fora desse campo; não houve rodada adicional sem ganho.

Freeze anterior à execução integral R2:

- Runner `p1323-ab-cli-r2.py`: `4fd6d63a200111db9ca595c9b17640184efe79bf8e66275b4b37d9631717c616`.
- Suite `p1323-ab-frozen-suite-r2.json`: `d98a1b73c0dcc5167f145c1195db6ccb9a84294dfce08b2c731c69b9f2248cb1`.

O recibo R2 `p1323-ab-process-receipt-r2.json`, SHA-256
`55e316ba86a7681bd01cef3cd67c3df6e6fed3b43e11cacceb06ae30e9193e9e`,
registra uma execução integral dos mesmos 15 casos em ordem normal,
repetida e inversa: 45 checks de warning `Preserved`, 45 de preservação
`Preserved`, e 60 comparações de estabilidade verdadeiras. Os PDF checks
usam o comparador sucessor; o hash de todos os demais bytes é
`367475fe2ef639c0697a7f93e9a3fa0dded1c918febda69489035eda343c9c81`.
Essas contagens são resultados do observador, não veredito do autor sobre
fechamento ou conformidade geral do produto.

Artefatos R2 ficam em `p1323-fixtures/ab-cli-r2-6l2nvu7q/`, com paths,
hashes e tamanhos no recibo. O verificador independente deve avaliar os
checks e a justificativa do comparador. R1 não foi absolvida ou reescrita.

Comando executado:

```sh
python3 00_nucleo/diagnosticos/p1323-ab-cli-r2.py run \
  --suite 00_nucleo/diagnosticos/p1323-ab-frozen-suite-r2.json \
  --baseline /tmp/p1322-target.Ir19xI/release/typst \
  --candidate /tmp/p1323-target.9NrOxY/release/typst \
  --receipt 00_nucleo/diagnosticos/p1323-ab-process-receipt-r2.json
```
