# Relatório — Passo 989: `dot(dot(x))` — dois pontos sobrepostos

**Estado do código das medições**: HEAD `9ec038a1c` (P988-A) + alterações
deste passo. Commit final no fim.
**Gate ADR-0127**: não aplicável — correcção de fórmula interna + correcção
de bug L3 num contrato já documentado (sem método/campo novo). Nota: a parte
B de P988 (método novo `top_accent_attach`) está em gate separado, a aguardar
o dono.

## Fase A — causa (instrumentação + fonte + vanilla)

Repro `temp/p987/dots.typ` (bandas a 1200dpi): vanilla tem os dois pontos de
`dot(dot(x))` distintos (centros a **2.52pt**); cristalino tinha **os dois na
mesma posição exacta** (só um visível) e a equação 2.52pt mais curta.
`dot(x)` simples era idêntico ao vanilla — o bug só aparece quando a caixa de
acento vira base de outro acento.

Instrumentação temporária (P989DBG, revertida): o acento interno ficava com
`new_ascent = 1.166pt` (correcto: 7.45pt) — a caixa não carregava o topo da
tinta do ponto; o acento externo usava então a mesma base que o interno →
mesma posição. Dois bugs compostos:

1. **L3 com sinal invertido** — `text_ink_bounds_signed` (ambas as
   implementações de `03_infra/src/font_metrics.rs`) devolvia
   `bottom = +y_min·s` em vez de `−y_min·s` (convenção documentada no trait,
   = `descent()` do vanilla). Para tinta flutuante (uni0307: y 571..677du)
   devolvia +6.28 em vez de −6.28 → `gap` de P922 invertido (−11.14 vs
   +1.42pt). Invisível até aqui porque o termo só entrava no `new_ascent`
   (a posição do acento cancela-o) e nenhuma caixa de acento tinha virado
   sub-caixa. Os critérios de P922 eram insensíveis ao sinal (ordenam por
   `min()`, e o render não depende do gap).
2. **`accent_h` sem sinal** — `accent_box.height()` do caminho de texto tem
   `descent` clampado a 0 para tinta flutuante, não a altura de tinta com
   sinal do fragmento vanilla (`y_max − y_min`). Os dois erros
   cancelavam-se parcialmente (1.17pt vs 7.45pt).

**Derivação** (registada em `accent.md` §P989): o valor correcto é
`max(base.ascent, −accent_y + accent.ascent)` — algebricamente idêntico à
fórmula aditiva do vanilla com altura de tinta com sinal, mas os termos com
sinal cancelam e dispensam o `signed_descent`. Fecha com os números reais:
7.45pt (dot(x)) e deslocamento 2.50pt (medido 2.52) no aninhado.

## Fase B — TDD

L0 primeiro: `accent.md` §P989 (fórmula `max`; fórmula aditiva de P922
substituída, com a derivação), `infra/font_metrics.md` §P989 (sinal L3).

RED confirmado (3 testes): `p989_acento_aninhado_...` falhou com os pontos em
`[0.0, 0.0]` — reprodução exacta do bug de produção via stub com o sinal da
L3 antiga; os dois P922 com valores assados actualizados falharam conforme o
novo modelo (p922-1: acento engolido pela base → 8.4; p922-3: valores
auto-consistentes, mesmo 14.32 por derivação diferente).

Implementação: `accent.rs` — fórmula `max`, helper `accent_signed_descent` e
variáveis `gap`/`accent_h` removidos (mortos), debug removido. L3 —
`bottom = −y_min·s` nas duas implementações + teste de integração com a fonte
real (bottom de `˙` ≈ −6.28pt; `g` positivo como guarda da convenção oposta).

GREEN: **5788 testes, 0 falhas**. `crystalline-lint .`: 0 violations (só V7
órfão pré-existente).

## Fase C — Revalidação

Repro (`temp/p987/dots-c2.png`, 1200dpi): bandas do cristalino
**pixel-idênticas ao vanilla** — `dot(dot(x))`: 49.08-50.16 / 51.60-52.68 /
54.18-59.10pt (dois pontos distintos, centros a 2.52pt); `dot(x)` inalterado.

Benchmark canónico (`benchmark-p989-canonical.py`, antes = release P988-A):
duas corridas — 04-math 1.033/1.019, restantes 0.997–1.022, médias
1.006/1.008. O 04-math ligeiramente acima de 1.0 é coerente com as caixas de
acento agora mais altas (layout correcto, não ruído de código); dentro da
banda histórica aceite (≤1.024) na segunda corrida. **Sem regressão
sistemática.**

## Nota de efeito lateral esperada

Todas as caixas de acento ficam com o `ascent` correcto (antes ~6pt mais
baixas) — o espaçamento vertical acima de conteúdo acentuado aumenta para
acompanhar o vanilla. Deslocamentos acumulados no doc canónico podem mudar
ligeiramente (para melhor paridade).
