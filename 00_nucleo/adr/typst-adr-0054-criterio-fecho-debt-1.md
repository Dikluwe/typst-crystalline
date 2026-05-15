# ⚖️ ADR-0054: Critério de fecho de DEBT-1 inclui consumo integral

**Status**: `EM VIGOR`
**Data**: 2026-04-24
**Autor**: Humano + IA
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/diagnostico-shaping-passo-135.md`](../diagnosticos/diagnostico-shaping-passo-135.md)

---

## Contexto

**DEBT-1** foi aberto no Passo 30 com âmbito "StyleChain +
propriedades adicionais para paridade total com o sistema de
styles do original" (DEBT.md linha 48). Interpretação usada nos
Passos 30–134: captura suficiente.

Os passos **126–134** capturaram a lista canónica completa
(`text.weight`, `text.lang`, `par.leading`, `text.font` incluindo
simbólico e agregado). Pós-134 os campos estão em `StyleDelta`
mas **5 de 10 são inertes**: layout actual usa `TextStyle`
plano que cobre só 5 campos (`bold, italic, size, fill,
heading_level`). `#set text(weight: 700)` é capturado mas PDF
é idêntico a sem o `#set`.

**ADR-0033** (paridade funcional) lido literal inclui output,
não só input processing. O diagnóstico do Passo 135 formalizou
o gap.

Esta ADR **redefine o critério de fecho de DEBT-1** para
incluir consumo integral.

## Decisão

DEBT-1 só fecha quando:

1. **Cada propriedade de `StyleDelta`** tem consumer em
   layout/export (ou é explicitamente marcada como scope-out
   com ADR de suporte).
2. **Output PDF observacional** é equivalente ao vanilla para
   inputs equivalentes (dentro dos limites do perfil de
   paridade adoptado — ver secção "Perfil de paridade" abaixo).
3. **DEBT-52** (rastreador aberto em Passo 135) encerra.

Captura sem consumer é **estado intermédio**, não fecho.

### Perfil de paridade

Três perfis possíveis, da mais restrita à mais permissiva:

1. **Bit-perfect**: output PDF binariamente idêntico.
   **Inalcançável** (font embedding, compressão, IDs).
2. **Visual**: render idêntico pixel-a-pixel depois de
   rasterizar. **Parcialmente alcançável** — mas sem
   rustybuzz, shaping real (ligatures, kern, bidi) diverge.
3. **Observacional graded**: métricas observáveis equivalentes
   (tamanho, cor, peso, espaçamento) para inputs de teste
   documentados, **sem garantia de shaping features**.
   **Alcançável em 4-8 passos**.

**Escolha**: **perfil observacional graded**.

Rustybuzz + shaping completo fica **explicitamente fora** do
critério de fecho de DEBT-1 (documentado como DEBT-52 fase
D/E opcional). Se no futuro for integrado, DEBT-1 pode
"reabrir" o critério para "visual", ou criar DEBT separado.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **(a)** Nota em ADR-0033 | Minimalista, não duplica | Enterra decisão de fecho sob princípio geral |
| **(b)** ADR-0054 dedicada ✓ | Formaliza mudança com precedência vanilla 0052/0053 | Mais uma ADR |
| **(c)** Só em DEBT-52 sem ADR | Zero ADR | Decisão perde peso; revisões futuras podem não a ver |

**Escolha**: (b). Mudança de critério de fecho de DEBT central
merece ADR própria. Precedente: ADR-0052 (Lang) e ADR-0053 (Font)
materializaram tipos como decisões próprias; esta materializa
critério como decisão própria.

## Consequências

### Positivas

- **Critério de fecho claro**: "capturou" deixa de ser
  suficiente. "Afectou output" torna-se obrigatório.
- **Base para DEBT-52**: o rastreador ganha âmbito claro.
- **Expectativa ajustada**: roadmap DEBT-1 expande
  significativamente. Fecho estimado ~4-8 passos adicionais
  (fase A/B/C do diagnóstico).
- **Perfil observacional graded** honesto: reconhece limites
  do stack actual (sem rustybuzz) sem esconder gap.
- **Precedente para futuros DEBTs fundamentais**: captura vs
  efeito é distinção explícita.

### Negativas

- **DEBT-1 permanece aberto mais 4-8 passos** (versus fecho
  imediato em 135 conforme roadmap original).
- **ADR-0033 ganha dimensão observável**: leitura mais estrita
  pode afectar critérios de outros DEBTs pendentes. Aceite.
- **DEBT-52 grande em escopo**: rastreador com muitos gaps.
  Mitigado: cada gap é passo dedicado S/M.

### Neutras

- **rustybuzz fica explicitamente fora de DEBT-1**: não é
  degradação nem escolha arquitectural — apenas delimitação
  realista do que 4-8 passos podem atingir.
- **TextStyle permanece como ponte**: DEBT-48 continua
  encerrado; extensão de TextStyle (fase A) é caminho menos
  resistência, não refactor fundacional.

## Plano de materialização (roadmap DEBT-52)

Resumido do diagnóstico 135 secção 4:

1. **Fase A (XS, 1 passo)**: estender `TextStyle` com
   `weight/tracking/leading/lang/font` + propagar via
   `From<&StyleChain>`.
2. **Fase B (S, 3 passos)**: consumers `tracking`, `leading`,
   `weight` faux-bold.
3. **Fase C (M, 4-5 passos)**: consumers `font` string/array,
   `lang` hyphenation, PDF font embedding.
4. **Fase D (opcional)**: font dict → ADR-0054bis autorizar
   `regex`.
5. **Fase E (opcional, escopo XL)**: rustybuzz integration —
   provavelmente série dedicada fora de DEBT-1.

DEBT-1 fecha quando **A + B + C** encerrarem. D/E não bloqueiam.

## Referências

- **DEBT-1** (DEBT.md) — alvo do critério.
- **DEBT-52** (aberto em 135) — rastreador de gaps.
- **ADR-0033** (paridade) — princípio geral.
- **ADR-0038** (StyleChain) — base estrutural.
- **ADR-0052** (Lang) / **ADR-0053** (Font) — precedentes.
- **Passo 135** diagnóstico.
- **DEBT-48** (ENCERRADO) — `TextStyle` como ponte aceite.
- **ADR-0082** (PROPOSTO P249) — formaliza pattern "Promoções
  reais scope-outs ADR-0054 graded" (4 critérios operacionais).

---

## §Promoções reais cumulativas (refino interno P249)

**Pós-P249**: o perfil graded permite **promoção real** de
scope-outs declarados (refino futuro per ADR-0054 graded
documentado em "Limitações conscientes" do passo de origem).

Tabela cumulativa pós-P250:

| # | Passo | Scope-out promovido | Origem (graded) |
|---|-------|---------------------|-----------------|
| 1 | P242 | `radius` (Block + Boxed) | P156G + P156H scope-out |
| 2 | P242 | `clip` (Block + Boxed) | P156G + P156H scope-out |
| 3 | P247 | `outset` semantic real (Block + Boxed) | P156G + P156H + P231 graded |
| 4 | P247 | `fill` (Block + Boxed) | P156G + P156H scope-out |
| 5 | P247 | `stroke` (Block + Boxed) | P156G + P156H scope-out |
| 6 | P248 | `Block.breakable` semantic real | P156G "semantic adiada" |
| 7 | P248 | `Boxed.height` overflow real | P156H "semantic adiada" |
| 8 | P248 | `TableCell.body` overflow clip implícito | P157B "ignorados em layout" |
| 9 | P250 | `Block.spacing` (cursor.y collapse) | P156G scope-out |
| 10 | P250 | `Block.above` (override spacing) | P156G scope-out |
| 11 | P250 | `Block.below` (override spacing) | P156G scope-out |
| 12 | P250 | `Block.sticky` (lookahead break) | P156G scope-out |
| 13 | P251 | `TableCell.body` overflow row break real cell-level | P157B "clip implícito P248" → row break γ-Items |
| 14 | P252 | `Boxed.stroke-overhang` semantic real (bounds Shape expandidos thickness/2 quando overhang=true) | P156H "rejeitado em native_box" → Stroke +1 field + activação Layouter |

**Marco P250**: Block A.4 COMPLETO 10/10 scope-outs originais
P156G fechados cumulativamente (incluindo breakable contado
como elemento original).

**Marco P251**: Categoria C.2 Fase 5 Layout activada parcialmente
(cell-level row break via γ-Items); multi-region completo
(column flow DEBT-56) continua diferido. Pattern "Slice frame
items at height" N=1 inaugurado.

**Marco P252**: **Boxed A.4 COMPLETO 6/6** — segundo variant
Content com 100% scope-outs originais P156H fechados
cumulativamente (após Block P250 10/10). Pattern "Refactor
cross-cutting entity primitivo" N=1 inaugurado.

**Divergência consciente P252**: construtor Rust low-level
`Stroke { paint, thickness, overhang: false }` é divergência
consciente face vanilla default `true`. Paridade user-facing
restaurada via stdlib `extract_stroke` helper (defaults
`overhang: true` para inputs Length/Color atalhos + Dict sem
chave explícita). Justificações cumulativas: backward compat
literal estrita pré-P252 (~42 construtores literais
preservados); anti-inflação 44ª (defaults zero-impact
construtor low-level paridade pattern P247 fill).

**Padrão metodológico de promoção formalizado em ADR-0082
PROPOSTO** (P249 administrativo XS): 4 critérios operacionais
(storage prévio + consumer Layouter graded + paridade vanilla
referência + backward compat literal).

**ADR-0054 status `EM VIGOR` preservado** — refino interno
secção nova apenas; não reaberta nem revogada. ADR-0082
formaliza metodologia downstream sem alterar perfil graded.
