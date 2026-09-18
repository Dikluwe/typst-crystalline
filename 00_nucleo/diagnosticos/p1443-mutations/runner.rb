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
LAYOUT = "01_core/src/compiler/math/layout/mod.rs"
SPACING = "01_core/src/compiler/math/layout/spacing.rs"

mutations = [
  ["M01", CORE,
   "    dict.insert(\n        \"overshell\".into(),\n        Value::Func(crate::entities::func::Func::native(\n            \"overshell\",\n            native_math_overshell,\n        )),\n    );\n", ""],
  ["M02", CORE,
   "    Ok(Content::math_overshell(body, annotation, annotation_explicit))",
   "    Ok(Content::math_underover(body, None, annotation))"],
  ["M03", CORE,
   "    Ok(Content::math_overshell(body, annotation, annotation_explicit))",
   "    Ok(Content::math_overshell(body, annotation, false))"],
  ["M04", REPR, "            if overshell.annotation_explicit {", "            if false {"],
  ["M05", CORE,
   "pub(crate) fn math_overshell_from_args(args: &Args) -> SourceResult<Content> {\n    if args.named.contains_key(\"body\") {\n        return Err(vec![SourceDiagnostic::error(\n            args.span,\n            \"the argument `body` is positional\".to_string(),\n        )]);\n    }\n    if let Some(name) = args.named.keys().next() {",
   "pub(crate) fn math_overshell_from_args(args: &Args) -> SourceResult<Content> {\n    if args.named.contains_key(\"body\") {\n        return Err(vec![SourceDiagnostic::error(\n            args.span,\n            \"the argument `body` is positional\".to_string(),\n        )]);\n    }\n    if let Some(name) = args.named.keys().find(|name| name.as_str() != \"annotation\") {"],
  ["M06", CORE,
   "        [] => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"missing argument: body\".to_string(),\n            )])\n        }\n        [_, _] | [_, _, ..] => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"unexpected argument\".to_string(),\n            )])\n        }\n    };\n    Ok(Content::math_overshell",
   "        [] => (Content::Empty, None, false),\n        [_, _] | [_, _, ..] => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"unexpected argument\".to_string(),\n            )])\n        }\n    };\n    Ok(Content::math_overshell"],
  ["M07", CORE,
   "        [_, _] | [_, _, ..] => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"unexpected argument\".to_string(),\n            )])\n        }\n    };\n    Ok(Content::math_overshell",
   "        [body, annotation, ..] => (math_lr_content_value(body, args.span)?, Some(math_lr_content_value(annotation, args.span)?), true)\n    };\n    Ok(Content::math_overshell"],
  ["M08", CORE,
   "fn math_lr_content_value(value: &Value, span: Span) -> SourceResult<Content> {\n    match value {\n        Value::Content(content) => Ok(content.clone()),\n        Value::LocatedContent(content, _) => Ok(content.content().clone()),\n        Value::Str(text) => Ok(Content::text(text.as_str())),\n        Value::Symbol(symbol) => Ok(Content::MathText(symbol.value.clone())),\n        Value::None => Ok(Content::Empty),",
   "fn math_lr_content_value(value: &Value, span: Span) -> SourceResult<Content> {\n    match value {\n        Value::Content(content) => Ok(content.clone()),\n        Value::LocatedContent(content, _) => Ok(content.content().clone()),\n        Value::Str(text) => Ok(Content::text(text.as_str())),\n        Value::Symbol(symbol) => Ok(Content::MathText(symbol.value.clone())),\n        Value::None | Value::Int(_) => Ok(Content::Empty),"],
  ["M09", LAYOUT, "    let shell = Content::MathText(\"⏠\".into());", "    let shell = Content::MathText(\"⏡\".into());"],
  ["M10", LAYOUT,
   "    let inner = Content::math_underover(elem.body.clone(), None, Some(shell));",
   "    let inner = Content::math_underover(elem.body.clone(), Some(shell), None);"],
  ["M11", SPACING,
   "        Content::MathOvershell(e) => node_math_class(&e.body),",
   "        Content::MathOvershell(_) => (MathClass::Normal, MathClass::Normal),"],
  ["M12", EVAL,
   "                \"overshell\" => {\n                    let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;\n                    crate::compiler::stdlib::math_overshell_from_args(&args)\n                }\n\n",
   ""]
]

def replace_exact(text, before, after)
  count = text.scan(before).length
  raise "precondition count #{count}, expected 1: #{before.inspect}" unless count == 1
  text.sub(before, after)
end

originals = mutations.map { |entry| entry[1] }.uniq.to_h do |path|
  [path, File.binread(File.join(ROOT, path)).force_encoding(Encoding::UTF_8)]
end

# Falha antes da campanha se qualquer âncora tiver derivado.
mutations.each { |_, path, before, _| replace_exact(originals.fetch(path), before, before) }

results = []
orders = [["forward", mutations], ["forward_repeat", mutations], ["reverse", mutations.reverse]]
begin
  orders.each do |order_name, ordered|
    ordered.each do |id, path, before, after|
      absolute = File.join(ROOT, path)
      File.binwrite(absolute, replace_exact(originals.fetch(path), before, after))
      started = Time.now.utc
      command = "cargo test -q -p typst-core p1443_"
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
  step: 1443,
  regime: "executed_without_isolation_attestation",
  orders: orders.map(&:first),
  executions: results.length,
  killed: killed,
  mutation_score: killed.fdiv(results.length),
  unknown: 0,
  results: results
}
receipt_dir = File.join(ROOT, "00_nucleo/diagnosticos/p1443-mutations/receipts")
FileUtils.mkdir_p(receipt_dir)
File.write(File.join(receipt_dir, "campaign-final.json"), JSON.pretty_generate(report) + "\n")
puts JSON.pretty_generate(report.reject { |key, _| key == :results })
exit(killed == results.length ? 0 : 3)
