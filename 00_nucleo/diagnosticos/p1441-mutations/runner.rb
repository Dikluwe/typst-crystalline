#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"
require "time"

ROOT = File.expand_path("../../..", __dir__)
CORE = "01_core/src/compiler/stdlib/structural/math.rs"
EVAL = "01_core/src/compiler/eval/math.rs"
REPR = "01_core/src/compiler/eval/repr.rs"
UNDERLINE = "01_core/src/compiler/math/layout/underline.rs"
SPACING = "01_core/src/compiler/math/layout/spacing.rs"

mutations = [
  ["M01", CORE,
   "    dict.insert(\n        \"overline\".into(),\n        Value::Func(crate::entities::func::Func::native(\n            \"math.overline\",\n            native_math_overline,\n        )),\n    );\n", ""],
  ["M02", CORE,
   "    Ok(Content::math_overline(body))",
   "    Ok(Content::overline(body, None, None, None))"],
  ["M03", REPR,
   "            format!(\"overline(body: {})\", repr_math_lr_body(&o.body))",
   "            format!(\"overline[{}]\", repr_math_lr_body(&o.body))"],
  ["M04", CORE,
   "pub(crate) fn math_overline_from_args(args: &Args) -> SourceResult<Content> {\n    if args.named.contains_key(\"body\") {\n        return Err(vec![SourceDiagnostic::error(\n            args.span,\n            \"the argument `body` is positional\".to_string(),\n        )]);\n    }",
   "pub(crate) fn math_overline_from_args(args: &Args) -> SourceResult<Content> {\n    if let Some(body) = args.named.get(\"body\") {\n        return Ok(Content::math_overline(math_lr_content_value(body, args.span)?));\n    }"],
  ["M05", CORE,
   "        [body] => math_lr_content_value(body, args.span)?,\n        [] => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"missing argument: body\".to_string(),\n            )])\n        }\n        [_, _, ..] => {",
   "        [body] => math_lr_content_value(body, args.span)?,\n        [] => Content::Empty,\n        [_, _, ..] => {"],
  ["M06", CORE,
   "    let body = match args.items.as_slice() {\n        [body] => math_lr_content_value(body, args.span)?,\n        [] => {",
   "    let body = match args.items.as_slice() {\n        [body, ..] => math_lr_content_value(body, args.span)?,\n        [] => {"],
  ["M07", CORE,
   "fn math_lr_content_value(value: &Value, span: Span) -> SourceResult<Content> {\n    match value {\n        Value::Content(content) => Ok(content.clone()),\n        Value::LocatedContent(content, _) => Ok(content.content().clone()),\n        Value::Str(text) => Ok(Content::text(text.as_str())),\n        Value::Symbol(symbol) => Ok(Content::MathText(symbol.value.clone())),\n        Value::None => Ok(Content::Empty),",
   "fn math_lr_content_value(value: &Value, span: Span) -> SourceResult<Content> {\n    match value {\n        Value::Content(content) => Ok(content.clone()),\n        Value::LocatedContent(content, _) => Ok(content.content().clone()),\n        Value::Str(text) => Ok(Content::text(text.as_str())),\n        Value::Symbol(symbol) => Ok(Content::MathText(symbol.value.clone())),\n        Value::None | Value::Int(_) => Ok(Content::Empty),"],
  ["M08", EVAL,
   "                \"overline\" => {\n                    let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;\n                    crate::compiler::stdlib::math_overline_from_args(&args)\n                }\n\n",
   ""],
  ["M09", UNDERLINE,
   "    let cramped_style = TextStyle { cramped: true, ..style.clone() };",
   "    let cramped_style = TextStyle { cramped: false, ..style.clone() };"],
  ["M10", UNDERLINE,
   "    let line_y = -(body.ascent + gap + thickness / 2.0);",
   "    let line_y = body.descent + gap + thickness / 2.0;"],
  ["M11", UNDERLINE,
   "        end: Point { x: Pt(body.width), y: Pt(line_y) },",
   "        end: Point { x: Pt((body.width - 1.25).max(0.0)), y: Pt(line_y) },"],
  ["M12", SPACING,
   "        Content::MathOverline(e) => node_math_class(&e.body),",
   "        Content::MathOverline(_) => (MathClass::Normal, MathClass::Normal),"],
  ["M13", CORE,
   "            \"math.overline\",\n            native_math_overline,",
   "            \"overline\",\n            native_math_overline,"]
]

def replace_exact(text, before, after)
  count = text.scan(before).length
  raise "precondition count #{count}, expected 1: #{before.inspect}" unless count == 1
  text.sub(before, after)
end

originals = mutations.map { |entry| entry[1] }.uniq.to_h do |path|
  [path, File.binread(File.join(ROOT, path)).force_encoding(Encoding::UTF_8)]
end

mutations.each do |id, path, before, _after|
  count = originals.fetch(path).scan(before).length
  raise "#{id} precondition count #{count}, expected 1" unless count == 1
end

results = []
orders = [["forward", mutations], ["forward_repeat", mutations], ["reverse", mutations.reverse]]
begin
  orders.each do |order_name, ordered|
    ordered.each do |id, path, before, after|
      absolute = File.join(ROOT, path)
      File.binwrite(absolute, replace_exact(originals.fetch(path), before, after))
      started = Time.now.utc
      command = "cargo test -q -p typst-core p1441_"
      output, status = Open3.capture2e("bash", "-lc", command, chdir: ROOT)
      results << {
        order: order_name,
        mutant: id,
        classification: status.success? ? "Preserved" : "Violated",
        exit_code: status.exitstatus,
        started_at: started.iso8601(6),
        finished_at: Time.now.utc.iso8601(6),
        command: command,
        tail: output.lines.last(12).join
      }
      File.binwrite(absolute, originals.fetch(path))
    end
  end
ensure
  originals.each { |path, content| File.binwrite(File.join(ROOT, path), content) }
end

killed = results.count { |result| result[:classification] == "Violated" }
report = {
  schema: "tekt.materialization.mutation-report/v1",
  step: 1441,
  regime: "executed_without_isolation_attestation",
  orders: orders.map(&:first),
  executions: results.length,
  killed: killed,
  mutation_score: killed.fdiv(results.length),
  unknown: 0,
  results: results
}
receipt_dir = File.join(ROOT, "00_nucleo/diagnosticos/p1441-mutations/receipts")
FileUtils.mkdir_p(receipt_dir)
File.write(File.join(receipt_dir, "campaign-final.json"), JSON.pretty_generate(report) + "\n")
puts JSON.pretty_generate(report.reject { |key, _| key == :results })
exit(killed == results.length ? 0 : 3)
