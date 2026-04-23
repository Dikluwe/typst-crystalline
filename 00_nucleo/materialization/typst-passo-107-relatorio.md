# Passo 107 — Relatório de encerramento (propagação `sink` + DEBT-49)

**Data**: 2026-04-23
**Precondição**: Passo 106 encerrado; canal Sink → L3 activo.
803 L1 + 178 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0036 (5ª aplicação), ADR-0042 (Sink),
ADR-0043 (canal estendido com hint).
**ADR nova**: não necessária (aplicação de padrões existentes).

---

## Sumário

DEBT-49 **ENCERRADO**. `#set text(font: ...)`, `#set text(lang: ...)`,
`#set par(...)` e outros silêncios passam a emitir warnings
estruturados que chegam ao caller L3 via `into_diagnostics()`.

A 5ª aplicação da ADR-0036 adiciona `sink: &mut TrackedMut<'_, Sink>`
como 10º parâmetro das funções `eval_*`. Gate 107.A.3 disparou:
`&mut Sink` não é obtível de `TrackedMut<Sink>` sem perder tracking
comemo — revista forma para `&mut TrackedMut<'_, Sink>`; `warn_note`
estendido com `hint: &str` (convenção `""` = sem hint).

Zero regressão funcional: **803 L1 + 184 L3 (+6) + 6 ignorados**.
`crystalline-lint .` → zero violations.

---

## 107.A — Inventário

Inventário em
`00_nucleo/diagnosticos/inventario-sink-propagacao-passo-107.md`.

### Sítios migrados

| Ficheiro:linha | Site | Warning emitido |
|----------------|------|-----------------|
| `rules/eval/rules.rs:287` | `if target != "text"` — target desconhecido | "set: target '\<name\>' ainda não suportado" |
| `rules/eval/rules.rs:320` | `_ => { }` — propriedade #set text desconhecida | "text: propriedade '\<key\>' ainda não suportada" |

Ambos emitem via helper central (`unsupported_target_warn` /
`unsupported_property_warn`) — mensagens consistentes para o
utilizador final. Hints referenciam ADR-0040 como catálogo vivo.

### Cadeia transitiva

- **K** (leitoras): **1** (`eval_set_rule`).
- **P** (propagadoras): **23** funções em 7 ficheiros.
- **K+P = 24** — dentro do gate (≤40).
- **D = 4** níveis (`eval` → `eval_markup` → `eval_expr` →
  `eval_set_rule`) — dentro do gate (≤6).

Comparação Passo 98 (4ª aplicação ADR-0036): ~25 LTs, ~4 níveis.
Mesma ordem de grandeza.

### Gate 107.A.3 disparado — decisão revista

Consulta à API comemo confirmou que `TrackedMut<T>` **não** expõe
`&mut T` directo. `Deref/DerefMut` pontam para `T::SurfaceMut<'a>`,
que só expõe os métodos declarados em `#[comemo::track] impl T`. Não
existe `into_inner_mut()` nem equivalente.

**Decisão revista**: propagar `&mut TrackedMut<'_, Sink>` (em vez de
`&mut Sink`). Emissão via `sink.warn_note(span, msg, hint)` —
DerefMut pelo surface tracked.

**Extensão da API tracked**: `Sink::warn_note` aceita agora
`(span, message: &str, hint: &str)` em vez de `(span, message: &str)`.
Convenção `hint == ""` = sem hint (alternativa a `Option<&str>` que
o macro `#[comemo::track]` rejeita por ambiguidade de elisão de
lifetimes).

---

## 107.B — Implementação

### Alterações por ficheiro

**`entities/world_types.rs`** — `warn_note` estendido:

```rust
// Antes (Passo 106)
pub fn warn_note(&mut self, span: Span, message: &str) { ... }

// Depois (Passo 107)
pub fn warn_note(&mut self, span: Span, message: &str, hint: &str) { ... }
```

**`rules/eval/rules.rs`** — helpers + migrações:

```rust
fn unsupported_property_warn(target: &str, field: &str) -> (String, String) {
    (
        format!("{target}: propriedade '{field}' ainda não suportada"),
        format!("ver ADR-0040 para propriedades cobertas por set {target}"),
    )
}

fn unsupported_target_warn(target: &str) -> (String, String) {
    (
        format!("set: target '{target}' ainda não suportado"),
        "targets suportados: heading, page, figure, text".to_string(),
    )
}
```

Sítios migrados (exemplo `eval_set_rule`):

```rust
// Antes
if target != "text" {
    return Ok(Value::None);
}
...
_ => {
    // DEBT: propriedades de #set text não suportadas ...
}

// Depois
if target != "text" {
    let (msg, hint) = unsupported_target_warn(&target);
    sink.warn_note(target_span, &msg, &hint);
    return Ok(Value::None);
}
...
_ => {
    let (msg, hint) = unsupported_property_warn("text", &key);
    sink.warn_note(named.name().to_untyped().span(), &msg, &hint);
}
```

### Assinatura das `eval_*`

Exemplo antes/depois em `eval_set_rule`:

```rust
// Antes (9 params + ctx)
pub(super) fn eval_set_rule<'r>(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
) -> SourceResult<Value>

// Depois (10 params + ctx)
pub(super) fn eval_set_rule<'r>(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,   // ← NOVO (5ª aplicação ADR-0036)
) -> SourceResult<Value>
```

### Assinatura pública de `eval()` — inalterada

`eval()` continua a receber `mut sink: TrackedMut<Sink>` por valor
(API do canal do Passo 106). Dentro, passa `&mut sink` aos internos.

---

## 107.C — Testes

6 testes novos em `03_infra/src/integration_tests.rs`:

1. `debt49_set_text_font_emite_warning` — `#set text(font: "Arial")`
   → 1 warning com mensagem específica e hint ADR-0040.
2. `debt49_set_text_lang_emite_warning` — `#set text(lang: "pt")`
   → warning análogo.
3. `debt49_set_text_multiplas_propriedades_desconhecidas` —
   `font, lang, weight` num único `#set` → 3 warnings distintos.
4. `debt49_set_text_propriedades_suportadas_sem_warnings` — regressão:
   `bold, italic, size` não geram warnings.
5. `debt49_set_target_desconhecido_emite_warning` — `#set par(...)`
   → warning de target.
6. `debt49_dedup_warnings_identicos` — `#set text(font: ...)`
   repetido em duas linhas → 2 warnings (spans diferem — dedup real
   testado em unit tests de `Sink`).

Todos passam.

**Teste removido**: nenhum. Todos os 178 testes L3 anteriores
continuam a passar.

---

## 107.D — Encerramento

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 803 passed; 0 failed; 0 ignored ...  (L1 inalterado)
test result: ok. 184 passed; 0 failed; 6 ignored ...  (L3 +6)

$ crystalline-lint .
✓ No violations found

$ grep -r "DEBT-49" 01_core/src/rules/eval/
rules.rs:31:  /// ... encerra DEBT-49 ...  (comentário histórico)
rules.rs:213: // ... encerra DEBT-49 ...   (comentário histórico)
rules.rs:319: // ... encerra DEBT-49 ...   (comentário histórico)
```

Zero silenciamentos DEBT-49 restantes em `eval/`. Comentários
históricos documentam o encerramento.

### DEBT-49 ENCERRADO

Movido para Secção 2 com "Encerramento (Passo 107)" detalhando
decisões e mudanças. Lacunas residuais (nota sobre
`text.weight` como string/int, silenciamentos fora do âmbito
estrito) registadas como "trabalho futuro, não bloqueiam DEBT-49".

### DEBT sucessor — não aberto

Nenhum silêncio legítimo ficou por migrar dentro do âmbito
estrito do 107. Os silêncios remanescentes (wildcards deliberados,
DEBT-10 em heading, defensivos) são intencionais e não reabrem o
DEBT-49.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 803 | 803 (inalterado) |
| L3 tests | 178 | **184** (+6) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 43 | 43 (inalterado) |
| DEBTs abertos | 13 | **12** (−DEBT-49) |
| Params `eval_*` | 9 + ctx | **10 + ctx** |
| Aplicações ADR-0036 | 4 | **5** |

---

## Lições

1. **Gate 107.A.3 era protecção real**: a hipótese "derivar `&mut Sink`
   de `TrackedMut`" era atractiva mas comemo impede-a por design —
   `TrackedMut` expõe apenas o surface tracked. O gate mudou a decisão
   para `&mut TrackedMut<'_, Sink>` + tracked `warn_note`. Consumir a
   gate em vez de ignorar evitou refactor inviável.

2. **Elisão de lifetimes em `#[comemo::track]`**: adicionar um
   segundo `&str` ao `warn_note` não compilou até resolver
   manualmente a elisão. `Option<&str>` + lifetime explícito também
   falha (o macro não preserva generics de método). Solução mais
   simples: `&str` com convenção `"" = sem hint`. Conhecer esta
   limitação ajuda em futuras tracked methods.

3. **API dupla continua a pagar**: `Sink::warn(diag)` não-tracked e
   `Sink::warn_note(span, msg, hint)` tracked coexistem sem fricção.
   Cada consumidor escolhe. Os helpers da migração DEBT-49 usaram o
   tracked porque os internos só têm `TrackedMut`.

4. **Spans dos args facilitam localização**: emitir warnings com
   `named.name().to_untyped().span()` (nome do argumento) em vez de
   `set.span()` (linha inteira) aponta o utilizador directamente
   para a propriedade problemática. Detalhe de UX que custou pouco
   código.

5. **10 params é o limite visual**: as assinaturas de `eval_*`
   passam agora a 10 + ctx. São lidas com esforço. Próxima
   aplicação de ADR-0036 (se houver) provavelmente deve vir depois
   de materializar `Engine<'a>` que absorve os 10 numa struct. O
   relatório regista esta pressão como evidência empírica acumulada.

6. **DEBT-49 geograficamente concentrado**: apenas 2 sítios num
   único ficheiro. A maior parte do trabalho foi **propagar**
   (mecânico em 7 ficheiros) + **decidir a forma** (gate 107.A.3).
   A migração em si foi de 3 edições. Pattern esperável para
   próximas aplicações da ADR-0036.

---

## Estado pós-Passo 107

### Pipeline warnings end-to-end completo

```
#set text(font: "Arial")
       ↓ parse
Expr::SetRule
       ↓ eval_expr → eval_set_rule
match key { ..., _ => unsupported_property_warn("text", "font") }
       ↓
sink.warn_note(arg_span, "text: propriedade 'font' ...", "ver ADR-0040 ...")
       ↓ TrackedMut<Sink> (tracked mutation)
Sink acumula (dedup por (span, message))
       ↓ retorno de eval()
caller drena: sink.into_diagnostics() → Vec<SourceDiagnostic>
       ↓ L3 formata
eprintln!("warning: {:?} {}", span, message)
       ↓
stderr do utilizador — vê directamente qual propriedade não é suportada
```

### Trabalho futuro identificado

1. **Engine<'a>**: absorver os 10 params + ctx numa struct.
   Depende de `Introspection` (relatório continuidade original).
   Agora com mais pressão empírica de 10 params como motivo.
2. **Formato rico**: Span → linha/coluna via Source; cores ANSI;
   modo JSON/SARIF. Passo dedicado ao formatter.
3. **`text.weight` como string/int**: mapeamento de
   `weight: "bold" | 700` para o modelo actual `bold: bool`. Work
   item separado — hoje emite warning correcto que orienta o
   utilizador.
4. **CLI real em `04_wiring`**: materializar main.rs com argumentos,
   drain_warnings_to_stderr e retornar códigos apropriados.
5. **Integração LSP**: warnings estruturados em JSON para editores.
