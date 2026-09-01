# P1288 — receipt adversarial pré-candidato

## Regime, papel e limites

- Regime: protocolo completo da skill `tekt-materializacao-segregada`.
- Papel: **adversário P1288 pré-candidato**, executor `/root/p1288_l0`.
- Sobreposição declarada: o executor foi autor/auditor de Núcleo + L0 em turno
  anterior; nunca foi autor do contrato, dos oráculos, da implementação ou do
  veredito.
- Candidato e harness da Fase A não foram lidos nem executados. Nenhum L0,
  contrato, baseline, fixture, oráculo, código produtivo, harness ou veredito
  foi editado.
- Escritas exclusivas: driver adversarial, log persistente, plano e este
  receipt.
- O checkout compartilhado permite segregação por papel, entradas, ordem e
  hashes, mas não atestação de isolamento ambiental forte.

## Proveniência e entradas

Medição em `2026-08-31T11:49:09-03:00`, HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, working tree não commitada e
compartilhada. `git diff HEAD --stat` registrava 137 ficheiros, 649005
inserções e 1897 remoções; esta contagem identifica o estado compartilhado e
não atribui as mudanças alheias a este papel.

Entradas principais congeladas:

```text
typst-passo-1288.md                    3858860e2d3e9916b051dfbbea0ff66009382bb5daa3fcec8025e9a2492a3e28
p1288-manifest.json                    0e7282b6b5b445d0532bb5456abfbd765e7878fc44d318f96a0b8b960351ed3a
p1288-contract-receipt.md              69d8ea63d2eb6f0267e135cdcd37a479c1b0a0b72814a1170a01be28cf48d34a
p1288-vanilla-baseline.json            057bab7534c4855b0b47cc7cd05e243058d8b39407e30c2b3ee478f04d24c377
p1288-pre-gate-l0-receipt.md           b259334c262683a2e62979b67cbf3b09f57f65ac9e07c4e5c504b6dda5436c6b
p1288-oracle-baseline.json             ac0d97379820eb09f89c69e52858b231794bb4db6455d16527be2d04f1fd7a3d
p1288_oracles.py                       98bb462601c255708a646fdc7dc27e2f59455ade5198b2d26105299934723e48
test_p1288_oracles.py                  59f3c8ea71df9fee0221c4daf50c5fb53c4fde6dd0992eb1fd0aa3e849c77f99
p1288-oracle-receipt.md                894bf70aef8b0cce128f1c3a050a7ed8a2cb895d48feafbc2f97a3c22f108ff4
```

O manifesto e o baseline oracle também enumeram e pinam individualmente
Núcleo, 15 L0s, receipt vanilla e 12 fixtures. O snapshot canônico de todos os
inputs protegidos antes/depois da campanha foi
`6c5f38e088202ba0505153e1287a3a97159f2499b9b73bd673020bb89ce8de00`.

## Campanha

O driver materializou cada mutante em diretório temporário e avaliou A01–A22
isoladamente. A22 alterou somente uma cópia temporária do manifesto para
demonstrar detecção de drift; os originais foram comparados antes/depois.

O runner congelado forneceu carregamento, verificação de hashes e casos de
evidência. Meta-gates derivados do manifesto/contrato cobriram obrigações sem
API decisória isolada no runner: ativação por target, MCID/ParentTree, lattice
e drift direto de Núcleo/L0. Não se atribui ao runner isolado poder que ele não
expõe.

Resultados persistidos:

```text
mutantes válidos             22
mutantes válidos mortos      22
mutantes válidos sobreviventes 0
mutation_score               1.0
mutantes inválidos excluídos 2
casos opacos Unknown         4
forward/reverse              idênticos
inputs protegidos            inalterados
controle sem mutação         Preserved
```

Os quatro `Unknown` são somente os opacos congelados: AT/screen reader,
PDF/UA/reflow/certificação, bundle e relações PDF não decodificadas. Nenhum
mutante válido usa `Unknown` como morte.

## Comandos e gates

```text
python3 -m py_compile lab/parity/matrix/p1288_adversarial.py
python3 lab/parity/matrix/p1288_adversarial.py \
  --output lab/parity/matrix/p1288-mutation-campaign.json
python3 -m json.tool lab/parity/matrix/p1288-mutation-campaign.json
```

Todos terminaram com exit 0. A campanha imprimiu
`P1288_MUTATION_SCORE=1.000000 KILLED=22/22 INVALID=2 OPAQUE=4`,
`ORDER_AGREEMENT=true` e `PROTECTED_UNCHANGED=true`.

## Saídas

```text
lab/parity/matrix/p1288_adversarial.py
  2e23f18ce69dfdd01399b104cf208bebab96b6f4574da9b9340b797ca38b7ddf
lab/parity/matrix/p1288-mutation-campaign.json
  049a8fbc6b924eec6c1c07fbbd7c930fc89b27cc2a5223a33dd8891891078aae
00_nucleo/diagnosticos/p1288-adversarial-mutation-plan.md
  a873ce5dab07ad33acac4dd55aa8c698822551347589ac9c68febeabb00d8307
```

Este receipt registra poder discriminatório pré-candidato e não sela o
contrato, não confirma o gate humano, não verifica implementação e não emite
`REFINED`, `NOT REFINED` ou `INCONCLUSIVE`.
