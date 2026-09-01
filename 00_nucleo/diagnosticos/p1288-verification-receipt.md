# P1288 — receipt de verificação final independente

## Juízo

| Questão | Resultado |
|---|---|
| Verificação deste relatório | **PASS** |
| Fragmento funcional P1288 do candidato | **PASS** |
| Gate de refinamento do selo pré-candidato | **FAIL** |
| Coerência do veredito do relatório | **PASS — `NOT REFINED` confirmado** |

O candidato preserva o contrato funcional congelado, mas o selo declara que
qualquer drift byte a byte dos 15 L0s o invalida. Todos os 15 diferem dos pins
pré-candidato e o controle A22 termina em falha. Portanto seria incoerente
emitir `REFINED`; `NOT REFINED` é o único dos vereditos permitidos compatível
com a evidência atual.

## Papel, autoridade e limite de independência

- Papel: verificador final posterior ao candidato e ao relatório.
- Executor: `/root/p1288_l0`.
- Escrita exclusiva: este receipt, criado via `apply_patch`.
- Nenhum L0, L1–L4, harness, contrato, manifesto, selo, oráculo, campanha ou
  relatório foi editado ou corrigido neste papel.
- Este executor não escreveu candidato, contrato, manifesto, selo, oráculos ou
  relatório. Porém foi autor/auditor de Núcleo + L0 e adversário em turnos
  anteriores. A independência final é, portanto, parcial: segregação por
  artefatos, capacidade e ordem, sem independência nominal absoluta nem
  atestação de isolamento ambiental forte.

## Proveniência

- Instante final antes da escrita deste receipt:
  `2026-08-31T13:46:09-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Working tree: não commitada e compartilhada.
- `git diff HEAD --stat`: `155 files changed, 650686 insertions(+), 2097
  deletions(-)`, exatamente o estado registrado no relatório.
- Vanilla `/usr/local/bin/typst`:
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Candidato `target/release/typst`:
  `5f2841b03cc776dfdc55b43211a8f1f90307f8ba8b5b1f0f448a6adb0398553d`.
- Selo pré-candidato:
  `22042fef8243a7bf9ce0f8e4ce1b5300dbc0bd1a13da97e97059bf59983b397e`.

Manifesto, contrato, runner/testes/baseline de oráculos, log adversarial e
Núcleo continuam nos hashes pinados pelo selo. Os testes dos oráculos também
revalidaram baseline, receipts e as 12 fixtures.

## Reexecução do contrato e dos oráculos

```text
python3 lab/parity/matrix/p1288_oracles.py \
  --binary target/release/typst --order both \
  --output /tmp/p1288-verify-oracles.json
python3 -m unittest lab/parity/matrix/test_p1288_candidate_contract.py
python3 -m unittest lab/parity/matrix/test_p1288_oracles.py
```

Resultados reproduzidos:

- candidato identificado como `future-binary` com o SHA acima;
- `46 Preserved`, `0 Violated`, `4 Unknown` deliberados;
- forward/reverse idênticos;
- output SHA-256
  `9702384222e5acd89d5365ec03ba562b64c170b01e85c3dd5c8a540e49bd426c`;
- contrato candidato: `10/10`, `OK`;
- oráculos: `10/10`, `OK`, incluindo `SELF_TEST_OK`.

Os quatro `Unknown` são somente AT/screen reader e tecnologia assistiva real,
PDF/UA/reflow/certificação, bundle e relações PDF não decodificadas. Não foram
contados como sucesso funcional.

## Matriz dos três perfis

Os três comandos `runner.py --profile ...` terminaram com exit 0, 20 casos
cada, totais fechados e forward/reverse idênticos:

| Perfil | MATCH | DIFFERENCE | DISABLED | Outros | SHA-256 |
|---|---:|---:|---:|---:|---|
| default | 15 | 2 | 3 | 0 | `b7fc4fa6bc47b07781f0192260cd0a36ce4f63c8c44432712a3959cfe95e0221` |
| html | 17 | 2 | 1 | 0 | `1db96db4077171caf46a5b63acb8de09363647880db7a885076eddd8228250d8` |
| a11y-extras | 16 | 2 | 2 | 0 | `7a8a41b6b5b93c5f8abaae2e24edc0093bb718f0fb84be1ac536f91d965d82c4` |

O harness atual congela exatamente os perfis default/html/a11y-extras,
mantém bundle como scope-out, não injeta feature em caso sem declaração,
separa `DISABLED_BY_PROFILE` de `MATCH` e degrada divergência de ordem para
`UNKNOWN`. Sua suíte passou `41/41`.

## Campanha e controle A22

O log pré-candidato protegido permanece em
`049a8fbc6b924eec6c1c07fbbd7c930fc89b27cc2a5223a33dd8891891078aae`:
22/22 mutantes válidos mortos, 2 inválidos fora do denominador, 4 opacos,
`mutation_score = 1.0` e ordem idêntica.

No estado pós-candidato, a repetição produziu:

```text
P1288_MUTATION_SCORE=1.000000 KILLED=22/22 INVALID=2 OPAQUE=4
ORDER_AGREEMENT=true PROTECTED_UNCHANGED=true
process exit = 1
control.status = Violated
control.failures.A22 = protected manifest/baseline/fixture/nucleus/L0 drift was ignored
```

O payload SHA-256 foi
`6727bb1495d120fdda4d064171b7f60dbf296b6eccf6e7a020a1d5c74b2bc6a0`,
igual ao relatório. `PROTECTED_UNCHANGED=true` significa apenas que a execução
não modificou mais os inputs; não apaga o drift entre o selo e o estado
pós-candidato.

## Auditoria dos 15 L0s

Os 15 hashes atuais diferem dos pins do selo. A reconstrução substituindo
somente a linha `Hash do Código` reproduziu exatamente o pin em 8:

- `entities/compiler_features.md`, `entities/html.md`, `wiring.md`;
- `entities/elements/table.md`, `entities/elements/table_cell.md`;
- `compiler/eval/table.md`, `compiler/layout/table.md` e
  `compiler/layout/table_cell.md`.

Nos outros 7, restaurar apenas essa linha não reproduz o pin, confirmando
drift textual adicional:

- `shell/cli.md`, `infra/pipeline.md`, `compiler/eval.md`;
- `compiler/stdlib/pdf.md`, `entities/layout_types.md`;
- `infra/export/stream.md` e `infra/export/builder.md`.

Para `compiler_features.md`, o preimage usado foi a linha nua
`Hash do Código: PENDENTE_CONFIRMACAO_HUMANA_P1288` e o blob reconstruído
resultou em
`5f12a9e8ca4b96fa3d9a06a54b5030f509ff2c5331374ff8fe5fb9b5ca567826`,
o pin exato. Isto é reconstrução por preimage compatível com o pin, não um
snapshot textual pré-gate preservado; a limitação não altera A22 porque o hash
atual diverge de qualquer modo.

## Gates finais reproduzidos

| Gate | Resultado |
|---|---|
| `cargo test --workspace -q` | exit 0; 5343 + 911 + 1 + 61 + 2 + 71 + 2 + 6 + 1 passaram; 3 ignorados |
| surface-inventory | 22/22, `OK` |
| harness | 41/41, `OK` |
| contrato P1288 | 10/10, `OK` |
| oráculos P1288 | 10/10 + self-test, `OK` |
| `cargo fmt --all -- --check` | exit 0 |
| `crystalline-lint --quiet .` | exit 0 |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | zero violações |
| `git diff --check` | exit 0 |

Warnings Rust existentes foram emitidos, mas nenhum teste falhou e os gates
registrados não usam `-D warnings`.

## Residual P1287 e conclusão

`p1287_global.py --binary p1287_candidate_adapter.py` terminou com exit 1 e
reproduziu `13 Violated` e `10 Unknown`. O adapter, baseline e manifesto
coincidem com a proveniência registrada. Esse residual permanece visível, não
é convertido em sucesso e, conforme o escopo do Passo 1288, não invalida por
si só o fragmento P1288. A alegação histórica 72/77 continua externa e não foi
substituída por proxy.

Conclusão final: os resultados funcionais do P1288 estão verdes, mas a cadeia
selada não está refinada porque A22 detecta drift pós-selo. A verificação do
relatório **PASSA** e seu veredito **`NOT REFINED` é coerente e confirmado**.
