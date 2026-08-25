# P1154 — fechamento consolidado de `path`, `location` e baseline

**Data:** 2026-08-25
**HEAD:** `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`
**Estado:** `FECHADO — typst-core e workspace GREEN após P1155`
**Vanilla ratificado:** `a51e02804`

## 1. Proveniência e escopo

Rebaseline final medido até `2026-08-25T07:21:44-03:00`, em working tree não
commitada. No início de P1154 havia 29 ficheiros rastreados alterados, 855
inserções e 182 remoções, mais os passos P1149–P1154 e o diagnóstico P1151 não
rastreados. P1154 acrescenta apenas este diagnóstico e atualiza seu próprio
passo; nenhum código, L0, contrato, default ou fase foi alterado.

As fontes históricas lidas foram o handoff explícito pós-P1140 e os passos
P1141–P1153. `00_nucleo/context/` e `00_nucleo/materialization/` não foram
listados nem lidos.

## 2. Linha temporal P1141–P1153

| Passo | Resultado | Classe ADR-0127 |
|---|---|---|
| P1141 | tipo público `path`, domínio virtual, `Value::Path` e fronteira `World` | gate público aprovado pelo dono antes do código |
| P1142 | `array.all` e `str.clusters` estáticos | correção de paridade em fluxo contínuo |
| P1143 | namespace tabelado de cores | correção de paridade em fluxo contínuo |
| P1144 | família pública de métodos de gradient | glue interno, sem gate público |
| P1145 | constructors/transformações de gradient e `anti_alias` | gate público aprovado |
| P1146 | accessors de datetime | gate aprovado antes da materialização |
| P1147 | scope de int e constructor | fluxo contínuo, sem ampliar entidade pública |
| P1148–P1149 | superfície e comportamento contextual de counter | gate de `CounterUpdate` aprovado; P1149 fechou defaults/callback |
| P1150 | superfície estática completa de content | glue sobre owner existente |
| P1151 | transporte exato de Location em conteúdo de query | gate de `Value::LocatedContent` aprovado |
| P1152 | migração dos mocks para `resolve_path`/`read_path` | test-only |
| P1153 | correção de 10 expectativas obsoletas | test-only, após sondas vanilla |

O Git mostra que P1141–P1148 já pertencem ao HEAD. A working tree atual contém
P1149–P1153 e seus documentos; portanto não se deve apresentar todo P1141–P1153
como um único diff ainda por commitar.

## 3. Contrato final de `path` e `World`

### Medição

P1141 mediu nos dois binários vanilla que `path` é tipo chamável, normaliza
`.`/`..`, usa repr virtual absoluto com `/`, rejeita barra invertida/escape e
preserva a raiz capturada ao atravessar módulos. A fonte ratificada separa o
DTO virtual da materialização física.

No cristalino atual:

- `entities/path.rs:12-104` define `VirtualRoot`, `VirtualPath`, `RootedPath` e
  `PathOrStr` em L1;
- `contracts/world.rs:50-61` separa `resolve_path` de `read_path`;
- `entities/value.rs:402` conserva `Value::Path` como `Type::Path`;
- loading aceita `Str | Path` em `stdlib/loading.rs:407` e `:445`;
- plugin aceita `Str | Path` em `stdlib/plugin.rs:62`;
- os consumers resolvem strings uma vez e leem por `RootedPath`; um path já
  enraizado preserva `VirtualRoot` e não é convertido novamente em string.

P1152 confirmou que as 30 falhas L1 de ficheiros não vinham desses consumers,
mas de mocks antigos que implementavam apenas `read_bytes`. Os worlds em
memória agora implementam a fronteira P1141 em loading (`:907-928`), plugin
(`:232-253`), eval/bibliography (`:369-390`), eval/tests e stdlib/tests. Não há
filesystem real nesses mocks.

### Decisão

O contrato final coincide com a língua medida: raiz e vpath são semântica;
`PathBuf`, interner e estratégia física de leitura são mecânica. A barra
inicial em `/logo.png` é a forma portátil canónica já exigida pelo L0 e pelos
testes L1 corrigidos em P1152.

## 4. Contrato final de `content.location()`

### Medição

P1151 mediu nos dois vanilla que duas headings com conteúdo igual são iguais
como conteúdo, mas carregam Locations distintas. Logo, lookup posterior por
igualdade perderia identidade e não pode reconstruir a localização correta.

### Cadeia implementada

```text
native_query
  -> Value::LocatedContent(Content, Location)
  -> field/método de content
  -> eval_content_method_at
  -> Value::Location ou none
```

- `stdlib/foundations/query.rs:45-64` conserva o par exato;
- `entities/value.rs:64-68` contém `LocatedContent`;
- `entities/value.rs:323` e `:378` mantêm nome/tipo públicos como `content`;
- `eval/bindings/field_access.rs:102`, `:325`, `:652-768` preserva a Location
  quando existe e devolve `none` para conteúdo inline;
- igualdade, repr, erros e métodos não relacionados desembrulham apenas o
  `Content`, preservando a morfologia da linguagem.

O gate ADR-0127 foi necessário porque a variante é contrato público Rust,
apesar de não criar novo tipo público Typst. O dono o aprovou antes da
materialização. P1154 não reabre essa decisão.

## 5. Fechamento das 40 falhas de `typst-core`

| Marco | Aprovados | Falhas | Lista causal |
|---|---:|---:|---|
| pós-P1151 | 5.183 | 40 | 30 I/O/harness + 1 int(float) + 3 P269 + 6 P273 |
| pós-P1152 | 5.213 | 10 | mocks I/O corrigidos; restaram exatamente os 10 testes semânticos |
| pós-P1153/P1154 | 5.223 | 0 | expectativas revalidadas e corrigidas após sondas vanilla |

As listas nominais de P1151–P1153 foram comparadas; nenhuma falha nova
substituiu outra. P1153 confirmou em ambos os vanilla: `int(3.7) == 3`, nomes
focais usam hífen e todos os gradients exigem dois stops. Também fortaleceu os
controles de focal-radius excessivo e relative inválido, que antes podiam
passar no estrato errado.

## 6. Matriz L0 → código → testes

`Hash L0` abaixo é o “Hash do Código” declarado no documento; `header` é o
`@prompt-hash` validado pelo linter. Eles têm funções diferentes e não devem
ser comparados como se fossem o mesmo digest.

| Eixo | L0 owner | Hash L0 / header | Código/sentinela | Gate/estado |
|---|---|---|---|---|
| path | `entities/path.md` | `26ff79df` / `c9a7b4f1` | `entities/path.rs:12-104`; testes P1141 | gate P1141 aprovado |
| World | `contracts/world.md` | `f4becd19` / `7efb7ad6` | `contracts/world.rs:50-61`; E2E P1141 | gate P1141 aprovado |
| loading | `stdlib/loading.md` | `27a4efb8` / `426f2a28` | `loading.rs:407,445,907-928`; P1152 | GREEN L1 |
| image | `stdlib/figure_image.md` | `27604782` / `6f29a5f2` | `figure_image.rs`; testes image P1152 | GREEN L1 |
| plugin | `stdlib/plugin.md` | `0d4aa4c7` / `905db97a` | `plugin.rs:62,232-253`; P1152 | GREEN L1 |
| bibliography | `structural/bibliography.md` | `2e904617` / `8184430e` | bibliography eval/stdlib; P1152 | GREEN L1 |
| Value/content | `entities/value.md` | `d01f6037` / `214626a3` | `value.rs:64-68,323,378`; P1151 | gate aprovado |
| query | `foundations/query.md` | `681adeae` / `b5ce4390` | `query.rs:45-64`; teste P1151 | GREEN |
| field access | `bindings/field_access.md` | `7303f0d6` / `2a3126bf` | `field_access.rs:102,325,652-768`; P1150/P1151 | GREEN |
| value methods | `bindings/value_methods.md` | `9433f2ee` / `a88fe70d` | `value_methods.rs`; P1150/P1151 | GREEN |
| call dispatch | `eval/call_dispatch.md` | `dc65d331` / `3796519a` | `call_dispatch.rs`; P1150/P1151 | GREEN |
| counter | `stdlib/counter.md` | `4d2d6a5b` / `8b1f067c` | `counter.rs:160`; P1148/P1149 | gate aprovado, GREEN |
| counter callbacks | `introspect/from_tags.md` | sem hash na abertura / `108c3320` | `from_tags.rs:96-135`; P1149 | GREEN |

`crystalline-lint .` terminou com exit status zero, sem V5 e sem violations.
Warnings V16–V20 e informações históricas permanecem; não foram resolvidos nem
são apresentados como parte deste fechamento.

## 7. Atribuição da working tree

Não há ficheiro rastreado alheio conhecido no estado atual, mas existem
sobreposições entre passos dentro dos mesmos owners. A atribuição segura é:

| Ficheiro(s) | Passo(s) | Classificação |
|---|---|---|
| L0s `counter.md`, `from_tags.md` | P1149 | lote P1149 |
| `introspect.rs`, `introspect/fixpoint.rs`, `introspect/from_tags.rs`, `stdlib/counter.rs` | P1149 | lote P1149 |
| L0s `field_access.md`, `value_methods.md`, `call_dispatch.md` | P1150 + P1151 | sobreposição; revisar hunks juntos |
| `bindings/field_access.rs`, `method_dispatch.rs`, `bindings/mod.rs`, `value_methods.rs`, `call_dispatch.rs` | P1150 + P1151 | sobreposição; não separar mecanicamente |
| L0s `entities/value.md`, `foundations/query.md` | P1151 | lote P1151 |
| `entities/value.rs`, `foundations/query.rs`, `eval/mod.rs`, equality/error/repr/state/transforms | P1151 | lote P1151 |
| `eval/tests.rs` | P1150/P1151 + mock P1152 | sobreposição intencional |
| `eval/bibliography.rs`, `stdlib/loading.rs`, `stdlib/plugin.rs` | P1152 | mocks test-only |
| `stdlib/mod.rs` | P1151 + P1152 + P1153 | sobreposição intencional; commit conjunto recomendado |
| passos P1149–P1154 e diagnóstico P1151/P1154 | respectivos passos | documentação do lote |

Os três ficheiros `/tmp/p1153-*.typ` são sondas externas temporárias e não
entram em commit. Nenhum ficheiro da working tree foi apagado, movido, stashed
ou reescrito por P1154.

## 8. Validação final e resíduo real

Passaram:

```text
cargo test -p typst-core --lib
  -> 5.223 passed; 0 failed; 0 ignored
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
  -> exit 0; zero violations; zero V5
```

`cargo test --workspace` não passou integralmente:

```text
typst-core:  5.223 passed; 0 failed
typst-infra:   836 passed; 1 failed
falha: integration_tests::integration::read_binario_pipeline
```

O teste em `03_infra/src/integration_tests.rs:1743-1760` espera
`... in logo.png:1:1`, enquanto a produção devolve a forma canónica P1141
`... in /logo.png:1:1`. O teste L1 equivalente já espera `/logo.png` em
`stdlib/loading.rs:970`. A primeira causa é portanto uma assertion L3
test-only não migrada por P1152, não um defeito de resolução nem mudança nova
de produção. O ficheiro L3 não está alterado na working tree.

## 9. Decisão de fechamento e empacotamento

O eixo `path`/`location` e o baseline integral de `typst-core` estão fechados.
O workspace, porém, não pode ser chamado GREEN enquanto a assertion L3 acima
permanecer. Deve existir um passo corretivo estreito que:

1. revalide a mensagem contra o vanilla/L0 vigente;
2. atualize somente a expectativa L3 para `/logo.png` se confirmada;
3. rode o teste isolado e `cargo test --workspace`;
4. atualize este diagnóstico com workspace GREEN.

Não criar commit antes desse controle. Depois dele, a working tree deve ser
revisada humanamente. Como vários owners têm hunks sobrepostos entre P1149,
P1150 e P1151–P1153, a recomendação segura é um commit coeso do lote ou
separação manual por hunks auditados; não uma divisão automática por ficheiro.

## 10. Atualização P1155 — workspace GREEN

P1155 executou o controle estreito em `2026-08-25T07:25:58-03:00`, no mesmo
HEAD e com a working tree não commitada preservada. O RED isolado reproduziu a
mensagem `/logo.png:1:1`. Foi alterada somente a assertion/comentário de
`read_binario_pipeline` em `03_infra/src/integration_tests.rs`; produção e L0
não mudaram.

Resultado após a correção:

```text
read_binario_pipeline: 1 passed; 0 failed
typst-infra:          837 passed; 0 failed; 0 ignored
typst-core:         5.223 passed; 0 failed; 0 ignored
cargo test --workspace: exit 0; todos os crates sem falhas
```

Também passaram `cargo check --workspace`, `cargo build --workspace`,
`cargo fmt --all -- --check`, `git diff --check` e `crystalline-lint .`. O
linter terminou com zero violations e zero V5; warnings históricos continuam
fora do scope.

O resíduo descrito na secção 8 permanece como fotografia reproduzível da
execução P1154, mas está resolvido. O fechamento integral está agora GREEN. O
próximo ato é revisão humana da working tree e decisão explícita de
empacotamento; nenhum commit foi criado.
