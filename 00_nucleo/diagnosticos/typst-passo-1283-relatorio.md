# Relatório do Passo 1283 — tabelas de símbolos e variantes

**Estado:** fechado em 2026-08-30.  As contagens abaixo são inventários de paths,
não uma percentagem global de paridade.

## Estado medido e fontes

A medição final foi feita no commit `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`,
com working tree não commitado. A lista exata de alterações e os hashes de cada input
estão congelados em `p1283-summary.json` (`provenance.git_status_short`,
`provenance.git_diff_head_stat` e `provenance.inputs`). O produto cristalino usado no
inventário tem SHA-256
`a6e03f6c3da68e689c3baa1d06d1fd8f6665b9294786e4add145eb13b4f47917`; o vanilla
ratificado `/usr/local/bin/typst` tem SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` e revisão
`a51e02804`.

A fonte declarativa é `codex = 0.3.0`, pinada exatamente, checksum Cargo
`0732ab1a27b4ea05e6f9f60a5122c9924dd5123defde0d8e907f58cf643d40e6`:

- `sym.txt:1-1340`, SHA-256 `6ee467d9939acb5c7d0a3eba30c9f640d157529cbf57df752367deb343e0fc16`;
- `emoji.txt:1-1472`, SHA-256 `8691ca68e09b6fedca61e00e824648e79f7502772eefe7af367e404e26161489`.

## L0, ownership e decisão

O contrato final `C-P1283-v4` congelou ownership 1:1:

- `compiler/stdlib/sym.md` ↔ `01_core/src/compiler/stdlib/sym.rs`;
- `compiler/stdlib/emoji.md` ↔ `01_core/src/compiler/stdlib/emoji.rs`;
- `compiler/stdlib/structural/math.md` ↔ `01_core/src/compiler/stdlib/structural/math.rs`.

Os L0 finais têm SHA-256 `42a21272…c816f3d`, `d5fda4cf…e3291f` e
`4441fe00…617bf`, respectivamente. Nenhum deles consome Núcleo Tekt; V15/V26 passaram
antes do resselo. `crystalline-lint --fix-hashes .` ressellou `sym` e `emoji`, e uma
segunda execução não encontrou nada a corrigir.

A implementação usa um adaptador puro e recursivo sobre `codex::SYM` e
`codex::EMOJI`. Sequências Unicode são transportadas integralmente; modifiers e sua
ordem de declaração são preservados; o default de pais sem bare usa o best-match do
codex. `math` copia recursivamente o scope de `sym` somente depois de registrar seus
bindings próprios e nunca sobrescreve `sqrt`, `class`, `equation` ou `op`.

Não houve mudança de contrato público Rust, default do produto, fase de pipeline nem
quebra de compatibilidade. Foi portanto aplicado o fluxo contínuo de correção interna
de tabela do ADR-0127: L0 → hash → RED → GREEN.

## RED → GREEN

O RED focal (`cargo test -p typst-core p1283_ --offline -- --nocapture`) falhou em
5/5 testes: `sym` tinha 72 bindings em vez dos 300 top-level esperados, faltavam os
submódulos `gender`/`control`, e `emoji` tinha 531 bindings em vez de 772. Após o
adaptador, os mesmos 5/5 testes passaram.

O primeiro gate completo revelou cinco expectativas históricas incompatíveis com o
vanilla pinado: VS15 em `arrow.r.filled` e `suit.heart`, `diamond.small = ⋄` e a
variante `sq` de `subset.neq`. As expectativas foram corrigidas a partir do catálogo
bilateral; a segunda execução completa passou.

## Inventário P1282 regenerado

Os perfis `default` e `html` deram os mesmos resultados nas três famílias:

| Família | Antes | Depois |
|---|---|---|
| `sym` | 34 MATCH; 228 missing; 38 metadata; 37 bloqueados; 1 extra | 337 MATCH; 0 missing; 0 metadata; 0 bloqueados; 1 extra |
| `emoji` | 464 MATCH; 241 missing; 68 metadata | 773 MATCH; 0 missing; 0 metadata; 0 bloqueados; 0 extras |
| `math` | 83 MATCH; 271 missing; 42 metadata; 37 bloqueados; 1 extra | 386 MATCH; 43 missing; 4 metadata; 0 bloqueados; 1 extra |

Em `sym`, os 337 MATCH são o módulo, dois submódulos e 334 símbolos. Em `emoji`, são
o módulo e 772 símbolos. O subgrafo declarativo espelhado em `math` está integralmente
em MATCH. Os 43 missing restantes de `math` são funções não declarativas fora de
`C-P1283-v4`; os quatro metadata são as funções próprias já presentes cuja assinatura
o enumerador não observa bilateralmente. Não recebem crédito de fechamento neste
passo e também não foram materializadas por extrapolação.

`sym.registered` e `math.registered` são os dois extras esperados: extensões
cristalinas preservadas, sem crédito vanilla e sem remoção automática. `sym.sqrt`
permanece bilateralmente ausente.

As sondas focais bilaterais passaram 33/33 em `default` e 33/33 em `html`. Elas cobrem
kinds, submódulos, pais sem bare, aliases, ordem de modifiers, VS15, VS16, ZWJ,
espelho math e proteção das quatro funções próprias. O probe de `sym.join` também
registrou em ambos os binários exatamente um warning
`` `join` is deprecated, use `bowtie.big` instead `` e sucesso.

## Ataques e divergências

O conjunto adversarial independente `A-P1283-v4` foi congelado antes da verificação.
O contrato está materializado em `p1283-contract-receipt.md`, SHA-256
`80eb980b8f22e23f3c9f56e2526f28a8591bde5eae3849b44348c60af239dc3f`; o conjunto
adversarial pina esse recibo e está em `p1283-adversarial-receipt.md`, SHA-256
`47da00869521924acfb99d6ec2ee133f3147fe113e74fc70e7ae1cfbb42016d5`.
Os oráculos exaustivos de módulo, inventário bilateral e sondas discriminantes rejeitam
as 16 mutações: ausência, achatamento de submódulo, kind, base, VS15, VS16, ZWJ,
remoção/rename/value de variante, ordem de modifiers, parent sem bare, alias, espelho
math, overwrite math, warning de `join` e remoção/crédito de `registered`. Resultado:
**16/16**, controles fora do denominador; `Unknown` nunca contou como sucesso.

Há uma divergência intencional individualizada: 13 depreciações anexadas a variantes
de `gt.tri*`, `lt.tri*` e `tack*` não emitem warning próprio porque `SymbolVariant` não
transporta mensagem. As variantes continuam presentes e idênticas; isto não conta
como paridade diagnóstica. Alterar esse transporte seria mudança de contrato público e
fica fora do Passo 1283 pelo ADR-0127.

A segregação foi procedimental: contrato, testes, ataques e verificação tiveram papéis
independentes, mas o filesystem compartilhado não oferece atestação técnica de
isolamento por allowlist.

## Gates finais

- `cargo test --workspace --offline --quiet`: verde — 5.305 + 909 + 1 + 56 + 2 + 70 + 2 testes passaram; 3 ignorados; zero falhas.
- `cargo build --workspace --bin typst --offline`: verde.
- `cargo build --workspace --bin typst --release --offline`: verde para o produto medido.
- `crystalline-lint . --quiet`: exit 0, zero violações.
- `cargo fmt --all` e `git diff --check`: verdes.

## Entrega ao Passo 1284

`p1283-residual-p1284.json` exclui `math`, `sym` e `emoji`, como determinado pelo
passo. No perfil default há 43 extras, 139 missing, 256 metadata e 21 bloqueados;
no perfil HTML há 43 extras, 190 missing, 319 metadata e 21 bloqueados. O ficheiro
mantém cada path e seus recibos bilaterais para o foco seguinte em módulos, tipos e
membros estáticos, sem converter as contagens em percentagem.

## Artefatos e hashes

| Artefato | SHA-256 |
|---|---|
| inventário default | `cd4318340628cdd724d4f372cadfee1f92592baf7d588d883a312764c609901d` |
| inventário HTML | `88935ac6090485b2e167d8bae9f7fd8c475936f2605f9fb5623ddbfb3e5aa6db` |
| probes default | `17074b88a523174dfc5636b43a6ff642e5f173c460a5762ae9ce7fea238a5099` |
| probes HTML | `b59172c8baa89bf101c8cfb6354d8d1f30c1194c74f15d4a3feeca13ce176b9e` |
| contrato C-P1283-v4 | `80eb980b8f22e23f3c9f56e2526f28a8591bde5eae3849b44348c60af239dc3f` |
| ataques A-P1283-v4 | `47da00869521924acfb99d6ec2ee133f3147fe113e74fc70e7ae1cfbb42016d5` |
| resumo | `3b7a9ba3c1e3e95efdf5f91fd92d50a02755646b7a354bc26abcc8e5081893c8` |
| residual P1284 | `42d99bb0bed744d8266cfee185b759eba781230e7ff8ac5a2dde64153fde7c97` |
