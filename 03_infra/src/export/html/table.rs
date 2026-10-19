//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/table.md
//! @prompt-hash 728ce710
//! @layer L3
//! @updated 2026-10-07

use super::inline::inline;
use super::{HtmlFrameResolver, HtmlSerializationMode};
use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;

pub(super) fn html_table(
    table: &typst_core::entities::elements::table::TableElem,
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
) -> Result<String, SourceDiagnostic> {
    fn flatten<'a>(content: &'a Content, out: &mut Vec<&'a Content>) {
        match content {
            Content::Sequence(items) => {
                for item in items.iter() {
                    flatten(item, out);
                }
            }
            Content::TableHeader(header) => flatten(&header.body, out),
            Content::TableFooter(footer) => flatten(&footer.body, out),
            other => out.push(other),
        }
    }

    fn rows(
        cells: &[&Content],
        columns: usize,
        tag: &str,
        mode: HtmlSerializationMode,
        resolver: Option<&HtmlFrameResolver<'_>>,
        chain: &typst_core::entities::style_chain::StyleChain,
    ) -> Result<String, SourceDiagnostic> {
        let mut out = String::new();
        for chunk in cells.chunks(columns.max(1)) {
            out.push_str("<tr>");
            for content in chunk {
                let (body, colspan, rowspan) = match content {
                    Content::TableCell(cell) => match &cell.resolved {
                        Some(r) => (&r.visual, r.colspan, r.rowspan),
                        None => (
                            cell.standalone.as_ref().map_or(&cell.body, |r| &r.visual),
                            cell.colspan.unwrap_or(1),
                            cell.rowspan.unwrap_or(1),
                        ),
                    },
                    other => (*other, 1, 1),
                };
                out.push('<');
                out.push_str(tag);
                if colspan > 1 {
                    out.push_str(&format!(" colspan=\"{colspan}\""));
                }
                if rowspan > 1 {
                    out.push_str(&format!(" rowspan=\"{rowspan}\""));
                }
                out.push('>');
                out.push_str(&inline(body, mode, resolver, chain)?);
                out.push_str("</");
                out.push_str(tag);
                out.push('>');
            }
            out.push_str("</tr>");
        }
        Ok(out)
    }

    if let Some(plan) = &table.resolved {
        let mut out = String::from("<table>");
        for (start, end, section, tag) in [
            (0, plan.header_rows, "thead", "th"),
            (plan.header_rows, plan.footer_start, "tbody", "td"),
            (plan.footer_start, plan.num_rows, "tfoot", "td"),
        ] {
            if start == end && section != "tbody" {
                continue;
            }
            out.push_str(&format!("<{section}>"));
            for row in start..end {
                let cells=plan.cells.iter().filter(|c|matches!(c,Content::TableCell(c) if c.resolved.as_ref().is_some_and(|r|r.y==row))).collect::<Vec<_>>();
                if cells.is_empty() {
                    out.push_str("<tr></tr>");
                } else {
                    out.push_str(&rows(&cells, cells.len(), tag, mode, resolver, chain)?);
                }
            }
            out.push_str(&format!("</{section}>"));
        }
        out.push_str("</table>");
        return Ok(out);
    }
    let columns = table.columns.len().max(1);
    let mut out = String::from("<table>");
    if let Some(header) = &table.header {
        let mut cells = Vec::new();
        flatten(header, &mut cells);
        out.push_str("<thead>");
        out.push_str(&rows(&cells, columns, "th", mode, resolver, chain)?);
        out.push_str("</thead>");
    }
    let mut cells = Vec::new();
    for child in &table.children {
        flatten(child, &mut cells);
    }
    out.push_str("<tbody>");
    out.push_str(&rows(&cells, columns, "td", mode, resolver, chain)?);
    out.push_str("</tbody></table>");
    Ok(out)
}
