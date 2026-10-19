---
# P772r — Hint de subtração em `unknown_variable`

> **Passo:** 772r
> **Data:** 2026-07-17
> **Foco:** P772l (§2.4) confirmou que `#foo-bar` dá `"unknown variable: foo-bar"` no cristalino e vanilla — mensagem base idêntica —, mas o vanilla acrescenta `hint: if you meant to use subtraction, try adding spaces around the minus sign: foo - bar`. O L0 vigente (`00_nucleo/prompts/engine/eval.md`, linha ~1191) documenta hoje a ausência do hint como comportamento aceite — este passo atualiza esse L0 antes de implementar (mudança de contrato documentado, não bug contra L0 já aprovado). A infraestrutura para hints já existe (`SourceDiagnostic::with_hint`, usada em `missing_key` no mesmo ficheiro).
> **Tipo:** Atualização de L0 + Implementação directa (infraestrutura já existe, custo baixo).
> **Tamanho:** S.
> **ADR-0108 EM VIGOR** — confirmar a heurística exata do vanilla para decidir quando o hint aparece, não assumir "sempre que há hífen".
> **Dependências:** P772l (achado original).

---

## Sonda — heurística exata do vanilla

```bash
grep -n "if you meant to use subtraction\|fn unknown_variable\b" lab/typst-original/crates/typst-library/src/foundations/scope.rs lab/typst-eval/src/*.rs 2>/dev/null
```

Confirmar:
1. A condição exata para o hint aparecer — é qualquer identificador com hífen que falha, ou há uma verificação adicional (ex: confirmar que as duas partes ao redor do hífen são, sozinhas, identificadores válidos ou nomes conhecidos)?
2. Se o hint aparece para hífens múltiplos (`#foo-bar-baz`) e qual o texto sugerido nesse caso.
3. Texto exato do hint, incluindo pontuação e formatação.

```bash
cat > /tmp/p772r-test1.typ <<'EOF'
#foo-bar
EOF
lab/typst-original/target/release/typst compile /tmp/p772r-test1.typ 2>&1

cat > /tmp/p772r-test2.typ <<'EOF'
#foo-bar-baz
EOF
lab/typst-original/target/release/typst compile /tmp/p772r-test2.typ 2>&1

cat > /tmp/p772r-test3.typ <<'EOF'
#simplyunknown
EOF
lab/typst-original/target/release/typst compile /tmp/p772r-test3.typ 2>&1
```

Confirmar que o terceiro caso (sem hífen) não ganha hint nenhum — só a mensagem base.

---

## Implementação

### 1. Atualizar L0

`00_nucleo/prompts/engine/eval.md`, linha ~1191: remover a nota que documenta ausência de hint como aceite; substituir pela especificação do hint (condição exata confirmada pela sonda, texto exato).

### 2. Código

`01_core/src/engine/eval/bindings.rs` (ou onde `unknown_variable` é construído): usar `SourceDiagnostic::with_hint`, replicando o padrão já usado em `missing_key` no mesmo ficheiro. Condição replicando exatamente a heurística confirmada.

---

## Validação

```bash
./target/release/typst compile /tmp/p772r-test1.typ 2>&1
./target/release/typst compile /tmp/p772r-test2.typ 2>&1
./target/release/typst compile /tmp/p772r-test3.typ 2>&1
```

Confirmar texto do hint idêntico ao vanilla no primeiro caso, comportamento correto (com ou sem hint, conforme a sonda confirmar) no segundo, e ausência de hint no terceiro.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Heurística exata do vanilla confirmada (condição para o hint aparecer).
- [ ] Texto do hint confirmado palavra por palavra.
- [ ] L0 atualizado antes do código.
- [ ] Hint implementado usando `SourceDiagnostic::with_hint` já existente.
- [ ] Caso com hífen dá hint; caso sem hífen não dá.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772r.md`.

---

## Próximo passo

Item novo identificado por P772p: lacuna geral de span em `Args`/chamada de funções nativas (afeta mensagens de erro de argumento de qualquer função nativa, não só `image()`). Depois, reconfirmar `lacuna-inventario` (estilo P772e, segunda rodada) para decidir se a varredura sistemática continua.
