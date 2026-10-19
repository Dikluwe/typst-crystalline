---
# P743 — O valor devolvido por `return` após conteúdo bate certo, não só o aviso?

> **Passo:** 743
> **Data:** 2026-07-10
> **Foco:** P738 confirmou que o aviso "this return unconditionally discards the content before it" está ausente no cristalino, mas nunca confirmou se o **valor devolvido** por `{ [conteúdo] return "x" }` é o mesmo nos dois compiladores — só que o aviso não aparece. Se o mecanismo por trás do aviso (descarte do `join` acumulado ao encontrar `return`) não estiver implementado correctamente, isto pode ser um comportamento divergente escondido atrás de "só falta um aviso".
> **Tipo:** Verificação directa. Prioridade alta — pode revelar um bug de valor, não só cosmético.
> **Tamanho:** XS-S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P738 (onde só o aviso foi verificado, não o valor), P728 (mecanismo de `join`, potencialmente relevante aqui).

---

## Verificação

### Confirmar o valor devolvido, não só a presença do aviso

```bash
cat > /tmp/p743-return-valor.typ <<'EOF'
#let f() = {
  [conteúdo antes]
  return "valor de retorno"
}
#f()
EOF
lab/typst-original/target/release/typst compile /tmp/p743-return-valor.typ /tmp/p743-vanilla.pdf 2>&1
pdftotext /tmp/p743-vanilla.pdf -
```

Confirmar exactamente o que o vanilla produz — o conteúdo desaparece do output final, ou aparece antes de "valor de retorno"?

```bash
./target/release/typst /tmp/p743-return-valor.typ /tmp/p743-cristalino.pdf 2>&1
pdftotext /tmp/p743-cristalino.pdf -
```

Comparar directamente — não assumir que bate certo só porque P738 disse "sem warning, exit 0".

### Testar casos adicionais que exercitam o mecanismo de descarte

```bash
cat > /tmp/p743-return-tipos.typ <<'EOF'
#let f() = {
  (1, 2, 3)
  return "x"
}
#f()

#let g() = {
  "texto acumulado"
  return 42
}
#g()
EOF
lab/typst-original/target/release/typst compile /tmp/p743-return-tipos.typ /tmp/p743-tipos-vanilla.pdf 2>&1
./target/release/typst /tmp/p743-return-tipos.typ /tmp/p743-tipos-cristalino.pdf 2>&1
```

Confirmar se algum destes produz erro num dos lados (por exemplo, se o cristalino tentar fazer `join` do array/string acumulado com o valor de retorno, e isso der erro "cannot join X with Y", enquanto o vanilla simplesmente descarta sem tentar juntar).

### Localizar o mecanismo exacto no código

```bash
grep -n "FlowEvent::Return\|fn eval_return" 01_core/src/engine/eval/*.rs
```

Confirmar se o `return` dentro de um bloco descarta correctamente o valor acumulado até esse ponto, ou se tenta combiná-lo de alguma forma com o valor do `return`.

### Critério de fecho da verificação

- [ ] Valor devolvido confirmado como idêntico nos dois lados, para os três casos testados.
- [ ] Confirmado se o mecanismo de descarte está correctamente implementado, ou se há uma tentativa incorrecta de `join`.

---

## Decisão

Se o valor bater certo nos três casos: confirma que o problema é mesmo só o aviso ausente (cosmético, já correctamente classificado como baixa prioridade em P738). Actualizar `achados-adiados-cetz.md` com esta confirmação mais forte.

Se o valor divergir nalgum caso: é um bug de valor, não cosmético — corrigir com prioridade alta, e reclassificar a entrada correspondente na lista de controlo.

---

## Critério de fecho do passo

- [ ] Verificação completa, valor confirmado (ou divergência real encontrada).
- [ ] Se confirmado correcto: lista de controlo actualizada com a confirmação.
- [ ] Se divergência encontrada: corrigida, testada, e classificação da lista de controlo revista.
- [ ] `cargo test --workspace` sem regressão, se houver mudança de código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p743.md`.
