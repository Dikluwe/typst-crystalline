# Passo 1470 — tornar executáveis os três doctests ignorados

**Estado:** EM MATERIALIZAÇÃO — fluxo contínuo ADR-0127

**Data da medição:** 2026-09-20T16:19:02-03:00

**Baseline:** `306ca50022104619447fae2625090b8d9c7b0a35`

**Tree:** `0f7f307e8ffda460270714cab0ae525237dc3433`

**Estado do workspace:** limpo (`git status --porcelain=v1` sem saída)

## 1. Objetivo

Converter os três exemplos Rust ignorados pela documentação de `typst-core`
em doctests compiláveis e executáveis, sem remover exemplos, esconder falhas
com `no_run` ou alterar comportamento produtivo.

O resultado esperado para os doctests do crate muda de três ignorados para
três aprovados e zero ignorados.

## 2. Medição anterior à decisão

A varredura exata

```text
rg -n '^\s*//[/!]\s*```ignore' --glob '*.rs' \
  01_core 02_shell 03_infra 04_wiring
```

encontrou somente:

| Owner | Linha no baseline | Exemplo |
|---|---:|---|
| `01_core/src/entities/introspector.rs` | 551 | `inject_positions` |
| `01_core/src/entities/introspector.rs` | 573 | `inject_pages` |
| `01_core/src/compiler/layout/mod.rs` | 3120 | `layout_with_introspector` |

Os três blocos são pseudocódigo antigo: omitem imports, construção de
`Content` e valores usados nas asserções. A suíte P1466 registrou exatamente
três doctests ignorados; não existem atributos funcionais `#[ignore]` ativos
nas camadas L1–L4.

## 3. Decisão

Cada cerca passa de `ignore` para `rust` e torna explícitos imports, entradas e
observáveis públicos:

1. `inject_positions` constrói um heading locatável, obtém sua `Location` pela
   introspecção, executa layout, injeta `extracted_positions` e exige
   `position_of(location).is_some()`;
2. `inject_pages` constrói um documento, deriva `NonZeroUsize` da quantidade
   real de páginas, injeta `PageStore` e exige que `pages(location)` devolva o
   total;
3. `layout_with_introspector` usa texto mínimo e exige que o documento preserve
   seu texto público.

Não usar `no_run`, `compile_fail`, estado global, I/O ou fixture externa. Os
exemplos devem ser determinísticos e exercitar apenas API pública L1.

## 4. Gate ADR-0127

Fluxo contínuo. O passo não adiciona nem altera campo público, método de trait,
assinatura, comportamento padrão, fase de pipeline ou compatibilidade. Apenas
transforma documentação já existente em teste executável. Os L0 proprietários
são atualizados antes dos consumers e os hashes são ressellados depois.

## 5. Validação

```text
rg -n '^\s*//[/!]\s*```ignore' --glob '*.rs' \
  01_core 02_shell 03_infra 04_wiring
cargo test -p typst-core --doc
cargo test --workspace --no-fail-fast -- --test-threads=1
cargo clippy --workspace --all-targets --all-features
cargo build --workspace
cargo fmt --all -- --check
crystalline-lint .
crystalline-lint --checks v5,v15,v26 --fail-on warning .
git diff --check
```

Sucesso exige zero cercas `ignore`, três doctests aprovados, zero ignorados e
nenhuma regressão na suíte completa.

## 6. RAM e temporários

Toda compilação, cache e temporário descartável usa somente
`/dev/shm/p1470-target` e `/dev/shm/p1470-tmp`, sem fallback para disco, com:

```text
ulimit -v 25165824
CARGO_BUILD_JOBS=1
CARGO_INCREMENTAL=0
RUST_TEST_THREADS=1
```

Se `/dev/shm` não tiver espaço ou o teto for atingido, parar. Ao terminar,
remover somente essas duas raízes e registrar resultados neste passo, sem
criar recibo, selo, certificado ou relatório paralelo.

## 7. Resultado da materialização

**Medição final:** 2026-09-20T17:03:19-03:00

**Estado medido:** baseline `306ca50022104619447fae2625090b8d9c7b0a35`
com working tree não commitada contendo quatro ficheiros rastreados alterados
e este passo novo. Antes deste resultado, o delta rastreado era de 54 inserções
e 19 remoções; o passo tinha 100 linhas.

O primeiro ciclo RED compilou e executou os três blocos: dois passaram e o
exemplo de `inject_pages` falhou porque tentava usar `Location::from_raw`, que
é corretamente privada fora do crate. O exemplo foi então corrigido para
obter a localização pela API pública `query_first(ElementKind::Heading)`.

O ciclo GREEN produziu:

- `cargo test -p typst-core --doc`: 3 aprovados, 0 falhas, 0 ignorados;
- suíte completa: 7.158 aprovados, 0 falhas, 0 ignorados;
- `cargo clippy --workspace --all-targets --all-features`: sucesso, mantendo
  apenas avisos preexistentes;
- `cargo build --workspace`: sucesso;
- `cargo fmt --all -- --check`: sucesso;
- `crystalline-lint .`: sucesso, mantendo avisos informativos preexistentes;
- `crystalline-lint --checks v5,v15,v26 --fail-on warning .`: zero violações;
- varredura das camadas L1–L4: zero cercas Rust `ignore`;
- `git diff --check`: sucesso.

Os testes e builds foram executados com teto de 24 GiB por processo, um job e
incremental desligado. Não houve OOM; no checkpoint final havia 44.952.932 kB
de `MemAvailable`. Às 2026-09-20T17:04:12-03:00, as raízes temporárias
exclusivas `/dev/shm/p1470-target` e `/dev/shm/p1470-tmp` foram removidas e
confirmadas ausentes; `/dev/shm` ficou com 32 GiB livres.

## 8. Revalidação antes do commit (2026-09-21)

O resultado de §7 ficou materializado mas por commitar — o working tree
permaneceu com os quatro ficheiros alterados intocado durante os passos
1467-1469 (que tocaram outros ficheiros). Antes do commit final desta
sessão, toda a validação de §5 foi repetida do zero contra o estado atual
(`git diff --stat` confirma os mesmos 54 inserções/19 remoções de §7, sem
deriva):

- `rg -n '^\s*//[/!]\s*```ignore' --glob '*.rs' 01_core 02_shell 03_infra
  04_wiring`: zero ocorrências;
- `cargo test -p typst-core --doc`: 3 aprovados, 0 falhas, 0 ignorados;
- `cargo test --workspace --no-fail-fast -- --test-threads=1`: **7.343**
  aprovados, 0 falhas, 0 ignorados (mais alto que os 7.158 de §7 porque
  P1467-1469, entretanto integrados noutros ficheiros, acrescentaram testes
  próprios — sem sobreposição com este diff);
- `cargo build --workspace`: sucesso;
- `cargo fmt --all -- --check`: sucesso;
- `cargo clippy --workspace --all-targets --all-features`: sucesso, zero
  `error`, só avisos pré-existentes fora do diff;
- `crystalline-lint --checks v5 .` e `crystalline-lint --checks
  v5,v15,v26 .`: zero violações (hashes de `compiler/layout.md` e
  `entities/introspector.md` já ressealados desde §7, sem deriva);
- `git diff --check`: sucesso.

Mesmo teto de RAM de §6 (`ulimit -v 25165824`, `CARGO_BUILD_JOBS=1`,
`CARGO_INCREMENTAL=0`, `RUST_TEST_THREADS=1`, raízes exclusivas em
`/dev/shm/p1470-target`/`/dev/shm/p1470-tmp`); sem OOM. Raízes removidas e
confirmadas ausentes ao final. Commit de código feito imediatamente a
seguir a este registo — sem gate de confirmação humana (fluxo contínuo,
§4), conforme já decidido em §4.
