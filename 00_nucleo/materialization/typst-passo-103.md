# Passo 103 — Activar `#show` no eval (heading + strong + emph)

**Série**: 103 (passo único de construção; sub-passos de
inventário, ADR e verificação).
**Precondição**: Passo 102 encerrado; `#set text(...)` funcional
(incluindo `fill`); 790 L1 + 174 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0026 (divergência de Content), ADR-0033
(paridade funcional), ADR-0036 (atomização), ADR-0037 (coesão),
ADR-0040 (activação de `#set`, coexistência bake-in + wrapping).
**ADR nova**: ADR-00NN "Activação de `#show` — heading, strong,
emph" — `PROPOSTO` em 103.B, `EM VIGOR` em 103.E.

---

## Objectivo

Implementar a sintaxe `#show SELECTOR: it => BODY` do Typst no
`eval_markup`, para três selectores:

- `#show heading: it => ...`
- `#show strong: it => ...`
- `#show emph: it => ...`

Depois deste passo:

```typst
#show heading: it => [*Título:* #it.body]
= Introdução
```

produz "**Título:** *Introdução*" (ou equivalente conforme a
closure).

---

## Decisões já tomadas

1. **Âmbito**: heading + strong + emph. Expõe a dívida latente
   do show selector (relatório Passo 101) deliberadamente.
2. **Tratamento da dívida strong/emph**: aceitar semântica
   diferente do vanilla e registar DEBT. **Não** adicionar
   flag de origem ao `Content::Styled` neste passo.
3. **Selectores fora do âmbito**:
   - `#show "literal": ...` (texto literal).
   - `#show HEADING.where(level: N): ...` (selector com filtro).
   - `#show: rest => ...` (tudo).
   - `#show RAW: ...`, `#show LIST: ...`, etc.
4. **Semântica de escopo**: consistente com `#set` (Passo 102).
   `#show` afecta o conteúdo seguinte no mesmo bloco até ser
   sobreposto por outro `#show` para o mesmo selector.

---

## Semântica da dívida strong/emph aceite

A razão para a aceitar como dívida em vez de resolver agora:

**Problema**: depois do Passo 101, `Content::Styled(body, [Bold(true)])`
é produzido tanto por `*bold*` (sintaxe Strong) como eventualmente por
`#set text(bold: true)` (directiva `#set`). O show selector actual
(`is_bold_styled`) não distingue as duas origens.

**Consequência em `#show`**:
```typst
#show strong: it => [FOI STRONG: #it.body]
#set text(bold: true)
texto
```
No vanilla: "texto" em bold, **sem** transformação (porque `#set` não
cria um elemento Strong).
No cristalino neste passo: "FOI STRONG: texto" (porque o selector
apanha qualquer `Styled` com `Bold(true)`).

**Porque aceitamos**: a resolução correcta requer rastrear a origem
de cada `Content::Styled`. Há duas abordagens possíveis no futuro:

- **Flag de origem no enum `Style`**: `Style::Bold { value: bool, from_strong: bool }`
  — intrusivo, adiciona ruído a toda a API.
- **Marcador separado no `Content::Styled`**: `Content::Styled(body, styles, origin)`
  onde `origin: Option<ElementKind>` — requer mudança no enum.

Ambas são mudanças estruturais que merecem decisão dedicada.
Neste passo, a dívida é **exposta** (o bug fica reproduzível) e
**documentada** (DEBT-XX registado). A paridade com vanilla é
conscientemente quebrada neste caso específico.

---

## Escopo

**Dentro**:
- `01_core/src/rules/eval/rules.rs` — `eval_show_rule`, registo
  e aplicação de show rules.
- `01_core/src/rules/eval/markup.rs` — processar `SyntaxKind::ShowRule`.
- `01_core/src/rules/eval/*` — quando um elemento target é produzido,
  verificar se há show rule aplicável e executar.
- Testes.

**Fora**:
- Introspection, Engine<'a>, materialização de stubs.
- Qualquer mudança a `Style`, `Styles`, `StyleChain`, `Content`.
- Activação de selectores além de heading/strong/emph.
- Resolução da dívida de origem (só expor + documentar).

---

## Sub-passos

### 103.A — Inventário

**Parte 1 — Estado actual de `eval_show_rule`**:

1. Grep por `eval_show_rule` e `show_rules` em `01_core/src/`.
2. Registar:
   - `eval_show_rule` existe? Se sim, o que faz hoje?
   - `show_rules` é campo/parâmetro nalguma estrutura? (Passo 95
     extraiu `show_rules: &mut Arc<[ShowRule]>` das funções eval_*).
   - `ShowRule` — qual a forma actual do tipo? Selector + closure?
3. Grep por `is_match` em `rules.rs` — esta é a lógica do Passo 101
   que casa `Content::Styled` com `NodeKind::Strong`. Documentar.

**Parte 2 — AST do ShowRule**:

1. Ler `01_core/src/entities/ast/code.rs` (ou equivalente) para
   a variante `ShowRule`:
   - `.selector() -> Option<Expr>` (ou nome equivalente).
   - `.transform() -> Expr` (a closure ou a expressão RHS).
2. Registar a API exacta.

**Parte 3 — Ponto de aplicação**:

1. Identificar onde no eval os elementos alvo (`Content::Heading`,
   `Content::Styled` com Bold/Italic) são produzidos.
2. Em cada um desses pontos, a aplicação de `#show` tem de verificar
   se há rule aplicável no `*show_rules` activo e executar.
3. Documentar no inventário: N pontos de produção → M pontos onde
   verificar aplicação.

**Parte 4 — Closure mechanics**:

1. `#show SELECTOR: it => BODY` cria uma closure com parâmetro `it`
   que recebe o `Content` original.
2. O mecanismo de closures existe desde o Passo 16
   (`apply_closure_func`). Confirmar:
   - Aceita `Value::Content` como argumento? (verificar variantes
     de `Value`).
   - O retorno é `Value::Content` que depois substitui o original?
3. Se `Value::Content` não existe, há bloqueador — adicionar ao
   enum `Value` é mudança estrutural fora do escopo; abrir DEBT
   e reduzir o âmbito do passo.

**Critério de saída 103.A**: inventário escrito em
`00_nucleo/diagnosticos/inventario-show-rule-passo-103.md` com:
- Estado actual (o que existe, o que falta).
- API da AST.
- Pontos de aplicação identificados.
- Confirmação que `Value::Content` suporta o fluxo closure.
- Lista de obstáculos encontrados (se existirem).

**Gate**: se o inventário revelar que `Value::Content` não aceita
o fluxo (ex: a conversão `Content <-> Value` não existe), parar e
reportar. Pode ser que o passo tenha de ser dividido em "adicionar
suporte a Content em closures" + "activar #show".

### 103.B — ADR nova

1. Criar `00_nucleo/adr/typst-adr-00NN-activacao-show-rule.md`
   com `PROPOSTO`.
2. Conteúdo:
   - Contexto: Passo 101 consolidou Strong/Emph em Styled; Passo
     102 activou `#set`. `#show` é o próximo mecanismo.
   - Âmbito: heading, strong, emph. Selectores não cobertos
     listados explicitamente.
   - Semântica de escopo (consistente com `#set`).
   - **Dívida aceite**: semântica de `#show strong` e `#show emph`
     diverge do vanilla quando há `#set text(bold/italic: true)`.
     Registar em DEBT-XX.
   - Mecanismo de aplicação: ao produzir `Content::Heading`,
     `Content::Styled` com Bold(true) ou Italic(true), verificar
     `show_rules` activas e aplicar a primeira que casa.
   - Relação com `show_rules: &mut Arc<[ShowRule]>` (Passo 95): o
     parâmetro já existe nas funções eval_*; este passo activa-o.
3. `EM VIGOR` em 103.E.

### 103.C — Implementação

Ordem obrigatória.

**103.C.1 — Tipo `ShowRule` (se não existe completo)**:

1. Confirmar que `ShowRule` tem forma utilizável. Expectável:
   ```rust
   pub struct ShowRule {
       pub selector: ShowSelector,  // heading | strong | emph
       pub transform: Value,        // deve ser Value::Func
   }
   pub enum ShowSelector {
       Heading,
       Strong,
       Emph,
   }
   ```
2. Se a forma actual for diferente, adaptar com mínimo impacto.
   Não renomear nem reestruturar sem razão empírica.

**103.C.2 — `eval_show_rule` produz `ShowRule`**:

1. No eval, quando `SyntaxKind::ShowRule` aparece:
   - Avaliar o selector (identificador `heading` / `strong` / `emph`
     é reconhecido como um selector, não uma função).
   - Avaliar a expressão de transformação (normalmente uma closure)
     como `Value::Func`.
   - Construir `ShowRule { selector, transform }`.
   - Empilhar em `show_rules` (que é `Arc<[ShowRule]>` — usar
     `Arc::make_mut` ou reconstruir com `.push`).
2. Retornar `Value::None` (semelhante a `#set`).
3. Propriedades não suportadas do selector (ex: `heading.where(level: 1)`)
   são silenciosamente ignoradas com DEBT (análogo ao 102).

**103.C.3 — Aplicação em pontos de produção**:

1. Definir função auxiliar:
   ```rust
   fn apply_show_rules(
       content: Content,
       show_rules: &Arc<[ShowRule]>,
       ctx: &mut EvalContext,
       world: Tracked<dyn TrackedWorld>,
       /* outros parâmetros que closure precisa */,
   ) -> SourceResult<Content>
   ```
2. A função percorre `show_rules` do fim para o início (último
   `#show` ganha), testa se alguma rule casa o `content`:
   - `ShowSelector::Heading` casa `Content::Heading { .. }`.
   - `ShowSelector::Strong` casa `Content::Styled(_, styles)`
     com `Style::Bold(true)` em `styles`.
   - `ShowSelector::Emph` casa `Content::Styled(_, styles)`
     com `Style::Italic(true)` em `styles`.
3. Se casar:
   - Converter `content` em `Value::Content(content)`.
   - Chamar `apply_closure_func(transform, Args::positional([content_value]), ...)`.
   - Converter resultado de volta em `Content`.
4. Se retorno não é `Value::Content`, emitir erro
   (`SourceDiagnostic::error`, "show rule must produce content").
5. Se nenhuma rule casar, devolver o `content` original.

**103.C.4 — Integração no eval**:

Pontos a instrumentar (vindos de 103.A):

1. Onde `Content::Heading { .. }` é construído (arm
   `SyntaxKind::Heading` em `eval_markup`): envolver com
   `apply_show_rules`.
2. Onde `Content::Styled(body, [Bold(true)])` é construído
   (o construtor `Content::strong(...)` após redefinição no Passo
   101): envolver com `apply_show_rules`.
3. Idem para `Content::emph(...)`.

**Nota sobre `Content::strong`**: como o construtor é usado em
múltiplos sítios (incluindo `stdlib::native_strong`), adicionar
`apply_show_rules` dentro do construtor é atraente mas não viável
— o construtor é uma função pura sem acesso a `show_rules`. A
aplicação tem de ser no **call site** onde o contexto eval
existe.

**103.C.5 — Caso especial: closure produz outro elemento target**:

Se `#show strong: it => heading(1, it.body)`, o retorno é um
`Content::Heading`. Isto pode disparar `#show heading` se também
estiver activo. Decisão:

- **Sem recursão** (mais simples): aplicar show rule uma vez; o
  retorno não é re-processado. Divergência do vanilla mas aceitável
  para este passo.
- **Com recursão limitada**: aplicar até N iterações ou até ponto
  fixo. Mais fiel ao vanilla mas mais complexo.

Recomendação: **sem recursão** neste passo, com DEBT se necessário.
O teste 103.D verifica se casos reais esbarram nesta limitação.

### 103.D — Testes

Testes de integração:

1. `#show heading: it => [Prefixo: #it.body]\n= Intro` → saída
   contém "Prefixo:" e "Intro".
2. `#show strong: it => [<<#it.body>>]\n*alvo*` → saída contém
   "<<alvo>>" (não "alvo" em bold).
3. `#show emph: it => [{#it.body}]\n_tipo_` → saída contém
   "{tipo}".
4. Regressão: sem `#show`, `*bold*` continua bold; `= heading`
   continua heading.
5. **Teste da dívida exposta**: 
   ```typst
   #show strong: it => [HIT]
   #set text(bold: true)
   texto
   ```
   Documentar o que acontece (esperado: "HIT" aparece — bug
   aceite). Adicionar um teste que **assert o comportamento
   actual** (não o correcto) com comentário explícito:
   ```rust
   #[test]
   fn dvida_show_strong_apanha_set_text_bold() {
       // Este teste documenta a dívida do show selector.
       // Vanilla: "texto" em bold, sem transformação.
       // Cristalino: transforma porque #set text(bold: true) produz
       // Content::Styled com Style::Bold(true), e o selector strong
       // não distingue origem.
       // DEBT-XX: resolver por flag de origem ou marcador.
       let output = layout_string("#show strong: it => [HIT]\n#set text(bold: true)\ntexto");
       assert!(output.contains("HIT"), "documenta dvida actual");
   }
   ```
6. Teste do caso 103.C.5: `#show strong: it => heading(1, it.body)`
   produz Content::Heading — verificar se `#show heading` também
   activo não é aplicado (dado que decidimos "sem recursão").

### 103.E — Encerramento

1. Grep: `SyntaxKind::ShowRule` tratado em `eval_markup`, não em
   wildcard.
2. `cargo test --workspace`: ≥ linha de base + testes novos
   (790 + ~7 = ~797).
3. `crystalline-lint` zero violations.
4. ADR nova → `EM VIGOR`.
5. DEBT-1 actualizado: `#show` parcialmente activo (só 3
   selectores); restantes (literal, where, catch-all, raw, list)
   pendentes.
6. DEBT novo: "Show selector para strong/emph não distingue
   origem — apanha `#set text(bold/italic: true)` como se fosse
   `*...*` ou `_..._`. Resolver por flag de origem ou marcador
   dedicado quando mecanismo for reavaliado."
7. DEBT novo (se aplicável): "Show rules não recursivas —
   retorno que produz elemento target não dispara outra rule.
   Rever se casos reais forem encontrados."
8. Relatório `typst-passo-103-relatorio.md`:
   - Inventário resumido (estado antes, estado depois).
   - Catálogo de selectores activos.
   - Dívida exposta — teste que documenta + DEBT registado.
   - Exemplo antes/depois de fluxo de `#show`.
   - Limitação de não-recursividade (se testada).

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 103.A escrito.
2. ADR-00NN criada e promovida.
3. `#show heading`, `#show strong`, `#show emph` funcionam e
   transformam o conteúdo respectivo.
4. Teste de regressão: sintaxe sem `#show` não mudou.
5. Teste da dívida: documenta comportamento actual (não
   paridade).
6. `cargo test --workspace` passa.
7. `crystalline-lint` zero violations.
8. DEBTs novos registados (selector origin, recursividade se
   aplicável).
9. DEBT-1 actualizado.
10. Relatório 103.E escrito.

---

## O que pode sair errado

- **`Value::Content` não aceita fluxo closure**. Inventário
  103.A tem gate específico para isto. Se falhar, passo é
  dividido.
- **`Arc::make_mut` para `show_rules`**. Se `ShowRule` não é
  `Clone`, não é usável neste padrão. Alternativa: reconstruir
  `Arc<[ShowRule]>` com extensão. Medir custo.
- **Closure falha em tempo de execução**. Se o corpo da closure
  referencia variáveis não capturadas ou chama funções não
  existentes, o eval produz erro. O teste tem de propagar o
  erro (não fazer unwrap).
- **Aplicação dupla (bug potencial)**. Se `apply_show_rules` for
  chamado tanto no construtor como no call site, o conteúdo é
  transformado duas vezes. Verificar com teste.
- **`show_rules` não propagado correctamente**. O Passo 95
  extraiu `show_rules` como `&mut Arc<[ShowRule]>` nas funções
  eval_*. Confirmar que em cada ponto de aplicação (103.C.4) o
  `show_rules` está visível e propagado.
- **Divergência visível**. A dívida aceite vai aparecer em
  qualquer teste de paridade que use `#show strong` + `#set text(bold)`.
  Se existe corpus de paridade (Passo 9), filtrar para não
  incluir este caso — ou marcar como `#[ignore]` com comentário.

---

## Notas operacionais

- Este passo não toca visibilidade, não reestrutura ficheiros.
- Não adicionar variantes ao enum `Style` ou `Content`.
- Se durante execução surgir vontade de "resolver a dívida
  enquanto estamos aqui", **resistir**. A dívida é exposta
  deliberadamente.
- Se 103.A revelar que activar só `#show heading` é
  significativamente mais simples que activar os três, considerar
  faseamento: 103 faz só heading (sem expor dívida); passo
  seguinte faz strong/emph (expõe dívida). Reportar antes de
  avançar.
