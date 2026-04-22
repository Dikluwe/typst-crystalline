# Diagnóstico: padrão de uso de `#[comemo::track]` no vanilla

**Tipo vanilla**: N/A (meta-análise do uso de atributo)
**Localização vanilla**: `lab/typst-original/crates/typst-library/`
**Data do diagnóstico**: 2026-04-22
**Contexto**: Passo 86, Tarefa B (preparação para ADR futuro sobre
relação cristalino ↔ `comemo`)

**Natureza**: registo factual do estado do vanilla na data acima.
Decisões arquitecturais derivadas deste diagnóstico ficam em
ADR/passo separados. Este ficheiro não contém decisões nem
recomendações.

---

## 1. Inventário de `#[track]` no vanilla

Procura: `grep -rn "#\[comemo::track\]\|#\[track\]" lab/typst-original/ --include="*.rs"`.

Resultado: **8 ocorrências**, todas com a forma `#[comemo::track]`
(nenhuma usa o alias `#[track]` curto no vanilla). Todas vivem no
crate `typst-library`.

| Ficheiro | Linha | Contexto |
|----------|-------|----------|
| `src/engine.rs` | 131 | `impl Traced` |
| `src/engine.rs` | 202 | `impl Sink` |
| `src/engine.rs` | 389 | `impl<'a> Route<'a>` |
| `src/foundations/context.rs` | 35 | `impl<'a> Context<'a>` |
| `src/introspection/introspector.rs` | 28 | `trait Introspector` |
| `src/introspection/locator.rs` | 208 | `impl<'a> Locator<'a>` |
| `src/lib.rs` | 59 | `trait World` |
| `src/model/link.rs` | 678 | `impl<'a> LateLinkResolver<'a>` |

Distribuição por directório:
- `engine.rs` — 3.
- `foundations/` — 1.
- `introspection/` — 2.
- `lib.rs` (raiz do crate) — 1.
- `model/` — 1.

---

## 2. Classificação por categoria

| Cat. | Descrição | Contagem | Exemplos |
|------|-----------|----------|----------|
| 1 | `impl StructName` (métodos inerentes) | **6** | `Traced`, `Sink`, `Route<'a>`, `Context<'a>`, `Locator<'a>`, `LateLinkResolver<'a>` |
| 2 | `impl TraitName for StructName` | **0** | — |
| 3 | `trait TraitName` (definição) | **2** | `World`, `Introspector` |

Exemplo de categoria 1 (`impl Traced` em `engine.rs:131`):
`#[comemo::track] impl Traced { pub fn get(&self, id: FileId) -> Option<Span> { ... } }`.

Exemplo de categoria 3 (trait definitions):
- `#[comemo::track] pub trait World: Send + Sync { ... 7 métodos ... }`
  em `lib.rs:59`.
- `#[comemo::track] pub trait Introspector: Send + Sync { ... 13 métodos ... }`
  em `introspection/introspector.rs:28`.

Categoria 2 (`#[track]` em `impl Trait for Struct`) **não é observada**
em `lab/typst-original/`.

---

## 3. Uso de trait objects

Procura: `grep -rn "Tracked<dyn " lab/typst-original/ --include="*.rs"`.

Resultado: **86 ocorrências** de `Tracked<dyn Trait + '_>` ou
`Tracked<dyn Trait + 'a>` em `lab/typst-original/`.

Distribuição rápida dos traits que aparecem tracked como trait object:

| Trait | Ocorrências | Forma |
|-------|-------------|-------|
| `World` | predominante | `Tracked<dyn World + '_>` |
| `Introspector` | também frequente | `Tracked<dyn Introspector + '_>` |

Exemplo de assinatura pública: `typst-eval/src/lib.rs:42`:
`pub fn eval(routines: &Routines, world: Tracked<dyn World + '_>, ...) -> SourceResult<Module>`.
Idêntico padrão em `typst-bundle/src/lib.rs:145–146` com
`world: Tracked<dyn World + '_>` + `introspector: Tracked<dyn Introspector + '_>`.

O padrão `Tracked<dyn Trait>` é **central** nas APIs públicas de
`typst-eval`, `typst-bundle`, `typst-html`, `typst-layout`. Não é
acidental — funções que atravessam fronteiras de crate recebem
dependências injectáveis via trait object tracked.

---

## 4. Caso `World`

Resposta às perguntas de B.4:

- **`World` é `trait` ou `struct`?** — É `trait`
  (`lab/typst-original/crates/typst-library/src/lib.rs:60`:
  `pub trait World: Send + Sync`).

- **Tem `#[comemo::track]`?** — Sim, na declaração do trait (linha 59,
  imediatamente antes de `pub trait World`).

- **Forma de aplicação?** — Categoria 3 (trait definition), não
  em `impl` block.

- **É usado como `Tracked<dyn World>` ou `Tracked<SomeWorld>`?** —
  Predominantemente como `Tracked<dyn World + '_>` (trait object).
  Existe também o macro `world_impl!` (lib.rs:100–136) que gera
  `impl<W: World> World for Box<W>`, `Arc<W>`, `&W` — mas isso é
  blanket forwarding, não tem `#[track]` próprio.

O cristalino já faz o mesmo (trait em L1 com `comemo::Tracked<dyn World>`
via o campo `_world: &dyn World` em `eval.rs`). Isto **não é
divergência**: o vanilla também usa o padrão trait + `#[track]` + trait
object para `World`.

---

## 5. Conclusão factual

O vanilla usa `#[comemo::track]` numa **mistura**: predominantemente
em `impl` de structs concretas (6 de 8 ocorrências, 75%), mas também
em definições de traits (2 de 8, 25%) — nomeadamente `World` e
`Introspector`. A escolha segue um critério observável: traits
tracked são precisamente os que representam dependências injectáveis
atravessando fronteiras de crate (`World`, `Introspector`); structs
tracked são tipos cuja identidade e invariante são definidos
localmente (`Traced`, `Sink`, `Route`, `Context`, `Locator`,
`LateLinkResolver`).

Categoria 2 (`impl Trait for Struct` com `#[track]`) **não é
observada** em `lab/typst-original/` — apenas categorias 1 e 3.

---

## 6. Magnitude da divergência estrutural

Se o cristalino adoptar o padrão "trait + `#[track]`" para os 6 tipos
que o vanilla implementa como struct concreta com `#[track]`, a
divergência estrutural seria:

- **6 structs vanilla** (`Traced`, `Sink`, `Route`, `Context`,
  `Locator`, `LateLinkResolver`) substituídas por traits no cristalino.
- **K impls** por struct que precisa existir no cristalino como
  "implementação default" para manter paridade. Valor exacto de K não
  observável sem construir a camada — mas o limite inferior é 1 (uma
  implementação por trait) e o limite superior depende de quantas
  variantes de cada "papel" o cristalino quiser expor (p. ex. `NoopSink`
  + `ProdSink`).

Em contraste, se o cristalino replicar o padrão vanilla exacto (trait
apenas para `World` e `Introspector`, struct concreta para os outros
6), a divergência estrutural em relação ao vanilla é **zero** neste
eixo.

Observações factuais adicionais (não prescritivas):

- O cristalino já adopta o padrão vanilla para `World` (trait em L1).
- Das 6 categorias-1 + 2 categorias-3 do vanilla, o cristalino só
  materializou `World` (categoria 3). As outras 7 estão por decidir.
- O ADR-0003 (`IDEIA`) propõe confinar `comemo` a L3 via trait
  `Trackable` — ortogonal a esta análise: diz "onde vive o `#[track]`"
  (L1 vs L3), não "que forma sintáctica" (struct vs trait).

---

## Referências

- Categoria 1: `typst-library/src/engine.rs:131,202,389` (`Traced`,
  `Sink`, `Route`); `foundations/context.rs:35` (`Context`);
  `introspection/locator.rs:208` (`Locator`); `model/link.rs:678`
  (`LateLinkResolver`).
- Categoria 3: `typst-library/src/lib.rs:59` (`World`);
  `introspection/introspector.rs:28` (`Introspector`).
- `00_nucleo/adr/typst-adr-0001-*.md` — `comemo` em `[l1_allowed_external]`.
- `00_nucleo/adr/typst-adr-0003-*.md` — `IDEIA` de confinar `comemo` a L3.
- `00_nucleo/diagnosticos/diagnostico-stubs-comemo-passo-86.md`
  — diagnóstico irmão da Tarefa A deste passo.
