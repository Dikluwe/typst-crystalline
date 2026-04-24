# ADR-0038 — Sistema de estilos em L1 (`Style`, `Styles`, `StyleChain`)

**Status**: EM VIGOR (Passo 99.E) — validado empiricamente com 780 testes a passar e zero violations no linter.
**Data**: 2026-04-23
**Autor**: Humano + IA
**Passo associado**: 99

---

## Contexto

O Passo 22 introduziu um `StyleChain` simplificado em L1 para suportar
herança de `bold`/`italic`/`size` em blocos aninhados de markup. Essa
versão usa um `StyleDelta` com os três campos `Option<T>` e não é
extensível para outras propriedades sem mudança estrutural — por isso
ficou registada como DEBT no próprio Passo 22.

Após os Passos 96.x (ADR-0037 aplicada) e 98 (ADR-0036 aplicada ao
`EvalContext`), o `StyleChain` viaja como `&mut StyleChain` entre as
funções `eval_*` (Passo 94). Isto cria condições para formalizar o
sistema de estilos em L1 como fundação sobre a qual `#set` e `#show`
possam construir no futuro.

Este ADR define o mapa de camadas, o âmbito do enum `Style`, a forma
da cadeia de resolução, e delimita claramente o que fica **fora**
deste passo.

---

## Decisão

### Mapa de camadas

| Camada | Entidade | Fonte |
|--------|----------|-------|
| L1 | `Style` enum | `01_core/src/entities/style.rs` |
| L1 | `Styles` collection | `01_core/src/entities/style.rs` |
| L1 | `StyleChain<'a>` | `01_core/src/entities/style_chain.rs` |
| L1 | `Content::Styled(Box<Content>, Styles)` | `01_core/src/entities/content.rs` |
| L3 | Memoização com `LazyHash` | Adiada. ADR-0016 permanece em vigor. Passo futuro quando pipeline incremental real for activado. |

`LazyHash<T>` continua **fora de L1** — não é revogação nem exceção
à ADR-0016.

### Divergência do vanilla

O vanilla usa proc macros `#[elem]` para gerar um enum `Property`
derivado. O cristalino usa **enum linear manual** em L1 — mesmo
precedente da ADR-0026 (`Content` enum fechado em vez de trait
objects). Razão: L1 não pode depender de proc macros custom sem
introduzir dependência de build complexa; a expressividade do enum
manual é suficiente para o conjunto de propriedades que o cristalino
materializa hoje.

### Âmbito do enum `Style` (Passo 99)

5 variantes (ver `inventario-style-passo-99.md` para detalhe):

- `Bold(bool)` — `text.bold`
- `Italic(bool)` — `text.italic`
- `Size(Pt)` — `text.size`
- `Fill(Color)` — `text.fill` (forward-compat)
- `HeadingLevel(u8)` — `heading.level` (forward-compat)

Adiadas: `text.font`, `text.lang`, `par.leading`, e ~todas as
propriedades derivadas de `#[elem]` proc macro no vanilla.

Nota: ADR futura (quando `Font` real entrar em L1) pode expandir o
enum. Esta ADR não fecha o conjunto.

### `StyleChain<'a>` como estrutura

Lista ligada imutável de blocos de `Styles`. Cada bloco é uma
referência (via `Arc` internamente) para um `Styles` + apontador para
o pai. Clone O(1), leitura O(N) na profundidade.

Lifetime `'a`: no Passo 99 mantém-se como `StyleChain` "sem lifetime"
(usa `Arc` internamente — não referências borrow-checked). Um refinement
futuro pode introduzir `StyleChain<'a>` com referências se o pipeline
incremental real exigir. Adiado para passo dedicado.

### Regra de resolução

**Top-wins** — o delta mais próximo do texto (mais recente via `push`)
ganha. Confere com o vanilla (`StyleChain::get`). Decisão confirmada
no 99.A por leitura directa.

### Decisão SUB vs COEX

O inventário 99.A contou 70 sítios de `TextStyle` (≈55 sítios de
consumo `.bold/.italic/.size`). O critério da spec (15 sítios) favorece
COEX.

**Decisão: COEX**. `TextStyle` permanece em `layout_types.rs` como
"vista achatada para o Layouter actual"; `From<&StyleChain> for TextStyle`
faz a ponte (já existe desde Passo 22). `Content::Styled` usa `Styles`,
não `TextStyle`. Substituição completa fica como DEBT sucessor aberto
em 99.E.

### O que esta ADR não decide

- **Quando `#set`/`#show` activam no eval**: este passo só materializa
  a fundação. A activação no `eval_markup` e `eval_set_rule` fica para
  passo dedicado.
- **Quando `LazyHash` vai para L3**: depende da activação do pipeline
  incremental real. ADR-0016 permanece em vigor.
- **Quando `Font` real entra em L1**: se entrar, o enum `Style` ganha
  `Font` variante; ADR nova será emitida.
- **Substituir `TextStyle` por `StyleChain` em `FrameItem::Text`**:
  novo DEBT aberto em 99.E.

### Coexistência com ADRs existentes

- **ADR-0016** (LazyHash fora de L1): **não revogada**. Este ADR clarifica
  que `Style`/`Styles`/`StyleChain` operam em L1 sem depender de
  `LazyHash`.
- **ADR-0026** (Content enum fechado): **precedente aplicado**. Enum
  linear em vez de proc macros no vanilla.
- **ADR-0033** (paridade com vanilla): **top-wins confirmado como
  paridade**.
- **ADR-0036** (atomização progressiva): `StyleChain` já é parâmetro
  explícito (Passo 94, quarta aplicação da Regra 1 — fechada no
  Passo 98).
- **ADR-0037** (coesão por domínio): novos ficheiros seguem o padrão
  (tests próximos, smoke test V2).

---

## Alternativas consideradas

1. **Proc macro `#[elem]` como no vanilla**: requer crate
   `typst-macros` ou equivalente — introduz dependência complexa em L1.
   Rejeitado pelo precedente ADR-0026.

2. **Apenas `StyleDelta` com os 3 campos actuais**: mínimo viável para
   o estado actual. Rejeitado — a spec do Passo 99 pede superconjunto
   preparado para futuro, e manter a struct fechada força nova mudança
   estrutural quando `Fill` ou `HeadingLevel` forem precisos.

3. **SUB (substituição completa)**: eliminar `TextStyle` a favor de
   `StyleChain` em `FrameItem::Text` e no Layouter. Rejeitado pelo
   critério objectivo (≈55 sítios de consumo; limite da spec: 15).
   Registado como DEBT sucessor.

4. **`StyleChain<'a>` com referências borrow-checked**: mais eficiente
   mas complicado — exigiria refactoring dos call sites para garantir
   lifetime apropriado. Adiada para passo futuro.

---

## Consequências

### Positivas

- Fundação tipada para `#set` e `#show` sem mais mudanças estruturais
  em `StyleChain`.
- Enum `Style` extensível: adicionar `Fill` ou `HeadingLevel` a uma
  cadeia é adicionar uma variante ao enum e um caso ao resolver.
- `Content::Styled` permite representar blocos estilizados na AST sem
  depender do Layouter.
- Precedente claro para o padrão "vocabulário tipado em L1, memoização
  em L3 se necessária" — ADR-0016 preservada.

### Negativas

- Coexistência de `TextStyle` (plano) e `StyleChain`/`Styles` (tipado)
  durante período interino. Documentada como COEX; DEBT sucessor
  aberto.
- O `Vec<Style>` em `Styles` tem `Clone` O(N) em vez do O(1) que
  `EcoVec` daria. Optimização diferida (ADR-0035 pode ser expandida
  para autorizar `EcoVec` em L1, mas não neste passo).

### Neutras

- `StyleDelta` actual (bold/italic/size) é mantido como representação
  interna por compatibilidade; exposto apenas como backing do accessor
  conversion. O consumidor externo usa `Styles` ou os accessors
  directos.

---

## Referências

- `00_nucleo/diagnosticos/inventario-style-passo-99.md`
- `00_nucleo/materialization/typst-passo-99.md`
- `lab/typst-original/crates/typst-library/src/foundations/styles.rs` —
  referência para a regra top-wins (`StyleChain::get`).

---

### Nota Passo 126 — `weight` como primeira propriedade numérica

**Data**: 2026-04-24. DEBT-1 subset pago.

`StyleDelta.weight: Option<u16>` adicionado para capturar
`#set text(weight: 700)` sem warning.

**Pattern estabelecido**: propriedades de `text` podem ser
adicionadas uma a uma como `Option<T>` em `StyleDelta`, com
captura em `eval_set_text`. Não exige materialização de tipos
Font/Lang/Par adjacentes, nem entrada no enum `Style` (bake-in
usa `StyleChain::push(delta)`, não `push_styles(&Styles)`).

**Forma simbólica** (`"bold"`, `"regular"`, mapeamento para
números do OpenType) fica para passo dedicado. Este passo
cobre só `weight: 700` numérico.

**Efeito de layout**: **nenhum**. `StyleDelta.weight` é
capturado mas inerte. Pipeline de layout hoje não consome
weight (sem selecção de variante de fonte, sem faux-bold).
Primeira aplicação que consumir `weight` materializa o path
pertinente (resolver em `StyleChain`, exposição via
`TextStyle`, consumer em `layout` ou `export`).

**Canary DEBT-50 preservado**: `#set text(font: "X")` continua
a emitir warning. Teste `eval_set_text_font_canary_passo_126`
garante.

**Range**: aceita qualquer `u16` (cast silencioso de `Value::Int`
via `u16::try_from`). CSS/OpenType definem 0-1000 mas validação
fica para o consumer. Out-of-range é silenciosamente ignorado
(coerente com outros arms de tipo errado).

---

### Nota Passo 127 — `tracking` como primeira propriedade com tipo semântico

**Data**: 2026-04-24. DEBT-1 subset (segunda propriedade numérica).

`StyleDelta.tracking: Option<Length>` adicionado para capturar
`#set text(tracking: 0.5pt)` (ou `0.1em`, ou `1pt + 0.05em`)
sem warning.

**Extensão do pattern do Passo 126**: propriedades podem usar
**tipo semântico** de L1 (`Length`), não apenas primitivos.
Preserva-se o valor inteiro — `Length { abs, em }` sobrevive
sem colapsar para pt. Quando consumer existir, `resolve_pt(
font_size)` dá o valor final.

**Contraste com `size`** (legado): `size` colapsa para
`Option<f64>` via `l.abs.to_pt()`, perdendo componente `em`.
Este passo **não** replica essa perda — futuras propriedades
novas devem preferir `Option<Length>` para preservar
precisão.

**Canary DEBT-50 preservado**: teste
`eval_set_text_font_canary_passo_127` garante em nova
iteração do pattern.

**Efeito de layout**: **nenhum** (inerte). Layout não consome
`tracking` hoje. Consumer futuro integra em `layout_text`
via offset inter-glyph.

---

### Nota Passo 129 — `weight` simbólico via helper em tipo semântico L1

**Data**: 2026-04-24. DEBT-1 subset (quarta aplicação do pattern).

Arm `"weight"` em `eval_set_text` estendido para aceitar
`Value::Str` com os 9 nomes canónicos do Typst vanilla
(`thin/extralight/light/regular/medium/semibold/bold/extrabold/black`).
Conversão feita por **helper em tipo semântico L1**:

```rust
impl FontWeight {
    pub fn from_name(name: &str) -> Option<Self> { ... }
}
```

Em `entities/font_book.rs` onde `FontWeight(u16)` + 9 constantes
já viviam. Arm delega ao helper e extrai `u16` via `to_number()`.
`StyleDelta.weight` permanece `Option<u16>` (decisão 126).

**Pattern variante registado**: "helper simbólico em tipo
semântico L1". Aplicável a futuras propriedades simbólicas
(`font-stretch`, `style`, etc.) — adicionar método
`from_name(&str) -> Option<Self>` ao tipo L1 existente, delegar
no arm do eval.

**Divergência vanilla** (nome inválido):
- Vanilla: erro de cast com mensagem listing valid options.
- Cristalino: silent skip (coerente pattern DEBT-1 XS).
Categoria ADR-0033: semântica; aceite temporalmente; teste
`eval_set_text_weight_simbolico_desconhecido_silent_passo_129`
documenta.

**Sem aliases**: vanilla não tem `"normal"` → `"regular"`;
cristalino também não (teste assertions confirmam
case-sensitive e alias-free).

**Canary DEBT-50 preservado**: quarta iteração consecutiva.

---

### Nota Passo 131B — `lang` não segue pattern DEBT-1 XS

**Data**: 2026-04-24. Refactor por paridade ADR-0033.

Campo `StyleDelta.lang` foi materializado como `Option<Lang>`
(tipo semântico L1) para obter paridade com vanilla. Arm
`"lang"` em `eval_set_text` valida via `Lang::from_str` e emite
`Err` hard em inválido (`SourceResult::Err` aborta
`eval_set_rule`).

Esta mudança **não é variante do pattern DEBT-1 XS**
(documentado nas notas Passos 126/127/129) — é refactor por
razões de paridade. Ver **ADR-0052** para contexto
arquitectural completo + diagnóstico 131A.

Futuras propriedades com necessidade análoga de paridade
semântica (validação + erro hard) seguem padrão do 131B:
diagnóstico obrigatório (ADR-0034) → ADR dedicada →
materialização. Não agregar neste ADR-0038.
