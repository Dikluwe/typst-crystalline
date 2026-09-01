# P1289 v3 — recibo de reverificação final A/B

**Estado:** `FINAL_VERDICT_WITHHELD_V3_DELTA_ISOLATED_SNAPSHOT_UNSTABLE`  
**Veredito:** fragmento C-P1289 v3 preservado e delta `+1 MATCH` atestado
por controle contrafactual; fecho global ainda não atestável na árvore
compartilhada  
**Verificador:** `/root/verificador_p1289`  
**Janela v3:** `2026-08-31T11:54:12-03:00`–`2026-08-31T12:20:20-03:00`  
**Regime:** Tekt A/B segregado por papel, capacidades, ordem e hashes, sem
isolamento forte do checkout.

Esta secção v3 substitui o veredito v2 abaixo, que permanece somente como
histórico da primeira verificação. O papel verificador não corrigiu artefatos;
a única escrita persistente continua sendo este recibo.

## V3.1 Integridade congelada

Todos os pins recebidos foram recalculados e coincidiram:

| Artefato v3 | SHA-256 |
|---|---|
| manifesto | `1bd7c90a45f9cc2a2301eaab89405915d5dfc7d74627508325ca0124c50167ca` |
| runner A | `e7a3d802c720ef2537ab2af8d6d634b81ffb474e7961a65bd2f05e83dc211390` |
| baseline A | `7c8337761109db2ffa30fcb112c97952caa632589eaecbf01757092bd8cfc44d` |
| teste protegido | `56c5093227465623fc9b27dd9dd1a4ee54ed6e7c0f78e0b41c0ab061fcce68ea` |
| novo L0 do teste | `55694b8c40b3366d1e5c2d850b0033091b39a11eab1fb70403d1e1dc484d0899` |
| candidato `/tmp/p1289-target-b-v2/debug/typst` | `f18a267eb83e785ca0c7c1713b2a613b54d331e4b60cd995d5d229198d07940d` |

Os cinco consumers produtivos mantiveram exatamente os hashes v2:

| Consumer | SHA-256 |
|---|---|
| `foundations/float.rs` | `fcf93a0bfc26a73d2b150d05089a0ff2ae5f2a55fbeb587ac402f196c7aad130` |
| `foundations/mod.rs` | `f94a2d4523e06893bda780c9fb400736e2686499cac176d747e4df48509d08c0` |
| `stdlib/mod.rs` | `c8d2ef3b6deec9ef979cfc48620b6519b83925388d912bbab0dc7a567aa15792` |
| `field_access.rs` | `979ed86569f6532815df3de25399efccba9585ef8daeb63f079e15132c732882` |
| `call_dispatch.rs` | `c249305d329cb42d73966ac53f9d26b765b30ed42d972310b9a3c13f19d366dd` |

O baseline vanilla e o pré-candidato continuaram, respectivamente,
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`
e `e7c81e3a6c6f1933db95c2249286fca19f3b5991eb8bc392f49ea659cec0bd78`.

Proveniência do snapshot v3: HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, working tree não commitida;
`git status --short` com 293 linhas e SHA-256
`a3c72f283c357708ddac76fc5606ecc9fd47eeb6a70046513273cb570009bdec`;
`git diff HEAD --stat` com 138 ficheiros, 649022 inserções e 1912 remoções,
SHA-256
`5c7a98512bd4d8e8fa5b5b6ffb8984e9549c35936caa1326d42c7ff4a54def28`.

## V3.2 RED, GREEN e mutações

Foram reexecutados:

```text
python3 lab/surface-inventory/run_p1289_oracles.py --candidate /tmp/p1289-target-pre/debug/typst --summary-only --pretty
python3 lab/surface-inventory/run_p1289_oracles.py --candidate /tmp/p1289-target-b-v2/debug/typst --summary-only --pretty
python3 lab/surface-inventory/run_p1289_oracles.py --self-test --pretty
CARGO_TARGET_DIR=/tmp/p1289-target-b-v2 cargo test --workspace --test p1289_float_is_infinite
```

Resultados: pré-candidato `1 Preserved / 10 Violated / 0 Unknown`, exit 1
contratual; candidato `11/11 Preserved / 0 Violated / 0 Unknown`, exit 0;
forward/reverse determinísticos nos dois; cinco mutações mortas de cinco,
`mutation_score = 1.0`; teste focal 1 passou, 0 falhou. O hash do candidato
permaneceu congelado depois do teste.

## V3.3 Ownership e gates arquiteturais

O achado P1289 direto da v2 foi removido:

- `crystalline-lint --checks v1 .` → exit 0, nenhuma violação;
- o teste possui owner L0 individualizado e `@prompt-hash 58174302`;
- `crystalline-lint --checks v15 --fail-on warning .` → exit 0;
- `crystalline-lint --checks v26 --fail-on warning .` → exit 0;
- `crystalline-lint .` → exit 0, embora reporte warnings gerais.

`crystalline-lint --checks v5 --fail-on warning .` termina com exit 1 por 14
drifts concorrentes. Nenhum aponta ao novo teste, ao novo L0 ou aos cinco
consumers P1289. Logo V1/V5 focal de P1289 está verde; V5 global
fail-on-warning permanece vermelho por estado externo ao passo.

## V3.4 Sonda padrão P1284

O runner padrão executou bilateralmente, sem edição de harness/fixtures, com
o mesmo universo de 111 ids do artefato P1284:

```text
python3 lab/surface-inventory/run_probes.py --profile default --vanilla-bin /usr/local/bin/typst --crystalline-bin /tmp/p1289-target-pre/debug/typst --output /tmp/p1289-v3-p1284-pre-default.json --probes lab/surface-inventory/probes.json --inventory 00_nucleo/diagnosticos/p1284-inventory-default.json
python3 lab/surface-inventory/run_probes.py --profile default --vanilla-bin /usr/local/bin/typst --crystalline-bin /tmp/p1289-target-b-v2/debug/typst --output /tmp/p1289-v3-p1284-candidate-default.json --probes lab/surface-inventory/probes.json --inventory 00_nucleo/diagnosticos/p1284-inventory-default.json
```

| Estado | MATCH | diferença/disabled | SHA-256 |
|---|---:|---:|---|
| artefato P1284 | 62 | 49 | `da86beaf2f94dd21458f27f84696f47a621fd10c131ae3b1fb76ece4b0e211fb` |
| pré-candidato P1289 | 72 | 39 | `824029595fce0495680be9be38a6a153ab8a5546898371e3dd47d1bfee59ab83` |
| candidato v3 | 95 | 16 | `387b71af376d6795bba70169a80ee55ba64ee4857cc78ea76df857b4840f4d41` |

Regressões dos 62 `MATCH` antigos: **zero**. Regressões pré-candidato →
candidato: **zero**. Os 23 novos `MATCH` pré-candidato → candidato são
exatamente:

1. `present-emoji-heart`;
2. `present-sym-arrow`;
3. `sample-match-emoji.bell`;
4. `sample-match-emoji.camera`;
5. `sample-match-emoji.checkmark`;
6. `sample-match-emoji.heart`;
7. `sample-match-emoji.ship`;
8. `sample-match-emoji.thumb`;
9. `sample-match-math.arrows`;
10. `sample-match-math.csc`;
11. `sample-match-math.dim`;
12. `sample-match-math.lt`;
13. `sample-match-math.sharp`;
14. `sample-match-sym.approx`;
15. `sample-match-sym.emptyset`;
16. `sample-match-sym.integral`;
17. `sample-match-sym.rect`;
18. `sample-missing_member-float.is-infinite`;
19. `sample-missing_member-math.bb`;
20. `sample-missing_member-math.frak`;
21. `sample-missing_member-math.inline`;
22. `sample-missing_member-math.scripts`;
23. `sample-missing_member-math.serif`.

Somente o item 18 pertence ao contrato P1289. Nesta primeira medição direta,
o ganho atribuível a P1289 era 1/23, mas a exclusividade `+1 MATCH` ainda não
podia ser promovida porque o binário congelado incorporava 22 ganhos
concorrentes. O runner não ficou bloqueado nem produziu `Unknown`. A medição
contrafactual isolada de V3.6 remove essa limitação sem alterar a workspace.

## V3.5 Gates finais

| Gate | Resultado v3 |
|---|---|
| `cargo build --workspace` | exit 0, com warnings gerais |
| `cargo test --workspace` | exit 101; suíte avançou, mas `p1168_html_typed_batch_repr_and_dom` falhou em `04_wiring/tests/cli.rs` |
| `cargo fmt --all -- --check` | exit 1; diffs concorrentes em `eval/repr.rs`, `math/layout/mod.rs` e `stdlib/structural/math.rs` |
| rustfmt focal nos cinco consumers + teste P1289, `skip_children=true` | exit 0 |
| `git diff --check` | exit 0 |
| V1/V15/V26 e lint global | exit 0 |
| V5 `fail-on-warning` global | exit 1, 14 drifts concorrentes, nenhum P1289 |

## V3.6 Isolamento contrafactual do delta `+1 MATCH`

Às `2026-08-31T12:04:59-03:00`, no mesmo HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, foi copiado um snapshot para
`/tmp/p1289-isolation-v3/candidate` contendo somente `Cargo.toml`,
`Cargo.lock`, `01_core`, `02_shell`, `03_infra`, `04_wiring` e `benches`, sem
`target`. O controle foi clonado desse snapshot, não recopiado da workspace,
portanto ambos começaram byte a byte idênticos no mesmo instante.

O inventário ordenado de SHA-256 por caminho relativo contém 512 ficheiros.
O próprio inventário tem SHA-256
`81463d6bfa6a9dd3d153cd2ee36ae58b2646e2ce0424dc91f3021a7493b9b7a7`
tanto no candidato como no controle antes do patch. Após aplicar o patch
somente no controle via `apply_patch`, seu inventário de 511 ficheiros tem
SHA-256
`8d3e094d5cf5e138d1aeb8ae6d176ba008df7bc6a1328d608b3845ab3896623a`.
Os inventários foram recalculados depois dos builds e permaneceram
idênticos aos respectivos valores anteriores ao build.

O patch controle → candidato tem 169 linhas, SHA-256
`b43d3cc27226f457cc0474c383be402762e0f8ab5d043e3b6c016a00513c1b81`
e o `diff -qr` enumera exatamente estes cinco caminhos, sem outro delta:

| Consumer | candidato | controle |
|---|---|---|
| `stdlib/foundations/float.rs` | `fcf93a0bfc26a73d2b150d05089a0ff2ae5f2a55fbeb587ac402f196c7aad130` | ausente |
| `stdlib/foundations/mod.rs` | `f94a2d4523e06893bda780c9fb400736e2686499cac176d747e4df48509d08c0` | `fc3c2f2dd0676e77cfafe19c0b071b8ced5811f7ee7fbb54e2f3a486d60a332c` |
| `stdlib/mod.rs` | `c8d2ef3b6deec9ef979cfc48620b6519b83925388d912bbab0dc7a567aa15792` | `aa4fcd38f9d80f69fd3cab4624aa3101e08c356e93537aac3bf8d756d2fff661` |
| `eval/bindings/field_access.rs` | `979ed86569f6532815df3de25399efccba9585ef8daeb63f079e15132c732882` | `8964287ab5f3c5f49c950a1cea82b1d8b67704c4bf3994c8bc7860762241b74d` |
| `eval/call_dispatch.rs` | `c249305d329cb42d73966ac53f9d26b765b30ed42d972310b9a3c13f19d366dd` | `9e93b2526b5c1ca21896037d3bea6791b40c2f716558b2598c65b22b08d898d3` |

Inspeção integral dos hunks confirma somente o delta funcional P1289
solicitado: presença de `float.rs`, declaração/reexports correspondentes,
braço `Type::Float` em field access e braço `Value::Float` que reconhece e
despacha o método de instância em call dispatch. Headers e documentação
permaneceram fora da decisão funcional; nenhum artefato da workspace foi
alterado.

Os dois binários foram compilados em paralelo, com target dirs separados e
o mesmo `TYPST_COMMIT_SHA=53d21c5a602f4045a769a0ab0c935baa5ecd3b88`:

```text
CARGO_TARGET_DIR=/tmp/p1289-isolation-v3/target-candidate TYPST_COMMIT_SHA=53d21c5a602f4045a769a0ab0c935baa5ecd3b88 cargo build --bin typst
CARGO_TARGET_DIR=/tmp/p1289-isolation-v3/target-control TYPST_COMMIT_SHA=53d21c5a602f4045a769a0ab0c935baa5ecd3b88 cargo build --bin typst
```

Ambos terminaram com exit 0 e reportam `typst 0.15.1 (53d21c5a)`. O binário
candidato tem SHA-256
`abc2c63e67f291cef899ab510f79036c14a14f0f423f2cc96a08c8e80cf66953`;
o controle,
`61363c4bd6d8709fe06a49baf3f3bbedad75aacec41db0d14d41514b188e6ed1`.

O runner padrão P1284 foi então executado contra ambos com o mesmo vanilla,
profile, probes e inventário. Os dois JSONs contêm os mesmos 111 ids únicos,
na mesma ordem do artefato P1284:

| Build temporário | MATCH | diferença/disabled | SHA-256 do JSON |
|---|---:|---:|---|
| controle | 94 | 17 | `9ecdbc62812bbc16efb255e38ee63d8e931ab345bf01763ef6dc19b961c75224` |
| candidato | 95 | 16 | `981d1ef8a81ec3a24d0d10dfc142662e47a54a7ac5718742c79dfe8b6447e206` |

A comparação por id possui exatamente uma transição e zero regressões:

```text
sample-missing_member-float.is-infinite:
  controle  DIFFERENCE_OR_DISABLED
  candidato MATCH
```

No controle, `repr((type(float.is-infinite), repr(float.is-infinite)))`
termina com exit 1 e `type float does not contain field "is-infinite"`; no
candidato e no vanilla termina com exit 0 e
`"(function, \"is-infinite\")"`. Nenhum dos 94 `MATCH` do controle nem dos
62 `MATCH` históricos de P1284 regrediu. Portanto o critério focal de ganho
**exatamente `+1 MATCH` para `float.is-infinite`, com zero regressões, está
isoladamente atestado**. Não houve ambiguidade de build/diff e o resultado
não é `Unknown`.

## V3.7 Veredito proporcional

O fragmento `C-P1289-FLOAT-IS-INFINITE-v3` está **Preserved**: integridade
dos pins, RED→GREEN, 11/11 observáveis, zero `Unknown`, 5/5 mutações, teste
focal e ownership 1:1 verdes. O blocker V1 direto da v2 foi efetivamente
removido.

O controle contrafactual temporário agora isola e atesta o delta `+1 MATCH`
do P1289. O fecho global do Passo 1289, porém, continua **não atestável neste
checkout** porque os gates workspace, formato global e V5 fail-on-warning
permanecem vermelhos por trabalho concorrente. Isso não refuta a semântica
focal nem o delta isolado P1289, mas impede declarar que todos os critérios de
fecho do passo estão satisfeitos. Resultado: **executado e atestado por
papel/artefato para o fragmento e seu delta funcional isolado, sem atestação
de isolamento forte do checkout completo e sem equivalência funcional
geral**.

## V3.8 Revalidação final do snapshot atual

Não foram repetidos builds ou probes focais já atestados. Houve duas janelas
de gates globais. Na primeira, de `2026-08-31T12:14:29-03:00` a
`2026-08-31T12:16:03-03:00`, HEAD, `git status --short`,
`git diff HEAD --stat` e os 11 artefatos P1289 permaneceram idênticos. Porém,
o inventário geral usou `xargs sha256sum` sem o separador `--`: um caminho foi
interpretado como opção e somente 7.048 de 9.558 caminhos foram processados.
O hash incompleto `d50d6732…` foi descartado e essa primeira janela **não é
usada para alegar estabilidade completa**.

A reexecução autoritativa decorreu de `2026-08-31T12:19:21-03:00` a
`2026-08-31T12:20:20-03:00`, sempre no HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`. O inventário correto foi
produzido por:

```text
git ls-files -co --exclude-standard -z | LC_ALL=C sort -z | xargs -0 sha256sum --
```

| Proveniência | Antes | Depois |
|---|---|---|
| caminhos inventariados | 9.558 | 9.559 |
| SHA-256 do inventário geral | `4b30e56f543d5c7751333dbb1ada176268379ffd98bc1486660b1c7af402a374` | `cac5f342ed008e9be0c809135cb0327fee61048e5f457bee4d3bb533d8cf594b` |
| linhas/SHA de `git status --short` | 308 / `a66d4cc1f03784fe3bcbc7af5b872a457f022afa71d5cc5da503655e4c1d6d89` | 309 / `38b701910ce7a7a07b8847b4ccbec96bc53eea9a7e53fd6ec686a96a3771ac6d` |
| `git diff HEAD --stat` | 144 ficheiros, 649087 inserções, 1975 remoções / `e91436029b557a0522b0e151079c3727b33783390b065e7847b26b0ed40002f4` | idêntico |
| inventário dos 11 artefatos P1289 | `4a4bf40860c55d468672c231ce7dc0bdc4a1cca6d15179d02bfe9efbebe98b9f` | idêntico |

A diferença completa antes/depois tem exatamente um caminho novo,
concorrente e alheio ao P1289:
`00_nucleo/diagnosticos/p1290-candidate-gate.py`, SHA-256
`ecd7c09865a9c42e78b565e56af34c932e7af806286327bd0ca69ee91b296d26`.
Por regra, o snapshot global desta janela é **instável**. Os artefatos P1289
permaneceram estáveis, mas os resultados globais abaixo são observações de
uma árvore em movimento, não um selo de snapshot final.

| Gate atual | Resultado da janela instável |
|---|---|
| `cargo test --workspace` | exit 101 antes de executar a suíte; compilação falhou com 31 erros concorrentes, incluindo E0061 em `eval/modules.rs`/`eval/mod.rs` e E0063 por campos `summary`/`kind` ausentes em `entities/content.rs` |
| `cargo fmt --all -- --check` | exit 1; diffs em `eval/repr.rs`, `math/layout/mod.rs`, `stdlib/structural/math.rs`, `entities/elements/table.rs`, `entities/elements/table_cell.rs`, `entities/html.rs` e `02_shell/src/cli.rs` |
| `crystalline-lint .` | exit 1; fatal V0 concorrente: metadata de hash canônico malformada em `00_nucleo/prompts/entities/compiler_features.md` |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | exit 1; o mesmo fatal V0 mais 14 warnings V5; nenhuma linha V15/V26, mas o fatal impede promover sucesso desses checks nesta janela |
| `git diff --check` | exit 0, confirmado novamente após os demais gates |

Os blockers globais **ainda existem** e agora incluem compilação/teste
workspace, formato global, fatal V0 e V5 global com `fail-on warning`.
Nenhum deles aponta aos cinco consumers ou ao teste P1289, cujos hashes
ficaram estáveis. O veredito proporcional permanece: fragmento e delta
isolado P1289 atestados; fecho global retido, snapshot global final instável
e nenhuma equivalência funcional geral declarada.

---

# Histórico — P1289 v2 — recibo de verificação final A/B

**Estado:** `FINAL_VERDICT_WITHHELD`  
**Veredito:** **não conforme para fecho do Passo 1289**  
**Fragmento focal:** `float.is-infinite` preservado pelo contrato v2  
**Regime:** Tekt A/B segregado por papel, capacidades, ordem e hashes; sem
atestação de isolamento forte do checkout compartilhado.  
**Verificador:** `/root/verificador_p1289`  
**Janela desta verificação:** `2026-08-31T11:29:13-03:00`–
`2026-08-31T11:36:02-03:00`.

## 1. Escopo e autoridade

Este papel leu os artefatos congelados e os cinco consumers candidatos e não
os corrigiu. A única escrita persistente deste papel é este recibo; as duas
saídas da sonda padrão foram gravadas em `/tmp`. O checkout e o host são
compartilhados com outros passos, logo hashes identificam os bytes executados,
mas não provam isolamento ambiental nem autoria exclusiva.

O veredito cobre somente o fragmento observável C-P1289-FLOAT-IS-INFINITE-v2.
Não declara equivalência funcional geral do compilador.

## 2. Proveniência e identidades verificadas

No início da verificação, `HEAD` era
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, com working tree não commitida.
No snapshot de `2026-08-31T11:36:02-03:00`:

- `git status --short`: 281 linhas; SHA-256
  `462de3fb0a82c0a496b24d2907734154fb57a64db836612f43383b240b955a94`;
- `git diff HEAD --stat`: 137 ficheiros, 648996 inserções, 1888 remoções;
  SHA-256
  `ba407ce84f9c1f6af398e943925075df0ba0baf64829e9058538d5767f14af25`.

A árvore mudou enquanto papéis concorrentes trabalhavam; por isso cada prova
abaixo está presa também aos hashes individuais.

| Entrada | SHA-256 recalculado | Resultado |
|---|---|---|
| manifesto v2 | `e3c45265597b5581985c0f0986807adededa369de1938d6e8a49517d6a9d2bdd` | coincide |
| runner A v2 | `0f648c8c1f923de48df24032d2dc7beb76be1ad1c6c2b5429567602f31c26abd` | coincide |
| baseline A v2 | `c3a3f96f7a8ff397aca23b83bbd1de827ea95d9c02328b88c3a77e4438e6a0ce` | coincide |
| integration test A v2 | `17ff6af64d73b26cb1ac03ec124221228f1cf41dcf4f6f9d9237538e45e4ec2d` | coincide |
| vanilla `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` | coincide |
| pré-candidato | `e7c81e3a6c6f1933db95c2249286fca19f3b5991eb8bc392f49ea659cec0bd78` | coincide |
| candidato B v2 | `a9f98e220f3efb1776f48c0ec05fd5d085c04c9404c4fe5ca0091eab649a43de` | coincide antes e depois do teste focal |

Os cinco L0s também coincidiram integralmente com o manifesto:

| L0 | SHA-256 |
|---|---|
| `foundations/float.md` | `b05da320cddd30a455dd60e027bcd8639261fff9be16b39dc4dd82610ea2b242` |
| `field_access.md` | `90155e0c0ed88d36e1c9d198782387a99ea7ad1813cbfd7402b8eca1176e4692` |
| `call_dispatch.md` | `66e9a245c7916e261a3c3970d9dd57f52d4a2626afe2afe13ae0a768b2933a6c` |
| `foundations.md` | `8df1e1b9317b6eeb11c691ed2126bd76901c8e2c207278172ce2e79984b0bc34` |
| `_comum.md` | `29aee6d1d5413c0023e6feeb3151bc4642705a556313cdf8f6bac86d92291d7b` |

E os cinco consumers permaneceram exatamente nos hashes recebidos:

| Consumer | SHA-256 |
|---|---|
| `foundations/float.rs` | `fcf93a0bfc26a73d2b150d05089a0ff2ae5f2a55fbeb587ac402f196c7aad130` |
| `foundations/mod.rs` | `f94a2d4523e06893bda780c9fb400736e2686499cac176d747e4df48509d08c0` |
| `stdlib/mod.rs` | `c8d2ef3b6deec9ef979cfc48620b6519b83925388d912bbab0dc7a567aa15792` |
| `field_access.rs` | `979ed86569f6532815df3de25399efccba9585ef8daeb63f079e15132c732882` |
| `call_dispatch.rs` | `c249305d329cb42d73966ac53f9d26b765b30ed42d972310b9a3c13f19d366dd` |

A allowlist declarada é coerente com os papéis: A escreveu somente runner,
baseline, integration test e recibos; B declara somente os cinco consumers; o
verificador não escreveu nenhum artefato avaliado. Isto é segregação por
papel/artefato, não prova física de que o checkout compartilhado impediu leitura
ou compilação de mudanças concorrentes.

## 3. RED, GREEN e poder discriminatório

Comandos reproduzidos:

```text
python3 lab/surface-inventory/run_p1289_oracles.py --candidate /tmp/p1289-target-pre/debug/typst --summary-only --pretty
python3 lab/surface-inventory/run_p1289_oracles.py --candidate /tmp/p1289-target-b-v2/debug/typst --summary-only --pretty
python3 lab/surface-inventory/run_p1289_oracles.py --self-test --pretty
```

Resultados:

- pré-candidato: exit 1 contratual, 1 `Preserved`, 10 `Violated`, 0
  `Unknown`, forward/reverse determinísticos;
- candidato: exit 0, 11/11 `Preserved`, 0 `Violated`, 0 `Unknown`,
  forward/reverse determinísticos;
- mutações: `always_false`, `nan_is_infinite`,
  `accept_extra_or_unknown_named`, `qualified_repr` e
  `presence_without_call` mortas, 5/5, `mutation_score = 1.0`, sem
  testemunha `Unknown`.

O teste protegido foi executado por:

```text
CARGO_TARGET_DIR=/tmp/p1289-target-b-v2 cargo test --workspace --test p1289_float_is_infinite
```

Resultado: 1 teste passou, 0 falhou. Assim, o contrato focal distingue o
pré-candidato e preserva no candidato a superfície, chamadas estática/ligada,
valores e diagnósticos registrados.

## 4. Sonda padrão P1284

O runner padrão foi executado sem editar harness, inventário ou fixtures, com
os mesmos 111 ids do artefato
`00_nucleo/diagnosticos/p1284-probes-default.json`, contra pré-candidato e
candidato:

```text
python3 lab/surface-inventory/run_probes.py --profile default --vanilla-bin /usr/local/bin/typst --crystalline-bin /tmp/p1289-target-pre/debug/typst --output /tmp/p1289-p1284-pre-default.json --probes lab/surface-inventory/probes.json --inventory 00_nucleo/diagnosticos/p1284-inventory-default.json
python3 lab/surface-inventory/run_probes.py --profile default --vanilla-bin /usr/local/bin/typst --crystalline-bin /tmp/p1289-target-b-v2/debug/typst --output /tmp/p1289-p1284-candidate-default.json --probes lab/surface-inventory/probes.json --inventory 00_nucleo/diagnosticos/p1284-inventory-default.json
```

| Estado | MATCH | diferença/disabled | SHA-256 da saída |
|---|---:|---:|---|
| artefato P1284 | 62 | 49 | `da86beaf2f94dd21458f27f84696f47a621fd10c131ae3b1fb76ece4b0e211fb` |
| pré-candidato P1289 | 72 | 39 | `824029595fce0495680be9be38a6a153ab8a5546898371e3dd47d1bfee59ab83` |
| candidato B v2 | 78 | 33 | `f8b192f3a81c90645bc8f0b2aaf520be26e2a5024a509b42acf314105401a35e` |

Os 62 `MATCH` antigos permanecem `MATCH`: regressões antigas = 0. Porém a
transição pré-candidato → candidato é **+6**, não +1:

- `float.is-infinite`;
- `math.bb`;
- `math.frak`;
- `math.inline`;
- `math.scripts`;
- `math.serif`.

Contra o artefato P1284, o candidato tem +16 devido a dez ganhos já presentes
no pré-candidato e aos seis acima. O runner padrão não ficou bloqueado pelo
harness P1288: executou bilateralmente, preservou o universo exato de 111 ids e
produziu resultados comparáveis. A limitação está no candidato compilado a
partir da árvore compartilhada: ele contém cinco ganhos concorrentes além de
P1289. A associação nominal desses cinco ganhos a trabalho matemático
concorrente é inferência; um binário reconstruído em worktree isolada contendo
somente o delta P1289 a confirmaria ou refutaria. No binário congelado atual, o
critério obrigatório de ganho exatamente `+1 MATCH` está objetivamente
violado, não pode ser convertido em sucesso nem em equivalência geral.

## 5. Gates finais

| Gate | Resultado |
|---|---|
| teste focal protegido | exit 0; 1 passou |
| `cargo build --workspace` | exit 0, com warnings da árvore geral |
| `cargo test --workspace` | **exit 101**; 5329 passaram, 8 falharam: seis testes P1290 de `repr` e dois de math layout |
| `cargo fmt --all -- --check` | **exit 1**; diffs em `eval/repr.rs`, `math/layout/mod.rs` e `stdlib/structural/math.rs`, fora dos cinco consumers P1289 |
| `rustfmt --edition 2021 --check --config skip_children=true` nos cinco consumers | exit 0 |
| `git diff --check` | exit 0 |
| `crystalline-lint .` | **exit 1**; erro V1 direto no teste protegido `04_wiring/tests/p1289_float_is_infinite.rs:1`, além de warnings gerais |
| `crystalline-lint --checks v5 --fail-on warning .` | **exit 1**; 14 drifts em outros consumers; nenhum dos cinco consumers P1289 |
| `crystalline-lint --checks v15 --fail-on warning .` | exit 0; nenhuma violação |
| `crystalline-lint --checks v26 --fail-on warning .` | exit 0; nenhuma violação |

O V1 no próprio teste A é blocker P1289 direto: o artefato protegido não tem
header `@prompt`. Este verificador não o corrigiu porque fazê-lo quebraria o
selo A e violaria sua autoridade somente leitora. As falhas da suíte, formato
global e V5 são externas aos cinco consumers P1289, mas ainda impedem alegar
que os gates finais obrigatórios estão verdes no estado medido.

## 6. Veredito proporcional

O fragmento C-P1289-FLOAT-IS-INFINITE-v2 está **Preserved** no candidato
pinado: RED→GREEN reproduzível, 11/11 observáveis, zero `Unknown`, cinco
mutações mortas e score 1.0. V15/V26 e a formatação focal dos consumers passam.

O Passo 1289, contudo, fica **não conforme para fecho** neste estado porque:

1. a sonda padrão demonstra +6 `MATCH` entre pré-candidato e candidato, e não
   o ganho exclusivo +1 exigido;
2. o teste protegido viola V1;
3. `cargo test --workspace`, `cargo fmt --all -- --check`, o lint global e V5
   fail-on-warning não estão verdes.

Resultado de segregação: **executado com segregação por papel e artefatos, sem
atestação de isolamento forte**. É necessário refazer o candidato numa árvore
isolada do trabalho concorrente e regenerar/recongelar a fase A se o teste
protegido for alterado; somente então os gates e o delta padrão podem ser
reverificados.
