# P1300 — recibo de implementação P6

## Identidade e regime

- Papel: `P6_implementer`.
- Regime: protocolo completo de materialização segregada Tekt.
- Atestação: `executed_without_technical_isolation_attestation` / executado sem
  atestação de isolamento técnico, porque os agentes partilham o filesystem.
- Contexto: contexto novo dedicado ao papel P6, iniciado independentemente do P5;
  nenhuma saída privada de P5/P7/P8 foi fornecida ao implementador.
- Commit base: `1f082370e59939de7b57992e137a9f74bfb6758f`.
- Instante da verificação final: `2026-09-03T20:12:19.830057476-03:00`.
- Manifesto congelado:
  `c3e946e1330ccab108dba2f9a438fd6a8d4c3a2af7fca3307842dbd7403feed8`.

## Capacidades e allowlists

Allowlist de leitura de ficheiros do repositório:

- `00_nucleo/diagnosticos/p1300-manifest.json`
- `00_nucleo/diagnosticos/p1300-contract-seal.json`
- `00_nucleo/prompts/compiler/eval.md`
- `00_nucleo/prompts/compiler/stdlib/color.md`
- `01_core/CLAUDE.md`
- `01_core/src/compiler/eval/mod.rs`
- `01_core/src/compiler/stdlib/color.rs`
- `Cargo.toml` e `01_core/Cargo.toml`, somente para selecionar os gates de build

Foram ainda lidas as instruções operacionais obrigatórias, externas ao
repositório, da skill `tekt-materializacao-segregada`: `SKILL.md`,
`references/papeis-e-capacidades.md` e `references/artefatos-e-gates.md`.
Comandos Git foram usados apenas para hashes, diff, estado do índice e proveniência;
não ampliaram a leitura de conteúdo.

Allowlist de escrita:

- `01_core/src/compiler/eval/mod.rs`
- `01_core/src/compiler/stdlib/color.rs`
- `00_nucleo/diagnosticos/p1300-implementation-receipt.md`

Declaração de disciplina: não abri, inspecionei nem editei
`01_core/src/compiler/eval/tests.rs`, `p1300-red-tests-receipt`, contrato,
oráculos, mutantes, runner ou outputs P5/P7/P8. Não executei `cargo test`.
Não alterei prompt L0, selo, baseline, contrato, oráculos ou testes. O
`cargo fmt --check` foi somente leitura e não modificou ficheiro algum.

## Entradas congeladas verificadas

| Entrada | SHA-256 observado |
|---|---|
| `00_nucleo/diagnosticos/p1300-manifest.json` | `c3e946e1330ccab108dba2f9a438fd6a8d4c3a2af7fca3307842dbd7403feed8` |
| `00_nucleo/diagnosticos/p1300-contract-seal.json` | `c8cb9cbfad604e6f5c638c63dfb1dc8c91a36f3bf6f50601624ed27de2439956` |
| `00_nucleo/prompts/compiler/eval.md` | `3bf911a0882e35f713cf8742f70ea921086a9a95ede6b02b8b938ab92becfc96` |
| `00_nucleo/prompts/compiler/stdlib/color.md` | `8115021062602c8a3663797b3fcfe0f17eb423f54a8ae7b437b260804ad58462` |
| baseline `01_core/src/compiler/eval/mod.rs` | `cafdcbf690f5ad8020bbe3da4457a759397c29ac091dc1624f33e14f5127c5b4` |
| baseline `01_core/src/compiler/stdlib/color.rs` | `d40197461efdee472734ef850e7a681821eb490624856bf15510d1b8a9336fed` |

Manifesto, selo e os dois prompts foram novamente hasheados após os gates e
permaneceram byte a byte nos digests acima.

## Implementação

Em `make_stdlib_with_features`, foram removidos somente os bindings globais
`linear_rgb`, `hsl` e `hsv` e os três imports locais que ficaram mortos. O
comentário local passou a registrar cinco constructors globais ratificados e
três constructors exclusivamente qualificados. Em `color.rs`, somente o
comentário que dizia que os oito constructors eram globais foi corrigido.

Hashes dos outputs de produto:

| Output | SHA-256 |
|---|---|
| `01_core/src/compiler/eval/mod.rs` | `2b805a20d6345f637a5a15d4eb43748583607aff677bde7ed28ebb38ac8cafd2` |
| `01_core/src/compiler/stdlib/color.rs` | `97f0fc77a3895dfb9ce28703bb2ef400e98123b4c97e0c09a13ed1d7164487fe` |

## Diff integral da implementação

```diff
diff --git a/01_core/src/compiler/eval/mod.rs b/01_core/src/compiler/eval/mod.rs
index e97e04180..234490a94 100644
--- a/01_core/src/compiler/eval/mod.rs
+++ b/01_core/src/compiler/eval/mod.rs
@@ -1542,14 +1542,11 @@ fn make_stdlib_with_features(
         native_here,
         native_hide,
         native_highlight,
-        native_hsl,
-        native_hsv,
         native_image,
         native_inline,
         native_json,
         native_layout,
         native_line,
-        native_linear_rgb,
         native_linebreak,
         native_link,
         // P470 — list/enum com marcadores configuráveis.
@@ -1640,18 +1637,13 @@ fn make_stdlib_with_features(
     scope.define("type", Value::Type(Type::Type));
     scope.define("repr", Value::Func(Func::native("repr", native_repr)));
     scope.define("range", Value::Func(Func::native("range", native_range)));
+    // P1300 — cinco constructors globais ratificados: `rgb`, `luma`, `cmyk`,
+    // `oklab` e `oklch`; `linear-rgb`, `hsl` e `hsv` são somente `color.*`.
     scope.define("rgb", Value::Func(Func::native("rgb", native_rgb)));
     scope.define("luma", Value::Func(Func::native("luma", native_luma)));
-    // P257 (ADR-0083 PROPOSTO) — 6 stdlib funcs novas para espaços
-    // de cor materializados (paridade vanilla `oklab`/`oklch`/
-    // `linear-rgb`/`cmyk`/`color.hsl`/`color.hsv`).
     scope.define("oklab", Value::Func(Func::native("oklab", native_oklab)));
     scope.define("oklch", Value::Func(Func::native("oklch", native_oklch)));
-    scope
-        .define("linear_rgb", Value::Func(Func::native("linear_rgb", native_linear_rgb)));
     scope.define("cmyk", Value::Func(Func::native("cmyk", native_cmyk)));
-    scope.define("hsl", Value::Func(Func::native("hsl", native_hsl)));
-    scope.define("hsv", Value::Func(Func::native("hsv", native_hsv)));
     // P685 — `str`, `int`, `float` são valores-tipo chamáveis. Os campos
     // `str.from-unicode` e `int.min`/`int.max` são agora resolvidos por field
     // access em `Value::Type` (ver `eval_field_access` em bindings.rs).
diff --git a/01_core/src/compiler/stdlib/color.rs b/01_core/src/compiler/stdlib/color.rs
index e97229ddc..cb8e1a00d 100644
--- a/01_core/src/compiler/stdlib/color.rs
+++ b/01_core/src/compiler/stdlib/color.rs
@@ -37,8 +37,8 @@ fn err_typed<T>(msg: impl Into<String>) -> SourceResult<T> {
 /// access em `Value::Type` (`eval/bindings.rs`), que delega aqui.
 ///
 /// **P742 — 17 fields** (paridade do inventário medido P736): 8 constructors
-/// (as mesmas nativas registadas globalmente: `rgb`, `linear-rgb`, `luma`,
-/// `cmyk`, `hsl`, `hsv`, `oklab`, `oklch`) + 9 operadores. Os nomes das
+/// (cinco também globais: `rgb`, `luma`, `cmyk`, `oklab`, `oklch`; três somente
+/// qualificados: `linear-rgb`, `hsl`, `hsv`) + 9 operadores. Os nomes das
 /// funcs são os **nomes plain do vanilla** — medido: `repr(color.rgb)` →
 /// `rgb`, `repr(color.lighten)` → `lighten`; `color.rgb == rgb` → `true`
 /// (igualdade por nome, `entities/func.rs`). `None` para campo inexistente
```

## Gates e resultados

Target dedicado: `/dev/shm/typst-crystalline-p1300-p6`.

| Instante inicial | Comando | Exit/result |
|---|---|---|
| `2026-09-03T20:08:45.165678740-03:00` | `cargo fmt -p typst-core --lib -- --check` | `2`; o frontend de `cargo fmt` não suporta `--lib`, portanto nenhum gate/ficheiro foi processado |
| `2026-09-03T20:09:01.725140414-03:00` | `cargo fmt -p typst-core -- --check` | `0` |
| `2026-09-03T20:09:09.777171575-03:00` | `CARGO_TARGET_DIR=/dev/shm/typst-crystalline-p1300-p6 cargo check -p typst-core --lib` | `0`; 72 warnings preexistentes, nenhum erro |
| `2026-09-03T20:09:36.750802971-03:00` | `CARGO_TARGET_DIR=/dev/shm/typst-crystalline-p1300-p6 cargo build -p typst-core --lib` | captura ficou destacada antes do exit; comando idêntico repetido para obter veredito inequívoco |
| `2026-09-03T20:10:16.832296425-03:00` | `CARGO_TARGET_DIR=/dev/shm/typst-crystalline-p1300-p6 cargo build -p typst-core --lib` | `0`; 72 warnings preexistentes, nenhum erro |
| `2026-09-03T20:12:19.830057476-03:00` | `git diff --check` | `0` |
| `2026-09-03T20:12:19.830057476-03:00` | `git diff --cached --quiet` | `0`; índice vazio |

Verificações estruturais finais:

- O inventário em `eval/mod.rs` encontrou exatamente os cinco definitions
  globais esperados: `rgb`, `luma`, `oklab`, `oklch`, `cmyk`.
- A busca pelos definitions globais `linear_rgb`, `hsl`, `hsv` devolveu exit
  `1` (nenhuma ocorrência).
- `color_type_field` ainda contém os oito arms de constructor: `rgb`,
  `linear-rgb`, `luma`, `cmyk`, `hsl`, `hsv`, `oklab`, `oklch`.
- Os pointers `native_linear_rgb`, `native_hsl`, `native_hsv` permanecem nos
  respectivos arms qualificados e em `native_color_space`; a função
  `native_color_space` permanece definida. Nenhuma nativa foi removida.
- Headers `@prompt-hash` não foram alterados; o resselo fica reservado ao P7.
- Nenhum staging, commit ou push foi executado.

## Veredito limitado do implementador

`P1300_IMPLEMENTATION_READY_FOR_INTEGRATION`: o patch candidato implementa
somente a remoção dos três aliases globais prevista no fragmento congelado,
preserva os oito constructors qualificados e compila como biblioteca. Este
recibo não atesta equivalência funcional geral nem isolamento técnico; o
veredito final pertence ao integrador/verificador segregado.
