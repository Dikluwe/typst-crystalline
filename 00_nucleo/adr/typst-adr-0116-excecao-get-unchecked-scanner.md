# ADR-0116 — Excepção permanente: `get_unchecked` no scanner

**Estado:** `EM VIGOR` (Passo 443 — decisão final sobre DEBT-42).
**Decisão do dono (registada):** manter as 5 ocorrências de `unsafe { self.string.get_unchecked(...) }` em `01_core/src/rules/lexer/scanner.rs` como excepção permanente à política de zero `unsafe` em L1, por regressão de performance medida entre 8% e 58% ao substituí-las por slicing seguro.
**ADRs relacionadas:** ADR-0032 (política de `unsafe` em L1), ADR-0014 (inlining de `unscanny`), ADR-0115 (infra de benchmark do scanner).

---

## Contexto

O `Scanner` em `01_core/src/rules/lexer/scanner.rs` é código inlinado de `unscanny` (ADR-0014) e contém 5 chamadas directas a `unsafe { self.string.get_unchecked(start..end) }` nos métodos `before`, `after`, `from`, `to` e `get`. A ADR-0032 estabelece que `unsafe` em L1 é eliminado por defeito, mas permite excepções permanentes quando um benchmark reprodutível demonstra regressão inaceitável e um ADR específico regista o número concreto.

O Passo 441 criou a infra de benchmark (ADR-0115) e o Passo 442 executou o refactor experimental numa branch isolada (`p442-get-unchecked-removal`), substituindo as chamadas por slicing seguro `&self.string[start..end]`. O Passo 443 correu o benchmark comparativo e aplicou o critério de decisão da ADR-0032.

## Decisão

1. **Manter `get_unchecked` em `scanner.rs`.** A remoção introduz regressões de performance superiores ao limiar de 5% em todos os inputs medidos, e superiores a 20% em quatro dos cinco inputs.
2. **Não fazer merge da branch `p442-get-unchecked-removal` para `Tekt`.** A branch permanece como candidate documentado.
3. **Fechar DEBT-42 como excepção permanente**, satisfeito pelo presente ADR com número concreto de regressão.

## Medições

Método: `cargo bench --bench scanner_bench` (Criterion, 100 amostras por input).
Baseline medido em `Tekt` (`get_unchecked` intacto). Candidate medido na branch `p442-get-unchecked-removal` (slicing seguro).

| Input | Tamanho | Baseline mediana | Candidate mediana | Delta tempo | Delta ns/byte | Faixa ADR-0032 |
|-------|--------:|-----------------:|------------------:|------------:|--------------:|:---------------|
| b1_hello | 11 B | 50,54 ns | 62,03 ns | +22,8% | +23,3% | > 20% |
| b2_text | 4.721 B | 11,77 µs | 19,00 µs | +58,3% | +54,1% | > 20% |
| b3_math | 2.451 B | 21,85 µs | 26,36 µs | +22,3% | +18,4% | > 20% |
| b4_code | 4.035 B | 34,11 µs | 44,04 µs | +27,1% | +26,7% | > 20% |
| b5_utf8 | 1.691 B | 13,16 µs | 13,87 µs | +6,4% | +8,5% | 5–20% |

A média ponderada por bytes dos deltas por input é de aproximadamente **+35%**. O pior caso observado é o input `b2_text` com **+58%**.

## Invariante de segurança

As 5 chamadas a `get_unchecked` são seguras porque os índices `start` e `end` são sempre derivados do estado interno do `Scanner`:

- `self.cursor`, que nunca excede `self.string.len()` e é sempre avançado para fronteiras UTF-8 válidas.
- Resultados de `self.snap(i)`, que limitam o índice aos bounds e avançam para a próxima fronteira de char.
- Resultados de `Pattern::matches`, cujo invariante de implementação exige que `len` esteja in-bounds e numa fronteira UTF-8.

A violação destes invariantes seria um bug de correcção do `Scanner`, não uma falha do uso de `get_unchecked`.

## Consequências

- **Positivas.** `unsafe` permanece num hot path crítico do lexer; a performance do scanner não regrediuse. DEBT-42 está fechado de forma documentada e mensurável.
- **Negativas.** Mantém-se uma dependência de invariantes manuais no L1. Futuras alterações ao `Scanner` devem continuar a respeitar as fronteiras UTF-8 e os bounds do buffer.
- **Neutras.** O slicing seguro (`&self.string[start..end]`) continua a ser a alternativa canónica se, no futuro, o custo relativo do bounds-checking deixar de ser relevante (por exemplo, após refactor que reduza o número de extrações por token).

## Alternativas consideradas

| Alternativa | Resultado | Decisão |
|-------------|-----------|---------|
| Substituir tudo por slicing seguro e aceitar regressão | +8% a +58% consoante o input; média ponderada +35% | Rejeitada — excede limiar ADR-0032 |
| Manter `get_unchecked` condicionalmente só em release (`#[cfg(not(debug_assertions))]`) | Não medido; adiciona complexidade e divergência debug/release | Rejeitada — fora do scope do P443 |
| Manter `get_unchecked` com ADR de excepção | Conforme ADR-0032 e comprovado por benchmark | **Adoptada** |

## Referências

- ADR-0032 — Política de `unsafe` em L1
- ADR-0014 — Inlining de `unscanny` em `scanner.rs`
- ADR-0115 — Infraestrutura de benchmark para o scanner/lexer
- DEBT-42 — `get_unchecked` no scanner (fechado pelo presente passo)
- Relatórios: `typst-passo-441-relatorio.md`, `typst-passo-442-relatorio.md`, `typst-passo-443-relatorio.md`
