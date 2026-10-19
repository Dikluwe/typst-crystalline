# P1496 — materialização do catálogo pinado de símbolos

## Proveniência e estado da medição

- HEAD de partida: `0d722061c6f285501e3614bef823b4f16bcb13c5`
  (P1495). A materialização P1496 foi medida sobre **working tree não
  commitada**, não sobre esse commit isolado.
- Oráculo: upstream/main
  `9f2b6e8715237cb086899a42873660fe744622e8`, binário
  `lab/typst-original/target/release/typst`, SHA-256
  `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
  A sonda P1495 verifica esse SHA antes e depois de cada execução.
- Catálogo: `codex` git
  `2e43ab90079cf9f9fa670d421c8c38233143b376`, pin completo em
  `Cargo.toml` e source correspondente em `Cargo.lock`. O lockfile troca
  somente a entrada `codex` de crates.io para git e acrescenta sua
  dependência `icu_properties`; não copia o lockfile do vanilla.
  `sym.txt` da revisão tem SHA-256
  `60c79540af8ebe590c80d672b55a41f337bc8d61ebd42d9be8879fce9b18f8bc`.
  `emoji.txt` continua byte-idêntico ao snapshot anterior, SHA-256
  `8691ca68e09b6fedca61e00e824648e79f7502772eefe7af367e404e26161489`.
- RED anterior ao código: relatório P1495, HEAD
  `3d61023e46475cd6e5df52021164523c84e9f074`, execução final
  2026-09-25 00:42:42–00:44:05 UTC em working tree documentada ali;
  binário cristalino SHA-256
  `b692397823a0e85c9c945feda329fb9687eef0cd6f30e91fc77caf99b04f6394`.
  Resultado: `198` casos, `162 RED`, `36 EXTENSION`.
- Medição intermediária P1496: por volta de **2026-09-25 01:19 UTC**
  (segundo exato não registrado), HEAD P1495 acima + working tree
  não commitada. O `git diff HEAD --stat` registrado para essa execução
  foi **9 arquivos tracked, 516 inserções, 40 deleções**. Os nove arquivos
  alterados eram os dois L0s `sym.md` e `field_access.md`, os cinco Rust
  `sym.rs`, `structural/math.rs`, `emoji.rs`, `field_access.rs`,
  `eval/tests.rs`, `Cargo.toml` e `Cargo.lock`. Binário cristalino
  `target/release/typst`, SHA-256
  `de3bd1ecb0e382519c1ddebcea09cf542d622434489b2ce38c57b3cc370919dd`.
  O horário aproximado não prova ausência de edições entre build e sonda.

## Sondas de linguagem antes da decisão de fechamento

O mesmo harness bilateral `lab/parity/tools/p1495_symbols_probe.py` conserva
o universo de 198 casos. `EXTENSION` significa que o alias é aceito pelo
cristalino com glifo, `repr` e política histórica de warning estáveis,
enquanto o vanilla novo o rejeita; não é crédito de paridade.
`TEXT_MATCH_CARRIER_DIFF` significa glifo e warnings bare convergentes,
mas `eval(..., mode: "math").body` ainda expõe carrier `text` no cristalino
e `symbol` no vanilla. Não renomear essa classe para `MATCH`.

| Estado | MATCH | EXTENSION | TEXT_MATCH_CARRIER_DIFF | RED | Total |
|---|---:|---:|---:|---:|---:|
| Antes de P1496 (P1495) | 0 | 36 | 0 | 162 | 198 |
| Release intermediária P1496 | 146 | 36 | 13 | 3 | 198 |
| Release P1496 (198 casos) | 146 | 36 | 16 | 0 | 198 |

Na medição intermediária, todas as 146 consultas qualificadas de `sym` e
`math` convergiram. Os 16 casos bare dividiram-se em 13 convergências
de glifo/warning com carrier diferente e três `RED`: `chess.king.white`,
`keyboard.tab.r` e `ordinal.feminine`. A causa era o lookup de módulos
filhos de `sym` no modo math. O ramo de `eval/math.rs` que consulta o
`math` padrão corrigiu esses três casos, inclusive com `math` lexical
sombreado. O alvo de 146 `MATCH`, 36 `EXTENSION`, 16
`TEXT_MATCH_CARRIER_DIFF` e zero `RED` foi atingido na release seguinte,
sem `BASELINE_DEFECT` ou classe inesperada. A divergência do carrier bare
é dívida separada, não mudança autorizada por P1496.

## Materialização e controles focais

`sym.rs` passa a usar a revisão git pinada do `codex::SYM`, incluindo
`chess`, `keyboard` e `ordinal`, seus 21 filhos por namespace e 19
variantes nomeadas, além das novas entradas e dos cinco valores
`tack.*.double`. `structural/math.rs` espelha os bindings de `sym`
sem substituir as nativas próprias de `math`, como `sqrt`. Os testes
P1496 em `sym.rs`, `structural/math.rs`, `emoji.rs` e `eval/tests.rs`
foram introduzidos para o ciclo RED→GREEN: catálogo recursivo,
valores Unicode com VS, famílias de metadados, aliases legados,
warnings, homônimos e controles positivos/negativos. A suíte focal
P1496 passou com 16/16 testes GREEN na working tree não commitada.

Os doze paths legados — `join{,.r,.l,.l.r}` e
`{gt,lt}.tri{,.eq,.eq.not,.not}` — continuam extensões deliberadas
nos três acessos. O overlay seletivo injeta `join` como binding de topo;
`gt` e `lt` permanecem exatamente os grupos do codex novo, sem `tri*`
na sua lista de variantes nem no seu `repr`. Acesso a `.tri` sobre o
símbolo canônico recorre apenas ao path de compatibilidade, inclusive
permutação de modifiers. Essa separação evita esconder divergência de
`repr(gt/lt)` por mesclagem de grupos antigos. O fallback manual geral
não fica aberto para outros nomes removidos.

O warning histórico de `join` permanece em `sym` e bare, não em
`math.join`; `spacebar` usa a depreciação do novo pin e deve avisar em
`sym`, `math` e bare. O owner de field access só deve avisar após
resolver um binding de símbolo do módulo padrão efetivo, não por nome
de módulo ou campo homônimo. `registered` continua extensão histórica;
`sym.sqrt` continua ausente e `math.sqrt` continua função própria.
`emoji.txt` idêntico e os testes de catálogo `emoji` são controle de
invariância, não um novo escopo de paridade.

## Medição de release e matriz

A release cristalina pós-implementação foi construída com
`CARGO_BUILD_JOBS=1 cargo build --release` (exit 0) e tem SHA-256
`3788909c5101272355ac40b15d57071449baff6096e078481630e5d5df19e919`.
A sonda `PYTHONDONTWRITEBYTECODE=1 python3
lab/parity/tools/p1495_symbols_probe.py` correu em
**2026-09-25 01:52:52–01:53:49 UTC**, exit 0. O oráculo foi o binário
pinado acima, com o mesmo SHA antes e depois. O HEAD era P1495
`0d722061c6f285501e3614bef823b4f16bcb13c5` + working tree
não commitada. `git status --short` antes/depois foi idêntico:
`Cargo.toml`, `Cargo.lock`, os cinco L0s `sym.md`, `emoji.md`, `eval/math.md`,
`eval/bindings.md`, `eval/bindings/field_access.md`, e os sete
Rust `sym.rs`, `emoji.rs`, `structural/math.rs`, `eval/math.rs`,
`eval/tests.rs`, `eval/bindings/mod.rs`,
`eval/bindings/field_access.rs`; este diagnóstico era o único
untracked. `git diff HEAD --stat` antes/depois: 14 tracked,
717 inserções e 68 deleções. O SHA do binário cristalino também
permaneceu idêntico.

Resultado completo: **146 MATCH, 36 EXTENSION, 16
TEXT_MATCH_CARRIER_DIFF, zero RED**, total 198, sem
`BASELINE_DEFECT` ou classe inesperada. Os três RED intermediários
(`chess.king.white`, `keyboard.tab.r`, `ordinal.feminine`) foram
eliminados pelo lookup dos submódulos do catálogo em modo math.
A classe carrier bare segue aberta, sem crédito de `MATCH`.

Os três casos de matriz foram executados com
`python3 lab/parity/matrix/runner.py --case ID` sobre a mesma release.
Antes do rebaseline, cada um observou `MATCH` bilateral, mas o runner
saiu 1 por `expected_state: VIOLATED`; a ordem comparada era idêntica.
Só depois dessa medição foram alterados `expected_state`,
`proveniencia_cristalino` e `nota`, preservando
`estado: UNMEASURED`. A reexecução de cada caso saiu 0 com
`expectation_met: true` e `order_check.identical: true`.

| Caso | UTC da medição anterior ao rebaseline | Valor JSON observado nos dois binários |
|---|---|---|
| P1489-B-002 | 2026-09-25 01:54:34 | `["string","<module chess>"]` |
| P1489-B-003 | 2026-09-25 01:54:18 | `["string","symbol(\"⊩\", (\"not\", \"⊮\"))"]` |
| P1489-B-004 | 2026-09-25 01:54:22 | `["string","symbol(\"⌙\")"]` |

`python3 lab/parity/matrix/runner.py --validate-only` validou
os 43 casos do manifesto. O rebaseline é desses três casos,
não do universo de paridade inteiro.

## Controles e gates

Em **2026-09-25 01:56:44–01:58:51 UTC**, a mesma release e o
mesmo vanilla passaram controles bilaterais adicionais. O HEAD continuou
P1495; `git status --short` e `git diff HEAD --stat` foram
idênticos antes/depois: 15 tracked, 720 inserções, 71 deleções,
este diagnóstico untracked; os dois SHA-256 de binários mantiveram-se.
`emoji.heart`, `emoji.arrow`, `emoji.dancing.ballet` tiveram
`repr` igual; `math.sqrt` continuou função e `sym.sqrt` continuou
erro nos dois; `registered` só existe no cristalino e não recebeu
crédito de paridade. `spacebar` em `sym`/`math` e bare conservou
valor ␣ e um aviso (bare ainda com carrier diferente), inclusive
na rota `std.sym`/`std.math` em modo math. Binding local
`spacebar = 42` não avisou; módulos locais e `math` lexical
sombreado preservaram resolução positiva.

Após resselo dos cinco owners L0 afetados, a suíte focal P1496 passou
16/16; `CARGO_BUILD_JOBS=1 cargo test -p typst-core --lib --quiet
-- --test-threads=4` passou 6190 testes, zero falhas;
`CARGO_BUILD_JOBS=1 cargo build` e o build release passaram;
`crystalline-lint .` saiu 0 (avisos V16–V20 não bloqueantes),
V15/V26 teve zero violações; `git diff --check` saiu 0.
Esses gates foram medidos em HEAD P1495 + working tree não commitada
de 14 tracked, 717 inserções e 68 deleções (antes da atualização
documental da matriz), sem mudança de código entre eles e a sonda.
A checagem adicional `cargo fmt --all -- --check` segue falhando por
divergências anteriores ao P1496 em vários arquivos. Os blocos novos
do passo foram formatados focalmente; o check ainda aponta apenas
blocos baseline nos arquivos Rust tocados, sem reformatação massiva.

## Ratificação final da árvore a commitar

Após a formatação focal das linhas novas, a release foi reconstruída com
`CARGO_BUILD_JOBS=1 cargo build --release --quiet` (exit 0). SHA-256
cristalino definitivo:
`2620ffb09b9f1ed2f51bdb412cd883cc1bf944c5f0a6a7433926eee7146412e3`;
o vanilla permaneceu em
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O HEAD continuou
`0d722061c6f285501e3614bef823b4f16bcb13c5` + working tree
não commitada. Antes/depois da sonda final, `git status --short`
e `git diff HEAD --stat` foram idênticos: 16 tracked, 761 inserções,
72 deleções, mais este diagnóstico untracked. Os 16 tracked eram
`Cargo.toml`, `Cargo.lock`, `lab/parity/matrix/manifest.yaml`,
`00_nucleo/materialization/typst-passo-1496.md`, os cinco L0s
`compiler/stdlib/{sym,emoji}.md`, `compiler/eval/math.md`,
`compiler/eval/bindings.md`, `compiler/eval/bindings/field_access.md`,
e os sete Rust `compiler/stdlib/{sym,emoji,structural/math}.rs`,
`compiler/eval/{math,tests,bindings/mod,bindings/field_access}.rs`
(sob `01_core/src/`). Os hashes dos dois binários também ficaram
idênticos antes/depois.

`PYTHONDONTWRITEBYTECODE=1 python3
lab/parity/tools/p1495_symbols_probe.py` rodou em
**2026-09-25 02:20:46–02:21:57 UTC**, exit 0: 146 `MATCH`,
36 `EXTENSION`, 16 `TEXT_MATCH_CARRIER_DIFF`, zero `RED`,
total 198, `first_red: []`. Os três runners da matriz foram
reexecutados individualmente sobre os mesmos binários em
**2026-09-25 02:22:10.865–02:22:13.074 UTC**. B-002,
B-003 e B-004 tiveram exit 0, `estado=MATCH`,
`expected_state=MATCH`, `expectation_met=true`,
`order_check.identical=true`, ambos os CLI exit 0 e outputs
JSON bilaterais da tabela acima. Durante a matriz, status, diff stat
e hashes permaneceram iguais. Depois dessa prova, somente a
proveniência desses três casos foi atualizada para o SHA definitivo.

Na árvore formatada, `CARGO_BUILD_JOBS=1 cargo test -p typst-core
--lib --quiet -- --test-threads=4` voltou a passar **6190/6190**;
`CARGO_BUILD_JOBS=1 cargo build --quiet` saiu 0;
`crystalline-lint --quiet .` saiu 0; V15/V26 não acharam violações;
`crystalline-lint --fix-hashes .` retornou `Nothing to fix`.
O check global de rustfmt permanece não limpo pelo baseline
descrito acima; não foram introduzidos blocos novos pendentes
nos arquivos P1496.

## Dívida separada de captura em closure

Uma sonda fora dos 198 casos revelou aviso ausente para alias do
módulo canônico capturado em closure depois de `std` lexical sombreado.
O valor ␣ converge; no vanilla há um aviso `spacebar`, no cristalino
desta release não. Reprodutor bilateral:
`typst eval 'repr(eval("#let alias = sym\n#let std = (sym: none)\n#let f = () => alias.spacebar\n#f()", mode: "markup"))' --format json`.
O mesmo ocorre com alias de `math`. Um teste experimental confirmou
RED (zero avisos em vez de um). A tentativa de consultar
`engine.world.library().global` foi revertida: a avaliação
constrói uma `Library` própria, diferente da do `World`, portanto
não fornece a identidade Arc do módulo efetivo. O teste RED
experimental também foi retirado; nenhum código quebrado ou teste
falhando integra P1496. Corrigir a captura dessa identidade exige
trabalho no carrier de closure/`Scopes`, além do lookup focal de
P1496, e fica como dívida explícita para passo próprio.

## Decisão de escopo

P1496 materializa o catálogo pinado e fecha apenas as 146 sondas
qualificadas e os três casos rebaselined da matriz. As 36 extensões
preservadas não são paridade; os 16 carriers bare diferentes e a
divergência de warning em closure permanecem abertos. Não inferir
paridade global de símbolos nem fechamento dessas dívidas.
