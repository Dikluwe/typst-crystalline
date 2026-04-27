//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content.md
//! @prompt-hash ec58d849
//! @layer L1
//! @updated 2026-04-25
//!
//! Excepção Regra 6 da ADR-0037: o enum `Content` é a entidade
//! fundamental do domínio visual — representa toda a árvore de
//! conteúdo do documento (markup, math, grid, figure, shape,
//! transform, align, place, etc.). Dividir por variante destruiria
//! a fonte única da verdade estrutural e forçaria consumidores
//! (eval, layout) a re-assemblar o enum via re-exports. ~1070 linhas
//! aceitas como custo de coesão por domínio.

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::counter_state::CounterAction;
use crate::entities::geometry::{ShapeKind, Stroke};
use crate::entities::label::Label;
use crate::entities::dir::Dir;
use crate::entities::layout_types::{Align2D, Color, Length, PlaceScope, Pt, TextStyle, TrackSizing, TransformMatrix};
use crate::entities::parity::Parity;
use crate::entities::ptr_eq_arc::PtrEqArc;
use crate::entities::sides::Sides;

/// Conteúdo declarativo produzido por `eval()`.
///
/// Diverge intencionalmente do original (`typst-library/foundations/content/`),
/// que usa vtable (`unsafe trait NativeElement`), proc macros e Arc manual.
/// Replicar essa metaprogramação em L1 seria arquitecturalmente inferior.
/// Enum linear com variantes declarativas — mais simples e testável.
///
/// **Invariante L1**: não desenha, não mede, não renderiza.
/// Qualquer operação que precise de métricas de fonte ou I/O pertence a L3.
///
/// `PartialEq` implementado manualmente — `Arc<[Content]>` compara por ponteiro
/// com `derive`, não por conteúdo (ADR-0026 revisão).
#[derive(Debug, Clone)]
pub enum Content {
    /// Conteúdo vazio.
    Empty,
    /// Texto simples com estilo capturado em eval (Passo 30).
    /// O estilo reflecte as `#set text()` rules activas no momento da produção.
    Text(EcoString, TextStyle),
    /// Espaço entre elementos (SpaceElem).
    Space,
    /// Sequência de elementos — clone O(1) via Arc (ADR-0026 revisão).
    Sequence(Arc<[Content]>),

    // ── Rich text (Passo 22, consolidado no Passo 101) ───────────────────
    // `Content::Strong` e `Content::Emph` removidos no Passo 101
    // (ADR-0038/0039): `*bold*` e `_italic_` passam a emitir
    // `Content::Styled(body, Styles::from_iter([Style::Bold(true) | Italic(true)]))`.
    // Os construtores `Content::strong(body)` e `Content::emph(body)` foram
    // redefinidos para preservar a API pública.
    /// Cabeçalho com nível 1–6 (`= Heading`).
    ///
    /// Permanece como variante dedicada: tem semântica adicional
    /// (`level` para introspecção, contadores hierárquicos). Futuro
    /// colapso em `Content::Styled` depende da materialização de
    /// `Introspection` — passo separado.
    Heading { level: u8, body: Box<Content> },

    // ── Passo 23 ────────────────────────────────────────────────────────────
    /// Código raw inline ou em bloco (`` `...` `` ou ```` ``` ... ``` ````).
    Raw {
        text:  EcoString,
        lang:  Option<EcoString>,
        block: bool,
    },
    /// Item de lista não ordenada (`- ...`).
    ListItem(Box<Content>),
    /// Item de lista ordenada (`+ ...` ou `1. ...`).
    EnumItem { number: Option<u32>, body: Box<Content> },
    /// Hiperligação (`https://...`).
    Link { url: EcoString, body: Box<Content> },

    // ── Matemática (Passo 34) ────────────────────────────────────────────────
    /// Equação matemática (`$...$` inline, `$ ... $` block).
    /// `block: true` → equação em linha própria (display mode).
    /// O motor de equações (Passo 36+) processa `body`.
    Equation {
        body:  Box<Content>,
        block: bool,
    },

    /// Sequência de nós matemáticos — corpo interno de uma equação.
    MathSequence(Arc<[Content]>),

    /// Identificador matemático: variável, função, símbolo (`x`, `sin`, `alpha`).
    MathIdent(EcoString),

    /// Texto literal em modo matemático (`"texto"` dentro de `$...$`).
    MathText(EcoString),

    /// Fracção matemática (`a/b` ou `frac(a, b)`).
    MathFrac {
        num: Box<Content>,
        den: Box<Content>,
    },

    /// Base com índice e/ou expoente (`x_1^2`, `{}^{14}_6 C`).
    /// `tl`/`bl` = pre-scripts à esquerda (Passo 46).
    /// `sub`/`sup` = scripts à direita.
    MathAttach {
        base: Box<Content>,
        tl:   Option<Box<Content>>, // top-left (pre-superscript)
        bl:   Option<Box<Content>>, // bottom-left (pre-subscript)
        sub:  Option<Box<Content>>, // bottom-right (subscript)
        sup:  Option<Box<Content>>, // top-right (superscript)
    },

    /// Raiz matemática (`√x`, `∛x`, `∜x`).
    /// `index`: None = raiz quadrada, Some(n) = raiz n-ésima.
    MathRoot {
        index:    Option<Box<Content>>,
        radicand: Box<Content>,
    },

    /// Expressão entre delimitadores (`(...)`, `[...]`, `{...}`).
    /// `open`/`close` são os caracteres delimitadores.
    /// Mantida como variante própria para que o layout possa
    /// seleccionar variantes de tamanho (Passo 42).
    MathDelimited {
        open:  char,
        body:  Box<Content>,
        close: char,
    },

    /// Ponto de alinhamento em equações matemáticas (`&`).
    /// Separa colunas no layout de grelha (Passo 51).
    MathAlignPoint,

    /// Quebra de linha em contexto matemático (`\\`).
    /// Separa linhas no layout de grelha (Passo 51).
    Linebreak,

    /// Matriz matemática produzida pela função `mat(...)`.
    /// `rows`: lista de linhas, cada linha é uma lista de células.
    /// `delim`: par de delimitadores (`('(', ')')` por defeito).
    MathMatrix {
        rows:  Vec<Vec<Content>>,
        delim: (char, char),
    },

    /// Função definida por ramos, produzida pela função `cases(...)`.
    /// `rows`: lista de ramos; cada ramo é um array de células (separadas por `&`).
    /// Delimitador esquerdo `{`; sem delimitador direito.
    MathCases {
        rows: Vec<Vec<Content>>,
    },

    /// Nó com etiqueta semântica (Passo 56).
    /// A `Label` é metainformação pura — não tem presença visual.
    /// Produzida por `= Título <label>` ou `#figure(...) <label>`.
    Labelled {
        target: Box<Content>,
        label:  Label,
    },

    /// Referência cruzada (Passo 56).
    /// Enquanto não existe motor de introspecção, renderiza literalmente `@nome`.
    Ref {
        target: Label,
    },

    // ── Introspecção / Contadores (Passo 57) ────────────────────────────────

    /// Activa ou desactiva a numeração automática de headings.
    /// Produzida por `#set heading(numbering: "1.1")` em eval.
    /// O Layouter consome esta variante actualizando `CounterState`.
    /// DEBT-10: substituir por StyleChain quando o motor de introspecção
    /// completo for implementado.
    SetHeadingNumbering { active: bool },

    /// Valor actual de um contador no ponto de inserção.
    /// Produzida por `counter(heading).get()` / `counter(heading).display()`.
    /// O Layouter resolve o valor no momento do layout (single-pass).
    /// DEBT-10: single-pass não suporta referências para a frente.
    CounterDisplay {
        /// Tipo de contador: "heading", "figure", "equation", ou chave arbitrária.
        kind: String,
    },

    /// Instrução de modificação de um contador (Passo 58).
    /// Produzida por `counter(key).step()` / `counter(key).update(n)`.
    /// O Layouter consome esta variante actualizando `CounterState`.
    CounterUpdate {
        key:    String,
        action: CounterAction,
    },

    /// Marcador para a Tabela de Conteúdos (Passo 61).
    /// O layouter substitui este nó pela lista de títulos do documento.
    Outline,

    /// Elemento com numeração própria e legenda opcional (Passo 62, DEBT-15 Passo 75).
    /// `kind` discrimina o contador: "image", "table", "raw", etc.
    /// `numbering` baked-in em eval via `#set figure(numbering: "1")` (DEBT-14).
    Figure {
        body:      Box<Content>,
        caption:   Option<Box<Content>>,
        /// Tipo da figura — discriminador para contadores independentes.
        /// Padrão: "image". Outros valores: "table", "raw".
        kind:      String,
        /// Padrão de numeração activo no momento da produção via `#set figure(numbering:)`.
        /// None → sem numeração; Some("1") → numeração arábica.
        numbering: Option<String>,
    },

    /// Activa a numeração automática de figuras a partir deste ponto (Passo 75).
    /// Produzida por `#set figure(numbering: "1")` em eval.
    /// Padrão idêntico a `SetHeadingNumbering` (Passo 57).
    SetFigureNumbering { pattern: String },

    /// Imagem carregada do disco (Passo 71, DEBT-24).
    ///
    /// `data: PtrEqArc<Vec<u8>>` — clones partilham a mesma alocação (O(1) clone)
    /// e PartialEq compara por ponteiro em vez de por valor (DEBT-26).
    /// `width`/`height` usam `Box<Value>` para quebrar o ciclo de tipos
    /// `Content → Value → Content` (sem Box seria recursão infinita).
    Image {
        path:   String,
        data:   PtrEqArc<Vec<u8>>,
        width:  Option<Box<crate::entities::value::Value>>,
        height: Option<Box<crate::entities::value::Value>>,
    },

    /// Forma geométrica primitiva (Passo 76).
    ///
    /// `width`/`height`: dimensões opcionais no AST — o layouter resolve os valores
    /// finais e emite `FrameItem::Shape` com `f64` concretos.
    /// `fill`/`stroke` resolvidos na stdlib — nunca por resolver no layouter.
    Shape {
        kind:   ShapeKind,
        width:  Option<Box<crate::entities::value::Value>>,
        height: Option<Box<crate::entities::value::Value>>,
        fill:   Option<Color>,
        stroke: Option<Stroke>,
    },

    /// Aplica uma transformação afim ao conteúdo interno (Passo 78).
    ///
    /// O layouter calcula a AABB do conteúdo transformado e reserva o espaço
    /// correcto na página. O exportador emite q → cm → conteúdo → Q.
    Transform {
        matrix: TransformMatrix,
        body:   Box<Content>,
    },

    /// Grid de colunas com células posicionadas por ordem de leitura (Passo 80).
    ///
    /// `rows` é consumido pelo layouter desde o Passo 83 (DEBT-34b encerrado).
    /// Comentário obsoleto removido na auditoria do Passo 105.
    Grid {
        columns: Vec<TrackSizing>,
        rows:    Vec<TrackSizing>,
        cells:   Vec<Content>,
    },

    /// Altera a configuração da página a partir deste ponto do documento (Passo 81).
    ///
    /// Se existir conteúdo na página actual, força uma quebra de página antes
    /// de aplicar a nova configuração. Se a página actual estiver vazia, aplica
    /// directamente sem quebra.
    SetPage {
        width:  Option<f64>,
        height: Option<f64>,
        margin: Option<f64>,
    },

    /// Altera a posição do conteúdo dentro do espaço disponível no fluxo (Passo 82).
    /// O cursor avança após o bloco — o espaço é consumido normalmente.
    Align {
        alignment: Align2D,
        body:      Box<Content>,
    },

    /// Posiciona o conteúdo de forma absoluta na página sem consumir espaço (Passo 82).
    /// O cursor não avança. Usado para cabeçalhos, rodapés e marcas de água.
    ///
    /// `scope` (Passo 84.6, encerra DEBT-37): `PlaceScope::Column` (default)
    /// ancora ao contentor activo (célula de Grid se houver, página caso
    /// contrário); `PlaceScope::Parent` ancora à página independentemente.
    Place {
        alignment: Align2D,
        dx:        f64,
        dy:        f64,
        scope:     PlaceScope,
        body:      Box<Content>,
    },

    /// Conteúdo estilizado — aplica um delta `Styles` ao corpo (Passo 99,
    /// ADR-0038).
    ///
    /// Fundação tipada para `#set`/`#show`. Ainda **não** é consumida pelo
    /// Layouter actual — é construída em testes de integração e pelo
    /// pipeline futuro quando o eval activar estilos. Acessores por
    /// `StyleChain::push_styles` garantem que o delta é aplicado na
    /// resolução.
    Styled(Box<Content>, crate::entities::style::Styles),

    // ── Estruturas de listas (Passo 154B, ADR-0060 Fase 1) ──────────────
    /// Separador horizontal estrutural (`#divider()`).
    /// Singleton sem dados; layouter emite linha horizontal.
    Divider,

    /// Lista de pares termo-descrição (`#terms(...)`) — Passo 154B.
    /// Cada item é tipicamente `Content::TermItem`.
    Terms { items: Vec<Content> },

    /// Par individual termo-descrição (Passo 154B).
    /// Aparece tipicamente dentro de `Content::Terms`, mas pode também
    /// surgir standalone (e.g. show rules futuras).
    TermItem { term: Box<Content>, description: Box<Content> },

    // ── Citação estrutural (Passo 155, ADR-0060 Fase 1, sub-passo 2) ────
    /// Citação estrutural com 4 atributos (vanilla `QuoteElem`).
    ///
    /// - `body`: conteúdo citado.
    /// - `attribution`: autor/fonte opcional.
    /// - `block`: `true` = parágrafo dedicado; `false` = inline.
    /// - `quotes`: `true` = aspas locale-apropriadas em torno do body.
    ///
    /// Smart-quotes resolvidas no layouter via
    /// `crate::rules::lang::quotes::localize_quotes(lang)` consultando
    /// `text.lang` activo (per ADR-0057).
    Quote {
        body:        Box<Content>,
        attribution: Option<Box<Content>>,
        block:       bool,
        quotes:      bool,
    },

    // ── Passo 156C (ADR-0061 Fase 1 sub-passo 1) — pad + hide ───────────
    // ── Passo 156L (ADR-0061 Fase 3 sub-passo 2; refino) — sides
    //    individualizadas: `padding: Sides<Length>` → `sides:
    //    Sides<Option<Length>>` per ADR-0064 Caso C (segunda
    //    aplicação concreta). `None` per lado ↔ default vanilla zero;
    //    distingue lado declarado vs lado não declarado para futura
    //    introspecção e show rules.
    /// Container que aplica padding ao body durante layout.
    ///
    /// `sides` em quatro lados (`Sides<Option<Length>>`); cada lado
    /// `None` ↔ default vanilla zero (resolvido em momento de uso no
    /// Layouter). Vanilla `PadElem` em
    /// `lab/typst-original/.../layout/pad.rs`. Atributos de stdlib
    /// (`left`/`right`/`top`/`bottom`/`x`/`y`/`rest`) resolvidos em
    /// `native_pad` com precedência específico > eixo > rest antes de
    /// chegar a este variant.
    Pad {
        body:  Box<Content>,
        sides: Sides<Option<Length>>,
    },

    /// Container que calcula dimensões mas não emite items visuais.
    ///
    /// Útil para placeholders e equilíbrio. Vanilla `HideElem` em
    /// `lab/typst-original/.../layout/hide.rs`. Cristalino preserva o
    /// avanço de cursor (consistente com vanilla "layout-aware mas não
    /// rende").
    Hide {
        body: Box<Content>,
    },

    // ── Passo 156D (ADR-0061 Fase 1 sub-passo 2) — h + v spacing ─────────
    /// Spacing primitive horizontal (vanilla `HElem`).
    ///
    /// `amount` em `Length`; `weak` armazenado mas comportamento de
    /// collapse adiado (perfil ADR-0054 graded). Layouter avança
    /// `cursor_x` por `amount`. Vanilla aceita `Fraction`; cristalino
    /// só aceita `Length` neste passo (ADR-0061 §6.3 refino futuro).
    HSpace {
        amount: Length,
        weak:   bool,
    },

    /// Spacing primitive vertical (vanilla `VElem`).
    ///
    /// Análogo a `HSpace` mas em eixo Y. Layouter força `flush_line`
    /// antes de avançar `cursor_y` (caso contrário texto na linha
    /// actual fica meio-render).
    VSpace {
        amount: Length,
        weak:   bool,
    },

    // ── Passo 156E (ADR-0061 Fase 1 sub-passo 3) — pagebreak manual ──────
    /// Quebra de página manual (vanilla `PagebreakElem`).
    ///
    /// `weak` armazenado mas comportamento de collapse adiado
    /// (perfil ADR-0054 graded; consistente com P156D HSpace/VSpace).
    /// `to: Some(parity)` força a próxima página a ter paridade
    /// especificada — Layouter insere página vazia se necessário.
    /// `to: None` == Auto (sem ajuste).
    Pagebreak {
        weak: bool,
        to:   Option<Parity>,
    },

    // ── Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo ──
    /// Container compositivo — vanilla `StackElem`. **Último sub-passo
    /// Fase 2; atinge target 72% Layout** declarado em ADR-0061.
    ///
    /// Distinção material face a Block/Boxed: **Arc<[Content]>** em vez
    /// de body único; atributos próprios `dir` (4 direcções) e
    /// `spacing` entre children.
    ///
    /// Decisão arquitectural reusada de P156G/H (variant rico) com
    /// adaptação para `Arc<[Content]>` (clone O(1) per ADR-0026
    /// revisão, consistente com `Sequence`/`MathSequence`).
    Stack {
        children: Arc<[Content]>,
        /// Direcção de empilhamento. Default `TTB`.
        dir:      Dir,
        /// Espaço entre children (avanço cursor); `None` == zero.
        spacing:  Option<Length>,
    },

    // ── Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container ──
    /// Container inline — vanilla `BoxElem`.
    ///
    /// Distinção material face a `Block` (P156G): **posicionamento
    /// inline** (não força flush_line). Atributos comuns com Block:
    /// `body`, `width`, `height`, `inset`. Atributo único: `baseline`.
    ///
    /// Decisão arquitectural P156H: variant rico (Opção A) reusada do
    /// padrão estabelecido em P156G — containers ricos preferem
    /// variants explícitos quando atributos não são propriedades de
    /// texto.
    ///
    /// Atributos vanilla scope-out per ADR-0054 graded: `outset`,
    /// `fill`, `stroke`, `radius`, `clip`, `stroke-overhang`.
    ///
    /// Naming: variant Rust é `Boxed` (não `Box`) para evitar confusão
    /// com `std::boxed::Box`; stdlib expõe `#box(...)` (paridade
    /// vanilla). Construtor Rust: `Content::boxed(...)`.
    Boxed {
        body:     Box<Content>,
        /// Largura explícita; `None` == content-based.
        width:    Option<Length>,
        /// Altura explícita; `None` == auto.
        height:   Option<Length>,
        /// Margem interna em quatro lados.
        inset:    Sides<Length>,
        /// Ajuste vertical da baseline; positivo move para baixo.
        /// Semantic real adiada se layouter actual não suporta
        /// (consistente com `breakable: false` em Block).
        baseline: Length,
    },

    // ── Passo 156G (ADR-0061 Fase 2 sub-passo 1) — block container ───────
    /// Container block — vanilla `BlockElem`.
    ///
    /// Forma minimalista per ADR-0054 graded. Atributos Fase 1
    /// (P156G): `body`, `width`, `height`, `inset`, `breakable`.
    /// Scope-out (refino futuro): `outset`, `fill`, `stroke`, `radius`,
    /// `clip`, `spacing`, `above`/`below`, `sticky`.
    ///
    /// Decisão arquitectural P156G.2: variant rico (Opção A) em vez de
    /// `Content::Styled`. Rationale: Block é container de layout (width/
    /// height/inset/breakable) — vocabulário diferente do `Style` enum
    /// que cobre propriedades de texto (Bold/Italic/Size/Fill/HeadingLevel).
    /// Coerente com `Content::Pad` (P156C) que também tem fields explícitos
    /// para padding.
    Block {
        body:      Box<Content>,
        /// Largura explícita; `None` == auto (largura disponível).
        width:     Option<Length>,
        /// Altura explícita; `None` == auto (calcular do body).
        height:    Option<Length>,
        /// Margem interna em quatro lados.
        inset:     Sides<Length>,
        /// `true` = pode quebrar entre páginas; `false` = atómico
        /// (semantic adiada per ADR-0054 graded; armazenado mas
        /// layouter não impede quebra ainda).
        breakable: bool,
    },

    // ── Passo 157B (ADR-0060 Fase 2 sub-passo 2) — table cell ───────────
    /// Cell estruturada de Table — vanilla `TableCell`.
    /// **Segundo sub-passo Model Fase 2**.
    ///
    /// Subset minimal per ADR-0054 graded e diagnóstico P157B §1:
    /// 5 fields críticos (body/x/y/colspan/rowspan); 6 atributos
    /// vanilla scope-out (align/stroke/fill/inset/breakable +
    /// internal fields kind/is_repeated).
    ///
    /// Decisões arquitecturais (per diagnóstico P157B):
    /// - `x`/`y`: ADR-0064 **Caso A** (`Smart<usize>` → `Option<usize>`;
    ///   None ↔ Auto auto-placement). **Primeira aplicação concreta
    ///   de Caso A em domínio Model** (P156G/H/I aplicaram-no em
    ///   Layout).
    /// - `colspan`/`rowspan`: ADR-0064 **Caso C** (`NonZeroUsize`
    ///   default 1 → `Option<usize>` com `None` ↔ default 1; zero
    ///   rejeitado em stdlib). **Primeira variação `usize` do Caso
    ///   C**; anteriores eram `Length`.
    ///
    /// Layouter renderiza `body` no contexto actual; `x`/`y`/colspan/
    /// rowspan **armazenados mas ignorados** per ADR-0054 graded —
    /// algoritmo de placement diferido em **DEBT-34e** (refactor
    /// dedicado a placement Grid completo).
    TableCell {
        body:    Box<Content>,
        x:       Option<usize>,
        y:       Option<usize>,
        colspan: Option<usize>,
        rowspan: Option<usize>,
    },

    // ── Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado) ──
    /// Lista bibliográfica — vanilla `BibliographyElem`.
    /// **Primeiro sub-passo Bibliography + Cite Model Fase 2**
    /// (par acoplado com `Cite`).
    ///
    /// Subset minimal per ADR-0054 graded e diagnóstico P159A §1:
    /// 2 fields críticos (entries/title); 6+ fields vanilla
    /// scope-out (sources/full/style/lang/region) + acoplamento
    /// hayagriva diferido per ADR-0062 reserva sem ficheiro.
    ///
    /// **Input cristalino literal `Vec<BibEntry>`** — sem
    /// parsing externo (`.bib`/`.yaml`). Refinos futuros
    /// (hayagriva, CSL) NÃO reservados per política P158.
    ///
    /// `title: Option<Box<Content>>` per ADR-0064 **Caso A**
    /// (Smart<Option<Content>> vanilla → Option<Box<Content>>
    /// cristalino). Patamar Caso A cresce N=4 → 5 com P159A.
    ///
    /// Layouter renderiza title (se Some) seguido de lista de
    /// entries formatadas como `"[{key}] {author}. {title}
    /// ({year})."` per ADR-0033 + ADR-0054 graded — paridade
    /// vanilla observable mínima.
    Bibliography {
        entries: Vec<crate::entities::bib_entry::BibEntry>,
        title:   Option<Box<Content>>,
    },

    /// Citação inline — vanilla `CiteElem`.
    /// Par com `Bibliography` (acoplamento semântico vanilla
    /// inseparável: cite referencia entries de bibliography).
    ///
    /// Subset minimal per ADR-0054 graded:
    /// 2 fields críticos (key/supplement); 2+ fields vanilla
    /// scope-out (form Normal/Prose/etc., style override CSL).
    ///
    /// `key: String` directo (paridade vanilla `Label` simplificado).
    /// `supplement: Option<Box<Content>>` per ADR-0064 Caso A
    /// (page/chapter override).
    ///
    /// Layouter renderiza placeholder `"[{key}]"` seguido de
    /// supplement (se Some) per ADR-0033 + ADR-0054 graded.
    /// **Sem validação cross-reference** `key ∈ Bibliography.keys`
    /// — diferida per ADR-0017 Introspection runtime adiada
    /// (cite cross-document ficaria como TODO).
    Cite {
        key:        String,
        supplement: Option<Box<Content>>,
    },

    // ── Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha table foundations) ──
    /// Header repetível de Table — vanilla `TableHeader`.
    /// **Terceiro e último sub-passo Model Fase 2**.
    ///
    /// Par simétrico com `TableFooter`. Subset minimal per
    /// ADR-0054 graded e diagnóstico P157C §1.3:
    /// 2 fields (body/repeat); fields diferidos: `level: NonZeroU32`
    /// (hierarquia Header), `repeat-rows: Smart<usize>`, children
    /// variádicos estruturados.
    ///
    /// **Divergência aceite per ADR-0033**: vanilla usa
    /// `#[variadic] children: Vec<TableItem>`; cristalino usa
    /// `body: Box<Content>` para uniformidade com containers
    /// existentes (use Sequence se múltiplos children necessários).
    ///
    /// `repeat: bool` ADR-0064 **Caso D** (`bool` directo com
    /// default `true` paridade vanilla — **primeira aplicação
    /// Caso D em domínio Model**; P156D weak / P156G breakable /
    /// P156J justify aplicaram-no em Layout).
    ///
    /// Layouter renderiza `body` no contexto actual; **`repeat`
    /// armazenado mas ignorado** per ADR-0054 graded — algoritmo
    /// de repetição em page breaks diferido em **DEBT-56**
    /// (refactor multi-region).
    TableHeader {
        body:   Box<Content>,
        repeat: bool,
    },

    /// Footer repetível de Table — vanilla `TableFooter`.
    /// Par simétrico com `TableHeader` (paridade absoluta:
    /// mesmos fields; `level` vanilla diferido em ambos para
    /// preservar simetria cristalina).
    ///
    /// Mesma divergência `body: Box<Content>` aceite per ADR-0033.
    /// Mesma decisão `repeat: bool` ADR-0064 Caso D.
    /// Mesma limitação per ADR-0054 graded (DEBT-56).
    TableFooter {
        body:   Box<Content>,
        repeat: bool,
    },

    // ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table minimal ────────
    /// Container tabular semântico — vanilla `TableElem`.
    /// **Primeiro sub-passo Model Fase 2**.
    ///
    /// Subset minimal per ADR-0054 graded e diagnóstico P157 §3:
    /// 3 fields críticos (columns/rows/children); ~9 atributos vanilla
    /// scope-out (gutter/inset/align/fill/stroke/summary; cells
    /// estruturadas + header/footer diferidos para P157B/C).
    ///
    /// Estruturalmente análogo a `Content::Grid` mas semanticamente
    /// distinto per ADR-0060 §"Decisão 4" (Model structural exige
    /// variant dedicado; reaproveitamento de Grid vive só no
    /// algoritmo de layout, não no enum). Field `children` (não `cells`)
    /// segue nomenclatura vanilla `Vec<TableChild>`; pequena
    /// divergência intra-cristalino vs `Grid.cells` documentada
    /// em diagnóstico P157A §3.2.
    ///
    /// Layouter delega a `layout_grid` clone simples — sem
    /// modificação de `grid.rs` per diagnóstico P157A §10.
    Table {
        columns:  Vec<TrackSizing>,
        rows:     Vec<TrackSizing>,
        children: Vec<Content>,
    },

    // ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat ────────────────
    /// Repetição de body para preencher espaço (vanilla `RepeatElem`).
    /// **Primeira aplicação Fase 3** declarada em ADR-0061.
    ///
    /// Caso de uso primário: TOC dot leaders `#box(width: 1fr,
    /// repeat[.])`. Em vanilla, o algoritmo de runtime calcula
    /// quantidade-para-encher dinamicamente; em cristalino, P156J
    /// implementa **paridade estrutural** (variant + stdlib + medição
    /// estática + layout single-render) per ADR-0054 graded —
    /// algoritmo dinâmico diferido para refino futuro (mesmo critério
    /// aceite em P156G/H/I para containers complexos).
    ///
    /// **Atributos** (paridade vanilla; total 3 fields):
    /// - `body`: conteúdo a repetir (obrigatório).
    /// - `gap: Option<Length>`: espaço entre cópias; `None` == zero
    ///   (padrão Smart→Option N=6 da série P156D-I).
    /// - `justify: bool`: default `true` (paridade vanilla;
    ///   distribuição de espaço residual diferida per ADR-0054).
    Repeat {
        body:    Box<Content>,
        gap:     Option<Length>,
        justify: bool,
    },

    // Variantes futuras — NÃO implementar sem ADR:
    // Elem(Arc<dyn NativeElement>),               // vtable — Passo 20+
}

impl Content {
    /// Cria conteúdo de texto com estilo por defeito (regular 11pt).
    /// Em eval, usar `Content::Text(s, TextStyle::from(&ctx.styles))` directamente
    /// para capturar o estilo activo no momento da produção.
    pub fn text(s: impl Into<EcoString>) -> Self {
        Self::Text(s.into(), TextStyle::regular(Pt(11.0)))
    }

    /// Cria conteúdo vazio.
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Cria uma sequência, normalizando casos degenerados.
    ///
    /// - 0 partes → `Empty`
    /// - 1 parte → desembrulha (evita `Sequence([x])`)
    /// - n > 1 → `Sequence(parts)`
    /// Negrito — produz `Content::Styled([Style::Bold(true)], body)`
    /// (Passo 101, ADR-0038/0039). A variante `Content::Strong` foi
    /// removida do enum; os callers continuam a usar este construtor.
    pub fn strong(body: Content) -> Self {
        use crate::entities::style::{Style, Styles};
        Self::Styled(Box::new(body), Styles::from_iter([Style::Bold(true)]))
    }

    /// Itálico — produz `Content::Styled([Style::Italic(true)], body)`
    /// (Passo 101, ADR-0038/0039).
    pub fn emph(body: Content) -> Self {
        use crate::entities::style::{Style, Styles};
        Self::Styled(Box::new(body), Styles::from_iter([Style::Italic(true)]))
    }
    pub fn heading(level: u8, body: Content) -> Self {
        Self::Heading { level: level.clamp(1, 6), body: Box::new(body) }
    }

    pub fn raw(text: impl Into<EcoString>, lang: Option<EcoString>, block: bool) -> Self {
        Self::Raw { text: text.into(), lang, block }
    }
    pub fn list_item(body: Content) -> Self { Self::ListItem(Box::new(body)) }
    pub fn enum_item(number: Option<u32>, body: Content) -> Self {
        Self::EnumItem { number, body: Box::new(body) }
    }
    pub fn link(url: impl Into<EcoString>, body: Content) -> Self {
        Self::Link { url: url.into(), body: Box::new(body) }
    }

    /// `pad(body, sides)` — Passo 156C (ADR-0061 Fase 1) /
    /// Passo 156L (refino sides individualizadas per ADR-0064 Caso C).
    pub fn pad(body: Content, sides: Sides<Option<Length>>) -> Self {
        Self::Pad { body: Box::new(body), sides }
    }

    /// `hide(body)` — Passo 156C (ADR-0061 Fase 1).
    pub fn hide(body: Content) -> Self {
        Self::Hide { body: Box::new(body) }
    }

    /// `h(amount, weak)` — Passo 156D (ADR-0061 Fase 1 sub-passo 2).
    pub fn h_space(amount: Length, weak: bool) -> Self {
        Self::HSpace { amount, weak }
    }

    /// `v(amount, weak)` — Passo 156D (ADR-0061 Fase 1 sub-passo 2).
    pub fn v_space(amount: Length, weak: bool) -> Self {
        Self::VSpace { amount, weak }
    }

    /// `pagebreak(weak, to)` — Passo 156E (ADR-0061 Fase 1 sub-passo 3).
    pub fn pagebreak(weak: bool, to: Option<Parity>) -> Self {
        Self::Pagebreak { weak, to }
    }

    /// `block(body, width, height, inset, breakable)` — Passo 156G
    /// (ADR-0061 Fase 2 sub-passo 1). Construtor com defaults sensatos
    /// (None/zero/true) para uso programático.
    pub fn block(
        body:      Content,
        width:     Option<Length>,
        height:    Option<Length>,
        inset:     Sides<Length>,
        breakable: bool,
    ) -> Self {
        Self::Block { body: Box::new(body), width, height, inset, breakable }
    }

    /// `box(body, width, height, inset, baseline)` — Passo 156H
    /// (ADR-0061 Fase 2 sub-passo 2). Naming `boxed` evita conflito com
    /// `std::boxed::Box`; stdlib expõe `#box(...)` (paridade vanilla).
    pub fn boxed(
        body:     Content,
        width:    Option<Length>,
        height:   Option<Length>,
        inset:    Sides<Length>,
        baseline: Length,
    ) -> Self {
        Self::Boxed { body: Box::new(body), width, height, inset, baseline }
    }

    /// `stack(dir, spacing, ..children)` — Passo 156I (ADR-0061 Fase 2
    /// sub-passo 3). Atinge target 72% Layout. Aceita Vec<Content> que
    /// converte para `Arc<[Content]>` (clone O(1) per ADR-0026 revisão).
    pub fn stack(
        children: Vec<Content>,
        dir:      Dir,
        spacing:  Option<Length>,
    ) -> Self {
        Self::Stack { children: children.into(), dir, spacing }
    }

    /// `repeat(body, gap, justify)` — Passo 156J (ADR-0061 Fase 3
    /// sub-passo 1). **Primeira Fase 3**. Default `justify == true`
    /// (paridade vanilla); algoritmo dinâmico de quantidade-para-encher
    /// diferido per ADR-0054 graded.
    pub fn repeat(
        body:    Content,
        gap:     Option<Length>,
        justify: bool,
    ) -> Self {
        Self::Repeat { body: Box::new(body), gap, justify }
    }

    /// `table(columns, rows, ..children)` — Passo 157A (ADR-0060
    /// Fase 2 sub-passo 1; **primeiro sub-passo Model Fase 2**).
    /// Subset minimal: cells distribuídas como `Content::Grid`;
    /// TableCell estruturado + Header/Footer diferidos para P157B/C.
    pub fn table(
        columns:  Vec<TrackSizing>,
        rows:     Vec<TrackSizing>,
        children: Vec<Content>,
    ) -> Self {
        Self::Table { columns, rows, children }
    }

    /// `table_cell(body, x, y, colspan, rowspan)` — Passo 157B
    /// (ADR-0060 Fase 2 sub-passo 2). `x`/`y` ADR-0064 Caso A;
    /// `colspan`/`rowspan` Caso C. Placement algorítmico diferido
    /// em DEBT-34e — fields armazenados mas ignorados em layout.
    pub fn table_cell(
        body:    Content,
        x:       Option<usize>,
        y:       Option<usize>,
        colspan: Option<usize>,
        rowspan: Option<usize>,
    ) -> Self {
        Self::TableCell { body: Box::new(body), x, y, colspan, rowspan }
    }

    /// `table_header(body, repeat)` — Passo 157C (ADR-0060 Fase 2
    /// sub-passo 3 — **fecha table foundations**). `repeat: bool`
    /// ADR-0064 Caso D (default `true` paridade vanilla — primeira
    /// aplicação Caso D em Model). Algoritmo de repetição diferido
    /// em DEBT-56.
    pub fn table_header(body: Content, repeat: bool) -> Self {
        Self::TableHeader { body: Box::new(body), repeat }
    }

    /// `table_footer(body, repeat)` — par simétrico de `table_header`
    /// (Passo 157C). Mesma decisão Caso D + DEBT-56.
    pub fn table_footer(body: Content, repeat: bool) -> Self {
        Self::TableFooter { body: Box::new(body), repeat }
    }

    /// `bibliography(entries, title)` — Passo 159A (par acoplado
    /// com `cite`). Subset minimal per ADR-0054 graded; input
    /// cristalino literal `Vec<BibEntry>`; sem hayagriva.
    pub fn bibliography(
        entries: Vec<crate::entities::bib_entry::BibEntry>,
        title:   Option<Content>,
    ) -> Self {
        Self::Bibliography {
            entries,
            title: title.map(Box::new),
        }
    }

    /// `cite(key, supplement)` — Passo 159A (par acoplado com
    /// `bibliography`). Sem validação cross-reference (ADR-0017
    /// Introspection runtime adiada).
    pub fn cite(key: impl Into<String>, supplement: Option<Content>) -> Self {
        Self::Cite {
            key: key.into(),
            supplement: supplement.map(Box::new),
        }
    }

    pub fn sequence(parts: Vec<Content>) -> Self {
        match parts.len() {
            0 => Self::Empty,
            1 => parts.into_iter().next().unwrap(),
            _ => Self::Sequence(parts.into()),  // Vec<Content> → Arc<[Content]>
        }
    }

    /// Retorna `true` se este conteúdo não contém informação visível.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Sequence(v) => v.is_empty(),
            Self::Labelled { target, .. } => target.is_empty(),
            // Figura: não está vazia se tiver body OU caption com conteúdo.
            Self::Figure { body, caption, .. } =>
                body.is_empty() && caption.as_ref().is_none_or(|c| c.is_empty()),
            Self::Grid { cells, .. } => cells.is_empty(),
            // Passo 157A (ADR-0060 Fase 2): Table é vazio se children
            // for vazio (paridade com Grid; cells / children indistintos
            // semanticamente para is_empty).
            Self::Table { children, .. } => children.is_empty(),
            // Passo 157B (ADR-0060 Fase 2 sub-passo 2): TableCell vazio
            // se body for (atributos x/y/colspan/rowspan não tornam o
            // container não-vazio — paridade Block/Boxed).
            Self::TableCell { body, .. } => body.is_empty(),
            // Passo 157C (ADR-0060 Fase 2 sub-passo 3): par simétrico
            // TableHeader/TableFooter vazio se body for (atributo
            // repeat não torna o container não-vazio — paridade Block/Boxed).
            Self::TableHeader { body, .. } => body.is_empty(),
            Self::TableFooter { body, .. } => body.is_empty(),
            // Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par
            // acoplado). Bibliography vazio se entries vazias E title
            // None. Cite nunca vazio (key sempre presente; placeholder
            // `[key]` é sempre observable).
            Self::Bibliography { entries, title } =>
                entries.is_empty() && title.is_none(),
            Self::Cite { .. } => false,
            // Passo 154B: Divider é singleton estrutural, nunca vazio.
            // Terms vazio (sem items) é considerado vazio; TermItem vazio
            // se ambos os lados forem vazios.
            Self::Divider => false,
            Self::Terms { items } => items.is_empty(),
            Self::TermItem { term, description } =>
                term.is_empty() && description.is_empty(),
            // Passo 155: Quote vazio se body for vazio.
            Self::Quote { body, .. } => body.is_empty(),
            // Passo 156C (ADR-0061 Fase 1): Pad/Hide vazios se o body for.
            Self::Pad  { body, .. } => body.is_empty(),
            Self::Hide { body }     => body.is_empty(),
            // Passo 156D: HSpace/VSpace vazios se amount for zero.
            Self::HSpace { amount, .. } => amount.is_zero(),
            Self::VSpace { amount, .. } => amount.is_zero(),
            // Passo 156E: Pagebreak nunca é vazio (event com efeito
            // mesmo sem body; cf. Divider em P154B).
            Self::Pagebreak { .. } => false,
            // Passo 156G: Block é vazio se o body for (atributos de
            // dimensão/inset não fazem o container deixar de ser vazio
            // semanticamente — análogo a Pad em P156C).
            Self::Block { body, .. } => body.is_empty(),
            // Passo 156H: Boxed (Box inline) — proxy análogo a Block.
            Self::Boxed { body, .. } => body.is_empty(),
            // Passo 156I: Stack é vazio se TODOS os children forem vazios
            // (consistente com Sequence; stack vazio é semanticamente
            // sem conteúdo).
            Self::Stack { children, .. } => children.iter().all(|c| c.is_empty()),
            // Passo 156J: Repeat é vazio se body for (atributos não
            // tornam o container não-vazio — análogo a Block/Boxed).
            Self::Repeat { body, .. } => body.is_empty(),
            _ => false,
        }
    }

    /// Extrai texto plano recursivamente — para verificação em testes.
    pub fn plain_text(&self) -> String {
        match self {
            Self::Empty                 => String::new(),
            Self::Text(s, _)            => s.to_string(),
            Self::Space              => " ".to_string(),
            Self::Sequence(v)        => v.iter().map(|c| c.plain_text()).collect(),
            // Passo 101: Content::Strong/Emph removidos — cobertos por
            // Content::Styled(body, _) => body.plain_text() no fim do match.
            Self::Heading { body, .. } => body.plain_text(),
            Self::Raw { text, .. }   => text.to_string(),
            Self::ListItem(c)        => format!("• {}", c.plain_text()),
            Self::EnumItem { number, body } => {
                let n = number.map(|n| format!("{}. ", n)).unwrap_or_default();
                format!("{}{}", n, body.plain_text())
            }
            Self::Link { body, .. }  => body.plain_text(),
            Self::Equation { body, block } => {
                if *block { format!("\n{}\n", body.plain_text()) }
                else       { body.plain_text() }
            }
            Self::MathSequence(nodes) => nodes.iter().map(|n| n.plain_text()).collect(),
            Self::MathIdent(s)        => s.to_string(),
            Self::MathText(s)         => s.to_string(),
            Self::MathFrac { num, den } => {
                format!("({})/({})", num.plain_text(), den.plain_text())
            }
            Self::MathAttach { base, tl, bl, sub, sup } => {
                let mut s = String::new();
                if let Some(tl) = tl { s.push_str(&format!("^{}", tl.plain_text())); }
                if let Some(bl) = bl { s.push_str(&format!("_{}", bl.plain_text())); }
                s.push_str(&base.plain_text());
                if let Some(sub) = sub { s.push_str(&format!("_{}", sub.plain_text())); }
                if let Some(sup) = sup { s.push_str(&format!("^{}", sup.plain_text())); }
                s
            }
            Self::MathRoot { index, radicand } => match index {
                None    => format!("sqrt({})", radicand.plain_text()),
                Some(i) => format!("root({}, {})", i.plain_text(), radicand.plain_text()),
            },
            Self::MathDelimited { open, body, close } => {
                format!("{}{}{}", open, body.plain_text(), close)
            }
            Self::MathAlignPoint => String::new(),
            Self::Linebreak      => "\n".to_string(),
            Self::MathMatrix { rows, .. } => {
                rows.iter().map(|row| {
                    row.iter().map(|c| c.plain_text()).collect::<Vec<_>>().join(", ")
                }).collect::<Vec<_>>().join("; ")
            }
            Self::MathCases { rows } => {
                rows.iter().map(|row| {
                    row.iter().map(|c| c.plain_text()).collect::<Vec<_>>().join(" & ")
                }).collect::<Vec<_>>().join(", ")
            }
            Self::Labelled { target, .. } => target.plain_text(),
            Self::Ref { target }          => format!("@{}", target.0),
            Self::SetHeadingNumbering { .. } => String::new(),
            Self::CounterDisplay { .. }      => String::new(),
            Self::CounterUpdate { .. }       => String::new(),
            Self::Outline                    => String::new(),
            Self::Figure { body, caption, .. } => {
                let body_text = body.plain_text();
                let cap_text  = caption.as_ref()
                    .map(|c| c.plain_text())
                    .unwrap_or_default();
                match (body_text.is_empty(), cap_text.is_empty()) {
                    (false, false) => format!("{} {}", body_text, cap_text),
                    (false, true)  => body_text,
                    (true,  false) => cap_text,
                    (true,  true)  => String::new(),
                }
            }
            Self::SetFigureNumbering { .. } => String::new(),
            Self::Image { .. } => String::new(),
            Self::Shape { .. } => String::new(),
            Self::Transform { body, .. } => body.plain_text(),
            Self::Grid { cells, .. } => {
                cells.iter().map(|c| c.plain_text()).collect::<Vec<_>>().join(" ")
            }
            // Passo 157A: Table concatena children com space (paridade
            // com Grid em plain_text — semântica de "células visíveis
            // em sequência").
            Self::Table { children, .. } => {
                children.iter().map(|c| c.plain_text()).collect::<Vec<_>>().join(" ")
            }
            // Passo 157B: TableCell é transparente para texto plano —
            // recurse no body sem multiplicar por colspan/rowspan
            // (paridade não visível em texto plano; spans são
            // runtime-only e diferidos em DEBT-34e).
            Self::TableCell { body, .. } => body.plain_text(),
            // Passo 157C: par simétrico TableHeader/TableFooter
            // transparente para texto plano — recurse no body sem
            // multiplicar por repeat (semântica de page-break
            // repetição não visível em texto plano; diferida em
            // DEBT-56).
            Self::TableHeader { body, .. } => body.plain_text(),
            Self::TableFooter { body, .. } => body.plain_text(),
            // Passo 159A: Bibliography concatena title (se Some) +
            // entries formatadas. Cite emite `"[{key}]"` placeholder
            // + supplement.
            Self::Bibliography { entries, title } => {
                let mut out = String::new();
                if let Some(t) = title {
                    out.push_str(&t.plain_text());
                    out.push('\n');
                }
                for e in entries {
                    out.push_str(&format!(
                        "[{}] {}. {} ({}).\n",
                        e.key, e.author, e.title, e.year
                    ));
                }
                out
            }
            Self::Cite { key, supplement } => {
                let mut out = format!("[{}]", key);
                if let Some(s) = supplement {
                    out.push_str(&s.plain_text());
                }
                out
            }
            Self::SetPage { .. } => String::new(),
            Self::Align { body, .. } => body.plain_text(),
            Self::Place { body, .. } => body.plain_text(),
            Self::Styled(body, _) => body.plain_text(),
            // Passo 154B: Divider é structural sem texto; Terms concatena
            // pares por linha; TermItem produz "term: description".
            Self::Divider => String::new(),
            Self::Terms { items } => items.iter()
                .map(|t| t.plain_text())
                .collect::<Vec<_>>()
                .join("\n"),
            Self::TermItem { term, description } => format!(
                "{}: {}",
                term.plain_text(),
                description.plain_text(),
            ),
            // Passo 155: Quote em texto plain usa ASCII fallback (sem
            // smart-quotes — interaction com lang só vive no layouter).
            // Com attribution: `"body" — attribution`; sem: `"body"`.
            // Passo 156C: Pad é transparente para texto plano (recurse no
            // body sem alterar texto). Hide produz string vazia (não rende).
            Self::Pad  { body, .. } => body.plain_text(),
            Self::Hide { .. }       => String::new(),
            // Passo 156D: HSpace/VSpace são spacing primitives sem texto.
            Self::HSpace { .. } | Self::VSpace { .. } => String::new(),
            // Passo 156E: Pagebreak é event sem texto.
            Self::Pagebreak { .. } => String::new(),
            // Passo 156G: Block é transparente para texto plano (recurse
            // no body; análogo a Pad em P156C).
            Self::Block { body, .. } => body.plain_text(),
            // Passo 156H: Boxed (Box) — análogo a Block.
            Self::Boxed { body, .. } => body.plain_text(),
            // Passo 156I: Stack concatena plain_text de children
            // (análogo a Sequence; preserva ordem).
            Self::Stack { children, .. } => children.iter().map(|c| c.plain_text()).collect(),
            // Passo 156J: Repeat é transparente para texto plano —
            // recurse no body sem multiplicar (paridade não visível em
            // texto plano; semântica de repetição é runtime-only).
            Self::Repeat { body, .. } => body.plain_text(),
            Self::Quote { body, attribution, quotes, .. } => {
                let body_txt = body.plain_text();
                let with_quotes = if *quotes {
                    format!("\"{}\"", body_txt)
                } else {
                    body_txt
                };
                match attribution {
                    Some(a) => format!("{} — {}", with_quotes, a.plain_text()),
                    None    => with_quotes,
                }
            }
        }
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty,                Self::Empty)                => true,
            (Self::Text(a, sa),          Self::Text(b, sb))          => a == b && sa == sb,
            (Self::Space,                Self::Space)                => true,
            (Self::Sequence(a),          Self::Sequence(b))          => a.as_ref() == b.as_ref(),
            // Passo 101: Content::Strong/Emph removidos — Content::Styled cobre.
            (Self::Heading { level: la, body: ba }, Self::Heading { level: lb, body: bb }) => la == lb && ba == bb,
            (Self::Raw { text: ta, lang: la, block: ba },
             Self::Raw { text: tb, lang: lb, block: bb })            => ta == tb && la == lb && ba == bb,
            (Self::ListItem(a),          Self::ListItem(b))          => a == b,
            (Self::EnumItem { number: na, body: ba },
             Self::EnumItem { number: nb, body: bb })                => na == nb && ba == bb,
            (Self::Link { url: ua, body: ba },
             Self::Link { url: ub, body: bb })                       => ua == ub && ba == bb,
            (Self::Equation { body: ba, block: ka },
             Self::Equation { body: bb, block: kb })                 => ba == bb && ka == kb,
            (Self::MathSequence(a), Self::MathSequence(b))           => a.as_ref() == b.as_ref(),
            (Self::MathIdent(a),    Self::MathIdent(b))              => a == b,
            (Self::MathText(a),     Self::MathText(b))               => a == b,
            (Self::MathFrac { num: na, den: da },
             Self::MathFrac { num: nb, den: db })                    => na == nb && da == db,
            (Self::MathAttach { base: ba, tl: tla, bl: bla, sub: sa, sup: pa },
             Self::MathAttach { base: bb, tl: tlb, bl: blb, sub: sb, sup: pb })
                => ba == bb && tla == tlb && bla == blb && sa == sb && pa == pb,
            (Self::MathRoot { index: ia, radicand: ra },
             Self::MathRoot { index: ib, radicand: rb })             => ia == ib && ra == rb,
            (Self::MathDelimited { open: oa, body: ba, close: ca },
             Self::MathDelimited { open: ob, body: bb, close: cb })  => oa == ob && ba == bb && ca == cb,
            (Self::MathAlignPoint, Self::MathAlignPoint)             => true,
            (Self::Linebreak,      Self::Linebreak)                  => true,
            (Self::MathMatrix { rows: ra, delim: da },
             Self::MathMatrix { rows: rb, delim: db })               => ra == rb && da == db,
            (Self::MathCases { rows: ra },
             Self::MathCases { rows: rb })                           => ra == rb,
            (Self::Labelled { target: ta, label: la },
             Self::Labelled { target: tb, label: lb })               => ta == tb && la == lb,
            (Self::Ref { target: ta }, Self::Ref { target: tb })     => ta == tb,
            (Self::SetHeadingNumbering { active: a }, Self::SetHeadingNumbering { active: b }) => a == b,
            (Self::CounterDisplay { kind: a }, Self::CounterDisplay { kind: b }) => a == b,
            (Self::CounterUpdate { key: ka, action: aa }, Self::CounterUpdate { key: kb, action: ab }) => ka == kb && aa == ab,
            (Self::Outline, Self::Outline) => true,
            (Self::Figure { body: ba, caption: ca, kind: ka, numbering: na },
             Self::Figure { body: bb, caption: cb, kind: kb, numbering: nb }) =>
                ba == bb && ca == cb && ka == kb && na == nb,
            (Self::SetFigureNumbering { pattern: a }, Self::SetFigureNumbering { pattern: b }) => a == b,
            (Self::Image { path: pa, data: da, width: wa, height: ha },
             Self::Image { path: pb, data: db, width: wb, height: hb }) =>
                pa == pb && da == db && wa.as_deref() == wb.as_deref() && ha.as_deref() == hb.as_deref(),
            (Self::Shape { kind: ka, width: wa, height: ha, fill: fa, stroke: sa },
             Self::Shape { kind: kb, width: wb, height: hb, fill: fb, stroke: sb }) =>
                ka == kb && wa.as_deref() == wb.as_deref() && ha.as_deref() == hb.as_deref()
                    && fa == fb && sa == sb,
            (Self::Transform { matrix: ma, body: ba }, Self::Transform { matrix: mb, body: bb }) =>
                ma == mb && ba == bb,
            (Self::Grid { columns: ca, rows: ra, cells: xa },
             Self::Grid { columns: cb, rows: rb, cells: xb }) =>
                ca == cb && ra == rb && xa == xb,
            // Passo 157A — Table.
            (Self::Table { columns: ca, rows: ra, children: xa },
             Self::Table { columns: cb, rows: rb, children: xb }) =>
                ca == cb && ra == rb && xa == xb,
            // Passo 157B — TableCell.
            (Self::TableCell { body: ba, x: xa, y: ya, colspan: csa, rowspan: rsa },
             Self::TableCell { body: bb, x: xb, y: yb, colspan: csb, rowspan: rsb }) =>
                ba == bb && xa == xb && ya == yb && csa == csb && rsa == rsb,
            // Passo 157C — par simétrico TableHeader/TableFooter.
            (Self::TableHeader { body: ba, repeat: ra },
             Self::TableHeader { body: bb, repeat: rb }) =>
                ba == bb && ra == rb,
            (Self::TableFooter { body: ba, repeat: ra },
             Self::TableFooter { body: bb, repeat: rb }) =>
                ba == bb && ra == rb,
            // Passo 159A — par acoplado Bibliography + Cite.
            (Self::Bibliography { entries: ea, title: ta },
             Self::Bibliography { entries: eb, title: tb }) =>
                ea == eb && ta == tb,
            (Self::Cite { key: ka, supplement: sa },
             Self::Cite { key: kb, supplement: sb }) =>
                ka == kb && sa == sb,
            (Self::SetPage { width: wa, height: ha, margin: ma },
             Self::SetPage { width: wb, height: hb, margin: mb }) =>
                wa == wb && ha == hb && ma == mb,
            (Self::Align { alignment: aa, body: ba },
             Self::Align { alignment: ab, body: bb }) => aa == ab && ba == bb,
            (Self::Place { alignment: aa, dx: dxa, dy: dya, scope: sa, body: ba },
             Self::Place { alignment: ab, dx: dxb, dy: dyb, scope: sb, body: bb }) =>
                aa == ab && dxa == dxb && dya == dyb && sa == sb && ba == bb,
            (Self::Styled(ba, sa), Self::Styled(bb, sb)) => ba == bb && sa == sb,
            // Passo 154B — terms + divider.
            (Self::Divider, Self::Divider) => true,
            (Self::Terms { items: a },     Self::Terms { items: b })     => a == b,
            (Self::TermItem { term: ta, description: da },
             Self::TermItem { term: tb, description: db })               => ta == tb && da == db,
            // Passo 155 — Quote.
            (Self::Quote { body: ba, attribution: aa, block: ka, quotes: qa },
             Self::Quote { body: bb, attribution: ab, block: kb, quotes: qb }) =>
                ba == bb && aa == ab && ka == kb && qa == qb,
            // Passo 156C / 156L — Pad / Hide.
            (Self::Pad  { body: ba, sides: sa },
             Self::Pad  { body: bb, sides: sb }) => ba == bb && sa == sb,
            (Self::Hide { body: ba }, Self::Hide { body: bb }) => ba == bb,
            // Passo 156D — HSpace / VSpace.
            (Self::HSpace { amount: aa, weak: wa },
             Self::HSpace { amount: ab, weak: wb }) => aa == ab && wa == wb,
            (Self::VSpace { amount: aa, weak: wa },
             Self::VSpace { amount: ab, weak: wb }) => aa == ab && wa == wb,
            // Passo 156E — Pagebreak.
            (Self::Pagebreak { weak: wa, to: ta },
             Self::Pagebreak { weak: wb, to: tb }) => wa == wb && ta == tb,
            // Passo 156G — Block.
            (Self::Block { body: ba, width: wa, height: ha, inset: ia, breakable: ka },
             Self::Block { body: bb, width: wb, height: hb, inset: ib, breakable: kb }) =>
                ba == bb && wa == wb && ha == hb && ia == ib && ka == kb,
            // Passo 156H — Boxed.
            (Self::Boxed { body: ba, width: wa, height: ha, inset: ia, baseline: ka },
             Self::Boxed { body: bb, width: wb, height: hb, inset: ib, baseline: kb }) =>
                ba == bb && wa == wb && ha == hb && ia == ib && ka == kb,
            // Passo 156I — Stack (Arc<[Content]> compara por conteúdo).
            (Self::Stack { children: ca, dir: da, spacing: sa },
             Self::Stack { children: cb, dir: db, spacing: sb }) =>
                ca.as_ref() == cb.as_ref() && da == db && sa == sb,
            // Passo 156J — Repeat.
            (Self::Repeat { body: ba, gap: ga, justify: ja },
             Self::Repeat { body: bb, gap: gb, justify: jb }) =>
                ba == bb && ga == gb && ja == jb,
            _ => false,
        }
    }
}

impl Content {
    /// Acesso a campos de elementos estruturados — usado pelas show rules (Passo 68).
    ///
    /// Ex: `it.body` onde `it` é um `Content::Heading` retorna `Some(Value::Content(body))`.
    /// Retorna `None` para campos inexistentes ou variantes sem campos nomeados.
    pub fn get_field(&self, field: &str) -> Option<crate::entities::value::Value> {
        use crate::entities::value::Value;
        match (self, field) {
            (Content::Heading { body, .. },  "body")  => Some(Value::Content(*body.clone())),
            (Content::Heading { level, .. }, "level") => Some(Value::Int(*level as i64)),
            (Content::Figure  { body, .. },  "body")  => Some(Value::Content(*body.clone())),
            _ => None,
        }
    }

    /// Percorre a árvore bottom-up, aplicando `transform` a cada nó após processar os filhos.
    ///
    /// `transform` retorna `Some(new)` → substituir (sem reentrada no novo nó).
    /// `transform` retorna `None` → manter o nó processado (com filhos já transformados).
    ///
    /// O `match` lista explicitamente todos os containers e terminais — sem `_ =>`.
    /// Containers com `Box<Content>` ou `Vec<Content>` recursam; terminais clonam directamente.
    pub fn map_content<F>(&self, transform: &mut F) -> crate::entities::source_result::SourceResult<Self>
    where
        F: FnMut(&Content) -> crate::entities::source_result::SourceResult<Option<Content>>,
    {
        // Passo 1: processar os filhos (bottom-up) para obter o nó com filhos transformados.
        let processed = match self {
            // ── Containers: propagar recursivamente ─────────────────────────
            Content::Sequence(seq) => {
                let new_seq: crate::entities::source_result::SourceResult<Vec<Content>> =
                    seq.iter().map(|c| c.map_content(transform)).collect();
                Content::Sequence(Arc::from(new_seq?))
            },
            // Passo 101: Content::Strong/Emph removidos — cobertos pelo
            // arm Content::Styled abaixo (que já propaga transform recursivamente).
            Content::Heading { level, body } => Content::Heading {
                level: *level,
                body:  Box::new(body.map_content(transform)?),
            },
            Content::ListItem(body) => Content::ListItem(Box::new(body.map_content(transform)?)),
            Content::EnumItem { number, body } => Content::EnumItem {
                number: *number,
                body:   Box::new(body.map_content(transform)?),
            },
            Content::Link { url, body } => Content::Link {
                url:  url.clone(),
                body: Box::new(body.map_content(transform)?),
            },
            Content::Labelled { target, label } => Content::Labelled {
                target: Box::new(target.map_content(transform)?),
                label:  label.clone(),
            },
            Content::Figure { body, caption, kind, numbering } => Content::Figure {
                body:      Box::new(body.map_content(transform)?),
                caption:   caption.as_ref()
                    .map(|c| c.map_content(transform))
                    .transpose()?
                    .map(Box::new),
                kind:      kind.clone(),
                numbering: numbering.clone(),
            },
            // Content::Equation tem body: Box<Content> → container.
            Content::Equation { body, block } => Content::Equation {
                body:  Box::new(body.map_content(transform)?),
                block: *block,
            },
            Content::MathSequence(seq) => {
                let new_seq: crate::entities::source_result::SourceResult<Vec<Content>> =
                    seq.iter().map(|c| c.map_content(transform)).collect();
                Content::MathSequence(Arc::from(new_seq?))
            },
            Content::MathFrac { num, den } => Content::MathFrac {
                num: Box::new(num.map_content(transform)?),
                den: Box::new(den.map_content(transform)?),
            },
            Content::MathAttach { base, tl, bl, sub, sup } => Content::MathAttach {
                base: Box::new(base.map_content(transform)?),
                tl:   tl.as_ref().map(|c| c.map_content(transform)).transpose()?.map(Box::new),
                bl:   bl.as_ref().map(|c| c.map_content(transform)).transpose()?.map(Box::new),
                sub:  sub.as_ref().map(|c| c.map_content(transform)).transpose()?.map(Box::new),
                sup:  sup.as_ref().map(|c| c.map_content(transform)).transpose()?.map(Box::new),
            },
            Content::MathRoot { index, radicand } => Content::MathRoot {
                index:    index.as_ref().map(|c| c.map_content(transform)).transpose()?.map(Box::new),
                radicand: Box::new(radicand.map_content(transform)?),
            },
            Content::MathDelimited { open, body, close } => Content::MathDelimited {
                open:  *open,
                body:  Box::new(body.map_content(transform)?),
                close: *close,
            },
            Content::MathMatrix { rows, delim } => {
                let new_rows: crate::entities::source_result::SourceResult<Vec<Vec<Content>>> =
                    rows.iter()
                        .map(|row| row.iter().map(|c| c.map_content(transform)).collect())
                        .collect();
                Content::MathMatrix { rows: new_rows?, delim: *delim }
            },
            Content::MathCases { rows } => {
                let new_rows: crate::entities::source_result::SourceResult<Vec<Vec<Content>>> =
                    rows.iter()
                        .map(|row| row.iter().map(|c| c.map_content(transform)).collect())
                        .collect();
                Content::MathCases { rows: new_rows? }
            },

            // Passo 154B: Terms recurse em items; TermItem recurse em par.
            Content::Terms { items } => {
                let new_items: crate::entities::source_result::SourceResult<Vec<Content>> =
                    items.iter().map(|c| c.map_content(transform)).collect();
                Content::Terms { items: new_items? }
            },
            Content::TermItem { term, description } => Content::TermItem {
                term:        Box::new(term.map_content(transform)?),
                description: Box::new(description.map_content(transform)?),
            },

            // Passo 155: Quote container — recurse em body e attribution;
            // block e quotes são primitivos.
            Content::Quote { body, attribution, block, quotes } => Content::Quote {
                body:        Box::new(body.map_content(transform)?),
                attribution: attribution.as_ref()
                    .map(|c| c.map_content(transform))
                    .transpose()?
                    .map(Box::new),
                block:       *block,
                quotes:      *quotes,
            },

            // Passo 156C / 156L: Pad / Hide containers — recurse em body;
            // sides é Copy primitivo (Sides<Option<Length>>).
            Content::Pad { body, sides } => Content::Pad {
                body:  Box::new(body.map_content(transform)?),
                sides: *sides,
            },
            Content::Hide { body } => Content::Hide {
                body: Box::new(body.map_content(transform)?),
            },

            // Passo 156G: Block container — recurse em body; atributos
            // são Copy primitivos (Option<Length>, Sides<Length>, bool).
            Content::Block { body, width, height, inset, breakable } => Content::Block {
                body:      Box::new(body.map_content(transform)?),
                width:     *width,
                height:    *height,
                inset:     *inset,
                breakable: *breakable,
            },

            // Passo 156H: Boxed (Box inline) — recurse análogo a Block.
            Content::Boxed { body, width, height, inset, baseline } => Content::Boxed {
                body:     Box::new(body.map_content(transform)?),
                width:    *width,
                height:   *height,
                inset:    *inset,
                baseline: *baseline,
            },

            // Passo 156I: Stack compositivo — mapear cada child;
            // preservar dir/spacing (Copy primitivos).
            Content::Stack { children, dir, spacing } => {
                let new_children: crate::entities::source_result::SourceResult<Vec<Content>> =
                    children.iter().map(|c| c.map_content(transform)).collect();
                Content::Stack {
                    children: Arc::from(new_children?),
                    dir:      *dir,
                    spacing:  *spacing,
                }
            },

            // Passo 156J: Repeat container — recurse em body; gap e
            // justify são Copy primitivos (Option<Length>, bool).
            Content::Repeat { body, gap, justify } => Content::Repeat {
                body:    Box::new(body.map_content(transform)?),
                gap:     *gap,
                justify: *justify,
            },

            // ── Terminais: clonar directamente ──────────────────────────────
            // Listados explicitamente — variantes novas não passam em silêncio.
            // Passo 156D: HSpace/VSpace são leaves (sem body), terminais.
            // Passo 156E: Pagebreak é leaf (event sem body), terminal.
            Content::Text(_, _)
            | Content::Space
            | Content::Empty
            | Content::Linebreak
            | Content::Outline
            | Content::Raw { .. }
            | Content::Ref { .. }
            | Content::SetHeadingNumbering { .. }
            | Content::SetFigureNumbering { .. }
            | Content::SetPage { .. }
            | Content::CounterUpdate { .. }
            | Content::CounterDisplay { .. }
            | Content::MathAlignPoint
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::Image { .. }
            | Content::Divider
            | Content::HSpace { .. }
            | Content::VSpace { .. }
            | Content::Pagebreak { .. }
            | Content::Shape { .. } => self.clone(),
            Content::Transform { matrix, body } => Content::Transform {
                matrix: *matrix,
                body:   Box::new(body.map_content(transform)?),
            },
            Content::Grid { columns, rows, cells } => {
                let new_cells: crate::entities::source_result::SourceResult<Vec<Content>> =
                    cells.iter().map(|c| c.map_content(transform)).collect();
                Content::Grid { columns: columns.clone(), rows: rows.clone(), cells: new_cells? }
            },
            // Passo 157A: Table — mapear children (paridade Grid).
            Content::Table { columns, rows, children } => {
                let new_children: crate::entities::source_result::SourceResult<Vec<Content>> =
                    children.iter().map(|c| c.map_content(transform)).collect();
                Content::Table { columns: columns.clone(), rows: rows.clone(), children: new_children? }
            },
            // Passo 157B: TableCell — recurse no body; preserva fields
            // x/y/colspan/rowspan (Copy primitivos Option<usize>).
            Content::TableCell { body, x, y, colspan, rowspan } => Content::TableCell {
                body:    Box::new(body.map_content(transform)?),
                x:       *x,
                y:       *y,
                colspan: *colspan,
                rowspan: *rowspan,
            },
            // Passo 157C: par simétrico TableHeader/TableFooter —
            // recurse no body; preserva repeat (Copy bool).
            Content::TableHeader { body, repeat } => Content::TableHeader {
                body:   Box::new(body.map_content(transform)?),
                repeat: *repeat,
            },
            Content::TableFooter { body, repeat } => Content::TableFooter {
                body:   Box::new(body.map_content(transform)?),
                repeat: *repeat,
            },
            // Passo 159A: Bibliography recurse em title; preserva
            // entries (BibEntry é dados puros, sem Content recursivo).
            // Cite recurse em supplement; preserva key.
            Content::Bibliography { entries, title } => Content::Bibliography {
                entries: entries.clone(),
                title:   title.as_ref()
                    .map(|t| t.map_content(transform))
                    .transpose()?
                    .map(Box::new),
            },
            Content::Cite { key, supplement } => Content::Cite {
                key:        key.clone(),
                supplement: supplement.as_ref()
                    .map(|s| s.map_content(transform))
                    .transpose()?
                    .map(Box::new),
            },
            Content::Align { alignment, body } => Content::Align {
                alignment: *alignment,
                body:      Box::new(body.map_content(transform)?),
            },
            Content::Place { alignment, dx, dy, scope, body } => Content::Place {
                alignment: *alignment,
                dx:        *dx,
                dy:        *dy,
                scope:     *scope,
                body:      Box::new(body.map_content(transform)?),
            },
            Content::Styled(body, styles) => Content::Styled(
                Box::new(body.map_content(transform)?),
                styles.clone(),
            ),
        };

        // Passo 2: aplicar a transformação ao nó já processado.
        match transform(&processed)? {
            Some(new_content) => Ok(new_content),
            None              => Ok(processed),
        }
    }

    /// Aplica uma função de transformação a todos os nós `Content::Text`,
    /// preservando a estrutura da árvore (Passo 67).
    ///
    /// O uso de `&mut F` permite que a closure carregue estado entre chamadas
    /// (ex: um contador de substituições restantes), o que é necessário para
    /// que `replace(count: N)` funcione correctamente através de múltiplos nós.
    pub fn map_text<F>(&self, transform: &mut F) -> Self
    where
        F: FnMut(&str) -> String,
    {
        match self {
            // O caso alvo: aplicar a transformação preservando o estilo.
            Content::Text(s, style) => Content::Text(transform(s.as_str()).into(), style.clone()),

            // ── Containers com filhos (propagação recursiva) ──────────────
            // Cada variante listada explicitamente — sem `_ =>` ou `other =>`.
            Content::Sequence(seq) => {
                Content::Sequence(
                    seq.iter().map(|c| c.map_text(transform)).collect::<Vec<_>>().into()
                )
            }
            Content::Heading { level, body } => Content::Heading {
                level: *level,
                body:  Box::new(body.map_text(transform)),
            },
            // Passo 101: Content::Strong/Emph removidos — cobertos pelo
            // arm Content::Styled abaixo (map_text recursivo).
            Content::Labelled { target, label } => Content::Labelled {
                target: Box::new(target.map_text(transform)),
                label:  label.clone(),
            },
            Content::Figure { body, caption, kind, numbering } => Content::Figure {
                body:      Box::new(body.map_text(transform)),
                caption:   caption.as_ref().map(|c| Box::new(c.map_text(transform))),
                kind:      kind.clone(),
                numbering: numbering.clone(),
            },
            Content::ListItem(body) => Content::ListItem(Box::new(body.map_text(transform))),
            Content::EnumItem { number, body } => Content::EnumItem {
                number: *number,
                body:   Box::new(body.map_text(transform)),
            },
            Content::Link { url, body } => Content::Link {
                url:  url.clone(),
                body: Box::new(body.map_text(transform)),
            },

            // Passo 154B: Terms recurse em items; TermItem recurse em par.
            Content::Terms { items } => Content::Terms {
                items: items.iter().map(|c| c.map_text(transform)).collect(),
            },
            Content::TermItem { term, description } => Content::TermItem {
                term:        Box::new(term.map_text(transform)),
                description: Box::new(description.map_text(transform)),
            },

            // Passo 155: Quote container — recurse em body e attribution.
            Content::Quote { body, attribution, block, quotes } => Content::Quote {
                body:        Box::new(body.map_text(transform)),
                attribution: attribution.as_ref().map(|c| Box::new(c.map_text(transform))),
                block:       *block,
                quotes:      *quotes,
            },

            // Passo 156C / 156L: Pad / Hide containers — recurse em body.
            Content::Pad { body, sides } => Content::Pad {
                body:  Box::new(body.map_text(transform)),
                sides: *sides,
            },
            Content::Hide { body } => Content::Hide {
                body: Box::new(body.map_text(transform)),
            },

            // Passo 156G: Block container — recurse em body.
            Content::Block { body, width, height, inset, breakable } => Content::Block {
                body:      Box::new(body.map_text(transform)),
                width:     *width,
                height:    *height,
                inset:     *inset,
                breakable: *breakable,
            },

            // Passo 156H: Boxed (Box inline) — recurse análogo a Block.
            Content::Boxed { body, width, height, inset, baseline } => Content::Boxed {
                body:     Box::new(body.map_text(transform)),
                width:    *width,
                height:   *height,
                inset:    *inset,
                baseline: *baseline,
            },

            // Passo 156I: Stack compositivo — map_text em cada child.
            Content::Stack { children, dir, spacing } => Content::Stack {
                children: children.iter().map(|c| c.map_text(transform)).collect::<Vec<_>>().into(),
                dir:      *dir,
                spacing:  *spacing,
            },

            // Passo 156J: Repeat container — map_text no body.
            Content::Repeat { body, gap, justify } => Content::Repeat {
                body:    Box::new(body.map_text(transform)),
                gap:     *gap,
                justify: *justify,
            },

            // ── Terminais — clonar directamente ──────────────────────────
            // Nós matemáticos e estruturais sem markup Text — não contêm
            // Content::Text, portanto clonar em bloco é correcto e seguro.
            // Passo 156D: HSpace/VSpace são leaves (sem body), terminais.
            // Passo 156E: Pagebreak é leaf (event sem body), terminal.
            Content::Empty
            | Content::Space
            | Content::Linebreak
            | Content::Outline
            | Content::Raw { .. }
            | Content::Ref { .. }
            | Content::SetHeadingNumbering { .. }
            | Content::SetFigureNumbering { .. }
            | Content::SetPage { .. }
            | Content::CounterUpdate { .. }
            | Content::CounterDisplay { .. }
            | Content::MathAlignPoint
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::Equation { .. }
            | Content::MathSequence(_)
            | Content::MathFrac { .. }
            | Content::MathAttach { .. }
            | Content::MathRoot { .. }
            | Content::MathDelimited { .. }
            | Content::MathMatrix { .. }
            | Content::MathCases { .. }
            | Content::Image { .. }
            | Content::Divider
            | Content::HSpace { .. }
            | Content::VSpace { .. }
            | Content::Pagebreak { .. }
            | Content::Shape { .. } => self.clone(),
            Content::Transform { matrix, body } => Content::Transform {
                matrix: *matrix,
                body:   Box::new(body.map_text(transform)),
            },
            Content::Grid { columns, rows, cells } => Content::Grid {
                columns: columns.clone(),
                rows:    rows.clone(),
                cells:   cells.iter().map(|c| c.map_text(transform)).collect(),
            },
            // Passo 157A: Table — map_text em children (paridade Grid).
            Content::Table { columns, rows, children } => Content::Table {
                columns:  columns.clone(),
                rows:     rows.clone(),
                children: children.iter().map(|c| c.map_text(transform)).collect(),
            },
            // Passo 157B: TableCell — map_text no body; preserva fields.
            Content::TableCell { body, x, y, colspan, rowspan } => Content::TableCell {
                body:    Box::new(body.map_text(transform)),
                x:       *x,
                y:       *y,
                colspan: *colspan,
                rowspan: *rowspan,
            },
            // Passo 157C: par simétrico — map_text no body.
            Content::TableHeader { body, repeat } => Content::TableHeader {
                body:   Box::new(body.map_text(transform)),
                repeat: *repeat,
            },
            Content::TableFooter { body, repeat } => Content::TableFooter {
                body:   Box::new(body.map_text(transform)),
                repeat: *repeat,
            },
            // Passo 159A: Bibliography map_text em title; entries
            // são dados puros (String fields) — sem map_text recursivo
            // em entries (mapeamento de strings em fields entities é
            // out of scope per ADR-0033 paridade observable).
            Content::Bibliography { entries, title } => Content::Bibliography {
                entries: entries.clone(),
                title:   title.as_ref().map(|t| Box::new(t.map_text(transform))),
            },
            // Cite map_text em supplement; preserva key (String).
            Content::Cite { key, supplement } => Content::Cite {
                key:        key.clone(),
                supplement: supplement.as_ref().map(|s| Box::new(s.map_text(transform))),
            },
            Content::Align { alignment, body } => Content::Align {
                alignment: *alignment,
                body:      Box::new(body.map_text(transform)),
            },
            Content::Place { alignment, dx, dy, scope, body } => Content::Place {
                alignment: *alignment,
                dx:        *dx,
                dy:        *dy,
                scope:     *scope,
                body:      Box::new(body.map_text(transform)),
            },
            Content::Styled(body, styles) => Content::Styled(
                Box::new(body.map_text(transform)),
                styles.clone(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_plain_text() {
        assert_eq!(Content::text("hello").plain_text(), "hello");
        assert_eq!(Content::text("").plain_text(), "");
    }

    #[test]
    fn empty_is_empty() {
        assert!(Content::empty().is_empty());
        assert_eq!(Content::empty().plain_text(), "");
    }

    #[test]
    fn space_nao_e_empty() {
        assert!(!Content::Space.is_empty());
        assert_eq!(Content::Space.plain_text(), " ");
    }

    #[test]
    fn sequence_zero_partes_e_empty() {
        let c = Content::sequence(vec![]);
        assert!(c.is_empty());
        assert_eq!(c, Content::Empty);
    }

    #[test]
    fn sequence_uma_parte_desembrulha() {
        let c = Content::sequence(vec![Content::text("a")]);
        assert_eq!(c, Content::text("a"));
    }

    #[test]
    fn sequence_multiplas_partes() {
        let c = Content::sequence(vec![
            Content::text("a"),
            Content::Space,
            Content::text("b"),
        ]);
        assert_eq!(c.plain_text(), "a b");
        assert!(!c.is_empty());
    }

    #[test]
    fn sequence_is_empty_para_vec_vazio() {
        let c = Content::Sequence(Arc::from(Vec::<Content>::new().into_boxed_slice()));
        assert!(c.is_empty());
    }

    #[test]
    fn clone_e_partial_eq() {
        let c1 = Content::text("hello");
        let c2 = c1.clone();
        assert_eq!(c1, c2);
        assert_ne!(Content::text("a"), Content::text("b"));
        assert_ne!(Content::text("a"), Content::Space);
    }

    #[test]
    fn strong_plain_text_preservado() {
        assert_eq!(Content::strong(Content::text("bold")).plain_text(), "bold");
    }

    #[test]
    fn emph_plain_text_preservado() {
        assert_eq!(Content::emph(Content::text("em")).plain_text(), "em");
    }

    #[test]
    fn heading_level_clamped() {
        assert!(matches!(Content::heading(0, Content::Empty), Content::Heading { level: 1, .. }));
        assert!(matches!(Content::heading(9, Content::Empty), Content::Heading { level: 6, .. }));
        assert!(matches!(Content::heading(3, Content::Empty), Content::Heading { level: 3, .. }));
    }

    #[test]
    fn heading_plain_text() {
        let h = Content::heading(1, Content::text("Title"));
        assert_eq!(h.plain_text(), "Title");
    }

    #[test]
    fn nested_sequence_plain_text() {
        let inner = Content::sequence(vec![Content::text("x"), Content::text("y")]);
        let outer = Content::sequence(vec![inner, Content::Space, Content::text("z")]);
        assert_eq!(outer.plain_text(), "xy z");
    }

    // ── Passo 23 ────────────────────────────────────────────────────────────

    #[test]
    fn raw_plain_text() {
        assert_eq!(Content::raw("fn main() {}", None, false).plain_text(), "fn main() {}");
    }

    #[test]
    fn list_item_tem_bullet_em_plain_text() {
        assert!(Content::list_item(Content::text("Apple")).plain_text().contains("Apple"));
    }

    #[test]
    fn enum_item_com_numero() {
        let t = Content::enum_item(Some(1), Content::text("First")).plain_text();
        assert!(t.contains("1") && t.contains("First"));
    }

    #[test]
    fn link_plain_text_e_o_corpo() {
        assert_eq!(
            Content::link("https://typst.app", Content::text("Typst")).plain_text(),
            "Typst",
        );
    }

    // ── Passo 34 — variantes matemáticas ─────────────────────────────────────

    #[test]
    fn content_equation_inline_plain_text() {
        let eq = Content::Equation {
            body:  Box::new(Content::MathIdent("x".into())),
            block: false,
        };
        assert_eq!(eq.plain_text(), "x");
    }

    #[test]
    fn content_equation_block_plain_text() {
        let eq = Content::Equation {
            body:  Box::new(Content::MathIdent("x".into())),
            block: true,
        };
        assert_eq!(eq.plain_text(), "\nx\n");
    }

    #[test]
    fn content_math_frac_plain_text() {
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        assert_eq!(frac.plain_text(), "(a)/(b)");
    }

    #[test]
    fn content_math_attach_plain_text() {
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   None,
            bl:   None,
            sub:  None,
            sup:  Some(Box::new(Content::MathText("2".into()))),
        };
        assert_eq!(attach.plain_text(), "x^2");
    }

    #[test]
    fn content_math_root_quadrada() {
        let root = Content::MathRoot {
            index:    None,
            radicand: Box::new(Content::MathIdent("x".into())),
        };
        assert_eq!(root.plain_text(), "sqrt(x)");
    }

    #[test]
    fn content_math_root_cubica() {
        let root = Content::MathRoot {
            index:    Some(Box::new(Content::MathText("3".into()))),
            radicand: Box::new(Content::MathIdent("x".into())),
        };
        assert_eq!(root.plain_text(), "root(3, x)");
    }

    #[test]
    fn content_math_sequence_plain_text() {
        let seq = Content::MathSequence(Arc::from(vec![
            Content::MathIdent("x".into()),
            Content::MathText("+".into()),
            Content::MathIdent("y".into()),
        ].into_boxed_slice()));
        assert_eq!(seq.plain_text(), "x+y");
    }

    #[test]
    fn content_math_partialeq() {
        let a = Content::MathIdent("x".into());
        let b = Content::MathIdent("x".into());
        let c = Content::MathIdent("y".into());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // ── Passo 26 — Content::Sequence com Arc (ADR-0026 revisão) ─────────────

    #[test]
    fn sequence_clone_e_o1() {
        let seq = Content::sequence(vec![
            Content::text("a"),
            Content::text("b"),
            Content::text("c"),
        ]);
        let clone = seq.clone();
        // PartialEq por conteúdo — não por ponteiro
        assert_eq!(seq, clone);
    }

    #[test]
    fn sequence_partialeq_por_conteudo() {
        let s1 = Content::sequence(vec![Content::text("hello")]);
        let s2 = Content::sequence(vec![Content::text("hello")]);
        // Dois Arc distintos com mesmo conteúdo → iguais
        assert_eq!(s1, s2);
    }

    #[test]
    fn sequence_partialeq_conteudos_diferentes() {
        let s1 = Content::sequence(vec![Content::text("a")]);
        let s2 = Content::sequence(vec![Content::text("b")]);
        assert_ne!(s1, s2);
    }

    // ── Passo 67 — map_text ───────────────────────────────────────────────────

    #[test]
    fn map_text_transforma_texto_simples() {
        let content = Content::text("hello");
        let result = content.map_text(&mut |s| s.to_uppercase());
        assert_eq!(result, Content::text("HELLO"));
    }

    #[test]
    fn map_text_desce_em_strong() {
        // Passo 101: `Content::strong(..)` produz `Content::Styled(.., [Bold])`.
        let content = Content::strong(Content::text("hello"));
        let result = content.map_text(&mut |s| s.to_uppercase());
        assert_eq!(result, Content::strong(Content::text("HELLO")));
    }

    #[test]
    fn map_text_preserva_terminais_sem_texto() {
        let content = Content::Space;
        let result = content.map_text(&mut |s| s.to_uppercase());
        assert_eq!(result, Content::Space);
    }

    #[test]
    fn map_text_closure_com_estado_entre_nos() {
        // Validar que o estado da closure (FnMut) persiste entre nós distintos.
        let content = Content::Sequence(vec![
            Content::text("a"),
            Content::strong(Content::text("a")),
            Content::text("a"),
        ].into());
        let mut count = 0usize;
        content.map_text(&mut |s| {
            count += 1;
            s.to_string()
        });
        assert_eq!(count, 3, "A closure deve ser chamada uma vez por nó Text");
    }

    // ── map_content (Passo 69 — DEBT-19) ─────────────────────────────────────

    #[test]
    fn map_content_substitui_heading_em_sequence() {
        let content = Content::Sequence(Arc::from(vec![
            Content::text("Antes"),
            Content::heading(1, Content::text("Titulo")),
            Content::text("Depois"),
        ]));

        let result = content.map_content(&mut |node| {
            if matches!(node, Content::Heading { .. }) {
                Ok(Some(Content::text("SUBSTITUIDO")))
            } else {
                Ok(None)
            }
        }).unwrap();

        assert_eq!(result.plain_text(), "AntesSUBSTITUIDODepois");
    }

    #[test]
    fn map_content_bottom_up_pai_ve_filhos_transformados() {
        // Passo 101: `Content::strong` passou a `Content::Styled([Bold], body)`.
        let content = Content::strong(Content::text("original"));

        let result = content.map_content(&mut |node| {
            match node {
                Content::Text(s, _) => Ok(Some(Content::text(s.to_uppercase()))),
                Content::Styled(body, _) => {
                    let text = body.plain_text();
                    assert_eq!(text, "ORIGINAL",
                        "Styled([Bold]) deve receber filho já transformado: {:?}", text);
                    Ok(None)
                },
                _ => Ok(None),
            }
        }).unwrap();

        assert_eq!(result.plain_text(), "ORIGINAL");
    }

    #[test]
    fn map_content_nao_reavaliar_no_substituido() {
        let content = Content::heading(1, Content::text("X"));
        let mut call_count = 0usize;

        content.map_content(&mut |node| {
            if matches!(node, Content::Heading { .. }) {
                call_count += 1;
                Ok(Some(Content::text("substituido")))
            } else {
                Ok(None)
            }
        }).unwrap();

        assert_eq!(call_count, 1, "Heading deve ser processado exactamente uma vez");
    }

    // ── Passo 99 (ADR-0038): Content::Styled ─────────────────────────────

    use crate::entities::style::{Style, Styles};

    #[test]
    fn styled_plain_text_transparente() {
        let inner = Content::text("hello");
        let styles = Styles::from_iter([Style::Bold(true), Style::Size(Pt(18.0))]);
        let styled = Content::Styled(Box::new(inner), styles);
        assert_eq!(styled.plain_text(), "hello");
    }

    #[test]
    fn styled_partial_eq() {
        let s1 = Content::Styled(
            Box::new(Content::text("x")),
            Styles::from_iter([Style::Bold(true)]),
        );
        let s2 = Content::Styled(
            Box::new(Content::text("x")),
            Styles::from_iter([Style::Bold(true)]),
        );
        let s3 = Content::Styled(
            Box::new(Content::text("x")),
            Styles::from_iter([Style::Bold(false)]),
        );
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    fn styled_preserva_estilos_em_map_text() {
        let inner = Content::text("abc");
        let styles = Styles::from_iter([Style::Italic(true)]);
        let styled = Content::Styled(Box::new(inner), styles.clone());
        let transformed = styled.map_text(&mut |s: &str| s.to_uppercase());
        match transformed {
            Content::Styled(body, st) => {
                assert_eq!(body.plain_text(), "ABC");
                assert_eq!(st, styles);
            }
            other => panic!("esperado Content::Styled, obteve {:?}", other),
        }
    }

    // ── Passo 154B (ADR-0060 Fase 1) — terms + divider ────────────────────

    #[test]
    fn divider_constructor_devolve_variant_correcto() {
        let c = Content::Divider;
        assert!(matches!(c, Content::Divider));
        // Divider é singleton estrutural: nunca empty.
        assert!(!c.is_empty());
    }

    #[test]
    fn divider_plain_text_devolve_vazio() {
        assert_eq!(Content::Divider.plain_text(), "");
    }

    #[test]
    fn terms_constructor_devolve_variant_correcto() {
        let t = Content::Terms {
            items: vec![Content::TermItem {
                term:        Box::new(Content::text("a")),
                description: Box::new(Content::text("b")),
            }],
        };
        assert!(matches!(t, Content::Terms { .. }));
        assert!(!t.is_empty());
        // Terms vazio é considerado empty.
        assert!(Content::Terms { items: vec![] }.is_empty());
    }

    #[test]
    fn terms_plain_text_concatena_pares() {
        let t = Content::Terms {
            items: vec![
                Content::TermItem {
                    term:        Box::new(Content::text("Apple")),
                    description: Box::new(Content::text("fruit")),
                },
                Content::TermItem {
                    term:        Box::new(Content::text("Banana")),
                    description: Box::new(Content::text("yellow")),
                },
            ],
        };
        assert_eq!(t.plain_text(), "Apple: fruit\nBanana: yellow");
    }

    #[test]
    fn term_item_plain_text() {
        let t = Content::TermItem {
            term:        Box::new(Content::text("key")),
            description: Box::new(Content::text("value")),
        };
        assert_eq!(t.plain_text(), "key: value");
    }

    #[test]
    fn terms_map_text_recurse() {
        let t = Content::Terms {
            items: vec![Content::TermItem {
                term:        Box::new(Content::text("apple")),
                description: Box::new(Content::text("fruit")),
            }],
        };
        let upper = t.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "APPLE: FRUIT");
    }

    #[test]
    fn terms_partial_eq() {
        let mk = || Content::Terms {
            items: vec![Content::TermItem {
                term:        Box::new(Content::text("k")),
                description: Box::new(Content::text("v")),
            }],
        };
        assert_eq!(mk(), mk());
        assert_ne!(mk(), Content::Divider);
        assert_eq!(Content::Divider, Content::Divider);
    }

    // ── Passo 155 (ADR-0060 Fase 1, sub-passo 2) — quote ─────────────────

    #[test]
    fn quote_constructor_devolve_variant_correcto() {
        let q = Content::Quote {
            body:        Box::new(Content::text("hello")),
            attribution: None,
            block:       false,
            quotes:      true,
        };
        assert!(matches!(q, Content::Quote { .. }));
        assert!(!q.is_empty());
    }

    #[test]
    fn quote_plain_text_sem_attribution() {
        let q = Content::Quote {
            body:        Box::new(Content::text("hello")),
            attribution: None,
            block:       false,
            quotes:      true,
        };
        assert_eq!(q.plain_text(), "\"hello\"");
    }

    #[test]
    fn quote_plain_text_com_attribution() {
        let q = Content::Quote {
            body:        Box::new(Content::text("Errare humanum est")),
            attribution: Some(Box::new(Content::text("Seneca"))),
            block:       true,
            quotes:      true,
        };
        assert_eq!(q.plain_text(), "\"Errare humanum est\" — Seneca");
    }

    #[test]
    fn quote_plain_text_quotes_false_omite_aspas() {
        let q = Content::Quote {
            body:        Box::new(Content::text("texto")),
            attribution: None,
            block:       false,
            quotes:      false,
        };
        assert_eq!(q.plain_text(), "texto");
    }

    #[test]
    fn quote_is_empty_proxy_para_body() {
        let empty = Content::Quote {
            body:        Box::new(Content::Empty),
            attribution: None,
            block:       false,
            quotes:      true,
        };
        assert!(empty.is_empty());
        let nonempty = Content::Quote {
            body:        Box::new(Content::text("x")),
            attribution: None,
            block:       false,
            quotes:      true,
        };
        assert!(!nonempty.is_empty());
    }

    #[test]
    fn quote_map_text_recurse_em_body_e_attribution() {
        let q = Content::Quote {
            body:        Box::new(Content::text("hello")),
            attribution: Some(Box::new(Content::text("seneca"))),
            block:       true,
            quotes:      true,
        };
        let upper = q.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "\"HELLO\" — SENECA");
    }

    #[test]
    fn quote_partial_eq() {
        let mk = || Content::Quote {
            body:        Box::new(Content::text("x")),
            attribution: None,
            block:       false,
            quotes:      true,
        };
        assert_eq!(mk(), mk());
        let other = Content::Quote {
            body:        Box::new(Content::text("x")),
            attribution: None,
            block:       true,        // diferente
            quotes:      true,
        };
        assert_ne!(mk(), other);
    }

    // ── Passo 156C / 156L (ADR-0061 Fase 1 + Fase 3 refino) — pad + hide ──

    #[test]
    fn pad_constructor_envolve_body() {
        use crate::entities::sides::Sides;
        use crate::entities::layout_types::Length;
        // P156L: cada side é Option<Length>; Some(...) ↔ lado declarado.
        let p = Content::pad(Content::text("x"), Sides::uniform(Some(Length::pt(10.0))));
        if let Content::Pad { body, sides } = &p {
            assert_eq!(body.plain_text(), "x");
            assert_eq!(sides.left,   Some(Length::pt(10.0)));
            assert_eq!(sides.right,  Some(Length::pt(10.0)));
            assert_eq!(sides.top,    Some(Length::pt(10.0)));
            assert_eq!(sides.bottom, Some(Length::pt(10.0)));
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn hide_constructor_envolve_body() {
        let h = Content::hide(Content::text("placeholder"));
        if let Content::Hide { body } = &h {
            assert_eq!(body.plain_text(), "placeholder");
        } else {
            panic!("esperado Content::Hide");
        }
    }

    #[test]
    fn pad_e_hide_is_empty_proxy_para_body() {
        use crate::entities::sides::Sides;
        use crate::entities::layout_types::Length;
        // Pad/Hide com body Empty são considerados vazios.
        let pad_empty  = Content::pad(Content::Empty, Sides::uniform(Some(Length::pt(5.0))));
        let hide_empty = Content::hide(Content::Empty);
        assert!(pad_empty.is_empty());
        assert!(hide_empty.is_empty());
        // Com body com texto, não vazios.
        let pad_text  = Content::pad(Content::text("a"), Sides::uniform(None));
        let hide_text = Content::hide(Content::text("a"));
        assert!(!pad_text.is_empty());
        assert!(!hide_text.is_empty());
    }

    #[test]
    fn pad_plain_text_recurse_no_body() {
        use crate::entities::sides::Sides;
        use crate::entities::layout_types::Length;
        let p = Content::pad(Content::text("hello"), Sides::uniform(Some(Length::pt(2.0))));
        assert_eq!(p.plain_text(), "hello");
    }

    #[test]
    fn hide_plain_text_e_string_vazia() {
        // Hide é layout-aware mas não rende — plain_text vazio.
        let h = Content::hide(Content::text("invisivel"));
        assert_eq!(h.plain_text(), "");
    }

    #[test]
    fn pad_partial_eq() {
        use crate::entities::sides::Sides;
        use crate::entities::layout_types::Length;
        let mk = || Content::pad(Content::text("x"), Sides::uniform(Some(Length::pt(3.0))));
        assert_eq!(mk(), mk());
        // Padding diferente → diferente (bottom 5pt em vez de 3pt).
        let other = Content::pad(
            Content::text("x"),
            Sides::new(Some(Length::pt(3.0)), Some(Length::pt(3.0)),
                       Some(Length::pt(3.0)), Some(Length::pt(5.0))),
        );
        assert_ne!(mk(), other);
        // P156L: distinção semântica nova — Some(zero) ≠ None.
        let some_zero = Content::pad(
            Content::text("x"),
            Sides::uniform(Some(Length::ZERO)),
        );
        let none = Content::pad(
            Content::text("x"),
            Sides::uniform(None),
        );
        assert_ne!(some_zero, none,
            "P156L: Some(zero) e None são semanticamente distintos");
    }

    #[test]
    fn hide_partial_eq() {
        let a = Content::hide(Content::text("x"));
        let b = Content::hide(Content::text("x"));
        let c = Content::hide(Content::text("y"));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn pad_e_hide_map_text_recurse_no_body() {
        use crate::entities::sides::Sides;
        use crate::entities::layout_types::Length;
        let pad  = Content::pad(Content::text("hello"), Sides::uniform(Some(Length::pt(1.0))));
        let hide = Content::hide(Content::text("hello"));
        let pad_upper  = pad.map_text(&mut |s| s.to_uppercase());
        let hide_upper = hide.map_text(&mut |s| s.to_uppercase());
        // Pad expõe via plain_text (recurse); Hide oculta plain_text mas
        // o body interno foi transformado — verificamos isso desembrulhando.
        assert_eq!(pad_upper.plain_text(), "HELLO");
        if let Content::Hide { body } = &hide_upper {
            assert_eq!(body.plain_text(), "HELLO");
        } else {
            panic!("esperado Content::Hide após map_text");
        }
    }

    // ── Passo 156D (ADR-0061 Fase 1, sub-passo 2) — h + v spacing ─────────

    #[test]
    fn hspace_constructor() {
        use crate::entities::layout_types::Length;
        let h = Content::h_space(Length::pt(12.0), false);
        if let Content::HSpace { amount, weak } = h {
            assert_eq!(amount, Length::pt(12.0));
            assert!(!weak);
        } else {
            panic!("esperado Content::HSpace");
        }
    }

    #[test]
    fn vspace_constructor() {
        use crate::entities::layout_types::Length;
        let v = Content::v_space(Length::pt(8.0), true);
        if let Content::VSpace { amount, weak } = v {
            assert_eq!(amount, Length::pt(8.0));
            assert!(weak);
        } else {
            panic!("esperado Content::VSpace");
        }
    }

    #[test]
    fn hspace_e_vspace_is_empty_se_amount_zero() {
        use crate::entities::layout_types::Length;
        // Amount zero → considerado vazio (consistente com Sequence vazia).
        let h_zero = Content::h_space(Length::ZERO, false);
        let v_zero = Content::v_space(Length::ZERO, false);
        assert!(h_zero.is_empty());
        assert!(v_zero.is_empty());
        // Amount não-zero → não vazio.
        let h_nonzero = Content::h_space(Length::pt(1.0), false);
        let v_nonzero = Content::v_space(Length::pt(1.0), false);
        assert!(!h_nonzero.is_empty());
        assert!(!v_nonzero.is_empty());
    }

    #[test]
    fn hspace_e_vspace_plain_text_vazio() {
        use crate::entities::layout_types::Length;
        let h = Content::h_space(Length::pt(5.0), false);
        let v = Content::v_space(Length::pt(5.0), false);
        // Spacing primitives não rendem texto.
        assert_eq!(h.plain_text(), "");
        assert_eq!(v.plain_text(), "");
    }

    #[test]
    fn hspace_partial_eq() {
        use crate::entities::layout_types::Length;
        let a = Content::h_space(Length::pt(3.0), false);
        let b = Content::h_space(Length::pt(3.0), false);
        let c = Content::h_space(Length::pt(3.0), true);   // weak diferente
        let d = Content::h_space(Length::pt(4.0), false);  // amount diferente
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    #[test]
    fn vspace_partial_eq() {
        use crate::entities::layout_types::Length;
        let a = Content::v_space(Length::pt(3.0), false);
        let b = Content::v_space(Length::pt(3.0), false);
        let c = Content::v_space(Length::pt(3.0), true);
        assert_eq!(a, b);
        assert_ne!(a, c);
        // VSpace e HSpace com mesmos campos NÃO são iguais (variantes
        // diferentes).
        let h = Content::h_space(Length::pt(3.0), false);
        assert_ne!(a, h);
    }

    #[test]
    fn hspace_e_vspace_map_text_preserva() {
        use crate::entities::layout_types::Length;
        // Spacing primitives são leaves — map_text não muda nada.
        let h = Content::h_space(Length::pt(7.0), false);
        let v = Content::v_space(Length::pt(7.0), true);
        let h_mapped = h.map_text(&mut |s| s.to_uppercase());
        let v_mapped = v.map_text(&mut |s| s.to_uppercase());
        assert_eq!(h, h_mapped);
        assert_eq!(v, v_mapped);
    }

    // ── Passo 156E (ADR-0061 Fase 1, sub-passo 3) — pagebreak ─────────────

    #[test]
    fn pagebreak_constructor() {
        use crate::entities::parity::Parity;
        let p = Content::pagebreak(false, None);
        if let Content::Pagebreak { weak, to } = p {
            assert!(!weak);
            assert_eq!(to, None);
        } else {
            panic!("esperado Content::Pagebreak");
        }
        let p2 = Content::pagebreak(true, Some(Parity::Even));
        if let Content::Pagebreak { weak, to } = p2 {
            assert!(weak);
            assert_eq!(to, Some(Parity::Even));
        } else {
            panic!("esperado Content::Pagebreak");
        }
    }

    #[test]
    fn pagebreak_is_empty_returns_false() {
        // Pagebreak é event observável mesmo "vazio" — análogo a Divider.
        let p = Content::pagebreak(false, None);
        assert!(!p.is_empty(),
            "Content::Pagebreak nunca é considerado vazio (event com efeito)");
    }

    #[test]
    fn pagebreak_plain_text_vazio() {
        let p = Content::pagebreak(false, None);
        assert_eq!(p.plain_text(), "");
    }

    #[test]
    fn pagebreak_partial_eq() {
        use crate::entities::parity::Parity;
        let a = Content::pagebreak(false, None);
        let b = Content::pagebreak(false, None);
        let c = Content::pagebreak(true,  None);                   // weak diferente
        let d = Content::pagebreak(false, Some(Parity::Even));     // to diferente
        let e = Content::pagebreak(false, Some(Parity::Odd));      // to diferente
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
        assert_ne!(d, e);
    }

    #[test]
    fn pagebreak_map_text_preserva() {
        use crate::entities::parity::Parity;
        let p = Content::pagebreak(true, Some(Parity::Even));
        // Pagebreak é leaf — map_text não tem effect.
        let p_mapped = p.map_text(&mut |s| s.to_uppercase());
        assert_eq!(p, p_mapped);
    }

    // ── Passo 156G (ADR-0061 Fase 2, sub-passo 1) — block container ───────

    #[test]
    fn block_constructor_default_field_values() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::block(
            Content::text("body"),
            None, None,
            Sides::uniform(Length::ZERO),
            true,
        );
        if let Content::Block { body, width, height, inset, breakable } = &b {
            assert_eq!(body.plain_text(), "body");
            assert_eq!(*width,  None);
            assert_eq!(*height, None);
            assert_eq!(inset.left, Length::ZERO);
            assert!(*breakable);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn block_with_explicit_width_height_inset() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::block(
            Content::text("x"),
            Some(Length::pt(100.0)),
            Some(Length::pt(50.0)),
            Sides::uniform(Length::pt(8.0)),
            false,
        );
        if let Content::Block { width, height, inset, breakable, .. } = &b {
            assert_eq!(*width,  Some(Length::pt(100.0)));
            assert_eq!(*height, Some(Length::pt(50.0)));
            assert_eq!(inset.left, Length::pt(8.0));
            assert!(!*breakable);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn block_is_empty_proxy_para_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        // Block com body Empty é vazio (atributos não-nulos não mudam isso).
        let b_empty = Content::block(
            Content::Empty,
            Some(Length::pt(100.0)),
            None,
            Sides::uniform(Length::pt(5.0)),
            true,
        );
        assert!(b_empty.is_empty());
        // Com texto, não vazio.
        let b_text = Content::block(
            Content::text("a"),
            None, None,
            Sides::uniform(Length::ZERO),
            true,
        );
        assert!(!b_text.is_empty());
    }

    #[test]
    fn block_plain_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::block(
            Content::text("hello"),
            None, None,
            Sides::uniform(Length::pt(2.0)),
            true,
        );
        assert_eq!(b.plain_text(), "hello");
    }

    #[test]
    fn block_partial_eq() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let mk = || Content::block(
            Content::text("x"),
            Some(Length::pt(50.0)),
            None,
            Sides::uniform(Length::pt(3.0)),
            true,
        );
        assert_eq!(mk(), mk());
        // Width diferente → diferente.
        let other_width = Content::block(
            Content::text("x"),
            Some(Length::pt(60.0)),
            None,
            Sides::uniform(Length::pt(3.0)),
            true,
        );
        assert_ne!(mk(), other_width);
        // breakable diferente → diferente.
        let other_breakable = Content::block(
            Content::text("x"),
            Some(Length::pt(50.0)),
            None,
            Sides::uniform(Length::pt(3.0)),
            false,
        );
        assert_ne!(mk(), other_breakable);
    }

    #[test]
    fn block_map_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::block(
            Content::text("hello"),
            None, None,
            Sides::uniform(Length::pt(1.0)),
            true,
        );
        let upper = b.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        // Atributos preservados após map_text.
        if let Content::Block { inset, .. } = upper {
            assert_eq!(inset.left, Length::pt(1.0));
        } else {
            panic!("esperado Content::Block após map_text");
        }
    }

    // ── Passo 156H (ADR-0061 Fase 2, sub-passo 2) — box inline container ──

    #[test]
    fn boxed_constructor_default() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::boxed(
            Content::text("body"),
            None, None,
            Sides::uniform(Length::ZERO),
            Length::ZERO,
        );
        if let Content::Boxed { body, width, height, inset, baseline } = &b {
            assert_eq!(body.plain_text(), "body");
            assert_eq!(*width,  None);
            assert_eq!(*height, None);
            assert_eq!(inset.left,  Length::ZERO);
            assert_eq!(*baseline, Length::ZERO);
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn boxed_constructor_explicit_atributos() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::boxed(
            Content::text("x"),
            Some(Length::pt(60.0)),
            Some(Length::pt(20.0)),
            Sides::uniform(Length::pt(2.0)),
            Length::pt(-3.0),  // baseline negativo aceito
        );
        if let Content::Boxed { width, height, inset, baseline, .. } = &b {
            assert_eq!(*width,  Some(Length::pt(60.0)));
            assert_eq!(*height, Some(Length::pt(20.0)));
            assert_eq!(inset.right, Length::pt(2.0));
            assert_eq!(*baseline, Length::pt(-3.0));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn boxed_is_empty_proxy_para_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let empty = Content::boxed(
            Content::Empty,
            Some(Length::pt(50.0)),
            None,
            Sides::uniform(Length::pt(5.0)),
            Length::ZERO,
        );
        assert!(empty.is_empty());
        let nonempty = Content::boxed(
            Content::text("a"),
            None, None,
            Sides::uniform(Length::ZERO),
            Length::ZERO,
        );
        assert!(!nonempty.is_empty());
    }

    #[test]
    fn boxed_plain_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::boxed(
            Content::text("hello"),
            None, None,
            Sides::uniform(Length::pt(1.0)),
            Length::ZERO,
        );
        assert_eq!(b.plain_text(), "hello");
    }

    #[test]
    fn boxed_partial_eq() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let mk = || Content::boxed(
            Content::text("x"),
            Some(Length::pt(40.0)),
            None,
            Sides::uniform(Length::pt(1.0)),
            Length::pt(2.0),
        );
        assert_eq!(mk(), mk());
        // baseline diferente → diferente.
        let other_baseline = Content::boxed(
            Content::text("x"),
            Some(Length::pt(40.0)),
            None,
            Sides::uniform(Length::pt(1.0)),
            Length::pt(5.0),
        );
        assert_ne!(mk(), other_baseline);
    }

    #[test]
    fn boxed_map_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::boxed(
            Content::text("hello"),
            None, None,
            Sides::uniform(Length::pt(1.0)),
            Length::pt(2.0),
        );
        let upper = b.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        // Atributos preservados.
        if let Content::Boxed { baseline, inset, .. } = upper {
            assert_eq!(baseline, Length::pt(2.0));
            assert_eq!(inset.left, Length::pt(1.0));
        } else {
            panic!("esperado Content::Boxed após map_text");
        }
    }

    // ── Passo 156I (ADR-0061 Fase 2, sub-passo 3) — stack compositivo ──────

    #[test]
    fn stack_constructor_default() {
        use crate::entities::dir::Dir;
        let s = Content::stack(
            vec![Content::text("a"), Content::text("b")],
            Dir::default(),
            None,
        );
        if let Content::Stack { children, dir, spacing } = &s {
            assert_eq!(children.len(), 2);
            assert_eq!(*dir, Dir::TTB);
            assert_eq!(*spacing, None);
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn stack_constructor_explicit_dir_spacing() {
        use crate::entities::dir::Dir;
        use crate::entities::layout_types::Length;
        let s = Content::stack(
            vec![Content::text("x"), Content::text("y")],
            Dir::LTR,
            Some(Length::pt(5.0)),
        );
        if let Content::Stack { dir, spacing, .. } = &s {
            assert_eq!(*dir, Dir::LTR);
            assert_eq!(*spacing, Some(Length::pt(5.0)));
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn stack_is_empty_se_todos_children_vazios() {
        use crate::entities::dir::Dir;
        // Children todos Empty → stack é vazio.
        let s_empty = Content::stack(
            vec![Content::Empty, Content::Empty],
            Dir::TTB, None,
        );
        assert!(s_empty.is_empty());
        // Stack sem children → também vazio.
        let s_zero = Content::stack(vec![], Dir::TTB, None);
        assert!(s_zero.is_empty());
        // Algum child com texto → não vazio.
        let s_nonempty = Content::stack(
            vec![Content::Empty, Content::text("a")],
            Dir::TTB, None,
        );
        assert!(!s_nonempty.is_empty());
    }

    #[test]
    fn stack_plain_text_concatena_children() {
        use crate::entities::dir::Dir;
        let s = Content::stack(
            vec![
                Content::text("Hello "),
                Content::text("world"),
            ],
            Dir::TTB, None,
        );
        // Plain text concatena (consistente com Sequence).
        assert_eq!(s.plain_text(), "Hello world");
    }

    #[test]
    fn stack_partial_eq() {
        use crate::entities::dir::Dir;
        use crate::entities::layout_types::Length;
        let mk = || Content::stack(
            vec![Content::text("a"), Content::text("b")],
            Dir::TTB,
            Some(Length::pt(3.0)),
        );
        assert_eq!(mk(), mk());
        // Dir diferente → diferente.
        let other_dir = Content::stack(
            vec![Content::text("a"), Content::text("b")],
            Dir::LTR,
            Some(Length::pt(3.0)),
        );
        assert_ne!(mk(), other_dir);
        // Spacing diferente → diferente.
        let other_spacing = Content::stack(
            vec![Content::text("a"), Content::text("b")],
            Dir::TTB,
            None,
        );
        assert_ne!(mk(), other_spacing);
        // Children diferentes → diferente.
        let other_children = Content::stack(
            vec![Content::text("a"), Content::text("c")],
            Dir::TTB,
            Some(Length::pt(3.0)),
        );
        assert_ne!(mk(), other_children);
    }

    #[test]
    fn stack_map_text_recurse_em_cada_child() {
        use crate::entities::dir::Dir;
        let s = Content::stack(
            vec![Content::text("hello"), Content::text("world")],
            Dir::TTB, None,
        );
        let upper = s.map_text(&mut |t| t.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLOWORLD");
        // Atributos preservados.
        if let Content::Stack { dir, .. } = upper {
            assert_eq!(dir, Dir::TTB);
        } else {
            panic!("esperado Content::Stack após map_text");
        }
    }

    // ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — Repeat ────────────────

    #[test]
    fn repeat_constructor_default_gap_justify() {
        let r = Content::repeat(Content::text("."), None, true);
        if let Content::Repeat { body, gap, justify } = &r {
            assert_eq!(body.plain_text(), ".");
            assert_eq!(*gap, None);
            assert!(*justify);
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn repeat_constructor_explicit_gap_justify_false() {
        use crate::entities::layout_types::Length;
        let r = Content::repeat(Content::text("a"), Some(Length::pt(2.0)), false);
        if let Content::Repeat { gap, justify, .. } = &r {
            assert_eq!(*gap, Some(Length::pt(2.0)));
            assert!(!*justify);
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn repeat_is_empty_proxy_via_body() {
        // Body Empty → repeat é vazio (atributos não tornam não-vazio).
        let r_empty = Content::repeat(Content::Empty, None, true);
        assert!(r_empty.is_empty());
        // Body com texto → não vazio.
        let r_dot = Content::repeat(Content::text("."), None, true);
        assert!(!r_dot.is_empty());
    }

    #[test]
    fn repeat_plain_text_recurse_no_body() {
        // Plain text recurse sem multiplicar — paridade não visível em
        // texto plano (semântica de repetição é runtime-only).
        let r = Content::repeat(Content::text("xy"), None, true);
        assert_eq!(r.plain_text(), "xy");
    }

    #[test]
    fn repeat_partial_eq_cobre_todos_os_fields() {
        use crate::entities::layout_types::Length;
        let mk = || Content::repeat(
            Content::text("."),
            Some(Length::pt(3.0)),
            true,
        );
        assert_eq!(mk(), mk());
        // Body diferente → diferente.
        let other_body = Content::repeat(
            Content::text("o"),
            Some(Length::pt(3.0)),
            true,
        );
        assert_ne!(mk(), other_body);
        // Gap diferente → diferente.
        let other_gap = Content::repeat(
            Content::text("."),
            None,
            true,
        );
        assert_ne!(mk(), other_gap);
        // Justify diferente → diferente.
        let other_justify = Content::repeat(
            Content::text("."),
            Some(Length::pt(3.0)),
            false,
        );
        assert_ne!(mk(), other_justify);
    }

    #[test]
    fn repeat_map_text_recurse_no_body() {
        use crate::entities::layout_types::Length;
        let r = Content::repeat(
            Content::text("hello"),
            Some(Length::pt(2.0)),
            false,
        );
        let upper = r.map_text(&mut |t| t.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        // Atributos preservados.
        if let Content::Repeat { gap, justify, .. } = upper {
            assert_eq!(gap, Some(Length::pt(2.0)));
            assert!(!justify);
        } else {
            panic!("esperado Content::Repeat após map_text");
        }
    }

    // ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — Table ─────────────────

    #[test]
    fn table_constructor_default() {
        let t = Content::table(vec![], vec![], vec![]);
        if let Content::Table { columns, rows, children } = &t {
            assert!(columns.is_empty());
            assert!(rows.is_empty());
            assert!(children.is_empty());
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn table_constructor_com_tracks_e_children() {
        use crate::entities::layout_types::TrackSizing;
        let t = Content::table(
            vec![TrackSizing::Auto, TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("a"), Content::text("b")],
        );
        if let Content::Table { columns, rows, children } = &t {
            assert_eq!(columns.len(), 2);
            assert_eq!(rows.len(), 1);
            assert_eq!(children.len(), 2);
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn table_is_empty_proxy_via_children() {
        use crate::entities::layout_types::TrackSizing;
        // Children vazios → table vazio (mesmo com tracks declaradas).
        let t_empty = Content::table(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![],
        );
        assert!(t_empty.is_empty());
        // Children com texto → não vazio.
        let t_full = Content::table(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("x")],
        );
        assert!(!t_full.is_empty());
    }

    #[test]
    fn table_plain_text_concatena_children_com_space() {
        use crate::entities::layout_types::TrackSizing;
        let t = Content::table(
            vec![TrackSizing::Auto, TrackSizing::Auto],
            vec![],
            vec![Content::text("hello"), Content::text("world")],
        );
        // Paridade Grid: join(" ").
        assert_eq!(t.plain_text(), "hello world");
    }

    #[test]
    fn table_partial_eq() {
        use crate::entities::layout_types::TrackSizing;
        let mk = || Content::table(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("a")],
        );
        assert_eq!(mk(), mk());
        // Children diferentes → diferente.
        let other_children = Content::table(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("b")],
        );
        assert_ne!(mk(), other_children);
        // Columns diferentes → diferente.
        let other_columns = Content::table(
            vec![TrackSizing::Auto, TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("a")],
        );
        assert_ne!(mk(), other_columns);
    }

    #[test]
    fn table_map_text_recurse_em_children() {
        use crate::entities::layout_types::TrackSizing;
        let t = Content::table(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("hello"), Content::text("world")],
        );
        let upper = t.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO WORLD");
    }

    // ── Passo 157B (ADR-0060 Fase 2 sub-passo 2) — TableCell ─────────────

    #[test]
    fn table_cell_constructor_default_todos_none() {
        // P157B: defaults — todos os fields x/y/colspan/rowspan None.
        let c = Content::table_cell(Content::text("body"), None, None, None, None);
        if let Content::TableCell { body, x, y, colspan, rowspan } = &c {
            assert_eq!(body.plain_text(), "body");
            assert_eq!(*x, None);
            assert_eq!(*y, None);
            assert_eq!(*colspan, None);
            assert_eq!(*rowspan, None);
        } else {
            panic!("esperado Content::TableCell");
        }
    }

    #[test]
    fn table_cell_constructor_com_x_y() {
        // P157B: ADR-0064 Caso A — Some(n) ↔ posição explícita.
        let c = Content::table_cell(
            Content::text("x"),
            Some(2), Some(3),
            None, None,
        );
        if let Content::TableCell { x, y, .. } = &c {
            assert_eq!(*x, Some(2));
            assert_eq!(*y, Some(3));
        } else {
            panic!("esperado Content::TableCell");
        }
    }

    #[test]
    fn table_cell_constructor_com_colspan_rowspan() {
        // P157B: ADR-0064 Caso C — Some(n) ↔ span explícito.
        let c = Content::table_cell(
            Content::text("x"),
            None, None,
            Some(2), Some(3),
        );
        if let Content::TableCell { colspan, rowspan, .. } = &c {
            assert_eq!(*colspan, Some(2));
            assert_eq!(*rowspan, Some(3));
        } else {
            panic!("esperado Content::TableCell");
        }
    }

    #[test]
    fn table_cell_is_empty_proxy_via_body() {
        // Body Empty → cell vazio (atributos não tornam não-vazio).
        let c_empty = Content::table_cell(Content::Empty, Some(2), Some(3), None, None);
        assert!(c_empty.is_empty());
        // Body com texto → não vazio.
        let c_full = Content::table_cell(Content::text("a"), None, None, None, None);
        assert!(!c_full.is_empty());
    }

    #[test]
    fn table_cell_plain_text_recurse_no_body() {
        // Plain text recurse sem multiplicar por colspan/rowspan
        // (paridade não visível em texto plano; spans são runtime
        // diferidos em DEBT-34e).
        let c = Content::table_cell(
            Content::text("xy"),
            None, None,
            Some(3), Some(2),
        );
        assert_eq!(c.plain_text(), "xy");
    }

    #[test]
    fn table_cell_partial_eq_cobre_todos_os_5_fields() {
        let mk = || Content::table_cell(
            Content::text("a"),
            Some(1), Some(2),
            Some(3), Some(4),
        );
        assert_eq!(mk(), mk());
        // x diferente → diferente.
        let other_x = Content::table_cell(Content::text("a"), Some(99), Some(2), Some(3), Some(4));
        assert_ne!(mk(), other_x);
        // y diferente → diferente.
        let other_y = Content::table_cell(Content::text("a"), Some(1), Some(99), Some(3), Some(4));
        assert_ne!(mk(), other_y);
        // colspan diferente → diferente.
        let other_cs = Content::table_cell(Content::text("a"), Some(1), Some(2), Some(99), Some(4));
        assert_ne!(mk(), other_cs);
        // rowspan diferente → diferente.
        let other_rs = Content::table_cell(Content::text("a"), Some(1), Some(2), Some(3), Some(99));
        assert_ne!(mk(), other_rs);
        // body diferente → diferente.
        let other_body = Content::table_cell(Content::text("b"), Some(1), Some(2), Some(3), Some(4));
        assert_ne!(mk(), other_body);
    }

    #[test]
    fn table_cell_map_text_recurse_no_body_preserva_fields() {
        let c = Content::table_cell(
            Content::text("hello"),
            Some(2), Some(3),
            Some(4), Some(5),
        );
        let upper = c.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        // Atributos preservados após map_text.
        if let Content::TableCell { x, y, colspan, rowspan, .. } = upper {
            assert_eq!(x, Some(2));
            assert_eq!(y, Some(3));
            assert_eq!(colspan, Some(4));
            assert_eq!(rowspan, Some(5));
        } else {
            panic!("esperado Content::TableCell após map_text");
        }
    }

    // ── Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha table foundations) ──
    // Par simétrico TableHeader/TableFooter — tests imediatamente
    // adjacentes para tornar paridade visualmente óbvia.

    #[test]
    fn table_header_constructor_default_repeat_true() {
        // P157C ADR-0064 Caso D: default vanilla `repeat=true`.
        let h = Content::table_header(Content::text("body"), true);
        if let Content::TableHeader { body, repeat } = &h {
            assert_eq!(body.plain_text(), "body");
            assert!(*repeat, "default vanilla repeat=true (Caso D)");
        } else {
            panic!("esperado Content::TableHeader");
        }
    }

    #[test]
    fn table_footer_constructor_default_repeat_true() {
        // Par simétrico — paridade absoluta com TableHeader.
        let f = Content::table_footer(Content::text("body"), true);
        if let Content::TableFooter { body, repeat } = &f {
            assert_eq!(body.plain_text(), "body");
            assert!(*repeat);
        } else {
            panic!("esperado Content::TableFooter");
        }
    }

    #[test]
    fn table_header_repeat_false_explicito() {
        let h = Content::table_header(Content::text("x"), false);
        if let Content::TableHeader { repeat, .. } = h {
            assert!(!repeat);
        } else {
            panic!("esperado Content::TableHeader");
        }
    }

    #[test]
    fn table_footer_repeat_false_explicito() {
        let f = Content::table_footer(Content::text("x"), false);
        if let Content::TableFooter { repeat, .. } = f {
            assert!(!repeat);
        } else {
            panic!("esperado Content::TableFooter");
        }
    }

    #[test]
    fn table_header_is_empty_proxy_via_body() {
        let h_empty = Content::table_header(Content::Empty, true);
        let h_full  = Content::table_header(Content::text("a"), true);
        assert!(h_empty.is_empty());
        assert!(!h_full.is_empty());
    }

    #[test]
    fn table_footer_is_empty_proxy_via_body() {
        let f_empty = Content::table_footer(Content::Empty, true);
        let f_full  = Content::table_footer(Content::text("a"), true);
        assert!(f_empty.is_empty());
        assert!(!f_full.is_empty());
    }

    #[test]
    fn table_header_plain_text_recurse_no_body() {
        let h = Content::table_header(Content::text("hi"), true);
        assert_eq!(h.plain_text(), "hi");
    }

    #[test]
    fn table_footer_plain_text_recurse_no_body() {
        let f = Content::table_footer(Content::text("hi"), true);
        assert_eq!(f.plain_text(), "hi");
    }

    #[test]
    fn table_header_partial_eq() {
        let mk = || Content::table_header(Content::text("a"), true);
        assert_eq!(mk(), mk());
        // body diferente → diferente.
        let other_body = Content::table_header(Content::text("b"), true);
        assert_ne!(mk(), other_body);
        // repeat diferente → diferente.
        let other_repeat = Content::table_header(Content::text("a"), false);
        assert_ne!(mk(), other_repeat);
    }

    #[test]
    fn table_footer_partial_eq() {
        let mk = || Content::table_footer(Content::text("a"), true);
        assert_eq!(mk(), mk());
        let other_body = Content::table_footer(Content::text("b"), true);
        assert_ne!(mk(), other_body);
        let other_repeat = Content::table_footer(Content::text("a"), false);
        assert_ne!(mk(), other_repeat);
    }

    #[test]
    fn table_header_e_footer_sao_variants_distintos() {
        // Paridade interna absoluta no contrato; mas variants
        // distintos ao nível do enum (não confundir Header com
        // Footer mesmo com mesmos fields).
        let h = Content::table_header(Content::text("a"), true);
        let f = Content::table_footer(Content::text("a"), true);
        assert_ne!(h, f, "TableHeader e TableFooter são variants distintos");
    }

    #[test]
    fn table_header_map_text_recurse_e_preserva_repeat() {
        let h = Content::table_header(Content::text("hello"), false);
        let upper = h.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        if let Content::TableHeader { repeat, .. } = upper {
            assert!(!repeat, "repeat preservado após map_text");
        } else {
            panic!("esperado Content::TableHeader");
        }
    }

    #[test]
    fn table_footer_map_text_recurse_e_preserva_repeat() {
        let f = Content::table_footer(Content::text("hello"), false);
        let upper = f.map_text(&mut |s| s.to_uppercase());
        assert_eq!(upper.plain_text(), "HELLO");
        if let Content::TableFooter { repeat, .. } = upper {
            assert!(!repeat);
        } else {
            panic!("esperado Content::TableFooter");
        }
    }

    // ── Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado) ──

    #[test]
    fn bibliography_constructor_default_vazia() {
        let b = Content::bibliography(vec![], None);
        if let Content::Bibliography { entries, title } = &b {
            assert!(entries.is_empty());
            assert!(title.is_none());
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn bibliography_constructor_com_entries_e_title() {
        use crate::entities::bib_entry::BibEntry;
        let b = Content::bibliography(
            vec![BibEntry::new("k1", "A1", "T1", 2024)],
            Some(Content::text("Referências")),
        );
        if let Content::Bibliography { entries, title } = &b {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].key, "k1");
            assert_eq!(title.as_ref().map(|t| t.plain_text()).as_deref(), Some("Referências"));
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn bibliography_is_empty_proxy_via_entries_e_title() {
        use crate::entities::bib_entry::BibEntry;
        // Vazia (sem entries, sem title) → empty.
        let b_empty = Content::bibliography(vec![], None);
        assert!(b_empty.is_empty());
        // Só com title → não empty.
        let b_title = Content::bibliography(vec![], Some(Content::text("R")));
        assert!(!b_title.is_empty());
        // Só com entries → não empty.
        let b_entries = Content::bibliography(
            vec![BibEntry::new("k", "A", "T", 2024)], None,
        );
        assert!(!b_entries.is_empty());
    }

    #[test]
    fn bibliography_plain_text_concatena_title_e_entries() {
        use crate::entities::bib_entry::BibEntry;
        let b = Content::bibliography(
            vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
            Some(Content::text("Referências")),
        );
        let txt = b.plain_text();
        assert!(txt.contains("Referências"), "title presente no plain_text");
        assert!(txt.contains("[smith2024]"), "key formatada como [key]");
        assert!(txt.contains("Smith, J."), "author presente");
        assert!(txt.contains("On Crystal Math"), "title presente");
        assert!(txt.contains("2024"), "year presente");
    }

    #[test]
    fn bibliography_partial_eq_cobre_2_fields() {
        use crate::entities::bib_entry::BibEntry;
        let mk = || Content::bibliography(
            vec![BibEntry::new("k", "A", "T", 2024)],
            Some(Content::text("R")),
        );
        assert_eq!(mk(), mk());
        // entries diferentes → diferente.
        let other_entries = Content::bibliography(
            vec![BibEntry::new("k", "A", "T", 2025)],  // year diferente
            Some(Content::text("R")),
        );
        assert_ne!(mk(), other_entries);
        // title diferente → diferente.
        let other_title = Content::bibliography(
            vec![BibEntry::new("k", "A", "T", 2024)],
            Some(Content::text("Bibliografia")),
        );
        assert_ne!(mk(), other_title);
    }

    #[test]
    fn cite_constructor_so_key() {
        let c = Content::cite("smith2024", None);
        if let Content::Cite { key, supplement } = &c {
            assert_eq!(key, "smith2024");
            assert!(supplement.is_none());
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn cite_constructor_com_supplement() {
        let c = Content::cite("smith2024", Some(Content::text("p. 42")));
        if let Content::Cite { key, supplement } = &c {
            assert_eq!(key, "smith2024");
            assert_eq!(supplement.as_ref().map(|s| s.plain_text()).as_deref(), Some("p. 42"));
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn cite_is_empty_sempre_false() {
        // Cite nunca vazio — placeholder [key] sempre observable.
        let c1 = Content::cite("k", None);
        let c2 = Content::cite("k", Some(Content::text("p. 1")));
        assert!(!c1.is_empty());
        assert!(!c2.is_empty());
    }

    #[test]
    fn cite_plain_text_emite_placeholder_com_key() {
        // Sem supplement.
        let c1 = Content::cite("smith2024", None);
        assert_eq!(c1.plain_text(), "[smith2024]");
        // Com supplement.
        let c2 = Content::cite("smith2024", Some(Content::text("p. 42")));
        assert_eq!(c2.plain_text(), "[smith2024]p. 42");
    }

    #[test]
    fn cite_partial_eq_cobre_2_fields() {
        let mk = || Content::cite("k", Some(Content::text("p. 1")));
        assert_eq!(mk(), mk());
        // key diferente → diferente.
        let other_key = Content::cite("k2", Some(Content::text("p. 1")));
        assert_ne!(mk(), other_key);
        // supplement diferente → diferente.
        let other_sup = Content::cite("k", Some(Content::text("p. 99")));
        assert_ne!(mk(), other_sup);
        // supplement None vs Some → diferente.
        let other_none = Content::cite("k", None);
        assert_ne!(mk(), other_none);
    }
}
