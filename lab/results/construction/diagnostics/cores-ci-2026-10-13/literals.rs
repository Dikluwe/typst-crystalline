fn main() {
let a: f32 = 0.412_221_470_8; let b: f32 = 0.412_221_46; assert_eq!(a.to_bits(), b.to_bits(), "literal 0");
let a: f32 = 0.536_332_536_3; let b: f32 = 0.536_332_55; assert_eq!(a.to_bits(), b.to_bits(), "literal 1");
let a: f32 = 0.051_445_992_9; let b: f32 = 0.051_445_995; assert_eq!(a.to_bits(), b.to_bits(), "literal 2");
let a: f32 = 0.211_903_498_2; let b: f32 = 0.211_903_5; assert_eq!(a.to_bits(), b.to_bits(), "literal 3");
let a: f32 = 0.680_699_545_1; let b: f32 = 0.680_699_5; assert_eq!(a.to_bits(), b.to_bits(), "literal 4");
let a: f32 = 0.107_396_956_6; let b: f32 = 0.107_396_96; assert_eq!(a.to_bits(), b.to_bits(), "literal 5");
let a: f32 = 0.088_302_461_9; let b: f32 = 0.088_302_46; assert_eq!(a.to_bits(), b.to_bits(), "literal 6");
let a: f32 = 0.281_718_837_6; let b: f32 = 0.281_718_85; assert_eq!(a.to_bits(), b.to_bits(), "literal 7");
let a: f32 = 0.629_978_700_5; let b: f32 = 0.629_978_7; assert_eq!(a.to_bits(), b.to_bits(), "literal 8");
let a: f32 = 0.210_454_255_3; let b: f32 = 0.210_454_26; assert_eq!(a.to_bits(), b.to_bits(), "literal 9");
let a: f32 = 0.793_617_785_0; let b: f32 = 0.793_617_8; assert_eq!(a.to_bits(), b.to_bits(), "literal 10");
let a: f32 = 0.004_072_046_8; let b: f32 = 0.004_072_047; assert_eq!(a.to_bits(), b.to_bits(), "literal 11");
let a: f32 = 1.977_998_495_1; let b: f32 = 1.977_998_5; assert_eq!(a.to_bits(), b.to_bits(), "literal 12");
let a: f32 = 2.428_592_205_0; let b: f32 = 2.428_592_2; assert_eq!(a.to_bits(), b.to_bits(), "literal 13");
let a: f32 = 0.450_593_709_9; let b: f32 = 0.450_593_7; assert_eq!(a.to_bits(), b.to_bits(), "literal 14");
let a: f32 = 0.025_904_037_1; let b: f32 = 0.025_904_037; assert_eq!(a.to_bits(), b.to_bits(), "literal 15");
let a: f32 = 0.782_771_766_2; let b: f32 = 0.782_771_77; assert_eq!(a.to_bits(), b.to_bits(), "literal 16");
let a: f32 = 0.808_675_766_0; let b: f32 = 0.808_675_77; assert_eq!(a.to_bits(), b.to_bits(), "literal 17");
println!("18 f32 identities verified");
}
