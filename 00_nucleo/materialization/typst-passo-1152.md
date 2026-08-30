# P1152 — reparar o baseline de I/O após a migração para `RootedPath`

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; 30 falhas de I/O eliminadas`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Dependência:** P1149–P1151 materializados na working tree, ainda não commitados

## 1. Objetivo

Eliminar as 30 falhas integrais ligadas a ficheiros, migrando os worlds e
fixtures de teste da ABI antiga `read_bytes(FileId, &str)` para a fronteira
vigente P1141:

```text
resolve_path(current_file, str) -> RootedPath
read_path(&RootedPath) -> bytes
```

O passo corrige primeiro o harness. Não deve enfraquecer a resolução virtual,
reintroduzir re-resolução de strings nos consumers nem usar filesystem real em
testes L1. As 10 falhas semânticas de `int`/gradient ficam para P1153.

## 2. Proveniência e preservação da working tree

- Hora: `2026-08-25T06:56:57-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Working tree não commitada: 26 ficheiros rastreados alterados, 694 inserções
  e 154 remoções, mais os relatórios P1149–P1151 não rastreados.
- Suite pós-P1151: 5.183 testes aprovados e 40 falhas.

Antes de executar, rebaselinear com:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
cargo test -p typst-core --lib
```

Preservar integralmente P1149–P1151. Não atribuir a P1152 alterações anteriores.

## 3. Medição que precede a decisão

O contrato vigente em `01_core/src/contracts/world.rs:45-63` fornece defaults
de erro para `resolve_path` e `read_path`. Os consumers P1141 já usam a nova
fronteira por meio de `read_path_value`; vários mocks antigos implementam
somente `read_bytes`. Assim, os bytes inseridos no mock nunca são alcançados e
o erro observado é `cannot access file system from here`.

O próprio L0 já decide a migração:

- `00_nucleo/prompts/contracts/world.md` P1141 exige resolução virtual
  determinística nos mocks;
- `compiler/stdlib/loading.md`, `figure_image.md` e
  `structural/bibliography.md` exigem `Path | Str -> RootedPath -> read_path`;
- voltar a chamar `read_bytes` diretamente contrariaria esses L0s.

Essa é uma hipótese causal forte, não ainda uma licença para alterar todos os
30 testes mecanicamente. Para cada falha, registrar a função chamada, o mock
concreto, quais métodos de `World` ele implementa e o primeiro erro real.

## 4. Inventário fechado das 30 falhas

| Cluster | Qt. | Superfície |
|---|---:|---|
| imagem em eval | 1 | `eval_image_le_ficheiro_para_content` |
| bibliografia em eval | 6 | P420 ×4, P421, P450 |
| CSV em eval | 3 | P787 ×3 |
| loading | 9 | JSON, CBOR P823, read P824 ×4, `read_*` ×3 |
| plugin | 1 | erro de leitura de caminho inexistente |
| bibliografia stdlib | 4 | BIB/YAML/style/path |
| imagem stdlib | 6 | formatos ×5 e P835 |
| **Total** | **30** | |

Manter lista nominal antes/depois. Uma redução numérica com falha nova não é
GREEN.

## 5. Fase A — auditoria do harness

Construir matriz:

```text
teste | owner | World concreto | fixture/key | current_file |
resolve_path | read_path | erro observado | decisão
```

Classificar cada caso:

1. **mock obsoleto** — implementa `read_bytes`, mas não a fronteira P1141;
2. **fixture mal endereçada** — mock implementa a fronteira, porém a vpath
   normalizada não coincide com a chave armazenada;
3. **expectativa diagnóstica antiga** — caminho inexistente chega ao estrato
   correto, mas o teste espera mensagem da ABI revogada;
4. **bug de produção** — world e fixture estão corretos, mas o consumer perde
   raiz/path ou chama método errado.

Só a classe 4 autoriza tocar em código de produção, após RED isolado e nova
medição. Não aceitar o enquadramento cômodo “são apenas mocks” sem verificar o
call path de cada cluster.

## 6. Fase B — forma esperada do reparo

Para mocks em memória:

- `resolve_path` cria `RootedPath` de raiz Project determinística e normaliza
  relativamente ao `current_file` conforme o contrato já vigente;
- `read_path` consulta o mapa em memória pela vpath portátil normalizada;
- nenhuma implementação usa `std::fs`, cwd, `/tmp` ou paths físicos;
- fixtures armazenam chaves na mesma forma canónica usada por `RootedPath`;
- `read_bytes` antigo pode permanecer apenas quando outro teste legado o chama
  diretamente; não é a fonte do caminho P1141.

Preferir um helper test-only partilhado somente quando pelo menos três owners
precisarem exatamente da mesma semântica. Caso contrário, atualizar os mocks
locais para não criar uma abstração de testes maior que o contrato real.

Para caminhos inexistentes, `resolve_path` deve ter sucesso quando a string é
virtualmente válida; `read_path` é que devolve “não encontrado”. Isso preserva
a separação resolver sem I/O / ler com I/O.

## 7. Nucleação e gate ADR-0127

L0s vigentes a auditar antes do RED:

- `00_nucleo/prompts/contracts/world.md`;
- `00_nucleo/prompts/compiler/stdlib/loading.md`;
- `00_nucleo/prompts/compiler/stdlib/figure_image.md`;
- `00_nucleo/prompts/compiler/stdlib/plugin.md`;
- `00_nucleo/prompts/compiler/stdlib/structural/bibliography.md`;
- L0 de eval/bibliography se código de produção for alcançado.

O reparo estritamente test-only já está legitimado pelas cláusulas P1141 dos
L0s e segue fluxo contínuo: RED nominal -> GREEN, sem mudança pública.

Parar no gate ADR-0127 se a auditoria exigir:

- alterar método ou assinatura do trait `World`;
- mudar `RootedPath`, `VirtualRoot` ou `FileId` públicos;
- mudar resolução padrão do produto ou fronteira L1/L3;
- reintroduzir compatibilidade pública com a ABI antiga;
- alterar mensagens/semântica públicas além de correção de paridade medida.

## 8. Plano RED→GREEN por fatias

Executar em fatias independentes, repetindo o teste antes e depois:

1. loading/read (9) — menor owner comum e melhor prova da fronteira;
2. imagem stdlib + imagem eval (7);
3. bibliografia stdlib + eval (10);
4. CSV eval (3);
5. plugin inexistente (1), verificando estratificação do diagnóstico;
6. suite integral e comparação nominal.

Em cada fatia:

- confirmar RED e capturar a primeira mensagem;
- atualizar L0 primeiro somente se a auditoria revelar lacuna real;
- corrigir mock/fixture mínimo;
- confirmar GREEN do cluster;
- executar controles de path já enraizado e string relativa;
- registrar contagem e proveniência.

## 9. Critérios de aceitação

- as 30 falhas deixam de falhar sem skip, ignore ou relaxamento de assertion;
- fixtures chegam ao parser/decoder/formato que o teste pretendia exercitar;
- nenhum teste L1 toca filesystem real;
- `Value::Path` preserva raiz e `Value::Str` resolve uma única vez;
- path inexistente falha no estrato de leitura, não no default de resolução;
- nenhuma das 10 falhas semânticas é mascarada ou alterada neste passo;
- suite integral termina com exatamente 5.213 aprovados e 10 falhas, se não
  houver mudança concorrente no número total de testes;
- lista nominal das 10 remanescentes: 1 `int(float)`, 3 P269 e 6 P273.

O número 5.213 deriva do estado não commitado medido: 5.183 + 30. Deve ser
recalculado se o rebaseline inicial mudar.

## 10. Validação

```text
cargo test -p typst-core loading --no-fail-fast
cargo test -p typst-core image --no-fail-fast
cargo test -p typst-core bibliography --no-fail-fast
cargo test -p typst-core p787 --no-fail-fast
cargo test -p typst-core plugin_caminho_inexistente --no-fail-fast
cargo test -p typst-core --lib
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Warnings preexistentes não contam como violações, mas nenhum warning novo do
lote deve ser introduzido.

## 11. Handoff

P1152 encerra a dívida do harness P1141. O sucessor P1153 mede no vanilla e
resolve as 10 falhas semânticas restantes em três sublotes: `int(float)`,
normalização dos named args radiais e quantidade mínima de gradient stops.

## 12. Execução e resultado

### 12.1 Rebaseline

- Hora: `2026-08-25T07:03:34-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Estado: working tree não commitada dos lotes P1149–P1151, preservada.
- Entrada: 5.183 testes aprovados e 40 falhas.

### 12.2 Causa confirmada e reparo

Os consumers de produção já chegavam corretamente a `resolve_path` e
`read_path`. A causa comum era test-only: os mocks locais ainda implementavam
somente `read_bytes`, portanto caíam nos defaults de erro do trait `World`.

Foram migrados os worlds em memória de:

- `compiler/eval/tests.rs`;
- `compiler/eval/bibliography.rs`;
- `compiler/stdlib/loading.rs`;
- `compiler/stdlib/plugin.rs`;
- `compiler/stdlib/mod.rs`.

Cada mock passou a resolver para `VirtualRoot::Project` e a ler o mapa em
memória pela vpath portátil. O filesystem real não foi usado. Nenhum contrato,
código de produção, comportamento por defeito ou fase do pipeline mudou; por
isso o gate ADR-0127 não foi acionado.

Quando os bytes voltaram a alcançar os parsers, seis assertions antigas foram
alinhadas à representação canónica P1141 com `/` inicial (`/foto.png`,
`/refs.bib`, `/logo.png`, `/latin1.txt` e `/multilinha.txt`). Isso altera apenas
expectativas internas dos testes.

### 12.3 GREEN por fatia

- loading: 33 aprovados, zero falhas;
- imagem: 47 aprovados, zero falhas;
- bibliografia: 98 aprovados, zero falhas;
- P787/CSV: 10 aprovados, zero falhas;
- plugin com caminho inexistente: 1 aprovado, zero falhas.

A suite integral terminou com **5.213 aprovados, 10 falhas e zero ignorados**.
As 30 falhas nominais de I/O desapareceram sem substituição por falhas novas.

As 10 remanescentes são exatamente o scope-out P1153:

- 1: `native_int_float_retorna_err`;
- 3 P269: `p269_stdlib_radial_focal_ambos_named`,
  `p269_stdlib_radial_focal_center_named` e
  `p269_stdlib_radial_focal_radius_named`;
- 6 P273: `p273_stdlib_conic_relative_self`,
  `p273_stdlib_linear_relative_auto_default_none`,
  `p273_stdlib_linear_relative_default_none_preserva_p270`,
  `p273_stdlib_linear_relative_parent`,
  `p273_stdlib_linear_relative_self` e
  `p273_stdlib_radial_relative_parent`.

### 12.4 Validação final

Concluíram com sucesso:

```text
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

O linter terminou sem violations; warnings/informações exibidos já eram
preexistentes. P1152 está encerrado e P1153 pode medir as três divergências de
linguagem contra o vanilla ratificado antes de decidir qualquer correção.
