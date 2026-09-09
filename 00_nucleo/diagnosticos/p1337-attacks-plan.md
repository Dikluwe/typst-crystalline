# P1337 — plano adversarial congelado antes de C

Regime A/B executado sem atestação técnica de isolamento, sem contrato ou selo
de refinamento. Executor `/root/p1337_attacks`, contexto novo limitado às
instruções do papel. Reviewer `/root/p1319_review` julga sem editar entradas.
UTC de redação 2026-09-09T19:18Z; instante exato e hashes no recibo de freeze.

Manifesto efetivo `p1337-manifest-r1.json` SHA-256
`7c09595cde021b89fffa7c316b7706797ba3898aa5b6687cdfaf2deacb0e65ff`.
Manifesto R0 das sondas SHA-256
`b089feee37941c88de47154adbde550f568d65f076716cdde5ef69beb4483b17`.
R1 muda somente identidade/contexto do reviewer e proveniência dessa sucessão;
L0 normativo, baseline, escopo e capacidades adversariais ficam constantes.

## Medição anterior à decisão

Sondas frescas `p1337-attacks-oracles.json` SHA-256
`1a0ef275ea38c77c89562fe76f43058a5f629f8c8f888966321c897303675d28`
preservam 14 execuções, argv, fontes, canais, UTC e custo com HEAD/diff-stat.
Vanilla `/usr/local/bin/typst`, upstream `a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Baseline `/tmp/p1336-target.QiOMGq/release/typst`, SHA-256
`646a8d97400c0abe262a65c9b9559b47a3ecf7d10eafd82fa79a64ade0504497`.

`false.unavailable` publica bool no baseline, boolean no vanilla. Aliases
parentetizados/multilinha Bool, None, Auto com field `ausência` publicam
âncora do receiver no baseline e somente field no vanilla. Array e valor-tipo
Bool têm dívidas próprias de nome/âncora, explicitamente excluídas. O positivo
`(not false, type(true), repr(none), repr(auto))` coincide integralmente.
Fonte baseline lida em `field_access.rs:532–550,839–849`: seleção fechada de
field-only omite Bool/None/Auto; fallback nominal separa Int/Str dos demais.
Não foi lido candidato C, ainda inexistente neste momento.

## Cinco famílias congeladas

| Família | Mutação produtiva exclusivamente na cópia | Testemunha e obrigação |
|---|---|---|
| M1 | Interceptar Bool no lookup puro e publicar `cannot access fields on type bool`, conservando span | `false.unavailable`; qualquer Bool exige `boolean` |
| M2 | Omitir somente Bool da seleção AST field-only | alias Bool multilinha/Unicode; exatamente todos bytes do field |
| M3 | Omitir somente None da seleção AST field-only | alias None multilinha/Unicode; exatamente todos bytes do field |
| M4 | Omitir somente Auto da seleção AST field-only | alias Auto multilinha/Unicode; exatamente todos bytes do field |
| M5 | Acrescentar Array à seleção AST field-only, extrapolando o recorte | `(3, 5).ausência`; conservar âncora integral e mensagem baseline, sem crédito de paridade |

A forma textual dos patches será resolvida após C e testes congelados sem
mudar estas famílias, orçamento ou testemunhas. Alterações ficam exclusivamente
em código produtivo da cópia; todos os testes são preservados byte a byte.
Inferência: as cinco regressões são distinguíveis por testes independentes
derivados do L0. Falta de teste discriminante, falha de compilação, cache antigo
ou identidade incerta refuta suficiência e resulta em Unknown/bloqueio.

## Perfil, capacidades, custo e critérios

Antes de C fica fixado o comando igualmente para controle C e cinco mutantes:

```
cargo test --release --config profile.release.package.typst-core.opt-level=0 --locked --offline -p typst-core --lib p1337_tests -- --nocapture
```

`CARGO_BUILD_JOBS=2`; `CARGO_TARGET_DIR` exclusivo temporário. Somente typst-core
usa opt-level=0; dependências mantêm release. Cargo.toml/código de produto não
são alterados por instrumentação. Esse perfil prova discriminação instrumental;
não prova execução release normal, cujos gates gerais cabem ao operador.
Controle C neste mesmo perfil é obrigatório antes de qualquer kill válido.

Budget: cinco famílias, uma rodada principal por família e controle C; timeout
2700 s por Cargo, 30 s por CLI. Dois ajustes sem ganho na mesma causa interrompem
o método e exigem revisão; não converter Unknown em sucesso. Custo real em UTC,
segundos e processos é publicado por rodada, com delta de discriminação.

Entradas legíveis: skill e referências, CLAUDE raiz/core, ADRs aplicáveis, L0
completo e manifestos; baseline e fontes baseline, binários pinados, scripts
P1336 como referência sem edição. C apenas após este plano congelado; testes
independentes apenas após seu freeze. Nenhuma leitura/listagem de pastas
materialization/context. Saídas graváveis somente `p1337-attacks-*` em
diagnósticos e workspace/target temporários exclusivos. Sem escrever main,
oráculos/testes alheios, planos congelados ou evidência histórica. Capacidade
técnica do processo é mais ampla que a allowlist declarada: não há atestação.

Copiar workspace/target sem hardlinks mutáveis; conferir inodes. Cada troca de
fonte via apply_patch recebe timestamp fresco. Exigir `Compiling typst-core`
com o caminho real da cópia, término normal de compilação, testes executados e
SHA-256 do executável efetivamente executado. Preservar fonte, patch, stdout,
stderr e cópia do executável por rodada antes do próximo link. Rejeição exige
teste discriminante falhando semanticamente, nunca mero exit 101 de compilação.
Recibos individuais são imutáveis; agregado final só após todas rodadas (ou
parada explícita inconclusiva). Reviewer decide aplicabilidade e testemunhas.
