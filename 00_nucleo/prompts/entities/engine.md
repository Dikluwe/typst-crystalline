# Prompt L0 — Engine<'a>
Hash do Código: 33010675

## Módulo
`01_core/src/entities/engine.rs`

## Propósito

`Engine<'a>` é o **agregador de estado de eval em L1**. Consolida
os N parâmetros propagados individualmente pelas funções `eval_*`
após as 5 aplicações da ADR-0036 (Passos 92, 94, 95, 98, 107).

Materializado no Passo 109 conforme ADR-0044 (inversão controlada
da ADR-0036: agregar depois de extrair).

## Contrato

### Struct `Engine<'a>`

```rust
pub struct Engine<'a> {
    // Handles externos (capacidades injetadas como trait objects)
    pub world: &'a dyn World,
    pub font_metrics: &'a dyn FontMetrics,

    // Fluxo de eval (ADR-0036)
    pub route: Tracked<'a, Route<'a>>,
    pub styles: &'a mut StyleChain,
    pub show_rules: &'a mut Arc<[ShowRule]>,
    pub active_guards: &'a mut Vec<RuleId>,
    pub current_file: FileId,

    // Efeitos laterais (ADR-0042, ADR-0043)
    pub sink: &'a mut TrackedMut<'a, Sink>,

    // Stubs futuros documentam divergência face ao vanilla:
    // pub introspector: Introspector,    // Passo dedicado
    // pub routines: &'a Routines,         // Passo dedicado
    // pub traced: Tracked<'a, Traced>,    // Passo dedicado
}
```

### Lifetime

Paramétrica num único `'a`. Campos tracked (`route`, `sink`)
conservam tracking individual. `Engine<'a>` em si **não** é
`#[comemo::track]`.

### Campos públicos

Campos `pub` em vez de getters — L1 não tem razão para encapsular
um agregador transparente.

## Ordem dos campos

Coesa por domínio (ADR-0037):

1. **Handles externos**: `world`, `font_metrics`. Ambos são
   capacidades injetadas como trait objects — L1 declara a interface,
   L3/L4 fornece a implementação concreta.
2. **Fluxo de eval**: `route`, `styles`, `show_rules`,
   `active_guards`, `current_file`.
3. **Efeitos laterais**: `sink`.

Diferente da ordem do `typst-library::engine::Engine` vanilla.
Prioridade: ADR-0037 sobre paridade literal de ordem. Nomes
batem com vanilla (paridade nominal).

## Construção

Sítios de construção em L1: `eval()` público em
`engine/eval/mod.rs`, além de reconstruções locais em scope changes
(`Expr::CodeBlock`, `Expr::ContentBlock`,
`eval_strong`/`eval_emph`/`eval_heading`, `apply_closure`,
`eval_module_include`, etc.).

Padrão: reborrow individual de cada campo do outer engine +
`TrackedMut::reborrow_mut(&mut *engine.sink)` para encurtar lifetime.
O campo `font_metrics` é propagado do outer engine para o local
(`font_metrics: engine.font_metrics`); em pontos raiz onde não há
outer engine (harness de teste, eval inicial antes de contexto),
usa-se `&FixedMetrics` como fallback heurístico.

## Evolução

Campos novos entram com ADR própria quando o subsistema for
materializado:

- `introspector` — quando Introspection materializar (Passo 110+;
  ver 108-relatorio).
- `routines` — quando Routines materializar.
- `traced` — quando Traced materializar com uso real.

Nome dos campos **deve** bater com o vanilla.
