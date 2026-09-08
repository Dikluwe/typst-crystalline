# P1311 — recibo do testador independente A/B

Executor: `/root/p1311_tests`. Regime A/B, executado sem atestação de
isolamento técnico. O filesystem é compartilhado; a segregação é de autoria
e entradas declaradas. Não houve leitura do patch candidato, diff candidato
ou testes do owner. Não houve escrita em código produtivo, L0 ou testes do
owner. As escritas deste executor limitam-se a `p1311-ab-*` em diagnósticos.

## Entradas e congelamento anterior ao candidato

- HEAD: `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, com P1310 não commitado.
  Diff/stat e status completos constam dos recibos JSON antes/depois.
- Vanilla ratificado upstream `a51e02804`, executável `/usr/local/bin/typst`,
  SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Baseline P1310 `/dev/shm/p1310-target.VeBP6Q/release/typst`, SHA-256
  `7f110b464745b880c357a9676fa302995873df5e67c5127adf0229cd25c5c146`.
- L0 integral lido; SHA-256 no freeze
  `5d19d634405c2a414a4cb7a1b133a202d99fd51b0478e7f96da4f0766f770758`.
  A proteção normativa exclui exatamente a linha de metadado `Hash do Código`
  conforme autorizado previamente pelo root. SHA-256 normativo:
  `c8f60e7a400ad1585008385d352f0f7b11d2e446918065e570868beaef0e1d51`.
- `p1311-ab-freeze.json`, SHA-256
  `37db1eee5ae27d597d11df9534574008b46a31fae56bbdd77b7135c772663b3a`,
  congelado em `2026-09-08T00:52:55.101638+00:00`.
- Revisão prépatch independente: `p1311-review-prepatch.md`, SHA-256
  `646632c9d5b8e0bb711297d73fd93db7012cfa269639ee1b35714fe9944dfda9`, GO.

## Medição e política de comparação

`p1311-ab-baseline-final.json`, SHA-256
`6428031aa5309044bcead62e4fbcb02eab881600303abef7fdd17748bb73c590`,
registra 440 execuções entre `2026-09-08T00:51:43.398005+00:00` e
`2026-09-08T00:52:05.399438+00:00`: 55 casos, quatro perfis, dois produtos.
Cada execução preserva argv, cwd, UTC, duração, exit e stdout/stderr literais.

São 24 casos alvo e 31 controles. Nos quatro perfis, todas as 96 observações
alvo divergem do vanilla antes do patch, constituindo RED. Para essas
observações, a expectativa congelada é a saída vanilla. Nos 124 controles,
a expectativa é a saída baseline, inclusive 68 observações já divergentes do
vanilla; as outras 56 já coincidem. Não se transforma dívida preexistente em
obrigação deste passo.

A revisão de classificação aconteceu antes do freeze e do candidato:
`assert.nope` é controle por namespace Some; `text.nope` e
`text.with(size: 9pt).nope` são alvos porque o binding real é Native None,
distinto da variante Element custom. Nenhuma expressão ou argv foi alterado
nessa classificação final; o freeze confere isso contra a medição preservada.

A matriz cobre leitura e callee, aliases, campo distinto, multiline, With e
With aninhado, nome qualificado `calc.abs` publicado como `abs`, armazenamento
em array/dicionário e `eval`. No erro originado em `eval("csv.encode")`, a
mensagem deve convergir, enquanto a propagação existente ancora a string
externa; não se inventa um field-only exterior. Os controles incluem
namespaces presentes, closures nomeadas/anônimas, Module, Dict, Type,
Content, float, gates PDF e sucesso/erros de loaders.

O comparador exige conjuntos exatos de chaves para ordens normal, repeat e
reverse, sem duplicatas, hashes dos arquivos protegidos e integridade
normativa do L0. Compara exit/stdout/stderr literais. Unknown ou falha de
instrumentação não é GREEN. A igualdade mecânica dos diagnósticos é usada
porque mensagem e âncora são o observável deste fragmento.

## Resultado candidato

PASS: 660/660 comparações, zero falhas e zero Unknown. São 288 observações
alvo coincidentes com o vanilla e 372 controles preservados em relação ao
baseline, distribuídos nas ordens normal, repeat e reverse. Cada ordem
contém as mesmas 220 chaves obrigatórias; nenhuma observação regrediu ou
mudou entre repetições. Os 96 RED distintos convergiram para GREEN nos
quatro perfis; não se contam repetições como casos semânticos novos.

Candidato `/dev/shm/p1311-target.JE8Cyy/release/typst`, SHA-256 conferido
`4bbced9ec793fb84daa560e0d965958b33e1eb2f5ecd4bfd3bc30b5900bb09fb`.
Execuções entre `2026-09-08T01:04:11.885154+00:00` e
`2026-09-08T01:05:46.707756+00:00`, com HEAD e diff/stat antes/depois no
recibo bruto `p1311-ab-candidate-runs.json`, SHA-256
`2ed38bccdbe988753eed68df2e008e8c67c44b09a6cbf7340059cef995f8a86a`.
Comparação `p1311-ab-comparison.json`, SHA-256
`0dd5ebf8ab5f18e9975ddf33c379a54f56a49cfd6e9cd1af6244262dc30fc4ac`,
produzida em `2026-09-08T01:05:57.677957+00:00`.

Todos os hashes protegidos e o L0 normativo foram reconferidos pelo
comparador congelado. O metadado `Hash do Código` recebeu o resselo autorizado;
as expectativas, casos e scripts congelados não mudaram. Comandos:

```text
python3 -B 00_nucleo/diagnosticos/p1311-ab-runner.py --candidate /dev/shm/p1311-target.JE8Cyy/release/typst --orders normal,repeat,reverse --output 00_nucleo/diagnosticos/p1311-ab-candidate-runs.json
python3 -B 00_nucleo/diagnosticos/p1311-ab-freeze.py --measurement 00_nucleo/diagnosticos/p1311-ab-freeze.json --candidate-runs 00_nucleo/diagnosticos/p1311-ab-candidate-runs.json --output 00_nucleo/diagnosticos/p1311-ab-comparison.json
```

O acesso ao executável em `/dev/shm` usou execução host autorizada; o sandbox
padrão deste ambiente enxerga outra instância desse diretório.

## Limites

Element custom arbitrário, native com namespace Some vazio e plugin não são
construídos por este corpus CLI; sua cobertura fica nos testes do owner,
sem alegação de independência A/B para essas construções. O recibo não atesta
isolamento de capacidades, refinamento, mutação de produto ou paridade geral
do compilador. Nenhum caso nem expectativa será adaptado ao candidato.
