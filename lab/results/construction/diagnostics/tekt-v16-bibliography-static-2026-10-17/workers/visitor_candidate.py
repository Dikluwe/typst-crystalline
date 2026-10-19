from pathlib import Path
out=Path('00_nucleo/diagnosticos/tekt-v16-bibliography-static-2026-10-17')
s='''    /// Borrow static children in order. False prunes without evaluating callbacks.
    pub(crate) fn visit_content_static<F>(&self, visit: &mut F)
    where
        F: FnMut(&Content) -> bool,
    {
        if let Content::Source(e) = self { e.body.visit_content_static(visit); return; }
        if let Content::ShowRecipe(e) = self { e.body.visit_content_static(visit); return; }
        if let Content::Located(e) = self { e.body.visit_content_static(visit); return; }
        if let Content::Realized(e) = self { e.visual.visit_content_static(visit); return; }
        if !visit(self) { return; }
        match self {
            Content::Source(_) | Content::ShowRecipe(_) | Content::Located(_) | Content::Realized(_) => unreachable!("transparent carriers dispatched above"),
            Content::Sequence(parts) | Content::MathSequence(parts) => {
                for child in parts.iter() { child.visit_content_static(visit); }
            }
            Content::Par { body } | Content::SmallCaps { body } => body.visit_content_static(visit),
            Content::HtmlElem(e) => {
                if let crate::entities::html::HtmlBody::Content(body) = &e.body { body.visit_content_static(visit); }
            }
            Content::PageRun(e) => {
                for marginal in [&e.header, &e.footer] {
                    if let Some(crate::entities::page_running::PageMarginal::Content(c)) = marginal { c.visit_content_static(visit); }
                }
                if let Some(crate::entities::page_supplement::PageSupplement::Content(c)) = &e.supplement { c.visit_content_static(visit); }
                for layer in [&e.background, &e.foreground] {
                    if let Some(Some(c)) = layer { c.visit_content_static(visit); }
                }
                e.body.visit_content_static(visit);
            }
'''
body='HtmlFrame Heading Title Strong Emph ListItem EnumItem Link Label Equation MathStretch MathDelimited MathLr MathMid MathCancel MathUnderline MathOverline MathClassOverride MathLimitsOverride MathStyled PdfArtifact Underline Strike Overline Pad Hide Block Boxed Repeat Columns Transform GridHeader GridFooter GridCell TableHeader TableFooter Footnote Align Place Styled'.split()
for n in body:s+=f'            Content::{n}(e) => e.body.visit_content_static(visit),\n'
s+='''            Content::Figure(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.caption { c.visit_content_static(visit); }
            }
            Content::FigureCaption(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.separator { c.visit_content_static(visit); }
            }
            Content::MathFrac(e) => { e.num.visit_content_static(visit); e.den.visit_content_static(visit); }
            Content::MathAttach(e) => {
                e.base.visit_content_static(visit);
                for slot in [&e.t, &e.b, &e.tl, &e.bl, &e.tr, &e.br] {
                    if let crate::entities::elements::math_attach::MathAttachSlot::Present(c) = slot { c.visit_content_static(visit); }
                }
            }
            Content::MathRoot(e) => {
                if let Some(c) = &e.index { c.visit_content_static(visit); }
                e.radicand.visit_content_static(visit);
            }
'''
for n in 'MathOverbrace MathUnderbrace MathUnderbracket MathUnderparen MathUndershell MathOverbracket MathOverparen MathOvershell'.split():
 s+=f'''            Content::{n}(e) => {{
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation {{ c.visit_content_static(visit); }}
            }}
'''
s+='''            Content::MathMatrix(e) => {
                for row in &e.rows { for c in row { c.visit_content_static(visit); } }
            }
            Content::MathCases(e) => { for c in &e.children { c.visit_content_static(visit); } }
            Content::MathVec(e) => { for c in &e.children { c.visit_content_static(visit); } }
            Content::MathAccent(e) => e.base.visit_content_static(visit),
            Content::MathUnderover(e) => {
                e.base.visit_content_static(visit);
                for c in [&e.under, &e.over].into_iter().flatten() { c.visit_content_static(visit); }
            }
            Content::MathOp(e) => e.text.visit_content_static(visit),
            Content::Terms(e) => { for c in &e.items { c.visit_content_static(visit); } }
            Content::TermItem(e) => { e.term.visit_content_static(visit); e.description.visit_content_static(visit); }
            Content::Quote(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.attribution { c.visit_content_static(visit); }
            }
            Content::Stack(e) => { for c in e.children.iter() { c.visit_content_static(visit); } }
            Content::Outline(e) => {
                if let Some(c) = &e.title { c.visit_content_static(visit); }
                if let Some(c) = e.realized_body() { c.visit_content_static(visit); }
            }
            Content::OutlineEntry(e) => {
                e.element.visit_content_static(visit);
                if let Some(c) = &e.fill { c.visit_content_static(visit); }
                if let Some(c) = e.prefix() { c.visit_content_static(visit); }
            }
            Content::Grid(e) => {
                for c in &e.cells { c.visit_content_static(visit); }
                for c in [&e.header, &e.footer].into_iter().flatten() { c.visit_content_static(visit); }
            }
            Content::Table(e) => {
                if let Some(plan) = &e.resolved {
                    for c in &plan.cells { c.visit_content_static(visit); }
                } else {
                    for c in &e.children { c.visit_content_static(visit); }
                    for c in [&e.header, &e.footer].into_iter().flatten() { c.visit_content_static(visit); }
                }
                if let Some(c) = &e.caption { c.visit_content_static(visit); }
            }
            Content::TableCell(e) => e.visual().visit_content_static(visit),
            Content::Bibliography(e) => {
                if let Some(c) = &e.title { c.visit_content_static(visit); }
            }
            Content::Cite(e) => {
                if let Some(c) = &e.supplement { c.visit_content_static(visit); }
            }
            Content::FootnoteEntry(e) => {
                e.note.visit_content_static(visit);
                if let Some(c) = &e.separator { c.visit_content_static(visit); }
            }
            Content::Document { body, title, .. } => {
                body.visit_content_static(visit);
                if let Some(c) = title { c.visit_content_static(visit); }
            }
'''
leaves='Empty Text Space SpaceFromNewline Parbreak Raw MathIdent MathText MathPrimes MathAlignPoint Linebreak Ref CounterDisplay CounterUpdate Image Shape Curve SetPage Flush Divider Asset SmartQuote PdfAttach HSpace VSpace Pagebreak Colbreak GridHLine GridVLine TableHLine TableVLine Metadata State StateUpdate StateDisplay CounterDisplayCallback ContextBlock Dynamic'.split()
units={'Empty','Space','SpaceFromNewline','Parbreak'}
s+='            '+'\n            | '.join('Content::'+n+('' if n in units else ' { .. }' if n=='Asset' else '(..)') for n in leaves)+' => {}\n        }\n    }\n'
(out/'visitor-candidate.rs').write_text(s);(out/'visitor-plan.json').write_text(__import__('json').dumps({'body_variants':body,'terminal_variants':leaves},indent=2)+'\n');(out/'workers/visitor_candidate.py').write_text(Path('/tmp/bib-i2-visitor.py').read_text());print('Borrowed visitor candidate prepared; not yet installed')
