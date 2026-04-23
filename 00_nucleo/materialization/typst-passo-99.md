# Passo 99 — Fundação de `Style`, `Styles` e `StyleChain` em L1

**Série**: 99 (passo único de construção; sub-passos apenas de
inventário e verificação).
**Precondição**: Passo 98 encerrado; `EvalContext` com 4 campos
Regra 4; 764 L1 + 174 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0016 (LazyHash fora de L1 — não revogada
por este passo), ADR-0026 (divergência intencional de Content —
precedente para enum linear sem proc macros), ADR-0036 (atomização),
ADR-0037 (coesão por domínio).
**ADRs novas a criar**: ADR-00XX "Sistema de estilos em L1"
(promovida a `EM VIGOR` neste passo após validação empírica,
padrão da ADR-0037).

---

## Objectivo

Materializar em L1 a fundação do sistema de estilos do Typst:

- `Style` — enum de propriedades individuais (superconjunto,
  preparado para futuro).
- `Styles` — colecção de `Style`.
- `StyleChain` — lista ligada de deltas com semântica set/show.
- `Content::Styled(Box<Content>, Styles)` — nova variante de
  Content.

`LazyHash<T>` continua fora de L1 (ADR-0016 mantém-se). A
memoização/hashing com `LazyHash` é trabalho de L3 num passo
futuro, quando o pipeline incremental real for activado.

O passo fecha o DEBT de `StyleChain` aberto no Passo 22.

**Não faz**: não activa `#set`/`#show` no eval. Não produz saída
estilizada diferente do actual. Fornece só a fundação testada e
um teste de integração conceptual que demonstra que a fundação
é usável (não que está em uso no pipeline real).

---

## Decisões já tomadas

Registadas em conversa; este bloco existe para referência do
executante:

1. **Âmbito do enum `Style`**: superconjunto, preparado para
   futuro. Replicar o catálogo de propriedades do vanilla
   relevantes, não só as 3 que `TextStyle` actual cobre. O
   inventário (99.A) determina a lista exacta.
2. **Coexistência com `TextStyle`**: decidir em 99.A com base
   no inventário. A opção "coexistência temporária com ponte"
   é permitida; a opção "substituição completa em 99" é
   permitida. O sub-passo 99.A produz a recomendação.
3. **Até onde ir**: só fundação + teste de integração
   conceptual. Não tocar no eval para activar `#set`. O teste
   de integração constrói manualmente um `Content::Styled` e
   verifica que `StyleChain` resolve as propriedades
   correctamente. Isto valida a API sem exigir mudanças no
   `eval_markup`.

---

## Escopo

**Dentro**:
- `01_core/src/entities/style.rs` — novo ficheiro; enum `Style`
  e struct `Styles`.
- `01_core/src/entities/style_chain.rs` — novo ficheiro;
  `StyleChain<'a>` com semântica de lista ligada.
- `01_core/src/entities/content.rs` — adicionar variante
  `Content::Styled(Box<Content>, Styles)` e ajustar métodos
  `plain_text()`, `is_empty()`, etc.
- `01_core/src/entities/layout_types.rs` — possivelmente
  `TextStyle` (ver 99.A).
- `01_core/src/rules/layout.rs` — possivelmente ponte de
  `StyleChain` → `TextStyle` para o Layouter actual (ver 99.A).
- ADR nova formalizando o mapa de camadas.

**Fora**:
- Activação de `#set` / `#show` no `eval_markup`.
- Qualquer mexida em `LazyHash` (continua fora de L1 por
  ADR-0016).
- Introspecção (`Introspection` / `Sink` / `Engine<'a>`) —
  passos futuros separados.
- Memoização de `Styles` em L3 com `comemo` — passo futuro.

---

## Sub-passos

### 99.A — Inventário e decisão de coexistência

**Parte 1 — Catálogo de propriedades (enum `Style`)**:

1. Ler `lab/typst-original/crates/typst-library/src/foundations/styles.rs`
   e ficheiros relacionados para identificar o enum `Style`/`Property`
   do vanilla e a sua lista de variantes.
2. Identificar quais variantes estão actualmente usadas pelo
   cristalino (via `TextStyle` e pela lógica do `Layouter`):
   `bold`, `italic`, `size`, e qualquer outra que apareça.
3. Identificar um superconjunto razoável para L1 — propriedades
   que são domínio puro (sem I/O, sem referências a fontes reais
   do filesystem) e que cobrem o grupo de `text`, `heading`,
   `raw`, `list`, cores, espaçamento. Excluir propriedades que
   exigem tipos ainda não materializados (ex: se uma propriedade
   referencia `Font` real, que é stub em L1, adiar essa variante
   ou usar placeholder).
4. Escrever o catálogo em
   `00_nucleo/diagnosticos/inventario-style-passo-99.md` com
   formato:
   ```
   Variantes do enum Style (L1):
     Propriedade          Tipo                   Origem vanilla
     text.bold            bool                   TextElem::bold
     text.italic          bool                   TextElem::italic
     text.size            Pt                     TextElem::size
     text.fill            Color (ADR-NN)         TextElem::fill
     heading.level        u8                     HeadingElem::level
     ...
   Variantes adiadas (bloqueadas por tipos não materializados):
     text.font            requer Font real       adiar até Passo NN
     ...
   ```

**Parte 2 — Decisão sobre `TextStyle`**:

1. Grep por `TextStyle` em `01_core/` e `03_infra/`. Contar:
   - Sítios de construção (onde `TextStyle { ... }` é criado).
   - Sítios de consumo (onde campos `.bold`, `.italic`, `.size`
     são lidos).
   - Testes que dependem da estrutura exacta.
2. Duas opções:
   - **Opção SUB (substituição completa)**: `TextStyle` é
     removido. `FrameItem::Text` passa a ter
     `styles: StyleChain<'static>` ou equivalente. Afecta
     layout, export, todos os testes de frame. Elimina a
     dívida de uma vez.
   - **Opção COEX (coexistência com ponte)**: `TextStyle`
     mantém-se como "vista achatada para o Layouter actual".
     Uma função `fn resolve_text_style(&StyleChain) -> TextStyle`
     faz a ponte. A dívida de `TextStyle` plano persiste, mas
     reduzida (só no Layouter/export; `Content::Styled` usa
     `StyleChain`). Permite passo mais pequeno.
3. Critério objectivo: se os sítios de consumo de `TextStyle`
   passam dos 15, preferir COEX (substituição fica num passo
   dedicado). Se ficam abaixo, preferir SUB. O inventário
   decide; registar a recomendação no ficheiro de inventário.

**Critério de saída de 99.A**: inventário escrito; decisão
SUB/COEX recomendada com número de sítios; catálogo de `Style`
com lista de variantes e lista de adiadas.

### 99.B — ADR nova: sistema de estilos em L1

1. Criar `00_nucleo/adr/typst-adr-00NN-sistema-estilos-l1.md`
   com status `PROPOSTO`.
2. Conteúdo obrigatório:
   - **Contexto**: DEBT do Passo 22; estado do EvalContext após
     Passo 98; razão para fazer agora.
   - **Mapa de camadas**: `Style`, `Styles`, `StyleChain` em L1;
     `LazyHash<T>` em L3 quando memoização incremental for
     activada; ADR-0016 permanece em vigor.
   - **Divergência do vanilla**: enum linear em vez de proc
     macros `#[elem]`, alinhado com ADR-0026 (mesmo precedente).
   - **`StyleChain` como estrutura**: lista ligada imutável de
     blocos; cada bloco é referência a `Styles` + apontador para
     o pai. Lifetime `'a` na struct para permitir encadeamento
     sem alocação por push.
   - **Decisão SUB vs COEX** (da 99.A): registada.
   - **Variantes adiadas do enum `Style`**: listadas com razão.
   - **O que esta ADR não decide**: quando `#set`/`#show` são
     activados; quando `LazyHash` vai para L3; quando Font real
     entra em L1 (se entrar).
3. Promoção a `EM VIGOR` apenas no sub-passo 99.E, após
   validação empírica (padrão ADR-0037).

**Critério de saída**: ficheiro de ADR criado com status
`PROPOSTO`.

### 99.C — Materialização em L1

Ordem obrigatória (cada etapa tem de compilar e passar testes
antes da próxima):

**99.C.1 — `Style` e `Styles`**:

1. Criar `01_core/src/entities/style.rs`.
2. Definir `enum Style` com as variantes do catálogo 99.A.
3. Definir `struct Styles(Vec<Style>)` (ou `EcoVec<Style>` se
   já autorizado em L1) com métodos mínimos: `new()`, `push()`,
   `iter()`, `is_empty()`, `len()`. Sem memoização, sem hash.
4. Adicionar a `entities/mod.rs`.
5. Testes unitários: construção, iteração, igualdade. Mínimo
   5 testes.
6. `cargo test -p typst-core`. Parar se falhar.

**99.C.2 — `StyleChain<'a>`**:

1. Criar `01_core/src/entities/style_chain.rs`.
2. Definir:
   ```rust
   pub struct StyleChain<'a> {
       head: Option<&'a Styles>,
       tail: Option<&'a StyleChain<'a>>,
   }
   ```
   (A forma exacta é decisão do executante; esta é ilustrativa.
   O importante: lista ligada imutável com lifetime.)
3. Métodos mínimos:
   - `empty() -> StyleChain<'static>`.
   - `chain(&'a self, styles: &'a Styles) -> StyleChain<'a>`.
   - `resolve<T>(&self, query: StyleQuery<T>) -> Option<T>`
     onde `StyleQuery` é um selector tipado para uma propriedade.
     Alternativa mais simples: métodos directos como
     `get_bold(&self) -> bool`, `get_size(&self) -> Pt`, com
     defaults. O executante escolhe a forma mais limpa para o
     catálogo actual.
4. Semântica de resolução: sobe a cadeia até encontrar a
   primeira variante que corresponde; se nenhuma, devolve
   default.
5. Testes unitários: cadeia vazia devolve defaults; cadeia com
   um bloco resolve; cadeia aninhada — o bloco mais recente
   prevalece; cadeia com delta parcial — propriedade não
   definida no topo cai no pai. Mínimo 6 testes.
6. `cargo test -p typst-core`. Parar se falhar.

**99.C.3 — `Content::Styled`**:

1. Adicionar variante `Content::Styled(Box<Content>, Styles)`
   ao enum em `01_core/src/entities/content.rs`.
2. Actualizar métodos existentes (`plain_text()`, `is_empty()`,
   e o que mais for afectado por `match` exaustivo).
   `plain_text()` ignora os estilos — só percorre o corpo.
3. Testes: `Content::Styled(Box::new(Content::text("x")), Styles::new()).plain_text()
   == "x"`. Mínimo 3 testes.
4. `cargo test -p typst-core`.

**99.C.4 — Ponte para o Layouter (condicional)**:

- Se a decisão 99.A foi **SUB**: substituir `TextStyle` por
  `StyleChain<'static>` (ou equivalente com lifetime resolvido)
  em `FrameItem::Text` e no Layouter. Actualizar `export.rs`
  em L3. Actualizar todos os testes afectados.
- Se a decisão 99.A foi **COEX**: implementar função
  `resolve_text_style(chain: &StyleChain) -> TextStyle` e usá-la
  onde o Layouter hoje lê o `TextStyle`. `TextStyle` continua a
  existir; a semântica de fonte/tamanho passa a sair da
  resolução da cadeia. Testes existentes permanecem inalterados.

**Critério de saída de 99.C**: ficheiros criados; `cargo test
--workspace` passa; `crystalline-lint` zero violations.

### 99.D — Teste de integração conceptual

Objectivo: demonstrar que a fundação é usável **sem activar
`#set` no eval**. Construir manualmente uma árvore de `Content`
com `Content::Styled` e verificar que o layout produz a saída
correcta.

1. Criar teste em `01_core/src/entities/style_chain.rs`
   (secção `#[cfg(test)]`) ou em `01_core/src/rules/layout/tests.rs`,
   conforme mais coerente com a ADR-0037.
2. Teste mínimo:
   ```rust
   // pseudo-código do teste
   let inner = Content::text("hello");
   let styles = Styles::from_iter([Style::Bold(true), Style::Size(Pt(18.0))]);
   let styled = Content::Styled(Box::new(inner), styles);
   let doc = layout(&styled);
   // Verificar que o FrameItem resultante tem bold=true e size=18pt
   assert!(doc.pages[0].items.iter().any(|i| match i {
       FrameItem::Text { style, .. } => style.bold && style.size == Pt(18.0),
       _ => false,
   }));
   ```
3. Teste secundário — encadeamento:
   ```rust
   // Styled dentro de Styled — o delta mais profundo sobrepõe
   let inner = Content::text("hi");
   let bold_level = Content::Styled(
       Box::new(inner),
       Styles::from_iter([Style::Italic(true)]),
   );
   let outer = Content::Styled(
       Box::new(bold_level),
       Styles::from_iter([Style::Bold(true), Style::Italic(false)]),
   );
   // Resolução do texto: bold=true (do outer), italic=true (do inner — outer
   //   definiu false mas inner sobrepõe porque é "mais perto" do texto)
   ```

Regra de resolução: **o delta mais próximo do texto ganha**.
Confirmar esta direcção lendo o vanilla em 99.A; se o vanilla
resolve na direcção oposta (raiz ganha), alinhar com o vanilla
por paridade funcional (ADR-0033) e registar a decisão na
ADR-00NN.

**Critério de saída**: testes passam; saída do layout reflecte
os estilos encadeados.

### 99.E — Encerramento e promoção de ADR

1. Grep final por `TextStyle` se decisão foi SUB: zero matches
   em L1.
2. Grep por `LazyHash` em L1: zero matches (garantia ADR-0016).
3. `cargo test --workspace`: contagem ≥ linha de base mais os
   testes novos de 99.C e 99.D (esperado: 764 + ~14 = ~778 L1).
4. `crystalline-lint` zero violations.
5. DEBT `StyleChain` do Passo 22: marcar como **ENCERRADO
   (Passo 99)** em `01_core/DEBT.md`. Se a decisão foi COEX,
   marcar como **PARCIALMENTE RESOLVIDO (Passo 99)** e abrir
   novo DEBT "Substituir TextStyle por StyleChain no Layouter
   e export" com escopo claro.
6. ADR-00NN: promover de `PROPOSTO` para `EM VIGOR` (padrão
   ADR-0037).
7. Escrever `typst-passo-99-relatorio.md` com:
   - Decisão SUB/COEX tomada e razão empírica.
   - Contagem de variantes no enum `Style` e lista de adiadas.
   - Exemplo de resolução de estilos (trecho de teste).
   - Estado de `DEBT-StyleChain`: encerrado ou
     parcialmente-resolvido com DEBT sucessor.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 99.A escrito com catálogo e decisão SUB/COEX.
2. ADR-00NN criada e promovida a `EM VIGOR`.
3. `Style`, `Styles`, `StyleChain` em L1 com testes unitários.
4. `Content::Styled` adicionado; enum exaustivo em todos os
   matches.
5. Ponte ou substituição para o Layouter conforme decisão
   99.A.
6. Teste de integração conceptual passa.
7. `cargo test --workspace` passa com contagem ≥ linha de base
   + novos.
8. `crystalline-lint` zero violations.
9. DEBT `StyleChain` do Passo 22 fechado ou reduzido.
10. Relatório 99.E escrito.

---

## O que pode sair errado

- **Catálogo de `Style` explode**. O vanilla tem centenas de
  propriedades. Se o inventário 99.A identificar mais de ~30
  variantes "plausíveis para superconjunto", parar e restringir
  à lista "mínimo + tudo o que layout já usa hoje + 5 de
  margem". "Superconjunto" não é "replicar tudo"; é "preparar
  vocabulário coerente para o que vem a seguir". Registar as
  outras como adiadas.
- **Lifetime de `StyleChain` entra em conflito com
  `EvalContext`**. Se surgir necessidade de passar `StyleChain`
  através do eval (o que este passo **não faz**), pode aparecer
  conflito com o lifetime `'w` do `world`. Como o eval não é
  tocado neste passo, isto só aparece quando `#set` for
  activado. Não resolver aqui.
- **Regra de resolução (topo vs raiz) diverge do vanilla**. Se
  o vanilla resolver "raiz ganha" e o cristalino implementar
  "topo ganha", os testes de paridade (Passo 9) vão falhar
  quando `#set` for activado. 99.A tem de confirmar a direcção
  correcta lendo o vanilla.
- **`EcoVec` vs `Vec` para `Styles`**. `EcoVec` dá clone O(1),
  relevante no hot path de eval. Se `EcoVec` já está autorizado
  em L1 (ADR-0035), usar. Se não, usar `Vec` e deixar
  substituição para optimização futura.
- **Invariante de `Content::Styled`**: deve ser exaustivo em
  todos os `match Content` do código. Um `match` não-exaustivo
  em layout/export é warning que se torna erro em CI. Verificar
  com `cargo build -p typst-core` após 99.C.3.

---

## Notas operacionais

- Este passo não toca visibilidade. Auditoria de `pub(super)`
  foi fechada no Passo 97.
- Novas declarações neste passo seguem a nota da Regra 3 da
  ADR-0037: preferir métodos; campos `pub(super)` só com
  justificação em comentário.
- Novos ficheiros devem ter smoke test V2 (requisito do linter)
  — pode contar no aumento de testes.
- Se o lint gerar falsos positivos por uso novo de
  `pub(crate)` em `Style`/`Styles`/`StyleChain` (tipos que
  precisam de ser visíveis para `Content::Styled` mas não
  devem poluir a API externa), aplicar a escada de visibilidade
  como no Passo 97 — privado → `pub(super)` → `pub(in path)`
  → `pub(crate)` → `pub`.
