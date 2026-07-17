---
# P708 — Corrigir binding de argumentos: parâmetros keyword-only não devem consumir posicionais

> **Passo:** 708
> **Data:** 2026-07-10
> **Foco:** P707 descobriu que `apply_closure` trata parâmetros posicionais e parâmetros `nome: default` (keyword-only no vanilla) de forma idêntica — ambos consomem `args.items` (posicionais) se não vierem por nome. Isto causa dois problemas: (1) valores posicionais perdidos silenciosamente quando há um parâmetro keyword-only a meio, e (2) argumentos extra aceites silenciosamente, corrompendo o tipo do parâmetro keyword-only (`close: false` vira `close: 3`, um `Int`, sem erro). Não é específico de `cetz` — afecta qualquer closure do utilizador com este padrão de assinatura. Prioridade máxima.
> **Tipo:** Sonda + Implementação. Prioridade máxima, alcance potencialmente amplo.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mudança no mecanismo central de chamada de funções; sonda obrigatória e cuidado redobrado, dado o alcance.

---

## Contexto

Código actual (`01_core/src/rules/eval/closures.rs`, confirmado por P707):

```rust
let mut pos_idx = 0;
for param in closure.params.iter() {
    let val = if let Some(v) = args.named.get(param.name.as_str()) {
        v.clone()
    } else if let Some(v) = args.items.get(pos_idx) {
        pos_idx += 1;
        v.clone()
    } else {
        param.default.clone().unwrap_or(Value::None)
    };
    call_scopes.define(param.name.as_str(), val);
}
```

`ClosureParam` não distingue parâmetro posicional de parâmetro `nome: default` (keyword-only). O vanilla distingue os dois na sintaxe (`nome` vs `nome: valor` na assinatura) e trata-os de forma diferente na chamada.

---

## Sonda

### Confirmar a distinção exacta positional vs keyword-only no vanilla, ao nível do AST

```bash
grep -n "struct Param\|enum.*Param\|Positional\|Named" lab/typst-original/crates/typst-syntax/src/ast.rs | head -20
```

Confirmar se a distinção já existe sintacticamente no parser do cristalino (`entities/ast/expr.rs`, nó `Param`), ou se precisa de ser introduzida também aí.

### Confirmar o comportamento completo do vanilla, com mais casos de borda

```bash
cat > /tmp/p708-binding.typ <<'EOF'
#let f(a, b, close: false) = (a, b, close)
#f(1, 2)
#f(1, 2, close: true)
EOF
lab/typst-original/target/release/typst compile /tmp/p708-binding.typ /tmp/p708-vanilla-ok.pdf
pdftotext /tmp/p708-vanilla-ok.pdf -

cat > /tmp/p708-erro.typ <<'EOF'
#let f(a, b, close: false) = (a, b, close)
#f(1, 2, 3)
EOF
lab/typst-original/target/release/typst compile /tmp/p708-erro.typ /tmp/p708-vanilla-erro.pdf 2>&1
```

Confirmar a mensagem de erro exacta para "argumento posicional extra sem posição correspondente".

### Reproduzir os dois casos mínimos já identificados por P707

```bash
cat > /tmp/p708-sink.typ <<'EOF'
#let f(..args, close: false) = args.pos().len()
#f(1, 2, 3)
EOF
lab/typst-original/target/release/typst compile /tmp/p708-sink.typ /tmp/p708-sink-vanilla.pdf
pdftotext /tmp/p708-sink-vanilla.pdf -
./target/release/typst /tmp/p708-sink.typ /tmp/p708-sink-cristalino.pdf
pdftotext /tmp/p708-sink-cristalino.pdf -
```

### Critério de fecho da sonda

- [ ] Distinção positional/keyword-only confirmada ao nível do AST do vanilla.
- [ ] Confirmado se essa distinção já existe no parser do cristalino, ou precisa de ser introduzida.
- [ ] Mensagem de erro exacta para argumento extra confirmada.
- [ ] Os dois casos mínimos de P707 reproduzidos e confirmados como ainda presentes.

---

## Implementação

Seguindo o plano já proposto por P707:

1. Adicionar a `ClosureParam` (ou equivalente) um marcador de tipo (`Positional | NamedWithDefault`, ou usar a distinção já existente no AST se P708 confirmar que já lá está).
2. Reescrever o binding em `apply_closure` em duas fases: parâmetros posicionais consomem `args.items` em ordem; parâmetros nomeados-com-default só olham para `args.named`, nunca `args.items`.
3. Confirmar que argumento posicional sem posição correspondente produz erro ("unexpected argument" ou equivalente), não aceitação silenciosa.

### Critério de fecho da implementação

- [ ] Os dois casos mínimos de P707 corrigidos, testados contra o vanilla.
- [ ] Argumento extra sem posição produz erro claro, não aceitação silenciosa.
- [ ] Closures já existentes, sem este padrão de assinatura, sem regressão.
- [ ] Varredura ampla: correr todo o corpus de testes já usado ao longo desta conversa (não só os testes unitários novos), dado o alcance potencial deste bug.

---

## Validação

```bash
./target/release/typst /tmp/p708-binding.typ /tmp/p708-depois-ok.pdf
pdftotext /tmp/p708-depois-ok.pdf -
./target/release/typst /tmp/p708-erro.typ /tmp/p708-depois-erro.pdf
echo "Exit code: $?"
./target/release/typst /tmp/p708-sink.typ /tmp/p708-depois-sink.pdf
pdftotext /tmp/p708-depois-sink.pdf -
```

Comparar todos com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

Dado o alcance potencial deste bug (qualquer closure do utilizador com este padrão), correr o corpus de benchmark e quaisquer outros documentos de teste já usados ao longo desta conversa, para confirmar que nenhum foi afectado silenciosamente até agora sem ninguém ter reparado.

### Repetir a reprodução de P700-707

```bash
cat > /tmp/p708-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p708-cetz.typ /tmp/p708-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p708-cetz.png -r 150 /tmp/p708-cetz.pdf 2>/dev/null
```

---

## Critério de fecho do passo

- [ ] Sonda completa, distinção positional/keyword-only confirmada ao nível do AST.
- [ ] Binding corrigido em duas fases, testado contra os casos de P707 e os novos casos de borda.
- [ ] Argumento extra produz erro, não aceitação silenciosa.
- [ ] Varredura ampla do corpus de testes já existente, confirmando que não havia mais casos afectados sem ninguém ter reparado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — sucesso completo com comparação visual, ou próximo bloqueio registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p708.md`, com hash do commit.
