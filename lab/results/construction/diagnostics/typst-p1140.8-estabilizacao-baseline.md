# Diagnóstico P1140.8 — estabilização do baseline pós-P1140.6

**Data:** 2026-08-24  
**Commit base:** `ca28f4ab74ae66985cdc66805c16c2ddc8f08366`  
**Estado medido:** working tree não commitado

## Resultado

O baseline funcional conhecido de L1 e L3 ficou verde. As duas falhas P862
tinham uma única causa: testes antigos exigiam uma segmentação
`Text/Space/Text` que contradizia o vanilla ratificado para um espaço ASCII
simples entre alfanuméricos. Não foi necessário alterar código de produção do
lexer ou eval.

O teste TLS de CA customizada passou integralmente num ambiente com loopback
permitido. No sandbox restrito, `PermissionDenied` ao criar o listener é agora
classificado de forma estreita e visível; falhas posteriores de TLS, cadeia,
hostname ou HTTP continuam a reprovar o teste.

## Medições de paridade

Executados em 2026-08-24 contra `/usr/local/bin/typst` (vanilla ratificado
`a51e02804`) e `./target/debug/typst` cristalino:

| expressão | vanilla | cristalino | resultado |
|---|---|---|---|
| `repr([hello world])` | `[hello world]` | `[hello world]` | igual |
| `repr([hello  world])` | `sequence([hello], [ ], [world])` | igual | igual |
| `repr([hello\nworld])` | `sequence([hello], [ ], [world])` | igual | igual |
| `repr([hello *world*])` | `sequence([hello], [ ], strong(body: [world]))` | igual | igual |
| `repr([hello#h(1pt)world])` | `sequence([hello], h(amount: 1pt), [world])` | `sequence([hello], hspace, [world])` | divergente |

As aspas externas impressas por `typst eval` indicam que o resultado de
`repr` é uma string; foram omitidas na tabela para legibilidade.

## Alterações

- `compiler/eval.md`: regra observável de whitespace e classificação
  língua versus mecânica;
- `stdlib/foundations/repr.md`: casos canônicos de um e dois espaços;
- `infra/package_downloader.md`: contrato estreito de capacidade ambiental;
- `compiler/eval/tests.rs`: substituição das duas expectativas P862 por testes
  de `repr` medidos;
- `package_downloader.rs`: criação do listener falível e tratamento somente de
  `PermissionDenied` como capacidade ausente;
- headers de linhagem ressellados por `crystalline-lint --fix-hashes .`.

## Validação

- testes focados P1140.8: **2 passados, 0 falhas**;
- `cargo test -p typst-core --lib`: **5144 passados, 0 falhas**;
- `cargo test -p typst-infra --lib`: **828 passados, 0 falhas**;
- sonda TLS fora do sandbox: **1 passada, 0 falhas**;
- `cargo build --workspace`: passou;
- `cargo fmt --all -- --check`: passou;
- `crystalline-lint .`: zero violações; permanecem avisos informativos já
  existentes;
- `git diff --check`: passou.

Às `2026-08-24T11:02:27-03:00`, antes da escrita deste diagnóstico e da marca
de fecho no passo, `git diff HEAD --stat` registava **15 ficheiros alterados,
81 inserções e 40 remoções**. O commit base continuava
`ca28f4ab74ae66985cdc66805c16c2ddc8f08366`.

## Achado separado

A quinta sonda encontrou divergência real em `repr(h(1pt))`: o vanilla
preserva `h(amount: 1pt)`, enquanto o cristalino produz `hspace` e perde o
argumento no `repr`. Ela não causava o baseline vermelho e não foi corrigida
neste passo para evitar ampliar o escopo sem auditoria L0 própria.

## Próxima frente

Antes da expansão de PDF tagueado, há uma correção pequena e bem delimitada a
considerar: medir e corrigir a representação de `h`/`v` spacing. Depois dela,
a frente maior continua sendo o inventário de papéis semânticos PDF e a escolha
de um primeiro lote (por exemplo headings e estrutura de documento), sem
reivindicar conformidade PDF/UA.
