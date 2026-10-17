use syn::{visit::{self, Visit}, spanned::Spanned};
struct Scan { path: String }
fn group(p: &syn::Path) -> bool {
 let v: Vec<_> = p.segments.iter().map(|s|s.ident.to_string()).collect();
 v.ends_with(&["FrameItem".to_string(),"Group".to_string()])
}
impl<'ast> Visit<'ast> for Scan {
 fn visit_item_fn(&mut self,n:&'ast syn::ItemFn) {
   println!("{}",serde_json::json!({"path":self.path,"kind":"fn","name":n.sig.ident.to_string(),"line":n.span().start().line,"end_line":n.span().end().line}));
   visit::visit_item_fn(self,n)
 }

 fn visit_macro(&mut self,n:&'ast syn::Macro) {
  use syn::parse::Parser;
  if let Ok(args)=syn::punctuated::Punctuated::<syn::Expr,syn::Token![,]>::parse_terminated.parse2(n.tokens.clone()) {
   for arg in &args { self.visit_expr(arg); }
  }
 }

 fn visit_expr_struct(&mut self,n:&'ast syn::ExprStruct) {
  if group(&n.path) {
   println!("{}",serde_json::json!({"path":self.path,"kind":"expr","line":n.span().start().line,"column":n.span().start().column,"open_line":n.brace_token.span.open().start().line,"open_column":n.brace_token.span.open().start().column,"fields":n.fields.iter().map(|f|match &f.member{syn::Member::Named(i)=>i.to_string(),syn::Member::Unnamed(i)=>i.index.to_string()}).collect::<Vec<_>>()}));
  }
  visit::visit_expr_struct(self,n)
 }
 fn visit_pat_struct(&mut self,n:&'ast syn::PatStruct) {
  if group(&n.path) {
   println!("{}",serde_json::json!({"path":self.path,"kind":"pat","line":n.span().start().line,"column":n.span().start().column,"open_line":n.brace_token.span.open().start().line,"open_column":n.brace_token.span.open().start().column,"rest":n.rest.is_some(),"fields":n.fields.iter().map(|f|match &f.member{syn::Member::Named(i)=>i.to_string(),syn::Member::Unnamed(i)=>i.index.to_string()}).collect::<Vec<_>>()}));
  }
  visit::visit_pat_struct(self,n)
 }
}
fn main(){for path in std::env::args().skip(1){let text=std::fs::read_to_string(&path).unwrap();let file=syn::parse_file(&text).unwrap_or_else(|e|panic!("{path}: {e}"));Scan{path}.visit_file(&file);}}
