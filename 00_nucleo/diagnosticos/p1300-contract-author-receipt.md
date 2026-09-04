# P1300 — recibo do autor independente do contrato observável

## Identidade e regime

- Passo: `P1300`.
- Papel: `P1_contract_author`.
- Executor: agente Codex `/root/p1300_contract`.
- Ambiente: host `Dell-G15-Linuxmint`, `Linux 6.17.0-42-generic x86_64`, shell `bash`, repositório `/repos/Antigravity/typst-crystalline`.
- Instante de conclusão da autoria: `2026-09-03T19:15:47-03:00`.
- Predecessor causal: manifesto P1300 SHA-256 `c3e946e1330ccab108dba2f9a438fd6a8d4c3a2af7fca3307842dbd7403feed8`, sobre `HEAD 1f082370e59939de7b57992e137a9f74bfb6758f`.
- Regime: protocolo completo Tekt, **executado sem atestação de isolamento técnico**. O filesystem foi compartilhado; a separação foi aplicada por allowlists explícitas de contexto, leitura e escrita, sem alegar isolamento que o ambiente não prova.

## Contexto e capacidades

O agente iniciou em contexto fresh para P1, contendo a diretiva do papel, as instruções do repositório fornecidas pelo ambiente e as instruções operacionais obrigatórias da skill `tekt-materializacao-segregada`. Não recebeu patch candidato, conclusões de implementação, oráculos, mutantes, selo ou veredito P2–P8.

Allowlist de leitura do repositório:

- `00_nucleo/diagnosticos/p1300-manifest.json`;
- `00_nucleo/prompts/compiler/stdlib/color.md`;
- `00_nucleo/prompts/compiler/eval.md`;
- `00_nucleo/prompts/compiler/eval/tests.md`;
- `00_nucleo/diagnosticos/p1300-pre-gate-measurement.json`;
- se necessário, `lab/typst-original/crates/typst-library/src/lib.rs` e `/usr/local/bin/typst`.

Os dois últimos inputs opcionais não foram lidos nem executados: os observáveis já medidos foram suficientes, e os outputs vanilla ausentes foram deliberadamente remetidos a P2 com `expectation_source: "oracle_suite"` e parâmetros explícitos de probe.

Fora do repositório, foram lidas somente as instruções impostas pela skill:

- `/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md`;
- `/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/papeis-e-capacidades.md`;
- `/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/artefatos-e-gates.md`.

Allowlist de escrita:

- `00_nucleo/diagnosticos/p1300-contract.json`;
- `00_nucleo/diagnosticos/p1300-contract-author-receipt.md`.

Nenhum outro caminho foi escrito. Não houve staging, commit ou push.

## Entradas congeladas e hashes efetivamente conferidos

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1300-manifest.json` | `c3e946e1330ccab108dba2f9a438fd6a8d4c3a2af7fca3307842dbd7403feed8` |
| `00_nucleo/prompts/compiler/stdlib/color.md` | `8115021062602c8a3663797b3fcfe0f17eb423f54a8ae7b437b260804ad58462` |
| `00_nucleo/prompts/compiler/eval.md` | `3bf911a0882e35f713cf8742f70ea921086a9a95ede6b02b8b938ab92becfc96` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `d4d6382d351e084869445fe58e1cb484e762d5b5dfc843b7974cd21149289104` |
| `00_nucleo/diagnosticos/p1300-pre-gate-measurement.json` | `1678b1aed89bfff7581a8fdd998a32f57df18f134226e680596c587620ea3efe` |

O hash vanilla `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` e a revisão `a51e02804` foram transportados do manifesto congelado; não são apresentados como uma nova medição deste papel.

## Saídas

| Saída | Estado | SHA-256 |
|---|---|---|
| `00_nucleo/diagnosticos/p1300-contract.json` | contrato candidato JSON válido, revisão focal 1, 662 linhas | `c5eace73e5ff07ab49057ec1741283543408accf132ca2f4431a1ad3827a3e37` |
| `00_nucleo/diagnosticos/p1300-contract-author-receipt.md` | este recibo | auto-hash não embutido; calculado e reportado após a gravação |

O contrato é binding-free/data-driven. Ele fixa os quatro perfis, diagnósticos negativos já medidos, rotas positivas, globals ratificados, `color.space`, `color.map`, 18 cores, os extras bare, política fechada `Preserved`/`Violated`/`Unknown`, opacos deliberados, repetição/ordem, 14 mutações nominais mínimas e a alegação máxima. Valores vanilla ainda não existentes nas entradas autorizadas não foram inventados.

## Revisão focal 1 — cardinalidade dos mutantes

- `reason_code`: `MUTATION_CATALOG_CARDINALITY_MISMATCH`.
- Predecessor da revisão: contrato SHA-256 `4b138ba6717404a032db6dd26a2bfbafad961fc0a991568c80ae37357caf8a26`.
- Hipótese: desagregar as oito famílias agregadas nos 14 mutantes nominais exigidos e acrescentar os controles públicos que faltavam tornaria cada mutante individualmente endereçável, sem alterar as expectativas previamente congeladas.
- Delta discriminatório: de `8` famílias agregadas com denominador `/8` para `14` mutantes nominais com denominador `/14`; `required_mutation_score` permaneceu `1.0`.
- Controles acrescentados: spelling alternativo bare `linear-rgb`, necessário ao mutante 10, e tabela pública de 12 operadores de cor, necessária ao mutante 14. As expectativas ainda não medidas continuam com `expectation_source: "oracle_suite"`.
- Negativos corretamente `Violated`: não executados neste papel; o contrato declara 14 obrigações com testemunha e reserva a medição ao gate discriminatório P4.
- Positivos corretamente `Preserved`: não executados neste papel; nenhuma expectativa positiva anterior foi removida.
- Opacos corretamente `Unknown`: não executados neste papel; os quatro casos e reason codes anteriores foram preservados.
- Regressões declarativas observadas: `0`; todas as famílias anteriores continuam presentes. Isto não é alegação de regressões funcionais zero, pois nenhum produto ou mutante foi executado.
- Custo material focal: uma revisão do contrato, duas novas famílias/controles declarativos, zero processos bilaterais e zero probes P2. Intervalo do ajuste do contrato: `2026-09-03T19:17:40-03:00`–`2026-09-03T19:19:08-03:00` (`88 s`).
- Resultado focal: causa pública de cardinalidade corrigida; o gate integral e o vetor de classificação continuam pendentes dos papéis P2/P4.

## Comandos e gates reproduzíveis

Comandos de identidade/proveniência usados:

```text
sha256sum 00_nucleo/diagnosticos/p1300-manifest.json 00_nucleo/prompts/compiler/stdlib/color.md 00_nucleo/prompts/compiler/eval.md 00_nucleo/prompts/compiler/eval/tests.md 00_nucleo/diagnosticos/p1300-pre-gate-measurement.json
git rev-parse HEAD
date --iso-8601=seconds
uname -srm
hostname
```

Gate JSON executado após a autoria do contrato:

```text
python3 -m json.tool 00_nucleo/diagnosticos/p1300-contract.json >/dev/null
exit 0
```

Gate final requerido, executado após ambos os outputs:

```text
git diff --check -- 00_nucleo/diagnosticos/p1300-contract.json 00_nucleo/diagnosticos/p1300-contract-author-receipt.md
```

Como os outputs são novos e não rastreados, o gate acima é complementado por `git diff --no-index --check /dev/null <output>` para que whitespace dos próprios ficheiros novos também seja inspecionado. O `exit 1` esperado de `--no-index` significa “há diferenças”; aprovação do check requer ausência de qualquer mensagem de whitespace/error.

## Declaração de independência e limitações

Não li nem toquei `01_core/src/compiler/eval/mod.rs`, `01_core/src/compiler/stdlib/color.rs`, `01_core/src/compiler/eval/tests.rs`, qualquer patch candidato ou artefato P2–P8. Não executei probes reservadas a P2 e não adaptei expectativas a implementação candidata.

Este recibo comprova a disciplina observada pelo executor e os artefatos/hashs reproduzíveis, mas não prova isolamento de leitura ao nível do filesystem compartilhado. Também não sela o contrato: expectativas marcadas `oracle_suite`, o gate discriminatório, o mutation score e o selo pertencem aos papéis posteriores.
