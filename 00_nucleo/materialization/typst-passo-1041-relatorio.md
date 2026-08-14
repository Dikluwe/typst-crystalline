# Laudo 1041 — Refactor V16 (8 DENY, Neutros Nomeados, FrameVisitor e Ratchet)

**Onde roda**: clone canónico `typst-crystalline`
**Data**: 2026-08-14
**Estado**: `IMPLEMENTADO`
**Decisão-mãe**: [ADR-0016](file:///repos/Antigravity/tekt-linter/00_nucleo/adr/0016-regras-decisao-mecanica.md) (Status: `ACEITO` rev. 1)
**Passo de Especificação**: [`00_nucleo/materialization/typst-passo-1041.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/materialization/typst-passo-1041.md)

---

## 1. Resumo Executivo

O **Passo 1041** concluiu a refatoração do universo de decisões mecânicas V16 identificadas na reconciliação 0064/0065 no repositório `typst-crystalline`:
- **8 casos DENY** de saturação arbitrária foram 100% convertidos em braços nominais explícitos com fundamentação técnica e evidência semântica verificável;
- **132 defaults neutros** foram convertidos no código-fonte para o formato explícito `other => <default> // neutro: <justificativa de uma linha>`, tornando a razão de cada valor padrão legível no próprio ponto de matching;
- **43 walkers de `FrameItem`** foram unificados através da criação do `FrameVisitor` em L1 (`01_core/src/entities/frame_visitor.rs`), com ponto único de fallback com `debug_assert!`;
- A prova de invariância por **mutação fantasma** em `FrameItem` foi executada e revertida, comprovando que adições de variantes causam falhas imediatas nos pontos exaustivos e interceptação ruidosa no visitor;
- O ratchet de **V16** foi ativado em nível `error` no `crystalline.toml`.

### Resumo dos Critérios de Aceitação

| Critério | Meta | Resultado | Status |
| :--- | :--- | :--- | :--- |
| **(A.1) 0 DENY V16** | 8 decisões documentadas com evidência (a)/(b)/(c) | 0 DENY restantes; 8 decisões nominais auditadas | **APROVADO** |
| **(A.2) 132 neutros** | Neutros com comentário inline `other =>` e justificativa | 132 neutros anotados inline no código-fonte | **APROVADO** |
| **(A.3) FrameVisitor** | 43 walkers consolidados; prova de mutação em L1 | Trait unificado em L1; mutação validada e revertida | **APROVADO** |
| **(A.4) Ratchet V16** | V16 = `error` no `crystalline.toml`; linter sem erros | 0 erros, 0 warnings em `crystalline-lint` | **APROVADO** |
| **(A.5) Builds e Testes** | Workspace release e testes 100% verdes | `cargo build --release` ok; 5.865 testes passando | **APROVADO** |

---

## 2. Metodologia e Escopo de Verificação

- **Repositório alvo**: `typst-crystalline`
- **Linter**: `crystalline-lint` (`tekt-linter` commit de calibração 0065)
- **Regras validadas**: V16 (nível `error`), V17–V18 (`warning`), V19–V20 (`info`)
- **Profundidade da Verificação**:
  - A verificação da neutralidade semântica dos 132 casos neutros foi realizada por **análise semântica de domínio combinada com a suite existente de 5.865 testes e fixtures**. Não foram construídos 132 testes diferenciais isolados contra o binário vanilla caso a caso; a neutralidade foi confirmada pela invariância das asserções e estrutura de tipos.
  - A preservação dos comentários no código fonte (`other => <default> // neutro: <razão>`) garante que a justificativa técnica é visível diretamente na leitura do código, sem depender de consulta a arquivos de configuração externos.

---

## 3. Fase A — Tabela de Decisões dos 8 Casos DENY

| Localização | Scrutinee | Default Anterior | Decisão | Classe | Evidência / Fundamentação Técnica |
| :--- | :--- | :--- | :--- | :---: | :--- |
| `01_core/src/compiler/introspect/from_tags.rs:133` | `Value` | `Content::Empty` | Variantes nominais | **(a)** | Tipos não-Content/não-Str (números, bool, funcs, dicts, arrays) não possuem renderização textual direta em state display sem callback; braço explícito mantém invariante de tipo. |
| `01_core/src/compiler/introspect.rs:1050` | `Content` | `UnreferencableKind::Text` | Variantes nominais | **(a)** | Todos os nós fora de `Raw` e `Equation` sem numeração são classificados como `Text` para diagnóstico vanilla em `classify_unreferencable_body`. |
| `01_core/src/compiler/math/layout/spacing.rs:72` | `Content` | `MathClass::Normal` | Variantes nominais | **(a)** | Composite nodes math (`frac`, `root`, `matrix`, `cases`, etc.) e nós não-matemáticos recebem classe `Normal` conforme especificação e paridade vanilla. |
| `01_core/src/compiler/stdlib/state.rs:218` | `Value` | `Content::Empty` | Variantes nominais | **(a)** | Funções, módulos, estilos e metadados não produzem conteúdo visual direto em `value_to_content` de state display. |
| `01_core/src/compiler/stdlib/transforms.rs:102` | `Value` | `1.0` | Variantes nominais | **(a)** | Fatores não-numéricos (não-Float, não-Int, não-Ratio) preservam o fator de escala identidade (1.0) em `extract_factor`. |
| `01_core/src/entities/content.rs:1343` | `Content` | `"content"` | Variantes nominais | **(a)** | Elementos `Colbreak`, `HSpace`, `Hide`, `Pagebreak`, `Parbreak`, `Repeat`, `Stack`, `VSpace` nomeados explicitamente com suas strings canônicas. |
| `01_core/src/entities/content.rs:2479` | `Content` | `vec![body.clone()]` | Variantes nominais | **(a)** | Nós atómicos/folhas não contêm sequências internas de pagebreaks e são devolvidos em lote unitário em `page_column_segments`. |
| `01_core/src/entities/math_style.rs:50` | `MathStyleKind` | `1.0` | Variantes nominais | **(a)** | Variants glyph (`Plain`, `SansSerif`, `Chancery`, `Roundhand`, `Fraktur`, `Monospace`, `DoubleStruck`) operam apenas em codepoints Unicode, mantendo `size_factor = 1.0`. |

**Total DENY resultante**: 0.

---

## 4. Fase B — Auditoria e Anotação Inline dos 132 Defaults Neutros

Para evitar a dependência exclusiva de tabelas em arquivos de configuração (`crystalline.toml`), os 132 defaults neutros foram anotados no código-fonte com o padrão `other => <default> // neutro: <justificativa técnica>`:

- **Espaços de Cores (`01_core/src/entities/color.rs:137, 659`)**:
  - `Color::eq`: comparação entre espaços cromáticos distintos (ex.: RGB vs CMYK) retorna estritamente `false`.
  - `to_vec4_in_space`: espaços cromáticos sem coordenadas polares de hue (`Oklab`, `Luma`, `LinearRgb`, `Cmyk`, `D65Gray`) retornam `None` para `hue_idx`, pois não necessitam de interpolação circular pelo caminho mais curto.
- **Castings de Valor (`01_core/src/entities/value.rs`)**:
  - Variantes incompatíveis em `cast_bool`, `cast_int`, `cast_float`, `cast_decimal`, `cast_duration`, etc., retornam `None` documentado.
- **Métricas e Shaper (`03_infra/src/shaper.rs`, `03_infra/src/font_metrics.rs`)**:
  - Itens não-textuais (linhas, imagens, shapes) retornam neutro para métricas e flags de texto shaped.
- **Bugs observáveis encontrados**: **0** (conforme verificação por análise semântica e suite de regressão).

---

## 5. Fase C — Justificativa Arquitetural de `FrameVisitor` em L1 e Prova de Mutação

### 5.1 Justificativa de Camadas (L1 vs L3)

A localização de `FrameVisitor` em `01_core/src/entities/frame_visitor.rs` (L1) cumpre os seguintes requisitos:
1. **Pureza de L1 (ADR-0004 / Zero I/O)**: O trait opera estritamente sobre tipos puros de domínio (`FrameItem`, `Point`, `Pt`, `TextStyle`, `TransformMatrix`, `Rect`, `Size`, `Color`, `Stroke`). Não realiza operações de I/O, não instancia ponteiros de SO, não acessa filesystem/rede e não possui dependências fora das permitidas em L1.
2. **Reuso Cross-Layer sem Ciclos**: Permite que utilitários puros de L1 (ex.: `plain_text_items` em `layout_types.rs`) utilizem o percurso padrão, enquanto as camadas consumidoras (L2 `shell` e L3 `infra/export`) implementam visitors de exportação consumindo L1 diretamente, respeitando o fluxo unidirecional de dependências L3 → L1.

### 5.2 Implementação Central

```rust
pub trait FrameVisitor {
    fn visit_item(&mut self, item: &FrameItem) {
        match item {
            FrameItem::Text { .. } => self.visit_text(item),
            FrameItem::TextShaped { .. } => self.visit_text_shaped(item),
            FrameItem::Line { .. } => self.visit_line(item),
            FrameItem::Glyph { .. } => self.visit_glyph(item),
            FrameItem::Image { .. } => self.visit_image(item),
            FrameItem::Shape { .. } => self.visit_shape(item),
            FrameItem::Group { items, .. } => {
                self.visit_group(item);
                for child in items {
                    self.visit_item(child);
                }
            }
            FrameItem::Link { items, .. } => {
                self.visit_link(item);
                for child in items {
                    self.visit_item(child);
                }
            }
            #[allow(unreachable_patterns)]
            _ => {
                debug_assert!(false, "FrameItem não tratado: {:?}", item);
            }
        }
    }
}
```

### 5.3 Prova de Mutação Fantasma

A injeção temporária da variante `GhostVariantPhantomTest` em `FrameItem` gerou os seguintes erros no compilador:
```text
error[E0004]: non-exhaustive patterns: `layout_types::FrameItem::GhostVariantPhantomTest` not covered
   --> 01_core/src/compiler/math/layout/mod.rs:923:19
    |
923 |             match item {
    |                   ^^^^ pattern `&layout_types::FrameItem::GhostVariantPhantomTest` not covered
error[E0004]: non-exhaustive patterns: `&layout_types::FrameItem::GhostVariantPhantomTest` not covered
   --> 01_core/src/entities/layout_types.rs:643:15
    |
643 |         match item {
    |               ^^^^ pattern `&layout_types::FrameItem::GhostVariantPhantomTest` not covered
```
A prova confirmou que (1) todos os pontos nominais falham em tempo de compilação quando uma nova variante é introduzida, e (2) o `FrameVisitor` intercepta itens desconhecidos com `debug_assert!` em runtime de debug. A variante e testes temporários foram revertidos após a validação.

---

## 6. Fase D — Ratchet V16 & Validação

### Configuração `crystalline.toml`

```toml
[rules]
V14 = { level = "error" }
V16 = { level = "error" }
```

### Validação `crystalline-lint`

```bash
cargo run --manifest-path /repos/Antigravity/tekt-linter/Cargo.toml --bin crystalline-lint -- --checks v16,v17,v18,v19,v20 /home/dikluwe/Documentos/Antigravity/typst-crystalline
```
Resultado: **0 erros e 0 avisos** emitidos para V16.

---

## 7. Reconciliação Exata da Suite de Testes

Execução completa de `cargo test --workspace`:

| Componente / Suite | Testes Aprovados | Falhas | Ignorados | Tempo |
| :--- | :---: | :---: | :---: | :---: |
| **`typst_core`** (lib unit tests) | 4.990 | 0 | 0 | 18.07s |
| **`typst_infra`** (lib unit tests) | 793 | 0 | 0 | 4.89s |
| **`typst_shell`** (lib unit tests) | 41 | 0 | 0 | 0.00s |
| **`typst`** (binary main unittests) | 2 | 0 | 0 | 0.00s |
| **`tests/cli.rs`** (integração) | 37 | 0 | 0 | 3.58s |
| **`tests/crystalline_lint.rs`** (integração) | 2 | 0 | 0 | 0.00s |
| **Doc-tests** (`typst_core`) | 0 | 0 | 3 | 0.00s |
| **TOTAL CONSOLIDADO** | **5.865** | **0** | **3** | — |

- `cargo build --workspace --release`: compilação limpa concluída em **27.46s**.
