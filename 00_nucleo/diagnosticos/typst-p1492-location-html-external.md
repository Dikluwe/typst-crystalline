# P1492 — localização HTML fora de `html.frame`

## Resultado

P1492 foi concluído em fluxo contínuo conforme ADR-0127. No target semântico
HTML externo, `location.page()` e `location.position()` agora devolvem
`none`, como no vanilla pinado. O dispatch usa `EvalContext::target`, não
a feature ativa, e conserva o registro de `ContextReadRequest`.
`page-numbering()`, os targets `Paged`/`Bundle` e os defaults paginados
não mudaram.

A fixture completa `P1489-I-001` continua deliberadamente `VIOLATED`
apenas porque `position()` dentro de `html.frame` ainda devolve `none`.
Esse recorte permanece reservado ao P1493.

## Medição anterior à decisão

Medição em 2026-09-24T14:47:52-03:00, no HEAD
`7f814615b786fddfceca30fb9fa11825c3b42eba`, antes do código P1492. O
vanilla foi o binário ratificado do pin
`9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

A sonda externa independente falhou no cristalino com `page() == 1` e passou
no vanilla. No release anterior, os perfis produziram `VIOLATED` e ordem
bilateral idêntica:

| Perfil | digest forward/reverse |
|---|---|
| `html` | `a749777285bf17e60a2ce18fc37d60c8dc09e8f689af7a7348022b2f28bd07ec` |
| `html+a11y` | `7873322997880a3a956d9d2ad6e5177675d64006f1c97a795f8cbb7a552b79bb` |
| `html+bundle` | `0982ff5f08b217a3da86e37eb9327b18dc66a760240fc99aeabb2f60e2714bae` |

A fonte upstream `introspection/location.rs:87-132` confirma `none` no
HTML externo e coordenadas sem campo `page` dentro da ilha. Isso refutou a
projeção cristalina anterior de página 1 e coordenadas 0pt.

## RED → GREEN e implementação

O teste focal começou com dois recortes: HTML externo e preservação dos
targets `Paged`/`Bundle`. No RED, o primeiro falhou em `page()`
(`1` contra `none`) e o controle paginado passou: 1/2. Após a mudança,
2/2 passaram.

`eval_location_method` retorna `Value::None` somente quando
`ctx.target == EvalTarget::Html` e o método é `page` ou `position`.
Os demais ramos continuam consultando o introspector e preservam os defaults.
Nenhuma entidade, trait, assinatura pública ou fase de pipeline mudou.

## Matriz bilateral e controles

Com o release cristalino SHA-256
`dc6a07e60e25e736b62ed1fce6c095498c332bd132e4999ab31ddc21fd389e63`,
`P1492-I-001` ficou `MATCH=1`, `VIOLATED=0`, expectativa satisfeita e
ordem bilateral idêntica nos três perfis:

| Perfil | digest forward/reverse |
|---|---|
| `html` | `5a808d28b0190618cfb2336fb24454d6f4067c6f55d90a285582fab2e625473b` |
| `html+a11y` | `e5212f038100b6098e5bb9e0fc854b8c71781d8c4373b86d301b1f2a985e01bf` |
| `html+bundle` | `fdb563c7afcaf124847af6cebfdcf9b13a7188dfde3f778e559fec56ff13eda7` |

Controles:

- `P1489-I-001` permaneceu `VIOLATED` somente na coordenada de
  `html.frame`, digest bilateral
  `e2d5cccc77b2eae70685ce827480f6479c16eb64bd1ed7c2b6eb2f0977a2bebc`;
- `P1487-X-002` (bundle misto) permaneceu `MATCH`, digest
  `c64a19364c30f48d101105dd9b27cd91878dcf3afa241d829cd349dd764a1f3e`;
- o teste unitário preserva página 1 e posição default nos targets
  `Paged` e `Bundle`.

## Linhagem ressellada

| Owner | `@prompt-hash` | `Hash do Código` |
|---|---|---|
| `compiler/eval/call_dispatch` | `865bf9da` | `65529090` |

O dry-run identificou somente esse owner. O resselo terminou com zero drift.

## Verificação final e proveniência

Verificação final em 2026-09-24T15:06:20-03:00, no HEAD
`7f814615b786fddfceca30fb9fa11825c3b42eba` e working tree P1492 não
commitado.

| Produto medido | SHA-256 |
|---|---|
| vanilla release pinado | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |
| cristalino debug | `378409e22194db96a7e4d802d2ea5073f04b3b70f1966308a1cff72da14f5611` |
| cristalino release | `dc6a07e60e25e736b62ed1fce6c095498c332bd132e4999ab31ddc21fd389e63` |

Gates finais:

- teste focal P1492: 2/2 passaram;
- `cargo test -p typst-core`: 6146 unitários, 8 integrações e 3 doctests passaram;
- `CARGO_BUILD_JOBS=1 cargo build`: passou;
- `CARGO_BUILD_JOBS=1 cargo build --release`: passou;
- `python3 -m unittest lab/parity/matrix/test_runner.py`: 56/56 passaram;
- `crystalline-lint .`: código 0;
- `git diff --check`: código 0.

## Estado exato do working tree medido

O estado final é não commitado sobre
`7f814615b786fddfceca30fb9fa11825c3b42eba` e contém:

- `00_nucleo/materialization/typst-passo-1492.md`;
- `00_nucleo/prompts/compiler/eval/call_dispatch.md`;
- `01_core/src/compiler/eval/call_dispatch.rs`;
- `lab/parity/matrix/manifest.yaml`;
- `lab/parity/matrix/fixtures/location-html-external.typ` (novo);
- `00_nucleo/diagnosticos/typst-p1492-location-html-external.md` (novo).

## Limite explícito

P1492 não implementa a `Page` intrínseca de `html.frame`, não expõe
`page` no dicionário de posição HTML e não usa `Position` paginada para
representar ausência. Esses limites evitam antecipar o desenho do P1493.
