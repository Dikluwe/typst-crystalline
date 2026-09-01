# P1288 — receipt de verificação discriminatória pré-selo

## Papel, regime e fronteira

- Regime: protocolo completo da skill `tekt-materializacao-segregada`.
- Papel: **verificador discriminatório pré-selo**, posterior ao contrato,
  baseline vanilla, Núcleo/L0 final, oráculos e campanha adversarial; anterior
  ao candidato, implementação, gate humano e veredito funcional.
- Escrita exclusiva deste papel:
  `00_nucleo/diagnosticos/p1288-contract-seal.json` e este receipt.
- Nenhum artefato julgado foi editado. O candidato, `target/release/typst` e
  L1–L4 não foram lidos, inspecionados nem executados por este verificador. A
  única varredura do workspace produtivo foi a execução mecânica autorizada do
  `crystalline-lint` limitada a V15/V26.
- Sobreposição anterior deste executor: autoria/teste do harness da Fase A.
  Esse harness não foi lido, executado, selecionado nem pinado neste papel e
  está explicitamente fora do selo.
- O checkout compartilhado não oferece isolamento ambiental forte. A alegação
  proporcional é **verificação segregada por papel, capacidade, ordem e
  hashes, sem atestação de isolamento ambiental forte**.

## Proveniência

- Verificação concluída em `2026-08-31T11:59:25-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Working tree: não commitada e compartilhada. No fecho, `git diff HEAD
  --stat` registrou `138 files changed, 649022 insertions(+), 1912 deletions(-)`;
  essas contagens identificam o estado compartilhado e não atribuem alterações
  de outros papéis a este verificador.
- Vanilla ratificado: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Linter: `/home/dikluwe/.cargo/bin/crystalline-lint`, SHA-256
  `80bb6b2aa23ce1b83f9a0a2540a4ff61a9a4993aca01b2e44f72bf16378e5cff`.

O selo contém os hashes completos do protocolo, Passo 1288, manifesto e
receipt de contrato, baseline/receipt vanilla, receipt de Núcleo/L0, Núcleo
raw + hash efetivo, 15 L0s, 12 fixtures, runner/testes/baseline/receipt dos
oráculos, driver/log/plano/receipt adversariais e identidades das ferramentas.

## Auditoria estática da cadeia

O manifesto contém exatamente 22 observáveis: 20 exigem `Preserved` e 2
exigem `Unknown`. Os dois `Unknown` cobrem somente AT/screen reader/reflow/
PDF-UA/certificação e bundle/relações PDF não decodificadas; não recebem
crédito de sucesso.

As 22 obrigações adversariais A01–A22 estão enumeradas exatamente. O baseline
oracle contém 27 casos eval, 10 casos de compile, 12 fixtures e 4 casos opacos
deliberados. A campanha exclui 2 mutantes inválidos do denominador e não usa
`Unknown` como morte de mutante válido.

Todos os hashes de `protected_inputs` do manifesto foram recalculados contra o
filesystem antes dos gates: `PROTECTED_OK`. Os pins raw/efetivo do Núcleo e os
pins nos L0s coincidem com o receipt final de L0.

## Gates reexecutados

### Estrutura e self-test

```text
python3 -m json.tool 00_nucleo/diagnosticos/p1288-manifest.json
python3 -m json.tool lab/parity/matrix/p1288-vanilla-baseline.json
python3 -m json.tool lab/parity/matrix/p1288-oracle-baseline.json
python3 -m json.tool lab/parity/matrix/p1288-mutation-campaign.json
python3 -m py_compile lab/parity/matrix/p1288_oracles.py \
  lab/parity/matrix/test_p1288_oracles.py \
  lab/parity/matrix/p1288_adversarial.py
python3 lab/parity/matrix/p1288_oracles.py --self-test
python3 -m unittest lab/parity/matrix/test_p1288_oracles.py
```

Resultado: JSON e bytecode válidos; `SELF_TEST_OK`; 10 testes, todos `OK`.

### Replay vanilla forward/reverse

```text
python3 lab/parity/matrix/p1288_oracles.py \
  --binary /usr/local/bin/typst --order both \
  --output /tmp/p1288-preseal-vanilla.json
```

Resultado reproduzido:

- identidade `ratified-vanilla`;
- 46 `Preserved`, 0 `Violated`, 4 `Unknown` deliberados;
- forward/reverse idênticos;
- SHA-256 do payload
  `04f2c408b9f923624825f6d6909a1deeb4a84c13f6ff8e5b6e6d7f659ae884fd`,
  exatamente o hash registrado pelo autor dos oráculos.

### Campanha discriminatória

```text
python3 lab/parity/matrix/p1288_adversarial.py \
  --output /tmp/p1288-preseal-campaign.json
python3 -m json.tool /tmp/p1288-preseal-campaign.json
cmp /tmp/p1288-preseal-campaign.json \
  lab/parity/matrix/p1288-mutation-campaign.json
```

Resultado:

```text
P1288_MUTATION_SCORE=1.000000 KILLED=22/22 INVALID=2 OPAQUE=4
ORDER_AGREEMENT=true PROTECTED_UNCHANGED=true
```

O payload reproduzido foi byte-idêntico ao persistente, SHA-256
`049a8fbc6b924eec6c1c07fbbd7c930fc89b27cc2a5223a33dd8891891078aae`.
Logo, `mutation_score = 22/22 = 1.0`, sem sobrevivente válido.

### Ownership, Núcleo e diff

```text
crystalline-lint --checks v15,v26 --fail-on warning .
git diff --check -- 00_nucleo/prompts 00_nucleo/diagnosticos \
  lab/parity/matrix
```

Resultado: `✓ No violations found`; diff-check sem output, exit 0.

## Selo e limites

O gate discriminatório pré-candidato passou e o contrato foi selado somente
para as entradas e o fragmento observável pinados. Selo:

- caminho: `00_nucleo/diagnosticos/p1288-contract-seal.json`;
- SHA-256:
  `22042fef8243a7bf9ce0f8e4ce1b5300dbc0bd1a13da97e97059bf59983b397e`;
- estado: `SEALED_PRE_CANDIDATE_CONTRACT`;
- mutation score: `1.0`.

Qualquer drift de um byte em entrada pinada invalida o selo desde a primeira
fase causal afetada. O selo não inclui o harness da Fase A, não aprova o gate
humano ADR-0127, não autoriza por si só implementação, não verifica candidato,
não emite veredito de refinamento e não afirma PDF/UA, AT real, reflow,
certificação ou equivalência funcional geral.
