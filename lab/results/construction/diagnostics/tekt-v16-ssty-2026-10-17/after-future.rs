#![allow(dead_code)]
mod typst_core {pub mod entities {pub mod layout_types {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MathSize {
    Display,
    /// Default neutro: contexto de texto corrente (inline).
    #[default]
    Text,
    Script,
    ScriptScript,
    TektSstyFuture,
}
}}}
use typst_core::entities::layout_types::MathSize;
struct TextStyle { math:bool,math_text_item:bool,math_size:MathSize }
pub(crate) fn ssty_eligible_text(text: &str) -> bool {
    let count = text.chars().count();
    if count == 1 {
        return true;
    }
    let mut decimal_count = 0usize;
    let all_digit_or_dot = text.chars().all(|c| {
        if c == '.' {
            decimal_count += 1;
        }
        c.is_ascii_digit() || c == '.'
    });
    all_digit_or_dot && decimal_count != count && decimal_count <= 1
}
mod metrics {use super::*;
pub(super) fn ssty_level_of(style: &TextStyle, ssty_eligible: bool) -> Option<u8> {
    if !style.math || style.math_text_item || !ssty_eligible {
        return None;
    }
    match style.math_size {
        MathSize::Script => Some(1),
        MathSize::ScriptScript => Some(2),
        MathSize::Display | MathSize::Text => None,
    }
}
}
mod shaping {use super::*;
pub(super) fn ssty_level_for_subrun(style: &TextStyle, text: &str) -> Option<u8> {
    if !style.math || style.math_text_item || !ssty_eligible_text(text) {
        return None;
    }
    match style.math_size {
        MathSize::Script => Some(1),
        MathSize::ScriptScript => Some(2),
        MathSize::Display | MathSize::Text => None,
    }
}
}

fn main() {
 let mut count=0;
 for (size,label,level) in [(MathSize::Display,"display",None),(MathSize::Text,"text",None),(MathSize::Script,"script",Some(1u8)),(MathSize::ScriptScript,"scriptscript",Some(2))] {
  for math in [false,true] { for math_text_item in [false,true] {
   let style=TextStyle{math,math_text_item,math_size:size};
   for eligible in [false,true] {
    let got=metrics::ssty_level_of(&style,eligible);
    assert_eq!(got, if math && !math_text_item && eligible { level } else { None });
    println!("M {label} {math} {math_text_item} {eligible} {got:?}");count+=1;
   }
   for (text,eligible) in [("",false),("x",true),("𝑥",true),(".",true),(" ",true),("12",true),("3.14",true),("..",false),("xy",false),("1.2.3",false),("١٢",false),("e\u{301}",false)] {
    assert_eq!(ssty_eligible_text(text),eligible);
    let got=shaping::ssty_level_for_subrun(&style,text);
    assert_eq!(got,if math && !math_text_item && eligible { level } else { None });
    println!("S {label} {math} {math_text_item} {text:?} {got:?}");count+=1;
   }
  }}
 }
 assert_eq!(count,224);
}
