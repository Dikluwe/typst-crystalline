# P1485 — sondas de `asset` e exportação bundle

**Estado:** medição pré-L0; nenhum código P1485 escrito.
**Proveniência:** 2026-09-23T21:43:17-03:00, HEAD
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, working tree não
commitada. Binário vanilla ratificado `/usr/local/bin/typst` SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
cristalino pré-P1485 `target/debug/typst` SHA-256
`459e434d51e97039491134203b64b8ccd2dbbdb227706f142e54942b92874607`.
Fonte vanilla: `lab/typst-original` pin `586e1bd43`;
`crates/typst-bundle/src/format.rs` SHA-256
`d7ef66ed1ec04e58fcb223cca3b3779a15778eaac5c0ee6488b1969f62ae43ed`;
`src/lib.rs` do mesmo crate SHA-256
`2ea4e3431f1ec8a62dcd4299b95c8cb3b8317eff12bc5f7df53aa590ab36a27b`.

Arquivos rastreados alterados no instante das sondas
(`git diff HEAD --stat`: 15 arquivos, 361 inserções, 53 remoções):
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`;
sob `00_nucleo/prompts/`:
`compiler/eval.md`, `compiler/eval/bindings/field_access.md`,
`compiler/eval/tests.md`, `compiler/stdlib/structural/document.md`,
`entities/compiler_features.md`, `infra/integration_tests.md`,
`shell/cli.md`;
`01_core/src/compiler/eval/bindings/field_access.rs`,
`01_core/src/compiler/eval/mod.rs`, `01_core/src/compiler/eval/tests.rs`,
`01_core/src/compiler/stdlib/structural/document.rs`,
`01_core/src/entities/compiler_features.rs`, `02_shell/src/cli.rs`,
`03_infra/src/integration_tests.rs`. Não rastreados já existentes:
`typst-p1484-matriz-bundle.md`, `typst-passo-1484.md` e rascunhos
`typst-passo-1485.md`–`typst-passo-1488.md`. Nenhum deles foi alterado
pelas sondas; fixtures temporários ficam em `/tmp/typst-p1485-probes.jhg6Ar`.

## Constructor e superfícies de linguagem

Comando-base: `/usr/local/bin/typst eval 'EXPR' --format json
--features bundle`. `type(format.bundle)` sem feature emite gate de
field `bundle`; com feature, `repr((type(format.bundle),
format.bundle, type(format.bundle.asset), format.bundle.asset,
type(asset)))` é `"(function, bundle, function, asset, function)"`.
`asset == format.bundle.asset` e `std.asset == asset` são `true`.
`format.bundle()` dá `cannot be constructed manually`.

`repr(asset("a.txt", "hello"))` é
`asset(path: "/a.txt", data: bytes(5))`. `asset("x", bytes((0,255)))`
conserva bytes; path e data são posicionais obrigatórios. Usar
`path:` ou `data:` causa `the argument ... is positional` com hint;
faltar data causa `missing argument: data`; `kind:` após os dois
posicionais causa `unexpected argument: kind`; inteiro/none/array em
data causa `expected string or bytes, found ...`, inteiro em path
`expected string, found integer`, terceiro posicional
`unexpected argument`.

`a/../b` normaliza para `/b`; `a//b` para `/a/b`; `/a` mantém
`/a`. `""`, `"."` e `"/"` falham com
`path must have at least one component`; `"../a"` e
`"a/../../b"` com `path ... would escape the bundle root`;
backslash é proibida com hint de barra normal. Estes são diagnósticos
observáveis; `VirtualPath` e o tipo Rust `BundlePath` são mecânica
da fonte, não contrato de representação cristalina.

## Exportação bilateral e limites

Fixtures temporários `.typ` e comando-base
`/usr/local/bin/typst compile INPUT.typ OUTPUT --format bundle
--features bundle`:

| Fixture (fonte literal) | Resultado medido |
|---|---|
| `#asset("assets/raw.bin", bytes((0, 255, 10)))` | `OUTPUT/assets/raw.bin` com bytes decimais `0 255 10` (SHA-256 `712450d3c4a79eea9509e75dc1dacdeff58034df538536cfae2da882bd8a0c50`) |
| `Hello` | erro `text is not allowed at the top-level in bundle export`, hint ``try wrapping the content in a `document` instead`` |
| `#asset("x.txt","one")` + `#asset("./x.txt","two")` | erro ``path `x.txt` occurs multiple times in the bundle``, hint de unicidade e localização original |
| `#document("a.pdf")[Hello]` + `#document("b.svg")[Hello]` | emite `a.pdf` e `b.svg` |
| `#document("index.pdf")[Hello]` + `#asset("style.css","body{}")` | emite os dois arquivos; `style.css` tem 6 bytes literais |
| `#document("index.html")[Hello]` | só emite `index.html` quando ambas `bundle,html` estão ativas; sem `html`, diagnóstico com hints específicos |
| `#document("x.weird")[Hello]` | erro `unknown document format` + hint para `format` explícito |
| `#asset("../outside","x")` | erro de escape do bundle root antes de I/O |

Todo compile bundle com feature emite warning experimental com headline
`bundle export is experimental` e dois hints. Sem feature, o target
falha antes de eval com
``bundle export is only available when `--features bundle` is passed``
e hint. `#asset(...)` sob `--format pdf --features bundle` falha com
`assets are only supported in the bundle target` e hint para ativá-lo.

O CLI upstream `compile.rs:114-144` não infere `bundle` da extensão de
output: exige `--format bundle`; sem output explícito, deriva diretório
retirando extensão do input. `compile.rs:420-472` escreve o virtual FS
em diretório, cria pais intermediários e rejeita stdout. A fonte
`typst-bundle/src/lib.rs:205-295` exige top-level apenas
`document`, `asset` ou tags, e detecta duplicata após normalização.
`model/document.rs:147-239` infere PDF/SVG/PNG/HTML da extensão ou
usa `format` explícito; PNG/SVG exigem uma página. O watch upstream
aceita `--format bundle` e serve com HTTP por padrão, salvo
`--no-serve` (`typst-cli/src/compile.rs:209-220` e help observado);
O watcher foi executado na sonda complementar abaixo; a paridade
cristalina de HTTP/live reload continua aberta.

## Decisão que a medição suporta

A paridade é com a linguagem e os arquivos observáveis. Igualdade
interna de structs, algoritmo, ordem de paralelismo e bytes de PDF/SVG
não é exigida, salvo bytes de asset. A implementação não pode declarar
MATCH por apenas expor `format.bundle`/`asset`; precisa de target
multi-arquivo, `document`, validação de paths e pelo menos os perfis
medidos. P1485 introduz novo target, contrato público e potencial fase
de pipeline: gate ADR-0127 obrigatório antes de L1–L4.
O dono incluiu HTTP/live reload no escopo de P1485; a sonda runtime abaixo
complementa a fonte. O código cristalino continua sujeito ao gate ADR-0127.

## Sonda complementar — servidor HTTP de watch

Proveniência: 2026-09-23T22:03:53–22:05:15-03:00, HEAD
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, working tree não
commitada (`git diff HEAD --stat`: 25 arquivos, 745 inserções, 61
remoções). Mesmo binário vanilla SHA-256 acima. Fixture literal
`#document("index.html")[Hello]`; comando `typst watch INPUT OUTPUT
--format bundle --features bundle,html --no-fullscreen --port 39761`.
Processos encerrados por SIGINT após as sondas.

`curl -i http://127.0.0.1:39761/index.html` e `/` retornaram 200
com `Content-Type: text/html` e script `EventSource("/__events")`
injetado. Com `--no-reload`, o HTML continuou 200 sem esse script;
o arquivo em disco não recebeu script em nenhum caso. Com
`--no-serve --port 39761`, o watcher compilou e a conexão a
`127.0.0.1:39761` foi recusada (`curl` exit 7); processo
encerrado por SIGINT. A fonte pinada
`typst-kit/src/server.rs:28-67,117-181,219-248,381-390` confirma
bind loopback, portas 3000–3005 por omissão, lookup de index, redirect,
404, SSE e notificação ao republicar. `typst-cli/src/args.rs:525-543`
confirma `--no-serve`, `--no-reload`, `--port`; `compile.rs:209-220,442-444`
confirma servidor somente em watch HTML/bundle. Alteração de fonte com
cliente SSE conectado não foi sondada: permanece obrigação RED→GREEN.
