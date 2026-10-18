from pathlib import Path
b=Path('01_core/src/compiler/layout')
f=b/'line_commit.rs';s=f.read_text();needle='''            FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => visit_text_styles(items, visit),''';replacement='''            FrameItem::Semantic {
                kind: crate::entities::layout_types::SemanticKind::ExplicitLinebreakBoundary
                    | crate::entities::layout_types::SemanticKind::ParbreakBoundary,
                ..
            } => {}
            FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => visit_text_styles(items, visit),''';assert s.count(needle)==1;s=s.replace(needle,replacement);f.write_text(s)
f=b/'cursor.rs';s=f.read_text();needle='''    pub(super) fn ensure_initial_baseline(&mut self) {
        if self.initial_baseline_pending {''';replacement='''    pub(super) fn ensure_initial_baseline(&mut self) {
        self.ensure_baseline_for(super::region_progress::Admission::Generic);
    }

    pub(super) fn ensure_text_baseline(&mut self) {
        self.ensure_baseline_for(super::region_progress::Admission::Text);
    }

    pub(super) fn ensure_line_baseline(&mut self, top: f64) {
        self.ensure_baseline_for(super::region_progress::Admission::KnownTop(top));
    }

    fn ensure_baseline_for(&mut self, admission: super::region_progress::Admission) {
        if self.initial_baseline_pending {''';assert s.count(needle)==1;s=s.replace(needle,replacement).replace('super::region_progress::admit_next_text_line(self);','super::region_progress::admit_next_text_line(self, admission);');f.write_text(s)
for name in ['text_baseline.rs','word_layout.rs']:
 f=b/name;s=f.read_text();assert s.count('layouter.ensure_initial_baseline();')==1;s=s.replace('layouter.ensure_initial_baseline();','layouter.ensure_text_baseline();');f.write_text(s)
f=b/'region_progress.rs';s=f.read_text();idx=s.index('/// Admit real continuation');s=s[:idx]+'''#[derive(Clone, Copy)]
pub(super) enum Admission {
    Generic,
    Text,
    KnownTop(f64),
}

''' +s[idx:];s=s.replace('''    layouter: &mut Layouter<M, S>,
) {
    if !layouter.is_sub_frame
        && layouter.page_config.height.is_finite()
        && layouter.regions.current.cursor_y.0 > layouter.page_bottom_limit()''','''    layouter: &mut Layouter<M, S>,
    admission: Admission,
) {
    if !layouter.is_sub_frame
        && layouter.page_config.height.is_finite()
        && layouter.regions.current.cursor_y.0 > layouter.page_bottom_limit()''',1)
needle='''    {
        layouter.new_page();
    }
}

fn reset_closed_region_flow''';replacement='''    {
        let top = match admission {
            Admission::Generic => None,
            Admission::Text => Some(layouter.metrics.text_edges(layouter.style.size, &layouter.style).0.0),
            Admission::KnownTop(top) => Some(top),
        };
        let candidate = top.map_or(layouter.regions.current.cursor_y.0, |top| {
            layouter.regions.current.cursor_y.0
                + top - layouter.line_assumed_ascent.unwrap_or(top)
        });
        if candidate > layouter.page_bottom_limit() {
            layouter.new_page();
        }
    }
}

fn reset_closed_region_flow''';assert s.count(needle)==1;s=s.replace(needle,replacement)
needle='''    layouter.regions.current.cursor_y = if layouter.initial_baseline_pending {
        Pt(layouter.page_config.margin.top)
    } else {
        let (top, _) = layouter.metrics.text_edges(layouter.style.size, &layouter.style);
        Pt(layouter.page_config.margin.top) + top
    };''';replacement='''    layouter.regions.current.cursor_y = if layouter.initial_baseline_pending {
        layouter.line_assumed_ascent = None;
        Pt(layouter.page_config.margin.top)
    } else {
        let (top, _) = layouter.metrics.text_edges(layouter.style.size, &layouter.style);
        layouter.line_assumed_ascent = Some(top.0);
        Pt(layouter.page_config.margin.top) + top
    };''';assert s.count(needle)==1;s=s.replace(needle,replacement);f.write_text(s)
# The borrowed profile keeps selected ranges rather than allocating geometry per line.
f=b/'paragraph_pagination.rs';s=f.read_text();tests=s[s.index('#[cfg(test)]'):];header=s[:s.index('use super::')];body='''use super::{paragraph_prepare::Prepared, paragraph_select::Line, FontMetrics, ImageSizer, Layouter};
use crate::entities::layout_types::{PageMargins, TextSeparatorOrigin, TextStyle};

#[derive(Clone, Copy)]
enum Lines<'a> {
    Selected(&'a Prepared, &'a [Line]),
    Prepared(&'a [Prepared]),
}
impl<'a> Lines<'a> {
    fn len(self) -> usize {
        match self {
            Self::Selected(_, lines) => lines.len(),
            Self::Prepared(lines) => lines.len(),
        }
    }
    fn get(self, index: usize) -> (&'a Prepared, Line) {
        match self {
            Self::Selected(part, lines) => (part, lines[index]),
            Self::Prepared(lines) => {
                let part = &lines[index];
                (part, Line { start: 0, end: part.words.len() })
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Profile<'a> {
    lines: Lines<'a>,
    leading: f64,
    widow: bool,
    orphan: bool,
}

fn visit_styles(part: &Prepared, line: Line, visit: &mut impl FnMut(&TextStyle)) {
    for i in line.start..line.end {
        if i > line.start || line.start == 0 {
            for gap in &part.words[i].before {
                if gap.origin == TextSeparatorOrigin::Literal {
                    visit(&gap.style);
                }
            }
        }
        for fragment in &part.words[i].fragments {
            visit(&fragment.style);
        }
    }
    let trailing = if line.end < part.words.len() { &part.words[line.end].before } else { &part.trailing };
    for gap in trailing {
        if gap.origin == TextSeparatorOrigin::Literal {
            visit(&gap.style);
        }
    }
}

fn geometry<M: FontMetrics, S: ImageSizer>(l: &Layouter<M,S>, part: &Prepared, line: Line) -> (f64,f64) {
    let (mut top,mut descent)=(0.0_f64,0.0_f64);
    visit_styles(part,line,&mut |style| {
        let edges=l.metrics.text_edges(style.size,style);
        top=top.max(edges.0.0);descent=descent.max(-edges.1.0);
    });
    (top,descent)
}

fn certify<'a,M: FontMetrics,S: ImageSizer>(l:&Layouter<M,S>,lines:Lines<'a>)->Option<Profile<'a>> {
    if !l.page_config.height.is_finite()
        || l.column_mode || l.page_columns.is_some() || l.regions.cell.is_some()
        || l.is_sub_frame || !l.floats_pending.is_empty() || !l.pending_footnote_bodies.is_empty()
        || l.flow_suffix_transaction_active || l.regions.current.height != l.page_config.height
    { return None; }
    let mut leading=None;
    for index in 0..lines.len() {
        let (part,line)=lines.get(index);
        if line.start>=line.end || line.end>part.words.len() {return None;}
        let mut valid=true;
        let mut any=false;
        visit_styles(part,line,&mut |style| {
            if !valid {return;}
            any=true;
            if style.size!=l.style.size || style.baseline_offset.resolve_pt(style.size.0)!=0.0 {valid=false;return;}
            let (top,bottom)=l.metrics.text_edges(style.size,style);
            let gap=style.leading.map(|v|v.resolve_pt(style.size.0)).unwrap_or(style.size.0*super::vanilla_defaults::PAR_LEADING);
            if [top.0,-bottom.0,gap].iter().any(|v|!v.is_finite() || *v<0.0)
                || leading.is_some_and(|before|before!=gap) {valid=false;return;}
            leading=Some(gap);
        });
        if !valid || !any {return None;}
    }
    let (widow,orphan)=crate::compiler::text_costs::prevention(&l.chain);
    Some(Profile{lines,leading:leading?,widow,orphan})
}

pub(super) fn certify_selected<'a,M:FontMetrics,S:ImageSizer>(l:&Layouter<M,S>,part:&'a Prepared,lines:&'a[Line])->Option<Profile<'a>> {
    certify(l,Lines::Selected(part,lines))
}
pub(super) fn certify_prepared<'a,M:FontMetrics,S:ImageSizer>(l:&Layouter<M,S>,lines:&'a[Prepared])->Option<Profile<'a>> {
    certify(l,Lines::Prepared(lines))
}

fn group_size(index: usize, len: usize, widow: bool, orphan: bool) -> usize {
    if index == 0 && len == 3 && widow && orphan {
        3
    } else if (index == 0 && len >= 2 && orphan)
        || (index >= 2 && index + 2 == len && widow)
    {
        2
    } else {
        1
    }
}

pub(super) fn admit<M:FontMetrics,S:ImageSizer>(l:&mut Layouter<M,S>,profile:Option<Profile<'_>>,index:usize,len:usize) {
    let Some(profile)=profile else{return;};
    let (part,line)=profile.lines.get(index);
    let (top,descent)=geometry(l,part,line);
    l.ensure_line_baseline(top);
    let baseline=l.regions.current.cursor_y.0 + top - l.line_assumed_ascent.unwrap_or(top);
    let group=group_size(index,len,profile.widow,profile.orphan);
    let mut tail=descent;
    for offset in 1..group {
        let (part,line)=profile.lines.get(index+offset);
        let (next_top,next_descent)=geometry(l,part,line);
        tail+=profile.leading+next_top+next_descent;
    }
    let next_page=std::num::NonZeroUsize::new(l.pages.len()+2).unwrap();
    let margin=PageMargins::resolve_for_page(l.page_config.margin_spec,l.page_config.auto_margin(),l.page_config.binding.resolve(l.page_config.page_dir),next_page);
    let next_bottom=l.page_config.height-margin.bottom;
    let next_baseline=margin.top+top;
    if !l.current_page_is_empty() && baseline+tail>l.page_bottom_limit() && next_baseline+tail<=next_bottom {l.new_page();}
}

''';f.write_text(header+body+tests)
f=b/'paragraph_emit.rs';s=f.read_text();s=s.replace('let pagination = super::paragraph_pagination::certify(l, &[p]);','let pagination = super::paragraph_pagination::certify_selected(l, p, lines);');needle='''    let refs: Vec<_> = lines.iter().collect();
    let pagination = super::paragraph_pagination::certify(l, &refs);''';assert s.count(needle)==1;s=s.replace(needle,'    let pagination = super::paragraph_pagination::certify_prepared(l, lines);');f.write_text(s)
