# P1301 — recibo do autor segregado do contrato observável

## Veredito deste papel

`CONTRACT_AUTHORED_WITHOUT_TECHNICAL_ISOLATION_ATTESTATION`

Foi materializado um contrato observável binding-free para o fragmento P1301.
Este papel não implementou a solução, não escreveu testes candidatos, não
executou mutantes e não sela nem verifica o próprio contrato. O `mutation_score
= 1.0` é o limiar obrigatório do contrato; um score alcançado só pode ser
declarado pelo recibo discriminatório posterior e independente.

## Identidade e regime

- papel: autor do contrato (`P1`, task `/root/p1301_contract`);
- regime: protocolo completo de materialização segregada;
- instante final: `2026-09-03T21:06:51.696979738-03:00`;
- predecessor causal: manifesto P1301 confirmado, SHA-256
  `074fabeda17f741f064fe1d561ad6a78ad81e45654861825586664a9646f05a8`;
- estado de repositório herdado do manifesto: HEAD
  `1f082370e59939de7b57992e137a9f74bfb6758f`, working tree não commitida;
- linguagem de atestação: **executado sem atestação de isolamento técnico**.

## Rehash obrigatório antes da autoria

O comando abaixo foi executado antes da leitura substantiva e da escrita:

```text
sha256sum AGENTS.md 00_nucleo/diagnosticos/p1301-manifest.json 00_nucleo/prompts/compiler/eval/bindings/field_access.md 00_nucleo/prompts/compiler/eval/tests.md 00_nucleo/diagnosticos/p1301-pre-gate-measurement.json
```

Resultado:

| Entrada | SHA-256 refeito | Estado |
|---|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` | lido como diretriz |
| `p1301-manifest.json` | `074fabeda17f741f064fe1d561ad6a78ad81e45654861825586664a9646f05a8` | coincide |
| `field_access.md` | `38d6f5cdee302491ec059577174d4f6dc6d3c28e3d2da2748f61c05853cc0c16` | coincide |
| `tests.md` | `a4ade7eda600891465c62c320d80b7c2182c30dbb5f456de210c4a823b3e609a` | coincide |
| `p1301-pre-gate-measurement.json` | `1202c06723e948f5586e10345c5f653f57b075e3ea41297f996fd5a27abb77cd` | coincide |

Não houve drift; a autoria prosseguiu. O binário vanilla foi refeito como
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
coincidente com o manifesto.

## Capacidades efetivamente usadas

Leitura no repositório limitada a:

- `AGENTS.md`;
- `00_nucleo/diagnosticos/p1301-manifest.json`;
- `00_nucleo/prompts/compiler/eval/bindings/field_access.md`;
- `00_nucleo/prompts/compiler/eval/tests.md`;
- `00_nucleo/diagnosticos/p1301-pre-gate-measurement.json`;
- o próprio output P1301 para validação sintática e hash.

Também foram lidas as instruções operacionais da skill
`tekt-materializacao-segregada` e os seus dois referenciais fora do
repositório. O único executável semântico consultado foi
`/usr/local/bin/typst`. Escrita restrita, via `apply_patch`, a:

- `00_nucleo/diagnosticos/p1301-contract.json`;
- `00_nucleo/diagnosticos/p1301-contract-author-receipt.md`.

Não foram lidos código candidato, testes candidatos, fonte pré-candidata, nem
artefatos P1300; a única informação P1300 usada foi o resumo canônico contido
nas entradas autorizadas.

## Contrato produzido

| Artefato | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1301-contract.json` | `2d8d7a141d63c13473681abdefa24ed9370e03925824a9cae74ca26305ea85f5` |

O contrato contém:

- regra categórica para todo field ausente de módulo, incluindo projeção
  pública `std` → `global` e nome público ordinário para `calc`, `sym` e
  `color.map`;
- igualdade exata de classe, mensagem, hints, span, exit code, stdout e stderr;
- as 12 coordenadas dos três aliases `std` nos quatro perfis;
- três controles negativos independentes de categoria;
- sete controles positivos de lookup/kind;
- um sentinel não-Module que preserva o span total cristalino vigente;
- duas entradas opacas deliberadas, cujo único veredito correto é `Unknown`;
- execução em ordem normal e integralmente invertida;
- `Unknown` como falha em caso obrigatório ou execução desconhecida;
- 12 mutantes nominais discrimináveis e limiar obrigatório
  `mutation_score = 1.0`, sem sobreviventes;
- `repr(std)` explicitamente fora de execução, comparação, reparo e mutation
  killing deste passo.

Os spans são definidos como ranges UTF-8 zero-based e half-open. O stderr
short é byte-exato, inclusive o warning público emitido nos perfis HTML.

## Comandos, processos e custo

Comandos funcionais usados, sem ficheiro temporário:

```text
/usr/local/bin/typst --help
/usr/local/bin/typst eval --help
/usr/local/bin/typst --color never eval <profile-argv...> --diagnostic-format short <expression>
/usr/local/bin/typst --color never eval --diagnostic-format human 'repr(type(std.hsl))'
```

As matrizes foram conduzidas por três processos Python `subprocess.run`, que
somente lançaram o comando público acima e capturaram stdout/stderr em memória.
Custo observado desta autoria:

- `40` execuções semânticas do vanilla: `22` na passagem canônica final e
  `18` sondas/calibrações anteriores;
- `2` invocações de help do vanilla;
- `3` processos condutores Python;
- `1` revisão material do contrato de um budget de `2`;
- `0` execuções de mutantes e `0` corpus discriminatórios, pois pertencem aos
  papéis adversário e sealer;
- `23` casos obrigatórios, `2` opacos deliberados e `12` mutantes nominais
  declarados no output.

Validação local do output:

```text
python3 -m json.tool 00_nucleo/diagnosticos/p1301-contract.json >/dev/null
sha256sum 00_nucleo/diagnosticos/p1301-contract.json
```

Ambos passaram. Uma tentativa anterior com `jq empty` não executou porque
`jq` não está instalado; não houve alteração nem relaxamento do contrato.

## Limitação de isolamento

O processo correu no mesmo checkout e filesystem partilhado pelos restantes
papéis. A allowlist foi obedecida por disciplina de comandos, mas não houve
sandbox técnico que tornasse código/testes candidatos inacessíveis, nem
atestação do sistema operativo sobre o contexto herdado. Os hashes provam a
identidade e a ordem das entradas congeladas; não provam isolamento de
capacidade. Por isso este recibo não usa a expressão “segregado e atestado”.

Também não foi recalculado um snapshot geral da working tree nesta fase,
porque isso ampliaria a leitura além da allowlist do papel. A proveniência de
HEAD/working tree aqui é a do manifesto congelado; mudanças concorrentes no
checkout devem ser excluídas pelo rehash obrigatório das entradas antes do
gate discriminatório.

## Próximo gate causal

O autor de oráculos pode consumir apenas o manifesto, os L0s, este contrato e
o vanilla. O adversário e o sealer devem demonstrar, independentemente, que
todos os mutantes válidos recebem ao menos um witness `Violated`, que os
opacos ficam `Unknown`, que os requeridos ficam `Preserved` nas duas ordens e
que o score efetivamente medido é `1.0`. Até esse recibo existir, o contrato é
candidato, não selo nem certificado.
