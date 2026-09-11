# P1339 — correção da preparação do teste local de bytes

Implementador `/root/p1339_remaining_l0`, executado sem atestação de
isolamento. Recorte autorizado pelo root: investigar a falha local integrada,
corrigir preparação legítima sem alterar oráculos/L0 e repetir testes numéricos.

## Evidência anterior à decisão

`p1339-implementation-integrated-local-r1.json`, SHA-256
`c1761190509b2fbaeecf33f39ce3a02bc34c4b984874004e7e290690a72efdc9`,
registrou 16 passes e uma falha em `float.rs:445` com
`type array does not have a constructor`. Estado e diff/stat completos estão
no recibo; HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, UTC
`2026-09-10T06:10:10.032924+00:00` a `06:11:37.982003+00:00`.

O teste local construído pelo implementador envolvia o retorno de to-bytes
com `array(...)` em `float.rs:438–439`. O acesso normal de expressão em
`compiler/eval/mod.rs:6161–6180` instala a stdlib normal; não é um efeito
especial de NullWorld. O match de constructors em `call_dispatch.rs:1791–1818`
não contém Type::Array e chega ao diagnóstico citado. A dependência em array
era auxiliar ao teste da API bytes e não consta como materialização autorizada
no L0 float selado.

## Alteração restrita ao teste local

Somente o corpo cfg(test) de
`p1339_float_public_numeric_values_and_bytes` em float.rs mudou. As chamadas
agora avaliam diretamente `float.to-bytes(...)` e exigem `Value::Bytes` com os
mesmos bytes exatos: `[0,0,0,0,0,0,248,63]` para 1.5 default e
`[128,0,0,0]` para -0.0 em binary32 big-endian. A comparação também rejeita
retorno Array; não enfraquece tipo, quantidade, endian ou sinal.

Nenhuma semântica produtiva, constructor Array, outro owner, L0 ou oráculo
externo foi alterado. Os oráculos externos também usam expressões com array;
isso foi avisado ao root e permanece uma questão separada do gate público.
Esta correção local não reclassifica os resultados desses oráculos nem
autoriza modificar suas fixtures depois do selo.

Source float antes:
`14f21c8c42a928d8c6d445034e111e604a1dea1fd55272ae81e9481a0b98c15d`;
depois:
`846c75540e439905ea033fae235c22ab030725971adb00f7f51e261e2fdd8bc6`.
Os demais sources desta autoridade permaneceram intactos durante a revisão.

## Execuções locais

Os recibos abaixo têm HEAD, status/diff-stat completos, UTC, comandos, canais,
hashes dos sources e custo. Seus quatro sources acompanhados ficaram
inalterados durante cada execução. A árvore integrada continua não commitada.

| Recibo | Resultado | SHA-256 |
|---|---|---|
| `p1339-implementation-numerics-local-green-float-r2.json` | 2 testes, 0 falhas, cargo release recompilado | `1e8e03baf5e9d8a17fba3d0e0000a806ea9e7c2a91576b11b50326a50cef6e04` |
| `p1339-implementation-numerics-local-green-version-r2.json` | 24 testes do owner version, 0 falhas | `cbce93d2154248dc30d4c85a512ee45567e98496eac113ed945c468e87eb7047` |
| `p1339-implementation-numerics-local-green-float-preservation-r2.json` | 11 testes float, incluindo os 2 P1339 e predicados anteriores, 0 falhas | `84dc932907692f9bf17f20be6e8f8445762a28a00cd323c96be07960bc868335` |

Build UTC `2026-09-10T06:23:04.465458+00:00` a
`06:24:30.385422+00:00`, custo 85.9244 s. Os dois recortes seguintes rodaram
o mesmo testbin diretamente em `06:25:22Z`, sem novo build:
`/tmp/p1339-target.UD8gh7/release/deps/typst_core-34577d6f7c6d8e68`,
SHA-256 aferido depois das execuções
`a964ebe4a03a1f83007aa76711f56c40f1fecea57c97a9634c2e1b359709e874`.
Os 2 testes repetidos não são somados como cobertura nova aos 11.
Rustfmt do arquivo e git diff --check local passaram.

Não é execução A/B independente nem veredito final. Os gates públicos,
arquiteturais, mutações e preservação global do P1339 permanecem sob suas
autoridades próprias. Sem resselo ou commit.
