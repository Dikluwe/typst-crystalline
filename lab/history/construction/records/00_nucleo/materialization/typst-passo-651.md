---
# P651 — Remover `eprintln!` de depuração esquecidos, e varrer por mais

> **Passo:** 651
> **Data:** 2026-07-09
> **Foco:** P650 encontrou um `eprintln!("P627 segments count: {}", ...)` esquecido em `content.rs`, disparando sempre que `#set page(columns: n)` é usado. É um resto de depuração de P627, nunca removido. Este passo remove-o, e varre o resto do código à procura de mais deixados para trás por outros passos — dado o volume desta conversa (650 passos), é provável que não seja o único.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P650 (onde o primeiro caso foi encontrado).

---

## Implementação imediata

### Remover o `eprintln!` de P627

```bash
sed -n '2305,2315p' 01_core/src/entities/content.rs
```

Confirmar o contexto exacto e remover a linha, sem afectar a lógica à volta.

---

## Sonda — procurar mais leftovers do mesmo tipo

### Varrer todo o código de produção por `eprintln!`/`println!` com referências a números de passo

```bash
grep -rn "eprintln!\|println!" 01_core/src/ 03_infra/src/ 02_shell/src/ 04_wiring/src/ --include="*.rs" | grep -v "test" | grep -iE "P[0-9]{2,3}"
```

Este padrão específico (menção a um número de passo dentro de uma mensagem de depuração) é um bom sinal de que a linha foi deixada por engano, não é um `eprintln!` intencional para produção.

### Varrer por `eprintln!`/`println!` em geral, para além dos já classificados por P650

```bash
grep -rn "eprintln!\|println!" 01_core/src/ 03_infra/src/ 02_shell/src/ 04_wiring/src/ --include="*.rs" | grep -v "test"
```

Comparar com a lista já classificada por P650 (5 casos, já com decisão própria) — confirmar se há mais além desses, não cobertos ainda.

### Critério de fecho da sonda

- [ ] Todo o código de produção varrido, não só os cinco casos já listados por P650.
- [ ] Cada ocorrência nova classificada: depuração esquecida (remover), ou aviso intencional a ser corrigido para chegar ao utilizador (tratar como os outros itens de P650).

---

## Validação

```bash
cat > /tmp/p651-columns.typ <<'EOF'
#set page(columns: 2)
#lorem(50)
EOF
./target/release/typst /tmp/p651-columns.typ /tmp/p651.pdf 2>&1 | grep -i "P627\|segments count"
```

Confirmar que a saída de depuração já não aparece.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `eprintln!` de P627 removido.
- [ ] Varredura completa por mais leftovers do mesmo tipo.
- [ ] Cada novo caso encontrado, classificado e tratado ou registado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p651.md`, com hash do commit.
