# P1353 — relatório de sanitização produtiva pós-P1339

## Veredito

**PASS.** A telemetria exclusiva de `cfg(p1339_observation)` foi retirada sem
alteração observável na matriz canônica. A estabilização contextual produtiva,
os contratos públicos e as dez rotas fechadas por P1339 foram preservados.

O dono autorizou expressamente a continuação depois do diff L0. P1353 não
usou a skill de materialização segregada, conforme a delimitação do próprio
passo: esta foi uma remoção estreita de instrumentação histórica, validada por
equivalência A/B, e não uma nova alegação semântica.

## Proveniência

- execução final: `2026-09-12`, árvore de origem
  `0b822176943f36f69b1a1d0576ea572c09de9530`;
- commit efetivo da cópia isolada:
  `ffe89b21011a467b442553b32c0be4eb7990ef39`;
- a execução começou no commit
  `e77fbd27308d5dc0eb4325e67b3f3f009f56ec2d`;
- uma reescrita Git externa mudou os IDs de commit durante o passo, mas a
  árvore permaneceu exatamente `0b822176943f36f69b1a1d0576ea572c09de9530`;
- para impedir que novos resets apagassem trabalho não commitado, a mudança e
  a validação foram concluídas na cópia isolada
  `/tmp/p1353-work.vbNjDg/repo`;
- temporário da matriz: `/dev/shm/p1353.hfM3Wo`;
- catálogo: 4.718 probes, SHA-256
  `a9853e4c54663a258a9dd706c1c974930579c65e335b283243b10eae19b4889a`;
- vanilla ratificado: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Os recibos compactos conservam o estado exato usado em cada medição, hashes
dos binários, horários UTC, perfis, lista integral dos paths divergentes e a
definição da serialização canônica.

## Matriz A/B

| Medida | Baseline A | Final B |
|---|---:|---:|
| células | 18.872 | 18.872 |
| `MATCH_VALUE` | 18.000 | 18.000 |
| `MATCH_DIAGNOSTIC` | 260 | 260 |
| `VANILLA_ONLY` | 284 | 284 |
| `CRYSTALLINE_ONLY` | 168 | 168 |
| `DIFFERENT_VALUE` | 40 | 40 |
| `DIFFERENT_DIAGNOSTIC` | 120 | 120 |
| Unknown | 0 | 0 |
| paths iguais nos quatro perfis | 4.543 | 4.543 |
| paths divergentes | 175 | 175 |
| SHA-256 canônico | `2479c9a248bb9d25832dfdf8cbed6a058dddf270af3a3af5a82a2392ccdff3aa` | `2479c9a248bb9d25832dfdf8cbed6a058dddf270af3a3af5a82a2392ccdff3aa` |

As dez rotas P1339 (`angle.deg`, `angle.rad`, `float.from-bytes`,
`float.inf`, `float.nan`, `float.signum`, `float.to-bytes`,
`function.where`, `function.with`, `version.at`) continuaram
`MATCH_VALUE` nos perfis `default`, `html`, `a11y` e `html+a11y`.

Binários medidos:

- baseline A: SHA-256
  `77a788c4b57ed882c8362484e633d3f756f95d64d54f77817d4a2df9948ac735`;
- final B: SHA-256
  `df94894ca6d19ffb1e0cef8fdb8bd10276812e7e7f543543218ba9445060a24c`.

O binário mudou porque a instrumentação condicional e seus símbolos foram
retirados; a representação observável das 18.872 células não mudou.

## Inventário da remoção

| Consumer | Removido | Preservado | Motivo |
|---|---|---|---|
| `compiler/eval/mod.rs` | harness condicional, DTOs, projeções e hooks de observação | `ContextRead`, `ContextReads`, `ObservationRelation`, replay e os três métodos contratuais | a telemetria não participava do caminho normal |
| `compiler/introspect/from_tags.rs` | callback de telemetria | aplicação real de `CounterUpdate::Func` | preservar semântica de contador |
| `stdlib/primitives_constructors/array.rs` | include externo e teste exclusivo da observação | `native_array_bytes` e testes locais | fixture histórica não é produto |
| `stdlib/state.rs` | hook condicional de callback | leitura observada e aplicação real do callback | preservar estado contextual |
| `entities/style_chain.rs` | método de identidade usado apenas pela observação | estrutura e APIs produtivas | evitar API substituta sem consumidor |
| `infra/pipeline.rs` | eventos `post_eval`, `compilation_returned`, exporter e include histórico | estágios normais do pipeline | não mudar fase nem default |
| `infra/pipeline/context_stabilization.rs` | módulo e hooks da observação | discovery, seleção, invalidação, replay, cinco tentativas, diagnóstico terminal, layout e paginação | preservar integralmente P1339 produtivo |

Os testes produtivos de replay, relação IEEE/recursiva e ordem de registro da
leitura contextual foram mantidos, apenas sem nomenclatura de telemetria.

## L0 e linhagem

Foram atualizados primeiro os sete prompts proprietários. Cada um agora declara
que a telemetria P1339 é histórica, proíbe callback/evento substituto e mantém
o contrato produtivo. O resselo foi feito exclusivamente por
`crystalline-lint --fix-hashes .`.

Hashes resultantes (`@prompt-hash` / hash do código):

| Prompt | L0 | Código |
|---|---|---|
| `compiler/eval.md` | `ab3e7ac3` | `e1b27745` |
| `compiler/introspect/from_tags.md` | `1bc222be` | `8db50518` |
| `compiler/stdlib/primitives-constructors/array.md` | `7062ef75` | `6508a7d5` |
| `compiler/stdlib/state.md` | `8eada901` | `cbb13196` |
| `entities/style_chain.md` | `1e0277a9` | `03ed7777` |
| `infra/pipeline/context_stabilization.md` | `fab9c64a` | `49eccf73` |
| `infra/pipeline.md` | `49c71764` | `2d97726a` |

Antes do resselo havia somente os sete V5 esperados; não havia V15 nem V26.
Depois do resselo, o linter informou zero drift warnings.

## Formatação e escopo

`cargo fmt --all` também normalizou dez arquivos que já estavam fora do
formato canônico. A comparação contra uma cópia limpa formatada do baseline
provou esses dez diffs byte a byte como `RUSTFMT_ONLY`. Nos sete consumers do
P1353, a única outra classe foi `P1339_OBSERVATION_REMOVAL`. Não houve terceira
classe semântica.

## Gates finais

- `cargo fmt --all -- --check`: exit 0;
- `cargo build --workspace --release --locked`: exit 0;
- busca de `p1339_observation` em L1–L4: zero ocorrências, exit 0;
- `crystalline-lint .`: exit 0, sem V5/V15/V26;
- `git diff --check`: exit 0;
- matriz B e comparação A/B: PASS;
- testes workspace release locked: PASS por partição ambiental reproduzível.

A invocação inicial dos testes em `/dev/shm` esgotou o espaço compartilhado em
dois contratos rustdoc; o cache foi movido para disco e recompilado. Em disco,
três testes de `font-path` ficaram lentos porque usam `env::temp_dir()` e
acabaram varrendo recursivamente todo `/tmp`, inclusive o cache. A validação
final foi particionada sem alterar código:

1. suíte integral, exceto esses três testes, com `TMPDIR` padrão: exit 0;
2. contrato P1286 com `TMPDIR` padrão, necessário para sua normalização de
   paths: exit 0;
3. os três testes de `font-path` com `TMPDIR` dedicado e vazio: 3/3, exit 0.

Uma tentativa intermediária de executar toda a suíte com o `TMPDIR` dedicado
fez o P1286 expor paths aleatórios fora do regex histórico de normalização e,
corretamente, retornar Unknown; o mesmo contrato passou no ambiente previsto.
Isso não é regressão do P1353.

O build final registrou 81 warnings preexistentes em `typst-core` e 42 em
`typst-infra`. Não houve warning de `p1339_observation`; P1353 não ocultou nem
corrigiu warnings fora do escopo.

## Conclusão

A instrumentação histórica foi removida e não foi substituída por outra
superfície. O contrato produtivo de P1339 permaneceu intacto. Não há crédito
novo de paridade, mudança de API, default, feature ou fase do pipeline.
