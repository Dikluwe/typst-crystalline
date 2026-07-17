# P772w — Classificação do resíduo de risco plausível: `target_`, `plugin_`, `image::pdf`, `layout::frame`, `math`

> **Passo:** 772w
> **Data:** 2026-07-17
> **Commit-base:** working tree após P772v (não commitado no início deste passo).
> **Dependências:** P772t (priorização), toda a metodologia estabelecida em P765a-P772v.

---

## 1. Sonda — recontagem exacta

```bash
for mod in "foundations::target_" "foundations::plugin_" "image::pdf" "layout::frame" "^typst_library::math::"; do
  awk -F'\t' -v m="$mod" '$1=="lacuna-inventario" && $5 ~ m' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt | cut -f5
done
```

| Módulo | Itens (P772t estimou) | Itens (recontagem exacta) |
|---|---:|---:|
| `foundations::target_` | 6 | 6 |
| `foundations::plugin_` | 5 | 5 |
| `image::pdf` | 4 | 4 |
| `layout::frame` | 4 | 4 |
| `math` (excl. `::style::`) | 4 | **5** |
| **Total** | **23** | **24** |

Pequena divergência na recontagem de `math`: `awk '$5 ~ /^typst_library::math::/' | grep -v "::style::"` devolve 5 itens
(`ClassElem`, `LeftRightAlternator`, `Mathy`, `attach::is_integral_char`, `families`), não 4 — a estimativa de P772t
estava a 1 item de distância. Registado para não repetir o número errado.

---

## 2. Classificação item a item

### 2.1 `foundations::target_` (6 itens)

| Item | Tipo real | Classificação |
|---|---|---|
| `target#1` (fn `pub fn target(...)`) | **Símbolo de língua real** | **Bug confirmado e corrigido** — ver §3.1 |
| `Target` (enum) | Mecanismo (tipo de retorno interno de `target()`, valores expostos só como string via `Cast`) | Mecanismo |
| `target#2` (segunda entrada "enum" com o mesmo nome) | Provável artefacto de duplicação do lens (mesmo símbolo `Target` contado 2×, uma vez como `enum` directo, outra via assinatura de `target()`) | Não classificável como item distinto — anomalia de medição, não substância |
| `Output` (trait) | Mecanismo — dispatch genérico interno para os 3 pipelines de export (paged/html/bundle) | Mecanismo |
| `AsOutput` (trait) | Mecanismo — helper de coerção `&impl Output` → `&dyn Output` | Mecanismo |
| `TargetElem` (struct) | Mecanismo — elemento interno só para hospedar o campo `target` na StyleChain, nunca construído pelo utilizador ("never constructed and not visible to users", doc vanilla) | Mecanismo |

**Confirmação por compilação real** (não só leitura de código):

```
$ echo '#context target()' > /tmp/t.typ
vanilla:    paged
cristalino (antes):  error: unknown variable: target
```

`target()` é o único item com efeito observável real do módulo — os outros 5 são mecanismo interno de um
sistema multi-target (paged/HTML/Bundle) que o cristalino não implementa e não precisa de implementar (só
produz PDF via layout paginado).

### 2.2 `foundations::plugin_` (5 itens)

| Item | Tipo real | Classificação |
|---|---|---|
| `plugin#1` (fn `pub fn plugin(...)`) | Símbolo de língua real | **Já implementado** — `native_plugin`, `01_core/src/rules/stdlib/plugin.rs:44` |
| `plugin#2` (provável `plugin.transition`, `#[scope] impl plugin`) | Símbolo de língua real (API de mutação/transition) | **Scope-out documentado e consciente** — P696 §5-6: "Transition API... **fora** do escopo... `cetz_core.wasm` (plugins puros)... transition não é necessário". Confirmado: não existe `transition` em `01_core/src/rules/stdlib/plugin.rs`, consistente com a decisão registada |
| `Plugin` (struct, privada no vanilla) | Mecanismo — pool de instâncias `wasmi` para execução multi-threaded | Mecanismo |
| `PluginInstance` (struct, privada) | Mecanismo — uma instância `wasmi::Instance` + `wasmi::Store` | Mecanismo |
| `Snapshot` (struct, privada) | Mecanismo — snapshot de memória WASM para restore (parte da transition API já scope-out) | Mecanismo |

Nenhum achado novo — granularidade do inventário, não ângulo novo. `plugin()` já implementado (P678-762);
`plugin.transition` é débito já registado e justificado (P696), não uma lacuna esquecida.

### 2.3 `image::pdf` (4 itens)

| Item | Tipo real | Classificação |
|---|---|---|
| `PdfDocument`/`PdfDocumentInner`/`PdfImage`/`PdfImageInner` | Estruturas de suporte a `#image("ficheiro.pdf")` — embutir uma página de PDF como imagem, via crate `hayro_syntax`/`hayro` (renderizador de PDF) | **Gap real, confirmado no escopo do vanilla 0.15.0** — ver §3.2 |

**Confirmação de escopo**: `lab/typst-original/Cargo.toml.original:7` → `version = "0.15.0"` — o vanilla clonado
É exactamente a versão 0.15.0 que este projecto rastreia; `image::pdf` faz parte dela, não é uma feature
futura fora de alcance.

**Confirmação do gap actual**: `#image("test.pdf")` no cristalino → `error: unknown image format` (a
detecção de assinatura de P772p não reconhece PDF como formato de imagem suportado — falha explícita, não
silenciosa).

**Decisão**: **não implementado neste passo** — requer uma dependência de renderização de PDF equivalente a
`hayro` (parsing de content streams, fontes embutidas, árvore de páginas), uma feature grande e nova, muito
além do tamanho "M" declarado para P772w. Registado como débito técnico priorizado para passo dedicado
futuro (precisaria da sua própria sonda de escolha de dependência L3 e provavelmente um novo ADR).

### 2.4 `layout::frame` (4 itens)

| Item | Tipo real | Classificação |
|---|---|---|
| `FrameKind` (enum Soft/Hard) | Mecanismo — determina o referencial de coordenadas para gradients dentro de um frame | Mecanismo |
| `GroupItem` (struct) | Mecanismo — bloco de construção interno do `Frame` (IR de layout), inclui `parent: Option<FrameParent>` | Mecanismo (mas ver achado colateral abaixo) |
| `FrameParent` (struct) | Mecanismo — localização + `Inherit` do grupo lógico pai (usado para ordenar `place()`/`footnote()` na árvore de introspecção) | Mecanismo |
| `Inherit` (enum Yes/No) | Mecanismo — mas **codifica um comportamento real observável**: se o conteúdo colocado via `place()` herda estilos do contexto envolvente (`Inherit::Yes`) enquanto `footnote()` não herda (`Inherit::No`) | Mecanismo cujo comportamento tem efeito de língua — ver §3.3 |

Confirmado exactamente o aviso do passo: os símbolos são mecanismo interno (estrutura de dados `Frame`),
não símbolos de língua. **Mas** investigar o comportamento por trás de `Inherit` (não o símbolo) revelou um
**achado real e significativo, fora do inventário original** — ver §3.3.

### 2.5 `math` (5 itens, excl. `::style::`)

| Item | Tipo real | Classificação |
|---|---|---|
| `Mathy` (trait vazio, marcador) | Mecanismo puro — marker trait para a macro `#[elem(..., Mathy)]` | Mecanismo |
| `LeftRightAlternator` (enum + `Iterator`) | Mecanismo — helper interno de iteração para alternância esquerda/direita em delimitadores | Mecanismo |
| `ClassElem` (struct, `math.class(...)`) | Símbolo de língua real (`math.class(class, body)` — força a classe de espaçamento de um símbolo) | **Gap real, confirmado ausente** — ver §3.4 |
| `families` (fn) | Determina a cadeia de fallback de fontes específica de modo matemático (`"new computer modern math"`, `"libertinus serif"`, fontes de emoji) | **Gap real, escopo estreito** — ver §3.5 |
| `attach::is_integral_char` (fn) | Símbolo de língua real por via indirecta — decide se limites empilham para operadores grandes | **Bug confirmado e corrigido** — ver §3.6 |

Nenhuma sobreposição com P765b (`math::style`) — os 5 itens ficam fora do namespace `math::style::*`
explicitamente excluído pelo filtro `grep -v "::style::"`.

---

## 3. Achados — detalhe

### 3.1 `target()` ausente do scope global (CORRIGIDO)

Ver §2.1. `#target()`/`#context target()` erravam "unknown variable: target"; vanilla devolve `"paged"`.
Corrigido: `native_target` (`01_core/src/rules/stdlib/foundations.rs`) devolve sempre `Value::Str("paged")`
(cristalino só produz PDF via layout paginado — não há outro valor possível). Registado no scope global
(`01_core/src/rules/eval/mod.rs`).

### 3.2 `#image()` com fonte PDF — gap real, não implementado (DÉBITO)

Ver §2.3. Requer dependência de renderização de PDF (`hayro`-equivalente). Fora do tamanho deste passo.

### 3.3 Decoração (underline/strike/overline) não propaga através de `layout_sub_frame` (ACHADO NOVO, GRANDE — DÉBITO)

**Fora do inventário original** — descoberto ao investigar o comportamento real por trás do símbolo mecânico
`Inherit` (§2.4), não por procurar directamente por ele.

**Repro**:
```typst
#underline[
  Some text #place(top+left)[explanation].
]
```

**Medição** (`mutool trace`, contagem de `stroke_path` — linhas de sublinhado):
- Vanilla: 4 `stroke_path` — cobre tanto "Some text" como "explanation" (herdado do `#underline` envolvente,
  mesmo estando "explanation" dentro de um `#place()` aninhado).
- Cristalino: 1 `stroke_path` — só "Some text"; "explanation" (dentro do `place()`) **não** é sublinhado.

**Causa raiz** (`01_core/src/rules/layout/sub_frame.rs::layout_sub_frame`): o mecanismo de decoração
wrap-aware (P284/P286) regista segmentos de sublinhado em `self.decoration_lines_collector` **dentro de
`flush_line()`** (`cursor.rs:231`). `layout_sub_frame` — usado por `place()`, células de grid, e outros
6 call-sites — faz o seu próprio flush manual da `current_line` **sem chamar `flush_line()`** (confirmado:
`grep flush_line sub_frame.rs` só encontra menções em comentários, nunca uma chamada real), pelo que o hook
do collector nunca dispara dentro de um sub-frame. Qualquer conteúdo decorado (`underline`/`strike`/
`overline`) que passe por um sub-frame (via `place()`, `grid()`, `box()`, etc.) perde a decoração.

**Por que não foi corrigido neste passo**: uma correcção correcta exige que o `DecoSegment` gerado dentro do
sub-frame seja traduzido para o referencial do frame pai — a MESMA translação (`offset`/`origin_x/y`) que já
é aplicada aos `FrameItem`s do sub-frame quando são colados de volta no pai. Isto tocaria os 7 call-sites de
`layout_sub_frame` (`placement.rs`, `place.rs`, `grid.rs`, `cursor.rs`, `mod.rs`), cada um com a sua própria
lógica de posicionamento — risco de regressão visual não trivial, muito além do tamanho "M" deste passo.
Registado como débito técnico priorizado, com repro exacto para não repetir a descoberta.

### 3.4 `math.class(...)` ausente (GAP REAL, ARQUITECTURALMENTE MAIOR — DÉBITO)

```
$ echo '#let loves = math.class("relation", sym.suit.heart); $x loves y$' > /tmp/t.typ
vanilla:     compila sem erro
cristalino:  error: módulo 'math' não tem campo 'class'
```

Confirmado ausente. Implementar correctamente exigiria primeiro confirmar/adicionar suporte a espaçamento
automático **baseado em `MathClass`** no motor de layout — `grep -rln MathClass 01_core/src/rules/layout`
não encontra nenhuma ocorrência; `MathClass` só existe hoje em `entities/math_class.rs` e nos ficheiros de
parsing/lexing (`rules/parse/math.rs`, `rules/lexer/math.rs`), nunca consumido pelo layout de equações. Isto
sugere que o espaçamento automático por classe pode não existir ainda no cristalino (ou usa outro
mecanismo não descoberto nesta sonda) — implementar `math.class()` de forma semanticamente correcta exige
primeiro resolver essa questão mais ampla. Fora do âmbito de uma correcção pontual "um a um"; registado como
débito, com a pista já registada (`MathClass` não chega ao layout) para o próximo passo não repetir a sonda.

### 3.5 Fallback de fontes específico de matemática (GAP REAL, ESTREITO — DÉBITO)

`families()` (vanilla) define a cadeia de fallback **específica de modo matemático**: `"new computer modern
math"` → `"libertinus serif"` → fontes de emoji. `03_infra/src/fallback_fonts.rs` só tem cadeias
genéricas serif/sans (P538e/P555), sem entrada específica para matemática. Impacto: quando a fonte de
matemática primária não tem um glifo, o cristalino cai na cadeia serif/sans genérica em vez da cadeia
matemática do vanilla — só manifesta em glifos ausentes da fonte primária (caso relativamente raro).
Registado como débito de escopo estreito, não implementado (exige localizar precisamente o ponto de
resolução de fonte em modo matemático e confirmar que não há já um mecanismo equivalente antes de adicionar
uma nova cadeia).

### 3.6 Integrais empilhavam limites incorrectamente + símbolo `integral` ausente (CORRIGIDO)

Dois bugs relacionados, descobertos ao testar `attach::is_integral_char` directamente (per instrução do
passo):

**Bug 1 — símbolo errado**: `ident_to_unicode("int")` devolvia `Some("∫")` — errado. No vanilla, `int` é o
construtor do tipo inteiro (`scope.define("int", Value::Type(Type::Int))`,
`01_core/src/rules/eval/mod.rs:1186`), não um símbolo — confirmado por compilação real do vanilla:
```
$ echo '$ int $' > /tmp/t.typ && vanilla compile
error: unknown variable: int
  hint: int is not available directly in math, but is in the standard library
  hint: to access int in code mode you can add a hash: #int
  hint: or access int in math mode by using the std module: std.int
```
O nome correcto do símbolo é `integral` (paridade com `sym.rs::("integral", '∫', ...)`, já correcto na
tabela completa usada por `#sym.integral`) — mas ausente de `ident_to_unicode` (a tabela usada para
identificadores **bare** em modo matemático, ex.: `$integral$` sem `sym.`). Antes da correcção, `$integral$`
produzia o texto literal "integral", não ∫.

**Bug 2 — limites empilhados incorrectamente**: `is_large_operator` (`math/symbols.rs`) incluía os
caracteres de integral no conjunto usado por `layout_attach` (`math/layout/attach.rs`) para decidir
empilhamento de `sub`/`sup` em modo bloco. Paridade vanilla (`Limits::for_char_with_class`,
`math/attach.rs` no vanilla): a classe `Large` só empilha (`Limits::Display`) se **não** for um sinal de
integral — integrais têm `Limits::Never` incondicional (scripts sempre ao lado, mesmo em display style).
Confirmado visualmente (`mutool draw`) antes/depois: `∫_0^1` empilhava `0`/`1` verticalmente como `∑_0^1`
antes da correcção; depois, `∫` mantém os scripts ao lado, `∑` continua a empilhar (controlo, sem
regressão).

**Correcção**:
- `ident_to_unicode`: `"int" => Some("∫")` removido; `"integral" => Some("∫")` adicionado.
- Nova função `is_integral_char(c: char) -> bool` (`math/symbols.rs`), faixas idênticas ao vanilla
  (`'∫'..='∳'`, `'⨋'..='⨜'`).
- `layout_attach` (`math/layout/attach.rs`): condição de empilhamento passa a
  `is_large_operator(ch) && !is_integral_char(ch) || is_limit_function(...)`.

---

## 4. Implementação — resumo

| Achado | Tamanho | Acção |
|---|---|---|
| `target()` ausente | Pequeno | **Corrigido** — `native_target`, registado no scope global |
| `"int"`→∫ errado + `"integral"` ausente | Pequeno | **Corrigido** — tabela `ident_to_unicode` |
| Integrais empilhavam limites | Pequeno (uma vez isolado) | **Corrigido** — `is_integral_char` + exclusão em `layout_attach` |
| `plugin.transition` ausente | — | Não é achado novo — scope-out já documentado (P696) |
| `#image()` com PDF | Grande | Débito técnico priorizado — nova dependência L3 |
| Decoração não propaga por `layout_sub_frame` | Grande | Débito técnico priorizado — achado novo, repro completo registado |
| `math.class(...)` ausente | Médio-grande | Débito técnico priorizado — depende de MathClass chegar ao layout |
| Fallback de fontes matemáticas | Estreito | Débito técnico priorizado |

### L0s actualizados antes do código

- `00_nucleo/prompts/rules/math/symbols.md` — `ident_to_unicode` (int→integral), nova função
  `is_integral_char`, Histórico de Revisões.
- `00_nucleo/prompts/rules/math/layout/attach.md` — nova secção "Empilhamento de limites (`is_limits`) —
  P772w" (mecanismo antes não documentado nesta L0, agora completo).
- `00_nucleo/prompts/rules/stdlib/foundations.md` — nova secção `native_target`.

### Testes automatizados novos

- `01_core/src/rules/math/symbols.rs`: `integral_converte_para_unicode`, `int_nao_e_simbolo_matematico`,
  `is_integral_char_cobre_a_familia_de_integrais`.
- `01_core/src/rules/math/layout/tests.rs`: `math_attach_sum_empilha_limites_em_modo_bloco` (controlo),
  `math_attach_integral_nao_empilha_limites_em_modo_bloco` (regressão).
- `01_core/src/rules/stdlib/mod.rs`: `p772w_target_devolve_paged`, `p772w_target_com_args_retorna_err`.

---

## 5. Validação

```
cargo build --release --workspace --tests   → 0 erros
cargo test --workspace --release
  typst-core:   4204 passed, 0 failed  (+7 novos, P772w)
  typst-infra:   647 passed, 0 failed, 5 ignored
  typst-shell:    33 passed, 0 failed
  typst-wiring:    2 passed, 0 failed
  cli (integration): 29 passed, 0 failed
  crystalline_lint (integration): 2 passed, 0 failed
crystalline-lint .
  0 violações (mesmo warning V7 pré-existente sobre
  package_version_resolution.md, não relacionado)
```

Confirmação visual (`mutool draw`) para os dois bugs corrigidos: `#context target()` → "paged" (igual ao
vanilla); `$integral_0^1 ... $` renderiza ∫ com scripts laterais (igual ao vanilla), `$sum_0^1$` continua a
empilhar (controlo, sem regressão).

---

## Critério de fecho do passo

- [x] Os 24 itens (5 módulos) classificados item a item (§2).
- [x] `target_`: testado com efeito observável real (`#context target()` → "paged" vs erro) — não só
      compilação sem erro.
- [x] `plugin_`: confirmado que `plugin.transition` é scope-out já documentado (P696), não ângulo novo.
- [x] `image::pdf`: confirmado no escopo do vanilla 0.15.0 (`Cargo.toml.original: version = "0.15.0"`)
      antes de julgar como lacuna — é lacuna real, registada como débito (tamanho grande demais para
      este passo).
- [x] `layout::frame`: confirmado que os 4 símbolos são mecânica interna (estrutura `Frame`) — mas
      investigação do comportamento por trás de `Inherit` revelou um achado real e maior (decoração não
      propaga por `layout_sub_frame`), registado como débito com repro completo.
- [x] `math`: confirmado sem sobreposição com P765b (`::style::` explicitamente excluído).
- [x] Bugs reais corrigidos com teste comparando comportamento/saída: `target()`, símbolo `integral`,
      empilhamento de limites em integrais.
- [x] `cargo test --workspace` verde (4204 em typst-core, +7 novos).
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772w.md`, com tabela de classificação
      completa (§2).

---

## Próximo passo

Com este passo, o resíduo de risco plausível identificado por P772t está coberto. Quatro itens de débito
técnico novo foram registados (§3.2-3.5), dois deles significativos (`#image()` PDF, decoração através de
`layout_sub_frame`). Avaliar se a série P765a-P772w encerra com um resumo final, ou se algum destes quatro
débitos justifica um próximo passo dedicado antes de encerrar — em particular, a propagação de decoração
por `layout_sub_frame` (§3.3) tem o maior raio de impacto observável (afecta `place()`, `grid()`, `box()`
e qualquer outro consumer de `layout_sub_frame` — 7 call-sites), mais do que os outros três combinados.
