//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/_comum.md
//! @prompt-hash 89eef8b8
//! @layer L1
//! @updated 2026-08-10

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use std::sync::Arc;

use ecow::EcoString;

use super::symbols;
use crate::compiler::layout::{vanilla_defaults::PAR_LEADING, FontMetrics};
use crate::entities::{
    content::Content,
    layout_types::{Color, FrameItem, Length, MathSize, Point, Pt, TextStyle},
    math_constants::MathConstants,
    math_style::{is_math_italic_default, map_glyph, map_glyph_vs, MathStyleKind},
};

// Sub-métodos do layout matemático extraídos por fase (Passo 96.8, ADR-0037).
mod accent;
mod assembly;
mod attach;
mod cancel;
mod cases;
mod delimited;
mod frac;
mod matrix;
mod op;
mod root;
mod spacing;
mod stretchy;
mod underover;

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
                    FrameItem::Semantic { ref mut items, .. } => {
                        for child in items {
                            *child = offset_item(
                                child.clone(),
                                Pt(x_origin),
                                Pt(baseline_y - self.ascent),
                            );
                        }
                    }
                }
                item
            })
            .collect()
    }
}

/// **P813** — Extensão geométrica de uma equação laid out, em pontos:
/// largura total e limites de tinta acima/abaixo da baseline. Produzido
/// por `MathLayouter::layout_equation_measured`; consumido pelo layout de
/// equações de bloco (`compiler/layout/equation.rs`) para centragem
/// horizontal e espaçamento vertical (paridade vanilla — ver
/// `compiler/layout/equation.md`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquationExtent {
    /// Limite direito máximo dos items (`pos.x + advance`).
    pub width: f64,
    /// Tinta máxima acima da baseline (>= 0).
    pub ascent: f64,
    /// Tinta máxima abaixo da baseline (>= 0).
    pub descent: f64,
    /// Número de linhas do run matemático exterior.
    pub line_count: usize,
    /// Baseline da primeira linha relativa à baseline da equação.
    pub first_baseline: f64,
    /// Baseline da última linha relativa à baseline da equação.
    pub last_baseline: f64,
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
        FrameItem::Glyph { pos, glyph_id, x_advance, size, style, base_char } => {
            FrameItem::Glyph {
                pos: Point {
                    x: Pt(pos.x.val() + dx.val()),
                    y: Pt(pos.y.val() + dy.val()),
                },
                glyph_id,
                x_advance,
                size,
                style,
                base_char,
            }
        }
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
        FrameItem::Semantic { kind, placement, alt, items } => FrameItem::Semantic {
            kind,
            placement,
            alt,
            items: items.into_iter().map(|child| offset_item(child, dx, dy)).collect(),
        },
    }
}

/// **P994** — `true` se o conteúdo exige o `layout_external` (o
/// `ExternalItem`/`BoxItem` do vanilla, `ir/resolve.rs:228-230, 192-194`):
/// contém — possivelmente sob wrappers `Sequence`/`Styled` — uma equação
/// embutida (`$…$` dentro de `text()`/`box()`/…) ou um container de
/// layout (`Boxed`/`Align`/`Pad`/`Block`), que a realização math do
/// vanilla NÃO consegue representar como corrido de texto.
///
/// Todo o resto (folhas de texto/markup, `MathOp`, `MathStyled`, e
/// `Sequence`/`Styled` só com esses dentro — ex.: output de funções de
/// utilizador, árvore medida em P966) é realizável COMO math
/// (`ir/resolve.rs:127-146`, `resolve_into_self`) e fica no caminho de
/// texto baseline-alinhado do catch-all. Mediado na Fase B: rotear esse
/// conteúdo pelo `layout_external` centrava-o no eixo (~4.5pt fora da
/// baseline da matemática vizinha, `$ 9 & "dado" $`, `$ sin(x) $`) e
/// aninhava-o num `Group` — regressão real de posicionamento, não só de
/// estrutura de items.
fn needs_external_layout(content: &Content) -> bool {
    match content {
        Content::Equation(_)
        | Content::Boxed(_)
        | Content::Align(_)
        | Content::Pad(_)
        | Content::Block(_) => true,
        Content::Sequence(items) => items.iter().any(needs_external_layout),
        // **P1027** — `Styled` que carrega propriedades de texto que o catch-all
        // de math não consegue aplicar (ele chama `layout_text_node` com o
        // `TextStyle` da equação, descartando o delta do `Styled`) sobe para
        // `layout_external`, onde a cadeia reconstruída aplica os overrides.
        // Markup puro e `Styled` semanticamente vazio mantêm o caminho
        // baseline-alinhado protegido pela adenda 2 de P994.
        Content::Styled(body, styles) => {
            let d = styles.delta();
            d.size.is_some()
                || d.fill.is_some()
                || d.weight.is_some()
                || d.font.is_some()
                || d.tracking.is_some()
                || d.leading.is_some()
                || d.lang.is_some()
                || d.bold.is_some()
                || d.italic.is_some()
                || needs_external_layout(body)
        }
        _ => false, // neutro: N16[γ] — fallback aberto: nós não listados assumem rota de texto math; novas variantes de container/layout exigirão braço explícito sob evolução de AST
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
    /// **P893** — `style` propagado a `metrics.math_constants(style)`, para
    /// que `FallbackFontMetrics` resolva a face MATH activa da cadeia de
    /// fallback em vez de devolver sempre `MathConstants::fallback()`.
    pub fn new(metrics: &'a M, block: bool, style: &TextStyle) -> Self {
        let constants = metrics.math_constants(style);
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
        // rationale: P1064 Classe 1C — ponto médio de caixa alinhado ao eixo (axis_pt - (ascent - descent) / 2.0)
        let shift = axis_pt - (b.ascent - b.descent) / 2.0;
        b.ascent += shift;
        b.descent -= shift;
        // **P919** — bug de omissão: faltava deslocar `b.items` (os glifos
        // reais em Y) — só ascent/descent (metadados) eram ajustados. Sinal
        // `-shift` (não `+shift`): o midpoint do box pré-offset
        // (`(descent-ascent)/2` local) tem de aterrar em `-axis_pt` (eixo
        // fica ACIMA da baseline, y cresce para baixo); resolvendo
        // `y_mid + d = -axis_pt` dá `d = -shift`. Confirmado pelos ascent/
        // descent já ajustados acima (`ascent += shift` ⇔ o topo, que sobe
        // por `d`, fica `ascent_pre + shift` acima da nova baseline).
        b.items = b
            .items
            .into_iter()
            .map(|item| offset_item(item, Pt(0.0), Pt(-shift)))
            .collect();
        b
    }

    /// **P918** — Converte a altura de tinta de uma grelha (`ascent+descent`,
    /// margem de 10%, P912) para design units, para dimensionar o
    /// delimitador esticável que a envolve. Partilhado por `layout_cases` e
    /// `layout_matrix` — fórmula confirmada idêntica nos dois (ver
    /// `_comum.md` §P918).
    pub(super) fn grid_delim_target_du(
        &self,
        grid_box: &MathBox,
        style: &TextStyle,
    ) -> f64 {
        let grid_height_pt = (grid_box.ascent + grid_box.descent) * 1.1;
        if style.size.val() > 0.0 {
            grid_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        }
    }

    /// **P945** — descida de **um nível MathSize** do vanilla
    /// (`style_for_denominator` = `style_for_numerator` + `cramped`,
    /// `lab/typst-original/crates/typst-library/src/math/style.rs:343-363`),
    /// usando o campo `TextStyle::math_size` (`entities/layout_types.md`
    /// §P945):
    ///
    /// | `style.math_size` | novo `math_size` | factor sobre `style.size` |
    /// |---|---|---|
    /// | `Display` | `Text` | ×1.0 |
    /// | `Text` | `Script` | ×`script_percent_scale_down` |
    /// | `Script` | `ScriptScript` | ×`sscript/script` |
    /// | `ScriptScript` | `ScriptScript` | ×1.0 |
    ///
    /// `cramped: true` sempre (é o denominador). Consumidores: `matrix.rs`
    /// e `cases.rs` (células) — substitui o ×`script_percent_scale_down`
    /// incondicional de P923, medido correcto só em inline (Text→Script).
    /// Ver `_comum.md` §P945.
    pub(super) fn denominator_style(&self, style: &TextStyle) -> TextStyle {
        let (math_size, factor) = match style.math_size {
            MathSize::Display => (MathSize::Text, 1.0),
            MathSize::Text => {
                (MathSize::Script, self.constants.script_percent_scale_down)
            }
            MathSize::Script => (
                MathSize::ScriptScript,
                self.constants.script_script_percent_scale_down
                    / self.constants.script_percent_scale_down,
            ),
            MathSize::ScriptScript => (MathSize::ScriptScript, 1.0),
        };
        TextStyle {
            size: style.size * factor,
            math_size,
            cramped: true,
            ..style.clone()
        }
    }

    /// **P952** — descida de **um nível MathSize** do vanilla
    /// (`style_for_numerator`,
    /// `lab/typst-original/crates/typst-library/src/math/style.rs:343-350`),
    /// simétrico de `denominator_style` (§P945 acima) — mesma tabela:
    ///
    /// | `style.math_size` | novo `math_size` | factor sobre `style.size` |
    /// |---|---|---|
    /// | `Display` | `Text` | ×1.0 |
    /// | `Text` | `Script` | ×`script_percent_scale_down` |
    /// | `Script` | `ScriptScript` | ×`sscript/script` |
    /// | `ScriptScript` | `ScriptScript` | ×1.0 |
    ///
    /// **`cramped` NÃO é forçado** — herda-o do ambiente (o vanilla
    /// `style_for_numerator` não aplica `style_cramped()`; só o denominador
    /// o faz). Consumidor: `frac.rs` (numerador — P952), em substituição do
    /// ×`script_percent_scale_down` incondicional de P915, medido correcto
    /// só em inline (Text→Script). Ver `_comum.md` §"numerator_style
    /// (novo helper, P952)" e `frac.md` §P952.
    pub(super) fn numerator_style(&self, style: &TextStyle) -> TextStyle {
        let (math_size, factor) = match style.math_size {
            MathSize::Display => (MathSize::Text, 1.0),
            MathSize::Text => {
                (MathSize::Script, self.constants.script_percent_scale_down)
            }
            MathSize::Script => (
                MathSize::ScriptScript,
                self.constants.script_script_percent_scale_down
                    / self.constants.script_percent_scale_down,
            ),
            MathSize::ScriptScript => (MathSize::ScriptScript, 1.0),
        };
        TextStyle {
            size: style.size * factor,
            math_size,
            ..style.clone()
        }
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
    /// Consumidor: `compiler/layout/equation.rs` (centragem + espaçamento
    /// vertical de equações de bloco — P813).
    pub fn layout_equation_measured(
        &self,
        body: &Content,
        style: &TextStyle,
    ) -> (Vec<FrameItem>, EquationExtent) {
        let transformed = apply_math_default(body);
        let (math_box, line_baselines) = match &transformed {
            Content::MathSequence(nodes) if self.block && needs_grid_layout(nodes) => {
                self.layout_grid_with_baselines(nodes, style)
            }
            _ => (self.layout_node(&transformed, style), vec![0.0]),
        };
        let baseline_y = math_box.ascent;
        let extent = EquationExtent {
            width: math_box.width,
            ascent: math_box.ascent,
            descent: math_box.descent,
            line_count: line_baselines.len().max(1),
            first_baseline: line_baselines.first().copied().unwrap_or(0.0),
            last_baseline: line_baselines.last().copied().unwrap_or(0.0),
        };
        let items = math_box.place(0.0, baseline_y);
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
                let text_style = TextStyle { italic: false, ..style.clone() };
                // **P952** — operador grande (classe `Large`) de 1 carácter
                // em Display: estica via variante vertical (alvo
                // `display_operator_min_height`, sem short_fall). Inline e
                // scripts mantêm o glifo base (`_comum.md` §P952).
                if name.chars().count() == 1 {
                    let c = name.chars().next().unwrap();
                    if symbols::is_large_operator(c) {
                        return self.layout_large_operator_display(c, &text_style);
                    }
                }
                self.layout_text_node(name, &text_style)
            }
            Content::MathText(text) => {
                // **P952** — mesmo guard do braço `MathIdent` acima: o lexer
                // pode entregar `∑`/`∫` por qualquer um dos dois braços.
                if text.chars().count() == 1 {
                    let c = text.chars().next().unwrap();
                    if symbols::is_large_operator(c) {
                        return self.layout_large_operator_display(c, style);
                    }
                }
                self.layout_text_node(text, style)
            }

            Content::MathSequence(nodes) => self.layout_sequence(nodes, style),

            // Modelo D (Lote 2 P317): destructuring de `Arc<…Elem>` — mesma lógica.
            Content::MathFrac(e) => {
                self.layout_frac_with_line(&e.num, &e.den, e.line, style)
            }

            Content::MathAttach(e) => self.layout_attach(
                &e.base,
                e.t.as_ref(),
                e.b.as_ref(),
                e.tl.as_ref(),
                e.bl.as_ref(),
                e.tr.as_ref(),
                e.br.as_ref(),
                style,
            ),

            Content::MathRoot(e) => {
                self.layout_root(e.index.as_ref(), &e.radicand, style)
            }

            Content::MathDelimited(e) => {
                self.layout_delimited(e.open, &e.body, e.close, style)
            }

            Content::MathMatrix(e) => self.layout_matrix(
                &e.rows,
                e.delim,
                e.row_gap,
                e.column_gap,
                e.gap,
                e.augment,
                style,
            ),

            Content::MathCases(e) => {
                self.layout_cases(&e.rows, e.delim, e.reverse, e.gap, style)
            }

            // P296 — Math accent/cancel handlers dedicados.
            Content::MathAccent(e) => self.layout_accent(&e.base, &e.accent, style),

            Content::MathCancel(e) => self.layout_cancel(&e.body, style),

            // P772y — `math.class(class, body)`: override de classe afecta
            // apenas espaçamento (spacing.rs); o layout do body é normal.
            Content::MathClassOverride(e) => {
                let mut box_ = self.layout_node(&e.body, style);
                // **P1132w** — ao transformar o GlyphItem num frame com
                // classe explícita, o vanilla conserva a IC no avanço. O
                // wrapper cristalino era totalmente transparente e perdia-a.
                if let Some(c) = single_math_leaf_char(&e.body) {
                    let extended = !self
                        .metrics
                        .vertical_glyph_variants(c, style)
                        .is_empty()
                        || !self.metrics.vertical_glyph_assembly(c, style).is_empty()
                        || !self.metrics.horizontal_glyph_variants(c, style).is_empty()
                        || !self.metrics.horizontal_glyph_assembly(c, style).is_empty();
                    if !extended {
                        box_.width += self
                            .metrics
                            .char_italics_correction(c, style.size, style)
                            .val();
                    }
                }
                box_
            }

            // P992 — `limits(body)`/`scripts(body)`: override afecta só o
            // discriminador `is_limits` de `layout_attach`; o layout do
            // body é normal (transparente, mesmo padrão de MathClassOverride).
            Content::MathLimitsOverride(e) => self.layout_node(&e.body, style),

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
                if let Some(k) = kind {
                    use crate::entities::layout_types::MathSize;
                    use crate::entities::math_style::MathStyleKind;
                    match k {
                        MathStyleKind::Display => {
                            math_style.math_size = MathSize::Display;
                        }
                        MathStyleKind::Inline => {
                            math_style.math_size = MathSize::Text;
                        }
                        MathStyleKind::Script => {
                            math_style.math_size = MathSize::Script;
                        }
                        MathStyleKind::SScript => {
                            math_style.math_size = MathSize::ScriptScript;
                        }
                        _ => {}
                    }
                }
                if let Some(c) = m.cramped {
                    math_style.cramped = c;
                }
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

            // **P990-C/P1132q** — `strike(...)` em contexto math: sem este arm,
            // `Content::Strike` caía no catch-all `plain_text()` abaixo —
            // perdia o itálico por defeito (o corpo nunca passava por
            // `layout_node` recursivo) E a linha (o catch-all não desenha
            // decorações). O corpo é layoutado como math normal (itálico já
            // aplicado por `apply_math_default`, braço adicionado em
            // conjunto). A medição P1132q retifica a hipótese da linha: o
            // vanilla converte strike em decoração de run textual, mas os
            // glifos math deste corpo não materializam essa decoração.
            // Preserva-se, portanto, somente o layout matemático do corpo.
            // Ver `_comum.md` §P990-C.
            Content::Strike(e) => self.layout_node(&e.body, style),
            Content::Overline(e) => {
                self.layout_overline(&e.body, e.stroke, e.offset, e.extent, style)
            }
            Content::Underline(e) => {
                self.layout_underline(&e.body, e.stroke, e.offset, e.extent, style)
            }

            // **P994** — o catch-all divide-se em dois (medição da Fase B,
            // registada no relatório do passo; ver `needs_external_layout`):
            //
            // - Conteúdo realizável como corrido de TEXTO math (markup e
            //   containers math sem equação/caixa dentro — corpo de
            //   `MathOp` como `sin`, strings `"dado"` em math, output de
            //   funções de utilizador — árvore medida em P966) mantém o
            //   caminho de texto baseline-alinhado: no vanilla este
            //   conteúdo é re-realizado COMO math
            //   (`ir/resolve.rs:127-146`, `resolve_into_self`), nunca vira
            //   `ExternalItem`. Mediado: pelo `layout_external` ficava
            //   centrado no eixo (deslocado ~4.5pt da baseline da
            //   matemática vizinha, `$ 9 & "dado" $`) e aninhado num
            //   `Group` — regressão real de posicionamento, não só de
            //   estrutura de items.
            // - Conteúdo com `Equation`/`Boxed`/`Align`/`Pad`/`Block` lá
            //   dentro — NÃO realizável como math — delega ao `Layouter`
            //   normal (`layout_external`, Opção β do `_comum.md` §P994):
            //   layoutado a sério num sub-frame isolado e os items
            //   ancorados na baseline da equação, em vez de achatados por
            //   `plain_text()` (que matava itálico, `^`/`_`, tamanhos e
            //   caixas — diagnóstico de P993).
            other if !needs_external_layout(other) => {
                let text: EcoString = other.plain_text().into();
                if text.trim().is_empty() {
                    MathBox {
                        width: 0.0,
                        ascent: 0.0,
                        descent: 0.0,
                        items: vec![],
                    }
                } else {
                    // `Content::Text` é `TextItem` no vanilla e passa pelo
                    // layout inline (shaping/kerning), ao contrário das
                    // folhas `MathText`/`MathIdent`, que são glifos math.
                    let mut text_box = self.layout_text_node(&text, style);
                    text_box.width =
                        self.metrics.text_width(&text, style.size, style).val();
                    text_box
                }
            }
            other => self.layout_external(other, style),
        }
    }

    /// **P994** — layout de conteúdo não-matemático embutido em math
    /// (equivalente ao `ExternalItem`/`layout_external` do vanilla,
    /// `ir/resolve.rs:228-230` → `typst-layout/src/math/mod.rs:585-603`).
    ///
    /// Constrói um `Layouter` temporário com as métricas REAIS da equação
    /// (`self.metrics` coagido a `&dyn FontMetrics` — `impl FontMetrics
    /// for &dyn FontMetrics`, `compiler/layout/metrics.rs:369`), cadeia de
    /// estilos reconstruída de `style` (é o que faz `#text(size: 20pt)`
    /// aplicar-se ao conteúdo embutido) e introspector vazio (mesma
    /// limitação registada de `measure()`: labels/links dentro de
    /// conteúdo externo em math não resolvem). Corre
    /// `layout_sub_frame` numa região ilimitada (precedente
    /// `measure_content_real`, `compiler/layout/mod.rs:2123-2158`), ancora o
    /// resultado na baseline da equação (passo 6 abaixo — paridade
    /// vanilla: a fórmula `height/2 + axis` só se aplica a frames SEM
    /// baseline declarada) e devolve os items ACHATADOS (sem wrapper
    /// `Group` — passo 7 abaixo; a convenção Y do export PDF para
    /// `Group` com filhos de texto é incoerente, medição Fase B.2).
    ///
    /// Guardas (aceites na adenda pós-Fase B do L0):
    /// - `items` vazio E `plain_text` vazio → caixa vazia (preserva o
    ///   comportamento do catch-all antigo para conteúdo vazio);
    /// - `items` vazio MAS `plain_text` não vazio → fallback ao antigo
    ///   `layout_text_node` (não perder texto silenciosamente se o
    ///   `Layouter` ignorar uma variante com texto).
    pub(super) fn layout_external(
        &self,
        content: &Content,
        style: &TextStyle,
    ) -> MathBox {
        use comemo::Track;

        use crate::compiler::layout::{Layouter, SubLayoutRegion};
        use crate::entities::image_sizer::NullImageSizer;
        use crate::entities::introspector::{Introspector, TagIntrospector};
        use crate::entities::style::{Style, Styles};
        use crate::entities::style_chain::StyleChain;

        // 1. Layouter temporário com as métricas reais (coerção &M → &dyn).
        let metrics_dyn: &dyn FontMetrics = self.metrics;
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let mut layouter = Layouter::new(
            metrics_dyn,
            NullImageSizer,
            style.size.val(),
            intr_dyn.track(),
        );

        // 2. Cadeia reconstruída dos campos de `TextStyle` com variante
        //    `Style` correspondente (os dois lados existem); `font`/`weight`
        //    propagam o show-set da equação (P944) — paridade vanilla, cujo
        //    `EquationElem::show_set` cobre o conteúdo embutido.
        let mut styles = vec![
            Style::Size(style.size),
            Style::bold(style.bold),
            Style::italic(style.italic),
        ];
        if let Some(c) = style.fill {
            styles.push(Style::Fill(c));
        }
        if let Some(h) = style.heading_level {
            styles.push(Style::HeadingLevel(h));
        }
        if let Some(w) = style.weight {
            styles.push(Style::Weight(w));
        }
        if let Some(t) = style.tracking {
            styles.push(Style::Tracking(t));
        }
        if let Some(l) = style.leading {
            styles.push(Style::Leading(l));
        }
        if let Some(l) = style.lang {
            styles.push(Style::Lang(l));
        }
        if let Some(f) = &style.font {
            styles.push(Style::Font(f.clone()));
        }
        layouter.chain =
            StyleChain::default_chain().push_styles(&Styles::from_iter(styles));
        layouter.style = style.clone();

        // 3. Sub-frame sem limites (largura infinita — decisão registada no
        //    L0: o vanilla usa `ctx.region`; em página auto o efeito
        //    coincide). `deco`/órfãos descartados com a instância (P908).
        let (height, items, _deco, _orphaned_x, _orphaned_y) = layouter.layout_sub_frame(
            content,
            SubLayoutRegion {
                origin_x: 0.0,
                width: f64::INFINITY,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
        );

        // 4. Guardas (ver doc acima).
        if items.is_empty() {
            let text: EcoString = content.plain_text().into();
            if text.trim().is_empty() {
                return MathBox {
                    width: 0.0,
                    ascent: 0.0,
                    descent: 0.0,
                    items: vec![],
                };
            }
            return self.layout_text_node(&text, style);
        }

        // 5. Largura = largura real do sub-frame (cursor advance) ou limite direito dos itens
        let items_width = items
            .iter()
            .map(|item| match item {
                FrameItem::Shape { pos, width, .. } => pos.x.val().max(0.0) + *width,
                other => {
                    let (x, _) = crate::compiler::layout::helpers::item_pos(other);
                    x + crate::compiler::layout::helpers::item_width(other, self.metrics)
                }
            })
            .fold(0.0, f64::max);
        let width = layouter.last_sub_frame_width.max(items_width);

        // 6. Âncora vertical (paridade vanilla `layout_external`,
        //    `typst-layout/src/math/mod.rs:585-603`): o vanilla só aplica
        //    `height/2 + axis` QUANDO o frame não declara baseline
        //    (`if !frame.has_baseline()`). Medição (Fase B.2): os items do
        //    sub-frame são baseline-ancorados (o `pos.y` de um `Text` é a
        //    baseline da linha) e o `height` devolvido é medido DESDE a
        //    primeira baseline (`sub_frame.rs`, `cell_height = end_y −
        //    start_y`) — a fórmula incondicional fazia o conteúdo flutuar
        //    ~10pt acima da baseline (`e1-text-size-italic.typ`). Com
        //    texto, a baseline interna (primeiro `Text`/`Glyph`, por ordem
        //    do documento) assenta na baseline da equação; sem texto,
        //    aplica-se a fórmula do vanilla sobre os extents de tinta.
        let mut first_baseline = None;
        let mut ink_top = None;
        let mut ink_bottom = None;
        self.scan_external_verticals(
            &items,
            0.0,
            &mut first_baseline,
            &mut ink_top,
            &mut ink_bottom,
        );
        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();
        let (anchor, ascent, descent) = match (first_baseline, ink_top, ink_bottom) {
            (Some(b), Some(top), Some(bot)) => {
                (b, (b - top).max(0.0), (bot - b).max(0.0))
            }
            (Some(b), Some(top), None) => (b, (b - top).max(0.0), (height - b).max(0.0)),
            (Some(b), None, Some(bot)) => (b, b, (bot - b).max(0.0)),
            (Some(b), None, None) => (b, b, (height - b).max(0.0)),
            // Sem texto: frame sem baseline declarada → fórmula do vanilla
            // sobre os extents reais (`H/2 + axis` medido do topo da tinta).
            (None, Some(top), Some(bot)) => {
                // rationale: P1064 Classe 1C — ponto médio de tinta ao eixo (top + (bot - top) / 2.0 + axis_pt)
                let a = top + (bot - top) / 2.0 + axis_pt;
                (a, a - top, (bot - a).max(0.0))
            }
            // Sem extents mensuráveis: fórmula original.
            _ => {
                // rationale: P1064 Classe 1C — meia-altura de frame ao eixo (height / 2.0 + axis_pt)
                let a = height / 2.0 + axis_pt;
                (a, a, (height / 2.0 - axis_pt).max(0.0))
            }
        };

        // 7. **Achatar** — os items entram DIRECTAMENTE na `MathBox`
        //    (transladados por `−anchor`), SEM wrapper `FrameItem::Group`.
        //    Medição Fase B.2: o caminho PDF de `Group` com filhos de texto
        //    tem a convenção de eixo Y incoerente (pré-existente — o texto
        //    de `box(height:, clip: true)` também desaparece do render; o
        //    `cm` de `stream.rs` não inverte Y para matrix identidade,
        //    enquanto o renderer raster assume filhos Y-down) — um texto
        //    dentro de um Group sai espelhado em Y (erro medido = 2×y_local,
        //    exactamente). Achatar faz o conteúdo embutido fluir pelos
        //    caminhos de emissão de topo (Text/Shape/Line/Glyph), provados
        //    por toda a renderização math. Grupos ANINHADOS dentro do
        //    conteúdo (ex.: clip de `block(clip:)`) passam intactos pelo
        //    `offset_item` — mesmo estado que têm fora de math.
        let items = items
            .into_iter()
            .map(|item| offset_item(item, Pt(0.0), Pt(-anchor)))
            .collect();

        MathBox { width, ascent, descent, items }
    }

    /// **P994** — varre os items de um sub-frame externo (recursivo em
    /// `Group`/`Link`, acumulando o offset Y) e reporta: a baseline do
    /// primeiro item de texto/glifo (por ordem do documento) e os extents
    /// de tinta vertical (topo/fundo), em coordenadas locais do sub-frame.
    /// Usado por `layout_external` para a âncora vertical (ver passo 6 lá).
    fn scan_external_verticals(
        &self,
        items: &[FrameItem],
        offset_y: f64,
        first_baseline: &mut Option<f64>,
        ink_top: &mut Option<f64>,
        ink_bottom: &mut Option<f64>,
    ) {
        fn grow(top: &mut Option<f64>, bottom: &mut Option<f64>, t: f64, b: f64) {
            *top = Some(top.map_or(t, |v: f64| v.min(t)));
            *bottom = Some(bottom.map_or(b, |v: f64| v.max(b)));
        }
        for item in items {
            match item {
                FrameItem::Text { pos, text, style }
                | FrameItem::TextShaped { pos, text, style, .. } => {
                    let y = offset_y + pos.y.val();
                    if first_baseline.is_none() {
                        *first_baseline = Some(y);
                    }
                    let (up, down) =
                        self.metrics.text_ink_bounds(text, style.size, style);
                    grow(ink_top, ink_bottom, y - up.val(), y + down.val());
                }
                FrameItem::Glyph { pos, size, .. } => {
                    let y = offset_y + pos.y.val();
                    if first_baseline.is_none() {
                        *first_baseline = Some(y);
                    }
                    let (top_edge, bottom_edge) =
                        self.metrics.text_edges(*size, &TextStyle::default());
                    grow(ink_top, ink_bottom, y - top_edge.0, y + bottom_edge.0.abs());
                }
                FrameItem::Shape { pos, height, .. } => {
                    // P1100: Shapes decorativos (como borda de box()) não devem
                    // inflar os limites de tinta de conteúdo que já possui texto/glifos com baseline.
                    if first_baseline.is_none() {
                        let t = offset_y + pos.y.val();
                        grow(ink_top, ink_bottom, t, t + height);
                    }
                }
                FrameItem::Line { start, end, thickness, .. } => {
                    // rationale: P1064 Classe 1B — semi-espessura de linha (thickness / 2.0)
                    let half = thickness / 2.0;
                    let y0 = offset_y + start.y.val();
                    let y1 = offset_y + end.y.val();
                    grow(ink_top, ink_bottom, y0.min(y1) - half, y0.max(y1) + half);
                }
                FrameItem::Image { pos, height, .. } => {
                    let t = offset_y + pos.y.val();
                    grow(ink_top, ink_bottom, t, t + height.val());
                }
                FrameItem::Group { pos, items, .. }
                | FrameItem::Link { pos, items, .. } => {
                    self.scan_external_verticals(
                        items,
                        offset_y + pos.y.val(),
                        first_baseline,
                        ink_top,
                        ink_bottom,
                    );
                }
                FrameItem::Semantic { items, .. } => {
                    self.scan_external_verticals(
                        items,
                        offset_y,
                        first_baseline,
                        ink_top,
                        ink_bottom,
                    );
                }
            }
        }
    }

    /// **P990-C** — `Content::Strike` em contexto math: layout do corpo
    /// (já com itálico por defeito, `apply_math_default`) + uma
    /// `FrameItem::Line` horizontal sobre a caixa. Mesma geometria do lado
    /// de texto (`compiler/layout/decorations.rs::layout`): offset por
    /// omissão `-0.25em`, thickness `max(0.05em, 0.4pt)`, extent simétrico
    /// nos dois lados. Convenção baseline-relativa (ADR-0123: `y=0` =
    /// baseline própria do `MathBox`, positivo para baixo) — `offset_pt`
    /// já vem negativo (acima da baseline) e aplica-se directamente, sem
    /// somar a nenhum `y` de base (ao contrário do lado de texto, que soma
    /// a `baseline_y` de cada linha). Ver `_comum.md` §P990-C.
    fn layout_overline(
        &self,
        body: &Content,
        stroke: Option<Color>,
        _offset: Option<Length>,
        _extent: Option<Length>,
        style: &TextStyle,
    ) -> MathBox {
        let cramped_style = TextStyle { cramped: true, ..style.clone() };
        let body_box = self.layout_node(body, &cramped_style);

        let size = style.size;
        let sep = self
            .constants
            .to_pt(self.constants.overbar_extra_ascender, size)
            .val();
        let thickness = self
            .constants
            .to_pt(self.constants.overbar_rule_thickness, size)
            .val();
        let gap = self.constants.to_pt(self.constants.overbar_vertical_gap, size).val();
        let extra_height = sep + thickness + gap;

        let width = body_box.width;
        let ascent = body_box.ascent + extra_height;
        let descent = body_box.descent;

        let line_y = -(body_box.ascent + gap + thickness / 2.0);
        let color = stroke.or(style.fill);

        let mut items = body_box.items;
        items.push(FrameItem::Line {
            start: Point { x: Pt(0.0), y: Pt(line_y) },
            end: Point { x: Pt(width), y: Pt(line_y) },
            thickness,
            color,
        });

        MathBox { width, ascent, descent, items }
    }

    fn layout_underline(
        &self,
        body: &Content,
        stroke: Option<Color>,
        _offset: Option<Length>,
        _extent: Option<Length>,
        style: &TextStyle,
    ) -> MathBox {
        let body_box = self.layout_node(body, style);

        let size = style.size;
        let sep = self
            .constants
            .to_pt(self.constants.underbar_extra_descender, size)
            .val();
        let thickness = self
            .constants
            .to_pt(self.constants.underbar_rule_thickness, size)
            .val();
        let gap = self.constants.to_pt(self.constants.underbar_vertical_gap, size).val();
        let extra_height = sep + thickness + gap;

        let width = body_box.width;
        let ascent = body_box.ascent;
        let descent = body_box.descent + extra_height;

        let line_y = body_box.descent + gap + thickness / 2.0;
        let color = stroke.or(style.fill);

        let mut items = body_box.items;
        items.push(FrameItem::Line {
            start: Point { x: Pt(0.0), y: Pt(line_y) },
            end: Point { x: Pt(width), y: Pt(line_y) },
            thickness,
            color,
        });

        MathBox { width, ascent, descent, items }
    }

    /// **P906** — guard partilhado por `layout_underover`/`layout_accent`
    /// (agora em `underover.rs`/`accent.rs`, ver P909): se `c` é
    /// `Content::MathText(s)` com exactamente 1 carácter, estica-o no eixo X
    /// via `layout_stretchy_glyph_horizontal` para cobrir `min_width_du`;
    /// para qualquer outro conteúdo (multi-carácter, sequência, etc.)
    /// comportamento inalterado (`layout_node`). Ver `math/layout/_comum.md`
    /// §P906. `pub(super)` (P909) — chamado de `accent.rs`/`underover.rs`,
    /// arquivos irmãos, não descendentes deste módulo.
    pub(super) fn layout_stretchy_or_node(
        &self,
        c: &Content,
        min_width_du: f64,
        style: &TextStyle,
        short_fall_em: f64,
    ) -> MathBox {
        if let Content::MathText(s) = c {
            if s.chars().count() == 1 {
                let ch = s.chars().next().unwrap();
                return self.layout_stretchy_glyph_horizontal(
                    ch,
                    min_width_du,
                    style,
                    short_fall_em,
                );
            }
        }
        self.layout_node(c, style)
    }

    /// **P952** — operador grande (`MathClass::Large`, ver
    /// `symbols::is_large_operator`) em Display: selecciona a primeira
    /// variante vertical com `advance >= display_operator_min_height`
    /// (design units) — **sem** `DELIM_SHORT_FALL` (o `StretchInfo::
    /// default()` do vanilla tem `short_fall = Em::zero()`), ao contrário
    /// de `layout_stretchy_delimiter` (P912). Integrais NÃO são excluídas
    /// (no vanilla são `Large` para o stretch; `is_integral_char` só impede
    /// empilhar limites).
    ///
    /// Emissão: `FrameItem::Glyph` com `x_advance`/`width` vindos de
    /// `hor_advance` (mesma disciplina P917 de `stretchy.rs` — `advance` é
    /// a medida do eixo de esticamento, nunca posiciona). `ascent`/`descent`
    /// vêm da **bbox de tinta real da variante** via
    /// `FontMetrics::glyph_ink_bounds` (paridade vanilla `update_glyph`,
    /// `fragment/glyph.rs:215-231`: `baseline = ascent`, `size = ascent +
    /// descent` da bbox) — **P959**: o split simétrico de P952
    /// (`advance/2` cada) dava extents errados da base aos shifts de
    /// limites de P959 (o sub ficava ~2.75pt abaixo do vanilla, medido no
    /// smoke end-to-end). O glifo é emitido na baseline do run
    /// (`pos.y = 0`), como o vanilla (que empurra o glifo na sua baseline
    /// de fonte). Sem variante >= alvo: glifo base (comportamento anterior,
    /// inalterado — SEM assembly nem máximo disponível). Ver
    /// `_comum.md` §P952 e §P959.
    pub(super) fn layout_large_operator_display(
        &self,
        c: char,
        style: &TextStyle,
    ) -> MathBox {
        let variants = self.metrics.vertical_glyph_variants(c, style);
        let target_du = self.constants.display_operator_min_height;

        if self.block {
            if let Some(picked) = variants.select_variant(target_du) {
                let x_advance = style.size * (picked.hor_advance / self.constants.upem);
                let (ink_up, ink_down) =
                    self.metrics.glyph_ink_bounds(picked.glyph_id, style.size, style);
                let b = MathBox {
                    width: x_advance.val(),
                    ascent: ink_up.val(),
                    descent: ink_down.val(),
                    items: vec![FrameItem::Glyph {
                        pos: Point { x: Pt(0.0), y: Pt(0.0) },
                        glyph_id: picked.glyph_id,
                        x_advance,
                        size: style.size,
                        style: style.clone(),
                        base_char: c,
                    }],
                };
                // **P1136** — operadores Large são sempre centrados no
                // eixo; a compensação interna preserva a tinta.
                return self.apply_axis_offset(b, style.size);
            }
        }

        // Inline/script, ou Display sem variante suficiente: glifo base.
        let text: EcoString = c.to_string().into();
        let b = self.layout_text_node(&text, style);
        self.apply_axis_offset(b, style.size)
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
        // **P1088** — `text_ink_bounds_signed` (P989) permite descent negativo para
        // glifos que flutuam acima da baseline (ex.: asterisco `*`, combining marks),
        // evitando falsa colisão no cálculo de gap de sub/sobrescrito (scripts.rs:464-466).
        let ib = self.metrics.text_ink_bounds_signed(text, style.size, style);
        let ascent = ib.0.val();
        let descent = ib.1.val();
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
            let mut flat = Vec::new();
            flatten_math_sequence_nodes(nodes, &mut flat);

            let mut filtered: Vec<Content> = flat
                .into_iter()
                .filter(|n| {
                    !matches!(n, Content::MathAlignPoint(_) | Content::Linebreak(_))
                })
                .collect();
            // **P1132m** — `HElem(..., weak:true)` colapsa nas bordas do
            // run. `dif` usa exactamente esse prefixo fino: preserva-o entre
            // dois itens, mas não cria margem em `$dif x$`.
            let is_weak_hspace = |n: &Content| matches!(n, Content::HSpace(e) if e.weak);
            let first_material = filtered.iter().position(|n| !is_weak_hspace(n));
            let last_material = filtered.iter().rposition(|n| !is_weak_hspace(n));
            if let (Some(first), Some(last)) = (first_material, last_material) {
                filtered = filtered
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, n)| {
                        (!is_weak_hspace(&n) || (i > first && i < last)).then_some(n)
                    })
                    .collect();
            } else {
                filtered.clear();
            }
            let text_space_pt = self.metrics.advance(" ", style.size, style).val();
            let gaps = spacing::compute_gaps(
                &filtered,
                style.size.val(),
                style.math_script
                    || matches!(
                        style.math_size,
                        MathSize::Script | MathSize::ScriptScript
                    ),
                text_space_pt,
            );
            let mut boxes: Vec<MathBox> =
                filtered.iter().map(|n| self.layout_node(n, style)).collect();
            for (i, (node, text_box)) in filtered.iter().zip(boxes.iter_mut()).enumerate()
            {
                if should_tighten_text_frame(node, filtered.get(i + 1)) {
                    text_box.width = (text_box.width - text_space_pt).max(0.0);
                }
            }
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
        row_gap: Pt,
        style: &TextStyle,
    ) -> MathBox {
        let n_cols = rows.iter().map(|row| row.len()).max().unwrap_or(0);
        // ── Passagem 1: medir todas as células ────────────────────────────
        let grid_boxes: Vec<Vec<MathBox>> = rows
            .iter()
            .map(|row| row.iter().map(|cell| self.layout_node(cell, style)).collect())
            .collect();
        let _ = n_cols;
        self.layout_grid_boxes(grid_boxes, align, column_gap, row_gap, &[], style)
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
        row_gap: Pt,
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

        // **P921/P923** — piso de altura por linha: cada linha fica no mínimo
        // tão alta quanto um `(` sintético em estilo de denominador (vanilla,
        // `table.rs:67-85`: "pad ascent/descent with the paren's, to ensure
        // that normal matrices are aligned with others unless they are way
        // too big"). O tamanho do `(` é o do estilo de denominador do ambiente
        // exterior. Quando `layout_grid_boxes` é chamado por matrizes/cases
        // (P923), o `style` recebido já é esse estilo de denominador
        // (**P945**: via `denominator_style` — descida por nível MathSize,
        // `_comum.md` §P945), pelo que não se aplica o factor outra vez. Para multiline math
        // (`layout_grid`), o `style` é o exterior e o piso continua a usar o
        // tamanho do denominador — impacto mínimo e consistente com a
        // intenção do piso.
        let paren_style = TextStyle { cramped: true, ..style.clone() };
        let paren_box = self.layout_text_node(&EcoString::from("("), &paren_style);
        let (paren_ascent, paren_descent) = (paren_box.ascent, paren_box.descent);

        // ── Passagem 2: posicionar células ────────────────────────────────
        let mut all_items: Vec<FrameItem> = Vec::new();
        let gap = column_gap.val();

        let total_ascent = grid_boxes
            .first()
            .map(|row| row.iter().map(|b| b.ascent).fold(0.0, f64::max).max(paren_ascent))
            .unwrap_or(0.0);
        let mut total_descent = grid_boxes
            .first()
            .map(|row| {
                row.iter().map(|b| b.descent).fold(0.0, f64::max).max(paren_descent)
            })
            .unwrap_or(0.0);

        let n_rows = grid_boxes.len();
        let mut row_baselines = vec![0.0_f64; n_rows];
        let mut cur_baseline = 0.0_f64;
        for row_idx in 0..n_rows {
            row_baselines[row_idx] = cur_baseline;
            if row_idx + 1 < n_rows {
                let row_descent = grid_boxes[row_idx]
                    .iter()
                    .map(|b| b.descent)
                    .fold(0.0, f64::max)
                    .max(paren_descent);
                let next_row = &grid_boxes[row_idx + 1];
                let next_row_ascent = next_row
                    .iter()
                    .map(|b| b.ascent)
                    .fold(0.0, f64::max)
                    .max(paren_ascent);
                let next_row_descent = next_row
                    .iter()
                    .map(|b| b.descent)
                    .fold(0.0, f64::max)
                    .max(paren_descent);
                let advance = row_descent + row_gap.val() + next_row_ascent;
                cur_baseline += advance;
                total_descent += row_gap.val() + next_row_ascent + next_row_descent;
            }
        }

        let mut col_offsets = vec![0.0_f64; n_cols];
        let mut cur_col_x = 0.0_f64;
        for col_idx in 0..n_cols {
            col_offsets[col_idx] = cur_col_x;
            cur_col_x += col_widths[col_idx];
            if col_idx + 1 < n_cols {
                cur_col_x += gap;
            }
        }
        let max_row_width = cur_col_x;

        if matches!(align, GridAlign::Center) {
            // **P1121**: Para matrizes matemáticas (GridAlign::Center), o vanilla emite
            // os nós por colunas (Coluna 0 inteira top-to-bottom, Coluna 1 top-to-bottom...).
            for col_idx in 0..n_cols {
                let col_w = col_widths[col_idx];
                let cursor_x = col_offsets[col_idx];
                for row_idx in 0..n_rows {
                    if let Some(cell_box) = grid_boxes[row_idx].get(col_idx) {
                        let cell_x = cursor_x + (col_w - cell_box.width) / 2.0;
                        let dy = row_baselines[row_idx];
                        for item in cell_box.items.clone() {
                            all_items.push(offset_item(item, Pt(cell_x), Pt(dy)));
                        }
                    }
                }
            }
        } else {
            let mut baseline_offset = 0.0_f64;
            for (row_idx, row) in grid_boxes.iter().enumerate() {
                let mut cursor_x = 0.0_f64;
                for (col_idx, cell_box) in row.iter().enumerate() {
                    let col_w = if col_idx < n_cols { col_widths[col_idx] } else { 0.0 };
                    let cell_x = match align {
                        GridAlign::Alternating => {
                            if col_idx % 2 == 0 {
                                cursor_x + (col_w - cell_box.width)
                            } else {
                                cursor_x
                            }
                        }
                        GridAlign::Center => cursor_x + (col_w - cell_box.width) / 2.0,
                        GridAlign::Left => cursor_x,
                    };
                    let dy = baseline_offset;
                    for item in cell_box.items.clone() {
                        all_items.push(offset_item(item, Pt(cell_x), Pt(dy)));
                    }
                    cursor_x += col_w;
                    if col_idx + 1 < n_cols {
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
                if row_idx + 1 < grid_boxes.len() {
                    let line_gap = row_gap.val();
                    let next_row = &grid_boxes[row_idx + 1];
                    let row_descent = row
                        .iter()
                        .map(|b| b.descent)
                        .fold(0.0, f64::max)
                        .max(paren_descent);
                    let next_row_ascent = next_row
                        .iter()
                        .map(|b| b.ascent)
                        .fold(0.0, f64::max)
                        .max(paren_ascent);
                    let advance = row_descent + line_gap + next_row_ascent;
                    baseline_offset += advance;
                }
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

    /// Layout em grelha 2D para equações com `&` e/ou `\\`.
    ///
    /// **P991** — o alinhamento depende de existir algum `&` nos nós de
    /// origem (`n_cols > 1`, o mesmo valor usado para `align_boundaries`):
    /// com `&`, alinhamento alternado (colunas pares à direita, ímpares à
    /// esquerda), paridade `run.rs::stack_rows` quando `has_alignment`;
    /// sem `&` (só `\\`, ex.: `(n \\ k)`), `GridAlign::Center` — paridade
    /// com o default `AlignElem::alignment = CENTER` de `equation.rs`,
    /// usado por `stack_rows` quando `!has_alignment`. Ver `_comum.md`
    /// §P991.
    ///
    /// Sem espaço extra entre colunas (o espaçamento de limite `&`, quando
    /// aplicável, é incorporado na largura da célula esquerda — ver
    /// abaixo). **P967b** — todos os limites produzidos por
    /// `partition_grid` são limites `&` por construção: o espaçamento do
    /// limite (`align_boundary_spacing`, com o fallback de item espaçado de
    /// P903 — ex.: espaço de texto antes de uma anotação entre aspas) é
    /// incorporado na largura da célula esquerda, mesmo modelo de P825 para
    /// matrizes. Antes de P967b passava `&[]` (boundary gap = 0) e as
    /// anotações colavam ao fim da célula ("9dado"). Ver `_comum.md` §P967b.
    fn layout_grid(&self, nodes: &[Content], style: &TextStyle) -> MathBox {
        self.layout_grid_with_baselines(nodes, style).0
    }

    fn layout_grid_with_baselines(
        &self,
        nodes: &[Content],
        style: &TextStyle,
    ) -> (MathBox, Vec<f64>) {
        let grid = partition_grid(nodes);
        let n_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);
        // Cada célula é Vec<Content> — envolver em MathSequence para layout_node.
        let rows: Vec<Vec<Content>> = grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell_nodes| Content::MathSequence(cell_nodes.into()))
                    .collect()
            })
            .collect();
        // **P1134** — o leading de math multilinha é propriedade do estilo
        // de parágrafo no vanilla (`run.rs:49-53`), não a MathConstant da
        // fonte. Em scripts, o vanilla troca para TIGHT_LEADING = 0.25em.
        // Matrizes e `cases` passam o seu próprio row_gap por caminho distinto.
        let row_gap = match style.math_size {
            MathSize::Display | MathSize::Text => Pt(style
                .leading
                .map(|leading| leading.resolve_pt(style.size.val()))
                .unwrap_or(PAR_LEADING * style.size.val())),
            MathSize::Script | MathSize::ScriptScript => Pt(0.25 * style.size.val()),
        };

        // Medir as células e incorporar o espaçamento do limite `&` na
        // largura da célula à esquerda (paridade vanilla `run.rs:76-94`).
        let mut grid_boxes: Vec<Vec<MathBox>> = rows
            .iter()
            .map(|row| row.iter().map(|cell| self.layout_node(cell, style)).collect())
            .collect();
        for (row_idx, row) in rows.iter().enumerate() {
            for col_idx in 1..row.len() {
                let gap =
                    self.align_boundary_spacing(&row[col_idx - 1], &row[col_idx], style);
                if let Some(left_box) = grid_boxes[row_idx].get_mut(col_idx - 1) {
                    left_box.width += gap;
                }
            }
        }
        // Todos os limites internos são `&` (ver doc do método).
        let align_boundaries: Vec<Vec<bool>> = rows
            .iter()
            .map(|row| (0..row.len()).map(|i| i > 0).collect())
            .collect();
        // **P991** — sem `&` (n_cols <= 1), centrar; com `&`, alternar.
        let align = if n_cols > 1 { GridAlign::Alternating } else { GridAlign::Center };
        let paren_style = TextStyle { cramped: true, ..style.clone() };
        let paren_box = self.layout_text_node(&EcoString::from("("), &paren_style);
        let mut baselines = Vec::with_capacity(grid_boxes.len());
        let mut baseline = 0.0;
        for (row_idx, row) in grid_boxes.iter().enumerate() {
            baselines.push(baseline);
            if let Some(next) = grid_boxes.get(row_idx + 1) {
                let descent = row
                    .iter()
                    .map(|b| b.descent)
                    .fold(0.0, f64::max)
                    .max(paren_box.descent);
                let next_ascent = next
                    .iter()
                    .map(|b| b.ascent)
                    .fold(0.0, f64::max)
                    .max(paren_box.ascent);
                baseline += descent + row_gap.val() + next_ascent;
            }
        }
        let math_box = self.layout_grid_boxes(
            grid_boxes,
            align,
            Pt(0.0),
            row_gap,
            &align_boundaries,
            style,
        );
        (math_box, baselines)
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
                    // **P994** — formas OCORREM desde P994 (borda de
                    // `box()` embutido, achatada por `layout_external`);
                    // deslocar em X como as outras variantes posicionadas.
                    FrameItem::Shape { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    // **P994** — grupos OCORREM desde P994 (conteúdo externo
                    // embutido via `layout_external`); deslocar em X como as
                    // outras variantes posicionadas.
                    FrameItem::Group { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::Link { .. } => {} // links não ocorrem em contexto math
                    FrameItem::Semantic { ref mut items, .. } => {
                        for child in items {
                            *child = offset_item(child.clone(), Pt(x), Pt(0.0));
                        }
                    }
                }
                items.push(item);
            }
            x += b.width;
        }

        MathBox { width: x, ascent, descent, items }
    }
}

/// Um `TextItem` inline só perde a cola terminal quando encosta a um
/// delimitador matemático estrutural. Texto/markup adjacente conserva o frame.
pub(super) fn should_tighten_text_frame(node: &Content, next: Option<&Content>) -> bool {
    matches!((node, next), (Content::Text(_), Some(Content::MathDelimited(_))))
}

/// Caractere de uma folha matemática simples, sem atravessar containers.
fn single_math_leaf_char(content: &Content) -> Option<char> {
    let text = match content {
        Content::Text(text) | Content::MathText(text) | Content::MathIdent(text) => text,
        _ => return None,
    };
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}

/// Achata estrutura matemática, mas preserva caixas de markup. `Sequence`
/// técnica só é aberta quando transporta `HSpace` no próprio nível (`dif`).
pub(super) fn flatten_math_sequence_nodes(nodes: &[Content], out: &mut Vec<Content>) {
    for node in nodes {
        match node {
            Content::MathSequence(children) => flatten_math_sequence_nodes(children, out),
            Content::Sequence(children)
                if children.iter().all(|child| {
                    matches!(
                        child,
                        Content::Text(_) | Content::MathText(_) | Content::MathIdent(_)
                    )
                }) =>
            {
                out.push(node.clone())
            }
            Content::Sequence(children) => flatten_math_sequence_nodes(children, out),
            other => out.push(other.clone()),
        }
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
            if matches!((chars.next(), chars.next()), (Some(c), None) if is_math_italic_default(c))
            {
                let c = name.chars().next().unwrap();
                return Content::MathIdent(
                    map_glyph(c, MathStyleKind::Plain, false, true).into(),
                );
            }
            body.clone()
        }
        Content::MathText(text) => {
            let mut chars = text.chars();
            if matches!((chars.next(), chars.next()), (Some(c), None) if is_math_italic_default(c))
            {
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
        Content::MathMatrix(e) => Content::math_matrix_full(
            e.rows
                .iter()
                .map(|row| row.iter().map(apply_math_default).collect())
                .collect(),
            e.delim,
            e.row_gap,
            e.column_gap,
            e.gap,
            e.augment,
        ),
        Content::MathCases(e) => Content::math_cases(
            e.rows
                .iter()
                .map(|row| row.iter().map(apply_math_default).collect())
                .collect(),
            e.delim,
            e.reverse,
            e.gap,
        ),
        Content::MathFrac(e) => {
            let num = apply_math_default(&e.num);
            let den = apply_math_default(&e.den);
            if e.line {
                Content::math_frac(num, den)
            } else {
                Content::math_frac_unlined(num, den)
            }
        }
        Content::MathAttach(e) => Content::math_attach(
            apply_math_default(&e.base),
            e.t.as_ref().map(apply_math_default),
            e.b.as_ref().map(apply_math_default),
            e.tl.as_ref().map(apply_math_default),
            e.bl.as_ref().map(apply_math_default),
            e.tr.as_ref().map(apply_math_default),
            e.br.as_ref().map(apply_math_default),
        ),
        Content::MathRoot(e) => Content::math_root(
            e.index.as_ref().map(apply_math_default),
            apply_math_default(&e.radicand),
        ),
        Content::MathDelimited(e) => {
            Content::math_delimited(e.open, apply_math_default(&e.body), e.close)
        }
        // **P961** — achado #5 de P906 + auditoria externa 2026-08-04: a
        // base de acentos (`hat(x)` etc.) nunca recebia o itálico por
        // defeito (caía em `other`). Recursão na base; o `accent` (combining
        // mark) passa inalterado. `MathUnderover`: recursão nos três
        // campos (a anotação de 1 letra também recebe itálico; a peça ⏟ não
        // é letra, inalterada na prática). Ver `_comum.md` §P961.
        Content::MathAccent(e) => {
            Content::math_accent(apply_math_default(&e.base), e.accent.clone())
        }
        Content::MathUnderover(e) => Content::math_underover(
            apply_math_default(&e.base),
            e.under.as_ref().map(apply_math_default),
            e.over.as_ref().map(apply_math_default),
        ),
        Content::MathOp(_) => body.clone(),
        // **P992** — `limits(body)`/`scripts(body)`: o override afecta só o
        // discriminador de `layout_attach` (`is_limits`); o `body` recursa
        // normalmente para receber o itálico por defeito (achado: base de
        // 1 letra `limits(A)` deve continuar 𝐴, não "A" recto).
        Content::MathLimitsOverride(e) => {
            Content::math_limits_override(apply_math_default(&e.body), e.limits, e.inline)
        }
        // **P990-C** — achado §8.1 da auditoria: `cancel(a+b)` e
        // `std.strike(a+b)` renderizavam o corpo em glifo RETO em vez de
        // itálico matemático — nenhum dos dois braços recursava no corpo
        // (caíam no catch-all `other`), mesma família de P961/P966.
        // `MathCancel`: recursão simples no corpo (espelho do braço
        // `MathAccent` de P961). `Strike`: recursão no corpo, `stroke`/
        // `offset`/`extent` preservados sem alteração. Ver `_comum.md`
        // §P990-C.
        Content::MathCancel(e) => Content::math_cancel(apply_math_default(&e.body)),
        Content::Strike(e) => {
            Content::strike(apply_math_default(&e.body), e.stroke, e.offset, e.extent)
        }
        Content::Overline(e) => {
            Content::overline(apply_math_default(&e.body), e.stroke, e.offset, e.extent)
        }
        Content::Underline(e) => {
            Content::underline(apply_math_default(&e.body), e.stroke, e.offset, e.extent)
        }
        // **P966** — conteúdo de função de utilizador dentro de math chega
        // como containers de markup (`Sequence`/`Styled`) com folhas mistas
        // `Text`/`MathText`. Recursão nos filhos/corpo para as folhas
        // `MathText` de 1 carácter receberem o default de itálico; as folhas
        // `Text` (literais) caem no catch-all, inalteradas (paridade com o
        // `resolve_text` do vanilla). Ver `_comum.md` §P966.
        Content::Sequence(items) => {
            let new_items: Vec<Content> = items.iter().map(apply_math_default).collect();
            Content::Sequence(Arc::from(new_items))
        }
        Content::Styled(inner, styles) => {
            Content::Styled(Box::new(apply_math_default(inner)), styles.clone())
        }
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
        Content::MathMatrix(e) => Content::math_matrix_full(
            e.rows
                .iter()
                .map(|row| {
                    row.iter().map(|c| apply_math_style(c, kind, bold, italic)).collect()
                })
                .collect(),
            e.delim,
            e.row_gap,
            e.column_gap,
            e.gap,
            e.augment,
        ),
        Content::MathCases(e) => Content::math_cases(
            e.rows
                .iter()
                .map(|row| {
                    row.iter().map(|c| apply_math_style(c, kind, bold, italic)).collect()
                })
                .collect(),
            e.delim,
            e.reverse,
            e.gap,
        ),
        // Modelo D (Lote 2 P317): destructuring de `Arc<…Elem>` + reconstrução
        // via construtor ergonómico — mesma lógica recursiva.
        Content::MathFrac(e) => {
            let num = apply_math_style(&e.num, kind, bold, italic);
            let den = apply_math_style(&e.den, kind, bold, italic);
            if e.line {
                Content::math_frac(num, den)
            } else {
                Content::math_frac_unlined(num, den)
            }
        }
        Content::MathAttach(e) => Content::math_attach(
            apply_math_style(&e.base, kind, bold, italic),
            e.t.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
            e.b.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
            e.tl.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
            e.bl.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
            e.tr.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
            e.br.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
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
