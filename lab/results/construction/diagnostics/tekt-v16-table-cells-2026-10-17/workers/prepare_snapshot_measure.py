from pathlib import Path
out=Path(__file__).resolve().parents[1];s=(out/'workers/public_measure.rs').read_text().replace('fn main(){','fn original_measure(){',1)
s+=r'''
use comemo::Track;
fn main(){
 use typst_core::entities::{content::ShowRecipeContent,engine::Engine,style::{Styles},style_chain::StyleChain,world_types::{Route,Sink},show::ShowRule};
 use typst_core::compiler::{layout::FixedMetrics,realize::realize_table_insertions};
 for mode in ["root","styled"] {for on in [false,true]{
  let world=NullWorld::default();let recipe=Content::ShowRecipe(Arc::new(ShowRecipeContent{body:Content::Empty,textual:false,context_id_floor:77,rules:Arc::from([]),styles:StyleChain::empty(),guards:vec![],file:world.main()}));let snapshot=Value::Content(source(recipe,on));
  let cell=Content::table_cell(Content::text("x"),None,None,None,None);let mut styles=StyleChain::default_chain();let content=if mode=="styled"{Content::styled(Box::new(cell),Styles::default().push_custom("eval.context.recipe",snapshot))}else{styles=styles.push_custom("eval.context.recipe",snapshot);cell};
  let route=Route::root().with_id(world.main());let mut rules:Arc<[ShowRule]>=Arc::from([]);let mut guards=vec![];let metrics=FixedMetrics;let mut sink=Sink::new();let mut ctx=EvalContext::new();
  {let mut tracked=sink.track_mut();let mut engine=Engine{world:&world,font_metrics:&metrics,route:route.track(),styles:&mut styles,show_rules:&mut rules,active_guards:&mut guards,current_file:world.main(),sink:&mut tracked};realize_table_insertions(content,&mut ctx,&mut engine).unwrap();}
  println!("snapshot mode={mode} source={on} context_floor={}",ctx.next_context_id);
 }}
}
''';(out/'workers/public_snapshot.rs').write_text(s)
w=(out/'workers/public_measure.py').read_text().replace("'workers/public_measure.rs'","'workers/public_snapshot.rs'").replace("'public-'","'snapshot-'").replace("'/tmp/tekt-table-cells-public-'","'/tmp/tekt-table-cells-snapshot-'")
w=w.replace("record=dict(start=", "comemo=list((root/'target/debug/deps').glob('libcomemo-*.rlib'));assert len(comemo)==1;cmd[1:1]=['--extern','comemo='+str(comemo[0])];record=dict(start=")
(out/'workers/snapshot_measure.py').write_text(w)
