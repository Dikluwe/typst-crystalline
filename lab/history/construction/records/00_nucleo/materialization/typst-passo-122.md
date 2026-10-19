# Passo 122 — `--font-path DIR` flag (terceira flag funcional; fecha preview ADR-0051)

**Série**: 122 (terceira flag funcional; fecho do preview original
da ADR-0051).
**Precondição**: Passo 121 encerrado (`--root`); 1029 total tests;
zero violations; 51 ADRs activas.
**ADRs aplicáveis**: ADR-0051 (flags funcionais).
**ADR nova**: **não por default**. Preview ADR-0051 já mencionou
`--font-path DIR` (repetível, `Vec<PathBuf>`). Se o passo executar
o preview literal, sem anotação. Se divergir, anotação conforme
padrão do 117/119.

---

## Objectivo

Adicionar flag `--font-path DIR` à CLI. Repetível
(`#[arg(long, action = Append)]`). Valor raw `Vec<PathBuf>`
em `RunIntent.font_paths`. L4 passa para `SystemWorld` que já
lida com font discovery em L3. Comportamento quando path inválido
**decidido em 122.A** com base no estado actual de L3.

Ao fim do passo:

1. `Args` em L2 tem campo `font_paths: Vec<PathBuf>` repetível.
2. `resolve_font_paths_with(...)` função pura em L2 (trivial —
   clone ou passagem directa).
3. `RunIntent` ganha campo `font_paths: Vec<PathBuf>`.
4. L4 passa `intent.font_paths` para construção do `SystemWorld`.
5. Tests 114-121 passam sem modificação.
6. Teste mínimo novo em L4.

Este passo **não**:
- Adiciona validação em L2 (P5 ADR-0051).
- Muda descoberta de fontes em L3 excepto o necessário para
  aceitar paths novos.
- Adiciona subcomandos.
- Toca L1.

---

## Decisões já tomadas

1. **Processamento raw em L2** — paths passam sem canonicalize,
   sem filter. L3 faz I/O.
2. **Default `Vec::new()` quando flag omitida** — sem lista de
   sistema em L2.
3. **Pattern ADR-0051** literal.

## Decisões diferidas (122.A)

4. **Comportamento quando path inválido**:
   - (a) L3 ignora silenciosamente — status quo típico.
   - (b) L3 emite warning via Sink (canal 106) — informativo.
   - (c) L3 erro fatal — excessivo.
   - Decisão: conforme o que L3 já faz hoje + o que é trivial
     acrescentar.
5. **Assinatura `SystemWorld::new` quanto a fontes**:
   - Se já aceita `Vec<PathBuf>` ou equivalente → trivial.
   - Se não aceita → passo ganha sub-passo em L3 para adicionar.
   - Decisão: 122.A inventário decide.

---

## Escopo

**Dentro**:
- `02_shell/src/cli.rs` — adicionar `font_paths` em `Args`;
  `resolve_font_paths_with` (pode ser trivial); `RunIntent.font_paths`.
- `04_wiring/src/main.rs` — passar `intent.font_paths` para L3.
- `03_infra/src/world.rs` — **se** 122.A mostrar que
  `SystemWorld::new` não aceita font paths, adicionar. Mínimo
  necessário.
- `04_wiring/tests/cli.rs` — 1 teste com `--font-path`.
- Prompts L0 actualizados: `shell/cli.md`, `infra/world.md`
  (se tocado), `wiring.md`.

**Fora**:
- Canonicalização ou filter em L2.
- Validação profunda de paths.
- Warning amigável (excepto se L3 já tem infra).
- Enum `--format`, subcomandos.

---

## Sub-passos

### 122.A — Inventário

**Parte 1 — Vanilla `--font-path`**:

1. `view` em `lab/typst-original/crates/typst-cli/src/args.rs`.
   Procurar `font-path`:
   - Declaração (`long`, `action`, `value_name`).
   - Env var associada (se existe, ex: `TYPST_FONT_PATHS`).
   - Repetível ou lista separada por `:`?
2. Registar literalmente.

**Parte 2 — `SystemWorld::new` actual**:

1. `view` em `03_infra/src/world.rs`. Procurar construtor.
2. Verificar se aceita font paths como argumento, ou se usa
   default interno, ou se há método separado.
3. `view` em `03_infra/src/fonts.rs` (ou equivalente). Funções
   públicas de font discovery: `discover_fonts`, `build_font_book`.

Possíveis cenários:

- **(α)** `SystemWorld::new(root, main)` + método separado
  `.with_fonts(paths)` ou campo `font_paths: Vec<PathBuf>` na
  construção.
- **(β)** `SystemWorld::new(root, main, font_paths: Vec<PathBuf>)`
  directo.
- **(γ)** `SystemWorld::new(root, main)` sem parâmetro de fonts.
  Uses hard-coded system paths.

Cenário (α) ou (β) → passo é pequeno/médio. Cenário (γ) → passo
é maior (muda assinatura, propaga).

**Parte 3 — Comportamento path inválido em L3**:

1. Ler `discover_fonts` ou equivalente. Verificar:
   - Se path não existe, o que faz? Panic? Warning? Skip?
2. Se skip silencioso: **cenário (a)** — aceitar.
3. Se panic ou erro: **cenário (c)** — inaceitável para
   `--font-path`. Pode precisar de ajuste em L3.
4. Se há infra de warning (Sink) acessível aqui: **cenário (b)**
   é opção.

Recomendação condicional:
- Vanilla comportamento + L3 alinhado → manter.
- L3 hoje diverge → manter L3 como está neste passo; candidato
  futuro.

**Parte 4 — Testes existentes**:

1. `grep` em `04_wiring/tests/cli.rs` por `font`. Listar.
2. Se testes existentes não tocam fonts, o teste novo é
   autocontido.

**Escrever** em `00_nucleo/diagnosticos/inventario-font-path-passo-122.md`:

```
Vanilla:
  Declaração: [...]
  Env: [...]
  Repetível: sim/não
  Default: [...]

SystemWorld::new:
  Assinatura actual: [...]
  Cenário: α / β / γ

discover_fonts ou equivalente:
  Assinatura: [...]
  Comportamento path inválido: ignora / warning / erro

Tamanho estimado:
  Se α ou β: pequeno
  Se γ: médio (ajustar SystemWorld::new)
```

**Gate 122.A**: se cenário (γ) + o ajuste em L3 exige propagação
> 2 funções, reportar e considerar se o escopo expande ou se o
passo divide em dois (primeiro ajustar L3, depois adicionar
flag).

### 122.B — ADR

Se preview ADR-0051 (Passo 120) bate com decisões reais →
**nenhuma ADR nova, sem anotação**. Aplicação limpa.

Se diverge (ex: L2 precisa de processar paths devido a limitação
técnica de clap):
- **Divergência pequena**: anotar ADR-0051.
- **Divergência significativa**: ADR nova (improvável).

### 122.C — Implementação

**122.C.1 — `Args` em L2**:

```rust
/// Additional directories to search for fonts. May be repeated.
#[arg(long = "font-path", value_name = "DIR", action = clap::ArgAction::Append)]
font_paths: Vec<PathBuf>,
```

Exemplo de uso:
`typst input.typ --font-path ./fonts --font-path /usr/share/fonts -o out.pdf`

**`resolve_font_paths_with`** pura (trivial, mas mantém pattern):

```rust
pub fn resolve_font_paths_with(font_paths: &[PathBuf]) -> Vec<PathBuf> {
    font_paths.to_vec()
}
```

Alternativa: passar `Vec<PathBuf>` directo no `parse()` sem
helper. Se for literal `args.font_paths.clone()`, helper não
adiciona valor. **Helper só se decisão futura implicar filter**.
Neste passo, passagem directa:

```rust
pub fn parse() -> RunIntent {
    let args = Args::parse();
    // ...
    RunIntent {
        input: args.input,
        output,
        root,
        font_paths: args.font_paths,  // move directo
        colored,
    }
}
```

**`RunIntent` ganha campo**:

```rust
pub struct RunIntent {
    pub input: PathBuf,
    pub output: PathBuf,
    pub root: PathBuf,
    pub font_paths: Vec<PathBuf>,  // NOVO
    pub colored: bool,
}
```

**122.C.2 — Testes unitários em L2**:

Se `resolve_font_paths_with` existe (helper), 2-3 testes:

```rust
#[test]
fn resolve_font_paths_vazio() {
    assert!(resolve_font_paths_with(&[]).is_empty());
}

#[test]
fn resolve_font_paths_preserva_ordem_e_duplicados() {
    let input = vec![
        PathBuf::from("/a"),
        PathBuf::from("/b"),
        PathBuf::from("/a"),
    ];
    let result = resolve_font_paths_with(&input);
    assert_eq!(result, input);
}
```

Se sem helper: nenhum teste unit L2. Campo é puro pass-through.

**122.C.3 — L3 ajustes (se 122.A cenário γ ou α)**:

**Cenário (β)**: nada — `SystemWorld::new(root, main, font_paths)`
já aceita.

**Cenário (α)**: L4 chama `SystemWorld::new(root, main).with_fonts(paths)`.
Se método existe, nada em L3. Se não existe, adicionar.

**Cenário (γ)**: mais complexo. Duas opções:
- Adicionar parâmetro `font_paths: Vec<PathBuf>` a
  `SystemWorld::new`. Propagação: só dentro de L3 e L4.
  Manageável.
- Criar método `SystemWorld::with_font_paths(self, paths) -> Self`
  builder. L4 faz `SystemWorld::new(...).with_font_paths(paths)`.

Preferência: modificar `SystemWorld::new` se os parâmetros
fundem semanticamente (root+main+fonts = configuração); builder
se font_paths é opcional e o default interno é útil.

**122.C.4 — L4 em `04_wiring/src/main.rs`**:

```rust
let RunIntent { input, output, root, font_paths, colored } = cli::parse();
// ...
let world = match SystemWorld::new(&root, &main_path, &font_paths) {
    Ok(w) => w,
    Err(e) => { /* ... */ }
};
```

(ajustar para assinatura real).

**122.C.5 — Prompts L0**:

Actualizar `00_nucleo/prompts/shell/cli.md`:
- Novo campo `font_paths` em `Args`.
- `RunIntent.font_paths`.

Actualizar `00_nucleo/prompts/wiring.md`:
- `SystemWorld` construído com `font_paths`.

Actualizar `00_nucleo/prompts/infra/world.md` se `SystemWorld::new`
ganhou parâmetro:
- Assinatura nova.

`crystalline-lint --fix-hashes .` regenera.

### 122.D — Teste L4

1 teste:

```rust
#[test]
fn cli_font_path_explicito() {
    let input = temp_typ("fontpath", "Olá");
    let output = temp_pdf("fontpath");
    let fontdir = std::env::temp_dir();  // existe (pode não ter fonts)
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg("--font-path")
        .arg(&fontdir)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");
    
    assert_eq!(result.status.code(), Some(0));
    assert!(output.exists());
    
    cleanup(&[&input, &output]);
}

#[test]
fn cli_font_path_repetivel() {
    let input = temp_typ("fontpath_multi", "Olá");
    let output = temp_pdf("fontpath_multi");
    let dir1 = std::env::temp_dir();
    let dir2 = std::env::temp_dir();
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg("--font-path")
        .arg(&dir1)
        .arg("--font-path")
        .arg(&dir2)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");
    
    assert_eq!(result.status.code(), Some(0));
    
    cleanup(&[&input, &output]);
}
```

Teste inválido path é **opcional** — comportamento definido em
122.A. Se L3 ignora silenciosamente:

```rust
#[test]
fn cli_font_path_inexistente_nao_falha() {
    let input = temp_typ("fp_invalid", "Olá");
    let output = temp_pdf("fp_invalid");
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg("--font-path")
        .arg("/path/que/nao/existe/xyz")
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");
    
    // Se L3 ignora silenciosamente:
    assert_eq!(result.status.code(), Some(0));
    
    cleanup(&[&input, &output]);
}
```

Registar resultado do path inválido em 122.E.

### 122.E — Encerramento

1. `cargo build --release` passa.
2. `cargo test --workspace`:
   - L1: 811 (inalterado).
   - L2: 24 → **24-26** (+0-2 testes, depende de helper).
   - L3: 186 (inalterado ou +1 se houver teste para
     SystemWorld nova assinatura).
   - L4: 8 → **10 ou 11** (+2 ou +3 tests).
   - Total: ≥ 1029.
3. `crystalline-lint` zero violations.
4. Validação manual:
   - `typst input.typ --font-path /tmp -o out.pdf` → compila.
   - `typst input.typ --font-path /a --font-path /b -o out.pdf`
     → compila, ambos paths considerados.
   - `typst --help` mostra `--font-path <DIR>` com nota "May be
     repeated".
5. ADR-0051 anotada (se divergência) ou não tocada.
6. Relatório `typst-passo-122-relatorio.md`:
   - Cenário identificado (α/β/γ).
   - Comportamento path inválido em L3 (a/b/c).
   - Diff de `SystemWorld::new` se tocado.
   - Diff de `Args` e `RunIntent`.
   - L4 `main.rs` antes/depois.
   - Limitações aceites.

---

## Critério de conclusão

1. Inventário 122.A escrito.
2. ADR-0051 inalterada **se aplicável**, anotada se divergência.
3. `--font-path` flag funcional no binário (repetível).
4. `Args.font_paths: Vec<PathBuf>` + `RunIntent.font_paths`.
5. L4 passa paths para `SystemWorld`.
6. Tests anteriores passam.
7. Tests L4 novos passam.
8. `cargo test --workspace` passa.
9. `crystalline-lint` zero violations.
10. Validação manual passa.
11. Relatório 122.E escrito.

---

## O que pode sair errado

- **Cenário (γ) + propagação larga em L3**: gate 122.A. Se
  `SystemWorld::new` ganha parâmetro novo e propagação toca > 2
  funções, dividir em dois passos.
- **L3 trata path inválido com `panic!`**: inaceitável para
  flag de CLI. Se é o caso, 122 ganha sub-passo em L3 para
  graceful handle. Pode inflar.
- **`clap::ArgAction::Append` não preserva ordem**: não é o
  caso; Append preserva. Documentar.
- **`--font-path` com valor vazio**: `typst input.typ --font-path ""`.
  Clap aceita `""` como PathBuf. L3 pode tratar como "current
  dir" ou ignorar. Edge case — aceitar.
- **Env `TYPST_FONT_PATHS`**: vanilla pode tê-lo. Se tiver, 122
  decide se implementa agora ou defere (como `TYPST_ROOT` do 121).
  Consistente: defere.
- **Teste com `temp_dir` duas vezes**: `dir1 == dir2` mas `Vec`
  com duplicados é OK. Clap não deduplica Append. L3 pode
  descobrir fontes duas vezes (redundante mas funcional).
- **Helpers L2 triviais**: se `resolve_font_paths_with` é só
  `clone()`, não é útil como função. Preferir passagem directa.
  Pattern ADR-0051 permite esta simplificação (P6 é sobre
  testabilidade; passagem directa não precisa de testes
  unitários próprios).

---

## Notas operacionais

- Este passo fecha o preview original da ADR-0051 (3 flags: -o,
  --root, --font-path). Futuros passos de flags (--format,
  subcommands) seguem o pattern estabelecido ou abrem novas ADRs.
- Se gate 122.A dispara (cenário γ com propagação larga), o
  relatório deve registar claramente qual sub-passo faltaria.
- Env vars (`TYPST_FONT_PATHS`) deferidas como `TYPST_ROOT` no
  121. Coerente.
- Se L3 emite warning via Sink para path inválido, confirmar
  que o warning chega a stderr via pipeline 106/111. Teste L4
  opcional pode capturar stderr e verificar.
- Para este passo, **nenhum helper L2 obrigatório** se a
  passagem é directa. Adicionar helper só se houver lógica real
  (ex: filter, dedup). P6 da ADR-0051 é sobre testabilidade —
  não é obrigação.
