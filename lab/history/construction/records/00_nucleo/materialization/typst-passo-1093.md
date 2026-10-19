# L0 — Passo 1093: Auditoria e Correcção — Literais Truncados de Conversão de Unidade

**Gate**: `ADR-0127` — mudança de comportamento por defeito (qualquer
documento usando `cm`/`mm` é afectado; a magnitude é pequena mas sistemática
e não-aleatória).

**Base**: achado lateral desta investigação (P1089-1092) — o offset de
`~0.00046pt` visto repetidamente em quase todas as medições com margem `1cm`
vem de `28.346`/`2.8346` truncados em `layout_types.rs:946-953` e
`eval/mod.rs:1135-1140`, em vez das razões exactas `3600/127` e `360/127`.

---

## 1. Não parar em 2 locais — auditar todos

Esta conversa já viu o mesmo padrão de "valor certo, mas duplicado e só
corrigido nalguns sítios" mais de uma vez (`KAPPA`, 3 cópias, P1069;
`FAUX_BOLD_K`, 2 cópias). Antes de corrigir só os 2 pontos já achados:

```bash
grep -rn "28\.346\|2\.8346\|28346\|28\.35\b" 01_core/src/ 03_infra/src/ | \
  grep -v "test\|Test"
grep -rn "9144\|72\.0.*2\.54\|2\.54.*72" 01_core/src/ 03_infra/src/
```

Verificar também se `In` (polegada) usa `72.0` directo ou algum literal
aproximado por coincidência — confirmar, não presumir que só `cm`/`mm` têm o
problema.

## 2. Verificar se há mais unidades no mesmo sistema

O vanilla (`abs.rs`, já citado) tem 4 unidades na mesma tabela (`Pt`, `Mm`,
`Cm`, `In`). Confirmar que o cristalino implementa as 4 pela mesma família de
conversão (razão exacta sobre `Pt` como base) — se alguma unidade usar
fórmula diferente (ex.: `Mm` calculado a partir de `Cm` já convertido, em vez
de directo de `Pt`), o erro de arredondamento pode compor-se de forma
diferente do que a auditoria dos 2 pontos já achados sugere.

## 3. Correcção

Substituir os literais truncados pelas razões exactas (`3600.0/127.0`,
`360.0/127.0`), ou por uma constante nomeada centralizada (mesmo padrão já
estabelecido nesta conversa para `vanilla_defaults.rs`/`pdf_defaults.rs` —
evitar reintroduzir duplicação ao corrigir).

## 4. Medição — confirmar que o offset desaparece em toda a investigação anterior

Este é um caso raro nesta conversa onde já temos múltiplas medições
anteriores (P1089-1092) que deviam mudar de forma previsível com esta
correcção — reaproveitar, não remedir do zero:

- Reconferir os deltas de margem já registados (`~-0.000456` a `-0.000460pt`,
  visto em todas as tabelas brutas de P1090-1092) — devem desaparecer
  (→ 0.0000pt) depois desta correcção, sem tocar em nada do resto do
  mecanismo de espaçamento de equações.
- Confirmar que isto **não** afecta os outros dois resíduos menores já
  identificados (space_after_script trailing, 0.616pt; formatação de
  MediaBox, 0.001-0.004pt) — são causas distintas, não devem mudar com esta
  correcção.

## Critério de conclusão

- Grep completo executado, não só os 2 locais já conhecidos confirmados como
  únicos.
- `In` (e qualquer outra unidade) confirmada correcta ou corrigida também.
- Offset de margem (~0.00046pt) desaparecido nas medições já existentes desta
  investigação, sem afectar os outros dois resíduos conhecidos.
- `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.
