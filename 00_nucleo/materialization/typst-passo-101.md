# Passo 101 — Consolidação: remover `Content::Strong`/`Emph`, usar `Content::Styled`

**Série**: 101 (passo único de construção; sub-passos de
inventário e verificação).
**Precondição**: Passo 100 encerrado; `Content::Styled` activo no
Layouter; `TextStyle` com `fill` + `heading_level`; 783 L1 + 174 L3
+ 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0026 (divergência intencional de Content),
ADR-0033 (paridade funcional com vanilla), ADR-0036 (atomização),
ADR-0037 (coesão por domínio), ADR-0038 (sistema de estilos em L1),
ADR-0039 (forma de estilo no `FrameItem`).

---

## Objectivo

Remover as variantes `Content::Strong(Box<Content>)` e
`Content::Emph(Box<Content>)` do enum `Content`. A sintaxe
`*bold*` e `_italic_` do Typst continua a funcionar, mas produz
`Content::Styled([Style::Bold(true)], body)` e
`Content::Styled([Style::Italic(true)], body)` directamente no
`eval_markup`.

Consequências esperadas:

1. Enum `Content` fica mais pequeno (menos duas variantes).
2. Layouter perde os arms dedicados para `Strong` e `Emph`; passa
   a ter apenas o arm `Styled` (já introduzido no Passo 100).
3. `plain_text()`, `is_empty()`, `map_text()`, `map_content()`,
   `PartialEq` em `Content` perdem também os arms dedicados.
4. `Content::Heading { level, body }` **permanece inalterado**.
   Tem semântica adicional (nível numérico para introspecção
   futura) que não se reduz a um `Styles` só.

O passo não introduz funcionalidade nova. É consolidação.

---

## Decisões já tomadas

1. **Âmbito**: Strong e Emph. Heading fica. Heading será
   reavaliado num passo separado, provavelmente depois de
   `Introspection` estar materializado.
2. **Onde fazer a conversão**: no `eval_markup`, no sítio onde
   `SyntaxKind::Strong` e `SyntaxKind::Emph` são processados. As
   variantes `Strong`/`Emph` do enum desaparecem completamente;
   **não** há adapter/normalização intermédia.
3. **Paridade funcional preservada**: para o mesmo input Typst,
   a saída visual tem de ser idêntica ao estado pós-Passo 100.
   Os testes de paridade (Passo 9) e os testes existentes de
   bold/italic têm de passar sem alteração de asserção.

---

## Escopo

**Dentro**:
- `01_core/src/entities/content.rs` — remover variantes e
  métodos `Content::strong(...)` / `Content::emph(...)`.
- `01_core/src/rules/eval/markup.rs` (ou onde `eval_markup` vive
  agora após a série 96.x) — substituir construção.
- `01_core/src/rules/layout/mod.rs` — remover arms dedicados.
- Qualquer outro ficheiro com `match Content { ... Strong ... Emph ... }`.
- Testes afectados.

**Fora**:
- `Content::Heading`.
- Activação de `#set`/`#show` (passo futuro).
- Mexer em `TextStyle`, `Style`, `Styles`, `StyleChain`.
- `Introspection`, `Engine<'a>`.

---

## Sub-passos

### 101.A — Inventário

1. Grep por `Content::Strong` e `Content::Emph` em todo o
   workspace (L1, L3, testes, ficheiros de documentação). Para
   cada match, classificar:
   - **E (enum)**: declaração da variante em `content.rs`.
   - **C (construtor)**: chamada a `Content::strong(...)` ou
     `Content::emph(...)` ou construção literal
     `Content::Strong(Box::new(...))`.
   - **M (match arm)**: `Content::Strong(body) => ...` ou
     equivalente para Emph.
   - **T (teste)**: asserção ou setup que referencia as
     variantes.
2. Escrever em
   `00_nucleo/diagnosticos/inventario-strong-emph-passo-101.md`:
   ```
   Content::Strong:
     E: 1 (content.rs)
     C: N sítios
     M: K sítios (layout, plain_text, is_empty, map_text,
                  map_content, PartialEq, introspect, ...)
     T: P sítios

   Content::Emph:
     (idem)

   Construtores Content::strong / Content::emph:
     (listar cada um)
   ```
3. Verificação cruzada: os testes de paridade (se existem) não
   devem depender da **estrutura interna** de `Content`, apenas
   do `plain_text()` ou de saída visual. Se algum teste assertar
   `matches!(c, Content::Strong(_))`, assinalar — terá de ser
   reescrito para assertar sobre `Content::Styled`.

Critério de saída: inventário escrito. Se o número de match
arms (M) for >30, parar e avaliar se faseaar (strong primeiro,
depois emph) faz sentido. Caso contrário, avançar.

### 101.B — Alteração do `eval_markup`

1. Localizar os arms `SyntaxKind::Strong` e `SyntaxKind::Emph`
   em `eval_markup`. Hoje produzem:
   ```rust
   parts.push(Content::strong(body));
   // ou
   parts.push(Content::Strong(Box::new(body)));
   ```
2. Substituir por:
   ```rust
   parts.push(Content::Styled(
       Box::new(body),
       Styles::from_iter([Style::Bold(true)]),
   ));
   ```
   e análogo com `Italic(true)` para Emph.
3. Uso de `Styles::from_iter(...)` ou outra factory — usar a
   que já existe e está testada em 99. Se não existir uma
   factory limpa, criar `Styles::single(s: Style) -> Self` como
   helper mínimo. Não re-desenhar a API de `Styles` neste passo.
4. `cargo test -p typst-core`. Se testes que constroem
   `Content::Strong` manualmente falharem neste ponto, é
   esperado — serão actualizados em 101.D.

### 101.C — Remoção no Layouter e outros matches

1. Remover arms `Content::Strong(body) => ...` e
   `Content::Emph(body) => ...` no Layouter. O comportamento é
   agora coberto pelo arm `Content::Styled`.
2. Repetir para `plain_text()`, `is_empty()`, `map_text()`,
   `map_content()`, `PartialEq` e qualquer outro match em
   `Content`. O compilador ajuda: ao remover as variantes do
   enum em 101.D, falha exaustivamente em cada match não
   actualizado.
3. Ordem recomendada: primeiro remover os arms (passo 101.C),
   **depois** remover as variantes do enum (passo 101.D). Isto
   evita erros de "variante ausente" durante a transição.
4. `cargo test -p typst-core` e `cargo test -p typst-infra`
   após cada ficheiro tocado.

### 101.D — Remoção das variantes do enum

1. Em `01_core/src/entities/content.rs`:
   - Remover `Strong(Box<Content>)` e `Emph(Box<Content>)` do
     enum.
   - Remover métodos construtores `Content::strong` e
     `Content::emph`.
   - Remover quaisquer implementações auxiliares (ex: `Display`,
     `Debug` custom que referenciem as variantes).
2. `cargo build -p typst-core`. Se houver erros, são matches
   ainda não actualizados em 101.C — voltar a esse passo.
3. Grep final: `Content::Strong` e `Content::Emph` devem ter
   zero matches no workspace (exceptuando comentários
   históricos, se optares por os manter).

### 101.E — Actualização de testes

1. Testes que asseram `matches!(c, Content::Strong(_))` são
   reescritos para assertar sobre o novo formato. Exemplo:
   ```rust
   // Antes:
   assert!(matches!(c, Content::Strong(_)));
   // Depois:
   assert!(matches!(&c,
       Content::Styled(_, styles) if styles.iter().any(|s| matches!(s, Style::Bold(true)))
   ));
   ```
   Para reduzir boilerplate, considerar helper
   `fn is_bold_styled(c: &Content) -> bool` numa secção
   `#[cfg(test)]` partilhada.
2. Testes de paridade com vanilla (Passo 9 e derivados) não
   devem precisar de mudança — comparam saída de parse, não
   resultado de eval. Confirmar.
3. Testes de integração do Passo 99/100 (`Content::Styled`
   encadeado) continuam a passar sem alteração.
4. Se algum teste construía `Content::Strong(Box::new(...))`
   directamente como input, converter para
   `Content::Styled(Box::new(...), Styles::from_iter([Style::Bold(true)]))`.

### 101.F — Verificação de paridade funcional

Este sub-passo é crítico pela ADR-0033 (paridade com vanilla).

1. Executar os testes de paridade existentes
   (`cargo test --workspace`, filtros que tocam paridade).
2. Executar, se houver, corpus de ficheiros `.typ` que testam
   output visual (`Passo 19+`).
3. Verificar que o output do export PDF para um ficheiro com
   `Hello *bold* and _italic_` continua a produzir PDF
   byte-compatível ou visualmente idêntico ao pré-Passo 101.
4. Se o output divergir, a causa mais provável é diferença na
   **ordem** ou **semântica** do push/pop: `Content::Styled`
   com `[Bold(true)]` tem de produzir exactamente o mesmo
   comportamento que o arm `Content::Strong` antigo.

### 101.G — Encerramento

1. Grep final:
   - `Content::Strong` → zero matches (exceptuando comentários
     históricos).
   - `Content::Emph` → idem.
   - `Content::strong(` → zero matches.
   - `Content::emph(` → zero matches.
2. `cargo test --workspace`: contagem ≥ linha de base. Este
   passo é de consolidação; não adiciona funcionalidade nem
   testes novos significativos. Esperado: 783 L1 inalterado (ou
   ±1 se algum teste for removido/reescrito).
3. `crystalline-lint` zero violations.
4. DEBT-1 revisto: o relatório 100 marcou-o como
   "PARCIALMENTE RESOLVIDO (estrutura paga em Passo 100)". O
   101 paga a tarefa "remover wrappers Strong/Emph". Actualizar
   a nota: uma das três tarefas pendentes foi feita.
5. Relatório `typst-passo-101-relatorio.md`:
   - Números de antes/depois.
   - Contagem de match arms removidos.
   - Confirmação de paridade funcional (output idêntico).
   - Nota sobre `Content::Heading` (ficou; razão).
   - Estado dos DEBTs tocados.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 101.A escrito.
2. `eval_markup` produz `Content::Styled` para Strong e Emph.
3. Enum `Content` não tem variantes `Strong` nem `Emph`.
4. Layouter sem arms dedicados a Strong/Emph.
5. Outros matches em `Content` actualizados.
6. Testes migrados para o novo formato.
7. Paridade funcional confirmada (output igual ao pós-Passo 100
   para os mesmos inputs).
8. `cargo test --workspace` passa.
9. `crystalline-lint` zero violations.
10. DEBT-1 actualizado.
11. Relatório 101.G escrito.

---

## O que pode sair errado

- **Heading continua a duplicar Strong semanticamente**.
  Heading aplica bold (via `TextStyle` ou via Styles) **e**
  muda o tamanho. Depois do 101, vai haver um arm
  `Content::Heading { level, body }` que, internamente, faz
  trabalho semelhante ao que `Content::Styled([Bold(true),
  Size(...)], body)` faria. Por decisão (âmbito), isto fica
  como está. Nota: **resistir a arrastar Heading para o 101**.
  É passo separado quando Introspection materializar.

- **Testes de paridade de AST (Passo 9)**. Os testes de
  paridade podem comparar a estrutura **após eval**, não após
  parse. Se um desses testes assert `Content::Strong(_)`,
  falha. Ler o Passo 9 antes de começar para confirmar que
  paridade é apenas sobre árvore de parse.

- **Output visual divergir por diferença de enum tag**. Pouco
  provável, mas possível: se o PDF tem alguma optimização que
  depende da variante de Content (ex: hash, cache), mudar de
  `Strong` para `Styled` muda o hash. Se o Passo 100 introduziu
  alguma memoização de `Content`, verificar.

- **`Styles::from_iter` pode ser caro no hot path do eval**. Se
  o eval cria muitos `Content::Styled` por segundo (um por nó
  Strong/Emph), alocar `Vec<Style>` ou `EcoVec<Style>` para
  cada um adiciona pressão na heap. Mitigação: se
  `Styles::single` (factory para um só elemento) já existe,
  usar. Se não, criar neste passo como helper mínimo. Não é
  optimização prematura — é evitar regressão.

- **`Content::strong(...)` em ficheiros de exemplo ou README**.
  Se houver documentação a demonstrar a API com exemplos de
  `Content::strong`, tem de ser actualizada. Registar no
  inventário.

---

## Notas operacionais

- Este passo não toca `StyleChain`, `Style` nem visibilidade.
- O DEBT-1 tem 3 tarefas pendentes listadas no relatório do
  Passo 100:
  1. Activar `#set`/`#show` no eval.
  2. **Remover wrappers Strong/Emph** ← este passo.
  3. Propriedades adicionais bloqueadas.

  Depois do 101, restam 1 e 3.
- Se durante a execução descobrires que o 101 também permite
  simplificação noutra variante (ex: `Content::Link` que só
  envolve um corpo), **não** arrastar. Abrir DEBT.
- Após o 101, o caminho natural é activar `#set`/`#show` no
  eval: é aí que `Content::Styled` passa a ser construído a
  partir de sintaxe Typst que hoje não tem efeito. Esse é
  trabalho do passo seguinte, não deste.
