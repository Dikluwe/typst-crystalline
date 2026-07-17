---
# P772 (lote 2) — Varredura da stdlib: próximo módulo por tamanho

> **Passo:** 772 (continuação da série iniciada em P765a "lote 0" e P765b "lote 1")
> **Data:** 2026-07-16
> **Foco:** P765b classificou `typst_library::math::style` (32 itens) e corrigiu 4 bugs reais (`cramped` em `script`/`sscript`, `display`/`inline` ausentes). O próprio P765b já tinha identificado, na sua sonda de identificação de módulo, os módulos seguintes por tamanho em `lacuna-inventario`: `typst_library::layout::grid::resolve` (24 itens, então classificado como infra-estrutura Rust) e `typst_utils` (14 itens, idem). Este passo reconfirma a contagem actual (a lista pode ter mudado desde 2026-07-15 se novos diagnósticos a actualizaram) e classifica o módulo seguinte que ainda não foi tratado.
> **Tipo:** Sonda + Implementação directa para achados confirmados como bugs reais.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0107** — distinguir símbolo de língua de infra-estrutura Rust antes de classificar como dívida.
> **Dependências:** P765b (lote 1, metodologia e módulos já descartados: `foundations`/`calc`, `diag`, `math::style`).

---

## Sonda — reconfirmar e identificar o módulo seguinte

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn | head -15
```

Excluir os módulos já tratados (`foundations::calc`, `foundations::ops`, `diag`, `math::style`, `typst_utils` se já avaliado como infra) e confirmar qual é o próximo por tamanho ainda não classificado. Se `layout::grid::resolve` (24 itens) for de facto o próximo e continuar a classificar-se como infra-estrutura Rust (resolução interna de grid, não símbolos de língua), avançar para o módulo seguinte na lista — não gastar o passo a reconfirmar uma conclusão já esperada sem necessidade.

Para cada item do módulo escolhido, aplicar a mesma disciplina de P765a/P765b: ler o código-fonte real antes de classificar.

```bash
grep -n "<símbolo>" 01_core/src/rules/stdlib/*.rs 01_core/src/entities/*.rs 2>/dev/null
```

---

## Implementação

Só para achados classificados como `bug real` (mesma régua: comparação directa de comportamento/saída contra o vanilla, com casos de teste mínimos). Corrigir um a um, cada correcção com o seu próprio teste e comparação de saída — mesmo padrão de P765b (named args, funções ausentes, etc.).

Se o módulo revelar mais de ~5 bugs reais distintos, não tentar corrigir todos num só lote — registar a lista completa e propor lote seguinte.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Para cada bug corrigido, documento de sonda mínimo comparando vanilla vs cristalino (saída de `repr()`, mensagens de erro, ou comportamento observável, conforme o caso).

---

## Critério de fecho do passo

- [ ] Contagem por módulo reconfirmada directamente, não assumida da medição de 2026-07-15.
- [ ] Módulo seguinte (ainda não tratado) identificado e classificado item a item.
- [ ] Cada item lido no código-fonte antes de classificar (bug real / diferença aceitável ADR-0107 / lacuna de granularidade / falso-positivo).
- [ ] Bugs reais corrigidos um a um, com teste e comparação directa.
- [ ] Se o volume exceder o razoável, dividido explicitamente em lotes.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772.md`, com a tabela de classificação completa do módulo.

---

## Próximo passo

P772a (lote 3): módulo seguinte por tamanho, mesma metodologia — continuar até a lista `lacuna-inventario` estar coberta ou até restarem só módulos de infra-estrutura Rust sem símbolos de língua.
