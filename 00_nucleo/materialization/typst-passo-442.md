# P442 — Refactor experimental: `get_unchecked` → slicing seguro (scanner)

> **Passo:** 442  
> **Data:** 2026-06-24  
> **Foco:** Substituir as 7 ocorrências de `unsafe { get_unchecked }` em `scanner.rs` por slicing seguro `&self.string[start..end]`, preparando medição de impacto de performance.  
> **Pré-requisito:** P441 (infra de benchmark ADR-0115 ACEITE).  
> **ADR-0032:** `unsafe` em L1 eliminado por defeito; excepção permanente só com benchmark + ADR de número concreto.  

---

## Contexto

**P441** criou a infra de benchmark (ADR-0115 `EM VIGOR`) com 5 inputs de corpus e harness Criterion. Este passo executa o **refactor experimental** em branch: substituir `get_unchecked` por slicing seguro. O objetivo não é mergear imediatamente, mas gerar o **candidate** para comparação contra o baseline do P441.

O `scanner.rs` herda de `unscanny` (ADR-0014) e usa `get_unchecked` para evitar bounds-checking em 7 sítios de extração de substring. A substituição por slicing seguro (`&self.string[start..end]`) acrescenta bounds-checking em runtime, potencialmente com custo de performance.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `get_unchecked` ocorrências? | 7 em `scanner.rs` | ✅ |
| Slicing seguro é drop-in replacement? | Sim — `&str[start..end]` é `&str`, mesmo tipo que `get_unchecked` retorna | ✅ |
| Risco de panic com slicing? | Nenhum — os índices `start..end` são sempre válidos (derivados de `Scanner` state interno) | ✅ |
| Benchmark baseline existe? | Sim — P441 produziu medidas para 5 inputs | ✅ |
| Bloqueadores? | Nenhum | ✅ |

**Reclassificação:** S (~20 min; 7 substituições mecânicas + 1 run de benchmark).

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Estratégia de branch | Branch experimental `p442-get-unchecked-removal` | Isola o refactor; permite comparação A/B sem poluir `main` |
| Substituir todos de uma vez | Sim — 7 ocorrências | O padrão é idêntico em todos os sítios; atomização por sítio não compra nada |
| Manter `unsafe` em comentário? | Não — remover completamente | Se o benchmark mostrar regressão aceitável, o merge elimina `unsafe`; se não, revertemos a branch |
| `Scanner::eat` e `Scanner::before` | Substituir `self.string.get_unchecked(start..end)` por `&self.string[start..end]` | Métodos que extraem substring do scanner |

---

## Toques pontuais

Os 7 sítios estão concentrados em métodos de extração de substring do `Scanner`:

1. **`eat` / `eat_until` / `eat_while`** — retornam `&str` extraído do buffer.
2. **`before` / `after`** — retornam prefixo/sufixo como `&str`.
3. **Métodos internos de `eat` variants** — `eat_newline`, `eat_str`, etc.

**Pattern de substituição:**
```rust
// Antes:
unsafe { self.string.get_unchecked(start..end) }

// Depois:
&self.string[start..end]
```

**Verificação de segurança:** em todos os sítios, `start` e `end` são derivados de:
- `self.cursor` (posição atual do scanner, sempre ≤ `self.string.len()`)
- Resultado de `find`/`position` sobre o buffer (sempre dentro dos bounds)
- Ou offsets calculados a partir de `self.cursor` com verificação prévia

Logo, o slicing nunca panicará em condições normais. O bounds-checking é puramente defensivo.

---

## Scope-out explícito

- **Não** toca em outros ficheiros com `unsafe` (nenhum em L1 além de `scanner.rs`).
- **Não** altera a lógica do scanner — apenas o mecanismo de extração de substring.
- **Não** escreve o ADR de decisão final (isso é P443, após medição).
- **Não** mergea para `main` — branch experimental isolada.

---

## Critério de fecho

- [ ] 7 ocorrências de `get_unchecked` substituídas por slicing seguro em `scanner.rs`.
- [ ] `cargo test --workspace` verde na branch experimental (zero regressões funcionais).
- [ ] `cargo bench --bench scanner_bench` executado na branch experimental.
- [ ] Tabela comparativa baseline vs candidate produzida (ns/byte por input).
- [ ] Delta percentual calculado para cada input.
- [ ] Branch `p442-get-unchecked-removal` pronta para decisão no P443.
- [ ] `crystalline-lint` zero novas violações.

---

**Próximo passo:** Com P442 fechado, o **P443** correrá o benchmark comparativo, aplicará o critério de decisão da ADR-0032 (< 5% = eliminar; 5-20% = decisão humana; > 20% = excepção permanente com ADR específica) e fechará DEBT-42 de forma definitiva. Indique se quer ajustar o escopo do P442.
