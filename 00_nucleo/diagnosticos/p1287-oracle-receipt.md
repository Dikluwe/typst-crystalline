# P1287 — receipt do Autor de Oráculos

**Estado:** `ORACLES_FROZEN_WITH_EXPLICIT_UNKNOWNS`  
**Papel:** Autor de Oráculos segregado `/root/oraculos_p1287`.  
**Regime:** protocolo completo da skill `tekt-materializacao-segregada`.  
**Instante final:** `2026-08-30T22:46:10-03:00`.  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.  
**Árvore:** working tree não commitida e compartilhada; sem atestação de isolamento forte.

## Capacidade e entradas

Foram lidos integralmente a skill, as duas referências, `AGENTS.md`, somente o
passo autorizado P1287, manifesto/receipts congelados e os certificados pinados
pelo manifesto. Não foi lido código candidato em `01_core`–`04_wiring`, não foi
executado binário em `target/debug` ou `target/release`, e não foram alterados
contrato, L0, produção, ataques ou veredito. A escrita ficou limitada ao runner,
testes, 12 fixtures, baseline e este receipt.

| Entrada protegida | SHA-256 |
|---|---|
| manifesto P1287 | `5d4cc9a6180300c8402be4a91b30104db08874f8540c9bc8af5b895a9fdf725b` |
| receipt de contrato | `4683d23a5a489e258931ea3389be809c7b4e5b143c4f7a66ec7210c4dbd4f7a5` |
| receipt de medição vanilla | `3bbba47235e4865bb3c9e788bb709215dd0a53297ee587d4334a01ca42126cd6` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

## Artefatos congelados

| Artefato | SHA-256 |
|---|---|
| `lab/parity/matrix/p1287_global.py` | `745a4c470704de54f9a550d220a3092bfa8452e3b02cb7010846758eab36bd37` |
| `lab/parity/matrix/test_p1287_global.py` | `3ac4daa6ff8cb2c9d8396ec4288b737c8496d0473c5d5d901165574af241c535` |
| `lab/parity/matrix/p1287-oracle-baseline.json` | `ff09c8146b73684b2e746c631a6103817b6df7070aa8248e2d88dbca34749488` |
| suíte semântica C01–C12/E01–E11 | `969a10aa549b61c2e118227159d334be482fe24ab82733896b315d67cd4609d0` |
| manifesto canônico das 12 fixtures | `d2141301df14112345b3a28de674380c0c99f189e83ba2b4f5f33e0b0ae5f9ac` |

O runner verifica em cada uso os hashes do manifesto, suíte, fixtures e
baseline. Aceita `--binary` para a comparação candidata futura, executa forward
e reverse e conserva `Preserved`, `Violated`, `Unknown` e `NotApplicable` sem
promover incapacidade. PNG usa o SHA-256 dos bytes RGBA decodificados e dimensão
exata; não existe máscara, tolerância ou comparação aproximada.

## Execuções

```text
python3 lab/parity/matrix/p1287_global.py --freeze
exit 0; baseline vanilla: 16 valid, 7 Unknown

python3 lab/parity/matrix/p1287_global.py --binary /usr/local/bin/typst
exit 2 deliberado; 16 Preserved, 7 Unknown, 0 Violated

python3 -m unittest lab/parity/matrix/test_p1287_global.py
exit 0; 6 testes, zero falhas
```

Os sete `Unknown` são E04–E10 e registram literalmente os carriers ausentes ou
insuficientes: produto completo Luma/CMYK, Conic por espaço, três tilings,
bundle cross-file, matriz completa de imagens, colisão/fontless e máscara/
clipping even-odd. A execução parcial vanilla não os converte em sucesso. E01,
E02, E03 e E11 possuem carriers observados. A identidade que associa os bytes
vanilla ao commit upstream `a51e02804` continua `Unknown`, conforme o contrato.

## Proveniência

No instante acima, `git diff HEAD --stat` registrou `124 files changed,
647273 insertions(+), 1789 deletions(-)`, SHA-256
`8b22c5e3e6a3427dde83ddce564dfa45986b4d49f341a92be0619104f4ec1d5c`;
`git status --short` teve SHA-256
`b2580318745cb27df76951851463642292a54577019fa1ebc6dcdf69a4895f5f`.
Esses números descrevem a árvore compartilhada, não autoria isolada deste papel.

Este receipt congela oráculos, não executa candidato e **não emite veredito
global de paridade**. Resultado proporcional: oráculos segregados por papel,
capacidade, ordem causal e hashes, sem isolamento ambiental forte.
