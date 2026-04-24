# Passo 121 — `--root DIR` flag (segunda flag funcional)

**Série**: 121 (segunda flag funcional; segue pattern ADR-0051).
**Precondição**: Passo 120 encerrado (`-o/--output`); 1025 total;
zero violations.
**ADRs aplicáveis**: ADR-0051 (flags funcionais).
**ADR nova**: **não**. Este passo aplica o pattern já estabelecido.
Se a semântica de `--root` divergir do preview da ADR-0051, **a
ADR é anotada** (não substituída) com a decisão factual.

---

## Objectivo

Adicionar flag `--root DIR` à CLI. Semântica e default **decididos
em 121.A** com base em inventário vanilla. Testes mínimos +
candidato registado para teste com imports real.

Ao fim do passo:

1. `Args` em L2 tem campo `root: Option<PathBuf>`.
2. `resolve_root_with(...)` função pura em L2.
3. `RunIntent` ganha campo `root: PathBuf` (sempre resolvido).
4. L4 passa `intent.root` para `SystemWorld::new` em vez de derivar
   de `input.parent()` localmente.
5. Tests 114 passam sem modificação (compat).
6. Teste mínimo novo em L4 (`--root` explícito).
7. Candidato registado: "teste de imports cross-file via --root"
   para passo futuro quando infra existir.

Este passo **não**:
- Adiciona `--font-path`.
- Toca pipeline de compilação.
- Muda L1.
- Adiciona subcomandos.

---

## Decisões já tomadas

1. **Pattern ADR-0051** — aplica-se literalmente.
2. **Validação profunda fora de L2** — se `--root /nao-existe`,
   L2 devolve PathBuf; L3 (`SystemWorld::new`) falha; L4 imprime
   erro. Alinhado com P5 da ADR-0051.
3. **`RunIntent` ganha campo `root: PathBuf`** — L4 desestrutura
   `root` como faz com `input`, `output`, `colored`.
4. **Teste mínimo em L4**: 1 teste que passa `--root` explícito
   sem verificar imports.
5. **Candidato futuro registado**: teste com `#import` cross-file
   via `--root`. Requer infra de ficheiros temporários com
   imports — passo dedicado.

## Decisões diferidas (121.A)

6. **Semântica de `--root`**:
   - **(a) Root só para imports; input absoluto** — input lê
     onde está; imports resolvem relativos a root.
   - **(b) Root contém input** — erro se input fora de root.
   - Decisão: conforme vanilla.
7. **Default quando `--root` omitido**:
   - **(a) `input.parent()` primeiro, CWD fallback** — retrocompat
     com Passo 113.
   - **(b) CWD directamente** — muda comportamento do 113.
   - **(c) Só `input.parent()` (erro se sem parent)** — mais
     restritivo.
   - Decisão: preferir (a) para retrocompat; mudar se vanilla
     tem convenção diferente.

---

## Escopo

**Dentro**:
- `02_shell/src/cli.rs` — adicionar `root` em `Args`;
  `resolve_root_with` função pura; testes unit.
- `02_shell/src/cli.rs` — `RunIntent` ganha campo `root:
  PathBuf`.
- `04_wiring/src/main.rs` — desestruturar `root` de `RunIntent`;
  passar para `SystemWorld::new`. Remover `input.parent()` local.
- `04_wiring/tests/cli.rs` — 1 teste novo `cli_root_explicito`.
- ADR-0051 anotada se semântica diverge do preview.
- Prompts L0 actualizados: `shell/cli.md`, `wiring.md`.

**Fora**:
- `--font-path`.
- Teste com imports real (candidato futuro).
- Mudanças em `SystemWorld::new` (usar API actual).
- Validação de path em L2.

---

## Sub-passos

### 121.A — Inventário vanilla + decisões

**Parte 1 — Semântica vanilla**:

1. `view` em `lab/typst-original/crates/typst-cli/src/args.rs`
   (ou equivalente). Procurar `root`:
   - Declaração do argumento.
   - Descrição no help.
2. `view` em `lab/typst-original/crates/typst-cli/src/main.rs`
   (ou onde `World` é construído). Procurar como `root` é usado:
   - É passado directo ao `World`?
   - Há validação (verificar se input está dentro)?
   - Fallback quando omitido?

Registar literalmente.

**Parte 2 — `SystemWorld::new` actual**:

1. `view` em `03_infra/src/world.rs`. Confirmar assinatura de
   `SystemWorld::new`.
2. Se aceita `(root: &Path, main: &Path)`, usar directo.
3. Se há método alternativo (ex: `SystemWorld::with_root(root)`),
   registar.

**Parte 3 — Decisão de semântica**:

Com base em Parte 1:

- **Vanilla (a)**: alinhar com (a) — root só para imports.
- **Vanilla (b)**: alinhar com (b) — root contém input.
- **Vanilla faz outra coisa**: registar e decidir.

**Parte 4 — Decisão de default**:

Com base em Parte 1:

- **Vanilla usa `input.parent()` ou CWD**: alinhar.
- **Vanilla erra sem `--root`**: divergir — manter (a) para
  retrocompat.

**Escrever** em `00_nucleo/diagnosticos/inventario-root-flag-passo-121.md`:

```
Vanilla:
  Declaração: [...]
  Uso: [...]
  Default quando omitido: [...]

SystemWorld::new:
  Assinatura: [...]

Decisões:
  Semântica: (a) / (b) / outro
  Default: (a) / (b) / (c)
  Razão: [...]
```

**Gate 121.A**: se vanilla tem semântica que contradiz P5 da
ADR-0051 (ex: vanilla faz validação em L2), registar e decidir
se seguir vanilla (divergir de ADR-0051) ou manter ADR-0051
(divergir de vanilla). Preferência: **manter ADR-0051**
(consistência arquitectural > alinhamento exacto com vanilla em
I/O concerns).

### 121.B — ADR (anotação, não nova)

Se semântica e default escolhidos batem com preview da ADR-0051,
**nenhuma ADR nova**. Só prompts actualizados.

Se semântica ou default divergem do preview, **anotar ADR-0051**
na secção "Preview" com:

```markdown
### Nota Passo 121 — decisão factual sobre --root

O preview acima sugeriu `input.parent()` como default; o inventário
do vanilla em 121.A mostrou [...]. Decisão final: [...]

Razão: [...]
```

Não criar ADR nova excepto se a divergência for significativa
(ex: semântica totalmente diferente que afecta o pattern P3/P4).

### 121.C — Implementação

Ordem obrigatória.

**121.C.1 — `Args` em `02_shell/src/cli.rs`**:

Adicionar campo:

```rust
/// Root directory for resolving imports and included files.
/// If omitted, defaults to the parent directory of `input`.
#[arg(long = "root", value_name = "DIR")]
root: Option<PathBuf>,
```

Adicionar `resolve_root_with` conforme decisão 121.A:

```rust
/// Resolve root directory for import resolution.
///
/// Pure function — takes raw args, returns resolved PathBuf.
/// Does not verify path exists (I/O is L3/L4 responsibility).
pub fn resolve_root_with(
    root: Option<&PathBuf>,
    input: &Path,
) -> PathBuf {
    root.cloned()
        .or_else(|| input.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}
```

(Ajustar conforme decisão em 121.A.)

Adicionar ao `RunIntent`:

```rust
pub struct RunIntent {
    pub input: PathBuf,
    pub output: PathBuf,
    pub root: PathBuf,   // NOVO — 121
    pub colored: bool,
}
```

Actualizar `parse()`:

```rust
pub fn parse() -> RunIntent {
    let args = Args::parse();
    let output = resolve_output_with(&args.input, args.output.as_ref(), args.output_flag.as_ref());
    let root = resolve_root_with(args.root.as_ref(), &args.input);
    let colored = resolve_colored(&args.color);
    RunIntent {
        input: args.input,
        output,
        root,
        colored,
    }
}
```

**121.C.2 — Testes unitários em L2**:

Em `02_shell/src/cli.rs` `#[cfg(test)]`:

```rust
#[test]
fn resolve_root_explicito_usa_valor() {
    let root = PathBuf::from("/custom/root");
    let input = PathBuf::from("/some/place/file.typ");
    assert_eq!(
        resolve_root_with(Some(&root), &input),
        PathBuf::from("/custom/root"),
    );
}

#[test]
fn resolve_root_omitido_usa_input_parent() {
    let input = PathBuf::from("/a/b/file.typ");
    assert_eq!(
        resolve_root_with(None, &input),
        PathBuf::from("/a/b"),
    );
}

#[test]
fn resolve_root_input_sem_parent_usa_cwd() {
    // Input = "file.typ" sem path — parent é Some("") ou None dependendo de Rust.
    let input = PathBuf::from("file.typ");
    let root = resolve_root_with(None, &input);
    // Em Rust, Path::parent() de "file.typ" é Some("")
    // Se `""` é aceitável, teste adapta. Se queremos "." como fallback:
    assert!(root == PathBuf::from("") || root == PathBuf::from("."));
}
```

Ajustar o terceiro teste após ver empiricamente o que
`Path::parent()` devolve para input sem directório.

**121.C.3 — L4 em `04_wiring/src/main.rs`**:

Antes (hoje):

```rust
let RunIntent { input, output, colored } = cli::parse();
let root = input.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
let world = match SystemWorld::new(&root, &input) { /* ... */ };
```

Depois:

```rust
let RunIntent { input, output, root, colored } = cli::parse();
// `root` já resolvido por L2 — usa directamente
let main_path = input.file_name()
    .map(PathBuf::from)
    .unwrap_or_else(|| input.clone());
let world = match SystemWorld::new(&root, &main_path) { /* ... */ };
```

Validar com `cargo check -p typst-wiring`.

**121.C.4 — Teste L4 novo**:

Em `04_wiring/tests/cli.rs`:

```rust
#[test]
fn cli_root_explicito() {
    let input = temp_typ("root", "Olá");
    let output = temp_pdf("root");
    let root = std::env::temp_dir();  // existe
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg("--root")
        .arg(&root)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");
    
    assert_eq!(result.status.code(), Some(0));
    assert!(output.exists());
    
    cleanup(&[&input, &output]);
}
```

**121.C.5 — Prompts L0**:

Actualizar `00_nucleo/prompts/shell/cli.md`:
- Descrever novo campo `root` em `Args`.
- `resolve_root_with` função pura + testes.
- `RunIntent.root`.

Actualizar `00_nucleo/prompts/wiring.md`:
- L4 passa `intent.root` para `SystemWorld::new`.
- Removido `input.parent()` local.

`crystalline-lint --fix-hashes .` regenera.

### 121.D — Registo de candidato futuro

Em `00_nucleo/diagnosticos/candidatos-passos-futuros.md` (ou
equivalente — criar se não existe), adicionar:

```markdown
## Candidato — Teste de imports cross-file via --root

Origem: Passo 121.
Trigger: após existir infra de ficheiros temporários com
imports em `04_wiring/tests/cli.rs` ou infra paralela.

Objectivo: teste que:
1. Cria temp dir com `main.typ` + `lib.typ`.
2. `main.typ` tem `#import "lib.typ"`.
3. Invoca binário com `--root <temp_dir>`.
4. Verifica que compila OK (imports resolvidos).
5. Inverte: mesma invocação sem `--root` → erro ou comportamento
   diferente.

Tamanho estimado: S.

Bloqueios: nenhum além de decidir se vive em
`04_wiring/tests/cli.rs` ou em módulo separado.
```

Este ficheiro serve de parking lot para candidatos identificados
mas não executados. Passos futuros consultam.

### 121.E — Encerramento

1. `cargo build --release` passa.
2. `cargo test --workspace` passa com contagem nova:
   - L1: 811 (inalterado).
   - L2: 21 → **24** (+3 testes `resolve_root_*`).
   - L3: 186 (inalterado).
   - L4: 7 → **8** (+1 teste `cli_root_explicito`).
   - Total: 1025 → **1029** (+4).
3. `crystalline-lint` zero violations.
4. Validação manual:
   - `typst input.typ -o out.pdf --root /tmp` → compila.
   - `typst input.typ` sem `--root` → comportamento igual a
     Passo 120.
   - `typst --help` mostra `--root <DIR>`.
5. ADR-0051 anotada (se divergência) ou não tocada.
6. Candidato futuro registado.
7. Relatório `typst-passo-121-relatorio.md`:
   - Decisão de semântica + razão vanilla.
   - Decisão de default + razão.
   - Se ADR-0051 foi anotada, diff.
   - L4 `main.rs` antes/depois.
   - Limitações aceites.

---

## Critério de conclusão

1. Inventário 121.A escrito.
2. ADR-0051 anotada **se aplicável**, senão inalterada.
3. `--root` flag funcional no binário.
4. `resolve_root_with` em L2 com testes.
5. `RunIntent.root` e L4 consome.
6. Tests 114 passam inalterados.
7. Teste L4 novo passa.
8. `cargo test --workspace` passa (total ≥ 1029).
9. `crystalline-lint` zero violations.
10. Candidato futuro registado.
11. Relatório 121.E escrito.

---

## O que pode sair errado

- **`SystemWorld::new` exige `main` relativo a `root`**: Passo
  113 fez `input.file_name()` como main (não path completo). Se
  `--root` é explícito e diferente de `input.parent()`, `main`
  pode não fazer sentido (não está dentro de root). Duas opções:
  - L4 calcula `main` relativo a `root` (ex: `pathdiff::diff_paths`).
  - L4 usa `input.file_name()` e aceita que funciona só se input
    está em root.
  Registar em 121.C.3 qual é usado.
- **Path::parent() de "file.typ" sem dir**: resultado em Rust
  depende da versão. Provável `Some("")`. Se é `""`, converter
  para `"."`. Teste cobre.
- **`--root` não existe em filesystem**: L2 não valida. L3 falha
  em `SystemWorld::new`. L4 imprime erro + exit 2. Teste manual
  confirma.
- **Vanilla usa `--root` de forma que contradiz ADR-0051 P5**:
  gate 121.A. Se vanilla faz validação em L2, preferência é
  manter ADR-0051 (P5). Registar divergência.
- **`--root` aceita `.` e `..`?**: clap aceita qualquer PathBuf.
  Resolução fica com `SystemWorld` (que provavelmente
  canonicaliza). Aceitar.
- **Tests existentes podem falhar se `RunIntent` quebra
  assinatura**: `RunIntent` ganha campo `root`. Se qualquer
  código fora de L2 criava `RunIntent` literal, quebra. Auditoria
  120 mostrou zero tais casos. Re-verificar.
- **L4 `input.file_name()` + `--root` explícito**: se input é
  `/tmp/a/b.typ` e `--root /other`, main_path é `b.typ` mas
  `/other/b.typ` não existe. `SystemWorld::new` falha. Este é
  comportamento esperado — registar como edge case documentado.

---

## Notas operacionais

- Este passo exercita directamente o pattern ADR-0051. Se algo
  não encaixa, é sinal de que o pattern precisa de refinamento
  (e anotar ADR-0051 é a forma certa de o registar).
- Teste realista com imports fica para o candidato futuro.
  Infra de tempdir com múltiplos ficheiros é o que falta.
- Se surgir pressão para adicionar validação em L2 (ex: verificar
  se `--root` é directório), resistir. L2 não faz I/O (P5). Usuário
  vê erro de `SystemWorld` se path é inválido.
- `resolve_root_with(None, Path::new(""))` é edge case.
  Provavelmente volta `""`. Se usuário passa input vazio, já era
  erro antes. Não cobrir neste passo.
- Próximo passo (`--font-path`) segue o mesmo esqueleto. Se este
  passo corre limpo, o próximo é quase mecânico.
