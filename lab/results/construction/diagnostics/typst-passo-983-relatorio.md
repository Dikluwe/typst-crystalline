# Relatório — Passo 983 (oráculo: split posicional por item math)

**Data:** 2026-08-05 · **Gate:** confirmado pelo dono em 2026-08-05
("continuar" após `typst-passo-983-faseA.md`).
**Proveniência**: HEAD no início = `a725e3b23` (P983 Fase A). Benchmark:
`temp/p983/typst-antes` = release de P981 (P982 não mudou código).

## Fase B — implementação

O desenho da Fase A (sem dados novos de fonte) foi implementado tal qual:

- `PageContext` ganhou `oracle: bool` (default `false`; setter interno
  `with_oracle`), propagado pelo `PdfBuilder` nos 3 pontos de construção
  de contexto (type1/cidfont/multifont).
- `verbose_run_end` (`stream.rs`): no caminho do oráculo, itens
  `style.math` nunca formam run de P979 — cada item math é o seu bloco,
  como o vanilla (medido na Fase A: 6 blocos para `$ 3x + y = 9 $`).
  Prosa continua a fundir. `collapse_trivial_tj` (P980) corre depois e
  converte os blocos triviais em `Tj`.

**Testes** (`p983_tests` em `export/tests.rs`; RED confirmado
retroativamente — implementação precedeu o teste neste passo, registado;
sem a regra, os 2 testes de split falham): math não funde no oráculo (2
blocos) mas funde no normal (1); posição exacta do item após o split
(`cm` com o `pos.x` do item); prosa funde mesmo no oráculo. Suite:
**5771 testes, 0 falhas** (+3).

## Fase C — Revalidação

Documento de 30 secções com `--oracle-pdf`:

| métrica | oráculo P980 | oráculo P983 | vanilla |
|---|---|---|---|
| blocos `BT…ET` | 1293 | **1957** | 1919 |
| `Tj` | 79.7% | **98.2%** | 92.5% |
| `TJ` com ajuste real | 263 | **35** | 148 |

- Blocos: 1957 vs 1919 do vanilla (2% a mais — os nossos itens são
  ligeiramente mais finos que os fragmentos do vanilla em alguns pontos).
- `Tj`% até **ultrapassa** o vanilla (98.2% vs 92.5%): os 35 `TJ`
  residuais são runs multi-glifo com ajustes internos reais (kerns e
  x_offsets de itens como "sin"/anotações), que o vanilla mantém como
  ajustes dentro dos seus blocos também.
- **Posições**: `compare.py` oráculo-vs-normal — **0/2965 glifos** acima
  de 0.5pt. O split usa `pos.x` dos itens; nada se move.
- **Saída principal inalterada**: PDF normal pós-P983 **byte-idêntico**
  ao de P981 (módulo timestamps/IDs). Nota de processo: a primeira
  comparação (contra o PDF de P980) falhou por 31 bytes — era a mudança
  intencional de P981 (`lr`), referência errada minha; com a referência
  certa (P981), identidade exacta.
- **Benchmark do caminho normal** (`benchmark-p983-canonical.py`,
  7 cenários, `tools/perf/results/p983-canonical/`): 01-hello 1.008 ·
  02-lorem 1.022 · 03-images 1.004 · 04-math 1.010 · 05-tables 1.009 ·
  06-long 0.994 · 07-context 1.049 — rácio médio 1.014. O 07-context
  fora da banda foi **re-medido isoladamente** (hyperfine, 30 runs):
  **1.00 ± 0.03** — ruído de carga concorrente, sem regressão (a única
  mudança no caminho normal é um `if ctx.oracle` por item).
- **Linter**: resselo dos ficheiros tocados; `crystalline-lint .` →
  0 violations (só o V7 órfão pré-existente).

## Resultado

- O oráculo particiona runs por item math, espelhando a granularidade do
  vanilla — blocos a 2% e `Tj`% acima do alvo (98.2% vs 92.5%).
- Posições de glifo inalteradas por construção e provadas por medição.
- Saída principal intocada (prova byte-a-byte).
- A ferramenta de auditoria fica com estrutura de operadores comparável
  ao vanilla para os próximos passos de paridade.
