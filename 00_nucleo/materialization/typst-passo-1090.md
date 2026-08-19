# L0 — Passo 1090: Corrigir `base_ic` Ausente para Bases `TextShaped` em `attach.rs`

**Gate**: `ADR-0127` — mudança de comportamento por defeito (qualquer subscrito
sobre base com correcção itálica não-nula é afectado).

---

## Análise (investigação P1089, confirmada por 4 rondas de verificação)

### O que está errado

Em `01_core/src/compiler/math/layout/attach.rs:257-269`, o cálculo de
recuo do subscrito pela correcção itálica da base (`base_ic`) só extrai o
valor quando o item da base é `FrameItem::Glyph`. Quando a base é
`FrameItem::TextShaped` — o caso de um identificador matemático como `g`
shaped junto com o resto da linha — a extracção falha e cai em
`unwrap_or(0.0)`, descartando a correcção itálica real.

### Por que isto só apareceu em linha completa, não em testes isolados

Achado da investigação: `$ g_i $` testado sozinho **já** produzia o desconto
correcto (`Δ=-0.3467pt`, valor esperado com `IC(g)=0.275pt` aplicado) — o
shaper, isolado, provavelmente resolve a base como `FrameItem::Glyph`
directo. Dentro da linha completa da Equação 3 (`∇g_i(x^*)`), `g` é shaped
em conjunto com o operador `∇` e o resto do contexto, produzindo
`FrameItem::TextShaped` em vez de `Glyph` — e é só nesse caminho que o bug
aparece. **Um teste isolado do termo, por si só, não teria apanhado este bug**
— só apareceu ao medir a linha inteira contra o vanilla.

### Cadeia de evidência que fechou a causa

1. Limiar de medição estrito (0.001pt, não o 0.5pt inicial que mascarava o
   problema) isolou exactamente `g_i`/`∇g_i(x^*)` como os únicos termos com
   divergência sistemática (`+0.275pt`) numa bateria de 8 termos testados.
2. Tabela `MathItalicsCorrectionInfo` da fonte real (`NewCMMath-Regular.otf`)
   confirmou `IC(g)=25du=0.275pt`, contra `IC(h)=0`, `IC(x)=0`,
   `IC(μ)=0`, `IC(i)=0` — refutando uma suposição anterior errada de que `g`
   e `h` partilhavam IC por "ambos serem glifos itálicos".
3. Reconciliação aritmética completa: soma dos 7 termos medidos
   isoladamente (`-4.0702pt`) vs delta da linha inteira sem a correcção
   (`-3.7742pt`) — a diferença de `+0.296pt` decompõe-se exactamente em
   `+0.275pt` (IC de `g` perdido em contexto de linha) + `+0.021pt` (ruído
   de `MediaBox` acumulado em 7 PDFs isolados). Fecha ponto a ponto.

---

## Mecanismo

Em `attach.rs`, no ponto onde `base_ic` é extraído (linhas ~257-269),
estender a extracção para cobrir `FrameItem::TextShaped`, não só `Glyph`.
Ler a definição real de `FrameItem::TextShaped` antes de codificar — não
presumir que carrega o mesmo tipo de informação de glifo/fonte que `Glyph`
sem confirmar (pode precisar de resolver o glifo base a partir do texto
shaped, não ter o `glyph_id` directamente disponível).

Reaproveitar o mecanismo já usado para `space_after_script` (mesma
investigação, correcção irmã) para obter a métrica da fonte real — não
duplicar lógica de acesso a `MathConstants`/tabela de fonte.

## Critérios de verificação — isolado E em linha completa, não só um dos dois

Per o achado desta investigação, testar só termos isolados não teria
apanhado o bug original — a verificação da correcção precisa do mesmo
cuidado, ao contrário:

1. `$ g_i $` isolado — continua correcto (já estava, não regressão).
2. **`$ nabla g_i(x^*) $` em linha completa, no mesmo contexto de shaping da
   Equação 3 original** — este é o teste que realmente valida a correcção,
   não o termo isolado.
3. A Secção 30 completa (3 equações), limiar 0.001pt (não 0.5pt) —
   confirmar 0 glifos acima do limiar, não "quase zero".
4. Largura de página — convergir para 262.649pt exactamente (não os 90%
   anteriores).
5. Testar pelo menos mais um caso de base com IC não-nulo em contexto de
   linha (`f`, IC=0.99pt, ou `λ`, IC=0.803pt, ambos já catalogados na
   auditoria da fonte) — confirmar que a correcção generaliza a outros
   glifos com IC diferente de `g`, não só o caso específico já medido.
6. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- `FrameItem::TextShaped` real lido antes de editar `attach.rs`.
- Critério 2 (linha completa) confirmado — não só o termo isolado do
  Critério 1.
- Critério 5 — pelo menos um glifo de IC diferente de `g` testado em
  contexto de linha, generalização confirmada.
- Largura de página convergindo exactamente, não parcialmente.
