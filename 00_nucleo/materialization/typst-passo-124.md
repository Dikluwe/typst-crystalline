# Passo 124 — Testes de disciplina CLI

**Série**: 124 (passo pequeno; só testes L4).
**Precondição**: Passo 123 encerrado; 1035 total tests; zero
violations; 51 ADRs activas.
**ADRs aplicáveis**: ADR-0046 (CLI mínima — estabelece
stdout/stderr discipline), ADR-0045 (formato diagnósticos —
warnings antes de errors).
**ADR nova**: **não**. Este passo valida invariantes já
estabelecidas; zero decisão arquitectural nova.

---

## Objectivo

Materializar em testes automatizados os invariantes estruturais
do binário que hoje só existem como convenção documentada:

1. **stdout sempre vazio**. Passo 113 estabeleceu "tudo
   diagnóstico para stderr; nada para stdout excepto bytes do
   PDF, e mesmo o PDF vai para ficheiro". Nenhum teste valida.
2. **PDF válido**: começa com `%PDF-`, termina com `%%EOF`.
   Tests existentes só verificam `output.exists()`.
3. **stderr vazio em compilação limpa**: input sem warnings
   não emite nada em stderr.
4. **Exit code 0 → PDF não-vazio**: se binário devolve 0,
   ficheiro output tem conteúdo.
5. **Ordem warnings antes de errors**: se input produz ambos,
   warnings aparecem primeiro em stderr. **Decidido em 124.A**
   se o cristalino aceita input misto (warning + error).

Este passo **não**:
- Muda código de produção.
- Adiciona flags ou funcionalidade.
- Toca L1, L2, L3.
- Cria ADR nova.
- Adiciona deps.

---

## Decisões já tomadas

1. **Escopo abrangente**: 4 invariantes + teste de ordem
   condicional.
2. **Localização**: `04_wiring/tests/cli.rs` (junto dos
   existentes).
3. **Zero deps novas**: continuar com `std::process::Command`.
4. **Helpers reutilizados**: `temp_typ`, `temp_pdf`, `cleanup`
   já existentes.

## Decisão diferida (124.A)

5. **Teste de ordem**: requer input que produz warning **E**
   error simultaneamente. Viável?
   - Se sim: teste incluído.
   - Se não: teste omitido, registar como candidato futuro.

---

## Escopo

**Dentro**:
- `04_wiring/tests/cli.rs` — 5+ testes novos.
- Inventário pequeno para gate 124.A.

**Fora**:
- Qualquer outra coisa.

---

## Sub-passos

### 124.A — Inventário

**Parte 1 — Input misto (warning + error)**:

1. Testar empiricamente se `#set text(font: "X")\n#variavel_desconhecida`
   produz warning do font + error da variável.
2. Ou: construir input com `#set text(font: "X")` (warning) +
   `#import "naoexiste.typ"` (erro de I/O) ou similar.
3. Se algum input produz ambos: **teste de ordem viável**.
4. Se não (ex: error aborta antes do warning emitir): teste
   omitido.

**Parte 2 — Comportamento actual de stdout**:

Confirmar manualmente que `typst input.typ -o out.pdf` produz
stdout vazio. Se vazio, o teste é trivial. Se não vazio,
há bug latente a corrigir antes.

**Parte 3 — Formato PDF exacto**:

Confirmar empiricamente:
- `head -c 5 out.pdf` → `%PDF-` (ou `%PDF-1.7` com versão).
- `tail -c 6 out.pdf` → contém `%%EOF`.

Se formato é diferente, teste ajusta.

**Escrever** em `00_nucleo/diagnosticos/inventario-disciplina-cli-passo-124.md`:

```
Stdout em compilação normal: vazio / não vazio
PDF header: <primeiros bytes>
PDF trailer: <últimos bytes>
Input misto (warning+error): viável / não viável
  Input testado: <...>
  Resultado: <...>

Decisões:
  Teste de ordem: incluir / omitir
```

**Gate 124.A**: se stdout não é vazio em compilação normal,
parar e reportar. É bug de produção que precede qualquer teste.

### 124.B — Implementação dos testes

Em `04_wiring/tests/cli.rs`:

**124.B.1 — Stdout sempre vazio (compilação com sucesso)**:

```rust
#[test]
fn disciplina_stdout_vazio_em_sucesso() {
    let input = temp_typ("stdout_ok", "Olá");
    let output = temp_pdf("stdout_ok");
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar");
    
    let stdout = String::from_utf8_lossy(&result.stdout);
    
    assert_eq!(result.status.code(), Some(0));
    assert!(
        stdout.is_empty(),
        "stdout deve estar vazio; got {:?}",
        stdout
    );
    
    cleanup(&[&input, &output]);
}
```

**124.B.2 — Stdout vazio em erro**:

```rust
#[test]
fn disciplina_stdout_vazio_em_erro() {
    let input = temp_typ("stdout_err", "#variavel_desconhecida");
    let output = temp_pdf("stdout_err");
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar");
    
    let stdout = String::from_utf8_lossy(&result.stdout);
    
    assert_eq!(result.status.code(), Some(1));
    assert!(
        stdout.is_empty(),
        "stdout deve estar vazio mesmo em erro; got {:?}",
        stdout
    );
    
    cleanup(&[&input, &output]);
}
```

**124.B.3 — PDF magic header**:

```rust
#[test]
fn disciplina_pdf_magic_header() {
    let input = temp_typ("magic", "Olá");
    let output = temp_pdf("magic");
    
    Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar");
    
    let bytes = std::fs::read(&output).expect("ler PDF");
    assert!(
        bytes.starts_with(b"%PDF-"),
        "PDF deve começar com '%PDF-'; primeiros bytes: {:?}",
        &bytes[..bytes.len().min(8)]
    );
    
    cleanup(&[&input, &output]);
}
```

**124.B.4 — PDF trailer `%%EOF`**:

```rust
#[test]
fn disciplina_pdf_trailer_eof() {
    let input = temp_typ("eof", "Olá");
    let output = temp_pdf("eof");
    
    Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar");
    
    let bytes = std::fs::read(&output).expect("ler PDF");
    // `%%EOF` pode ou não ter newline depois; procurar no final
    let tail_len = 16.min(bytes.len());
    let tail = &bytes[bytes.len() - tail_len..];
    assert!(
        tail.windows(5).any(|w| w == b"%%EOF"),
        "PDF deve conter '%%EOF' perto do fim; tail: {:?}",
        tail
    );
    
    cleanup(&[&input, &output]);
}
```

**124.B.5 — Stderr vazio em compilação limpa**:

```rust
#[test]
fn disciplina_stderr_vazio_em_compilacao_limpa() {
    let input = temp_typ("clean_err", "= Título\n\nTexto.");
    let output = temp_pdf("clean_err");
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar");
    
    let stderr = String::from_utf8_lossy(&result.stderr);
    
    assert_eq!(result.status.code(), Some(0));
    assert!(
        stderr.is_empty(),
        "stderr deve estar vazio em compilação limpa; got {:?}",
        stderr
    );
    
    cleanup(&[&input, &output]);
}
```

**124.B.6 — Exit 0 implica PDF não-vazio**:

```rust
#[test]
fn disciplina_exit_zero_implica_pdf_nao_vazio() {
    let input = temp_typ("nonempty", "Olá");
    let output = temp_pdf("nonempty");
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar");
    
    assert_eq!(result.status.code(), Some(0));
    assert!(output.exists());
    
    let size = std::fs::metadata(&output).expect("metadata").len();
    assert!(
        size > 0,
        "PDF deve ter conteúdo; tamanho: {}",
        size
    );
    
    cleanup(&[&input, &output]);
}
```

**124.B.7 — Ordem warnings antes de errors** (condicional ao
gate 124.A):

```rust
#[test]
fn disciplina_warnings_antes_de_errors() {
    // Input misto decidido em 124.A, ajustar conforme viabilidade
    let input = temp_typ("order", "#set text(font: \"X\")\n#variavel_desconhecida");
    let output = temp_pdf("order");
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar");
    
    let stderr = String::from_utf8_lossy(&result.stderr);
    
    assert_eq!(result.status.code(), Some(1));
    
    let warning_pos = stderr.find("warning:");
    let error_pos = stderr.find("error:");
    
    match (warning_pos, error_pos) {
        (Some(w), Some(e)) => assert!(
            w < e,
            "warning: deve aparecer antes de error:; stderr:\n{}",
            stderr
        ),
        (None, _) => panic!(
            "esperava warning no input misto; stderr:\n{}",
            stderr
        ),
        (_, None) => panic!(
            "esperava error no input misto; stderr:\n{}",
            stderr
        ),
    }
    
    cleanup(&[&input, &output]);
}
```

Se 124.A concluir que input misto não é viável, **omitir este
teste** e registar candidato futuro.

### 124.C — Encerramento

1. `cargo build --release` não precisa — testes novos não
   alteram binário.
2. `cargo test -p typst-wiring` passa. Contagem L4:
   - 14 → **20** se todos os 6 incluídos.
   - 14 → **19** se teste de ordem omitido (gate 124.A).
3. `cargo test --workspace` passa (≥ 1041 ou 1040).
4. `crystalline-lint` zero violations.
5. Relatório `typst-passo-124-relatorio.md`:
   - Gate 124.A resultado (teste de ordem incluído ou omitido).
   - Input usado (se ordem incluído).
   - Descobertas empíricas (ex: bytes exactos do header,
     primeiros/últimos bytes).
   - Contagem final.
   - Limitações.

---

## Critério de conclusão

1. Inventário 124.A escrito.
2. 5 ou 6 testes novos em `04_wiring/tests/cli.rs` (conforme
   gate).
3. Zero mudança em código de produção.
4. `cargo test --workspace` passa.
5. `crystalline-lint` zero violations.
6. Relatório 124.C escrito.

---

## O que pode sair errado

- **Stdout NÃO é vazio**: gate 124.A. Se `typst input.typ -o out.pdf`
  produz output em stdout, é bug de produção. Parar, reportar,
  corrigir em passo dedicado antes de 124.
- **PDF header não é `%PDF-`**: a biblioteca de export pode
  produzir `%PDF-1.7\n` com newline; o `starts_with(b"%PDF-")`
  aceita variações de versão. Se for formato totalmente
  diferente (ex: PDF/A com BOM), ajustar.
- **`%%EOF` não está nos últimos 16 bytes**: PDFs tipicamente
  têm newline após `%%EOF`. `tail[..].windows(5)` com 16 bytes
  é robusto a isso. Se PDF termina com padding substancial,
  aumentar janela.
- **Input misto produz só um dos diagnósticos**: se
  `#variavel_desconhecida` aborta eval antes de `#set text(font: "X")`
  ser processado, só error aparece. Alternativa: inverter ordem
  do input (`#variavel_desconhecida\n#set text(font: "X")`) —
  mas comportamento depende da ordem de eval. Testar empiricamente
  em 124.A.
- **Input misto produz warning diferido**: warning do Sink é
  drenado no fim; pode aparecer depois do error se pipeline
  aborta cedo. O Passo 113 garantiu "warnings primeiro, errors
  depois" mas baseou-se em `drain` explícito. Se o eval aborta
  antes do drain, ordem pode inverter. Teste empírico é crucial.
- **Testes paralelos conflitam em temp files**: helpers já usam
  `process::id()` + nome distintivo. Cada teste aqui usa nome
  próprio (`stdout_ok`, `magic`, `eof`, etc.). Sem colisão.

---

## Notas operacionais

- Este passo é **puramente defensivo**. Zero novo código de
  produção, zero deps. Apenas materializa em testes invariantes
  que já existem.
- Se algum dos 6 testes falha, há bug latente — documentar e
  decidir se corrigir neste passo ou passo dedicado.
- Teste de disciplina stdout é o mais importante. Se um
  `println!` por engano aparecer em L3 ou L4 no futuro, este
  teste apanha imediatamente.
- PDF magic + `%%EOF` são validações estruturais mínimas. Não
  verificam se o PDF é "bem formado" em sentido rigoroso —
  para isso seria passo dedicado com parser PDF real (registado
  no relatório 114 como candidato).
- Se o gate 124.A descobrir que input misto não é viável hoje,
  o teste de ordem fica registado em
  `00_nucleo/diagnosticos/candidatos-passos-futuros.md`.
