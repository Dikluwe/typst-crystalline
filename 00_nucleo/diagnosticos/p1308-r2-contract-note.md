# P1308-R2 — sucessor independente de transcripts

Congelado após autorização do dono: “Faça o commite do que falta e autorizo”.
Regime da skill: executado sem atestação de isolamento técnico. Autor
`/root/p1307_contract`; nenhuma leitura de fonte produtiva candidata, edição
Rust/L0, commit ou veredito por este papel. Produto foi observado apenas pelo
binário explicitamente autorizado. Root integra o patch e revalida.

## Causalidade e medida anterior às expectativas

O recibo independente `p1308-verification-final.json`, SHA-256
`06a57344f4e8039c1cd96a1df1aa2b2b2c159797e8080f1841844541e012ac60`,
identificou 19 casos/76 células com somente um trace anexado. O dono autorizou
essa migração; os L0s foram escritos antes dos sucessores:

- eval/tests.md: `575f9d220dfc5355db14d040dba64104d2cd0ba489c513ad81f7544ee3ee78cd`;
- wiring/tests/p1293_contract.md: `480c39a74a6c0974c79da93dfc2db0e3db9aa4833d6714884d3d2940ea240e7b`.

`p1308-r2-measure.json`, SHA-256
`b507c4a71345012a2e6b2ccd7ad3c7c89f130f1d66c32c853e5d0d28d4637079`,
conserva horários, HEAD, status/diffstat, prepin do teste, fixtures, argv e
saídas integrais. Binário observado:
`/dev/shm/p1307-r4-target.8W1BEA/release/typst`, SHA-256
`09725fff8b4c472ee6b50a9ca6105d90268b2a0da8a22f1d8abd0f45e20c4c50`.

Foram 26 casos × 4 perfis × 2 ordens = 208 processos: 208 Preserved,
0 Violated, 0 Unknown. São os 19 R6, os seis erros cell/header/footer de
grid/table e o controle positivo serializer do mesmo teste. Nenhum assert
ficou esperando uma falha posterior para ser medido.

O expected R6 foi construído **antes de cada execução** pela soma do stderr
protegido com o suffix exato do recibo independente, nunca copiando a saída
observada. Para P1293, preservou-se a mensagem portuguesa e acrescentou-se
o trace da chamada interna (coluna 5), não do repr externo. O sucesso foi
comparado com a string integral preexistente. Qualquer outro delta seria
Violated e bloquearia a migração; nenhum foi observado.

## Saídas congeladas

| Artefato | SHA-256 |
|---|---|
| `p1308-r2-oracle.json` | `7d076a223c97c33768955a22fc43dd5bd8d5d29be75dc4b57346f86de8504312` |
| `p1308-r2-oracle.py` | `05fe94ccee1cd4df811a2fa2d6a5f6b4338f4ad80ed597864b64a2579cc3be7c` |
| `p1308-r2-tests.patch` | `c69ab727396cb3352aa23a79aa4be905044c00c67dc5478175151b6adbfe2552` |
| `p1308-r2-measure.py` | `18d181efe022718c512eccb2cf6a9a9c672a1ebf45b96ce1baaefeb1d73792cc` |
| `p1308-r2-contract-freeze.py` | `a295d1b869b06232fe1678cf897a78ffbf1cb0e52dda650b69269480f755dcd5` |

O oráculo sucessor mantém todos os casos, fixtures e células R6, alterando
somente `future_expected.stderr` nos 19 IDs enumerados no recibo. Preserva
mensagens, hints, exit, stdout, primary ranges e metadados históricos das
observações; metadata superior registra a exceção trace-only autorizada.
Isto não declara paridade vanilla das dívidas primárias.

O runner oferece `replay --binary PATH [--case REGEX] [--profile PERFIL]
[--reverse]`. Importa diretamente `run`, envelopes e `classify` R4 pinados,
como fazia R6; não acrescenta normalização, tolerância, filtro de traces ou
nova regra de Unknown. Os arquivos R6 e todas as provas anteriores permanecem
intactos. Registros Violated históricos não são reescritos.

O patch muda somente as seis strings de stderr dentro de
`p1293_d_preexisting_arity_and_serializer_transcript_is_frozen`, preservando
expressões, loop, comparador, ordem, sucesso e registry. `git apply --check`
passou com exit 0 antes da entrega. Nenhuma expectativa dos 18 testes P1308
foi modificada. A matriz global, a repetição inversa e o veredito final são
gates posteriores dos outros papéis, não resultados reivindicados aqui.
