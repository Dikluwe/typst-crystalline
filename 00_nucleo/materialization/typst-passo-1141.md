# P1141 — auditoria e nucleação do tipo público `path`

**Data:** 2026-08-24  
**Estado:** `CONCLUÍDO — gate ADR-0127 confirmado pelo dono em 2026-08-24`  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Escopo desta execução:** medir, inventariar consumidores, atualizar L0 e parar antes de código.

## 1. Proveniência da medição

- HEAD cristalino: `a3e72f35b3fc0f5cd765d0dc0ca89990ff1314a5`, branch `Tekt`.
- Estado inicial às `2026-08-24T18:28:09-03:00`: working tree não commitado;
  `git diff HEAD --stat` sem saída e uma entrada untracked já pertencente ao dono:
  `00_nucleo/diagnosticos/typst-auditoria-handoff-pos-p1140.md`.
- Oráculo executável: `/usr/local/bin/typst` e
  `lab/typst-original/target/release/typst`; ambos imprimem
  `typst 0.15.1 (e0e8ca4d)`, mas a proveniência normativa é o hash pinado
  `a51e02804`, não essa string de versão.
- Fonte ratificada: `lab/typst-original/crates/typst-library/src/foundations/path.rs`
  e `lab/typst-original/crates/typst-syntax/src/path.rs`, conferidas com o objeto
  Git `a51e02804` presente no repositório.
- Sondas desta auditoria: ficheiros temporários sob `temp/p1141/`, removidos
  após execução; foram criados depois da fotografia inicial e não integram
  produto nem prova de código cristalino. A fonte essencial está transcrita
  abaixo para reprodução.

## 2. Fase A — medição antes da decisão

### 2.1 Contrato na fonte vanilla

- `foundations/path.rs:134-166`: `path` é tipo público e constructor; recebe
  `path | str`, resolve pelo `FileId` do span e devolve o mesmo valor se já for
  `path`.
- `foundations/path.rs:168-174`: `repr` usa sempre path virtual absoluto com
  barra inicial; a identidade de pacote não é exposta textualmente.
- `foundations/path.rs:181-224`: `PathOrStr` conserva um `RootedPath` já
  resolvido ou resolve uma string relativamente ao ficheiro chamador.
- `foundations/path.rs:226-250`: escape da raiz e barra invertida são erros
  observáveis com hints próprios.
- `typst-syntax/src/path.rs:16-60`: `RootedPath` é a composição pura de
  `VirtualRoot` e `VirtualPath`.
- `typst-syntax/src/path.rs:72-81`: a raiz é `Project` ou `Package(PackageSpec)`.
- `typst-syntax/src/path.rs:189-357`: normalização de `.`, `..`, slash e join é
  lexical e não realiza I/O.
- `foundations/path.rs:9-133`: a documentação declara a intenção pública:
  resolver no ponto de construção para que operações posteriores se mantenham
  estáveis ao atravessar ficheiros e pacotes.

### 2.2 Sondas observáveis no vanilla ratificado

`typst query temp/p1141/main.typ '<probe>' --field value --one` produziu:

```typ
#let p = path("data/./nested/../file.txt")
#metadata((type-path: type(path), type-value: type(p),
  repr-relative: repr(p), repr-absolute: repr(path("/assets/../logo.svg")),
  dot-equal: path("./x") == path("x"),
  parent-equal: path("a/b/../c") == path("a/c"),
  passthrough: path(p) == p, distinct: path("a") == path("b"))) <probe>
```

O resultado foi:

```json
{"type-path":"type","type-value":"path","repr-relative":"path(\"/data/file.txt\")","repr-absolute":"path(\"/logo.svg\")","dot-equal":true,"parent-equal":true,"passthrough":true,"distinct":false}
```

Logo, no nível da língua: `path` é um tipo chamável; o valor tem tipo `path`;
`.`/`..` normalizam antes da igualdade; `path(path)` é identidade; `repr`
mostra a forma absoluta normalizada.

As sondas negativas mediram, verbatim no observável relevante:

```text
path("a\\b")       -> error: path must not contain a backslash
path("../secret") -> error: path `"../secret"` would escape the project root
```

com hints para slash portátil e sandbox/`--root`. A sonda cross-file
`temp/p1141/cross/main.typ` produziu:

```json
{"helper-repr":"path(\"/sub/local.txt\")","caller-repr":"path(\"/caller.txt\")","helper-read":"caller-root\n"}
```

Isto mede que a raiz/base fica presa no constructor: um path criado no chamador
continua a apontar para o ficheiro do chamador quando consumido num módulo filho.
A fonte foi `main.typ`: `#import "sub/helper.typ": make-local, consume` seguido
de `caller-path = path("caller.txt")`; e `sub/helper.typ` definiu
`make-local() = path("local.txt")` e `consume(p) = read(p)`. Os fixtures
continham respectivamente `caller-root` e `helper-local`.

### 2.3 Estado cristalino e consumidores

- `entities/value.rs:30-212`: não há `Value::Path` nem `Type::Path`.
- `compiler/eval/mod.rs:1634-1660`: o scope global regista loaders e `image`,
  mas não `path`.
- `entities/file_id.rs` e seu L0 mantêm `FileId` opaco; L1 não consegue derivar
  raiz e caminho virtual de um handle sem perguntar ao `World`.
- `contracts/world.rs:53-69`: `read_bytes` e `include_source` recebem
  `(current_file, &str)`, portanto re-resolvem sempre no consumidor.
- `03_infra/src/world.rs:493-531`: L3 já distingue raiz de projeto/pacote e
  resolve absoluto/relativo, mas devolve apenas `PathBuf`; essa identidade ainda
  não é um valor de linguagem.
- Consumidores string-only encontrados: `read`, `csv`, `json`, `yaml`, `toml`,
  `cbor`, `xml` (`stdlib/loading.rs`); `image` (`stdlib/figure_image.rs`);
  `plugin` (`stdlib/plugin.rs`); bibliografia (`eval/bibliography.rs` e
  `stdlib/structural/bibliography.rs`); `include` (`eval/modules.rs`). Imports
  de ficheiro são sintaticamente especiais e permanecem string/pacote/módulo;
  a fonte vanilla `typst-eval/src/import.rs` também constrói `PathOrStr::Str`.
- `entities/geometry::PathItem` representa geometria vetorial. A homonímia não
  autoriza reuso: domínio, igualdade e consumidores são diferentes.

### 2.4 Classificação ADR-0107/0108

Tipo, constructor, `repr`, igualdade normalizada, erros de sandbox e retenção da
raiz ao atravessar módulos são semântica/sintaxe observável da língua. Interner,
`PathBuf`, bytes e algoritmo de normalização são mecânica livre. É inferência
arquitetural — e seria refutada por uma fonte ratificada que mostrasse I/O no
tipo ou re-resolução de `RootedPath` no consumidor — que o valor deve ser um DTO
puro L1 e a materialização física deve permanecer no `World` L3.

## 3. Decisão L0 nuclearizada

Os L0s atualizados por este passo são:

1. novo `00_nucleo/prompts/entities/path.md` — domínio `VirtualRoot`,
   `VirtualPath`, `RootedPath` e `PathOrStr`, normalização, igualdade e `repr`;
2. `entities/value.md` — `Value::Path`, `Type::Path`, `type_of`, nome e
   chamabilidade;
3. `entities/file-id.md` — retira a decisão antiga de empurrar o domínio path
   para L3; só o interner/lookup físico permanece fora do handle;
4. `contracts/world.md` — resolução contextual explícita e I/O por
   `RootedPath`, preservando a raiz já capturada;
5. `compiler/eval.md` — binding/constructor contextual e `repr`;
6. L0s dos consumidores (`stdlib/loading`, `stdlib/figure_image`,
   `stdlib/plugin`, `stdlib/structural/bibliography`) — aceitam `path | str`
   sem duplicar normalização.

Imports de ficheiro permanecem fora da ampliação de valor; `include` aceita
`path | str` porque sua fonte é uma expressão. `asset`, extensão cristalina sem
I/O, não é promovido automaticamente: exige decisão própria sobre se seu campo
é path de linguagem ou mero identificador textual.

## 4. Plano RED→GREEN após o gate

Somente após confirmação do dono:

1. testes RED do domínio lexical e do valor público;
2. testes RED de constructor/`repr`/igualdade e erros;
3. testes RED cross-file e project/package root;
4. alteração dos contratos públicos do `World` e mocks;
5. adaptação de L3 para resolver/materializar `RootedPath` sem escape;
6. migração atomizada dos consumidores listados;
7. resselo de todos os `@prompt-hash` afetados;
8. `cargo build && crystalline-lint .`, rebaseline e sondas GREEN.

Aceitação é pela língua: o mesmo path resolve o mesmo recurso após atravessar
ficheiros/pacotes, `repr`/igualdade/erros coincidem semanticamente. Não se exige
estrutura Rust, interner, `PathBuf` ou bytes idênticos ao vanilla.

## 5. Gate ADR-0127

Este lote adiciona `Value::Path`, `Type::Path`, tipos públicos de domínio e
altera assinaturas do trait público `World`. Incide inequivocamente nos pontos
1 e 4 do ADR-0127 (contrato público e compatibilidade).

**PARAR AQUI. Não escrever testes nem código L1–L4, não executar
`crystalline-lint --fix-hashes` e não ressellar headers antes de o dono auditar
os L0s, confirmar o gate e autorizar a continuação.**

## 6. Execução após confirmação do gate

O dono respondeu `Prossiga` em 2026-08-24. O gate foi aberto e o lote foi
executado em RED→GREEN:

- RED transversal: `cargo check -p typst-core` detectou quatro matches fechados
  que ainda não cobriam `Value::Path` (`error_formatting`, `from_tags`,
  `transforms`, `state`); cada owner recebeu braço explícito.
- GREEN domínio: `cargo test -p typst-core p1141_ --lib` — 3 testes, 3 passam.
- GREEN E2E: `cargo test -p typst-infra p1141_path --lib` — 2 testes, 2 passam,
  incluindo preservação da base ao atravessar módulo.
- Regressão de leitura: `read_texto_utf8_pipeline` passa.
- `cargo check --workspace` passa.
- `cargo build --workspace` passa.
- `crystalline-lint --fix-hashes .` ressellou 19 headers; nova análise sem
  drift.
- `crystalline-lint .` termina com exit 0 e zero violations; permanecem apenas
  warnings/info históricos fora do gate de violações.
- `git diff --check` sem saída.

Medição final às `2026-08-24T18:44:18-03:00`, HEAD
`a3e72f35b3fc0f5cd765d0dc0ca89990ff1314a5`, working tree não commitado. O
`git diff HEAD --stat` mediu 36 ficheiros rastreados, 496 inserções e 86
remoções; além deles permanecem untracked o handoff do dono, este passo, o novo
L0 e os dois novos módulos de código. A fotografia exata é recuperável pelo
status/diff desse instante registrado na sessão.
