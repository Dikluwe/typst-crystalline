---
# P688 — Retestar `cetz` com o padrão correcto da documentação e versão actual

> **Passo:** 688
> **Data:** 2026-07-10
> **Foco:** A documentação oficial de `cetz` mostra um padrão de uso diferente do usado em toda a cadeia P678-P687 (`#import "@preview/cetz:VERSAO"` + `import cetz.draw: *` dentro do `canvas`, chamando `line(...)` directamente), e uma versão muito mais recente (0.5.x) do que a usada até agora (0.2.2, provavelmente incompatível com Typst 0.15.0). Este passo repete a validação com a versão e o padrão correctos, para separar "cetz genuinamente incompatível" de "documento de teste mal construído desde o início".
> **Tipo:** Sonda directa.
> **Tamanho:** S-M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P678-P687 (toda a cadeia de validação com o documento de teste antigo).

---

## Sonda

### Descarregar a versão actual e testar com o padrão documentado

```bash
cat > /tmp/p688-cetz-correto.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
lab/typst-original/target/release/typst compile /tmp/p688-cetz-correto.typ /tmp/p688-vanilla.pdf
echo "Vanilla exit: $?"
mutool draw -o /tmp/p688-vanilla.png -r 150 /tmp/p688-vanilla.pdf
```

Confirmar, pela primeira vez nesta cadeia, se o vanilla consegue mesmo produzir um PDF visual completo com `cetz`, usando a versão e o padrão certos.

### Confirmar se o manifesto declara compatibilidade de compilador

```bash
cat ~/.cache/typst/packages/preview/cetz/0.5.2/typst.toml 2>/dev/null
cat ~/.cache/typst/packages/preview/cetz/0.2.2/typst.toml 2>/dev/null
```

Comparar o campo `compiler` (se existir) das duas versões, para confirmar se `0.2.2` declara compatibilidade só com versões antigas do Typst.

### Testar o cristalino com a versão e padrão correctos

```bash
./target/release/typst /tmp/p688-cetz-correto.typ /tmp/p688-cristalino.pdf
echo "Cristalino exit: $?"
mutool draw -o /tmp/p688-cristalino.png -r 150 /tmp/p688-cristalino.pdf
```

### Critério de fecho da sonda

- [ ] Confirmado se o vanilla produz PDF completo com a versão e padrão correctos.
- [ ] Confirmado se `0.2.2` tinha uma incompatibilidade de versão declarada no manifesto.
- [ ] Cristalino testado com o mesmo documento corrigido.

---

## Decisão

Se o vanilla conseguir renderizar com a versão/padrão correctos: comparar visualmente com o cristalino, e qualquer diferença agora é uma divergência real, digna de investigação — a validação de `cetz` finalmente fica num terreno comparável.

Se o cristalino falhar num ponto que o vanilla (agora com sucesso) não falha: esse é o próximo bloqueio real, não um artefacto do documento de teste mal construído.

Se mesmo com a versão/padrão correctos o vanilla continuar a falhar: a causa fica mais restrita — não é o documento de teste, é algo específico do ambiente ou de uma incompatibilidade genuína, a investigar à parte.

---

## Critério de fecho do passo

- [ ] Sonda completa, com a versão e padrão correctos testados nos dois lados.
- [ ] Se o vanilla funcionar: comparação visual directa entre cristalino e vanilla, com o próximo bloqueio real (se houver) identificado.
- [ ] Se o vanilla continuar a falhar mesmo assim: causa investigada à parte, não assumida.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p688.md`, com as imagens ou descrição suficiente.
- [ ] Estado da validação de `cetz` actualizado com precisão — não repetir a conclusão anterior ("vanilla também falha") sem a reconfirmar com o documento certo.
