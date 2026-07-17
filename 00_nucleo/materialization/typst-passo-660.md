---
# P660 — Implementar `variant: (eixo: valor)` para fontes variáveis

> **Passo:** 660
> **Data:** 2026-07-09
> **Foco:** P659 não conseguiu reproduzir a colisão de cache através de sintaxe real, porque o cristalino ainda não suporta `#text(font: (name: "...", variant: (wdth: 62.5)))` — só nomes de variante e os campos `weight`/`style`/`stretch` já conhecidos. Este passo implementa a forma explícita de eixos, permitindo aceder a eixos não-standard de fontes variáveis, e torna testável por documento real o que P659 só conseguiu confirmar por teste unitário.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P659 (onde a ausência foi encontrada), P525 (onde `axis_variations_for_font_variant` já existe, ligando peso/estilo a eixos OpenType internamente).

---

## Sonda

### Confirmar a sintaxe exacta do vanilla

```bash
cat > /tmp/p660-variant.typ <<'EOF'
#text(font: (name: "Noto Sans", variant: (wdth: 62.5)))[Texto estreito]
#text(font: (name: "Noto Sans", variant: (wdth: 125)))[Texto largo]
EOF
lab/typst-original/target/release/typst compile /tmp/p660-variant.typ /tmp/p660-vanilla.pdf
mutool draw -o /tmp/p660-vanilla.png -r 150 /tmp/p660-vanilla.pdf
```

Confirmar a estrutura exacta aceite (nomes de eixo, tipos de valor, se `wght`/`ital`/`slnt` também passam por aqui ou continuam a ter os seus próprios campos `weight`/`style`).

```bash
grep -n "variant\|FontVariant\|font.*dict" lab/typst-original/crates/typst-library/src/text/font/mod.rs 2>/dev/null | head -20
```

### Confirmar o estado actual do cristalino

```bash
grep -n "FontVariant\|font.*name.*variant\|\"variant\"" 01_core/src/rules/eval/rules.rs 01_core/src/entities/font*.rs 2>/dev/null | head -20
```

### Critério de fecho da sonda

- [ ] Sintaxe exacta do vanilla confirmada, incluindo tipos de valor aceites para cada eixo.
- [ ] Confirmado se eixos standard (`wght`, `wdth`, `ital`, `slnt`, `opsz`) e eixos personalizados (arbitrários, definidos pela fonte) são tratados da mesma forma ou de forma diferente.
- [ ] Estado actual do cristalino confirmado — onde a sintaxe `font: (name: ..., ...)` já é parseada, e onde falta o campo `variant`.

---

## Implementação

Adicionar reconhecimento do campo `variant` no dicionário de fonte, aceitando um dicionário de eixo→valor, e propagando isso até `axis_variations_for_font_variant` (ou o mecanismo equivalente que já liga peso/estilo a eixos OpenType desde P525), para que os valores explícitos de `variant` sejam usados directamente, em vez de derivados só de peso/estilo.

### Critério de fecho da implementação

- [ ] `#text(font: (name: "...", variant: (eixo: valor)))` reconhecido e aplicado.
- [ ] Testado com pelo menos dois eixos diferentes (`wdth`, e um segundo à escolha, `opsz` ou personalizado se a fonte de teste tiver).
- [ ] `weight`/`style`/`stretch` já existentes continuam a funcionar sem regressão, e a interagir correctamente com `variant` quando os dois forem usados juntos (confirmar qual tem prioridade, seguindo o vanilla).

---

## Validação

```bash
./target/release/typst /tmp/p660-variant.typ /tmp/p660-depois.pdf
mutool draw -o /tmp/p660-depois.png -r 150 /tmp/p660-depois.pdf
```

Comparar visualmente com a imagem do vanilla já obtida na sonda.

### Repetir P659 com sintaxe real, não só teste unitário

```bash
cat > /tmp/p660-cache-real.typ <<'EOF'
#set text(font: "Noto Sans Devanagari", size: 40pt)
#text(font: (name: "Noto Sans Devanagari", variant: (wdth: 62.5)))[नमस्ते संसार]
#text(font: (name: "Noto Sans Devanagari", variant: (wdth: 125)))[नमस्ते संसार]
EOF
./target/release/typst /tmp/p660-cache-real.typ /tmp/p660-cache.pdf
mutool draw -o /tmp/p660-cache.png -r 150 /tmp/p660-cache.pdf
```

Confirmar visualmente que as duas linhas têm larguras diferentes, provando com documento real (não só teste unitário) que a correcção de P659 continua correcta.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, sintaxe do vanilla confirmada.
- [ ] `variant: (eixo: valor)` implementado.
- [ ] Testado contra o vanilla, visualmente.
- [ ] P659 revalidado com documento real, não só teste unitário.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p660.md`, com hash do commit.
