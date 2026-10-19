#[path="/repos/Antigravity/typst-crystalline/01_core/src/entities/duration.rs"] mod duration;
fn main() {
 for (name,nanos) in [("min",i128::MIN),("min_plus_one",i128::MIN+1),("max",i128::MAX), ("zero",0), ("negative_second",-1_000_000_000)] {
  let r=std::panic::catch_unwind(||duration::Duration::from_nanos(nanos).to_string());
  println!("{}\t{:?}",name,r);
 }
}
