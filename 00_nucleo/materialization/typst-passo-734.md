---
# P734 — Restringir coordenadas de `polygon`/`curve` a `Length`/`Ratio`

> **Passo:** 734
> **Data:** 2026-07-10
> **Foco:** P732 encontrou que `polygon` (e, por partilhar `coord_component`, `curve`) aceita `Int`/`Float` como coordenada, que o vanilla rejeita ("expected relative length, found integer"). Confirmado como decisão deliberada de design do Typst (exigir unidade explícita para qualquer comprimento), não uma lacuna do vanilla — logo esta é uma divergência de linguagem a corrigir, seguindo a regra de P662-664 (nome igual obriga a comportamento igual).
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P732 (onde a divergência foi encontrada), P727 (`curve`, que partilha o mesmo `coord_component`).

---

## Sonda

### Confirmar exactamente o que o vanilla aceita como coordenada

```bash
cat > /tmp/p734-coord.typ <<'EOF'
#polygon((0pt, 0pt), (50pt, 0pt), (25pt, 40pt))
#polygon((0, 0), (50, 0), (25, 40))
#polygon((50%, 0pt), (0pt, 40pt), (25pt, 0pt))
EOF
lab/typst-original/target/release/typst compile /tmp/p734-coord.typ /tmp/p734-vanilla.pdf
echo "Exit code: $?"
```

Confirmar que a primeira funciona, a segunda (inteiros nus) falha, e confirmar se a terceira (percentagem) funciona — isso decide se o tipo aceite é só `Length` ou `Rel<Length>` (length + ratio).

### Confirmar consumidores reais de `cetz` que dependem da aceitação actual de Int/Float

```bash
grep -rn "polygon(\|curve(" ~/.cache/typst/packages/preview/cetz/0.5.2/src/*.typ ~/.cache/typst/packages/preview/cetz/0.5.2/src/**/*.typ 2>/dev/null | grep -v "\.pt\|\.mm\|\.cm\|\.em"
```

Confirmar se `cetz` de facto depende da aceitação de números nus, ou se sempre usa unidades — se `cetz` não depender, a correcção é segura sem risco de regressão na cadeia já fechada.

### Critério de fecho da sonda

- [ ] Tipo aceite pelo vanilla confirmado (`Length` só, ou `Rel<Length>`).
- [ ] Confirmado se `cetz` depende da aceitação actual de números nus.

---

## Implementação

Restringir `coord_component` (ou separar `polygon`/`curve` do caminho legado, se `curve` precisar de manter compatibilidade por outra razão) para rejeitar `Int`/`Float`, aceitando só o tipo confirmado pela sonda.

### Critério de fecho da implementação

- [ ] `Int`/`Float` como coordenada produzem erro, igual ao vanilla.
- [ ] `Length`/`Rel<Length>` (conforme confirmado) continuam a funcionar.
- [ ] `cetz` (cadeia P678-731, já fechada) sem regressão — repetir a reprodução final com diff de pixels.

---

## Validação

```bash
./target/release/typst /tmp/p734-coord.typ /tmp/p734-depois.pdf
echo "Exit code: $?"
```

```bash
cargo test --workspace
crystalline-lint .
```

### Confirmar que `cetz` continua a funcionar

```bash
cat > /tmp/p734-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p734-cetz.typ /tmp/p734-cetz.pdf
mutool draw -o /tmp/p734-cetz.png -r 150 /tmp/p734-cetz.pdf
```

Diff de pixels contra o vanilla — confirmar que continua próximo de zero (nível de anti-aliasing), sem regressão da cadeia já fechada.

---

## Critério de fecho do passo

- [ ] Sonda completa, tipo aceite confirmado.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` (cadeia já fechada) sem regressão, confirmado com diff de pixels.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p734.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
