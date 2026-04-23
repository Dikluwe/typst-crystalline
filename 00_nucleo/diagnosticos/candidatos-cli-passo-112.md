# Passo 112.D — Candidatos de escopo para CLI

**Data**: 2026-04-23
**Input**: 112.A (04_wiring stub), 112.B (L3 APIs), 112.C (perímetro
vanilla).
**Propósito**: ranquear 3-4 candidatos para o passo de construção 113.

**Escala de tamanho** (referência):
- Passo 104 (Sink): ~200 linhas, 8 testes. **Médio-pequeno**.
- Passo 109 (Engine): ~300 linhas + refactor big-bang em 7 ficheiros,
  0 testes novos. **Médio-grande**.
- Passo 111 (formato diag): ~60 linhas L1 + ~50 linhas L3 + 13
  testes. **Pequeno-médio**.

---

## Candidato 1 — **Micro** (argparsing positional)

### O que

CLI com argparsing manual (`std::env::args`) — uma posicional
para input, outra para output. Sem flags, sem subcomandos.

```bash
typst input.typ output.pdf
```

### Funções/APIs invocadas

- `SystemWorld::new(root=".", main=input_path)`.
- `world.source(world.main())`.
- `eval()` (directo do L1 via typst-core) com boilerplate comemo.
- `module.content()` + `introspect()` + `layout()` (L1).
- `export_pdf(&doc)` (L3 sem fonte custom).
- `fs::write(output_path, pdf_bytes)`.

### Deps externas necessárias

**Nenhuma nova.** Apenas workspace-level `anyhow` (já declarada).

### File loading

`SystemWorld::new(root=cwd, main=arg[1])`. World real do L3.

### Tamanho estimado

~80–120 linhas em `04_wiring/src/main.rs`. Sem tocar L3. **Muito
pequeno**, abaixo de Passo 111.

### O que o utilizador externo consegue fazer

Compilar um ficheiro `.typ` para `.pdf`. Sem feedback estruturado
de erros (erros saem como `anyhow` debug). Sem warnings visíveis
(Sink drena silenciosamente).

### O que fica por fora

- Warnings visíveis.
- Errors formatados (gcc/clang).
- `--root`, `--font-path`, `--format`.
- Stdin/stdout.
- Subcomandos.
- Cores.
- PNG/SVG/HTML.

### Avaliação

**Muito pequeno**. Viável. Valor limitado — equivalente a
"prova de vida". Faria sentido como sub-passo de Candidato 2.

---

## Candidato 2 — **Mínimo com warnings** (recomendação primária)

### O que

Candidato 1 + feedback rico de warnings/errors via formatter do
Passo 111 (ADR-0045):

```bash
$ typst input.typ output.pdf
input.typ:3:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
```

### Funções/APIs invocadas

Tudo de Candidato 1 +:

- **Promover `format_diagnostic`** de test-only para `pub fn` em
  `03_infra/src/diagnostic_format.rs` (ficheiro novo) ou dentro
  de `lib.rs`.
- **Promover `drain_diagnostics_to_stderr`** idem.
- **Promover `do_eval_with_sink`** (opcional) para `pub fn` em
  `03_infra/src/pipeline.rs` (ficheiro novo) ou similar. Alternativa:
  inline o boilerplate em `04_wiring/main.rs`.
- **`comemo` passa de `dev-dependencies` para `[dependencies]`**
  em `03_infra` se as promoções de `do_eval_with_sink` forem
  feitas. Ou: 04_wiring chama `typst-core` directo (já tem comemo).

### Deps externas necessárias

**Nenhuma nova.** `comemo` apenas muda de scope em `03_infra` (se
escolhermos essa via).

### File loading

SystemWorld real.

### Tamanho estimado

- 04_wiring: ~120–150 linhas.
- 03_infra: ~80 linhas novas (um módulo `diagnostic_format.rs`
  e/ou `pipeline.rs`) **OU** 0 linhas novas se inline em 04_wiring.
- Testes: 0 novos (testes actuais continuam a cobrir os helpers).

**Pequeno-médio**, comparável a Passo 111. **≤ Passo 109 em
tamanho.** ✓

### O que o utilizador externo consegue fazer

Compilar ficheiros reais e **ver o que falhou ou o que merece
atenção**. Feedback gcc/clang-compatível — editores podem parsear
o output.

### O que fica por fora

- `--root`, `--font-path`, `--format` (flags).
- Stdin/stdout.
- Subcomandos explícitos.
- Cores ANSI.
- Watch, init, query, eval, fonts, update, completions, info.
- PNG/SVG/HTML.

### Avaliação

**Recomendação primária.** Valor visível real (user pode usar a
CLI num fluxo simples), custo baixo (sem deps novas), fundamento
para candidatos futuros (3+).

---

## Candidato 3 — **Subset compile com clap**

### O que

Candidato 2 + clap para argparsing declarativo + flags mínimas:

```bash
typst compile input.typ --output output.pdf --root . --font-path ./fonts
```

Subcomando `compile` (com alias `c` como vanilla). Sem outros
subcomandos ainda.

### Funções/APIs invocadas

Tudo de Candidato 2 +:
- `clap` derive macros.
- `discover_fonts(&[PathBuf])` + `SystemWorld::with_fonts(slots)`.
- `export_pdf_with_font(&doc, &font_data)` se `--font-path` passado.

### Deps externas necessárias

- **`clap`** com feature `derive` — **nova dep no workspace
  `[workspace.dependencies]`** + no `04_wiring/Cargo.toml`.
- Opcional: `clap_complete` para completions (mas esse é Candidato
  separado).

### File loading

SystemWorld + fontes descobertas.

### Tamanho estimado

- 04_wiring: ~250–350 linhas (clap derive structs + dispatch +
  compile path).
- 03_infra: mesmos ~80 do Candidato 2.
- Testes: 0 novos (testes L3 já cobrem o pipeline).

**Médio**. Comparável a Passo 109 (refactor). **≤ Passo 109 em
tamanho.** ✓

### O que o utilizador externo consegue fazer

Compilar ficheiros com configuração realista: raiz de projecto,
fontes custom, path de output arbitrário. Help gerado por clap
(`typst compile --help`).

### O que fica por fora

- `--format` (só PDF por agora).
- Stdin/stdout (`-` como arg).
- Watch, init, query, eval, fonts, update.
- PNG/SVG/HTML.
- Cores.

### Avaliação

**Valor alto** — CLI começa a parecer a vanilla para o user
externo. **Custo**: 1 dep externa nova + ~200 linhas acima do
Candidato 2. Viável em 1 passo, mas preferivelmente depois do
Candidato 2 (construção incremental).

---

## Candidato 4 — **Subset vanilla: compile + watch**

### O que

Candidato 3 + subcomando `watch` (file watching + recompilação).

```bash
typst watch input.typ --output output.pdf
```

### Deps externas necessárias

- `clap` (como Candidato 3).
- **`notify`** — file watching. **Nova dep no workspace.**

### Tamanho estimado

- 04_wiring: ~400-500 linhas (Candidato 3 + lógica de watch +
  debounce).
- Testes: tricky — testes de watch são não-determinísticos.

**Grande**. **> Passo 109**. ✗

### Avaliação

**Excede limite**. Recomendável como passo separado **depois** do
Candidato 3. Watch traz complexidade assíncrona (thread/debounce/
cancelamento) que merece dedicação.

---

## Resumo ranqueado

| # | Candidato | Tamanho | Deps novas | Valor visível | Satisfaz ≤ 109? |
|---|-----------|---------|:---:|:---:|:---:|
| 1 | Micro positional | XS | 0 | Baixo | ✓ |
| 2 | **Mínimo com warnings** | S-M | 0 | **Médio** | **✓** |
| 3 | Subset compile (clap) | M | 1 (clap) | Alto | ✓ |
| 4 | compile + watch | L | 2 (clap, notify) | Alto | ✗ |

---

## Recomendação (para 112.E)

**Candidato 2 — Mínimo com warnings**.

Razões:
1. **Zero deps novas** — mantém enxuto.
2. **Reaproveita** toda a infra: SystemWorld (Passo 111 anterior),
   formatter gcc/clang (Passo 111).
3. **Valor visível imediato**: user compila + vê diagnósticos
   legíveis. Equivalente em valor ao que o Passo 111 abriu para
   testes, agora disponível em binário.
4. **Precondição limpa para 3/4**: com Candidato 2 no lugar,
   adicionar clap (Candidato 3) é só mudar argparsing manual para
   declarativo — sem re-arquitectura.
5. **Tamanho mensurável** — 120–150 linhas 04_wiring + ~80 linhas
   03_infra de promoção de helpers. Abaixo de Passo 109.

**Argparsing**: **manual** (`std::env::args`) no Candidato 2 —
simplicidade. Se Candidato 3 for escolhido, passar directamente
a `clap`.

**File loading**: **SystemWorld real** (L3). Já existe
production-ready; usar.

---

## Se a decisão for "micro-first" (Candidato 1)

Aceitável como precondição ao Candidato 2. Entrega prova-de-vida
em <100 linhas, sem promoções em L3. Custo: uma segunda iteração
para adicionar diagnostics.

## Se a decisão for "subset clap" (Candidato 3) directamente

Também viável — pular Candidato 2 e começar directamente em
Candidato 3. Custo: dep nova no workspace, mais ~200 linhas em 1
passo, mas evita passo intermédio.

## Se a decisão for "vanilla subset" (Candidato 4)

**Não recomendado.** Excede limite. Dividir em Candidato 3
primeiro, depois watch em passo separado.

---

## Gate 112.D

**Não disparado**. Pelo menos 3 candidatos ≤ Passo 109 existem
(1, 2, 3). Recomendação primária (Candidato 2) é a mais pequena
das três que entrega valor user-facing visível.
