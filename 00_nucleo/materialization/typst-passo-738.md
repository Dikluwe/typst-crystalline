---
# P738 — `FlowEvent::Return` condicional no fim de `while`/`for`

> **Passo:** 738
> **Data:** 2026-07-10
> **Foco:** P729 encontrou, por inspecção, que o vanilla marca `FlowEvent::Return` como condicional no fim de `while`/`for` (`flow.rs:105-108,183-185`), e que o cristalino só faz isso em `if`/`else` (P635), não em `while`/`for`. Sem caso medido com comportamento divergente — este passo confirma se existe um caso real antes de decidir corrigir.
> **Tipo:** Sonda directa. Implementação só se a sonda confirmar divergência real.
> **Tamanho:** XS para a sonda; S se confirmado.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P729 (onde o achado foi feito por inspecção, sem caso medido), P635 (mecanismo já correcto para `if`/`else`).

---

## Sonda

### Tentar construir um caso onde a diferença é observável

```bash
cat > /tmp/p738-return-loop.typ <<'EOF'
#let f() = {
  for i in (1, 2, 3) {
    if i == 2 {
      return "encontrado"
    }
  }
  "não encontrado"
}
#f()
EOF
lab/typst-original/target/release/typst compile /tmp/p738-return-loop.typ /tmp/p738-vanilla.pdf
pdftotext /tmp/p738-vanilla.pdf -
./target/release/typst /tmp/p738-return-loop.typ /tmp/p738-cristalino.pdf
pdftotext /tmp/p738-cristalino.pdf -
```

Tentar também com `while`, e com casos mais elaborados (return dentro de for aninhado, return condicional que às vezes não dispara).

### Ler o mecanismo vanilla com atenção para entender o que "condicional" realmente significa aqui

```bash
sed -n '95,115p;175,190p' lab/typst-original/crates/typst-eval/src/flow.rs 2>/dev/null
```

Confirmar o que este mecanismo faz exactamente — pode ser sobre um caso de borda mais subtil do que "return simples dentro de um loop", que já deve funcionar.

### Critério de fecho da sonda

- [ ] Caso real com comportamento divergente encontrado, ou confirmado que não existe nenhum caso alcançável por sintaxe de utilizador.

---

## Decisão

Se a sonda encontrar um caso real: implementar a correcção, seguindo o mecanismo do vanilla.

Se não encontrar nenhum caso divergente: manter como scope-out, mas com a razão agora confirmada por tentativa real, não só por inspecção — actualizar a lista de controlo com essa confirmação mais forte.

---

## Critério de fecho do passo

- [ ] Sonda completa, com tentativa real de reprodução, não só leitura de código.
- [ ] Se confirmado: implementado e testado.
- [ ] Se não confirmado: scope-out reforçado com a tentativa registada.
- [ ] `cargo test --workspace` sem regressão, se houver mudança de código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p738.md`.
- [ ] Item actualizado em `achados-adiados-cetz.md` (fechado, ou scope-out reforçado).
