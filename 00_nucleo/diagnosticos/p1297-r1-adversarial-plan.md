# P1297/R1 — plano adversarial e de calibração da capacidade armada

## Regime, papel e limite da alegação

Regime Tekt completo. Esta fase exerce exclusivamente P3-R1,
adversário/calibrador independente posterior ao RED e anterior ao selo. A
segregação é por entradas causais congeladas, capacidades de escrita, ordem e
artefatos canônicos; o workspace compartilhado não permite alegar isolamento
técnico de leitura. Nenhuma implementação candidata P5 existe ou foi lida.

O papel pode escrever no repositório apenas este plano e
`p1297-r1-discrimination-receipt.json`. Controle calibrador, mutantes, cópias,
targets, runners e logs ficam exclusivamente sob `/tmp`. P3 não edita L0,
owner/consumer de contrato, produto, headers, manifesto, ledger, receipts
predecessores, selo ou certificado.

Claim máximo desta fase: poder discriminatório do contrato R1 para os quatro
mutantes pinados, nas versões e observáveis registrados. Não se alega
equivalência funcional geral nem prontidão para implementação.

## Entradas congeladas

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1297.md` | `5116ffc535bb8cbd33ca8fb449b492ddfcf9e133246d7143c27a97960748c682` |
| `00_nucleo/diagnosticos/p1297-manifest.json` | `5bf3f29e353403006b90971e05f786ad2548648c053b9db59f879841aecdf322` |
| `00_nucleo/diagnosticos/p1297-redesign-ledger.json` | `f0959318cd00b9801d22268e608d063ba9d924d1d5362a52614cb84e8f764a93` |
| `00_nucleo/diagnosticos/p1297-r1-l0-gate-receipt.md` | `11777b20d0fc62256077339c35b86c559ba90cce167da2aee2ca081a9bd446e2` |
| `00_nucleo/diagnosticos/p1297-r1-red-tests-receipt.json` | `2b6a89b1fb1eca99f516470c5c339f9c2321234180e2cee86ef3249e117285be` |
| `00_nucleo/prompts/shell/watch.md` | `0988aae004f94b52730c1b605b20249e261bdeac70249b7f99d2f0deeee23483` |
| `00_nucleo/prompts/wiring.md` | `4e0661d71303fc367d9f29992809e21ecc3b66fcb6e4869eb45d705f8fe5876d` |
| `00_nucleo/prompts/wiring/tests/cli.md` | `a1190628c91f0f8c958bad2babe68e5f425931e7f885e8d97bc8b95f9ecc13ad` |
| owner R1 | `4b30a7da9310892ad639ba2483fd8d995f6cae4adb4f5ec959247b7911f583e8` |
| consumer R1 | `9721cfb5fce6dc5bb9c41d004495a0838069c2f9ae2492731fde30b53827f089` |
| baseline `03_infra/src/watch.rs` | `0232f1baa3b06bde809941938ea0633982a78ec066ce8e38466f346e63403a35` |
| baseline `04_wiring/src/main.rs` | `03dfc017bb832264b16d50c6314c1e0d48304f657f2054af378032944c747678` |
| baseline `04_wiring/tests/cli.rs` | `56d989a4f79112d59558a71b0dbf609aa4022f9df0290e56afebc1b2b218a42b` |
| contrato P1295 | `8dae68ffc12f2708b6bbe3d503a7d0d90a2cb92681762ff5a992ca3f178c7abf` |
| receipt vermelho P1295 | `06104b7d41f8e456e67df90c8036ba4362af67f3247880278bfa587e1d2c0573` |
| receipt P2 final P1296 | `be8aaf3537bb068aab8eff1c5b656fe74c56cec8bd144f22e48a12b486d3e8a0` |
| receipt discriminatório P1296 | `51073959b26cc834f1f1dbe69b5c4e70b5e7f1c71d28282b7424e0306e641525` |

Política: `Unknown` nunca recebe crédito; survivor, `Unknown` inesperado ou
regressão de controle encerra R1 e impede selo. Score requerido: `4/4 = 1.0`.

## Controle de calibração temporário

A cópia exclui `.git`, `target`, `00_nucleo/materialization/` e
`00_nucleo/context/`. O cache `target` é copiado por reflink para `/tmp`; após
uma preflight detectar reutilização cruzada indevida, cada variante recebe
target isolado e limpeza restrita de `typst-infra` antes do primeiro build.

O controle é descrito apenas estruturalmente: capacidade `ArmedWatch` com
snapshot já materializado; `arm` captura; `publish`/`abandon` consomem e
devolvem o mesmo snapshot; helpers crus são privados; L4 compõe
`compile → normalize → arm → publish/abandon → evict → wait`. Seu corpo não é
artefato canônico, não pode ser promovido e não será entregue a P5.

## Mutantes independentes e testemunhas

| ID | Mutação negativa | Testemunha causal requerida |
|---|---|---|
| R1M1 | capacidade guarda paths e `abandon` captura depois de remover | o caso sobreposto de abandono não retorna, pois `None` virou baseline |
| R1M2 | `publish` renomeia antes de capturar o baseline devolvido | o caso sobreposto de publicação não retorna, pois staging ausente virou baseline |
| R1M3 | helpers crus públicos; L4 finaliza por bypass antes de armar | rustdoc JSON enumera e rejeita `commit_output`/`discard_output` públicos |
| R1M4 | `wait_for_change_since` recaptura e substitui o baseline recebido | transições já concluídas deixam de ser detectadas; P1295 também deve rejeitar recaptura |

Cada mutante deriva de uma cópia independente do controle. Entre mutantes o
controle é reexecutado; restauração é provada pelos hashes do controle, não por
rollback do working tree do usuário.

## Ordem e gates

1. Recorte focal do controle e controles de fronteira: abandono, publicação,
   erros originais de rename/cleanup, preservação do destino, assinaturas e
   inventário rustdoc pinado.
2. Recorte focal R1M1–R1M4. Só prosseguir se controle preservado e cada mutante
   possuir testemunha específica.
3. Controle completo inicial: R1 `7/7`, P1295 `6/6`, P1137 watch `2/2`.
4. Ordem direta R1M1→R1M4, com o mesmo corpus e controle completo reposto entre
   todos os casos.
5. Ordem inversa R1M4→R1M1, novamente com controle completo entre todos os
   casos, seguida de controle final.
6. Revalidar hashes protegidos, HEAD, working tree, índice vazio e logs.

Classificação: controle verde é `Preserved`; mutante com a testemunha prevista
é `Killed`; mutante verde é `Survived`; identidade, construção ou execução
ambígua é `Unknown`. Ensaio inválido não entra no denominador e não autoriza
revisão sem o `reason_code` e a correção mecânica permitida.

## Budget e condições de parada

Budget: quatro mutantes, duas ordens completas, um controle antes/depois e
entre casos, e no máximo uma correção mecânica focal. São proibidos aumento de
timeout, sleep de prontidão, carga, retry-until-pass, polling test-side ou
alteração do corpus. Qualquer regressão, survivor ou `Unknown` produz receipt
bloqueante; P3 não corrige contrato/produto e não inicia R2. Passando todos os
gates, P3 autoriza somente P4-R1 a avaliar o selo.
