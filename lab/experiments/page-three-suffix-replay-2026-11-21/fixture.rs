
fn lab_page_three_fixture(mut text: &str) -> bool {
    const HEADER: &str = "#set page(width:400pt,height:600pt)\n#set heading(numbering:\"1.\")\n#outline()\n";
    const LOOP: &str = "#for j in range(25) [";
    const PARAGRAPH_TAIL: &str = " #j with several words to lay out across lines.\n\n]\n";
    let Some(body) = text.strip_prefix(HEADER) else { return false };
    text = body;
    for i in 0..20 {
        let block = format!("= Section {i} <s{i}>\nSee @s{i}.\n{LOOP}");
        let Some(body) = text.strip_prefix(&block) else { return false };
        let Some((words, rest)) = body.split_once(PARAGRAPH_TAIL) else { return false };
        if if i == 0 {
            words.is_empty() || words.len() > 96
                || !words.bytes().all(|b| b.is_ascii_alphabetic() || b == b' ')
        } else {
            words != "Paragraph"
        } {
            return false;
        }
        text = rest;
    }
    text.is_empty() || text.strip_prefix("\n// ").and_then(|s| s.strip_suffix('\n'))
        .is_some_and(|s| s.is_ascii() && !s.contains(['\n','\r']))
}
