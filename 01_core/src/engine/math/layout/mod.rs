//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/_comum.md
//! @prompt-hash c83026f1
//! @layer L1
//! @updated 2026-04-11

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use std::sync::Arc;

use ecow::EcoString;

use super::symbols;
use crate::engine::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Point, Pt, TextStyle},
    math_constants::MathConstants,
    math_style::{is_math_italic_default, map_glyph, map_glyph_vs, MathStyleKind},
};

// Sub-métodos do layout matemático extraídos por fase (Passo 96.8, ADR-0037).
mod assembly;
mod attach;
mod cases;
mod delimited;
mod frac;
mod matrix;
mod root;
mod spacing;
mod stretchy;

/// Caixa tipográfica de um nó matemático.
/// Todas as medidas são em pontos, relativas à baseline da equação.
/// `ascent` > 0 (acima da baseline), `descent` > 0 (abaixo da baseline).
//
// Visibilidade `pub(super)` nos campos (Passo 96.8): os submódulos
// `attach.rs`, `root.rs`, `frac.rs`, `stretchy.rs`, `grid_layout.rs`
// constroem e compõem `MathBox` directamente — os acessos são quase
// todos em construção (`MathBox { .. }`) ou leitura simples. A ADR-0037
// Regra 3 autoriza `pub(super)` quando métodos não agregam invariante.
#[derive(Debug, Clone)]
pub(super) struct MathBox {
    pub(super) width: f64,
    pub(super) ascent: f64,
    pub(super) descent: f64,
    /// Items com posições relativas ao topo esquerdo deste MathBox.
    pub(super) items: Vec<FrameItem>,
}

impl MathBox {
    fn height(&self) -> f64 {
        self.ascent + self.descent
    }

    /// Converte para FrameItems com posição no frame pai.
    ///
    /// `x_origin`: deslocamento horizontal no frame pai.
    /// `baseline_y`: posição da baseline no frame pai (y cresce para baixo).
    ///
    /// Conversão: `parent_y = baseline_y - ascent + local_y`
    ///   - `local_y = 0` → topo do box (acima da baseline)
    ///   - `local_y = ascent` → na baseline
    fn place(self, x_origin: f64, baseline_y: f64) -> Vec<FrameItem> {
        self.items
            .into_iter()
            .map(|mut item| {
                match item {
                    FrameItem::Text { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x_origin);
                        pos.y = Pt(baseline_y - self.ascent + pos.y.val());
                    }
                    FrameItem::TextShaped { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x_origin);
                        pos.y = Pt(baseline_y - self.ascent + pos.y.val());
                    }
                    FrameItem::Line { ref mut start, ref mut end, .. } => {
                        start.x = Pt(start.x.val() + x_origin);
                        end.x = Pt(end.x.val() + x_origin);
                        start.y = Pt(baseline_y - self.ascent + start.y.val());
                        end.y = Pt(baseline_y - self.ascent + end.y.val());
                    }
                    FrameItem::Glyph { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x_origin);
                        pos.y = Pt(baseline_y - self.ascent + pos.y.val());
                    }
                    FrameItem::Image { .. } => {} // imagens não ocorrem em contexto math
                    FrameItem::Shape { .. } => {} // formas não ocorrem em contexto math
                    FrameItem::Group { .. } => {} // grupos não ocorrem em contexto math
                    FrameItem::Link { .. } => {}  // links não ocorrem em contexto math
                }
                item
            })
            .collect()
    }
}

/// **P813** — Extensão geométrica de uma equação laid out, em pontos:
/// largura total e limites de tinta acima/abaixo da baseline. Produzido
/// por `MathLayouter::layout_equation_measured`; consumido pelo layout de
/// equações de bloco (`engine/layout/equation.rs`) para centragem
/// horizontal e espaçamento vertical (paridade vanilla — ver
/// `engine/layout/equation.md`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquationExtent {
    /// Limite direito máximo dos items (`pos.x + advance`).
    pub width: f64,
    /// Tinta máxima acima da baseline (>= 0).
    pub ascent: f64,
    /// Tinta máxima abaixo da baseline (>= 0).
    pub descent: f64,
}

/// Desloca um `FrameItem` por `(dx, dy)`.
pub(super) fn offset_item(item: FrameItem, dx: Pt, dy: Pt) -> FrameItem {
    match item {
        FrameItem::Text { pos, text, style } => FrameItem::Text {
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            text,
            style,
        },
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            FrameItem::TextShaped {
                pos: Point {
                    x: Pt(pos.x.val() + dx.val()),
                    y: Pt(pos.y.val() + dy.val()),
                },
                glyphs,
                style,
                text,
                units_per_em,
            }
        }
        FrameItem::Line { start, end, thickness, color } => FrameItem::Line {
            start: Point {
                x: Pt(start.x.val() + dx.val()),
                y: Pt(start.y.val() + dy.val()),
            },
            end: Point {
                x: Pt(end.x.val() + dx.val()),
                y: Pt(end.y.val() + dy.val()),
            },
            thickness,
            // P285: reflector preserva cor original (translação não afecta paint).
            color,
        },
        FrameItem::Glyph { pos, glyph_id, x_advance, size, style, base_char } => FrameItem::Glyph {
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            glyph_id,
            x_advance,
            size,
            style,
            base_char,
        },
        FrameItem::Image {
            pos,
            data,
            width,
            height,
            intrinsic_width,
            intrinsic_height,
            orientation,
            ..
        } => FrameItem::Image {
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            data,
            width,
            height,
            intrinsic_width,
            intrinsic_height,
            clip_rect: None,
            orientation,
        },
        FrameItem::Shape {
            pos,
            kind,
            width,
            height,
            fill,
            stroke,
            parent_bbox_at_emit,
        } => FrameItem::Shape {
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            kind,
            width,
            height,
            fill,
            stroke,
            parent_bbox_at_emit,
        },
        FrameItem::Group {
            pos,
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
        } => FrameItem::Group {
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
        },
        FrameItem::Link { target, items, pos, size } => FrameItem::Link {
            target,
            items,
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            size,
        },
    }
}

/// Verifica se uma sequência de nós matemáticos precisa de layout em grelha.
///
/// Retorna `true` se houver pelo menos um `MathAlignPoint` ou `Linebreak`.
/// Se `false`, o layout linear existente é usado sem custo adicional.
fn needs_grid_layout(nodes: &[Content]) -> bool {
    nodes
        .iter()
        .any(|c| matches!(c, Content::MathAlignPoint(_) | Content::Linebreak(_)))
}

/// Particiona uma sequência flat em linhas e colunas.
///
/// Retorna `Vec<Vec<Vec<Content>>>`:
///   - dim 0: linhas (separadas por `Linebreak`)
///   - dim 1: colunas (separadas por `MathAlignPoint`)
///   - dim 2: items da célula
///
/// Células e linhas finais vazias são removidas.
fn partition_grid(nodes: &[Content]) -> Vec<Vec<Vec<Content>>> {
    let mut lines: Vec<Vec<Vec<Content>>> = vec![vec![vec![]]];

    for node in nodes {
        match node {
            Content::Linebreak(_) => {
                lines.push(vec![vec![]]);
            }
            Content::MathAlignPoint(_) => {
                lines.last_mut().unwrap().push(vec![]);
            }
            other => {
                lines.last_mut().unwrap().last_mut().unwrap().push(other.clone());
            }
        }
    }

    // Remover células finais vazias em cada linha
    for line in &mut lines {
        while line.last().map(|c| c.is_empty()).unwrap_or(false) {
            line.pop();
        }
    }

    // Remover linhas finais completamente vazias
    while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines
}

/// Motor de layout matemático — stateless.
///
/// Recebe `Content` matemático e produz um `Vec<FrameItem>` com posições
/// relativas à origem `(0, 0)`, prontas para integração no layouter principal.
///
/// **Passo 36**: `MathIdent` e `MathText` → `FrameItem::Text`.
/// **Passo 37**: `MathFrac` (numerador/denominador) e `MathAttach`
///   (sup/sub com posicionamento vertical) implementados via `MathBox`.
/// **Passo 41**: constantes OpenType MATH via `FontMetrics::math_constants()`.
/// Política de alinhamento horizontal das células numa grelha matemática.
pub(super) enum GridAlign {
    /// `MathAlignPoint` (`&`): colunas pares à direita, ímpares à esquerda.
    Alternating,
    /// `MathMatrix` (`mat`): todas as células centradas na sua coluna.
    Center,
    /// `MathCases` (`cases`): todas as colunas alinhadas à esquerda.
    Left,
}

// Regra 3 (ADR-0037): os campos `pub(super)` abaixo são lidos por quase
// todos os submódulos (`attach`, `root`, `frac`, `matrix`, `cases`, `stretchy`,
// `assembly`, `delimited`). `metrics` é o acesso ao trait `FontMetrics`,
// `constants` cachea as constantes OpenType MATH da fonte activa, `block`
// controla display vs inline. São dados passivos — getters triplicariam
// acessos sem invariante. Validado no Passo 97 (DEBT-47).
pub struct MathLayouter<'a, M: FontMetrics> {
    pub(super) metrics: &'a M,
    pub(super) constants: MathConstants,
    /// True se a equação é de bloco (display mode); false se inline.
    /// Controla se operadores grandes usam limites verticais (Passo 50).
    pub(super) block: bool,
}

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub fn new(metrics: &'a M, block: bool) -> Self {
        let constants = metrics.math_constants();
        Self { metrics, constants, block }
    }

    /// Centra um MathBox no eixo matemático ajustando ascent/descent.
    ///
    /// O eixo matemático é `axis_height` (design units) acima da baseline.
    /// Após este ajuste, o centro vertical do box fica no eixo.
    ///
    /// Aplica-se a fracções, delimitadores e raízes — não a elementos inline.
    pub(super) fn apply_axis_offset(&self, mut b: MathBox, size: Pt) -> MathBox {
        let axis_pt = self.constants.to_pt(self.constants.axis_height, size).val();
        let shift = axis_pt - (b.ascent - b.descent) / 2.0;
        b.ascent += shift;
        b.descent -= shift;
        b
    }

    /// Ponto de entrada: recebe o body de uma equação e produz `Vec<FrameItem>`.
    ///
    /// Os items retornados têm posições **relativas à baseline da fórmula**
    /// (baseline em `y = 0`; scripts acima têm `y` negativo) — o layouter
    /// principal ajusta para posição absoluta na página somando a baseline
    /// pretendida (P800: para inline, a baseline do texto circundante).
    pub fn layout_equation(&self, body: &Content, style: &TextStyle) -> Vec<FrameItem> {
        // **P809** — itálico matemático por defeito aplicado UMA vez aqui
        // (paridade codex `MathStyle::select`). **P812** — via
        // `apply_math_default` (não `apply_math_style`): esta PRESERVA os
        // nós `MathStyled` (a sua composição — incluindo o default de
        // itálico para `bold(x)` — corre no handler dedicado em
        // `layout_node`, que também aplica o factor de tamanho); a versão
        // P809 com `apply_math_style` no topo consumia o wrapper e
        // destruía display/script/sscript (regressão medida em P812-A).
        let transformed = apply_math_default(body);
        let math_box = self.layout_node(&transformed, style);
        // `place` com baseline_y = ascent ⇒ parent_y = local_y: os items
        // internos já são compostos com y=0 na baseline (ex.: attach põe sup
        // em -sup_offset), logo o resultado fica baseline-relativo.
        let baseline_y = math_box.ascent;
        math_box.place(0.0, baseline_y)
    }

    /// **P813** — como `layout_equation`, mas devolve também a extensão
    /// geométrica da equação (`EquationExtent`), calculada dos **mesmos**
    /// items (não é um segundo caminho de layout):
    ///
    /// - `width`: limite direito máximo (`pos.x + advance`) — para a
    ///   centragem horizontal de equações de bloco;
    /// - `ascent`/`descent`: limites de **tinta** acima/abaixo da baseline,
    ///   via `FontMetrics::text_ink_bounds` — paridade vanilla, cujo frame
    ///   math usa as bounding boxes dos glyphs (não as métricas globais da
    ///   fonte). Para `FrameItem::Glyph` (delimitadores extensíveis, sem
    ///   texto Unicode) usa-se `cap_height` como aproximação.
    ///
    /// Consumidor: `engine/layout/equation.rs` (centragem + espaçamento
    /// vertical de equações de bloco — P813).
    pub fn layout_equation_measured(
        &self,
        body: &Content,
        style: &TextStyle,
    ) -> (Vec<FrameItem>, EquationExtent) {
        let items = self.layout_equation(body, style);
        let mut extent = EquationExtent { width: 0.0, ascent: 0.0, descent: 0.0 };
        for item in &items {
            match item {
                FrameItem::Text { pos, text, style }
                | FrameItem::TextShaped { pos, text, style, .. } => {
                    let right =
                        pos.x.val() + self.metrics.advance(text, style.size, style).val();
                    extent.width = extent.width.max(right);
                    let (ink_up, ink_down) =
                        self.metrics.text_ink_bounds(text, style.size, style);
                    extent.ascent = extent.ascent.max(ink_up.val() - pos.y.val());
                    extent.descent = extent.descent.max(pos.y.val() + ink_down.val());
                }
                FrameItem::Glyph { pos, x_advance, size, .. } => {
                    extent.width = extent.width.max(pos.x.val() + x_advance.val());
                    // Aproximação documentada (P813): sem texto Unicode, a
                    // tinta é estimada pela cap-height; sem descent.
                    let up = self.metrics.cap_height(*size, &TextStyle::default());
                    extent.ascent = extent.ascent.max(up.val() - pos.y.val());
                    extent.descent = extent.descent.max(pos.y.val());
                }
                FrameItem::Line { start, end, thickness, .. } => {
                    extent.width = extent.width.max(start.x.val()).max(end.x.val());
                    let half = thickness / 2.0;
                    extent.ascent =
                        extent.ascent.max(half - start.y.val().min(end.y.val()));
                    extent.descent =
                        extent.descent.max(start.y.val().max(end.y.val()) + half);
                }
                FrameItem::Image { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Group { .. }
                | FrameItem::Link { .. } => {} // não ocorrem em contexto math
            }
        }
        (items, extent)
    }


    /// Percorre a árvore de Content matemático recursivamente, produzindo um `MathBox`.
    pub(super) fn layout_node(&self, content: &Content, style: &TextStyle) -> MathBox {
        match content {
            Content::MathIdent(name) => {
                // **P809** — o itálico por defeito (codepoint) já foi aplicado
                // no topo (`layout_equation`); aqui é só layout. A flag de
                // fonte italic já não é usada para variáveis de 1 letra (o
                // codepoint carrega o estilo — paridade vanilla).
                self.layout_text_node(
                    name,
                    &TextStyle { italic: false, ..style.clone() },
                )
            }
            Content::MathText(text) => self.layout_text_node(text, style),

            Content::MathSequence(nodes) => self.layout_sequence(nodes, style),

            // Modelo D (Lote 2 P317): destructuring de `Arc<…Elem>` — mesma lógica.
            Content::MathFrac(e) => self.layout_frac(&e.num, &e.den, style),

            Content::MathAttach(e) => self.layout_attach(
                &e.base,
                e.tl.as_ref(),
                e.bl.as_ref(),
                e.sub.as_ref(),
                e.sup.as_ref(),
                style,
            ),

            Content::MathRoot(e) => {
                self.layout_root(e.index.as_ref(), &e.radicand, style)
            }

            Content::MathDelimited(e) => {
                self.layout_delimited(e.open, &e.body, e.close, style)
            }

            Content::MathMatrix(e) => self.layout_matrix(&e.rows, e.delim, style),

            Content::MathCases(e) => self.layout_cases(&e.rows, style),

            // P296 — Math accent/cancel handlers dedicados.
            Content::MathAccent(e) => self.layout_accent(&e.base, &e.accent, style),

            Content::MathCancel(e) => self.layout_cancel(&e.body, style),

            // P772y — `math.class(class, body)`: override de classe afecta
            // apenas espaçamento (spacing.rs); o layout do body é normal.
            Content::MathClassOverride(e) => self.layout_node(&e.body, style),

            // P297 — Math underover (paralelo P296 layout_accent/cancel).
            Content::MathUnderover(e) => {
                self.layout_underover(&e.base, e.under.as_ref(), e.over.as_ref(), style)
            }

            // P298 — Math op (trivial delegate; limits flag consumido em layout_attach).
            Content::MathOp(e) => self.layout_op(&e.text, style),

            // P311b.4 — Math style wrapper: aplica map_glyph + size factor.
            // Composição outer-wins é resolvida por `apply_math_style` que
            // funde MathStyled aninhados via Option::or (outer set ganha).
            Content::MathStyled(m) => {
                // Modelo D (P316): MathStyled delegado; re-bind dos campos.
                let kind = &m.kind;
                let bold = &m.bold;
                let italic = &m.italic;
                let body = &m.body;
                let transformed = apply_math_style(body, *kind, *bold, *italic);
                let size_factor = kind
                    .filter(|k| k.is_size_variant())
                    .map(|k| k.size_factor())
                    .unwrap_or(1.0);
                let mut math_style = style.clone();
                math_style.size = style.size * size_factor;
                // Se kind glyph foi aplicado (chars já variant-encoded) OU
                // italic explícito foi set, suprimir auto-itálico do
                // `MathIdent` handler. Itálico explícito é honrado via
                // codepoint já transformado (Italic plane).
                if kind.is_some() || italic.is_some() || bold.is_some() {
                    math_style.italic = false;
                }
                self.layout_node(&transformed, &math_style)
            }

            // **P895** — espaçamentos nomeados de modo math (`thin`/`med`/
            // `thick`/`quad`/`wide`, registados em `make_math_module()` como
            // `Content::HSpace`) precisam de contribuir a largura real ao
            // `MathBox`. Sem este arm, `HSpace` caía no catch-all abaixo
            // (`other.plain_text()` — `HSpace` não tem texto, `width: 0.0`),
            // produzindo o mesmo resultado (nenhum espaço) para os 5 nomes.
            // Só `Spacing::Absolute` é resolvido — `Fractional` (`1fr`) não
            // tem "espaço restante" bem definido dentro de uma sequência
            // math de largura própria; fica scope-out (largura 0), caso não
            // exercitado pelos 5 nomes registados (todos `Absolute`).
            Content::HSpace(e) => {
                let width = match &e.amount {
                    crate::entities::elements::h_space::Spacing::Absolute(len) => {
                        len.resolve_pt(style.size.val())
                    }
                    crate::entities::elements::h_space::Spacing::Fractional(_) => 0.0,
                };
                MathBox { width, ascent: 0.0, descent: 0.0, items: vec![] }
            }

            other => {
                let text: EcoString = other.plain_text().into();
                if text.trim().is_empty() {
                    MathBox {
                        width: 0.0,
                        ascent: 0.0,
                        descent: 0.0,
                        items: vec![],
                    }
                } else {
                    self.layout_text_node(&text, style)
                }
            }
        }
    }

    // ── Passo 296 — Math accent + cancel handlers ─────────────────────────
    //
    // **A.0.0 N=4 (HIV)**: features ausentes apesar de Tabela A.4
    // marcar `parcial`. P296 materializa from-scratch via variants
    // novos + handlers minimal.
    //
    // ADR-0098 honrada N=13 — hash export.rs preservado bit-exact
    // (math layout emite FrameItem standard).

    /// **P906** — guard partilhado por `layout_underover`/`layout_accent`:
    /// se `c` é `Content::MathText(s)` com exactamente 1 carácter, estica-o
    /// no eixo X via `layout_stretchy_glyph_horizontal` para cobrir
    /// `min_width_du`; para qualquer outro conteúdo (multi-carácter,
    /// sequência, etc.) comportamento inalterado (`layout_node`). Ver
    /// `math/layout/_comum.md` §P906.
    fn layout_stretchy_or_node(
        &self,
        c: &Content,
        min_width_du: f64,
        style: &TextStyle,
    ) -> MathBox {
        if let Content::MathText(s) = c {
            if s.chars().count() == 1 {
                let ch = s.chars().next().unwrap();
                return self.layout_stretchy_glyph_horizontal(ch, min_width_du, style);
            }
        }
        self.layout_node(c, style)
    }

    /// **P296** — Posiciona `accent` glyph centrado horizontalmente
    /// acima de `base`. Heurística minimal per ADR-0054 graded:
    /// - Sem `dotless` (i/j) handling — base mantém glyph original.
    /// - Sem `size` ratio — accent é width natural.
    fn layout_accent(
        &self,
        base: &Content,
        accent: &Content,
        style: &TextStyle,
    ) -> MathBox {
        let base_box = self.layout_node(base, style);
        // **P906** — accent de 1 carácter estica para cobrir `base_box.width`
        // (ver `math/layout/_comum.md` §P906). Multi-carácter: inalterado.
        let min_width_du =
            base_box.width * self.constants.upem / style.size.val().max(0.001);
        let accent_box = self.layout_stretchy_or_node(accent, min_width_du, style);
        // Centrar accent horizontalmente. dx é deslocamento do accent
        // para alinhar centro do accent com centro da base.
        let dx = (base_box.width - accent_box.width) / 2.0;
        let accent_h = accent_box.height();
        let new_ascent = base_box.ascent + accent_h;
        // **P906** — convenção baseline-relativa (mesmo achado/correcção já
        // aplicado a `frac.rs` P905, `root.rs` P901 e `layout_underover`
        // acima, neste mesmo passo): `local_y=0` é a BASELINE PRÓPRIA desta
        // `MathBox`. A versão anterior posicionava accent/base por offsets
        // topo-relativos — funcionava por coincidência no topo da equação
        // (P899 Parte A), quebrava se esta caixa fosse usada como sub-caixa
        // de outra (`hconcat_spaced`, ou aninhada como base de outro
        // `MathUnderover`/`MathAccent`). `ascent`/`descent` já correctos —
        // só os offsets dos items mudam.
        let mut items: Vec<FrameItem> = Vec::new();
        // Base: a sua própria baseline já é `local_y=0` — sem deslocamento.
        for item in base_box.items {
            items.push(offset_item(item, Pt(0.0), Pt(0.0)));
        }
        // Accent: a sua baseline própria sobe o suficiente para que o seu
        // descent pare exactamente no topo da tinta da base.
        let accent_y = -(base_box.ascent + accent_box.descent);
        for item in accent_box.items {
            items.push(offset_item(item, Pt(dx), Pt(accent_y)));
        }
        MathBox {
            width: base_box.width.max(accent_box.width),
            ascent: new_ascent,
            descent: base_box.descent,
            items,
        }
    }

    /// **P297** — Layout underover: empilha `over` (topo), `base`
    /// (meio), `under` (fundo). Cada Option é skipped se `None`.
    /// Width final = max das 3 partes; cada parte centrada
    /// horizontalmente. Agregação cristalina per ADR-0054 graded —
    /// vanilla typst fragmenta em 12 elementos (underbrace/overbrace/
    /// underbracket/etc.); cristalino unifica num único variant.
    fn layout_underover(
        &self,
        base: &Content,
        under: Option<&Content>,
        over: Option<&Content>,
        style: &TextStyle,
    ) -> MathBox {
        let base_box = self.layout_node(base, style);
        // **P906** — over/under de 1 carácter esticam para cobrir
        // `base_box.width` (ver `math/layout/_comum.md` §P906). Anotação
        // multi-carácter (`underbrace`/`overbrace`) fica inalterada.
        let min_width_du =
            base_box.width * self.constants.upem / style.size.val().max(0.001);
        let over_box = over.map(|c| self.layout_stretchy_or_node(c, min_width_du, style));
        let under_box = under.map(|c| self.layout_stretchy_or_node(c, min_width_du, style));

        let over_w = over_box.as_ref().map(|b| b.width).unwrap_or(0.0);
        let under_w = under_box.as_ref().map(|b| b.width).unwrap_or(0.0);
        let w = base_box.width.max(over_w).max(under_w);

        let over_h = over_box.as_ref().map(|b| b.height()).unwrap_or(0.0);
        let under_h = under_box.as_ref().map(|b| b.height()).unwrap_or(0.0);

        // **P906** — convenção baseline-relativa (mesmo achado/correcção já
        // aplicado a `frac.rs` em P905 e `root.rs` em P901, nunca antes
        // auditado aqui): `local_y=0` é a BASELINE PRÓPRIA desta `MathBox`,
        // não o topo. A versão anterior posicionava `over`/`base`/`under`
        // por offsets crescentes a partir de `Pt(0.0)` (topo-relativo) —
        // funcionava por coincidência quando `MathUnderover` era o conteúdo
        // de TOPO da equação (P899 Parte A, `hat(a)`), porque `place()` no
        // topo cancela o termo `-ascent` independentemente da convenção
        // interna — mas quebrava assim que esta caixa fosse usada como
        // `base` de outra `MathUnderover` (caso de `underbrace`/`overbrace`
        // COM anotação, `eval.md` §P906: aninhamento de 2 níveis) ou
        // concatenada com irmãs via `hconcat_spaced`. `ascent`/`descent`
        // (valores escalares) já estavam correctos — só os offsets dos
        // items estavam errados.
        let mut items: Vec<FrameItem> = Vec::new();
        // Base: a sua própria baseline já é `local_y=0` — sem deslocamento.
        let base_dx = (w - base_box.width) / 2.0;
        for item in base_box.items {
            items.push(offset_item(item, Pt(base_dx), Pt(0.0)));
        }
        // Over (topo): a sua baseline própria sobe o suficiente para que o
        // seu descent pare exactamente no topo da tinta da base.
        if let Some(ob) = over_box {
            let dx = (w - ob.width) / 2.0;
            let over_y = -(base_box.ascent + ob.descent);
            for item in ob.items {
                items.push(offset_item(item, Pt(dx), Pt(over_y)));
            }
        }
        // Under (fundo): a sua baseline própria desce o suficiente para que
        // o seu ascent pare exactamente no fundo da tinta da base.
        if let Some(ub) = under_box {
            let dx = (w - ub.width) / 2.0;
            let under_y = base_box.descent + ub.ascent;
            for item in ub.items {
                items.push(offset_item(item, Pt(dx), Pt(under_y)));
            }
        }

        MathBox {
            width: w,
            ascent: base_box.ascent + over_h,
            descent: base_box.descent + under_h,
            items,
        }
    }

    /// **P298** — Layout op: emite text como math child standard.
    /// Handler trivial (delegate) — verdadeira lógica está em
    /// `layout_attach` que detecta `Content::MathOp { limits: true, .. }`
    /// como base e renderiza scripts em limits-style.
    fn layout_op(&self, text: &Content, style: &TextStyle) -> MathBox {
        self.layout_node(text, style)
    }

    /// **P296** — Layout cancel: body + linha diagonal sobre bbox.
    /// Heurística minimal per ADR-0054 graded:
    /// - Diagonal default (bottom-left → top-right; "rising"
    ///   per vanilla angle padrão).
    /// - Sem `inverted`/`cross`/`angle`/`stroke` cosméticos —
    ///   scope-out frente futura P296.X.
    fn layout_cancel(&self, body: &Content, style: &TextStyle) -> MathBox {
        let body_box = self.layout_node(body, style);
        let h = body_box.height();
        // Linha diagonal de canto inferior-esquerdo (0, h) a canto
        // superior-direito (width, 0). Local coords relativos a topo
        // do MathBox.
        let line = FrameItem::Line {
            start: Point { x: Pt(0.0), y: Pt(h) },
            end: Point { x: Pt(body_box.width), y: Pt(0.0) },
            thickness: 0.5,
            color: None,
        };
        let mut items = body_box.items;
        items.push(line);
        MathBox {
            width: body_box.width,
            ascent: body_box.ascent,
            descent: body_box.descent,
            items,
        }
    }

    /// Nó folha: texto com métricas tipográficas.
    ///
    /// Posição do item dentro do MathBox: `(0, 0)` — relativo ao topo esquerdo.
    pub(super) fn layout_text_node(
        &self,
        text: &EcoString,
        style: &TextStyle,
    ) -> MathBox {
        if text.is_empty() {
            return MathBox {
                width: 0.0,
                ascent: 0.0,
                descent: 0.0,
                items: vec![],
            };
        }
        let width = self.metrics.advance(text, style.size, style).val();
        let vm = self.metrics.vertical_metrics(style.size, style);
        let ascent = vm.0.val();
        let descent = (vm.1 - vm.0).val();
        MathBox {
            width,
            ascent,
            descent,
            items: vec![FrameItem::Text {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                text: text.clone(),
                style: style.clone(),
            }],
        }
    }

    fn layout_sequence(&self, nodes: &[Content], style: &TextStyle) -> MathBox {
        if self.block && needs_grid_layout(nodes) {
            self.layout_grid(nodes, style)
        } else {
            let filtered: Vec<Content> = nodes
                .iter()
                .filter(|n| {
                    !matches!(n, Content::MathAlignPoint(_) | Content::Linebreak(_))
                })
                .cloned()
                .collect();
            // P772y — espaçamento automático por MathClass entre nós
            // adjacentes (paridade `process.rs::spacing()`, vanilla).
            // P891 — `style.math_script` suprime toda a regra quando a
            // sequência inteira está em script size.
            // P903 — largura real de um espaço de texto (medida via
            // `FontMetrics`, não hardcoded) para o fallback de "item
            // espaçado" (texto literal entre aspas) de `compute_gaps`.
            let text_space_pt = self.metrics.advance(" ", style.size, style).val();
            let gaps = spacing::compute_gaps(
                &filtered,
                style.size.val(),
                style.math_script,
                text_space_pt,
            );
            let boxes: Vec<MathBox> =
                filtered.iter().map(|n| self.layout_node(n, style)).collect();
            self.hconcat_spaced(boxes, &gaps)
        }
    }

    /// Grelha 2D generalizada — usada por `layout_grid` e `MathMatrix`.
    ///
    /// Duas passagens:
    ///   1. Mede todas as células → largura máxima por coluna.
    ///   2. Posiciona com essas larguras + `column_gap` entre colunas.
    pub(super) fn layout_grid_rows(
        &self,
        rows: &[Vec<Content>],
        align: GridAlign,
        column_gap: Pt,
        style: &TextStyle,
    ) -> MathBox {
        let n_cols = rows.iter().map(|row| row.len()).max().unwrap_or(0);
        // ── Passagem 1: medir todas as células ────────────────────────────
        let grid_boxes: Vec<Vec<MathBox>> = rows
            .iter()
            .map(|row| row.iter().map(|cell| self.layout_node(cell, style)).collect())
            .collect();
        let _ = n_cols;
        self.layout_grid_boxes(grid_boxes, align, column_gap, &[], style)
    }

    /// **P825** — passagem 2 de `layout_grid_rows` (posicionamento de uma
    /// grelha já medida), extraída para que `matrix.rs` possa ajustar as
    /// `MathBox` medidas antes de posicionar (incorporar o espaçamento de
    /// classe do limite `&` na largura da célula par — paridade vanilla
    /// `run.rs:76-94`).
    ///
    /// `align_boundaries[r][i] == true` marca que o limite **antes** da
    /// célula `i` da linha `r` foi produzido por um `&` — esse limite não
    /// leva `column_gap` (o espaçamento já está na largura da célula à
    /// esquerda). Slice vazio = nenhum limite de alinhamento.
    pub(super) fn layout_grid_boxes(
        &self,
        grid_boxes: Vec<Vec<MathBox>>,
        align: GridAlign,
        column_gap: Pt,
        align_boundaries: &[Vec<bool>],
        style: &TextStyle,
    ) -> MathBox {
        let n_cols = grid_boxes.iter().map(|row| row.len()).max().unwrap_or(0);
        if n_cols == 0 {
            return MathBox {
                width: 0.0,
                ascent: 0.0,
                descent: 0.0,
                items: vec![],
            };
        }

        let mut col_widths = vec![0.0_f64; n_cols];
        for row in &grid_boxes {
            for (col_idx, cell_box) in row.iter().enumerate() {
                col_widths[col_idx] = col_widths[col_idx].max(cell_box.width);
            }
        }

        // ── Passagem 2: posicionar células ────────────────────────────────
        let mut all_items: Vec<FrameItem> = Vec::new();
        let mut baseline_offset = 0.0_f64;
        let gap = column_gap.val();

        let total_ascent = grid_boxes
            .first()
            .map(|row| row.iter().map(|b| b.ascent).fold(0.0, f64::max))
            .unwrap_or(0.0);
        let mut total_descent = grid_boxes
            .first()
            .map(|row| row.iter().map(|b| b.descent).fold(0.0, f64::max))
            .unwrap_or(0.0);

        let mut max_row_width = 0.0_f64;
        for (row_idx, row) in grid_boxes.iter().enumerate() {            let row_ascent = row.iter().map(|b| b.ascent).fold(0.0, f64::max);
            let row_descent = row.iter().map(|b| b.descent).fold(0.0, f64::max);

            let mut cursor_x = 0.0_f64;
            for (col_idx, cell_box) in row.iter().enumerate() {
                let col_w = if col_idx < n_cols { col_widths[col_idx] } else { 0.0 };

                let cell_x = match align {
                    GridAlign::Alternating => {
                        if col_idx % 2 == 0 {
                            cursor_x + (col_w - cell_box.width) // par: à direita
                        } else {
                            cursor_x // ímpar: à esquerda
                        }
                    }
                    GridAlign::Center => cursor_x + (col_w - cell_box.width) / 2.0,
                    GridAlign::Left => cursor_x,
                };

                let dy = baseline_offset - row_ascent;
                for item in cell_box.items.clone() {
                    all_items.push(offset_item(item, Pt(cell_x), Pt(dy)));
                }

                cursor_x += col_w;
                if col_idx + 1 < n_cols {
                    // P825 — limites produzidos por `&` não levam
                    // `column_gap` (o espaçamento de classe já foi
                    // incorporado na largura da célula par).
                    let is_align_boundary = align_boundaries
                        .get(row_idx)
                        .and_then(|marks| marks.get(col_idx + 1))
                        .copied()
                        .unwrap_or(false);
                    if !is_align_boundary {
                        cursor_x += gap;
                    }
                }
            }
            max_row_width = max_row_width.max(cursor_x);

            if row_idx + 1 < grid_boxes.len() {
                let line_gap =
                    self.constants.to_pt(self.constants.math_leading, style.size).val();
                let advance = row_descent + line_gap + {
                    let next_row = &grid_boxes[row_idx + 1];
                    next_row.iter().map(|b| b.ascent).fold(0.0, f64::max)
                };
                baseline_offset += advance;
                total_descent += row_descent
                    + line_gap
                    + grid_boxes[row_idx + 1]
                        .iter()
                        .map(|b| b.ascent + b.descent)
                        .fold(0.0, f64::max);
            }
        }

        // P825 — sem limites de alinhamento, a largura é a fórmula
        // clássica (colunas + gap fixo), preservando o comportamento
        // anterior para grelhas irregulares; com limites `&`, é a maior
        // largura de linha real (os gaps variam por limite).
        let total_width = if align_boundaries.is_empty() {
            col_widths.iter().sum::<f64>() + n_cols.saturating_sub(1) as f64 * gap
        } else {
            max_row_width
        };

        MathBox {
            width: total_width,
            ascent: total_ascent,
            descent: total_descent,
            items: all_items,
        }
    }

    /// Layout em grelha 2D para equações com `&` e `\\`.
    ///
    /// Chama `layout_grid_rows` com alinhamento alternado (colunas pares à
    /// direita, ímpares à esquerda) e sem espaço entre colunas.
    fn layout_grid(&self, nodes: &[Content], style: &TextStyle) -> MathBox {
        let grid = partition_grid(nodes);
        // Cada célula é Vec<Content> — envolver em MathSequence para layout_node.
        let rows: Vec<Vec<Content>> = grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell_nodes| Content::MathSequence(cell_nodes.into()))
                    .collect()
            })
            .collect();
        self.layout_grid_rows(&rows, GridAlign::Alternating, Pt(0.0), style)
    }

    /// Concatenação horizontal: posiciona MathBoxes lado a lado, sem
    /// espaçamento extra entre eles (abertura+corpo+fecho de delimitadores
    /// — a regra de classe já dá 0pt aí, ADR-0108 P772y).
    pub(super) fn hconcat(&self, boxes: Vec<MathBox>) -> MathBox {
        self.hconcat_spaced(boxes, &[])
    }

    /// Concatenação horizontal com espaçamento extra entre boxes
    /// consecutivas — `gaps[i]` é o espaço (pt) inserido entre a box `i` e
    /// a box `i + 1`. `gaps` mais curto que `boxes.len() - 1` trata os
    /// gaps em falta como 0 (P772y — tabela de espaçamento por MathClass).
    pub(super) fn hconcat_spaced(&self, boxes: Vec<MathBox>, gaps: &[f64]) -> MathBox {
        let mut x = 0.0_f64;
        let mut ascent = 0.0_f64;
        let mut descent = 0.0_f64;
        let mut items = Vec::new();

        for (i, b) in boxes.into_iter().enumerate() {
            if i > 0 {
                x += gaps.get(i - 1).copied().unwrap_or(0.0);
            }
            ascent = ascent.max(b.ascent);
            descent = descent.max(b.descent);
            for mut item in b.items {
                match item {
                    FrameItem::Text { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::TextShaped { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::Line { ref mut start, ref mut end, .. } => {
                        start.x = Pt(start.x.val() + x);
                        end.x = Pt(end.x.val() + x);
                    }
                    FrameItem::Glyph { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::Image { .. } => {} // imagens não ocorrem em contexto math
                    FrameItem::Shape { .. } => {} // formas não ocorrem em contexto math
                    FrameItem::Group { .. } => {} // grupos não ocorrem em contexto math
                    FrameItem::Link { .. } => {}  // links não ocorrem em contexto math
                }
                items.push(item);
            }
            x += b.width;
        }

        MathBox { width: x, ascent, descent, items }
    }
}

// ── Passo 311b.4 — `apply_math_style` helper ─────────────────────────────
//
// Aplica recursivamente o variant glyph + flags (bold/italic) aos chars
// do body. Composição: outer-wins via Option::or (outer set ganha sobre
// inner). Para sub-arvores não-`MathStyled` (MathFrac/MathSequence/etc.),
// propaga o contexto recursivamente.

/// **P812** — aplicação do itálico matemático **por defeito** (P809) no topo
/// de `layout_equation`. Percorre a árvore mapeando folhas de 1 carácter
/// (`is_math_italic_default`) para o codepoint math italic, mas **preserva
/// os nós `MathStyled` intocáveis** — a composição de estilos explícitos
/// (incluindo o default de itálico para `bold(x)`) corre depois no handler
/// `MathStyled` de `layout_node`, que também aplica o factor de tamanho.
/// (Substitui o uso P809 de `apply_math_style` no topo, que consumia o
/// wrapper e destruía display/script/sscript — P812-A.)
fn apply_math_default(body: &Content) -> Content {
    match body {
        // Wrapper explícito: não tocar — o handler dedicado trata-o.
        Content::MathStyled(_) => body.clone(),
        // Folhas: default de itálico por codepoint (P809).
        Content::MathIdent(name) => {
            let mut chars = name.chars();
            if matches!((chars.next(), chars.next()), (Some(c), None) if is_math_italic_default(c)) {
                let c = name.chars().next().unwrap();
                return Content::MathIdent(
                    map_glyph(c, MathStyleKind::Plain, false, true).into(),
                );
            }
            body.clone()
        }
        Content::MathText(text) => {
            let mut chars = text.chars();
            if matches!((chars.next(), chars.next()), (Some(c), None) if is_math_italic_default(c)) {
                let c = text.chars().next().unwrap();
                return Content::MathText(
                    map_glyph(c, MathStyleKind::Plain, false, true).into(),
                );
            }
            body.clone()
        }
        // Containers: recursão (mesma cobertura de `apply_math_style`).
        Content::MathSequence(seq) => {
            let new_seq: Vec<Content> = seq.iter().map(apply_math_default).collect();
            Content::MathSequence(Arc::from(new_seq))
        }
        Content::MathMatrix(e) => Content::math_matrix(
            e.rows
                .iter()
                .map(|row| row.iter().map(apply_math_default).collect())
                .collect(),
            e.delim,
        ),
        Content::MathCases(e) => Content::math_cases(
            e.rows
                .iter()
                .map(|row| row.iter().map(apply_math_default).collect())
                .collect(),
        ),
        Content::MathFrac(e) => Content::math_frac(
            apply_math_default(&e.num),
            apply_math_default(&e.den),
        ),
        Content::MathAttach(e) => Content::math_attach(
            apply_math_default(&e.base),
            e.tl.as_ref().map(apply_math_default),
            e.bl.as_ref().map(apply_math_default),
            e.sub.as_ref().map(apply_math_default),
            e.sup.as_ref().map(apply_math_default),
        ),
        Content::MathRoot(e) => Content::math_root(
            e.index.as_ref().map(apply_math_default),
            apply_math_default(&e.radicand),
        ),
        Content::MathDelimited(e) => {
            Content::math_delimited(e.open, apply_math_default(&e.body), e.close)
        }
        Content::MathOp(_) => body.clone(),
        other => other.clone(),
    }
}

fn apply_math_style(
    body: &Content,
    kind: Option<MathStyleKind>,
    bold: Option<bool>,
    italic: Option<bool>,
) -> Content {
    match body {
        // Composição: inner MathStyled é fundido com outer via Option::or.
        // Modelo D (P316): MathStyled delegado; campos via Arc<Elem>.
        // **P812** — os eixos são ortogonais no vanilla (tamanho × glifo):
        // outer size-variant + inner glyph-variant → o GLYPH do inner
        // prevalece (`script(bb(R))` → ℝ pequeno); ambos size-variants →
        // outer vence (regra P311b.4); caso contrário outer vence.
        Content::MathStyled(m) => {
            let eff_kind = match (kind, m.kind) {
                (Some(outer), Some(inner))
                    if outer.is_size_variant() && !inner.is_size_variant() =>
                {
                    Some(inner)
                }
                (outer, inner) => outer.or(inner),
            };
            apply_math_style(&m.body, eff_kind, bold.or(m.bold), italic.or(m.italic))
        }
        // Folha textual: aplica map_glyph char-by-char.
        Content::MathIdent(name) => {
            // **P812** — variants de tamanho (Display/Inline/Script/SScript)
            // não têm mapping de glifo próprio: tratam-se como Plain no eixo
            // glifo (o itálico por defeito atravessa o wrapper de tamanho —
            // `$script(x)$` → 𝑥, medido no vanilla).
            let k = kind.unwrap_or(MathStyleKind::Plain);
            let k_glyph = if k.is_size_variant() { MathStyleKind::Plain } else { k };
            let b = bold.unwrap_or(false);
            // **P809** — default de italic: `is_math_italic_default` para
            // folhas de 1 carácter (paridade codex `MathStyle::select`,
            // medida: `bold(x)` → bold-italic U+1D499). `italic: Some(_)`
            // explícito prevalece (upright continua plain).
            let i = italic.unwrap_or_else(|| {
                let mut chars = name.chars();
                matches!((chars.next(), chars.next()), (Some(c), None) if is_math_italic_default(c))
            });
            let new: EcoString = name
                .chars()
                .flat_map(|c| {
                    let mapped = map_glyph(c, k_glyph, b, i);
                    // P812-C — variation selector de cal/scr (codex: VS após
                    // cada letra latina mapeada).
                    match map_glyph_vs(c, k_glyph) {
                        Some(vs) => [mapped, vs],
                        None => [mapped, '\0'],
                    }
                })
                .filter(|c| *c != '\0')
                .collect();
            Content::MathIdent(new)
        }
        Content::MathText(text) => {
            let k = kind.unwrap_or(MathStyleKind::Plain);
            let k_glyph = if k.is_size_variant() { MathStyleKind::Plain } else { k };
            let b = bold.unwrap_or(false);
            let i = italic.unwrap_or_else(|| {
                let mut chars = text.chars();
                matches!((chars.next(), chars.next()), (Some(c), None) if is_math_italic_default(c))
            });
            let new: EcoString = text
                .chars()
                .flat_map(|c| {
                    let mapped = map_glyph(c, k_glyph, b, i);
                    match map_glyph_vs(c, k_glyph) {
                        Some(vs) => [mapped, vs],
                        None => [mapped, '\0'],
                    }
                })
                .filter(|c| *c != '\0')
                .collect();
            Content::MathText(new)
        }
        // Containers math: propaga context.
        Content::MathSequence(seq) => {
            let new_seq: Vec<Content> =
                seq.iter().map(|c| apply_math_style(c, kind, bold, italic)).collect();
            Content::MathSequence(Arc::from(new_seq))
        }
        // **P809** — matrix/cases: recursão por célula (estavam no braço
        // `other` — as células não eram estilizadas, ex.: a,b,c,d plain).
        Content::MathMatrix(e) => Content::math_matrix(
            e.rows
                .iter()
                .map(|row| {
                    row.iter().map(|c| apply_math_style(c, kind, bold, italic)).collect()
                })
                .collect(),
            e.delim,
        ),
        Content::MathCases(e) => Content::math_cases(
            e.rows
                .iter()
                .map(|row| {
                    row.iter().map(|c| apply_math_style(c, kind, bold, italic)).collect()
                })
                .collect(),
        ),
        // Modelo D (Lote 2 P317): destructuring de `Arc<…Elem>` + reconstrução
        // via construtor ergonómico — mesma lógica recursiva.
        Content::MathFrac(e) => Content::math_frac(
            apply_math_style(&e.num, kind, bold, italic),
            apply_math_style(&e.den, kind, bold, italic),
        ),
        Content::MathAttach(e) => Content::math_attach(
            apply_math_style(&e.base, kind, bold, italic),
            e.tl.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
            e.bl.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
            e.sub.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
            e.sup.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
        ),
        Content::MathRoot(e) => Content::math_root(
            e.index.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
            apply_math_style(&e.radicand, kind, bold, italic),
        ),
        Content::MathDelimited(e) => Content::math_delimited(
            e.open,
            apply_math_style(&e.body, kind, bold, italic),
            e.close,
        ),
        // `MathOp` (operadores texto) passa-through — variant não aplica.
        // Operadores como "sin"/"lim" mantêm aparência normal mesmo dentro
        // de `bb(...)` (paridade vanilla).
        Content::MathOp(_) => body.clone(),
        // Outros chars / variants não-math: clone literal.
        other => other.clone(),
    }
}

#[cfg(test)]
mod p311b_tests {
    use super::*;

    fn mk_ident(s: &str) -> Content {
        Content::MathIdent(s.into())
    }

    #[test]
    fn p311b4_apply_math_style_bb_substitutes_chars() {
        let body = mk_ident("x");
        let out = apply_math_style(&body, Some(MathStyleKind::DoubleStruck), None, None);
        match out {
            Content::MathIdent(s) => assert_eq!(s.as_str(), "\u{1D569}"),
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_bold_italic_orthogonal() {
        let body = mk_ident("x");
        let out = apply_math_style(&body, None, Some(true), Some(true));
        // Plain Bold Italic 'x' = U+1D499 (base U+1D468 + 26 + 23).
        match out {
            Content::MathIdent(s) => assert_eq!(s.as_str(), "\u{1D499}"),
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p809_apply_math_style_bold_compoe_italic_default() {
        // P809 — paridade vanilla medida: `$bold(x)$` → bold-ITALIC (U+1D499),
        // não bold upright. O default de italic em folhas de 1 carácter é
        // `is_math_italic_default` (codex `MathStyle::select`).
        let body = mk_ident("x");
        let out = apply_math_style(&body, None, Some(true), None);
        match out {
            Content::MathIdent(s) => assert_eq!(
                s.as_str(),
                "\u{1D499}",
                "bold(x) deve compor com o itálico por defeito"
            ),
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p809_apply_math_style_upright_continua_plain() {
        // Controlo: upright(x) com italic Some(false) explícito → plain.
        let body = mk_ident("x");
        let out = apply_math_style(&body, None, None, Some(false));
        match out {
            Content::MathIdent(s) => assert_eq!(s.as_str(), "x"),
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_bb_cal_outer_wins() {
        // bb(cal(x)) — outer Bb deve ganhar.
        let inner = Content::math_styled(
            Some(MathStyleKind::Chancery),
            None,
            None,
            mk_ident("x"),
            None,
        );
        let out = apply_math_style(&inner, Some(MathStyleKind::DoubleStruck), None, None);
        match out {
            Content::MathIdent(s) => {
                assert_eq!(s.as_str(), "\u{1D569}", "outer Bb deve ganhar")
            }
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_upright_italic_outer_wins() {
        // upright(italic(x)) — outer upright (italic=false) deve ganhar.
        let inner = Content::math_styled(None, None, Some(true), mk_ident("x"), None);
        let out = apply_math_style(&inner, None, None, Some(false));
        match out {
            Content::MathIdent(s) => {
                assert_eq!(s.as_str(), "x", "upright deve suprimir italic")
            }
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_bold_preserves_inner_bb() {
        // bold(bb(x)) — inner Bb preservado; outer bold ortogonal aplicado.
        let inner = Content::math_styled(
            Some(MathStyleKind::DoubleStruck),
            None,
            None,
            mk_ident("x"),
            None,
        );
        let out = apply_math_style(&inner, None, Some(true), None);
        // Bold Double-Struck small x: U+1D569 (DS x não tem variant bold no
        // plano; nossa tabela aplica DS base). Aceita qualquer variant
        // contendo DS via verificação parcial.
        match out {
            Content::MathIdent(s) => {
                let c = s.chars().next().unwrap();
                let u = c as u32;
                // Espera dentro do plano DS U+1D552-U+1D56B (lowercase) ou
                // similar; aceita variantes encoded.
                assert!(
                    u >= 0x1D552 && u <= 0x1D56B,
                    "esperado DS lowercase, obteve U+{:X}",
                    u
                );
            }
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_recurses_through_mathfrac() {
        let frac = Content::math_frac(mk_ident("a"), mk_ident("b"));
        let out = apply_math_style(&frac, Some(MathStyleKind::DoubleStruck), None, None);
        match out {
            Content::MathFrac(e) => {
                match (&e.num, &e.den) {
                    (Content::MathIdent(n), Content::MathIdent(d)) => {
                        assert_eq!(n.as_str(), "\u{1D552}"); // DS a
                        assert_eq!(d.as_str(), "\u{1D553}"); // DS b
                    }
                    other => panic!("esperado MathIdent/MathIdent, obteve {other:?}"),
                }
            }
            other => panic!("esperado MathFrac, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_size_variant_passthrough_glyph() {
        // script(x) — kind Script é size variant: não tem mapping de glifo
        // próprio, mas o eixo itálico é ortogonal (P812 — paridade vanilla
        // medida: `$script(x)$` → 𝑥 U+1D465). Antes de P812 a asserção era
        // passthrough plain "x" — incorrecta face ao vanilla.
        let out =
            apply_math_style(&mk_ident("x"), Some(MathStyleKind::Script), None, None);
        match out {
            Content::MathIdent(s) => assert_eq!(s.as_str(), "\u{1D465}"),
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_math_op_passthrough() {
        // bb(op("sin")) — operadores texto não devem receber variant.
        let op = Content::math_op(Content::text("sin"), false);
        let out = apply_math_style(&op, Some(MathStyleKind::DoubleStruck), None, None);
        match out {
            Content::MathOp(e) => {
                assert_eq!(e.text.plain_text(), "sin");
                assert!(!e.limits);
            }
            other => panic!("esperado MathOp, obteve {other:?}"),
        }
    }
}

#[cfg(test)]
mod tests;
