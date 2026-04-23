# Passo 102 — Activar `#set` no eval

**Série**: 102 (passo único de construção; sub-passos de
inventário, ADR e verificação).
**Precondição**: Passo 101 encerrado; `Content::Strong`/`Emph`
removidos; `Content::Styled` é a forma canónica; 783 L1 + 174 L3
+ 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0033 (paridade funcional), ADR-0036
(atomização), ADR-0037 (coesão por domínio), ADR-0038 (sistema
de estilos em L1), ADR-0039 (forma de estilo no `FrameItem`).
**ADR nova**: ADR-00NN "Activação de `#set` em eval" — `PROPOSTO`
no 102.B, `EM VIGOR` em 102.E.

---

## Objectivo

Implementar a sintaxe `#set` do Typst no `eval_markup`. Depois
deste passo, o seguinte código tem efeito visível na saída:

```typst
#set text(size: 18pt)
Texto em 18pt
```

O passo **não** activa `#show`. O show rule selector está
identificado como dívida latente no relatório do Passo 101
(casamento por `Content::Styled` + `Style::Bold(true)`); a
activação de `#show` expõe essa dívida e fica para passo
separado.

O passo **não** introduz propriedades novas no enum `Style`
além do que já existe. As propriedades cobertas são determinadas
pelo inventário 102.A.

---

## Decisões já tomadas

1. **Âmbito**: só `#set`. `#show` é trabalho do próximo passo.
2. **Propriedades cobertas**: decisão em 102.A com base em:
   - Que propriedades do enum `Style` estão já implementadas
     (`Bold`, `Italic`, `Size`, `Fill`, `HeadingLevel` após
     Passo 100).
   - Quais são expressáveis através de `#set text(...)`,
     `#set heading(...)`, `#set par(...)` do Typst.
   - A intersecção é o catálogo.
3. **Paridade funcional**: para cada `#set text(bold: true); ABC`
   o output tem de ser equivalente a `*ABC*` no estado actual.
   Isto é o teste objectivo de correcção.

---

## Escopo

**Dentro**:
- `01_core/src/rules/eval/markup.rs` (ou onde `eval_markup`
  vive agora) — activar o processamento de `SyntaxKind::SetRule`.
- `01_core/src/rules/eval/rules.rs` — `eval_set_rule` existe
  desde o Passo 98 (lá para escrever `figure_numbering`);
  estender para produzir `Styles` reais.
- `01_core/src/entities/style.rs` — se o inventário mostrar
  que alguma propriedade precisa de conversão a partir de
  `Value`, adicionar helper.
- Testes de integração que cobram `#set ... ; conteúdo`.

**Fora**:
- `#show` (selectores, closures a produzir conteúdo
  transformado).
- Introspection, Engine<'a>, materialização de folhas.
- `#set page(...)` se o passo decidir não cobrir propriedades
  de página (depende de 102.A).
- Colapsar `Content::Heading` em `Content::Styled` (passo
  futuro, requer Introspection).

---

## Sub-passos

### 102.A — Inventário

**Parte 1 — Estado actual de `eval_set_rule`**:

1. Ler `01_core/src/rules/eval/rules.rs` para perceber o que
   `eval_set_rule` faz hoje. O Passo 98 indica que escreve em
   `figure_numbering`; confirmar:
   - Para que `targets` (text, heading, figure, ...) tem
     lógica?
   - Para que propriedades?
   - O que produz (mutação directa de `figure_numbering`?
     `Styles`? `Value::None`?).
2. Ler `01_core/src/rules/eval/markup.rs` para perceber se
   o arm `SyntaxKind::SetRule` já está activo ou se está
   wildcarded em `_ => Ok(Value::None)`.

**Parte 2 — Catálogo de propriedades disponíveis**:

1. Enum `Style` hoje tem (confirmar por grep):
   - `Style::Bold(bool)`
   - `Style::Italic(bool)`
   - `Style::Size(Pt)`
   - `Style::Fill(Color)` (se Color existe em L1; se não,
     este item fica adiado)
   - `Style::HeadingLevel(u8)` — presumivelmente não
     atribuível via `#set` (é estrutural, produzido por
     `= título`, não por directiva).
2. Sintaxe de `#set` do Typst é `#set TARGET(arg: value, ...)`.
   Mapear:
   - `#set text(weight: "bold")` → `Style::Bold(true)` (o vanilla
     aceita também `size`, `fill`, `font`, `lang`, `style`, ...).
   - `#set text(size: 18pt)` → `Style::Size(Pt(18.0))`.
   - `#set text(style: "italic")` → `Style::Italic(true)`.
   - `#set text(fill: color)` → `Style::Fill(color)` se Color
     materializado.
3. Catálogo de pares (TARGET.field → Style::Variant) a suportar:
   - Lista **mínima**: text.weight, text.size, text.style.
   - Lista **estendida**: + text.fill (se Color existe),
     + par.leading (se aplicável).
   - Fica fora: text.font, text.lang (bloqueadas por tipos não
     materializados; abrir DEBT se necessário).

**Parte 3 — AST do SetRule**:

1. Ler `01_core/src/entities/ast/code.rs` (ou equivalente) para
   confirmar a API da variante `SetRule`:
   - Como se obtém o target (função-alvo, ex: `text`)?
   - Como se obtêm os argumentos (pares chave-valor)?
   - O target é um identificador ou um `Expr` que o eval tem
     de resolver?
2. Na maioria das implementações Typst, `SetRule` tem:
   - `.target() -> Expr` (o identificador da função)
   - `.args() -> Args` (os argumentos como pares)
3. Registar a API exacta antes de escrever código.

**Escrever em** `00_nucleo/diagnosticos/inventario-set-rule-passo-102.md`:

```
eval_set_rule estado actual:
  targets suportados: [ ... ]
  propriedades escritas: [ ... ]
  como produz resultado: ...

Enum Style (propriedades disponíveis):
  Style::Bold, Style::Italic, Style::Size, Style::Fill?, Style::HeadingLevel

Catálogo a suportar neste passo:
  text.weight → Style::Bold
  text.size   → Style::Size
  text.style  → Style::Italic
  text.fill   → Style::Fill (se Color existe)
  (adiadas: text.font, text.lang, ...)

AST SetRule:
  target: ...
  args: ...
```

**Critério de saída 102.A**: ficheiro escrito; catálogo
confirmado; API do AST registada.

### 102.B — ADR nova: activação de `#set`

1. Criar `00_nucleo/adr/typst-adr-00NN-activacao-set-rule.md`
   com `PROPOSTO`.
2. Conteúdo:
   - Contexto: Passo 100 (`Content::Styled` no Layouter), Passo
     101 (consolidação). Agora `#set` pode produzir `Styled`
     sem problemas estruturais.
   - Catálogo de propriedades suportadas (de 102.A).
   - Propriedades adiadas com razão (bloqueadas por Color,
     Font, Lang, etc.).
   - Semântica de escopo: `#set` em markup afecta o conteúdo
     **seguinte no mesmo bloco** até ao fim do bloco (ou até a
     um `#set` posterior que sobreponha a mesma propriedade).
     Confirmar esta semântica lendo o vanilla em 102.A.
   - Integração com `StyleChain`: o `#set` produz `Styles`
     que são empilhados na cadeia via `push_styles`. O
     mecanismo já existe desde o Passo 99/100.
   - Relação com `#show`: este passo **não** activa `#show`.
     Dívida do show selector (relatório 101) fica registada
     aqui como "abordada no passo futuro".
3. `EM VIGOR` só em 102.E.

### 102.C — Implementação

Ordem obrigatória. Cada etapa compila e passa testes antes da
próxima.

**102.C.1 — Helper de conversão de argumentos**:

1. Criar `fn args_to_styles(target: &str, args: &Args) -> Styles`
   (ou equivalente; nome a escolher conforme coesão) em
   `eval/rules.rs` ou módulo dedicado.
2. A função recebe o target (ex: `"text"`) e os argumentos;
   percorre o catálogo 102.A e produz `Styles` com os
   `Style::Variant` correspondentes.
3. Argumentos desconhecidos (fora do catálogo):
   - **Opção ignorar**: devolver `None` e logar warning se
     `Sink` estiver acessível (não estará — Sink é stub).
   - **Opção erro**: devolver `SourceDiagnostic::error` com
     "unsupported set property".
   - **Opção silenciar**: devolver `Styles` vazio.
   - Recomendação: **opção silenciar** com comentário explícito
     `// TODO: DEBT — propriedades de #set não suportadas são
     silenciosamente ignoradas neste passo`. Abrir DEBT-XX
     para isto ser substituído por warning real quando `Sink`
     materializar.
4. Testes unitários: `args_to_styles("text", {weight: "bold"})`
   → `Styles` com `Style::Bold(true)`. Mínimo 4 testes.

**102.C.2 — `eval_set_rule` estendido**:

1. Modificar `eval_set_rule` para:
   - Resolver o target (nome da função: `text`, `heading`,
     `par`, `figure`).
   - Se o target é um dos suportados (catálogo 102.A), chamar
     `args_to_styles`.
   - Produzir `Styles` resultante.
2. Retorno: `eval_set_rule` hoje pode retornar `Value::None`;
   passa a retornar **algo** que o chamador (`eval_markup`)
   possa usar para empilhar na cadeia. Duas opções:
   - Retornar `Value::Styles(Styles)` — requer nova variante
     de `Value`. Invasivo.
   - Modificar `eval_markup` para detectar o arm
     `SyntaxKind::SetRule` e chamar uma função
     `eval_set_rule_to_styles` que retorna `Styles`
     directamente. Menos invasivo.
   - Recomendação: **segunda opção**. Mantém `Value` estável.
3. Preservar a lógica existente de `figure_numbering` do Passo
   98 (se `target == "figure"` e arg `numbering` está presente,
   continuar a escrever em `figure_numbering`). Não regredir.

**102.C.3 — `eval_markup` processa `SetRule`**:

1. No sítio onde `eval_markup` constrói a sequência de `Content`
   a partir de filhos do `Markup`, processar `SyntaxKind::SetRule`:
   ```rust
   SyntaxKind::SetRule => {
       if let Some(set) = ast::SetRule::from_untyped(child) {
           let styles = eval_set_rule_to_styles(ctx, ..., set)?;
           // Envolver os filhos seguintes num Content::Styled
           // ou acumular styles para o resto do bloco
       }
   }
   ```
2. **Decisão de semântica — âmbito do `#set`**:
   - O vanilla: `#set` afecta tudo o que vem a seguir até ao
     fim do bloco. Se existirem múltiplos `#set`, o último por
     propriedade ganha.
   - Implementação: acumular `Styles` ao longo da iteração
     pelos filhos; quando um filho não-SetRule é encontrado,
     envolvê-lo num `Content::Styled(child_content, styles_acumulados)`.
   - Alternativa: no fim da iteração, envolver toda a sequência.
     Menos preciso (afecta coisas anteriores ao `#set`, o que é
     bug).
   - A alternativa correcta é a primeira. Implementar com
     cuidado para garantir que `#set` a meio do bloco não afecta
     o conteúdo anterior.
3. Testes unitários mínimos no eval:
   - `#set text(size: 18pt)\ntext` → `Content::Styled` com
     `Size(18pt)` envolvendo o texto.
   - `#set text(bold: true); before #set text(italic: true); after` →
     "before" só com bold; "after" com bold + italic.

**102.C.4 — Remover wildcard de `SetRule` se existia**:

Se `eval_markup` tinha `_ => Ok(Value::None)` a cobrir
`SyntaxKind::SetRule` (silenciosamente), o arm explícito
substitui isso. Confirmar que nenhum outro `SyntaxKind` fica
acidentalmente sem tratamento.

### 102.D — Testes de integração

1. Testes end-to-end (parse → eval → layout → export):
   - `"#set text(size: 18pt)\nHello"` → FrameItem com size=18pt.
   - `"#set text(weight: \"bold\"); *italic*"` → FrameItem
     com bold=true **e** italic=true (bold do set, italic do
     `*...*`).
   - `"#set text(weight: \"bold\")\nbold text\n#set text(weight:
     \"regular\")\nregular text"` → primeiro texto bold,
     segundo regular.
2. Teste de paridade visual (se corpus existe): adicionar
   ficheiro `set-simple.typ` com `#set text(size: 14pt)` e
   confirmar que o PDF resultante tem o size correcto.
3. Teste de regressão: `*bold* and _italic_` (sem `#set`)
   continua a produzir a saída do Passo 101. Não deve regredir.

### 102.E — Encerramento

1. Grep: `SyntaxKind::SetRule` em `eval_markup` tem arm
   explícito, não cai em wildcard.
2. `cargo test --workspace`: contagem ≥ linha de base + novos
   testes (esperado: 783 + ~8 = ~791 L1).
3. `crystalline-lint` zero violations.
4. ADR nova promovida a `EM VIGOR`.
5. DEBT-1: actualizar. Das 2 pendências restantes (relatório
   Passo 101):
   - "Activar `#set`/`#show`" → agora parcialmente resolvida
     (`#set` activo; `#show` pendente).
   - "Propriedades adicionais bloqueadas por tipos não
     materializados" → registar as propriedades **tentadas**
     em 102.A mas adiadas (ex: text.font requer Font real).
6. DEBT novo (se aplicável): "Propriedades de `#set` não
   suportadas são silenciosamente ignoradas; substituir por
   warning quando `Sink` materializar".
7. Dívida latente do show selector (Passo 101) — **não
   tocada neste passo**; documentada como próximo trabalho
   relacionado.
8. Relatório `typst-passo-102-relatorio.md`:
   - Catálogo de propriedades suportadas.
   - Propriedades adiadas e razões.
   - Exemplo antes/depois de assinatura de `eval_set_rule`.
   - Teste de paridade visual (se aplicável).
   - DEBTs novos.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 102.A escrito.
2. ADR-00NN criada e promovida a `EM VIGOR`.
3. `eval_set_rule` produz `Styles` para propriedades do
   catálogo.
4. `eval_markup` processa `SyntaxKind::SetRule` envolvendo
   conteúdo seguinte em `Content::Styled`.
5. Testes de integração passam:
   - `#set text(size: ...)` afecta size.
   - `#set` combinado com `*bold*` acumula.
   - `#set` posterior sobrepõe anterior para a mesma
     propriedade.
6. Regressão zero: `*bold*` sem `#set` continua igual.
7. `cargo test --workspace` passa.
8. `crystalline-lint` zero violations.
9. DEBT-1 actualizado; DEBT novo (se aplicável).
10. Relatório 102.E escrito.

---

## O que pode sair errado

- **Semântica do âmbito errada**. Se `#set` a meio do bloco
  afecta conteúdo anterior (bug de implementação), o output
  diverge do vanilla. Teste específico em 102.D apanha isto.
- **`figure_numbering` regride**. O Passo 98 introduziu
  escrita em `figure_numbering` via `eval_set_rule`. Se a
  reestruturação do 102.C.2 quebrar esse caminho, os testes
  de figure numbering falham. Ler o estado actual antes de
  modificar; preservar os caminhos existentes.
- **Propriedades não suportadas silenciadas**. Utilizadores
  esperam feedback quando escrevem `#set text(font: "Arial")`
  e nada acontece. Silenciar é decisão pragmática (sink é
  stub), mas tem custo de UX. DEBT fica registado.
- **Catálogo explode em 102.A**. Se a tentação for "suportar
  tudo que o vanilla aceita", o passo torna-se enorme. Manter
  o catálogo **pequeno** — só as propriedades que mapeiam
  directamente em `Style::Variant` existentes. Propriedades
  que precisariam de estender o enum `Style` são passo
  separado.
- **Dívida do show selector é activada prematuramente**.
  Este passo não mexe em `#show`. Mas se alguém decidir "já
  que estou aqui" e tocar no show selector para experimentar,
  arrasta o passo. Resistir. O show selector é trabalho
  separado; a razão para não fazer agora é precisamente
  validar primeiro se a dívida existe realmente.
- **Value::Styles?** Se o executante optar pela primeira opção
  em 102.C.2 (adicionar `Value::Styles` ao enum `Value`),
  abre-se uma porta para `Styles` serem primeira classe no
  eval. Isto pode ser desejável no futuro mas é decisão
  estrutural que merece ADR própria. A recomendação é **não**
  abrir essa porta neste passo.
- **Duplicação entre `#set text(weight: "bold")` e `*bold*`**.
  Depois deste passo, há duas maneiras de produzir
  `Style::Bold(true)`: directiva e sintaxe. Ambas produzem
  `Content::Styled`. Isto é correcto — é como o vanilla
  funciona. A consolidação (Passo 101) preparou o terreno.

---

## Notas operacionais

- `eval_set_rule` foi modificado no Passo 98. Ler esse código
  antes; preservar o que lá está para `figure_numbering`.
- Este passo não toca visibilidade, não reestrutura ficheiros,
  não mexe em `Style`/`Styles`/`StyleChain` internos.
- Se durante execução surgir tentação de estender o enum
  `Style` (adicionar `Style::Weight`, `Style::FontFamily`,
  etc.), **parar e abrir DEBT**. Esse trabalho é dependente da
  materialização de tipos que hoje são stubs (Font, Lang).
- Testes novos contam para o total; se 102.D tem 4 testes,
  esperar 783 + 4 L1 no fim.
