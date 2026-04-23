# Passo 107 — Propagar `sink` pelas `eval_*` e migrar silêncios DEBT-49

**Série**: 107 (passo único; 5ª aplicação da ADR-0036).
**Precondição**: Passo 106 encerrado; canal Sink → L3 funcional;
803 L1 + 178 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0036 (atomização — 5ª aplicação),
ADR-0042 (Sink), ADR-0043 (canal de saída).
**ADR nova**: não necessária (aplicação do padrão existente).

---

## Objectivo

Resolver DEBT-49 completamente:

1. Propagar `sink: &mut Sink` pelas funções `eval_*` que
   precisam de emitir warnings, conforme a 5ª aplicação da
   ADR-0036 (dependências na assinatura).
2. Migrar os sítios silenciados marcados com `DEBT-49` ou
   `silenciad` — emitir `sink.warn(SourceDiagnostic::warning(...))`
   em vez de ignorar.
3. Manter o padrão das 4 aplicações anteriores da ADR-0036:
   `route`, `styles`, `show_rules + active_guards`,
   `current_file + figure_numbering`.

**Não faz**:
- Migrar wildcards deliberados (`_ => Ok(Value::None)`) em
  `eval_markup` / `eval_expr`. Esses continuam a ser fronteira
  deliberada.
- Materializar `Engine<'a>` (fica para passo dedicado; consciente
  que este passo leva o total de parâmetros a 10).
- Mudar assinatura pública de `eval()`. `TrackedMut<Sink>`
  permanece; `&mut Sink` é derivado internamente.

---

## Decisões já tomadas

1. **Forma**: `&mut Sink` derivado de `TrackedMut<Sink>` via
   desreferenciação. Dentro de `eval`, obter `&mut Sink` a partir
   do `TrackedMut` uma vez; passar referência às funções internas.
2. **Âmbito estrito**: só migrar sítios explicitamente marcados
   `DEBT-49` ou `silenciad*`. Wildcards deliberados ficam.
3. **API**: `sink.warn(SourceDiagnostic::warning(span, msg).with_hint(...))`
   — API rica não-tracked, conforme API dupla do Passo 106.
4. **DEBT-49 fecha neste passo**. Se ficar algum silêncio legítimo
   não migrável (ex: por acesso a `sink` impossível sem grande
   refactor), abrir DEBT sucessor com escopo específico, em vez
   de deixar DEBT-49 parcial.

---

## Contexto sobre trade-off Engine<'a>

Este passo leva as funções `eval_*` de 9 para 10 parâmetros.
É decisão consciente — a alternativa (materializar `Engine<'a>`
primeiro) foi considerada e adiada. Razões:

- `Engine<'a>` depende de `Introspection` para ter valor completo
  (relatório de continuidade original).
- DEBT-49 está desbloqueado e é pequeno; fechá-lo agora dá valor
  visível imediato (warnings chegam ao utilizador).
- Quando `Engine<'a>` materializar, absorver `sink` (junto com
  os outros 9) é trabalho mecânico.

A pressão de 10 parâmetros fica documentada como evidência
empírica adicional para `Engine<'a>`.

---

## Escopo

**Dentro**:
- `01_core/src/rules/eval/` — todas as funções `eval_*` que
  recebem os 9 parâmetros actuais e que são transitivamente
  chamadas a partir de sítios onde `sink` precisa de chegar.
- `01_core/src/rules/eval/rules.rs` — sítio DEBT-49 em
  `eval_set_rule` (e outros se o inventário encontrar).
- Testes afectados.

**Fora**:
- `eval()` público (só desreferencia `TrackedMut`; assinatura
  pública intacta).
- Wildcards `_ => Ok(Value::None)`.
- Outros silenciamentos sem marca `DEBT-49` (aceitar como
  legítimos).
- Qualquer reestruturação de ficheiros.

---

## Sub-passos

### 107.A — Inventário

**Parte 1 — Sítios a migrar**:

1. Grep por `DEBT-49`, `silenciad`, `// TODO.*warn`, `// warning`
   em `01_core/src/rules/eval/` e `01_core/src/rules/stdlib/`.
2. Para cada sítio, registar:
   - Ficheiro:linha.
   - Contexto: que caso silencia (ex: "propriedade font de
     #set text").
   - A função envolvente.
   - Profundidade até `eval_set_rule` ou `eval()` (quantas funções
     entre o sítio e o sítio onde `sink` já é acessível).

**Parte 2 — Cadeia transitiva**:

Igual ao padrão das 4 aplicações anteriores da ADR-0036:

1. Partindo de cada sítio identificado, subir a cadeia de chamadas
   para identificar **todas** as funções que ganham parâmetro
   novo `sink: &mut Sink`.
2. Medir profundidade máxima e número total de funções tocadas.
3. Registar em
   `00_nucleo/diagnosticos/inventario-sink-propagacao-passo-107.md`:
   ```
   Sítios silenciados (estrito):
     eval_set_rule:NNN — propriedade font
     ...

   Cadeia transitiva:
     Funções leitoras: K (chamam sink.warn directamente)
     Funções propagadoras: P (só passam o parâmetro adiante)
     Total: K+P ganham parâmetro novo
     Profundidade máxima: D níveis
   ```

**Gate**: se K+P > 40, ou profundidade > 6, este passo excede o
padrão das aplicações anteriores (Passo 98 teve ~25 LTs e 4
níveis). Parar e reportar. Alternativa: fazer `Engine<'a>` antes
para evitar o inchaço.

**Parte 3 — Obtenção de `&mut Sink` a partir de `TrackedMut`**:

1. Consultar a API comemo para como derivar `&mut Sink` de
   `TrackedMut<Sink>`. Possibilidades:
   - `TrackedMut<Sink>` deref automático para `&mut Sink`.
   - Método explícito `tracked.into_inner_mut()`.
   - Nenhum dos dois — forçar uso de API tracked (`warn_note`).
2. Se não é possível obter `&mut Sink` sem esvaziar o
   tracking, voltar a 107 com decisão revista.

**Gate**: se `&mut Sink` não é obtível do `TrackedMut` sem perder
o tracking, reverter decisão 1 (API dupla favorece `warn_note`
tracked) e ajustar 107 antes de executar.

### 107.B — Implementação

Ordem obrigatória:

**107.B.1 — Obter `&mut Sink` dentro de `eval()`**:

No corpo de `eval()`, obter `&mut Sink` uma vez:

```rust
// Placeholder — substituir pela API real do comemo apurada em 107.A
let sink_inner: &mut Sink = /* derivar de TrackedMut<Sink> */;
```

Passar `sink_inner` como `&mut Sink` aos call sites internos.

**107.B.2 — Adicionar parâmetro `sink: &mut Sink`**:

Nas funções `eval_*` identificadas (propagadoras + leitoras):

```rust
fn eval_expr(
    ctx: &mut EvalContext,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut Sink,  // NOVO — 5ª aplicação ADR-0036
    expr: &Expr,
) -> SourceResult<Value>
```

Ordem recomendada dos parâmetros: manter consistência com as
aplicações anteriores. `sink` pode entrar antes do argumento
principal (`expr`, `markup`, etc.), a seguir ao último "estado de
eval" (`figure_numbering`).

**107.B.3 — Migrar sítios silenciados**:

Em cada sítio identificado em 107.A, substituir:

```rust
// Antes
_ => {
    // DEBT-49: silenciado.
}

// Depois
unknown @ _ => {
    sink.warn(
        SourceDiagnostic::warning(
            span,
            format!("set: propriedade '{unknown}' ainda não suportada"),
        )
        .with_hint("ver ADR-0040 para propriedades cobertas"),
    );
}
```

A mensagem deve:
- Identificar qual propriedade é.
- Referenciar a ADR que cobre o catálogo actual.
- Não prometer comportamento que o eval não tem.

**107.B.4 — Compilação incremental**:

Depois de cada ficheiro tocado:

- `cargo check -p typst-core` para detectar assinaturas
  incompletas antes de correr testes.
- `cargo test -p typst-core` para garantir nada regride.

### 107.C — Testes

1. **Actualizar testes existentes**: se algum teste chamava
   função `eval_*` directamente (improvável mas possível), adicionar
   `&mut sink` ao call site.

2. **Novos testes de warning real**:
   - Input Typst com `#set text(font: "Arial")` → warning
     específico no Sink.
   - Input com `#set text(lang: "pt")` → warning análogo (se a
     propriedade também era silenciada).
   - Input com múltiplas propriedades não suportadas → N
     warnings (com dedup se span+message coincidirem).

3. **Teste de regressão**: input sem propriedades silenciadas
   continua a não gerar warnings.

4. **Dedup real**: `#set text(font: "X")\n#set text(font: "X")`
   → 1 warning (dedup do Passo 104 funciona agora em caso real).

### 107.D — Encerramento

1. Grep: `DEBT-49` e `silenciad` retornam zero matches em
   `01_core/src/rules/eval/` (ou apenas comentários históricos
   justificados).
2. `cargo test --workspace`: ≥ linha de base + testes novos.
3. `crystalline-lint` zero violations.
4. **DEBT-49 marcado como ENCERRADO (Passo 107)**. Mover para
   Secção 2. Se algum silêncio legítimo ficou por migrar, abrir
   DEBT sucessor com escopo específico (ex: "DEBT-52:
   silenciamentos em stdlib não cobertos pelo 107").
5. Contagem final de parâmetros das funções `eval_*`:
   era 9 + `ctx`, passa a 10 + `ctx`. Registar.
6. Relatório `typst-passo-107-relatorio.md` com:
   - Inventário 107.A (sítios migrados, cadeia, profundidade).
   - Exemplo antes/depois de uma assinatura.
   - Exemplo antes/depois de um sítio silenciado.
   - Mensagens literais escolhidas (audit trail).
   - Estado final DEBT-49.
   - DEBT sucessor aberto se aplicável.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 107.A escrito.
2. `&mut Sink` derivado de `TrackedMut<Sink>` dentro do `eval()`.
3. Funções `eval_*` ganham parâmetro `sink: &mut Sink`.
4. Todos os sítios silenciados identificados em 107.A emitem
   `sink.warn(...)`.
5. Testes end-to-end passam: warnings chegam via o canal do
   Passo 106.
6. `cargo test --workspace` passa.
7. `crystalline-lint` zero violations.
8. DEBT-49 fechado ou DEBT sucessor aberto.
9. Relatório 107.D escrito.

---

## O que pode sair errado

- **`&mut Sink` não obtível de `TrackedMut<Sink>`**. Gate em
  107.A.3. Se falhar, o passo tem de ser redesenhado com
  `warn_note` tracked em vez de `warn` (API dupla favorece
  caminho tracked). Impacto: perdem-se hints; warnings ficam
  minimalistas. Reportar antes de mudar tudo.
- **Cadeia transitiva muito grande**. Se K+P > 40, o passo é
  grande demais para ser feito sem `Engine<'a>`. Gate em
  107.A.2. Reportar e considerar fazer `Engine<'a>` primeiro.
- **Parâmetros excedem 10**. Se este passo leva as `eval_*` a 10
  parâmetros e algum eval subsequente aparecer com 11, a
  assinatura torna-se visualmente insustentável. Aceitar neste
  passo; registar como motivo adicional para `Engine<'a>`.
- **Testes antigos passaram silenciosamente**. Depois de migrar,
  inputs que antes produziam `Ok(Module)` sem warnings produzem
  `Ok(Module)` com warnings no Sink. Se algum teste asserta
  `sink.is_empty()` sobre input que agora dispara warning,
  falha. Detectar em 107.C e actualizar.
- **Mensagens inconsistentes**. Se cada sítio usa template
  diferente para a mensagem de "propriedade não suportada", o
  utilizador final vê mensagens incoerentes. Definir o template
  uma vez (helper em `rules/eval/rules.rs` ou similar) e reutilizar.
- **DEBT-49 deixa silêncios não explícitos**. Se há sítios
  silenciados sem marca (ex: um `match` que devolve `Value::None`
  sem comentário), o âmbito estrito deixa-os por fora. Documentar
  no relatório como limitação; não converter âmbito estrito em
  âmbito amplo no meio do passo.

---

## Notas operacionais

- Este passo é a 5ª aplicação da ADR-0036. Seguir o padrão dos
  Passos 92, 94, 95, 98 na forma de adicionar o parâmetro e
  propagar pela cadeia.
- Não mudar a assinatura pública de `eval()` (`TrackedMut<Sink>`
  permanece — é a API do canal do Passo 106).
- Não adicionar nem remover `#[derive]`s em `Sink` — a `Clone`
  adicionada no 106 é suficiente.
- Se surgir tentação de fazer `Engine<'a>` "já que estou aqui",
  resistir. É passo dedicado. Neste, o objectivo é fechar DEBT-49.
- Helper para template de mensagem:
  ```rust
  fn unsupported_property_warn(span: Span, target: &str, field: &str) -> SourceDiagnostic {
      SourceDiagnostic::warning(
          span,
          format!("{target}: propriedade '{field}' ainda não suportada"),
      ).with_hint(format!("ver ADR-0040 para propriedades cobertas por set {target}"))
  }
  ```
  Vive em `rules/eval/rules.rs` ou local análogo. Reutilizado em
  todos os sítios migrados.
