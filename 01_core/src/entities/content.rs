//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content.md
//! @prompt-hash 85fae9b9
//! @layer L1
//! @updated 2026-04-20

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::counter_state::CounterAction;
use crate::entities::geometry::{ShapeKind, Stroke};
use crate::entities::label::Label;
use crate::entities::layout_types::{Align2D, Color, Pt, TextStyle, TrackSizing, TransformMatrix};
use crate::entities::ptr_eq_arc::PtrEqArc;

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

    // ── Rich text (Passo 22) ─────────────────────────────────────────────
    /// Conteúdo em negrito (`*Strong*`).
    Strong(Box<Content>),
    /// Conteúdo em itálico (`_Emph_`).
    Emph(Box<Content>),
    /// Cabeçalho com nível 1–6 (`= Heading`).
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
    /// `rows` é armazenado no AST mas ignorado pelo layouter (DEBT-34b).
    Grid {
        columns: Vec<TrackSizing>,
        rows:    Vec<TrackSizing>, // DEBT-34b: ignorado — todas as linhas são Auto
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
    /// DEBT-37: ancora às margens da página, não ao contentor pai.
    Place {
        alignment: Align2D,
        dx:        f64,
        dy:        f64,
        body:      Box<Content>,
    },

    // Variantes futuras — NÃO implementar sem ADR:
    // Styled(Box<Content>, StyleChain),          // requer StyleChain — Passo 30+
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
    pub fn strong(body: Content) -> Self { Self::Strong(Box::new(body)) }
    pub fn emph(body: Content)   -> Self { Self::Emph(Box::new(body)) }
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
            Self::Strong(c)          => c.plain_text(),
            Self::Emph(c)            => c.plain_text(),
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
            Self::SetPage { .. } => String::new(),
            Self::Align { body, .. } => body.plain_text(),
            Self::Place { body, .. } => body.plain_text(),
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
            (Self::Strong(a),            Self::Strong(b))            => a == b,
            (Self::Emph(a),              Self::Emph(b))              => a == b,
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
            (Self::SetPage { width: wa, height: ha, margin: ma },
             Self::SetPage { width: wb, height: hb, margin: mb }) =>
                wa == wb && ha == hb && ma == mb,
            (Self::Align { alignment: aa, body: ba },
             Self::Align { alignment: ab, body: bb }) => aa == ab && ba == bb,
            (Self::Place { alignment: aa, dx: dxa, dy: dya, body: ba },
             Self::Place { alignment: ab, dx: dxb, dy: dyb, body: bb }) =>
                aa == ab && dxa == dxb && dya == dyb && ba == bb,
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
            Content::Strong(body) => Content::Strong(Box::new(body.map_content(transform)?)),
            Content::Emph(body)   => Content::Emph(Box::new(body.map_content(transform)?)),
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

            // ── Terminais: clonar directamente ──────────────────────────────
            // Listados explicitamente — variantes novas não passam em silêncio.
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
            Content::Align { alignment, body } => Content::Align {
                alignment: *alignment,
                body:      Box::new(body.map_content(transform)?),
            },
            Content::Place { alignment, dx, dy, body } => Content::Place {
                alignment: *alignment,
                dx:        *dx,
                dy:        *dy,
                body:      Box::new(body.map_content(transform)?),
            },
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
            Content::Text(s, style) => Content::Text(transform(s.as_str()).into(), *style),

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
            Content::Strong(body) => Content::Strong(Box::new(body.map_text(transform))),
            Content::Emph(body)   => Content::Emph(Box::new(body.map_text(transform))),
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

            // ── Terminais — clonar directamente ──────────────────────────
            // Nós matemáticos e estruturais sem markup Text — não contêm
            // Content::Text, portanto clonar em bloco é correcto e seguro.
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
            Content::Align { alignment, body } => Content::Align {
                alignment: *alignment,
                body:      Box::new(body.map_text(transform)),
            },
            Content::Place { alignment, dx, dy, body } => Content::Place {
                alignment: *alignment,
                dx:        *dx,
                dy:        *dy,
                body:      Box::new(body.map_text(transform)),
            },
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
        let content = Content::Strong(Box::new(Content::text("hello")));
        let result = content.map_text(&mut |s| s.to_uppercase());
        assert_eq!(result, Content::Strong(Box::new(Content::text("HELLO"))));
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
            Content::Strong(Box::new(Content::text("a"))),
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
        let content = Content::Strong(Box::new(Content::text("original")));

        let result = content.map_content(&mut |node| {
            match node {
                Content::Text(s, _) => Ok(Some(Content::text(s.to_uppercase()))),
                Content::Strong(body) => {
                    let text = body.plain_text();
                    assert_eq!(text, "ORIGINAL",
                        "Strong deve receber filho já transformado: {:?}", text);
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
}
