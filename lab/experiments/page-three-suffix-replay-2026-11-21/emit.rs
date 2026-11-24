
#[derive(Debug, Clone, PartialEq)]
pub(super) struct LabThreeLine {
    entry: [u64; 6],
    line: super::paragraph_select::Line,
    width: u64,
    flags: [bool; 2],
    style: TextStyle,
    words: Vec<(u64, Vec<LabThreeFragment>, Vec<LabThreeGap>)>,
    trailing: Vec<LabThreeGap>,
    overhang: u64,
}

type LabThreeFragment = (String, Option<String>, TextStyle, u64, bool, bool);
type LabThreeGap = (TextStyle, u64, crate::entities::layout_types::TextSeparatorOrigin);

impl PageThreeEmitter {
    pub(super) fn lab_three_line(&self) -> LabThreeLine {
        use super::paragraph_prepare::Gap;
        let gap = |gap: &Gap| (gap.style.clone(), gap.width.to_bits(), gap.origin);
        let line = self.product.lines[self.index];
        let prepared = &self.product.prepared;
        LabThreeLine {
            entry: self.resume_entry,
            line,
            width: self.width.to_bits(),
            flags: [self.justify, self.lend],
            style: self.restore_style.clone(),
            words: prepared.words[line.start..line.end].iter().map(|word| {
                (word.width.to_bits(), word.fragments.iter().map(|fragment|
                    (fragment.text.clone(), fragment.replacement.clone(), fragment.style.clone(),
                     fragment.width.to_bits(), fragment.hyphenate, fragment.overhang)).collect(),
                    word.before.iter().map(gap).collect())
            }).collect(),
            trailing: prepared.trailing.iter().map(gap).collect(),
            overhang: prepared.right_overhang.to_bits(),
        }
    }
}
