# P1372 — relatório do observador vertical S07

## Veredito

**DIFFERENCE_WITH_LIMITED_ATTESTATION.** A sonda textualmente equivalente
fecha a limitação instrumental de S07 e encontra diferença vertical nos dois
perfis (`a11y-extras` e `html+a11y`). No vanila, `After` aparece na página de
índice 1 em `yMin=7,404`; no cristalino, na página de índice 0 em
`yMin=42,230`. Lone e controle coincidem dentro de cada produto, portanto a
diferença não é causada pelo metadata nem por textos distintos: o cristalino
não reproduz a expansão de `block(height: 100%)` observada no vanila.

O passo fecha apenas `S07_VERTICAL_DISCRIMINATION` como lacuna de observador e
abre evidência de gap de produto. A morfologia das tags PDF continua sendo um
eixo separado e não é reclassificada aqui.

## Correção do oráculo durante a medição

A primeira matriz completa demonstrou que comparar apenas o delta
lone-controle era insuficiente: ambos os produtos produziram delta de página
0 e delta Y `0,000`, apesar de suas posições absolutas divergirem. Esse
resultado preliminar foi preservado em `p1372-calibration-rejected.json`
(SHA-256 `81205948…4847`) e não recebe crédito de paridade.

Contrato, oráculo, teste e executor foram então corrigidos para usar a
assinatura formada pelas duas posições absolutas mais o delta. O teste focal
falhou pela ausência de `vertical_signature` e passou após sua implementação.
A matriz focal revista classificou os dois perfis como `DIFFERENCE`.

## Execução e proteção de memória

- jobs Typst: 8/8; observações `pdftotext`: 8/8;
- processos simultâneos: 1;
- teto de endereço por subprocesso: 2 GiB;
- teto de arquivo por subprocesso: 16 MiB;
- leitura máxima por canal: 1 MiB;
- timeout e CPU por subprocesso: 45 s;
- maior subprocesso: 316,052 ms;
- maior PDF: 6.244 bytes;
- maior stdout: 1.137 bytes; maior stderr: 0 bytes.

Não houve OOM, `SIGKILL`, timeout, estouro de canal ou estouro de arquivo. O
pico real de RSS não foi medido. Não houve Cargo, rustc, build, watch, rede ou
suíte ampla. A tentativa anterior ao primeiro job foi bloqueada pelo pin de
`pdftotext` transcrito incorretamente; nenhum compilador foi iniciado nela.

## Proveniência e regime

A medição final ocorreu em 2026-09-13T16:02:00Z sobre o commit
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, com working tree não commitado.
Os arquivos próprios do P1372 presentes eram passo, contrato, oráculos,
executor, teste, calibração rejeitada e resultado. O repositório já continha
alterações não relacionadas de P1356–P1371 e código de produto; a listagem
integral foi observada por `git status --short` e `git diff HEAD --stat` no
mesmo turno. Por compartilharem workspace e autoridade, autor, implementador
e verificador não têm isolamento forte: protocolo completo executado sem
atestação de isolamento.

Identidades congeladas: vanila `eb60986b…1468f`, cristalino
`60851811…761f`, `/usr/bin/pdftotext` `0fb98ea1…565e7`. O resultado final tem
SHA-256 `2204623b…1ec`.

## Gates

`py_compile`, o teste focal (1/1), a validação dos quatro JSONs,
`git diff --check` e `crystalline-lint .` retornaram exit 0. O lint foi
executado com 2 GiB de memória virtual, arquivo limitado a 32 MiB e timeout de
180 s; apresentou warnings/infos preexistentes e saída visual truncada, sem
violação bloqueante.
