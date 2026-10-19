# Relatório — typst-passo-893: `FallbackFontMetrics::math_constants` usa fallback fixo (Fase A)

**Data:** 2026-07-24T15:30:13Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `ccbe6816c5c26b84a780cc9aa15e2950cd5a6587` (HEAD do ramo `Tekt`; P891 e P892 já
reconciliados — P891 commitado nesse commit, P892 é diagnóstico puro sem código alterado).
**Working tree no início:** `M .gitignore` (não relacionado, pré-existente), mais os relatórios
untracked de P892/893 e dois PDFs soltos não relacionados (`output_vanilla.pdf`, `test_vanilla.pdf`)
— nada pendente para decidir sobre este passo.

---

## Fase A, ponto 1 — o que `MathConstants::fallback()` define exactamente

`01_core/src/entities/math_constants.rs:73-91`, 14 campos (`upem=1000` mais 13 constantes),
valores "baseados em STIX Two Math" (comentário do próprio código, linha 70).

## Fase A, ponto 2 — divergência medida contra a fonte real (`NewCMMath-Regular.otf`)

Ferramenta: `fontTools` (mesma usada em P892), lendo `font['MATH'].table.MathConstants`
directamente da fonte embutida real (`~/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/
files/fonts/NewCMMath-Regular.otf` — mesmo `rev` pinado em `Cargo.toml`, mesmos bytes do binário).

| Constante | Fallback (`STIX Two Math`) | Real (`NewCMMath-Regular`) | Divergência |
|---|---:|---:|---:|
| `axis_height` | 500.0 | 250 | **-50% (2×)** |
| `fraction_rule_thickness` | 66.0 | 40 | -39% |
| `fraction_num_gap` | 50.0 | 40 | -20% |
| `fraction_denom_gap` | 50.0 | 40 | -20% |
| `superscript_shift_up` | 362.0 | 363 | +0.3% (desprezível) |
| `subscript_shift_down` | 130.0 | 247 | **+90%** |
| `radical_vertical_gap` | 60.0 | 50 | -17% |
| `radical_rule_thickness` | 66.0 | 40 | -39% |
| `script_percent_scale_down` | 0.70 | 0.70 | 0% (idêntico) |
| `script_script_percent_scale_down` | 0.50 | 0.50 | 0% (idêntico) |
| `upper_limit_gap_min` | 100.0 | 200 | **+100% (2×)** |
| `lower_limit_gap_min` | 100.0 | 167 | +67% |
| `math_leading` | 200.0 | 154 | -23% |

(`upem` idêntico nos dois lados, 1000 — divergências acima já são directamente comparáveis sem
conversão adicional.)

**Veredicto: divergência NÃO desprezível.** `axis_height` (o centro vertical em torno do qual TODA
fracção/delimitador/raiz é alinhado, via `apply_axis_offset`) está errado por um factor de 2×.
`subscript_shift_down` e `upper_limit_gap_min` também divergem por ≥67-100%. Só 2 das 13 constantes
(`script_percent_scale_down`/`script_script_percent_scale_down`) e uma parcialmente (`superscript_
shift_up`) estão próximas do valor real. **Isto não é um ajuste fino — são proporções estruturalmente
diferentes**, confirmando o raciocínio teórico do passo (afecta todas as equações, não só casos
pontuais). Prossegue-se para a Fase B sem necessidade de consultar o dono primeiro (a condição de
pausa do passo só se aplica quando a divergência é desprezível — não é o caso).

Sanidade do método: `upem` bate exactamente (1000 nos dois lados, não é coincidência de unidades
trocadas); os 2-3 campos "quase iguais" confirmam que a leitura não está sistematicamente deslocada
por um factor global (ex.: unidades erradas produziriam divergência uniforme em todos os campos, não
seletiva).

## Fase A, ponto 3 — como o vanilla lê `MathConstants` da fonte real

`lab/typst-original/crates/typst-library/src/text/font/metrics.rs:196-324`
(`impl MathConstants { fn new(font: &FontInstance) -> Box<Self> }`):

```rust
ttf.tables().math.and_then(|math| math.constants)
    .map(|constants| Self::from_constants(font, &constants, space_width))
    .unwrap_or_else(|| Self::fallback(font, space_width))
```

Lê a tabela MATH real do `FontInstance` activo; só cai no fallback (baseado no MathML Core spec, não
STIX — divergência de proveniência entre vanilla e cristalino já pré-existente, não introduzida por
este passo) quando essa face específica não tem tabela MATH. **Não** faz cascata por uma lista de
fontes candidatas à procura de uma com tabela MATH — os `MathConstants` são cacheados via `OnceLock`
por `Font` (linha 31), amarrados à face que já foi resolvida para aquele texto.

## Fase A, ponto 4 — estado do trait e desenho da correcção

`FontMetrics::math_constants(&self) -> MathConstants` (`01_core/src/engine/layout/metrics.rs:76`)
já tem default (`MathConstants::fallback()`), confirmado em P891. `FontBookMetrics::math_constants`
(`03_infra/src/font_metrics.rs:321-347`) **já lê a tabela real correctamente** (única face fixa, sem
necessidade de resolver qual fonte usar). `FallbackFontMetrics` (variante multi-fonte, a usada em
produção) **nunca sobrepõe este método** — confirmado por grep, mesmo padrão exacto do achado de
P891 para `math_kern`.

### Ponto de integração — `MathLayouter::new` não recebe `style`

Diferente de `math_kern` (chamado por-glifo, já tem `style` disponível no call site), `math_constants`
é chamado **uma única vez**, em `MathLayouter::new(metrics: &'a M, block: bool) -> Self`
(`01_core/src/engine/math/layout/mod.rs:314-317`), que **não recebe `style`** — os `constants`
resultantes ficam cacheados no campo `self.constants` e reusados por toda a equação. O único call
site de produção (`01_core/src/engine/layout/equation.rs:62`) já constrói `math_style` (com
`math: true`) na linha imediatamente anterior — disponível, só não é passado.

**Desenho da correcção** (a confirmar antes da Fase B):

1. `FontMetrics::math_constants(&self) -> MathConstants` ganha `style: &TextStyle` (mesma mudança
   mecânica de assinatura que P891 aplicou a `math_kern`).
2. `FontBookMetrics::math_constants` aceita e ignora `style` (só tem uma face).
3. `FallbackFontMetrics::math_constants(&self, style)`: resolve `variant` +
   `primary = resolve_primary_with_math_fallback(style, &variant)` (mesmo mecanismo de P890/P891);
   **em vez de `covering(c, ...)`** (não há `char` — constantes são por-fonte, não por-glifo), itera
   `primary` em ordem e usa a **primeira face com tabela MATH presente** (`face.tables().math.
   is_some()`) — paridade com o efeito prático do vanilla (que cairia no fallback se a face
   específica não tivesse tabela MATH; aqui, com uma lista de candidatos, a primeira COM tabela é a
   escolha natural, já que nenhuma das faces sem tabela poderia contribuir nada de qualquer forma).
   Sem candidato com tabela MATH: `MathConstants::fallback()`.
4. Lógica de leitura extraída de `FontBookMetrics::math_constants` para uma função livre partilhada
   `math_constants_from_face(face: &ttf_parser::Face) -> MathConstants` (mesmo padrão de
   `math_kern_from_face`, P891).
5. `MathLayouter::new(metrics: &'a M, block: bool, style: &TextStyle) -> Self` ganha o parâmetro,
   chamando `metrics.math_constants(style)`. Único call site de produção (`equation.rs:62`) passa
   `&math_style` (já construído). **~48 call sites de teste** (`tests.rs`, todos
   `MathLayouter::new(&FixedMetrics, true)`) precisam do terceiro argumento — mudança mecânica pura
   (`FixedMetrics` não sobrepõe `math_constants`, sempre devolve o fallback independentemente do
   `style` passado), usando o helper `default_style()` já existente em `tests.rs`.

### Nota sobre por que `primary.first()` sozinho seria errado

No caso comum (sem `#set text(font:)` explícito), `resolve_primary_with_math_fallback` resolve
primeiro a lista genérica serif/sans (`fallback_font_list_for`, ex. `Libertinus Serif` — sem tabela
MATH) e só DEPOIS acrescenta `math_fallback_font_list()` (`New Computer Modern Math` primeiro nessa
lista) quando `style.math` é verdadeiro. Tomar cegamente `primary[0]` devolveria sempre
`Libertinus Serif` (sem tabela MATH) → fallback sempre, **sem qualquer correcção no caso comum** —
por isso o desenho acima procura a **primeira com tabela MATH**, não a primeira candidata
incondicional.

---

## STOP — aguardando confirmação do dono do projecto

Mudança de assinatura em duas fronteiras públicas: `FontMetrics::math_constants` (trait, L1) e
`MathLayouter::new` (construtor público, L1) — a segunda propaga a ~48 call sites de teste (mudança
mecânica, sem lógica nova) mais o único call site de produção. 4 L0s a editar antes da Fase B:
`00_nucleo/prompts/infra/font_metrics.md` (nova secção `P893` + correcção da assinatura de
`math_kern` já desactualizada na secção "Interface" desde P891 — achado incidental, corrigido de
caminho), `00_nucleo/prompts/engine/layout.md` (assinatura nova do trait), `00_nucleo/prompts/engine/
math/layout/_comum.md` (assinatura nova de `MathLayouter::new` — este L0 já estava parcialmente
desactualizado antes deste passo quanto a outros aspectos não relacionados, não tocados aqui), `00_
nucleo/prompts/engine/layout/equation.md` (call site).

**Hashes já sincronizados** (`crystalline-lint --fix-hashes .`, corrido após as 4 edições de L0):
17 ficheiros `.rs` actualizados (headers `@prompt-hash`), `crystalline-lint .` confirma **0 drift
novo** (só o V7 pré-existente, não relacionado). Aguardo confirmação do dono de que os 4 L0s estão
bons antes de avançar para a Fase B (TDD, implementação, suíte completa, confirmação visual com
`04-math.typ` + um documento com fracção) e a Fase C (benchmark completo).
