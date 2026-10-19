---
# P772c (lote 5) — Varredura da stdlib: `typst_syntax::package`

> **Passo:** 772c (continuação da série P765a/P765b/P772/P772a/P772b)
> **Data:** 2026-07-16
> **Foco:** Classificar `typst_syntax::package` (8 itens na recontagem de P772a), módulo de parsing/representação de especificações de pacote (`@preview/nome:versão`). Ligação directa com a linha de trabalho de download de pacotes (P763-P763b) e resolução de versão (P764/P764a) — qualquer divergência aqui pode já ter sido coberta por esses passos, ou pode ser um ângulo não verificado (ex: mensagens de erro de parsing de especificação malformada, distintas de erros de resolução/download).
> **Tipo:** Sonda + Implementação directa para achados confirmados.
> **Tamanho:** S/M.
> **ADR-0108 EM VIGOR.** **ADR-0107** — mensagens de erro de parsing são observável legítimo.
> **Dependências:** P772b (mesmo lote), P763/P764 (contexto de pacotes já resolvido, para não duplicar trabalho).

---

## Sonda — classificar os 8 itens

```bash
awk -F'\t' '$1=="lacuna-inventario" && $5 ~ /typst_syntax::package/' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt
```

```bash
grep -n "<item>" lab/typst-original/crates/typst-syntax/src/package.rs 2>/dev/null
grep -rn "<item>\|PackageSpec\|PackageVersion" 01_core/src/entities/*.rs 03_infra/src/*.rs 2>/dev/null
```

### Casos de teste — parsing de especificação malformada

Distinto do que já foi coberto por P763/P764 (download e resolução de versão): aqui o foco é o **parsing da string** antes de qualquer tentativa de resolução — mensagens de erro para sintaxe inválida.

```bash
for spec in "@preview" "@preview/" "@preview/nome:" "@preview/nome:abc" "preview/nome:1.0.0" "@preview/nome:1.0.0.0"; do
  echo "=== $spec ==="
  cat > /tmp/p772c-test.typ <<EOF
#import "$spec"
EOF
  echo "--- vanilla ---"
  lab/typst-original/target/release/typst compile /tmp/p772c-test.typ 2>&1 | head -3
  echo "--- cristalino ---"
  ./target/release/typst compile /tmp/p772c-test.typ 2>&1 | head -3
done
```

Comparar as mensagens de erro exactas para cada caso malformado — não só se ambos dão erro, mas se o texto bate (ou diverge de forma consciente e nomeada, regra 7 do handoff).

---

## Implementação

Só para divergências de mensagem confirmadas. Corrigir replicando o texto exacto do vanilla, com teste comparando a mensagem completa.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Os 8 itens de `typst_syntax::package` classificados item a item.
- [ ] Casos de parsing malformado testados com mensagem exacta comparada (não só presença de erro).
- [ ] Confirmado que não há sobreposição não resolvida com o trabalho já feito em P763/P764 (se houver, referenciar, não duplicar).
- [ ] Bugs reais corrigidos com teste de mensagem exacta.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772c.md`.

---

## Próximo passo

Com `image::raster` (P772), `pdf::accessibility` (P772a), `span` (P772b) e `package` (P772c) cobertos, reconfirmar a lista `lacuna-inventario` restante e decidir se vale continuar (módulos cada vez menores, provavelmente rendimento decrescente) ou encerrar a varredura sistemática com o que já foi coberto.
