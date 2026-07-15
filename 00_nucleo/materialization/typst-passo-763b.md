---
# P763b — Validação ponta a ponta: `cetz` com cache totalmente vazia

> **Passo:** 763b
> **Data:** 2026-07-15
> **Foco:** P763a implementou e validou o download com um pacote isolado (`@preview/fletcher:0.5.4`). Este passo fecha o ciclo indicado no próprio relatório de P763a: testar um pacote com dependências `@preview` encadeadas — `cetz`, que já teve paridade de pixels exacta (AE=0) confirmada em P678–P762, mas sempre com a cache pré-populada manualmente. Este é o primeiro teste real de `cetz` sem essa muleta.
> **Tipo:** Validação (teste de aceitação). Sem código novo esperado — só corre se P763a revelar um bug não capturado pelos casos isolados de P763/P763a (dependências transitivas, versões múltiplas do mesmo pacote pedidas por dependências diferentes, etc.).
> **Tamanho:** S — validação; vira Implementação só se algo falhar.
> **ADR-0108 EM VIGOR** — medir antes de declarar fechado.
> **Dependências:** P763a (download implementado e validado isoladamente).

---

## Sonda / Validação

```bash
# Limpar TUDO — cache e data dir, não só o pacote de teste
rm -rf ~/.cache/typst/packages
rm -rf ~/.local/share/typst/packages
```

```bash
cat > /tmp/p763b-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst compile /tmp/p763b-cetz.typ /tmp/p763b-cetz.pdf
```

Confirmar:
1. Compila sem intervenção manual, com cache vazia.
2. Todas as dependências transitivas de `cetz` (se houver — confirmar com `cat ~/.cache/typst/packages/preview/cetz/0.5.2/typst.toml`) são descarregadas automaticamente, não só `cetz` em si.
3. Tempo de compilação — se `cetz` chamar plugins WASM (P698/P699) múltiplas vezes, confirmar que o download não é repetido a cada chamada (deve descarregar uma vez, não uma vez por uso).

### Comparação de paridade visual (repetir a medição de P762, agora com cache vazia)

```bash
lab/typst-original/target/release/typst compile /tmp/p763b-cetz.typ /tmp/p763b-cetz-vanilla.pdf
mutool draw -o /tmp/p763b-vanilla.png -r 300 /tmp/p763b-cetz-vanilla.pdf
mutool draw -o /tmp/p763b-cristalino.png -r 300 /tmp/p763b-cetz.pdf
compare -metric AE /tmp/p763b-vanilla.png /tmp/p763b-cristalino.png /tmp/p763b-diff.png
```

Esperado: AE=0, mesma paridade já confirmada em P762 — este passo não deve introduzir regressão visual, só confirma que o caminho de aquisição do pacote (antes manual, agora automático) não muda o resultado.

---

## Critério de fecho do passo

- [ ] Cache e data dir completamente vazios antes do teste (confirmado, não assumido).
- [ ] `cetz` compila sem intervenção manual.
- [ ] Dependências transitivas (se existirem) confirmadas como descarregadas automaticamente.
- [ ] Paridade visual (AE) igual à já confirmada em P762 — sem regressão.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p763b.md`.

---

## Próximo passo

Se tudo passar: a linha de trabalho de download de pacotes (P763, P763a, P763b) fecha. Se algo falhar (dependências transitivas, cache concorrente, etc.): abrir P763c com o achado específico, não expandir o âmbito deste passo.
