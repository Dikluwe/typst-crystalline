# V-P1283-v1 — certificado de verificação independente final

**Veredito:** `Conformant`

**Escopo atestado:** somente o fragmento observável de C-P1283-v4 para `sym`,
`emoji`, o espelho declarativo `sym → math`, as extensões `registered` e os
diagnósticos de depreciação explicitamente classificados. Este certificado não
afirma equivalência funcional geral de Typst.

**Regime:** protocolo completo de materialização segregada, executado sem
atestação técnica de isolamento. O verificador não editou contrato, L0,
baseline, implementação, testes, catálogos, probes ou relatório; escreveu
somente este certificado depois do veredito.

## 1. Estado e entradas verificadas

- instante final dos gates: `2026-08-30T10:34:44-03:00`;
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`;
- estado: working tree não commitado;
- proveniência da medição: `p1283-summary.json`, SHA-256
  `3b7a9ba3c1e3e95efdf5f91fd92d50a02755646b7a354bc26abcc8e5081893c8`,
  medido em `2026-08-30T13:25:23.068645+00:00`, com
  `git_status_short`, `git_diff_head_stat` e hashes dos inputs;
- produto cristalino: `target/release/typst`, SHA-256
  `a6e03f6c3da68e689c3baa1d06d1fd8f6665b9294786e4add145eb13b4f47917`;
- baseline vanilla: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
  revisão ratificada `a51e02804`;
- C-P1283-v4: `p1283-contract-receipt.md`, SHA-256
  `80eb980b8f22e23f3c9f56e2526f28a8591bde5eae3849b44348c60af239dc3f`;
- A-P1283-v4: `p1283-adversarial-receipt.md`, SHA-256
  `47da00869521924acfb99d6ec2ee133f3147fe113e74fc70e7ae1cfbb42016d5`,
  pinando o SHA integral do contrato;
- Passo 1283 explicitamente autorizado: SHA-256
  `67de6827bf12e281e45cda4fa4d1ed67dd1a9956f3d55b3f36f9a1ae9e6d892f`.

Os inputs declarativos foram conferidos diretamente no cache Cargo:

| Input | Resultado |
|---|---|
| `codex = 0.3.0` | pin exato no workspace e checksum Cargo `0732ab1a27b4ea05e6f9f60a5122c9924dd5123defde0d8e907f58cf643d40e6` |
| `sym.txt` | 1.340 linhas; SHA-256 `6ee467d9939acb5c7d0a3eba30c9f640d157529cbf57df752367deb343e0fc16` |
| `emoji.txt` | 1.472 linhas; SHA-256 `8691ca68e09b6fedca61e00e824648e79f7502772eefe7af367e404e26161489` |

O rebuild release final reproduziu o mesmo SHA do produto usado pelos catálogos;
portanto, os recibos não apontam para um binário candidato obsoleto.

## 2. L0, hashes e ownership

| Prompt L0 | SHA-256 | Consumer único | SHA-256 do consumer |
|---|---|---|---|
| `compiler/stdlib/sym.md` | `42a2127215e9a37d69c90695e1be66a61320fa086ffd36b51a1ac3441c816f3d` | `01_core/src/compiler/stdlib/sym.rs` | `8f45dc0953e591b8b058370b6a935a7982dcf25aeb3dcf625bcdf05898fa256f` |
| `compiler/stdlib/emoji.md` | `d5fda4cff41bab53b5e877061a63aeb1fda57896d328552a637e6e9cc8e3291f` | `01_core/src/compiler/stdlib/emoji.rs` | `4a31ea32e78cec391a4088fd953fa03871dea8039be45bc05ff13029a462077c` |
| `compiler/stdlib/structural/math.md` | `4441fe00c99c3475179c4a90060f5698e4472a368756ead6e4312781229617bf` | `01_core/src/compiler/stdlib/structural/math.rs` | `8e03f4e7e87cbeb91045842fda784310e2e112d1716fdc775f51812976330873` |

Busca bilateral de `@prompt` encontrou exatamente um consumer produtivo para
cada L0. Nenhum dos três declara Núcleo Tekt. `crystalline-lint . --quiet`
passou duas vezes durante a verificação, inclusive depois da materialização dos
recibos C/A, cobrindo V5, V15 e V26 sem violações.

## 3. Observáveis e inventário

Os catálogos `default` e `html` foram comparados diretamente path a path,
incluindo kind e metadata integral de `Symbol`:

| Família | Baseline e candidato observados | Resultado |
|---|---|---|
| `sym` | baseline: 3 módulos (`sym`, `gender`, `control`) + 334 símbolos + 1.206 records; candidato: o mesmo subgrafo mais `sym.registered` | `Preserved`, com extensão separada |
| `emoji` | 1 módulo + 772 símbolos + 1.386 records nos dois lados | `Preserved` |
| `math` declarativo | mesmo subgrafo de 334 símbolos, 2 submódulos e 1.206 records; `sqrt`, `class`, `equation` e `op` continuam funções | `Preserved` |

Não houve path ausente, kind divergente ou metadata divergente em `sym`/`emoji`.
Em `math`, os 43 `MISSING_MEMBER` e quatro `UNVERIFIED_METADATA` restantes são
funções não declarativas fora do contrato e não receberam crédito neste passo.
`sym.registered` e `math.registered` são os únicos extras das famílias
selecionadas, ambos símbolo `®`, ausentes no vanilla e classificados como
extensão sem crédito de paridade. `sym.sqrt` permanece ausente.

O before/after reproduzido em ambos os perfis foi:

| Família | Before P1282 | After P1283 |
|---|---|---|
| `sym` | 34 MATCH; 228 missing; 38 metadata; 37 bloqueados; 1 extra | 337 MATCH; 0 missing; 0 metadata; 0 bloqueados; 1 extra |
| `emoji` | 464 MATCH; 241 missing; 68 metadata | 773 MATCH; zero restantes |
| `math` | 83 MATCH; 271 missing; 42 metadata; 37 bloqueados; 1 extra | 386 MATCH; 43 missing; 4 metadata; 0 bloqueados; 1 extra |

Os hashes before pinados no resumo também foram reproduzidos:
`17923ca1aedb5dfb94e9e64d5eeae481ae33e3bb3c2917ceda4a00ea20a349c2`
(`default`) e
`3e389363f019c9c00f21fe47f23342a624b27c76d9c59002b870c5e1195659ed`
(`html`).

As sondas bilaterais deram 33/33 em `default` e 33/33 em `html`, com os
receipts SHA-256
`17074b88a523174dfc5636b43a6ff642e5f173c460a5762ae9ce7fea238a5099`
e `b59172c8baa89bf101c8cfb6354d8d1f30c1194c74f15d4a3feeca13ce176b9e`.
Além de stdout e exit code, o verificador inspecionou stderr: `sym.join`
produziu exatamente um warning idêntico em cada binário e perfil, sem hint,
com a mensagem `` `join` is deprecated, use `bowtie.big` instead ``.

O residual P1284, SHA-256
`42d99bb0bed744d8266cfee185b759eba781230e7ff8ac5a2dde64153fde7c97`,
é projeção exata do inventário after não-MATCH, exclui integralmente
`math`/`sym`/`emoji` e contém 459 entradas default e 573 HTML. As contagens
recalculadas coincidem com o ficheiro.

## 4. Gate mutacional observado

O score não foi herdado do relatório. O verificador carregou em memória os
catálogos vanilla/cristalino e os probes pinados, confirmou primeiro o candidato
sem violações e depois aplicou cada transformação de A-P1283-v4 a uma cópia
profunda. O classificador independente exigiu igualdade bilateral de paths,
kinds e metadata, espelho recursivo, proteção das quatro funções, extensões
separadas, igualdade dos probes e warning exato de `join`.

| ID | Mutação executada | Veredito observado | Testemunha concreta |
|---|---|---|---|
| N01 | remover `sym.alpha` | `Violated` | `sym.alpha: absent` |
| N02 | remover/achatar `sym.gender` | `Violated` | `repr(type(sym.gender))` deixa de ser `module`; descendants ficam ausentes |
| N03 | trocar kind de `sym.alpha` para módulo | `Violated` | kind esperado `symbol`, observado `module` |
| N04 | trocar base de `sym.alpha` para `β` | `Violated` | metadata difere de `α` |
| N05 | remover VS15 de `sym.trademark` | `Violated` | `™` difere de `U+2122 U+FE0E` |
| N06 | remover VS16 de `emoji.heart` | `Violated` | `❤` difere de `U+2764 U+FE0F` |
| N07 | truncar ZWJ de `emoji.dancing.ballet` | `Violated` | variant difere de `U+1F9D1 U+200D U+1FA70` |
| N08 | renomear `emoji.heart.lightblue` para `skyblue` | `Violated` | lista de variants e path original divergem |
| N09 | trocar `emoji.heart.arrow` de `💘` para `💝` | `Violated` | valor da variant diverge |
| N10 | fazer a ordem dos modifiers divergir | `Violated` | probe `bowtie.r.l.big.stroked` deixa de coincidir com `⟗` |
| N11 | usar primeira variant como default de `emoji.arrow` | `Violated` | `➡️` difere do best-match `↙️` |
| N12 | quebrar `.zero` somente em `sym.nothing` | `Violated` | identidade integral `emptyset`/`nothing` diverge |
| N13 | omitir `math.control.dc` | `Violated` | `math.control.dc: mirror absent` |
| N14 | sobrescrever `math.sqrt` com símbolo | `Violated` | binding próprio deixa de ser `function` |
| N15 | suprimir warning de `join` | `Violated` | warning singular/exato ausente |
| N16 | remover os dois `registered` | `Violated` | extensões ausentes; ramo adicional que as creditou como `MATCH` também foi rejeitado |

**Mutation score observado:** `16/16 = 1.0`. Nenhum `Unknown` entrou no
numerador e nenhum controle entrou no denominador.

Controles executados:

| ID | Resultado observado |
|---|---|
| C01 | `sym.gt.tri` existe, é símbolo, resolve para `⊳`, coincide em valor com o vanilla e não emite warning no cristalino: `IntentionalVariantDeprecationDivergence` |
| C02 | remover a variant `tri` e alegar a exceção: `Violated` por metadata/path ausente |
| C03 | manter `join` mas suprimir o warning exato: `Violated` |

Probes bilaterais adicionais confirmaram o mesmo padrão intencional em
`sym.lt.tri.eq.not` e `sym.tack.r.double`: valores iguais, um warning no
vanilla e zero no cristalino. `sym.join` permaneceu o controle oposto: um
warning exato nos dois binários.

## 5. Gates finais reexecutados

| Comando | Resultado |
|---|---|
| `cargo test -p typst-core p1283_ --offline -- --nocapture` | 5 passed; 0 failed |
| `python3 -m unittest lab/surface-inventory/test_merge.py lab/surface-inventory/test_run_probes.py` | 22 passed |
| `cargo test --workspace --offline --quiet` | 5.305 + 909 + 1 + 56 + 2 + 70 + 2 passed; 3 ignored; 0 failed |
| `cargo build --workspace --bin typst --offline` | exit 0 |
| `cargo build --workspace --bin typst --release --offline` | exit 0; SHA release reproduzido |
| `crystalline-lint . --quiet` | exit 0; zero violations |
| `cargo fmt --all -- --check` | exit 0 |
| `git diff --check` | exit 0 |

Warnings de compilação preexistentes não foram promovidos a falha por estes
gates e não afetam o fragmento C-P1283-v4.

## 6. Limitações e blockers

- A segregação é procedimental, não tecnicamente atestada: filesystem e
  contexto não foram confinados por allowlist/worktree independente.
- O verificador recebeu contrato, ataques e candidato no mesmo workspace, mas
  exerceu somente autoridade de leitura e de escrita deste certificado.
- `p1283-summary.json` conserva a etiqueta textual histórica
  `outside C-P1283-v3` para funções math não declarativas. C-P1283-v4 declara
  explicitamente que v3 foi invalidado apenas por resselo mecânico e que as
  obrigações semânticas são idênticas; a etiqueta é uma limitação menor de
  rastreabilidade, não uma mudança de escopo ou blocker.
- Os receipts C/A foram adicionados depois do instante de geração do resumo;
  seus hashes foram verificados separadamente e os hashes dos L0, consumers,
  binários e artefatos de medição permaneceram inalterados.
- As 13 depreciações de variant permanecem divergência intencional, não
  paridade diagnóstica. Mudá-las exige novo contrato público/gate ADR-0127.

**Blockers:** nenhum para fechar P1283 sob C-P1283-v4. `Unknown` não foi usado
como sucesso.

