# Passo 100 — Substituir `TextStyle` por `StyleChain` no Layouter e export (DEBT-48)

**Série**: 100 (passo único de construção; sub-passos de
inventário, decisão e verificação).
**Precondição**: Passo 99 encerrado; `Style` / `Styles` /
`StyleChain` em L1; `Content::Styled` no enum; COEX com
`TextStyle` em vigor; 780 L1 + 174 L3 + 6 ignorados; zero
violations.
**ADRs aplicáveis**: ADR-0016 (LazyHash fora de L1 — preservada),
ADR-0026 (divergência via enum linear), ADR-0033 (paridade
funcional com vanilla), ADR-0036 (atomização), ADR-0037 (coesão
por domínio), ADR-0038 (sistema de estilos em L1).

---

## Objectivo

Pagar DEBT-48. Eliminar `TextStyle` como representação primária
de estilo no pipeline de layout e export. Depois deste passo:

1. O Layouter aceita `StyleChain<'a>` no seu pipeline interno
   (escopo **amplo**: o estilo flui pela estrutura como cadeia
   de deltas, não como struct achatado passado entre métodos).
2. `FrameItem::Text` deixa de carregar `TextStyle` com a forma
   actual. A forma exacta (Styles owned ou struct resolvido) é
   decidida em 100.A com base no inventário.
3. O `export.rs` em L3 consome a nova representação. Se a forma
   escolhida for "struct resolvido", `export.rs` lê o struct. Se
   for "Styles owned", `export.rs` resolve antes de escrever o
   PDF (function de resolução vive em L1 ou L3 conforme a
   decisão da ADR-NN nova).

O passo **não** activa `#set`/`#show` no `eval_markup`. A activação
é o passo seguinte. Este passo só prepara o Layouter para as
receber.

---

## Decisões já tomadas

1. **Escopo**: **AMPLO**. O Layouter aceita `StyleChain` no
   pipeline interno, não apenas na interface pública. Isto
   antecipa a activação de `#set`/`#show` num passo futuro
   sem exigir segundo refactor do Layouter.
2. **Forma do estilo no `FrameItem`**: adiada para 100.A. O
   inventário recomenda com base em dados empíricos. Gate
   objectivo em 100.A.
3. **DEBT-48 fecha neste passo**. Se a refactorização revelar
   aspectos que excedem o âmbito, abrir DEBT novo em vez de
   deixar DEBT-48 parcial.

---

## Escopo

**Dentro**:
- `01_core/src/entities/layout_types.rs` — `FrameItem::Text` e,
  possivelmente, remoção/redução de `TextStyle`.
- `01_core/src/rules/layout/` — `Layouter`, métodos internos,
  sub-módulos (metrics, cursor, placement, etc.) que hoje
  passam ou lêem `TextStyle`.
- `01_core/src/rules/layout/tests.rs` — testes que constroem
  `FrameItem::Text { ..., style: TextStyle { ... } }`.
- `03_infra/src/export.rs` — consumo de estilo para selecção
  de fonte no PDF.
- Testes cross-module (paridade, integração) que referenciam
  `TextStyle`.

**Fora**:
- `eval_markup` e activação de `#set`/`#show`. Se for necessário
  tocar o eval para compilar (ex: se `Content::Styled` passa a
  ser produzido no Layouter como antes), permitido; mas sem
  adicionar semântica nova de `#set`.
- `Introspection`, `Engine<'a>`, materialização de outros
  stubs.
- `LazyHash` em L3 (continua fora deste passo).

---

## Sub-passos

### 100.A — Inventário e decisão da forma

**Parte 1 — Inventário de consumo de `TextStyle`**:

1. Grep por `TextStyle` em todo o workspace (L1 + L3). Classificar
   cada match:
   - **C (construção)**: `TextStyle { bold, italic, size, ... }`
     ou `TextStyle::regular(...)` / `bold(...)` / `italic(...)`.
   - **L (leitura de campo)**: `.bold`, `.italic`, `.size`, `.fill`,
     etc.
   - **T (tipo em assinatura/struct)**: `style: TextStyle` como
     parâmetro ou campo.
2. Para cada local de **L**, anotar *quais* campos são lidos.
   Isto determina o catálogo mínimo da forma "resolvida" (se for
   essa a opção).
3. Grep por referências ao `FrameItem::Text { .. , style, .. }`.
   Contar sítios que dependem da estrutura exacta do `style`.
4. Escrever em
   `00_nucleo/diagnosticos/inventario-textstyle-passo-100.md`:
   ```
   TextStyle:
     Construções (C): N
     Leituras (L) por campo:
       .bold    → K sítios
       .italic  → M sítios
       .size    → P sítios
       .fill    → Q sítios (se existir)
       .heading_level → R sítios (se existir)
     Tipos em assinatura (T): S
     FrameItem::Text dependentes da forma exacta: U
   ```

**Parte 2 — Inventário do pipeline do Layouter**:

1. Identificar onde o `Layouter` hoje constrói ou propaga
   `TextStyle` (em campos `self.style`, em argumentos de
   métodos internos).
2. Para cada sítio, decidir se a substituição natural é:
   - `StyleChain<'a>` (o método recebe cadeia como referência).
   - `Styles` (owned, o método recebe a colecção).
   - Struct resolvido (o método recebe a vista achatada já
     computada).
3. Identificar o **ponto de resolução** — onde a cadeia é lida
   pela última vez e os valores finais são computados. Hoje,
   este ponto é implícito (o `TextStyle` já é a resolução). No
   pipeline amplo, o Layouter mantém a cadeia e resolve só
   quando emite `FrameItem::Text`.

**Parte 3 — Decisão da forma do `FrameItem`**:

Duas opções finais (a terceira do debate inicial — "StyleChain
owned" — está descartada: lifetimes fariam `FrameItem` não
`'static`, o que quebra muitos callers).

- **SR (Struct Resolvido)**: `FrameItem::Text { ..., style: Resolved }`
  onde `Resolved` é um struct com os campos que o export e
  render leem. É `TextStyle` renomeado e estendido com `fill`,
  `heading_level`, etc. Zero lifetime. Construído pelo Layouter
  no ponto de resolução.
- **SO (Styles Owned)**: `FrameItem::Text { ..., styles: Styles }`
  carrega a colecção de deltas. A resolução é feita pelo
  consumidor (export) através de uma função
  `resolve_on_frame(styles: &Styles) -> Resolved` em L1.

Gate de decisão:
- Se >30 sítios de leitura (L) directa de campos e o export
  resolve cedo, preferir **SR**.
- Se o objectivo incluir preservação da estrutura de deltas
  (para inspecção futura, re-render condicional, etc.),
  preferir **SO**.

Escolha por defeito (baseline): **SR**. O cristalino hoje não
tem caso de uso para preservar deltas no frame, e SR mantém o
contrato simples para o export. A opção SO deve ser justificada.

**Critério de saída de 100.A**: inventário escrito; decisão
SR/SO registada com justificação empírica; catálogo de campos
do struct resolvido (se SR).

### 100.B — ADR nova: forma de estilo no FrameItem

1. Criar `00_nucleo/adr/typst-adr-00NN-frameitem-style.md` com
   status `PROPOSTO`.
2. Conteúdo:
   - Contexto: DEBT-48; escopo amplo escolhido no Passo 100;
     evidência empírica do 100.A.
   - Decisão SR/SO.
   - Catálogo dos campos do struct resolvido (se SR).
   - Contrato: "`FrameItem::Text` é a saída do Layouter; o
     consumidor (export) lê propriedades finais sem precisar de
     aceder a `StyleChain`".
   - Relação com ADR-0038: `StyleChain` continua em L1;
     `FrameItem` deixa de depender de `TextStyle` plano; a
     função de resolução (se SO) vive em L1 para `export` poder
     chamá-la sem importar L1 via interfaces opacas.
   - Plano de activação: quando `#set` for activado, o Layouter
     já aceita `StyleChain`; o eval só precisa de produzir
     `Content::Styled` e chamar `chain.push_styles(...)`.
3. Promovida a `EM VIGOR` em 100.E após validação empírica.

### 100.C — Refactorização

Ordem obrigatória; cada etapa tem de compilar e passar testes
antes da próxima.

**100.C.1 — Criar o tipo de saída (só se SR)**:

1. Definir `struct Resolved { bold, italic, size, fill?,
   heading_level?, ... }` em
   `01_core/src/entities/layout_types.rs` ou em módulo dedicado
   conforme coesão (ADR-0037).
2. Implementar `From<&StyleChain<'_>> for Resolved` (ou função
   livre equivalente) com defaults claros para cada campo.
3. Testes: cadeia vazia dá defaults; cadeia com Bold dá
   `Resolved { bold: true, ... }`; cadeia aninhada resolve
   top-wins (consistente com testes do Passo 99).

**Se SO**: implementar `fn resolve(styles: &Styles) -> Resolved`
em L1 (o tipo `Resolved` continua a existir para o consumidor,
mas não vive dentro do `FrameItem`).

**100.C.2 — Mudar `FrameItem::Text`**:

1. Alterar a assinatura:
   - SR: `Text { pos, text, style: Resolved }`.
   - SO: `Text { pos, text, styles: Styles }`.
2. Actualizar todos os construtores e matches no workspace.
   Seguir os erros do compilador como lista de trabalho.
3. Actualizar `Frame::plain_text()`, `Frame::push()`, e métodos
   que tocam `FrameItem`.
4. `cargo test -p typst-core`. Esperam-se muitos erros aqui —
   resolver sequencialmente.

**100.C.3 — Pipeline do Layouter**:

1. No Layouter, substituir `self.style: TextStyle` por
   `self.chain: StyleChain<'a>` (ou representação equivalente).
2. Métodos internos que hoje recebem `style: TextStyle` passam
   a receber `chain: &StyleChain<'_>` (ou owned `Styles`
   conforme SR/SO).
3. Ponto de resolução: no sítio onde `FrameItem::Text` é emitido,
   chamar `Resolved::from(&chain)` (SR) ou guardar `chain.flatten()`
   (SO).
4. `Content::Styled` no Layouter: em vez de ser transparente
   (como no Passo 99 COEX), passa a fazer push/pop na cadeia
   interna. Este é o único ponto onde o comportamento *muda*:
   antes, `Content::Styled` era ignorado pelo Layouter; agora é
   processado.
5. Testes: construir `Content::Styled` manualmente (como no
   99.D), verificar que a saída de layout reflecte os estilos.

**100.C.4 — `export.rs` em L3**:

1. O export lê o estilo de cada `FrameItem::Text` para escolher
   `F1`/`F2`/`F3` no PDF.
2. SR: export lê `style.bold`, `style.italic` directamente como
   hoje (só muda o nome/tipo).
3. SO: export chama `resolve(&styles)` e depois lê os campos.
4. Actualizar; correr `cargo test -p typst-infra`.

**100.C.5 — Remover `TextStyle` (se SR) ou deprecar**:

1. Se todos os sítios foram actualizados, remover
   `pub struct TextStyle` e imports associados.
2. Se algum teste legacy ainda usa `TextStyle` de forma isolada
   e converter dá retrabalho desproporcional, deixar
   `TextStyle` como alias: `pub type TextStyle = Resolved;` e
   registar em DEBT novo ou no relatório 100.E.

**Critério de saída de 100.C**: `cargo test --workspace` passa;
zero violations no lint.

### 100.D — Teste de integração amplo

Objectivo: demonstrar que o pipeline completo
(Content → Layouter → Frame → export) funciona com estilo
encadeado, **sem** passar pelo eval.

1. Teste principal (em `rules/layout/tests.rs`):
   ```rust
   // Construir manualmente árvore com Styled aninhado
   let hello = Content::text("hello");
   let bold_hello = Content::Styled(
       Box::new(hello),
       Styles::from_iter([Style::Bold(true), Style::Size(Pt(18.0))]),
   );
   let doc = layout(&bold_hello);
   // SR: verificar que FrameItem::Text tem Resolved { bold: true, size: 18pt }
   // SO: verificar que resolve(&frame.styles) dá o mesmo
   ```
2. Teste de encadeamento (variação do 99.D com saída real do
   layout):
   ```rust
   let inner = Content::Styled(
       Box::new(Content::text("hi")),
       Styles::from_iter([Style::Italic(true)]),
   );
   let outer = Content::Styled(
       Box::new(inner),
       Styles::from_iter([Style::Bold(true), Style::Italic(false)]),
   );
   // Esperado: bold=true (outer), italic=true (inner, top-wins)
   ```
3. Teste cross-layer (em testes de integração L3, se existem):
   construir `Content::Styled`, chamar `layout` e `export_pdf`,
   verificar que o PDF resultante usa a fonte correcta
   (Helvetica-Bold para `bold=true`).

### 100.E — Encerramento

1. Grep final: `TextStyle` não deve aparecer (ou aparece apenas
   como alias opcional com comentário explicativo).
2. `cargo test --workspace`: ≥ linha de base + novos testes
   (esperado: 780 L1 + alguns novos de 100.C.1 e 100.D).
3. `crystalline-lint` zero violations.
4. `grep -r 'LazyHash' 01_core/src/` retorna zero usos reais
   (ADR-0016 continua em vigor).
5. DEBT-48 marcado como **ENCERRADO (Passo 100)** em
   `01_core/DEBT.md`.
6. DEBT-1 revisto: o Passo 99 marcou-o como PARCIALMENTE
   RESOLVIDO. Depois do 100, verificar se pode ficar ENCERRADO
   (a activação de `#set` no eval é trabalho futuro, mas o
   pipeline Layouter → export já está preparado; a dívida
   estrutural foi paga).
7. ADR-00NN (forma do `FrameItem`): promovida a `EM VIGOR`.
8. Relatório `typst-passo-100-relatorio.md`:
   - Decisão SR/SO com razão.
   - Contagem antes/depois por categoria (C, L, T).
   - Campos do struct resolvido (se SR).
   - Como ficou o ponto de resolução no Layouter.
   - Novo DEBT aberto (se algum).
   - Estado final de DEBT-1 e DEBT-48.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 100.A escrito.
2. ADR-00NN criada e promovida a `EM VIGOR`.
3. `FrameItem::Text` usa a forma decidida (SR ou SO); zero
   referências a `TextStyle` no seu campo de estilo (ou
   mantido como alias com justificação).
4. Layouter aceita `StyleChain` no pipeline interno.
5. `Content::Styled` é processado pelo Layouter (push/pop na
   cadeia), já não é transparente.
6. `export.rs` consome a nova forma.
7. Testes 100.D passam.
8. `cargo test --workspace` passa com contagem ≥ linha de
   base.
9. `crystalline-lint` zero violations.
10. DEBT-48 fechado; DEBT-1 revisto.
11. Relatório 100.E escrito.

---

## O que pode sair errado

- **Lifetime contagion**. `StyleChain<'a>` no pipeline do
  Layouter pode arrastar `'a` para vários métodos e structs. Se
  o `Layouter` hoje é `'static`, passa a ser `Layouter<'a>` ou
  equivalente. Isto não é problema em si, mas se o lifetime se
  propagar para tipos públicos de L1 que eram antes `'static`,
  pode quebrar callers em L3. Mitigação: manter o ponto de
  resolução **dentro** do Layouter; a cadeia nunca escapa para
  a API pública.
- **`Content::Styled` mal processado**. No Passo 99 COEX, o
  Layouter trata `Styled` como transparente. Ao tornar-se
  activo, qualquer bug na semântica push/pop produz saída
  diferente. Os testes do 99.D (que verificam resolução) não
  chegam — precisam de testes que comparem output do Layouter
  antes e depois para `Content` que **não** contém `Styled`,
  para garantir que nada muda no caso simples.
- **Export em L3 perde acesso a contexto**. Se a decisão for SO
  e a função `resolve` viver em L1 mas depender de tipos que
  L3 não importa directamente, pode surgir erro de visibilidade.
  Resolver com `pub` no módulo correcto; não com
  `pub(crate)` que esconde a função do consumidor L3.
- **`TextStyle` em testes legacy**. Testes antigos que usam
  `TextStyle::bold(Pt(12.0))` vão todos quebrar. Grep cuidadoso
  no início para não descobrir 40 testes quebrados só na
  compilação final.
- **Ponto de resolução repetido**. Se `Resolved::from(&chain)`
  for chamada em cada palavra (hot loop do word-wrap), o custo
  pode aparecer em perfil. Aceitável neste passo (não é
  optimização prematura); registar como DEBT se o perfil
  mostrar problema.
- **SR com demasiados campos**. Se o inventário 100.A revelar
  15+ campos distintos em leituras, `Resolved` torna-se um
  struct inflado. Alternativa: `Resolved` agrupa por subsistema
  (`ResolvedText`, `ResolvedBox`, `ResolvedColour`) em vez de
  um struct plano. Decidir em 100.A com base no catálogo.

---

## Notas operacionais

- Este passo não toca visibilidade gratuitamente. Mudanças em
  `pub(super)` seguem a escada da nota da Regra 3 (ADR-0037),
  com justificação.
- Novos ficheiros (se houver) precisam de smoke test V2.
- O teste de integração 100.D **não** passa pelo `eval_markup`.
  A activação de `#set` no eval é passo separado.
- Se em algum ponto o trabalho exceder o escopo (ex: precisar
  de materializar `Introspection` para continuar), **parar** e
  abrir DEBT em vez de arrastar.
