# P1301 revisão 2 — recibo do autor do oráculo

## Resultado

Suite independente do oráculo vanilla materializada em
`00_nucleo/diagnosticos/p1301r2-oracle-suite.json`, SHA-256
`4c1cbbafd5e81517061a2c369186a41d6de30a41abc29ff122ee9e77ed64b3ac`.

Estado: `ORACLE_SUITE_MEASURED`; resultado do fragmento vanilla: `PASS`.
Este recibo não é selo do contrato, ataque, implementação, teste Rust nem
veredito de equivalência funcional geral.

Linguagem de atestação: **executado sem atestação de isolamento técnico**.

## Papel e capacidades

- Papel: `P2-r2`, autor segregado do oráculo.
- Ambiente: `/repos/Antigravity/typst-crystalline`, working tree não commitida.
- Contexto herdado: tarefa restrita P1301 revisão 2, `AGENTS.md` fornecido pelo
  orquestrador e referências operacionais externas da skill
  `tekt-materializacao-segregada`.
- Leituras do repositório limitadas ao manifesto r2, contrato r2 e recibo do
  autor do contrato r2. Foi executado somente o baseline ratificado
  `/usr/local/bin/typst`.
- Escritas autorizadas e realizadas: somente a suite do oráculo e este recibo,
  ambas via `apply_patch`.
- Não foram lidos implementação candidata, testes Rust candidatos, artefactos
  P1301 v1, `00_nucleo/materialization/` nem `00_nucleo/context/`.
- Não foram usados fixtures de filesystem nem ficheiros temporários; portanto
  não houve temporários fora de `/dev/shm`.

Regime aplicado: protocolo completo da skill `tekt-materializacao-segregada`.
Este papel encerra-se na autoria do oráculo e não controla implementação ou
veredito final.

## Entradas congeladas

| Entrada | SHA-256 observado | Resultado |
|---|---|---|
| `00_nucleo/diagnosticos/p1301r2-manifest.json` | `1526dc81e08a36ce0153fe7639dd2558c220a85d44c4b1de7098a59c099a5712` | confere |
| `00_nucleo/diagnosticos/p1301r2-contract.json` | `69e852d38636afd0ce996f10a64cbf0e6dc0be12dca453a65ee3e0d515c88fad` | confere |
| `00_nucleo/diagnosticos/p1301r2-contract-author-receipt.md` | `34c777fa06897e6dffe0efa40abc4cdcae67bcff151722113abef08b40b68318` | registado |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` | confere com o baseline ratificado `a51e02804` |

## Execução reproduzível

Foram executados os 22 casos vanilla nas ordens `normal` e `inverted`, num
total de 44 processos decisórios para esta suite. Cada comando concreto está
registado em `runs.<ordem>[].argv`; todos obedecem exclusivamente à forma:

```text
/usr/local/bin/typst eval <expressão> --format json [--features <perfil>]
```

Os únicos sufixos de perfil usados foram os quatro congelados no contrato:
nenhum argumento `--diagnostic-format`, `--target`, seleção de renderer ou
canal interno foi usado.

Para cada execução, a suite preserva o exit code, stdout exato, stderr bruto
UTF-8 e SHA-256 do stderr. O adaptador
`vanilla_ratified_human_v1/native_human_single_primary_ascii_v1` apenas extrai
classe, mensagem, hints na ordem observada e span; não reescreve semântica,
aspas, nomes ou offsets.

O comando de reprodução de cada observação é o respetivo array `argv`, sem
shell. A verificação estrutural final foi:

```text
python3 -m json.tool 00_nucleo/diagnosticos/p1301r2-oracle-suite.json
```

Resultado: exit `0`.

## Contagens e ordem

| Ordem | Total | Preserved | Violated | Unknown |
|---|---:|---:|---:|---:|
| normal | 22 | 20 | 0 | 2 |
| inverted | 22 | 20 | 0 | 2 |

Os 20 casos não opacos coincidem exatamente com o envelope esperado. Os casos
`V21` e `V22`, declarados opacos antes da execução, permanecem
`Unknown/UNSUPPORTED_DIAGNOSTIC_CARDINALITY`; `exact_axis` permanece `null` e
nenhum envelope foi sintetizado.

Os vetores canónicos ordenados por `case_id` são byte-idênticos:

- normal: `d9da688b205bec20d6548a59017df64b53845681566b76700c30f7f0b1d3d31e`;
- inverted: `d9da688b205bec20d6548a59017df64b53845681566b76700c30f7f0b1d3d31e`.

Resultado de independência de ordem: `PASS`.

## Sentinela cristalina

`C23-dictionary-crystalline-span-sentinel` não pertence à partição vanilla e
não foi executada contra o vanilla para decisão. A suite mantém literalmente a
expectativa contratual cristalina `span_bytes: [10, 21]` com estado
`NOT_RUN_NON_DECISORY`. A referência vanilla separada `V20` mediu `[14, 21]`;
uma não foi usada para substituir ou reinterpretar a outra.

## Proveniência da medição

- Instante de início da medição materializada:
  `2026-09-03T21:54:36.744011-03:00`.
- HEAD: `1f082370e59939de7b57992e137a9f74bfb6758f`.
- Estado: working tree não commitida.
- `git status --short` anterior à escrita da suite: SHA-256
  `bf0d5851b95ae8b2bbc7f734b633198cd6e687cd7e9c4ab1ac44c749da5aedbc`.
- `git diff HEAD --stat` anterior à escrita da suite: SHA-256
  `df861a696777f2714969a76128a6c5deafd7383537223c9dbb1efc2f1f1e12b6`.
- O conteúdo bruto de `git diff HEAD --stat` está preservado em
  `measurement_provenance`; o `git status --short` foi conservado somente pelo
  seu SHA-256, para não persistir uma enumeração de namespaces cuja leitura é
  restrita. O diff stat regista exatamente os ficheiros tracked alterados.

## Limites

O `stderr` nativo é evidência reprodutível, mas não campo de igualdade entre
renderers. Esta autoria não inventa equivalência entre renderizadores e atesta
somente o fragmento observável congelado no contrato r2. `Unknown` não foi
promovido a sucesso fora dos dois casos opacos previamente declarados.
