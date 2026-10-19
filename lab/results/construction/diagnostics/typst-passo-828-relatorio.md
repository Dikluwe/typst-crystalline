# Relatório — typst-passo-828: consolidar a working tree de P813 a P827 e rodar a suíte completa uma vez só

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-828.md`; único ficheiro acedido em `materialization/` neste passo).
**Proveniência das medições (regra de 2026-07-05):** commit HEAD `af4887e7e5f0b042ccd4bc7cce4b57367ce61490` ("chore: relatórios P813 a P827 e ajustes no engine e stdlib", 2026-07-22 06:55 -03:00). Working tree **limpa** em todas as medições deste passo — `git status --short` mostra apenas 2 ficheiros untracked (`00_nucleo/materialization/typst-passo-828.md`, `typst-passo-829.md`), nenhum ficheiro modificado. Todas as contagens abaixo vêm deste estado exacto. Horas: verificações ~10:15–10:45 UTC.

---

## Passo 1 — Levantar o estado real (prova da árvore única)

**Resposta à pergunta do prompt (1.1), verificada objectivamente:** os 15 passos P813–P827 **não** existem como 15 cópias separadas. Foram executados sequencialmente sobre uma única working tree e o resultado foi **commitado** em `af4887e7e` (HEAD). Prova:

```text
$ git status --short
?? 00_nucleo/materialization/typst-passo-828.md
?? 00_nucleo/materialization/typst-passo-829.md

$ git stash list
stash@{0}: On Tekt: P571: salvar working tree para testes de isolacao de causa

$ git branch -a
* Tekt
  main
  p442-get-unchecked-removal
  (remotes: origin/{main,Tekt}, upstream/* — ramos do projecto vanilla, alheios)

$ git log --oneline -5
af4887e7e chore: relatórios P813 a P827 e ajustes no engine e stdlib
2acc14eac chore: consolidação e relatórios dos passos P808 a P827 e ajustes no engine/math
c98ffc8ac chore: executa e relata passos e materializações (P798 a P807)
0661aef91 [P797] cargo fmt (formatação global)
d7421b639 [P797] Adiciona relatório de diagnóstico de paridade
```

- O stash (`P571`) e a branch local (`p442-get-unchecked-removal`) são **anteriores a P813** e não contêm nenhum destes diffs — não são cópias separadas dos 15 passos.
- `git show --stat af4887e7e` confirma que o commit contém exactamente o conteúdo dos 15 passos: os 15 relatórios `typst-passo-813..827-relatorio.md`, os L0s tocados (`eval.md`, `ops.md`, `calc.md`, `loading.md`, `plugin.md`, `sym.md`, `structural.md`, `pdf.md`, `equation.md`, `matrix.md`, etc.) e o código (`eval/math.rs`, `eval/operators.rs`, `stdlib/loading.rs`, `stdlib/calc.rs`, `stdlib/sym.rs`, `stdlib/pdf.rs`, `layout/grid_placement.rs`, `layout/equation.rs`, `math/layout/*`, `03_infra/src/{world.rs,plugin_host.rs,integration_tests.rs}`, `04_wiring/tests/cli.rs`, …).

**Desvio face ao esperado no briefing (registado, não bloqueante):** o briefing esperava HEAD `2acc14ea` com tudo **não commitado**; a realidade é HEAD `af4887e7e` com tudo **já commitado** (commit de 2026-07-22 06:55 -03, posterior aos relatórios). Não é uma cópia separada — é a mesma árvore única, apenas consolidada em commit antes deste passo. Consequência: o Passo 2 não tem merge a fazer (tal como previsto para o caso sequencial) e o commit deste passo contém apenas este relatório + o prompt P828 (ver Passo 4).

**Ordem real de execução (reconstruída das proveniências dos 15 relatórios):**
`P823 → P824 (principal) → P827 → P824 (adenda) → P814 → P815 → P821 → P816 → P820 → P817 → P818 → P822 → P826 → P813 → P825 → P819`.
A ordem **não** foi numérica (P813 correu depois de P826; P819 foi o último), o que explica as contagens intermédias aparentemente contraditórias entre relatórios (ex.: P813 mede ANTES 4464 em `typst-core`, P814 mede ANTES 4362 — árvores em momentos diferentes da cadeia, não testes perdidos). A cadeia de contagens `typst-core` é contínua e consistente de 4356 a 4494 (ver Passo 3.3).

## Passo 2 — Consolidar (verificação dos pontos de risco)

Não havia merge a fazer (aplicação sequencial já commitada). Os 4 pontos de risco do briefing foram verificados na árvore consolidada:

**(a) Correcção de `read_binario_pipeline` (adenda de P824) — PRESENTE e a passar.**

```text
$ grep -n 'read_binario_pipeline|read_binario_encoding_none_pipeline|failed to convert to string' 03_infra/src/integration_tests.rs
1616:    fn read_binario_pipeline()  → aserta "failed to convert to string (file is not valid UTF-8 in logo.png:1:1)" (linha 1630)
1637:    fn read_binario_encoding_none_pipeline()  (teste novo da adenda)

$ cargo test -p typst-infra read_binario
test integration_tests::integration::read_binario_pipeline ... ok
test integration_tests::integration::read_binario_encoding_none_pipeline ... ok
test result: ok. 2 passed; 0 failed; 675 filtered out
```

**(b) `03_infra/src/world.rs` com as mensagens P827 e P819 — AMBAS PRESENTES.**

```text
483:  format!("file not found (searched at {})", full_path.display())   (P819)
534:  Err(format!("package not found (searched for {spec})"))           (P827)
$ cargo test -p typst-infra system_world_resolve_package → 1 passed (teste P827)
```

**(c) `01_core/src/engine/eval/math.rs` com P820 e P825 — AMBOS PRESENTES.**

```text
157-158:  if matches!(val, Value::Module(_)) && name != "std" { return Err(... unknown_variable_math ...) }   (P825, sub-B)
165, 248: sym_deprecation(name) → warning de depreciação                                                   (P820)
67:       fn unknown_variable_math(span, name, in_global)                                                  (P820/P825)
$ cargo test -p typst-core p82 → 54 passed; 0 failed (inclui p820_*, p821_*, …, p827-adjacentes)
```

**(d) `01_core/src/engine/stdlib/loading.rs` com P823+P824 — AMBOS PRESENTES.**

```text
178-179:  "failed to parse CBOR ({razão}[ in {path}])"   (P823)
188:      fn cbor_error_reason                            (P823)
494-506:  "failed to convert to string (file is not valid UTF-8 in {path}:{l}:{c})" + vanilla_line_col  (P824)
1001-1002: 'expected "utf8" or none[, found {tipo}]'      (P824)
```

## Passo 3 — Validar o estado consolidado

### 3.1 `cargo build --release`

```text
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 40.52s
```
Limpo — só warnings de compilação pré-existentes (`typst-infra` 26, `typst-core` 28; sugestões `cargo fix`, alheias a este passo). Log: `temp/p828/build-release.log`.

### 3.2 `cargo test --workspace` — uma corrida só

Comando exacto: `cargo test --workspace` (log completo em `temp/p828/cargo-test-workspace.log`). Resultado:

| Alvo | Resultado |
|---|---|
| `typst-core` (lib) | **4494 passed; 0 failed; 2 ignored** |
| `typst-infra` (lib, incl. `integration_tests`) | **672 passed; 0 failed; 5 ignored** |
| `typst-shell` (lib) | **33 passed; 0 failed** |
| `typst` (main, wiring) | 2 passed; 0 failed |
| `typst-wiring` (`tests/cli.rs`) | **31 passed; 0 failed** |
| `typst-wiring` (`tests/crystalline_lint.rs`) | 2 passed; 0 failed |
| Doc-tests (core / infra / shell) | 0 passed; 0 failed; 3 / 0 / 0 ignored |

**Total: 5234 passed; 0 failed.** Uma única corrida, sem flakes (a instabilidade reportada por P816 não se reproduziu).

### 3.3 Soma ingénua dos "+N" vs contagem real

**`typst-core`:** baseline 4356 (ANTES de P823) + Σ(+N declarados):

| Passo | +N | Passo | +N |
|---|---|---|---|
| P823 | +2 | P820 | +13 |
| P824 | +4 | P817 | +20 |
| P814 | +18 | P818 | +10 |
| P815 | +15 | P822 | +9 |
| P821 | +5 | P826 | +6 |
| P816 | +6 | P813 | +8 |
| P827 | +0 | P825 | +15 |
| | | P819 | +7 |

Σ = +138 → **4356 + 138 = 4494 = contagem real** ✓ — bate exactamente.

**`typst-infra`:** baseline 659 (pré-P824, deduzido da cadeia: P824 principal quebrou `read_binario_pipeline` → 658+1failed; P827 +1 → 659+1failed; adenda P824 +1 novo +1 corrigido → 661+0) + P827(+1) + adenda P824(+2) + P821(+3) + P816(+3) + P813(+1) + P819(+4) = +13 → **659 + 13 = 672 = contagem real** ✓. O teste que P827 mediu a falhar (`read_binario_pipeline`, 1 failed) é o mesmo que a adenda de P824 corrigiu — na árvore consolidada final passa (0 failed), confirmando que as duas medições históricas estavam certas para as árvores parciais que cada passo viu (o sintoma que motivou este passo está resolvido).

**`typst-wiring` (`tests/cli.rs`):** baseline 29 + P819(+2) = **31 = real** ✓. **`typst-shell`:** 33, inalterado ✓.

**Conclusão da comparação:** a soma ingénua bate em todas as suítes. Nenhum teste perdido, duplicado ou contado duas vezes — consistente com a aplicação sequencial provada no Passo 1. As contagens dos relatórios P813–P827 passam a ser históricas; o estado actual é o deste relatório.

### 3.4 `crystalline-lint .`

```text
$ ~/.cargo/bin/crystalline-lint .   → exit 0, zero violations
```
Exactamente os **6 warnings V7 "prompt órfão" pré-existentes** (não cresceram com a consolidação; nenhum V3/V4/V5/V13/V14):

1. `00_nucleo/prompts/engine/eval/field-access.md`
2. `00_nucleo/prompts/engine/layout/enum_item.md`
3. `00_nucleo/prompts/engine/model/document.md`
4. `00_nucleo/prompts/engine/stdlib/layout.md`
5. `00_nucleo/prompts/engine/stdlib/structural.md`
6. `00_nucleo/prompts/infra/package_version_resolution.md`

Log: `temp/p828/crystalline-lint.log`.

### 3.5 Casos manuais cruzados (um por passo, fixtures `temp/p8XX/`)

Binário: `./target/release/typst` (rebuildado neste passo). Logs: `temp/p828/manual-casos-{1,2,3,4}.log`. Todos os resultados batem com o que o relatório respectivo documenta:

| Passo | Fixture | Esperado (relatório) | Observado P828 |
|---|---|---|---|
| P813 (equação) | `p813/doc.typ` | exit 0; math `x` centrado x=292.55, baseline 99.984; "Depois" 120.543 | exit 0; mutool trace: `x` @ **292.55**/741.906, `D` @ 70.867/721.347 (baselines 99.984/120.543) ✓ |
| P814 (eval) | `p814/t2.typ` (`mode: "markup"`), `t4.typ` | t2: exit 0 "Heading"; t4: `1:5: error: expected expression` exit 1 | t2: exit 0, pdftotext `Heading` ✓; t4: `t4.typ:1:5: error: expected expression`, exit 1 ✓ |
| P815 (métodos) | `p815/m1.typ`, `m3.typ` | m1: `type integer has no method `foo``; m3: dict-key + 2 hints | verbatim, exit 1, spans 1:1/2:1 ✓ |
| P816 (set) | `p816/a.typ`, `b.typ`, `c.typ` | a: `unexpected argument: nonexistent-prop` exit 1; b: `warning: unknown font family: familiaquenaoexiste` exit 0; c: `expected length, found integer` + hint exit 1 | verbatim nos três, spans 1:10/1:16/1:16 ✓ |
| P817 (calc) | `p817/probe-a.typ`, `probe-b.typ`, `probe-d.typ` | `30deg 60deg 45deg 63.43deg`; `-4 3 -4`; `decimal("342.440") decimal("3.14")` | byte-idênticos (pdftotext) ✓ |
| P818 (ops) | `p818/ord.typ`, `angle.typ` | `false true true true true true`; `true true 45deg 60deg 1` | byte-idênticos ✓ |
| P819 (plugin) | `p819/tm13-semantics-noctor.typ`, `p810/plugin/t10-transition.typ` | tm13: exit 0, pdftotext `abc`; t10: exit 0 | tm13: exit 0, `abc` ✓; t10: exit 0 ✓ |
| P820 (sym/join) | `p820/m1.typ`, `m2.typ` | m1: warning `` `join` is deprecated, use `bowtie.big` instead `` exit 0, render ⨝; m2: exit 0 ⨝ | verbatim; pdftotext ⨝ nos dois ✓ |
| P821 (target) | `p821/k1.typ`, `k5.typ` | k1: `can only be used when context is known` + 2 hints exit 1; k5: exit 0 `int` | verbatim ✓ |
| P822 (grid footer) | `p822/a_footer_mid_table.typ`, `e_invalid_column.typ` | `footer must end at the last row`; `cell could not be placed at invalid column 5` | verbatim, exit 1 ✓ |
| P823 (cbor) | `p823/doc.typ` | `failed to parse CBOR (invalid type: break, expected non-break in invalid.cbor)` exit 1 | verbatim ✓ |
| P824 (read) | `p824/noutf8.typ`, `enc-latin1.typ` | `failed to convert to string (file is not valid UTF-8 in latin1.txt:1:1)`; `expected "utf8" or none` | verbatim, exit 1 ✓ |
| P825 (math) | `p825/a2.typ` (`$ math.class… $` bare), `cls-banana.typ`, `cls-int.typ`, `s8.typ` (`std.math`), `d1.typ` (mat `&`) | a2: `unknown variable: math` + 3 hints; cls-*: cast verbatim; s8: exit 0; d1: `=` alinhados @301.215 | verbatim; mutool trace d1: `=` das duas linhas @ **301.215** ✓ |
| P826 (pdf.artifact) | `p826/k-header.typ`, `k-invalid.typ`, `kt.typ` | k-header exit 0; k-invalid: lista verbatim dos 12 kinds; kt: idem + `, found integer` | verbatim, exit codes correctos ✓ |
| P827 (pacote) | `p827/local.typ`, `preview.typ` | `package not found (searched for @local/inexistente:1.0.0)` / `@preview/…` exit 1 | verbatim nos dois ✓ |

## Passo 4 — Commit

- **Estado pré-existente:** o conteúdo dos 15 passos já estava commitado em `af4887e7e` (ver Passo 1). O commit deste passo contém apenas: este relatório (`00_nucleo/diagnosticos/typst-passo-828-relatorio.md`) e o prompt executado (`00_nucleo/materialization/typst-passo-828.md`).
- **Deixado de fora propositadamente:** `00_nucleo/materialization/typst-passo-829.md` (untracked — passo futuro, fora do âmbito de P828) e `temp/` (gitignored: `/temp/`, linha 42 do `.gitignore` — logs e fixtures ficam só em disco, como nos passos anteriores).
- **Hooks:** nenhum hook activo em `.git/hooks/` (só samples) — commit sem `--no-verify`, sem force.
- **Via escolhida (das duas autorizadas):** relatório escrito antes, commit, hash preenchido no relatório e `git commit --amend --no-edit` — um só commit final.
- **Hash do commit P828:** o commit deste relatório é o HEAD no fecho deste passo (`git log -1`). Auto-referência exacta é impossível (o hash muda a cada emenda); registo parcial: primeiro commit `5bbb5d565`, emendado **apenas** para preencher esta nota — conteúdo idêntico fora esta linha.

## Estado final declarado

A partir deste relatório, as contagens de teste de P813–P827 são históricas. O estado actual do projecto (commit `af4887e7e` + este commit): `typst-core` 4494, `typst-infra` 672, `typst-shell` 33, `typst-wiring` 31+2+2, 0 failed em todo o lado, lint com zero violations e os mesmos 6 V7 órfãos pré-existentes.
