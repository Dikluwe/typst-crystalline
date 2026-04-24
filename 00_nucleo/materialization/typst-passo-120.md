# Passo 120 — `-o/--output` flag (primeira flag funcional)

**Série**: 120 (primeira flag funcional da CLI; primeiro passo da
ADR-00NN "flags funcionais").
**Precondição**: Passo 119 encerrado (formatter em L2); 811 L1 +
15 L2 + 186 L3 + 5 L4 + 6 ignorados; 1017 total; zero violations.
**ADRs aplicáveis**: ADR-0047 (clap), ADR-0049 (CLI em L2),
ADR-0050 (formatter em L2).
**ADR nova**: ADR-00NN "Flags funcionais em L2 — pattern para
`-o`, `--root`, `--font-path`" — `PROPOSTO` em 120.B, `EM VIGOR`
em 120.E. **Abrangente** — aplica-se a `-o` agora e a
`--root`/`--font-path` em passos futuros.

---

## Objectivo

Adicionar flag `-o/--output` à CLI. Forma exacta (só flag, dupla
aceitação, sinónimo) **decidida em 120.A** com base em inventário
do vanilla e impacto nos testes 114.

Ao fim do passo:

1. `Args` em L2 tem campo `output` conforme decisão de forma.
2. `RunIntent.output` é sempre `PathBuf` resolvido (caller não
   recebe `Option`).
3. Se default for derivado, `input.with_extension("pdf")` é a
   regra.
4. ADR-00NN estabelece **pattern** para flags funcionais: "L2
   recebe raw, resolve defaults, produz intenção pura em
   `RunIntent`".
5. Tests 114 actualizados se a forma exigir (só flag).

Este passo **não**:
- Adiciona `--root`, `--font-path`.
- Toca pipeline de compilação.
- Muda L1 ou L3.
- Adiciona subcomandos.

---

## Decisões já tomadas

1. **Âmbito: só `-o/--output`** neste passo.
2. **ADR abrangente** cobre o pattern para flags funcionais
   futuras.

## Decisões diferidas (120.A)

3. **Forma**:
   - **(a) Só flag, positional removido** — `typst input.typ -o output.pdf`
     ou `typst input.typ` (default `input.pdf`). Tests 114 migram.
   - **(b) Dupla aceitação** — positional + flag. Se ambos, flag
     vence. Confuso mas compat.
   - **(c) Positional + `-o` sinónimo** — API dupla; utilizador
     escolhe. Tests 114 intactos.
4. **Default quando `-o` omitido**:
   - Derivar: `input.with_extension("pdf")`.
   - Erro: não aceitar omissão.
   - Stdout: convenção unix mas PDF em stdout é raro.

Ambas decidem-se com base em **como o vanilla faz**.

---

## Escopo

**Dentro**:
- `02_shell/src/cli.rs` — adicionar flag `output` em `Args`;
  adaptar `parse()` / `RunIntent` conforme decisão de 120.A.
- `04_wiring/tests/cli.rs` — actualizar se forma = (a).
- ADR nova em `00_nucleo/adr/`.
- Prompts L0 actualizados: `shell/cli.md`, `wiring.md`.

**Fora**:
- `--root`, `--font-path` (passos separados).
- Validação avançada (ex: `-o` aponta para directório
  inexistente — deixar `fs::write` falhar depois).
- Mudar L1, L3.
- Subcomandos.

---

## Sub-passos

### 120.A — Inventário e decisão de forma

**Parte 1 — Vanilla**:

1. `view` em `lab/typst-original/crates/typst-cli/src/args.rs`
   (ou equivalente). Procurar:
   - Como é declarado `input`? Positional ou flag?
   - Como é declarado `output`? Positional ou flag?
   - Se flag, short é `-o`? Default derivado?
2. Registar literalmente.

**Parte 2 — Tests 114**:

1. `view` em `04_wiring/tests/cli.rs`. Registar como os 5 testes
   invocam o binário.
2. Se forma é (a), contar quantos testes migram (provavelmente
   todos 5).
3. Se forma é (b) ou (c), testes não precisam de migrar.

**Parte 3 — Decisão**:

Com base nas Partes 1 e 2:

- Se vanilla usa positional: forma (c) mantém compat vanilla +
  tests. Mínima fricção.
- Se vanilla usa `-o` obrigatório: forma (a) alinha com vanilla;
  tests 114 migram (~5 linhas).
- Se vanilla usa `-o` opcional com default derivado: forma (a)
  com default.

**Recomendação condicional**:
- Se vanilla tem `-o` obrigatório → (a) com migração tests.
- Se vanilla tem `-o` opcional + default → (a) com default derivado.
- Se vanilla usa positional → (c) sinónimo, positional mantido.

**Escrever** em `00_nucleo/diagnosticos/inventario-output-flag-passo-120.md`.

**Gate 120.A**: se vanilla tem forma distinta de todas as
opções listadas (improvável), parar e reportar.

### 120.B — ADR abrangente

Criar `00_nucleo/adr/typst-adr-00NN-flags-funcionais.md` com
`PROPOSTO`.

Conteúdo:

- **Contexto**: CLI tem flags "meta" (`--color`, `--help`,
  `--version`) mas zero flags funcionais. Utilizador externo
  precisa de `-o`, `--root`, `--font-path` para casos básicos.
  Este passo inicia a série.
- **Pattern estabelecido**:
  - L2 define campos em `Args` com derive clap.
  - L2 converte args raw → valores resolvidos no `parse() ->
    RunIntent`.
  - `RunIntent` cresce com campos novos conforme flags surgem.
    L4 consome sem conhecer clap.
  - Defaults resolvidos em L2, não em L4. L4 recebe valores
    prontos.
  - Validação que exige I/O (ex: path existe) fica em L3 ou L4.
- **Decisão específica para `-o/--output`**: [forma de 120.A].
- **Próximas flags** (preview; não executadas neste passo):
  - `--root DIR`: override de `input.parent()` para import
    resolution. Pattern: L2 resolve default com input.parent();
    L4 passa para `SystemWorld::new`.
  - `--font-path DIR`: repetível (Append). L2 valida existência?
    Não — L3 descobre fontes, se path inválido, warning ou erro.
    Pattern: L2 coleciona; L4 passa para `SystemWorld::with_fonts`.
- **Limitações**:
  - Sem validação profunda em L2 (ex: verificar se path é
    directório). L2 é tradutor, L3/L4 verifica.
  - Sem short options para todas as flags (`-o` sim, `-r`/`-f`
    a decidir — reservado para subcomandos se vierem).
- **Alternativas rejeitadas**:
  - **Defaults em L4**: L4 fica menos thin; perde razão de
    existência de `RunIntent`.
  - **Validação em L2**: L2 não faz I/O (pós-117). Verificar se
    directório existe é I/O.

Promover em 120.E.

### 120.C — Implementação

**120.C.1 — Ajustar `Args` em `02_shell/src/cli.rs`**:

Para forma (a) — só flag com default derivado (exemplo):

```rust
#[derive(Parser, Debug)]
#[command(name = "typst", version, about = "Typst compiler (crystalline)")]
struct Args {
    /// Input .typ file.
    input: PathBuf,
    
    /// Output PDF file. Defaults to input with .pdf extension.
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,
    
    /// When to use coloured diagnostics.
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}
```

Para forma (c) — positional + sinónimo (exemplo):

```rust
#[derive(Parser, Debug)]
#[command(name = "typst", version, about = "Typst compiler (crystalline)")]
struct Args {
    input: PathBuf,
    /// Output PDF file.
    output: Option<PathBuf>,
    /// Alternative: output via flag. Wins over positional if both provided.
    #[arg(short = 'o', long = "output")]
    output_flag: Option<PathBuf>,
    
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}
```

Para forma (b) — dupla aceitação: semelhante a (c) mas
documentação mais explícita sobre precedência.

**Ajustar `parse()`** em `cli.rs`:

```rust
pub fn parse() -> RunIntent {
    let args = Args::parse();
    let output = resolve_output(&args);
    let colored = resolve_colored(&args.color);
    RunIntent {
        input: args.input,
        output,
        colored,
    }
}

fn resolve_output(args: &Args) -> PathBuf {
    // Exemplo para forma (a) com default derivado:
    args.output.clone()
        .unwrap_or_else(|| args.input.with_extension("pdf"))
    
    // Exemplo para forma (c):
    // args.output_flag.clone()
    //     .or(args.output.clone())
    //     .expect("either positional or -o required")
}
```

`RunIntent` permanece `{ input, output, colored }` — L4 não nota
mudança.

**Testar** `resolve_output` com testes unitários:

```rust
#[cfg(test)]
mod tests_output {
    #[test]
    fn output_explicito_e_usado() { /* ... */ }
    
    #[test]
    fn output_omitido_deriva_de_input_pdf() { /* ... */ }
    
    // Para forma (c):
    // #[test]
    // fn flag_vence_positional() { /* ... */ }
}
```

**120.C.2 — Actualizar tests 114 (se forma = a)**:

Em `04_wiring/tests/cli.rs`:

```rust
// Antes
Command::new(BIN)
    .arg(&input)
    .arg(&output)

// Depois
Command::new(BIN)
    .arg(&input)
    .arg("-o")
    .arg(&output)
```

Mudança em cada um dos 5 testes. ~5 linhas.

**Alternativa para testar default derivado**: um teste novo que
omite `-o` e verifica que `<input>.pdf` foi criado.

**120.C.3 — L4 inalterado**:

`main.rs` consome `RunIntent.output: PathBuf`. Como `RunIntent`
tem campos iguais, `main.rs` não muda.

**120.C.4 — Prompts L0**:

Actualizar `00_nucleo/prompts/shell/cli.md` para descrever
campo `output` + forma escolhida + função `resolve_output`.

`crystalline-lint --fix-hashes .` regenera hashes.

### 120.D — Testes

**Contagem esperada**:

- L1: 811 (inalterado).
- L2: 15 → **17-18** (+2-3 testes `resolve_output_*`).
- L3: 186 (inalterado).
- L4: 5 → **5 ou 6** (inalterado ou +1 teste de default derivado).

Se forma = (a) e default derivado implementado, +1 teste L4 é
desejável:

```rust
#[test]
fn cli_default_output_deriva_de_input() {
    let input = temp_typ("default", "#set text(font: \"X\")\nOlá");
    // Sem -o — espera input.pdf
    let expected_output = input.with_extension("pdf");
    let result = Command::new(BIN)
        .arg(&input)
        .output()
        .expect("executar");
    assert_eq!(result.status.code(), Some(0));
    assert!(expected_output.exists());
    cleanup(&[&input, &expected_output]);
}
```

### 120.E — Encerramento

1. `cargo build --release` passa.
2. `cargo test --workspace` passa (contagem espelha ≥ 1017 +
   novos).
3. `crystalline-lint` zero violations.
4. Validação manual:
   - `typst input.typ -o out.pdf` → funciona.
   - `typst input.typ` (sem -o, se forma permite) → funciona,
     `input.pdf` criado.
   - `typst input.typ output.pdf` (positional, se forma permite)
     → funciona.
   - `typst --help` mostra `-o, --output` na lista.
5. ADR-00NN promovida.
6. Relatório `typst-passo-120-relatorio.md`:
   - Decisão de forma + razão vanilla.
   - Diff de `Args` e `parse()`.
   - Se tests 114 migraram, diff.
   - Teste novo (se default derivado).
   - Limitações aceites.

---

## Critério de conclusão

1. Inventário 120.A escrito.
2. ADR-00NN criada e promovida.
3. `-o/--output` funcional no binário.
4. `resolve_output` em L2 com testes.
5. Se forma = (a), tests 114 migrados.
6. `cargo test --workspace` passa.
7. `crystalline-lint` zero violations.
8. Validação manual passa.
9. Relatório 120.E escrito.

---

## O que pode sair errado

- **Vanilla usa forma que não está nas 3 opções**: improvável.
  Gate 120.A.
- **Default derivado cria colisão silenciosa**: se input é
  `foo.typ` e já existe `foo.pdf`, o passo **sobrescreve sem
  aviso**. Convenção aceite em compilers (gcc sobrescreve). Se
  for preocupação, `--force` futuro. Não neste passo.
- **Positional mantido + `-o` passado** em forma (c): se ambos
  são passados, regra tem de ser clara. Preferência: flag vence.
  Documentar.
- **`resolve_output` puro mas com `PathBuf` heavy**: `with_extension`
  é allocation. Se função é chamada uma vez, custo trivial. Não
  optimizar prematuramente.
- **Tests 114 quebram em forma (a) sem migração**: `cargo test`
  falha se forma (a) escolhida mas tests não migraram. Ordem
  obrigatória em 120.C: migrar tests **antes** de correr
  `cargo test --workspace`.
- **Clap conflito com positional `Option<PathBuf>`**: em forma
  (c), `input: PathBuf` obrigatório + `output: Option<PathBuf>`
  positional pode não compilar sem `#[arg(num_args = 0..=1)]`
  ou similar. Se compila, OK; se não, ajustar ou ir para (a).
- **Stdout via `-o -`**: convenção unix para stdout. Neste passo,
  **não suportado**. Se `-o -` passado, fica como path literal
  "-". Não é crítico — utilizador pode redireccionar.

---

## Notas operacionais

- Este é o primeiro passo sob ADR "flags funcionais". Decisões
  de pattern aqui propagam.
- Se a decisão de forma for (a) com default derivado, é a opção
  mais idiomática de CLIs modernas (rustc, cargo, clang).
- Se for (c), preservação máxima é o valor; custo é API dupla.
- `resolve_output` é exemplo limpo de "tradução em L2 → valor
  pronto em `RunIntent`". Pattern a seguir para `--root` e
  `--font-path`.
- L4 **não muda** neste passo porque `RunIntent` não ganha
  campos novos; só muda o **conteúdo** de `output` (resolvido,
  não opcional).
- ADR abrangente "flags funcionais" pode descrever preview das
  3 flags mas executar só uma. Documentar preview alinha os
  próximos passos sem antecipar código.
