# P441 — Infra de benchmarking para desbloquear DEBT-42

> **Passo:** 441  
> **Data:** 2026-06-24  
> **Foco:** Criar infraestrutura de benchmark reprodutível para medir impacto de remover `get_unchecked` do scanner, desbloqueando o fecho de DEBT-42.  
> **ADR-0032:** `unsafe` em L1 eliminado por defeito; excepção permanente só com benchmark + ADR de número concreto.  

---

## Contexto

**DEBT-42** está aberto desde o Passo 84.8a. O `scanner.rs` (herdado de `unscanny` via ADR-0014) contém 7 ocorrências de `unsafe { self.string.get_unchecked(start..end) }`. A ADR-0032 exige que qualquer excepção a `unsafe` em L1 seja sustentada por benchmark reprodutível que demonstre regressão inaceitável ao eliminar o `unsafe`, registada em ADR específico com número concreto.

O que falta não é o refactor em si (mecânico: 7 substituições de `get_unchecked` por slicing seguro), mas a **infra de medição** que permita tomar a decisão honestamente.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `get_unchecked` ocorrências em `scanner.rs`? | 7 ocorrências | ✅ |
| Infra de benchmark existe no projeto? | Não — zero ficheiros `bench/` ou `criterion` | ❌ |
| Critério de regressão definido? | ADR-0032: regressão > 20% = excepção permanente; 5-20% = decisão humana; < 5% = eliminar | ✅ |
| Conjunto de inputs representativos definido? | Não — precisa de corpus de stress para lex | ❌ |
| Bloqueadores técnicos? | Nenhum — `criterion` em cache local; `cargo` resolve | ✅ |

**Reclassificação:** S-M (~45 min; 1 ADR + 1 harness + 1 suite de inputs + 1 teste de reprodutibilidade).

---

## ADR-0112 — Infra de benchmark (PROPOSTO)

### Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Framework | `criterion` 0.5 | Padrão de facto Rust; já em cache local (probe implícito); suporta estatísticas robustas (outlier detection, confidence intervals) |
| Localização | `benches/` na raiz do workspace | Convenção Cargo; não polui `01_core/src/` com código de benchmark |
| Isolamento de L1 | Benchmarks invocam `Lexer::new(src).next()` via API pública de `01_core` | Não toca em código privado; mede o que o utilizador observa |
| Inputs | 4 classes de documento (ver §Inputs abaixo) | Cobertura de cenários: micro, texto corrido, math denso, código denso |
| Métrica primária | Tempo de lex por byte (ns/byte) | Normaliza por tamanho; permite comparar inputs de tamanhos distintos |
| Critério de reprodutibilidade | 3 execuções sequenciais com variação < 3% entre mediana e média | Evita outliers de thermal throttling / scheduler noise |

### Inputs do corpus de stress

| ID | Descrição | Tamanho alvo | Fonte |
|----|-----------|-------------|-------|
| B1 | Micro: `"Hello World"` | ~11 bytes | Hand-crafted; baseline de overhead mínimo |
| B2 | Texto corrido: 10 parágrafos de lorem ipsum | ~5 KB | Gerado; stress de markup, texto, espaços |
| B3 | Math denso: 50 equações inline + block | ~3 KB | Hand-crafted; stress de math mode switching |
| B4 | Código denso: 100 let-bindings + closures | ~4 KB | Hand-crafted; stress de code mode, identificadores, números |
| B5 | Edge: UTF-8 multibyte denso (emoji + CJK + acentuação) | ~2 KB | Hand-crafted; stress de boundary correctness |

### Estrutura do harness

```
benches/
├── scanner_bench.rs          # Criterion harness principal
├── corpus/
│   ├── b1_hello.typ
│   ├── b2_text.typ
│   ├── b3_math.typ
│   ├── b4_code.typ
│   └── b5_utf8.typ
└── lib_bench.rs              # Helpers: load_corpus, black_box consume
```

O harness `scanner_bench.rs` define 5 grupos de benchmark (1 por input), cada um medindo `Lexer::new(src).collect::<Vec<_>>()` (consumo total de tokens). O critério de comparação será:

1. **Baseline:** branch `main` com `get_unchecked`.
2. **Candidate:** branch experimental com `&self.string[start..end]`.
3. **Delta:** `(candidate - baseline) / baseline * 100%`.

### Critério de aceitação da infra

- [ ] `cargo bench` corre sem erros em ambiente limpo (sem `RUST_MIN_STACK` ou variáveis especiais).
- [ ] 5 inputs carregam do `benches/corpus/` e produzem métricas estáveis (variação < 3% entre 3 runs).
- [ ] Relatório de benchmark gera tabela Markdown comparativa (ns/byte por input).
- [ ] ADR-0112 transita PROPOSTO → **ACEITE**.
- [ ] `DEBT.md` atualizado: DEBT-42 "desbloqueado por ADR-0112 ACEITE; aguarda execução de benchmark".

---

## Scope-out explícito

- **Não** executa o refactor de `get_unchecked` → slicing seguro (isso é P442, após a infra existir).
- **Não** escreve o ADR específico que autoriza ou proíbe o `unsafe` (isso é P443, após medição).
- **Não** adiciona benchmarks para parse, eval, layout — escopo estrito ao scanner/lexer.
- **Não** integra CI para benchmark — local-only por ora.

---

## Critério de fecho

- [ ] `benches/scanner_bench.rs` criado com harness Criterion funcional.
- [ ] 5 ficheiros de corpus em `benches/corpus/` (B1–B5).
- [ ] `Cargo.toml` raiz atualizado com `[[bench]]` section.
- [ ] `cargo bench` produz relatório estável (3 runs, variação < 3%).
- [ ] ADR-0112 criado em `00_nucleo/adr/` e transita para ACEITE.
- [ ] `DEBT.md` atualizado com nota de desbloqueio.
- [ ] `crystalline-lint` zero novas violações (benches/ fora de L1, não afecta lint).
- [ ] `cargo test --workspace` verde (zero código L1 modificado).

---

**Próximo passo:** Com P441 fechado, o P442 executa o refactor experimental (branch) e o P443 toma a decisão com números. Indique se quer ajustar o escopo do P441.
