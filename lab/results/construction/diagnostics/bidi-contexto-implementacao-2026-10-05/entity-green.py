from pathlib import Path
p=Path('01_core/src/entities/bidi_context.rs');s=p.read_text().replace('Some(Self { base_dir, segments })','''if !base_dir.is_horizontal() || !segments.iter().any(|s| matches!(s, ParagraphBidiSegment::Text { text, .. } if !text.is_empty())) { return None; }
        Some(Self { base_dir, segments })''').replace('Some(Self { input, segment, range })','''let text = match input.segments.get(segment)? {
            ParagraphBidiSegment::Text { text, .. } => text.as_str(),
            ParagraphBidiSegment::Space { .. } => " ",
            ParagraphBidiSegment::Linebreak => return None,
        };
        if range.start > range.end || text.get(range.clone()).is_none() { return None; }
        Some(Self { input, segment, range })''').replace('Self::new(self.input.clone(), self.segment, relative)','''if relative.start > relative.end || relative.end > self.range.end - self.range.start { return None; }
        Self::new(self.input.clone(), self.segment,
            self.range.start.checked_add(relative.start)?..self.range.start.checked_add(relative.end)?)''');p.write_text(s)
