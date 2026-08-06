# Relatório — Passo 980 (oráculo de paridade de operador: transformação `Tj`/`TJ` fora da saída principal)

**Data:** 2026-08-05 · **Gate:** confirmado pelo dono em 2026-08-05
("continue" após `typst-passo-980-faseA.md`).
**Proveniência**: HEAD no início = `9a319e366` (P980 Fase A). Benchmark:
`temp/p980/typst-antes` = release de P979.

## Fase B — o que foi construído (escopo v1: só o PDF ajustado)

- **L0**: `00_nucleo/prompts/infra/export/oracle.md` (o rascunho confirmado).
- **Módulo** `03_infra/src/export/oracle.rs`: `collapse_trivial_tj` —
  função pura sobre a string do content stream; arrays `[ … ] TJ` com
  todos os ajustes inteiros zero viram `<hex…> Tj` (hex concatenado);
  números parseados como inteiros (`-0` conta como zero); qualquer ajuste
  ≠ 0, array vazio ou token inesperado ⇒ intocado. Seis testes de unidade
  (2 falharam primeiro por um off-by-one meu no scanner — corrigido e
  registado aqui por honestidade de processo).
- **Wiring aditivo** (nenhuma assinatura existente muda):
  `PdfBuilder::with_oracle` (pub(super)) aplica a transformação ao stream
  de cada página antes da compressão; `export_pdf_oracle` (mesma dispatch
  de fontes da emissão normal); `compile_to_pdf_bytes_oracle` (o impl
  privado ganhou o bool `oracle`; as entradas actuais passam `false`);
  flag `--oracle-pdf` em `cli.rs` (marcada como ferramenta de diagnóstico)
  → `main.rs` (só PDF; PNG/SVG ignoram-na como `--compact`).
- **Saída principal inalterada — prova directa**: o PDF do documento de
  30 secções compilado sem a flag é **byte-idêntico** ao de antes do
  passo (módulo timestamps/IDs XMP+Info, normalizados na comparação), e
  os snapshots binários P307b passam sem regeneração.

## Fase C — Revalidação

- **Proporção `Tj`/`TJ`** (documento de 30 secções): 31.2% → **79.7%**
  Tj (TJ 889→263, Tj 404→1030 — exactamente os 626 colapsáveis medidos
  na Fase A). Vanilla: 92.5%. A diferença residual vem dos ajustes TJ
  reais que o nosso delta model emite mais do que o vanilla — caminho
  futuro do oráculo, não bug.
- **Posições**: `compare.py` oráculo-vs-normal no documento completo —
  **0/2997 glifos acima de 0.5pt** (a transformação não move nada).
- **Prosa** (02-lorem): nada a colapsar (0 TJ triviais) — o PDF do
  oráculo é igual ao normal, como deve ser (os dois lados já convergiam
  aí desde P979).
- **Benchmark do caminho normal** (`benchmark-p980-canonical.py`,
  7 cenários, `tools/perf/results/p980-canonical/`): 01-hello 1.012 ·
  02-lorem 0.993 · 03-images 1.008 · 04-math 1.000 · 05-tables 1.008 ·
  06-long 1.004 · 07-context 1.000 — rácio médio **1.004** (a única
  diferença de código no caminho normal é um `if self.oracle` por
  página).
- **Linter**: resselo dos ficheiros novos/tocados;
  `crystalline-lint .` → 0 violations (só o V7 órfão pré-existente).

## Resultado

- Oráculo separado do exportador de produção, activado só por
  `--oracle-pdf`; transformação `Tj`/`TJ` dentro do oráculo.
- Saída principal comprovadamente inalterada (byte-idêntica módulo
  timestamps).
- Proporção `Tj` da saída do oráculo aproxima-se do vanilla (79.7% vs
  92.5%; era 31.2%).
- Próximo degrau natural (P981+): o relatório de paridade do desenho
  original (proporções `Tr 2`, `BDC`/`EMC`, balanceamento `q`/`cm`/`Q`)
  dentro do mesmo módulo.
