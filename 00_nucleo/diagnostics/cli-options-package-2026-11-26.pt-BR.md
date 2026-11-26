# Proposta de pacote: opções de CLI do vanilla

**26 de novembro de 2026 · aprovado pelo dono em 26/11/2026 (“Aprovo A–F”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) §1 (contrato de CLI)**

Primeiro pacote escolhido pela revisão de L2 (`02_shell`). O L2 tem `cli.rs` (4.372 linhas), `diagnostic.rs`, `fonts.rs`, `info.rs`, `completions.rs` e `duration.rs`.

## O problema

O Tekt recusa com `unexpected argument` opções que o vanilla aceita em `compile`, `watch` e `eval`. Um script ou editor que passe essas opções para o Typst falha no Tekt com código 2.

## Medição

HEAD `3cdb73482`, árvore não commitada com o pacote do PNG de várias páginas e a alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 19 arquivos; o pacote do PNG não toca `02_shell`). Cristalino debug compilado dessa árvore em `/dev/shm/build-main`. Vanilla `lab/typst-original/target/release/typst`. Sondas de 11:45 a 11:54 de Brasília; entradas em `lab/results/cli-options-package-2026-11-26/`, local e não versionado.

Opções de `--help` que só o vanilla lista:

| Comando | Só no vanilla |
|---|---|
| `compile`, `watch` | `--creation-timestamp`, `--deps`, `--deps-format`, `--diagnostic-format`, `--ignore-embedded-fonts`, `--ignore-system-fonts`, `--jobs`, `--open`, `--package-path`, `--package-cache-path`, `--timings`; `watch` também `--no-fullscreen` |
| `eval` | `--root`, `--input`, `--font-path`, `--target`, `--diagnostic-format`, `--ignore-embedded-fonts`, `--ignore-system-fonts`, `--creation-timestamp`, `--jobs`, `--package-path`, `--package-cache-path` |
| `fonts` | `--ignore-embedded-fonts` |

Variáveis de ambiente que só o vanilla lê: `TYPST_DIAGNOSTIC_FORMAT`, `TYPST_FEATURES`, `TYPST_IGNORE_EMBEDDED_FONTS`, `TYPST_IGNORE_SYSTEM_FONTS`, `TYPST_PACKAGE_PATH`, `TYPST_PACKAGE_CACHE_PATH`. `SOURCE_DATE_EPOCH` já vale nos dois.

| Caso | Vanilla | Cristalino |
|---|---|---|
| `compile … --diagnostic-format short` | `err.typ:2:2: error: cannot add integer and string`, código 1 | `unexpected argument`, código 2 |
| `--deps d.json` | `{"inputs":["lib.typ","dep.typ"],"outputs":["o.pdf"]}` | código 2 |
| `--deps d.d --deps-format make` | `o.pdf: lib.typ dep.typ` | código 2 |
| `--deps-format zero` | caminhos separados por NUL | código 2 |
| `--make-deps m.d` | grava e avisa `--make-deps is deprecated, use --deps and --deps-format instead` | código 2 |
| `--creation-timestamp 0` | data do PDF em 1970-01-01 | código 2 |
| `--ignore-system-fonts`, `--ignore-embedded-fonts`, `--jobs 2` | aceitos | código 2 |
| `eval '1+1' --root .`, `--input k=v` | código 0 | código 2 |
| `eval … --target html` | `html export is only available when --features html is passed`, código 1 | código 2 |
| `--pages 1` em PDF | `warning: using --pages implies --pdf-tagged=false`, dica `add --pdf-tagged=false to silence this warning` | `warning: exporting a page range disables PDF tagging`, dica `set pdf(tagged: false) …` |

O texto dos diagnósticos humanos já coincide (`cannot add integer and string` com o mesmo trecho e a mesma marcação).

## Causa

O analisador de argumentos de `02_shell/src/cli.rs` não declara essas opções. O L0 do CLI não as recusa; a única menção, em `00_nucleo/prompts/shell/cli.md:421`, pedia para não inventá-las numa correção anterior de stdin. O L3 já tem o que algumas precisam: as dependências do mundo (`world.dependencies()`, usadas pelo `watch`), a escolha de fontes do sistema (`03_infra/src/fonts.rs:70`) e `SOURCE_DATE_EPOCH`.

## Escopo proposto

| Parte | Mudança |
|---|---|
| A | `--diagnostic-format human\|short` e `TYPST_DIAGNOSTIC_FORMAT` em `compile`, `watch` e `eval`; `short` imprime `arquivo:linha:coluna: nível: mensagem`, como o vanilla |
| B | `--deps ARQUIVO` com `--deps-format json\|zero\|make`, e `--make-deps` com o aviso de obsoleto |
| C | `--ignore-system-fonts`, `--ignore-embedded-fonts` e as variáveis de ambiente em `compile`, `watch` e `eval`; `--ignore-embedded-fonts` em `fonts` |
| D | `--creation-timestamp` como `SOURCE_DATE_EPOCH`, `TYPST_FEATURES`, e `--jobs` aceito |
| E | `eval` com `--root`, `--input`, `--font-path` e `--target` |
| F | o aviso de `--pages` com o texto do vanilla |

## Compatibilidade e contrato

É mudança de CLI, contrato protegido (ADR-0127 §1). Só acrescenta opções; nada que hoje funciona muda, exceto o texto do aviso de F.

`--jobs` não tem efeito observável na linguagem: o vanilla usa o número para paralelizar a exportação. Aqui ele é aceito e validado como inteiro positivo, e não muda a saída.

## Fora

- `--timings` no formato de rastreio do vanilla; o Tekt mantém `--timings-json`.
- `--open` e `--no-fullscreen`, que abrem um visualizador externo.
- `--package-path` e `--package-cache-path`: o L3 usa um diretório de cache fixo (`03_infra/src/world.rs:121`), e o caminho de pacotes locais precisa de um pacote próprio.
- `image("missing.png")`, que diz `current file is outside its sandbox root` em vez de `file not found`: é do mundo, em L3.
- As opções próprias do Tekt (`--full-error`, `--timings-json`, `--html-serialization` e outras) continuam.

## Aceitação

1. L0 do CLI e dos donos em L3 e L4 tocados, primeiro, com selos e linhagem limpos.
2. RED no CLI para cada linha da tabela de medição: mesmo código de saída, mesmo arquivo gravado e mesma mensagem que o vanilla.
3. Ajuda e completamento listam as opções novas.
4. Suíte completa, lint e linhagem reversa, com build em RAM.

## Decisão pedida

Aprovar A–F. A alternativa é aprovar só A–C, que são as opções mais usadas por editores e sistemas de build, deixando D–F para depois.

## Execução

**26 de novembro de 2026.** HEAD `3cdb73482`, árvore não commitada com este pacote, o pacote do PNG de várias páginas e a alteração de outra autoria (`git diff HEAD --stat`: 43 arquivos). Cristalino debug `e290c0430d3d67da81ec52a5065cf29933dcaafb751f50f7f4dad93e7064fbd5`, compilado em RAM. Suíte do projeto 10.997 aprovados, 0 falhas, 7 ignorados; doc-tests à parte, 5 aprovados; lint sem violações; linhagem reversa 1.034 consumidores, 0 erros.

**Comparação com o vanilla.** `cliprobe.sh` roda as linhas da tabela de medição nos dois e compara código de saída, saída padrão, erro padrão, arquivos gravados e data do PDF (`cliprobe.txt`, SHA-256 `0d0c22e2226b82f28b68d225e4c6646c8ead678c5920887f3eb0e60b9024d3e0`, às 12:17). Oito das doze linhas saem idênticas. As quatro de dependências gravam os mesmos arquivos em outra ordem: o vanilla percorre um `HashMap` de identificadores de arquivo (`typst-kit/src/files.rs:91-99`), e o cristalino grava em ordem alfabética, que é estável entre execuções. A ordem é mecânica, não linguagem (ADR-0107).

**Como ficou.**

- A: `--diagnostic-format short` e `TYPST_DIAGNOSTIC_FORMAT` em `compile`, `watch` e `eval`, pelo estilo curto do `codespan`.
- B: `--deps` nos três formatos; o `watch` grava depois de cada publicação, com os nomes finais. `--make-deps` avisa como o vanilla.
- C: `--ignore-system-fonts` e `--ignore-embedded-fonts` em `compile`, `watch` e `eval`, por `SystemWorld::with_font_selection`; `fonts --ignore-embedded-fonts`.
- D: `--creation-timestamp`, `TYPST_FEATURES` e `--jobs` aceitos.
- E: `eval` com `--root`, `--input`, `--font-path` e `--target`; `--target html` sem o recurso dá o erro do vanilla, e com o recurso imprime o aviso de HTML experimental.
- F: `--pages` em PDF avisa com o texto do vanilla e desliga as tags explicitamente, sem repetir o aviso de documento.

**Limites que ficam.**

- `eval --in` com `--target html` dá erro explícito: o contexto do documento só é compilado para páginas.
- `target()` fora de `context` em `eval` dá `can only be used when context is known`; o vanilla devolve `"paged"`. É anterior a este pacote.
- Medido às 12:20, anterior a este pacote e fora dele: `compile --root . sub/x.typ o.pdf` dá `main file not found: ./x.typ`, porque o mundo recebe só o nome do arquivo (`04_wiring/src/compile.rs`); o vanilla compila. E `compile im.typ o.pdf` rodado no diretório do arquivo falha ao ler uma imagem que existe, com `image(): não foi possível ler: current file is outside its sandbox root`; com o caminho absoluto da entrada a leitura funciona. Os dois ficam para um pacote próprio de caminhos.
