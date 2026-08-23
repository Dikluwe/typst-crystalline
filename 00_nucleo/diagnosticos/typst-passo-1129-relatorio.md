# Relatório de Investigação e Correcção — Passo 1129

**Protocolo L0**: `00_nucleo/materialization/typst-passo-1129.md`
**Data**: 2026-08-21/22
**Status**: Concluído — causa exacta identificada e corrigida ($\Delta = 0.00000\text{ pt}$ na legenda de teste).

---

## 0. Base do passo

Nota externa (secção 40, reteste pós-P1127): altura de fonte correcta
($7.70\text{ pt}$) mas largura de cada glifo sistematicamente maior no
cristalino, razão média $1.146$ (perto de $8/7 \approx 1.143$), só no
texto de legenda de `underbrace(..., "cinco estrelas")` — o título da
secção (texto normal) não é afectado.

---

## 1. Isolamento da variável causal (§1)

Testados os 3 casos pedidos pelo L0, comparando PDF cristalino vs
vanilla (`/W` array do `CIDFontType0`, glifo a glifo):

1. `text(size: 7.7pt)[cinco estrelas]` **fora** de math — `/W`
   cristalino bate exacto com o vanilla (mesmos glifos, mesmas
   larguras: `c=428 i=271 n=542 c=428 o=504 …`, fonte
   `LibertinusSerif-Regular`). **Tamanho reduzido isolado não é a
   causa.**
2. `underbrace(x, "cinco estrelas")` isolado — a legenda usa a fonte
   `NewCMMath-Book` (não `LibertinusSerif`) e diverge: cristalino lia
   `c=508 i=323 n=631 c=508 o=569 …` (soma $6577\text{ du}$) vs vanilla
   `c=444 i=278 n=556 c=444 o=500 …` (soma $5789\text{ du}$). Razão por
   glifo **não constante** ($1.134$–$1.162$), refutando a hipótese
   `8/7` da nota como explicação literal (era só a média aproximada).
3. `overbrace(x, "cinco estrelas")` — reproduz exactamente o mesmo
   padrão de divergência (mesmo mecanismo partilhado em `underover.rs`,
   confirmado, não presumido).

**Veredicto §1**: a causa é específica ao texto de legenda dentro de
`underbrace`/`overbrace` (mais geralmente: qualquer texto de várias
letras em contexto `math` a `MathSize::Script`/`ScriptScript`), não ao
tamanho reduzido em si.

---

## 2. Caminho de código real (§2)

Divergência isolada nos glifos individuais lidos: cada letra da legenda
usava a variante `.st` da fonte (`c.st`, `i.st`, `n.st`, …, medidas
via `fontTools` contra `03_infra/fixtures/fonts/NewCMMath-Book.otf`)
em vez do glifo base (`c`, `i`, `n`, …). `.st`/`.sts` são as variantes
da feature GSUB `ssty` ("Math Script Style alternates" — cada letra do
alfabeto latino minúsculo tem duas: nível 1 para `Script`, nível 2 para
`ScriptScript`).

`ssty` já tinha sido implementada correctamente no Passo 977 — mas só
testada contra `$ K_n $` (subscrito de **1 carácter**). A implementação
(`ssty_level_of` em `03_infra/src/font_metrics.rs`, e o bloco espelhado
em `03_infra/src/shaper.rs::try_shape`) activava `ssty` sempre que
`style.math && math_size ∈ {Script, ScriptScript}`, sem nenhuma
restrição de comprimento de texto — aplicando-a a **cada carácter** de
qualquer run, incluindo a legenda multi-carácter.

Confirmado por leitura do vanilla (`lab/typst-original`):

- `ir/resolve.rs::resolve_text` — texto (`TextElem`, incluindo strings
  citadas em math como `"cinco estrelas"`) vira `TextItem`.
- `math/text.rs::layout_text` — um `TextItem` é laid out via
  `crate::inline::layout_inline`, o shaper de **parágrafo comum**, que
  usa detecção normal de script Unicode (`latn` para letras latinas) —
  **nunca** força o script OpenType `math`.
- `text/mod.rs::tags()` — a feature `ssty` é *pedida* genericamente
  sempre que `EquationElem::size` é Script/ScriptScript, mas fica
  **inerte**: confirmado via `fontTools` que a GSUB de
  `NewCMMath-Book.otf` só regista `ssty` no sistema de scripts `math`
  da `ScriptList` (`latn`/`DFLT` → `False`, `math` → `True`).
- `math/shaping.rs::shape_text` — só usado para `GlyphItem`
  (`resolve_symbol`, glifo matemático atómico — o `x` de `$ K_n $`) —
  este **força** incondicionalmente `buffer.set_script(Tag("math"))`.
  Só aqui `ssty` tem efeito real.

**Veredicto §2**: dois caminhos de shaping distintos no vanilla
(glifo atómico força `math`; texto de parágrafo não), colapsados num
só no cristalino (`style.math` sozinho decide), sem margem para a
distinção.

---

## 3. Causa numérica exacta (§3)

Não é `upem` trocado nem uma escala `8/7` literal — confirmado (i)
pela variação de razão por glifo (§1.2) e (ii) porque o `upem` da fonte
é `1000` nos dois lados (`ttf_parser`/`fontTools` concordam). A causa
é a **ausência de restrição de comprimento** na condição de activação
de `ssty`, com o valor de cada glifo `.st` a divergir do glifo base por
uma quantidade *própria da fonte* (não uma fracção fixa):

| Letra | base (du) | `.st` (du) | razão |
|---|---|---|---|
| c/e | 444 | 508 | 1.1441 |
| i/l | 278 | 323 | 1.1619 |
| n | 556 | 631 | 1.1349 |
| o/a | 500 | 569 | 1.1380 |
| s | 394 | 453 | 1.1497 |
| t | 389 | 446 | 1.1465 |
| r | 392 | 446 | 1.1378 |
| espaço | 332 | 332 | 1.0000 (sem `.st` para espaço) |

---

## 4. Correcção implementada

### 4.1 Primeira versão (só "1 carácter") e a sua regressão

A primeira correcção restringiu `ssty` a texto de **exactamente 1
carácter** (`text.chars().count() == 1`). Corrigiu os 3 casos do L0,
mas a revalidação do corpus (§5) apanhou uma regressão: `a_10`
(secção 39, dígito duplo em subscrito) deixou de receber `ssty`.

Medição directa do vanilla (`$ a_1 + a_10 = b $`) mostrou que **cada
dígito** de `"10"` sai em `BT…Tj…ET` próprio, com largura `.st`
($569\text{ du}$, não a base $500\text{ du}$) — "10" não é um
`TextItem`: `resolve_text` classifica texto puramente numérico
(dígitos ASCII + no máximo 1 ponto decimal, pelo menos 1 dígito) como
`NumberItem` (`ir/resolve.rs:284-292`), cujo `layout_number`
(`math/text.rs`) chama `GlyphFragment::synthetic` **por carácter** —
cada dígito força o script `math` individualmente, como um `GlyphItem`.

### 4.2 Correcção final

`ssty_eligible_text(text: &str) -> bool` (duplicada em
`font_metrics.rs` e `shaper.rs` — mesma convenção de P977 original,
"os dois lados têm de concordar", P772o) replica exactamente o
predicado `num` de `resolve_text`:

```rust
fn ssty_eligible_text(text: &str) -> bool {
    let count = text.chars().count();
    if count == 1 { return true; }
    let mut decimal_count = 0usize;
    let all_digit_or_dot = text.chars().all(|c| {
        if c == '.' { decimal_count += 1; }
        c.is_ascii_digit() || c == '.'
    });
    all_digit_or_dot && decimal_count != count && decimal_count <= 1
}
```

`ssty_level_of` (agora com parâmetro `ssty_eligible: bool` em vez de
`is_atomic_glyph`) só devolve `Some` quando este predicado é
verdadeiro, nos 4 pontos de chamada que percorrem `text.chars()` num
loop multi-carácter (`FontBookMetrics::advance`,
`FallbackFontMetrics::advance`, `text_ink_bounds`,
`text_ink_bounds_signed`); os pontos de assinatura de 1 carácter
(`char_italics_correction`, `top_accent_attach`) mantêm `true`
literal (já sempre atómicos). Em `shaper.rs::try_shape`, o pedido da
feature `ssty` ganha a mesma condição. `buffer.set_script(...)`
**não** foi alterado — fica incondicional a `style.math`, por não
haver medição de nenhuma outra feature GSUB math-only indevidamente
activada por isso (ADR-0108: não corrigir por especulação).

P975 (italics correction) **não muda** — continua restrito a 1 carácter
exacto; não há medição que justifique estendê-lo a números.

L0s actualizados **antes** do código, hash reselado
(`crystalline-lint --fix-hashes .`) nos dois passos (versão "1
carácter" e depois o refinamento numérico) — fluxo contínuo (ADR-0127:
correcção de paridade com o vanilla, sem paragem).

---

## 5. Validação

### 5.1 Testes (RED → GREEN)

`03_infra/src/integration_tests.rs`, módulo `p977_tests`:

- `p1129_underbrace_legenda_multi_char_sem_ssty` — RED
  ($50.643\text{pt}$ obtido) → GREEN ($44.575\text{pt}$, alvo do L0
  $44.58\text{pt}$, dentro de tolerância — o `44.58` da nota externa é
  arredondado a 2 casas, o valor exacto de design units é
  $44.5753\text{pt}$).
- `p1129_overbrace_legenda_multi_char_sem_ssty` — idem, confirma
  mecanismo partilhado.
- `p1129_subscrito_1_char_continua_com_ssty` — guarda de regressão do
  caso original de P977 (`$ K_n $` continua com `.st`).

### 5.2 Medição directa PDF → `/W`

Legenda `"cinco estrelas"` a $7.7\text{pt}$: soma de larguras
$5789\text{ du} \times 7.7/1000 = 44.5753\text{pt}$ — bate exacto com
o `/W` array do vanilla, glifo a glifo (`c=444 i=278 n=556 o=500
espaço=332 e=444 s=394 t=389 r=392 l=278 a=500`).

### 5.3 Revalidação do corpus (§ "P1086-1128 — zero regressão")

Recompilados os 44 `.typ/sec_*.typ` com o binário novo, comparados
(via `qpdf --qdf --stream-data=uncompress`, ignorando metadados
voláteis — `CreationDate`/`ID`/`InstanceID`) contra os
`sec_*_crystalline.pdf` do commit anterior:

- **Antes da correcção numérica (§4.1)**: 5 secções divergiam —
  `sec_10`, `sec_18`, `sec_25`, `sec_39`, `sec_40`.
- **Depois (§4.2)**: 4 divergem — `sec_39` (`a_10`) voltou a bater
  exacto com o commit anterior (confirmado glifo a glifo:
  `<0007>`/`<0003>` = `.st` de `'1'`/`'0'`, $569\text{ du}$, igual ao
  vanilla).
- As 4 restantes (`sec_10`: `underbrace(a+b+c, "soma")`; `sec_18`:
  `underbrace(overbrace(a+b, "top"), "bottom")`; `sec_25`: `"Res"`,
  `"prime"` em subscrito de somatório; `sec_40`: caso de teste do
  L0) são **todas** o mesmo mecanismo de bug (string citada
  multi-carácter em `math_size` de script) — não são regressões, são
  a correcção a propagar-se correctamente a todo o corpus. Nenhuma
  reduz para uma diferença inesperada: variam só em largura/posição
  vertical de sub-milímetro (≤0.08pt), consistente com o ink-bounds
  corrigido dessas legendas.

### 5.4 Suite completa

- `cargo build -p typst-infra` — 0 erros.
- `cargo test -p typst-infra --lib` — **799 passed, 0 failed**
  (796 + 3 testes novos de P1129), confirmado em 3 corridas
  consecutivas.
- `cargo test --workspace` — **100% pass** (5082 + 799 + 41 + 2 + 37 +
  2 testes, mais 3 doc-tests ignorados). Nota de proveniência: uma
  primeira corrida de `--workspace` mostrou 3 falhas isoladas em
  `export::tests::p270_1_pdf_bytes_{oklab_default,oklch_hue_wrap,hsl}
  _reproduziveis` (determinismo de bytes de gradiente PDF — código de
  cor, não relacionado com texto/fontes). Isolado: falha **não**
  reproduz em 3 corridas isoladas da mesma suite (`-p typst-infra
  --lib`, com e sem as alterações deste passo — inclusive no
  `git stash` do baseline sem as alterações, correndo limpo 3×), nem
  em 2 corridas subsequentes de `--workspace` completo (limpo nas
  duas). Classificação: intermitência pré-existente do ambiente
  (thread scheduling sob carga do `--workspace` completo), não
  causada por este passo — registado aqui por disciplina de
  proveniência (regra "registar a proveniência de cada medição"), não
  investigada a fundo por estar fora do escopo de P1129.
- `crystalline-lint .` — exit code 0, 0 violations (só avisos
  informativos V19 pré-existentes, não relacionados).

---

## 6. Ficheiros alterados

- `00_nucleo/prompts/infra/font_metrics.md` — L0 actualizado (§P977,
  correcção de escopo + refinamento numérico), hash reselado.
- `00_nucleo/prompts/infra/shaper.md` — idem.
- `03_infra/src/font_metrics.rs` — `ssty_eligible_text`,
  `ssty_level_of` com novo parâmetro, 4 pontos de chamada corrigidos.
- `03_infra/src/shaper.rs` — `ssty_eligible_text` (duplicada),
  condição de pedido da feature `ssty` corrigida.
- `03_infra/src/integration_tests.rs` — 3 testes novos em
  `p977_tests`.

---

## 7. Critérios de conclusão do L0 — checklist

- [x] §1 executado antes de §2 — isolamento antes de leitura de código.
- [x] Causa numérica exacta (não só "próxima de 8/7"), com código citado
      (`ssty_level_of`/`ssty_substitute`, GSUB `ssty` scoped a `math`).
- [x] Confirmado que `overbrace` também é afectado (não presumido).
- [x] Largura de "cinco estrelas" convergindo para $44.575\text{pt}$
      (alvo da nota: $44.58\text{pt}$, arredondado).
- [x] P1086-1128 revalidado — zero regressão (4 secções corrigidas,
      não regredidas; 1 falsa-regressão numérica apanhada e corrigida
      dentro do próprio passo antes de fechar).
- [x] `crystalline-lint .` — 0 erros. `cargo test --workspace` —
      100% pass.
