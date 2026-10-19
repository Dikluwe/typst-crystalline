---
# P780 — `MathIdent` bare não resolve variável do utilizador em modo matemático

> **Passo:** 780
> **Data:** 2026-07-17
> **Foco:** P772y encontrou, como efeito colateral da validação de `math.class()`, que um identificador multi-letra em modo matemático (`$x loves y$`, onde `loves` é uma variável definida via `#let`) não resolve para o valor vinculado — renderiza como texto literal, glifo a glifo (`l`,`o`,`v`,`e`,`s`). `Expr::MathIdent` (arm principal do avaliador, distinto do caminho de callee que P772y já corrigiu) só resolve símbolos Unicode/operadores `math` conhecidos; qualquer outro nome, mesmo vinculado no scope a `Content`/`Value`, cai sempre em `Content::MathIdent(name)` como texto. Este passo é o primeiro dos três débitos deixados por P772y — abre uma nova linha de numeração (P780+) por decisão do utilizador, dado o volume acumulado da série P765a-P772y.
> **Tipo:** Sonda + Implementação directa (causa já apontada por P772y, mecanismo a confirmar em detalhe).
> **Tamanho:** M/L — mexe em `eval_math_expr`, área já identificada como sensível (P772l §2.5, `unknown_variable_math`, debt de P301 nunca fechado — este passo pode sobrepor-se a essa frente antiga; confirmar antes de duplicar).
> **ADR-0108 EM VIGOR** — confirmar o mecanismo exato do vanilla (`get_in_math`) antes de implementar, não assumir da descrição de P301/P772l.
> **Dependências:** P772y (achado, correção irmã já feita para callees namespaced), P772l §2.5 e o diagnóstico `diagnostico-auto-lookup-math-passo-301.md` (debt antigo relacionado — confirmar sobreposição antes de prosseguir).

---

## Passo 0 — Confirmar que este não é apenas o debt de P301 já catalogado

```bash
cat 00_nucleo/diagnosticos/diagnostico-auto-lookup-math-passo-301.md 2>/dev/null | head -60
```

P772l (§2.5) já registou este debt como "reconfirmado, não novo" — ligado a P301. Confirmar se o achado de P772y é exatamente o mesmo caso (`$x$` com variável definida) ou se há alguma diferença de superfície que P772y expôs e P301/P772l não tinham medido (ex: interação com `math.class`/callees namespaced especificamente). Se for exatamente o mesmo: este passo é a implementação que fecha o debt de P301, não um achado novo — tratar como tal na numeração e nas referências do relatório final.

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "fn.*get_in_math\|MathIdent" lab/typst-eval/src/math.rs 2>/dev/null
```

Confirmar (P772l §2.5 já tinha uma leitura parcial disto — revalidar, não copiar):
1. `get_in_math` é chamado incondicionalmente para todo `MathIdent`, antes de qualquer fallback simbólico?
2. A fronteira entre "letra única sempre simbólica" (ex: `$r$` pode renderizar como itálico matemático mesmo com `r` definido no scope) vs "identificador multi-carácter sempre via scope" — P772l não tinha certeza suficiente disso; medir directamente.

```bash
cat > /tmp/p780-mathident-r.typ <<'EOF'
#let r = [texto]
$ r $
EOF
cat > /tmp/p780-mathident-multi.typ <<'EOF'
#let myvar = [texto]
$ myvar $
EOF
lab/typst-original/target/release/typst compile /tmp/p780-mathident-r.typ 2>&1
lab/typst-original/target/release/typst compile /tmp/p780-mathident-multi.typ 2>&1
```

Confirmar o comportamento exato do vanilla para os dois casos — letra única vs multi-carácter — antes de decidir a regra a implementar.

### Caso de erro — identificador desconhecido em modo math

```bash
cat > /tmp/p780-unknown.typ <<'EOF'
$ foobarbaz $
EOF
lab/typst-original/target/release/typst compile /tmp/p780-unknown.typ 2>&1
```

P772l já mediu isto: vanilla dá `error: unknown variable: foobarbaz` + 2 hints. Confirmar se ainda é esse o comportamento e replicar junto com a resolução de variável (mesma função, `get_in_math`, cobre os dois casos).

---

## Implementação

1. Em `eval_math_expr` (`01_core/src/rules/eval/math.rs`), antes do fallback para símbolo/operador conhecido, consultar o scope real (`scopes.get`/equivalente) para o identificador — replicando a fronteira confirmada pela sonda (letra única vs multi-carácter, ou outra regra se a medição revelar algo diferente).
2. Se encontrado no scope: usar o valor vinculado (Content/Value convertido para Content, mesma conversão já usada no caminho de callee corrigido por P772y).
3. Se não encontrado: erro `unknown variable` com os hints corretos (mesmo padrão já implementado por P772r para o caso de texto normal — reutilizar `unknown_variable`/`SourceDiagnostic::with_hint` se a mensagem for idêntica).

---

## Validação

```bash
cat > /tmp/p780-test1.typ <<'EOF'
#let myvar123 = 5
$ myvar123 $
EOF
./target/release/typst compile /tmp/p780-test1.typ 2>&1
```

Confirmar que mostra `5`, não `myvar123` literal.

```bash
cat > /tmp/p780-test2.typ <<'EOF'
$ foobarbaz $
EOF
./target/release/typst compile /tmp/p780-test2.typ 2>&1
```

Confirmar erro com hints, não sucesso silencioso.

```bash
# Não regressão — símbolos de letra única continuam simbólicos mesmo se definidos
cat > /tmp/p780-test3.typ <<'EOF'
#let r = [nunca deve aparecer]
$ r $
EOF
./target/release/typst compile /tmp/p780-test3.typ 2>&1
```

Confirmar que bate com o comportamento do vanilla medido no Passo de sonda (provavelmente continua símbolo itálico, não o valor de `r`).

```bash
cargo test --workspace
crystalline-lint .
```

Reconfirmar testes de math existentes (P299-301, P765b, P772w, P772y) sem regressão.

---

## Critério de fecho do passo

- [ ] Confirmado se este é o mesmo debt de P301 (provavelmente sim) — referenciado como tal, não tratado como achado novo duplicado.
- [ ] Fronteira letra-única vs multi-carácter confirmada contra o vanilla real, não assumida.
- [ ] `get_in_math` (ou equivalente) implementado, consultando o scope antes do fallback simbólico.
- [ ] Erro `unknown variable` com hints para identificador realmente desconhecido.
- [ ] Variável multi-carácter resolve corretamente.
- [ ] Letra única continua simbólica mesmo se definida no scope (comportamento confirmado do vanilla).
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de eval matemático atualizado antes do código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p780.md`.

---

## Próximo passo

Os outros dois débitos de P772y: splice de `#expr` em modo math (pode ter causa relacionada, confirmar sobreposição antes de tratar como independente), e cobertura de glifo `♥`/Unicode fora do alfabeto comum (não relacionado a modo math, gap de fonte fallback). Mais os débitos antigos de P772w (`image::pdf`, fallback de fontes matemáticas).
