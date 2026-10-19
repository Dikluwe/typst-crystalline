# Relatório de Paridade — P651

**Passo:** 651  
**Data:** 2026-07-09  
**Foco:** Remover `eprintln!` de depuração esquecidos e varrer por mais leftovers do mesmo tipo.  
**Dependências:** P650 (onde o caso de `content.rs` foi encontrado).  
**Hash do commit com as alterações:** `68367aa10`

---

## 1. Sumário

Foi removido o `eprintln!("P627 segments count: ...", ...)` esquecido em `01_core/src/entities/content.rs:2311`. Uma varredura completa por `eprintln!` / `println!` em todo o código de produção confirmou que não há mais leftovers do tipo "mensagem de depuração com referência a número de passo".

Os restantes `eprintln!` em produção são intencionais: erros da CLI (`02_shell`, `04_wiring`), avisos de imagens/fontes que precisam de passos próprios (já listados em P650), ou estão dentro de testes.

---

## 2. Implementação

### 2.1 `01_core/src/entities/content.rs`

Removida a linha:

```rust
eprintln!("P627 segments count: {}", segments.len());
```

A lógica de partição de `columns` com pagebreaks (P627) permanece inalterada; apenas o output de debug foi removido.

---

## 3. Sonda — varredura completa

### 3.1 `eprintln!` / `println!` com referência a número de passo

Padrão: `grep -iE "P[0-9]{2,3}"` sobre todos os `eprintln!`/`println!`.

**Resultado:** nenhuma ocorrência adicional além da já removida.

### 3.2 Todos os `eprintln!` / `println!` em produção

| Ficheiro | Linha | Tipo | Decisão |
|---|---|---|---|
| `01_core/src/entities/content.rs` | 2311 | `eprintln!("P627 segments count: ...")` | **Removido** |
| `01_core/src/engine/eval/tests.rs` | várias | `eprintln!` em testes | Manter (test-only) |
| `03_infra/src/shaper.rs` | várias | `eprintln!("SKIP: ...")` em testes | Manter (test-only) |
| `03_infra/src/integration_tests.rs` | várias | `eprintln!("[skip] ...")` em testes | Manter (test-only) |
| `03_infra/src/export/tests.rs` | várias | `eprintln!("SKIP ...")` em testes | Manter (test-only) |
| `03_infra/src/export/subset.rs` | 206 | `eprintln!` em teste | Manter (test-only) |
| `03_infra/src/export/images.rs` | 279, 285 | Avisos de imagem inválida omitida | Manter — item próprio de P650 |
| `03_infra/src/export/builder.rs` | 658 | Aviso de fonte variável não instanciada | Manter — item próprio de P650 |
| `03_infra/src/font_variant.rs` | 193 | Log de falha do fontTools instancer | Manter — item próprio de P650 |
| `02_shell/src/cli.rs` | 150 | Erro de CLI (`invalid document ID`) | Manter — erro intencional |
| `04_wiring/src/main.rs` | várias | Erros de CLI e estatísticas do linter | Manter — intencional |

---

## 4. Validação

```bash
cat > /tmp/p651-columns.typ <<'EOF'
#set page(columns: 2)
#lorem(50)
EOF
./target/release/typst /tmp/p651-columns.typ /tmp/p651.pdf 2>&1 | grep -i "P627\|segments count"
```

Resultado: nenhum output de debug.

```bash
cargo test --workspace
```

Resultado: todos os crates passaram.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## 5. Decisão

- O único leftover de depuração do tipo "referência a passo" foi removido.
- Os outros `eprintln!` identificados são erros/aviso intencionais da CLI ou itens já priorizados por P650 para correcção futura.
