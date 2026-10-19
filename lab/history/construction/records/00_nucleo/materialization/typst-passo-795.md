---
# P795 — Math/symbol scope: modificadores de símbolo e identificadores em modo matemático

> **Passo:** 795
> **Data:** 2026-07-20
> **Foco:** P786 confirmou três divergências em `symbols`: (1) `#sym.subset.neq` erra (`unknown symbol modifier 'neq'`), enquanto o vanilla renderiza `⊊`; (2) `$ arrow.r $` dentro de modo matemático erra (`variável desconhecida: arrow`), enquanto o vanilla renderiza `→`; (3) `$ integral x dif x $` erra em `dif` (`unknown variable: dif` + 2 hints), enquanto o vanilla renderiza normalmente. Em contraste, `#sym.alpha #sym.arrow.r` em markup (fora de `$...$`) já funciona. Isso sugere que o problema é específico do caminho de resolução dentro de modo matemático, não do sistema de símbolos em si — possivelmente relacionado ao trabalho já feito em P780/P782 (resolução de identificador em modo math), a confirmar.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar se este é o mesmo mecanismo de P780/P782 ou uma lacuna distinta (cobertura de tabela de símbolos, não resolução de nome).
> **Prioridade:** Alta — símbolos matemáticos básicos (`arrow.r`, `dif`) são uso extremamente comum em modo matemático.
> **Dependências:** P786 (achado, evidência em `temp/temp_p786/a_symbols_neq.typ`, `a_symbols_math2.typ`, `a_symbols_math3.typ`), P780/P782 (mecanismo de resolução de identificador em modo math já reforçado).

---

## Passo 0 — Confirmar se é o mesmo mecanismo de P780/P782

```bash
cat > /tmp/p795-arrow.typ <<'EOF'
$ arrow.r $
EOF
./target/release/typst compile /tmp/p795-arrow.typ 2>&1
```

Se o erro for `unknown variable: arrow` (mesma família do que P780 corrigiu para variáveis de usuário): confirmar se `arrow` deveria resolver como símbolo (via `sym.arrow`, implicitamente) dentro de modo matemático — diferente de uma variável de usuário, mas usando caminho de resolução relacionado.

```bash
grep -n "\"arrow\"\|sym::.*scope\|math.*symbol.*scope" 01_core/src/rules/eval/math.rs 01_core/src/rules/stdlib/sym.rs 2>/dev/null
```

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "fn.*modifier\|SymbolModifier\|neq" lab/typst-original/crates/typst-library/src/foundations/symbols.rs 2>/dev/null
```

Confirmar:
1. Como modificadores de símbolo (`.neq`, `.r`, etc.) são resolvidos — mesmo mecanismo já usado para `sym.arrow.r.filled` (corrigido em P765a)?
2. Se `arrow`/`dif` dentro de `$...$` sem `sym.` são resolvidos por um escopo implícito de símbolos matemáticos comuns (diferente do escopo geral de variáveis).

```bash
cat > /tmp/p795-neq.typ <<'EOF'
#repr(sym.subset.neq)
EOF
lab/typst-original/target/release/typst compile /tmp/p795-neq.typ 2>&1
```

---

## Implementação

Conforme a sonda revelar — provavelmente duas causas relacionadas mas distintas:
1. Modificador `neq` ausente na tabela de variantes de `subset` (mesma família de trabalho de P765a/P766 — expandir cobertura de tabela de símbolos).
2. Resolução de símbolos comuns (`arrow`, `dif`, etc.) sem prefixo `sym.` dentro de modo matemático — confirmar se é um escopo implícito que precisa ser adicionado à resolução de `MathIdent` (relacionado a P780, mas mecanismo possivelmente distinto: é resolução de símbolo, não de variável de usuário).

---

## Validação

```bash
./target/release/typst compile /tmp/p795-neq.typ 2>&1
cat > /tmp/p795-test2.typ <<'EOF'
$ arrow.r $
EOF
./target/release/typst compile /tmp/p795-test2.typ 2>&1
cat > /tmp/p795-test3.typ <<'EOF'
$ integral x dif x $
EOF
./target/release/typst compile /tmp/p795-test3.typ 2>&1
```

Confirmar todos os três renderizando corretamente.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] Confirmado se compartilha mecanismo com P780/P782 ou é lacuna distinta.
- [x] Modificador `neq` (e outros ausentes, se a sonda revelar mais) implementado.
- [x] Resolução de símbolos comuns sem prefixo `sym.` em modo matemático implementada.
- [x] Os três casos de P786 corrigidos.
- [x] `cargo test --workspace` verde, contagem da suíte mostrada.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p795.md`, com comandos, saídas reais e nomes dos testes persistidos.

---

## Próximo passo

Sys.version/CLI, ênfase/fontes — restam da lista de P786 §5.
