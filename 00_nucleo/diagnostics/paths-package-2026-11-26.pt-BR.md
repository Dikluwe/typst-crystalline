# Proposta de pacote: caminhos de entrada, raiz e acesso a arquivos

**26 de novembro de 2026 · aprovado pelo dono em 26/11/2026 (“Aprovo A–C”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) §1 (diagnósticos observáveis)**

Os defeitos apareceram ao medir o [pacote das opções de CLI](cli-options-package-2026-11-26.pt-BR.md).

## Os problemas principais

1. **Com caminho de entrada relativo, nenhum arquivo do projeto pode ser lido.** `typst compile main.typ` com `#image("img/a.svg")` falha com `image(): não foi possível ler: current file is outside its sandbox root`. Só um caminho absoluto funciona. Isso vale para imagem, `read`, `json`, `csv`, `import` e `include`, em `compile`, `watch` e `eval --in`.
2. **`--root` não acha o arquivo principal** quando ele não está direto na raiz: `compile --root . sub/s.typ` dá `main file not found: ./s.typ`.
3. **As mensagens de acesso a arquivo têm prefixos que a linguagem não tem**, como `image(): não foi possível ler: file not found (…)` e `include: ficheiro não encontrado: …`.

## Medição

HEAD `b3951f21f`, árvore com a alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 5 arquivos, nenhum de caminhos). Cristalino debug `e290c0430d3d67da81ec52a5065cf29933dcaafb751f50f7f4dad93e7064fbd5`, compilado do mesmo código Rust que `b3951f21f`, com a mesma alteração de outra autoria. Vanilla `lab/typst-original/target/release/typst`. Sondas de 15:20 a 15:40 de Brasília num projeto com `main.typ`, `img/a.svg` e `sub/s.typ`. O projeto está em `lab/results/paths-package-2026-11-26/proj/`, local e não versionado.

| Diretório e comando | Vanilla | Cristalino |
|---|---|---|
| `proj`: `compile main.typ o.pdf` | código 0 | `current file is outside its sandbox root`, código 1 |
| `proj`: `compile ./main.typ o.pdf` | código 0 | o mesmo erro |
| raiz do teste: `compile proj/main.typ o.pdf` | código 0 | o mesmo erro |
| `proj/sub`: `compile ../main.typ o.pdf` | código 0 | o mesmo erro |
| `proj`: `compile /…/proj/main.typ o.pdf` | código 0 | código 0 |
| `link` → `proj`: `compile link/main.typ o.pdf` | código 0 | o mesmo erro |
| `proj`: `eval 'read("img/a.svg").len()' --in main.typ` | código 0 | o mesmo erro |
| `proj`: `compile --root . sub/s.typ o.pdf` | código 0 | `main file not found: ./s.typ`, código 2 |
| `compile --root proj proj/sub/s.typ o.pdf` | código 0 | `main file not found: proj/s.typ` |
| `proj/sub`: `compile --root .. s.typ o.pdf` | código 0 | `main file not found: ../s.typ` |
| `compile --root nope proj/main.typ` | `root directory not found (searched at nope)` | `main file not found: nope/main.typ` |
| `compile --root proj other/o.typ` | `source file must be contained in project root` | `main file not found: proj/o.typ` |
| `compile proj/missing.typ` | `input file not found (searched at proj/missing.typ)` | `main file not found: proj/missing.typ` |
| `proj`: `compile sub/s.typ` com `../img/a.svg` | `path "../img/a.svg" would escape the project root` | `current file is outside its sandbox root` |

Mensagens de acesso a arquivo, com entrada absoluta:

| Fonte | Vanilla | Cristalino |
|---|---|---|
| `#image("nope.png")` | `file not found (searched at …/nope.png)` | `image(): não foi possível ler: file not found (searched at …/nope.png)` |
| `#read("nope.txt")` | `file not found (searched at …)` | `read(): não foi possível ler: file not found (…)` |
| `#json("nope.json")` | `file not found (searched at …)` | `json(): não foi possível ler: file not found (…)` |
| `#import "nope.typ"`, `#include "nope.typ"` | `file not found (searched at …/nope.typ)` | `include: ficheiro não encontrado: …/nope.typ` |
| `#image("/../x.png")` | ``path `"/../x.png"` would escape the project root`` | o mesmo, com o prefixo `image(): não foi possível ler: ` |
| `#image("img")`, um diretório | `failed to load file (is a directory)` | `image(): não foi possível ler: failed to load file (Is a directory (os error 21))` |
| `#bibliography("nope.bib")` | `file not found (searched at …)` | igual |

## Causa

- `SystemWorld::new_configured` guarda a raiz como recebida (`.` ou relativa) e o arquivo principal já canonizado (`03_infra/src/world.rs:164-180`). `world_resolve_path` tira a raiz do caminho do arquivo atual com `strip_prefix`, que falha entre um caminho relativo e um absoluto (`03_infra/src/world/paths.rs:110-112`). O mesmo vale para links simbólicos, em que só um dos dois lados está resolvido.
- O `compile` de L4 entrega ao mundo só o nome do arquivo de entrada, juntado à raiz (`04_wiring/src/compile.rs`). Sem `--root`, a raiz é o diretório do arquivo e isso funciona; com `--root`, o caminho dentro da raiz se perde.
- Os construtores de L1 acrescentam prefixos às mensagens do mundo: `figure_image.rs:620`, `loading/read.rs:45`, `loading/csv.rs:330`, `visualize.rs:69`, e `ficheiro não encontrado` em `eval/modules.rs:472`, `call_dispatch/call.rs:390` e `stdlib/loading.rs:1713`.

## Escopo proposto

| Parte | Mudança |
|---|---|
| A | o mundo canoniza a raiz, e o arquivo atual é comparado com ela já canonizado; raiz inexistente dá `root directory not found (searched at …)` |
| B | com `--root`, `compile` e `watch` entregam o caminho da entrada; entrada fora da raiz dá `source file must be contained in project root`, e entrada inexistente `input file not found (searched at …)` |
| C | as mensagens de acesso a arquivo de `image`, `read`, os carregadores de dados, `tiling`, `import` e `include` são as do mundo, sem prefixo, com `failed to load file (is a directory)` para diretórios |

## Compatibilidade e contrato

- A e B fazem compilar documentos que hoje falham. Nenhum documento que hoje compila muda.
- C muda o texto de diagnósticos, que é contrato observável, para o texto da linguagem.

Não há flag, padrão, fase nem dependência nova.

## Fora

- Pacotes locais (`--package-path`), que pedem um pacote próprio.
- Espaços e caracteres especiais em nomes de arquivo além do que A e B tocarem.

## Aceitação

1. L0 dos donos em L1, L3 e L4 primeiro, com selos e linhagem limpos.
2. RED no CLI: cada linha das duas tabelas igual ao vanilla em código de saída e mensagem, rodando no diretório indicado.
3. A suíte oficial e a suíte do projeto sem nenhum teste que hoje passa falhando.
4. Suíte completa, lint e linhagem reversa, com build em RAM.

## Decisão pedida

Aprovar A–C. A alternativa é aprovar só A e B, que fazem compilar documentos comuns, deixando o texto das mensagens para depois.

## Execução

**26 de novembro de 2026.** HEAD `b3951f21f`, árvore não commitada com A–C e a alteração de outra autoria (`git diff HEAD --stat`: 25 arquivos). Cristalino debug `93de6f29844060f87b19db0ccc4985b920c9252c0ec975832c18be1222c09213`, compilado em RAM. Suíte do projeto 10.999 aprovados, 0 falhas, 7 ignorados; doc-tests 5; lint sem violações; linhagem reversa 1.034 consumidores, 0 erros.

Cada linha das duas tabelas de medição sai igual ao vanilla em código de saída e mensagem (`04_wiring/tests/cli.rs`, módulo `paths_package`), inclusive o link simbólico.

**Suíte oficial.** De 16:00:31 a 16:03:16, via PDF (`pdf-paths.json`, SHA-256 `629ba9ed4912df06c6f585fcb83626458816915e06818d3d8d424689ee569ae5`), comparada com a última medição do pacote de cabeçalhos. Nenhum teste piorou, e os mesmos 955 saem idênticos. Os erros com a mesma mensagem do vanilla passaram de 568 para 604, pela parte C.

**Como ficou.**

- A: `SystemWorld::new` canoniza a raiz e confere que o arquivo principal está dentro dela. Os mundos de `eval` e da entrada padrão canonizam a raiz quando ela existe.
- B: `compile` e `watch` entregam ao mundo o caminho da entrada. Entrada inexistente, raiz inexistente e entrada fora da raiz dão as mensagens do vanilla e código 1, também em `eval --in`.
- C: `image`, `read`, os carregadores de dados, `csv` e `tiling` repassam a mensagem do mundo sem prefixo; `include` e `import` inexistentes dão `file not found (searched at …)`, e um diretório dá `failed to load file (is a directory)`.

**Testes ajustados à linguagem.** Três testes antigos de `--root` passavam só o nome do arquivo e esperavam achá-lo dentro da raiz. No vanilla, a entrada é relativa ao diretório atual: `compile n.typ --root dir` dá `input file not found (searched at n.typ)` com código 1 (medido às 16:05). Os testes passam a dar o caminho da entrada. O teste de entrada inexistente espera o código 1, e o de `import` inexistente, a mensagem `file not found`.
