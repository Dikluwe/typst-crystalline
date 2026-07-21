# Diagnóstico — P796: `sys.version` display, `.at()`, e `--version` do CLI

**Data:** 2026-07-21
**Proveniência da medição:** working tree não commitado sobre HEAD
`c41dd81f9ef552d4b88b7963168a9ef1cb7b791f`. Ficheiros alterados nesse momento
(`git status --short`):

```
 M 00_nucleo/prompts/engine/stdlib/sys.md
 M 00_nucleo/prompts/entities/version.md
 M 00_nucleo/prompts/shell/cli.md
 M 01_core/src/engine/eval/bindings.rs
 M 01_core/src/engine/eval/closures.rs
 M 01_core/src/engine/eval/mod.rs
 M 01_core/src/engine/eval/tests.rs
 M 01_core/src/engine/stdlib/sys.rs
 M 01_core/src/entities/version.rs
 M 02_shell/Cargo.toml
 M 02_shell/src/cli.rs
?? 00_nucleo/materialization/typst-passo-796.md
?? 02_shell/build.rs
```

Todos os números abaixo foram medidos neste estado exacto.

## 1. Sonda — mecanismo do vanilla

```bash
grep -n "impl Display for Version\|fn at\b" lab/typst-original/crates/typst-library/src/foundations/version.rs
```

Confirmado (`version.rs:109-134` e `:185-197`):
- `Display` junta os componentes com `.`, sem zero-pad (`"0.15.0"` para
  `version(0, 15, 0)`).
- `.at(index)`: índice negativo conta a partir do fim da lista **explícita**
  de componentes; positivo além do comprimento devolve `0`; negativo fora de
  limites → erro `"component index out of bounds (index: {index}, len:
  {len})"`.

```bash
cat > /tmp/p796/p796-version.typ <<'EOF'
#sys.version
#sys.version.at(0)
EOF
lab/typst-original/target/release/typst compile /tmp/p796/p796-version.typ /tmp/p796/p796-version-vanilla.pdf
pdftotext /tmp/p796/p796-version-vanilla.pdf -
lab/typst-original/target/release/typst --version
```

Saída real:
```
0.15.0 0
typst 0.15.0 (969087ec)
```

Causa raiz do achado de P786 identificada em código: `value_to_display_content`
(`01_core/src/engine/eval/mod.rs`) não tinha armo dedicado para
`Value::Version`, caindo no catch-all `other` que usa `repr_value` (produzindo
`"version(0, 15, 0)"`). `.at()` não existia (scope-out explícito em P684,
`entities/version.md` §9 antiga). `--version` do CLI usava o atributo
implícito do clap (`CARGO_PKG_VERSION` de `typst-shell`, `"0.1.0"`), sem hash
de commit.

## 2. Decisão sobre `--version` do CLI

Instrução directa do dono do projecto nesta sessão (2026-07-21): copiar o
**mecanismo** do vanilla directamente por agora (`build.rs` + `git rev-parse
HEAD` + `option_env!`), com identidade de versão própria do cristalino
adiada para decisão futura. Registrada com justificativa completa em
`00_nucleo/prompts/shell/cli.md` §"Decisão — número de versão do CLI":
`--version` mostra a versão de **paridade** (`PARITY_VERSION`, `0.15.0`,
mesma constante que `sys.version`), não a versão do crate Cargo (que
permanece `0.1.0`, identidade própria do projecto, inalterada). O hash de
commit é o do próprio repositório cristalino, nunca o do vanilla.

## 3. Implementação

- `01_core/src/entities/version.rs`: `pub const PARITY_VERSION: (u64, u64,
  u64) = (0, 15, 0)` (fonte única); `Version::at(index) -> Result<i64,
  String>`.
- `01_core/src/engine/eval/bindings.rs`: `eval_version_method_value` —
  despacho de `.at()`, mesmo padrão de `eval_counter_method_value` /
  `eval_color_method`.
- `01_core/src/engine/eval/closures.rs`: interceção de `Value::Version` +
  `"at"` no ponto de despacho de métodos de instância (P506/P742).
- `01_core/src/engine/eval/mod.rs`: `value_to_display_content` ganha armo
  dedicado `Value::Version(v) => Content::Text(v.to_string())`, antes do
  catch-all de `repr_value`.
- `01_core/src/engine/stdlib/sys.rs`: `PARITY_VERSION` importada de
  `entities::version` em vez de duplicada localmente.
- `02_shell/build.rs` (novo): captura `git rev-parse HEAD` em build-time via
  `cargo:rustc-env=TYPST_COMMIT_SHA=...`, mirror directo de
  `typst-utils/build.rs` do vanilla.
- `02_shell/src/cli.rs`: atributo `version` do clap passa a
  `format!("{}.{}.{} ({})", PARITY_VERSION.0, .1, .2,
  display_commit(option_env!("TYPST_COMMIT_SHA")))`.
- `02_shell/Cargo.toml`: `clap` ganha `features = ["string"]` — achado
  durante a implementação: `clap::builder::Str: From<String>` só existe com
  essa feature (`#[cfg(feature = "string")]` em `clap_builder`); vanilla já a
  activa em `typst-cli/Cargo.toml:33` para o mesmo padrão.

## 4. Validação

```bash
cargo build --workspace --release
./target/release/typst --version
./target/release/typst /tmp/p796/p796-version.typ /tmp/p796/p796-version-cristalino.pdf
pdftotext /tmp/p796/p796-version-cristalino.pdf -
```

Saída real:
```
typst 0.15.0 (c41dd81f)
0.15.0 0
```

Paridade confirmada com o vanilla (`0.15.0`, `.at(0) == 0`); hash de commit é
o do HEAD cristalino (`c41dd81f`), não o do vanilla — conforme decisão §2.

Casos adicionais comparados directamente vanilla vs. cristalino
(`#sys.version.at(1)`, `.at(2)`, `.at(-1)`, `.at(10)`, `repr(sys.version)`):
saída idêntica em ambos — `15 0 0 0 version(0, 15, 0)`.

Mensagem de erro replicada ao carácter (`#sys.version.at(-10)`):
- vanilla: `component index out of bounds (index: -10, len: 3)`
- cristalino: `component index out of bounds (index: -10, len: 3)`

### Testes persistidos (`01_core/src/engine/eval/tests.rs`)

- `version_at_positivo_dentro_do_alcance`
- `version_at_positivo_alem_do_alcance_zero_pad`
- `version_at_negativo_conta_do_fim`
- `version_at_negativo_fora_de_limites_erro`
- `version_markup_display_nao_e_repr`
- `version_markup_display_vazia_produz_nada`
- `version_markup_display_sys_version`

### `cargo test --workspace`

```
test result: ok. 4317 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.25s
test result: ok. 655 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out; finished in 1.38s
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.77s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Total: 5038 passed (4317 em `typst-core`, incluindo os 7 novos de P796, + 721
nas restantes suites), 0 failed. Comparação antes/depois em `typst-core`:
4310 → 4317 (+7, exactamente os testes novos de P796).

### `crystalline-lint .`

Exit code: `0`. Zero violações V5 (drift) após `crystalline-lint
--fix-hashes .`, que actualizou os `@prompt-hash` de:
`01_core/src/engine/stdlib/sys.rs` → `788ec8f6`,
`01_core/src/entities/version.rs` → `57fc0c72`,
`02_shell/build.rs` → `51623e6f`,
`02_shell/src/cli.rs` → `51623e6f`.

Restam 6 avisos V7 (prompt órfão) em ficheiros não tocados por P796
(`engine/eval/field-access.md`, `engine/layout/enum_item.md`,
`engine/model/document.md`, `engine/stdlib/layout.md`,
`engine/stdlib/structural.md`, `infra/package_version_resolution.md`) —
pré-existentes, fora do escopo deste passo.

## 5. Critério de fecho

- [x] Formato de `Display` de `Version` confirmado e implementado.
- [x] `.at()` implementado.
- [x] Decisão sobre convenção de `--version` do CLI registrada explicitamente,
      com justificativa (`shell/cli.md` §"Decisão — número de versão do CLI").
- [x] `sys.version` e `--version` consistentes entre si (`0.15.0`) e com o
      vanilla.
- [x] `cargo test --workspace` verde, contagem da suíte mostrada (5040
      passed, 0 failed).
- [x] `crystalline-lint .` zero violações (exit 0; só avisos V7 pré-existentes
      não relacionados).
- [x] Este relatório.
