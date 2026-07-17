---
# P616 — Rejeitar `dir: ttb`/`btt` em texto, como o vanilla

> **Passo:** 616
> **Data:** 2026-07-05
> **Foco:** P614 encontrou que o cristalino aceita `#set text(dir: ttb)` sem erro nem efeito — o texto continua horizontal, sem aviso ao utilizador. O vanilla rejeita explicitamente com "text direction must be horizontal". Isto é diferente de "escrita vertical ausente" (que é falta de funcionalidade em ambos) — é um caso de erro engolido em silêncio, que deve ser corrigido independentemente de quando (ou se) a escrita vertical vier a ser implementada.
> **Tipo:** Implementação directa. Pequeno, a causa já está confirmada por P614.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P614 (onde o comportamento foi encontrado).

---

## Contexto

`#set text(dir: ...)` no cristalino (implementado em P576) aceita qualquer valor de `Dir`, incluindo `Ttb`/`Btt`, sem verificar se faz sentido para texto. O vanilla valida e rejeita, porque a escrita vertical de texto não está implementada — um erro claro é melhor do que aceitar e ignorar silenciosamente.

---

## Implementação

Em `01_core/src/rules/eval/rules.rs`, no arm `"dir"` de `#set text(...)` (introduzido em P576), adicionar validação: se o valor for `Dir::Ttb` ou `Dir::Btt`, devolver erro, com mensagem igual ou equivalente à do vanilla.

```rust
// Esboço, a confirmar contra a estrutura real:
"dir" => {
    let dir = /* parse do valor */;
    if matches!(dir, Dir::Ttb | Dir::Btt) {
        return Err(/* erro: "text direction must be horizontal" ou equivalente */);
    }
    // continuar como antes para Ltr/Rtl
}
```

### Critério de fecho da implementação

- [ ] `#set text(dir: ttb)` produz erro claro, não aceitação silenciosa.
- [ ] `#set text(dir: btt)` produz o mesmo erro.
- [ ] `#set text(dir: ltr)` e `#set text(dir: rtl)` continuam a funcionar sem regressão (toda a sequência RTL depende disto).

---

## Validação

```bash
cat > /tmp/p616-ttb.typ <<'EOF'
#set text(dir: ttb)
Texto.
EOF
./target/release/typst /tmp/p616-ttb.typ /tmp/p616.pdf
echo "Exit code: $?"
```

Confirmar que produz erro, não um PDF.

```bash
cat > /tmp/p616-rtl.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 20pt)
مرحبا
EOF
./target/release/typst /tmp/p616-rtl.typ /tmp/p616-rtl.pdf
```

Confirmar que RTL continua a funcionar sem regressão.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `dir: ttb`/`btt` em texto produz erro, igual ao vanilla.
- [ ] `dir: ltr`/`rtl` sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p616.md`, com hash do commit.
- [ ] Item de escrita vertical na lista de disparidades actualizado: de "ausente, disparidade" para "ausente em ambos, não é disparidade — funcionalidade nova, ver mapa de P614 se avançar no futuro".
