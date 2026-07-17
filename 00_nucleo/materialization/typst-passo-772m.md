---
# P772m — Varredura da stdlib: `typst_library::text::font::*`

> **Passo:** 772m (continuação da série P765a→P772l)
> **Data:** 2026-07-16
> **Foco:** Classificar `text::font::*` (~22 itens: variations, metrics, info, book, exceptions, case — confirmar a lista exacta e submódulos). Maior módulo restante de alto risco — tipografia tem efeito observável directo, e esta conversa já viu bugs graves de fontes (fontes variáveis, P525→P666-669; fonte por defeito errada, P753; modelo de avanço entre linhas, P761-762). Aplicar a mesma disciplina de coordenadas/medição já estabelecida nessas correcções anteriores.
> **Tipo:** Sonda + Implementação directa para achados confirmados.
> **Tamanho:** L — módulo maior da lista restante, tipografia é área historicamente propensa a bugs nesta conversa.
> **ADR-0108 EM VIGOR.** **ADR-0107** — métricas de fonte com efeito no layout renderizado são sempre candidatas a bug real.
> **Dependências:** P772l (lote anterior), P666-669/P753/P761-762 (contexto de bugs de fonte já corrigidos, para não duplicar).

---

## Sonda — classificar os itens

```bash
awk -F'\t' '$1=="lacuna-inventario" && $5 ~ /text::font/' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt | cut -f5 | sort
```

Confirmar os submódulos exactos (`variations`, `metrics`, `info`, `book`, `exceptions`, `case` — a lista do prompt é uma hipótese a partir da recontagem de P772e, confirmar contra a lista real).

```bash
grep -n "<item>" lab/typst-original/crates/typst-library/src/text/font/*.rs 2>/dev/null
grep -rn "<item>" 01_core/src/entities/*.rs 03_infra/src/font_metrics.rs 03_infra/src/embedded_fonts.rs 2>/dev/null
```

### Atenção especial a `exceptions` — provável mecanismo de casos especiais por família de fonte

Se o vanilla tiver uma tabela de excepções por fonte (comportamento especial para fontes conhecidas problemáticas), confirmar se o cristalino replica isso ou ignora — pode ser a causa de divergências já observadas noutros passos sem explicação (ex: o resíduo mecânico de P772j, ou outros "residuais" não totalmente explicados ao longo desta conversa).

```bash
grep -n "FONT_EXCEPTIONS\|exception" lab/typst-original/crates/typst-library/src/text/font/exceptions.rs 2>/dev/null | head -20
```

### Casos de teste — métricas com efeito directo

```bash
cat > /tmp/p772m-font-test.typ <<'EOF'
#set text(font: "DejaVu Sans", size: 12pt)
Hello World
EOF
lab/typst-original/target/release/typst compile /tmp/p772m-font-test.typ /tmp/p772m-vanilla.pdf
./target/release/typst compile /tmp/p772m-font-test.typ /tmp/p772m-cristalino.pdf
mutool trace /tmp/p772m-vanilla.pdf > /tmp/p772m-trace-vanilla.txt
mutool trace /tmp/p772m-cristalino.pdf > /tmp/p772m-trace-cristalino.txt
diff /tmp/p772m-trace-vanilla.txt /tmp/p772m-trace-cristalino.txt
```

Testar também `case` (transformação de maiúsculas/minúsculas, se o módulo cobrir `text(case: ...)`) com casos que tenham regras especiais por idioma (ex: "İ"/"i" turco, se aplicável — confirmar se o vanilla trata isso ou é scope-out consciente).

---

## Implementação

Só para achados confirmados como bug real, com coordenadas/métricas comparadas directamente (não só ausência de erro). Corrigir um a um.

Se o módulo revelar muitos bugs (dado ser a maior lista restante), dividir em lotes por submódulo (`variations`, `metrics`, `exceptions`, etc.) em vez de forçar tudo num passo.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Reconfirmar que os testes de regressão de P666-669/P753/P761-762 (fontes) continuam verdes — este módulo tem sobreposição directa com esses passos.

---

## Critério de fecho do passo

- [ ] Lista real de submódulos/itens de `text::font::*` confirmada (não a hipótese do prompt).
- [ ] Itens classificados, com atenção especial a `exceptions` como possível causa de residuais já vistos noutros passos.
- [ ] Bugs reais corrigidos com métricas/coordenadas comparadas directamente.
- [ ] Sem regressão nos testes de fonte já existentes (P666-669, P753, P761-762).
- [ ] Se volume grande: dividido em lotes por submódulo, não forçado num passo só.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772m.md`.

---

## Próximo passo

Com `image::svg`, `foundations::scope` e `text::font::*` cobertos, reconfirmar a lista `lacuna-inventario` restante (segunda rodada, mesmo padrão de P772e) e decidir se vale continuar ou encerrar a varredura sistemática.
