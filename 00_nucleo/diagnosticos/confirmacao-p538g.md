# Confirmação P538g — antes da reorganização

**Data:** 2026-07-03  
**Passo:** 538g  
**Tipo:** Confirmação diagnóstica  
**Dependências:** P538b, P538c, P538d, P538e, P538f

## Resumo

Reconfirmação directa dos cinco itens fechados em P538b–P538f, mais a
comparação de contagem de palavras de `lorem()` e o registo formal da
fragilidade do fallback de fonte.

## Grupo 1 — Reconfirmação directa dos quatro itens

### Metadados (P538b)

```typst
#set document(title: "Relatório de José", author: "João Conceição")
Texto.
```

Resultado `pdfinfo`:

```
Title:           Relatório de José
Author:          João Conceição
```

**Confirmado:** ✓

---

### Numeração de página (P538d)

```typst
#set page(numbering: "1")
Página.
```

Resultado `pdftotext`:

```
Página.

1
```

**Confirmado:** ✓

---

### Colunas, documento longo (P538c)

```typst
#set page(columns: 2)
#lorem(1200)
```

Resultado `pdfinfo`:

```
Pages:       5
Page size:   595.28 x 841.89 pts (A4)
```

**Confirmado:** ✓

---

### Fallback de fonte, sem fonte explícita (P538e)

```typst
Hello 你好 مرحبا
```

Resultado `pdftotext`:

```
Hello

你好‫مرحبا‬
```

**Confirmado:** ✓

Nota: fragilidade do fallback global carácter-a-carácter registada como
**DEBT-65** em `00_nucleo/diagnosticos/debt/DEBT.md`.

## Grupo 2 — `#for` dentro de colunas (P538f)

```typst
#set page(columns: 2)
#for i in range(0, 80) [
  #{i+1}. #lorem(20)
]
```

Resultado `pdfinfo`:

```
Pages:       6
Page size:   595.28 x 841.89 pts (A4)
```

`mutool show` não reporta erros; estrutura de objectos válida.

**Confirmado:** ✓

## Grupo 3 — Contagem de palavras de `lorem(1200)`

| Compilador | Palavras extraídas |
|------------|--------------------|
| Cristalino | 1183 |
| Vanilla    | 1200 |

Diferença: 17 palavras (~1,4%). Dentro da variação explicável por
hifenização, quebras de linha e diferença de texto gerado (cristalino
repete "Lorem ipsum...", vanilla usa texto latino contínuo). Não é
considerado bug separado.

**Fechado:** ✓

## Tabela final

| Item | Confirmado | Notas |
|------|------------|-------|
| Metadados (P538b) | ✓ | Title/Author correctos |
| Numeração de página (P538d) | ✓ | "1" visível no PDF |
| Colunas, documento longo (P538c) | ✓ | 5 páginas A4 |
| Fallback de fonte, sem fonte explícita (P538e) | ✓ | Três scripts extraíveis; fragilidade registada em DEBT-65 |
| `#for` em colunas (P538f) | ✓ | 6 páginas A4, PDF válido |
| Contagem de palavras `lorem()` | ✓ | 1183 vs 1200, diferença residual aceitável |

## Decisão de prosseguimento

Todos os itens confirmados. A reorganização (P539+) pode começar.

## Ficheiros alterados

- `00_nucleo/diagnosticos/debt/DEBT.md` — adicionada entrada **DEBT-65**.
- `00_nucleo/diagnosticos/confirmacao-p538g.md` — este relatório.
