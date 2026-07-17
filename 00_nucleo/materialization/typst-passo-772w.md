---
# P772w — Classificação do resíduo de risco plausível: `target_`, `plugin_`, `image::pdf`, `layout::frame`, `math`

> **Passo:** 772w
> **Data:** 2026-07-17
> **Foco:** P772t identificou 23 itens em 5 módulos como o resíduo de maior risco linguístico plausível dentro do que restou de `lacuna-inventario` depois de excluir mecanismo puro de parsing/CST/utils. Este é um passo de classificação (como P772a-m), não um sweep amplo — o volume é pequeno o suficiente para cobrir os 5 módulos num só passo.
> **Tipo:** Sonda + Implementação directa para achados confirmados.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0107** — cada item avaliado por efeito observável real, não por nome do módulo.
> **Dependências:** P772t (priorização), toda a metodologia já estabelecida em P765a-P772v.

---

## Sonda — os 5 módulos

```bash
for mod in "foundations::target_" "foundations::plugin_" "image::pdf" "layout::frame" "^typst_library::math::"; do
  echo "=== $mod ==="
  awk -F'\t' -v m="$mod" '$1=="lacuna-inventario" && $5 ~ m' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt | cut -f5
done
```

### `foundations::target_` (6 itens) — link targets

```bash
grep -n "<item>" lab/typst-original/crates/typst-library/src/foundations/target.rs 2>/dev/null
```

Testar efeito observável real: `<label>`/refs internos ao PDF (bookmarks, links clicáveis para `#ref()`).

```bash
cat > /tmp/p772w-target-test.typ <<'EOF'
= Heading <mylabel>
See @mylabel
EOF
```

Comparar via `pdfimages`/`pdftotext -layout`/extração de anotações de link do PDF (`mutool show <pdf> trailer` ou `pdfinfo -js`), não só compilação sem erro — links internos são um observável do PDF, não do texto renderizado.

### `foundations::plugin_` (5 itens) — plugins WASM

```bash
grep -n "<item>" lab/typst-original/crates/typst-library/src/foundations/plugin.rs 2>/dev/null
```

Contexto: plugins WASM já foram trabalhados extensivamente (P678-762, handoff). Confirmar se estes 5 itens são já cobertos por esse trabalho (lacuna de granularidade do inventário) ou são um ângulo não tocado.

### `image::pdf` (4 itens) — embutir PDF como imagem

```bash
cat > /tmp/p772w-pdf-image-test.typ <<'EOF'
#image("test.pdf")
EOF
```

Confirmar se o cristalino suporta `#image()` com um PDF como fonte (o vanilla suporta desde uma versão recente — confirmar se está no escopo de 0.15.0).

### `layout::frame` (4 itens) — núcleo de layout

```bash
grep -n "<item>" lab/typst-original/crates/typst-library/src/layout/frame.rs 2>/dev/null
```

Atenção: "núcleo de layout" pode ser nome genérico que na prática é mecânica interna (estrutura de dados `Frame`), não símbolo de língua — não assumir risco alto só pelo nome, confirmar com leitura.

### `math` (4 itens)

```bash
awk -F'\t' '$1=="lacuna-inventario" && $5 ~ /^typst_library::math::/' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt | grep -v "::style::"
```

Confirmar que não se sobrepõe ao que P765b (math::style) já cobriu.

---

## Implementação

Só para achados confirmados como bug real, com caso de teste mínimo comparando saída/comportamento contra o vanilla. Corrigir um a um.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Os 23 itens (5 módulos) classificados item a item.
- [ ] `target_`: testado com efeito observável real (links/bookmarks no PDF, não só compilação).
- [ ] `plugin_`: confirmado se é granularidade do trabalho já feito ou ângulo novo.
- [ ] `image::pdf`: confirmado se está no escopo do vanilla 0.15.0 antes de julgar como lacuna.
- [ ] `layout::frame`: confirmado se é mecânica interna ou símbolo de língua antes de classificar como alto risco.
- [ ] `math`: confirmado sem sobreposição com P765b.
- [ ] Bugs reais corrigidos com teste comparando comportamento/saída.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772w.md`, com a tabela de classificação completa.

---

## Próximo passo

Com este passo, o resíduo de risco plausível identificado por P772t está coberto. Avaliar se resta algo que justifique continuar a série P765a-P772w, ou se é o ponto natural de encerrar a varredura sistemática com um resumo final.
