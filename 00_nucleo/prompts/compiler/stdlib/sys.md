# Prompt L0 — `rules/stdlib/sys` — módulo builtin `sys`
Hash do Código: 1c1b08a1

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/sys.rs`
**Passo de origem**: P694
**ADRs relevantes**: ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes
de decidir), ADR-0023 (IndexMap com ordem de inserção), ADR-0024 (EcoString em
`Value::Str`), ADR-0018 (rustc_hash)

---

## Contexto e objectivo

O Typst expõe o módulo builtin `sys` com **exactamente dois campos** (medido no
vanilla 0.15.0, `lab/typst-original/crates/typst-library/src/sys.rs:15-16`):

- `sys.version: version` — a versão do compilador, como valor `version`.
- `sys.inputs: dict` — pares `chave → valor` passados na CLI via
  `--input chave=valor`. Por omissão é o dicionário vazio `(:)`. **Os valores
  são sempre strings** (mesmo `--input n=42` produz `sys.inputs.n == "42"`).

`sys` desbloqueia pacotes do ecossistema que fazem *feature-gate* por versão —
nomeadamente `oxifmt`, que faz `sys.version >= version(0, 11, 0)` e é
dependência de `cetz`. Sem `sys`, o `#import "@preview/oxifmt:1.0.0"` falha na
resolução do campo.

## Decisão: versão de **paridade**, não versão "real" (ADR-0107)

`sys.version` devolve `version(0, 15, 0)` — a versão **da linguagem com que
somos paridade**, não a versão do binário cristalino. Critério de ADR-0107: a
semântica observável pela linguagem é "que versão de Typst este documento vê";
pacotes comparam-na com `version(…)` literais. Devolver a versão real do
cristalino (que não segue a numeração Typst) quebraria `oxifmt`/`cetz` sem
ganho — a paridade é com a **linguagem**, não com a mecânica/identidade do
nosso binário. `version(0, 15, 0)` é a constante de paridade vigente.

**P796** — a constante `(0, 15, 0)` passa a viver em
`01_core/src/entities/version.rs` como `pub const PARITY_VERSION`
(`entities/version.md` §9), fonte única também consumida por `--version` do
CLI (`shell/cli.md`), fechando a inconsistência registada em P786/P796 entre
`sys.version` e `--version`. `sys.rs` importa em vez de duplicar o literal.

`#sys` repr era um **dicionário** (`(version: version(0, 15, 0), inputs: (:))`)
até P730, não `<module sys>` como no vanilla — divergência de repr/mecânica
aceita na altura (ADR-0107). **P731 fechou a divergência na origem**: `sys`
passa a ser `Value::Module` (paridade vanilla — medido: `type(sys)` →
`module`), com `version` e `inputs` no scope do módulo; a semântica de
acesso (`sys.version`, `sys.inputs`) mantém-se via `eval_field_access`
sobre `Value::Module` (P679), e o repr segue o de `Module`. A conversão
foi feita em bloco com `calc`/`math`/`sym`; `color`/`gradient` permanecem
`Value::Dict` (o vanilla expõe-os como **tipo** — achado registado).

## Decisão: fio de `inputs` via trait `World`, não via novos parâmetros

`sys.inputs` muda por invocação (depende dos `--input` da CLI), logo não pode
ser uma constante de `make_stdlib`. O caminho escolhido é **através do `World`**:

- `World` ganha `fn inputs(&self) -> SysInputs` com **implementação por omissão
  vazia** (`SysInputs::default()`). O default cobre todos os `MockWorld` de
  teste sem os tocar (espelha `read_bytes`/`include_source`/`resolve_package`).
- `SystemWorld` (L3) ganha campo `inputs` + builder `with_inputs(...)` e
  implementa `World::inputs()` devolvendo o campo (clone barato — poucos pares).
- `eval_with_full_error` (L1) lê `world.inputs()` e passa-o a
  `make_stdlib(&inputs)`, que faz `scope.define("sys", make_sys_module(&inputs))`.

Isto evita adicionar parâmetros a `eval`/`pipeline` (que partiria os 4000+
testes e os 8 callers de `eval_with_full_error`). O `World` já é a fronteira
"ambiente → núcleo"; `inputs` é ambiente, logo pertence ao `World`.

## Tipo `SysInputs`

```rust
pub type SysInputs = IndexMap<EcoString, EcoString, FxBuildHasher>;
```

Vive em `01_core/src/contracts/world.rs` (é o contrato). `IndexMap` preserva a
ordem de inserção (determinismo do repr e do `Dict`); `FxBuildHasher` é o hasher
já usado em `Value::Dict`; `EcoString` dá clone O(1). Os três crates estão em
`[l1_allowed_external]` (indexmap/ecow/rustc_hash) — `SysInputs` na assinatura
do trait **não** dispara V14.

## `make_sys_module`

```rust
pub fn make_sys_module(inputs: &SysInputs) -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("version".into(), Value::from(Version::new(0, 15, 0)));
    let mut inputs_dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    for (k, v) in inputs {
        inputs_dict.insert(k.clone(), Value::Str(v.clone()));
    }
    dict.insert("inputs".into(), Value::Dict(inputs_dict));
    Value::Dict(dict)
}
```

Puro, zero I/O, determinista. Não conhece CLI nem `World` — recebe `&SysInputs`
já resolvido.

## Língua vs mecânica (ADR-0107)

| Aspecto | Classificação | Paridade? |
|---------|---------------|-----------|
| `sys.version` é `version` e compara com `version(…)` | semântica | sim |
| `sys.inputs` é `dict` str→str, vazio por omissão | semântica | sim |
| `--input chave=valor` popula `sys.inputs.chave == "valor"` | semântica | sim |
| valores de `--input` são sempre strings | semântica | sim |
| `#sys` imprime `<module sys>` vs `(: … :)` dict | repr/mecânica | diverge (aceite) |
| estrutura interna `Value::Dict` vs `Value::Module` | mecânica | diverge (aceite) |

## Critérios de verificação

- `make_sys_module(&SysInputs::default())` → `Value::Dict` com
  `version == version(0, 15, 0)` e `inputs == (:)`.
- Com `inputs = {n: "42"}`, `make_sys_module(...).inputs.n == Value::Str("42")`.
- Header `@prompt 00_nucleo/prompts/compiler/stdlib/sys.md` e `@prompt-hash`
  correcto (via `crystalline-lint --fix-hashes .`).
- `crystalline-lint .` zero violations; em particular sem V14 (`SysInputs`
  usa tipos whitelisted) e sem V3/V4 (L1 não toca em I/O/CLI).
