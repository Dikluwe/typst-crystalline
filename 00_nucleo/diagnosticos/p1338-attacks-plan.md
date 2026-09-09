# P1338 — plano adversarial anterior a C

Regime A/B, executado sem atestação técnica de isolamento nem selo de
refinamento. Executor `/root/p1337_attacks`: contexto P1337 retido por limite
total de agentes; aquele candidato é baseline autorizado, nenhum C P1338 foi
lido ou existe na preparação deste plano. Reviewer `/root/p1319_review`.

Manifesto efetivo `p1338-manifest-r1.json` SHA-256
`14a36a87a116dee46464c32d358f3019702c201ea74809c90cc8bf81cdeeb843`;
antecedente R0 SHA-256
`8c51d637e9b248628d4bfe32b684a00f201b3139b84868b1f6cb351bb6290e4b`.
R1 corrige exclusivamente a referência canônica ao L0
`compiler/eval/bindings/access.md` e o header causal; a obrigação Array é
inalterada. Sondas R0 permanecem evidência imutável, sem repetição documental.

## Medição anterior à decisão

`p1338-attacks-oracles.json` SHA-256
`c1160832d8f73d98a6b297af60cc05d30e3020c41584532fc4ddec48a49b1af1`
contém 18 execuções frescas, fontes, canais integrais, UTC, custo e HEAD/diff-stat.
Baseline SHA-256 `6d85aece9e323950a8f722b11e87eb125b4a342a7404f969f60ea69595790b3c`;
binário `/tmp/p1337-target.F7jPNj/release/typst`, SHA-256
`55b5dc263bba15b050e9caab755c17deb22f617eec35b1e9259a20a57f9b350b`.
Vanilla `/usr/local/bin/typst`, upstream ratificado `a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Arrays vazio/heterogêneo e alias multilinha medem mensagem e span divergentes:
baseline nomeia o field arbitrário e sublinha acesso integral; vanilla publica
`cannot access fields on type array` e só o field. Length preserva dívida de
âncora total. `(13,27).first` produz 13 no baseline mas erro no vanilla;
`().first` produz erro anterior ao lookup. Métodos first()/len() e array.len
coincidem como `[2,13,2]`. Type::Array mede dívida própria fora do recorte.

Fonte baseline `01_core/src/compiler/eval/bindings/field_access.rs:468–481`
faz pré-despacho; `:670–679` preserva lookup puro len/first/last e diagnóstico
ausente. O lookup puro first lê arr.first(), enquanto a AST pode retornar
antes de chegar ali. Portanto a CLI first não comprova sozinha esse helper:
a obrigação do L0 exige teste puro independente para a terceira família.

## Quatro famílias congeladas

| Família | Mutação produtiva somente na cópia após C | Discriminante necessário |
|---|---|---|
| M1 | Interceptar Array ausente (excluindo len/first/last) com mensagem errada `array field access forbidden`, mantendo span | lookup puro e AST de fields arbitrários exigem mensagem exata do L0 |
| M2 | Retirar Array da seleção AST field-only | AST array ausente, aliases/Unicode/multilinha exigem todos e somente os bytes do field |
| M3 | Trocar exclusivamente first do lookup puro para arr.last(), mantendo clone/None e pré-despacho | array puro populado com primeiro/último diferentes deve devolver o primeiro |
| M4 | Acrescentar Length à seleção AST field-only | AST Length ausente deve preservar span integral baseline; não contar aproximação ao vanilla como sucesso |

São quatro alterações independentes de linguagem observável; enum/estrutura
do patch é mecanismo. M3 testa preservação explícita e M4 extrapolação indevida.
Antes do GO será conferido que o filtro realmente executa os quatro
discriminantes no módulo congelado do autor independente, sem escrever testes.
Qualquer lacuna de observabilidade, compilação inválida ou identidade incerta
refuta a suficiência e bloqueia; não contam como kill ou paridade.

## Instrumentação, autoridade e custo

Runner novo `p1338-attacks-runner.py` reutiliza P1337 sem alterar histórico,
com famílias/filtro/identidades P1338. Não executa preparação antes de C; os
patches concretos só são produzidos após plano e testes congelados e janela
autorizada pelo operador. Comando fixado igualmente para controle C/mutantes:

```
cargo test --release --config profile.release.package.typst-core.opt-level=0 --locked --offline -p typst-core --lib p1338_tests -- --nocapture
```

`CARGO_BUILD_JOBS=2`, target exclusivo temporário. Somente typst-core usa opt0;
dependências release. Cargo.toml e implementação de main não mudam. C pareado
deve passar antes de qualquer kill válido. Esta prova é instrumental; release
normal e gates gerais pertencem ao operador e não são substituídos.

Budget: controle C + quatro famílias produtivas, timeout 2700 s por Cargo,
30 s por CLI. Registrar UTC/segundos/processos por rodada e delta de cada
revisão; duas revisões sem ganho na mesma causa param o método. Unknown
obrigatório bloqueia. Não se prevê repetir corpus completo por alteração
meramente documental ou enquanto houver falha focal sem hipótese nova.

Leituras: skill/referências, CLAUDE/ADRs, L0 completo, manifestos/baseline,
binários e scripts antecedentes; testes P1338 somente congelados; C somente
após freeze deste plano. Escritas: `p1338-attacks-*` e cópia/target exclusivos
em /tmp. Sem leitura/listagem de materialization/context; sem escrever main,
testes/oráculos alheios ou histórico. As capacidades técnicas são maiores que
essa allowlist declarada; não existe atestação de isolamento.

Copiar workspace e cache estável P1337 sem hardlinks mutáveis. Cada rodada
aplica fonte via apply_patch e timestamp fresco, exige `Compiling typst-core`
do caminho real da cópia, compilação normal concluída e teste discriminante
falhando semanticamente. Guardar fonte, patch, canais integrais e executável
real por rodada antes do próximo link; hashes/recibos individuais imutáveis.
Falha de compilação ou cache antigo nunca contam. Agregado final só ao término
das rodadas ou parada inconclusiva explícita; reviewer julga independentemente.
