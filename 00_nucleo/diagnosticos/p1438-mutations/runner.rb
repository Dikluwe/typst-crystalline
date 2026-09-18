#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"
require "time"

ROOT = File.expand_path("../../..", __dir__)
CORE = "01_core/src/compiler/stdlib/structural/math.rs"
EVAL = "01_core/src/compiler/eval/math.rs"

NORM_START = "pub(crate) fn math_norm_from_args"
NORM_END = "pub fn native_math_norm"
CONTENT_START = "fn math_lr_content_value"
CONTENT_END = "/// Validação e construção compartilhada de `math.lr`"

mutations = [
  ["M01", CORE, :global,
   "    dict.insert(\n        \"norm\".into(),\n        Value::Func(crate::entities::func::Func::native(\"norm\", native_math_norm)),\n    );\n", ""],
  ["M02", CORE, :norm,
   "    Ok(Content::math_lr(\n        Content::MathSequence(\n            vec![Content::MathText(\"‖\".into()), body, Content::MathText(\"‖\".into())]\n                .into(),\n        ),\n        size,\n        size_explicit,\n    ))",
   "    Ok(Content::math_delimited('‖', body, '‖'))"],
  ["M03", CORE, :norm,
   "vec![Content::MathText(\"‖\".into()), body, Content::MathText(\"‖\".into())]",
   "vec![body, Content::MathText(\"‖\".into())]"],
  ["M04", CORE, :norm, "Content::MathText(\"‖\".into())", "Content::MathText(\"|\".into())", 2],
  ["M05", CORE, :norm, "        size_explicit,\n", "        false,\n"],
  ["M06", CORE, :norm,
   "    if args.named.contains_key(\"body\") {",
   "    if false && args.named.contains_key(\"body\") {"],
  ["M07", CORE, :norm,
   "        [] => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"missing argument: body\".to_string(),\n            )])\n        }",
   "        [] => Content::Empty,"],
  ["M08", CORE, :norm,
   "        _ => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"unexpected argument\".to_string(),\n            )])\n        }",
   "        [value, ..] => math_lr_content_value(value, args.span)?,"],
  ["M09", CORE, :content,
   "        Value::None => Ok(Content::Empty),\n        other => Err(vec![SourceDiagnostic::error(",
   "        Value::None | Value::Int(_) => Ok(Content::Empty),\n        other => Err(vec![SourceDiagnostic::error("],
  ["M10", EVAL, :global,
   "                \"norm\" => {\n                    let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;\n                    crate::compiler::stdlib::math_norm_from_args(&args)\n                }\n\n", ""]
]

mutations.select! { |entry| entry[0] == ENV["MUTANT"] } if ENV["MUTANT"]

def replace_nth(text, before, after, occurrence = 1)
  positions = []
  offset = 0
  while (index = text.index(before, offset))
    positions << index
    offset = index + before.length
  end
  raise "precondition count #{positions.length} < #{occurrence}: #{before.inspect}" if positions.length < occurrence
  index = positions.fetch(occurrence - 1)
  text[0...index] + after + text[(index + before.length)..]
end

def mutate(text, scope, before, after, occurrence)
  return replace_nth(text, before, after, occurrence || 1) if scope == :global

  start_marker, end_marker = if scope == :norm
    [NORM_START, NORM_END]
  else
    [CONTENT_START, CONTENT_END]
  end
  start = text.index(start_marker) or raise "region start missing"
  finish = text.index(end_marker, start) or raise "region end missing"
  region = text[start...finish]
  changed = replace_nth(region, before, after, occurrence || 1)
  text[0...start] + changed + text[finish..]
end

def command_for(id)
  return "cargo test -q -p typst-core p1438_" unless id == "M10"

  expected = "equation(block: true, body: lr(body: sequence([‖], [x], [‖])))"
  "cargo test -q -p typst-core p1438_ && " \
    "test \"$(cargo run -q --bin typst -- eval --format raw 'repr($ norm(x) $)' 2>/dev/null)\" = '#{expected}'"
end

originals = mutations.map { |entry| entry[1] }.uniq.to_h do |path|
  [path, File.binread(File.join(ROOT, path)).force_encoding(Encoding::UTF_8)]
end

results = []
orders = [["forward", mutations], ["forward_repeat", mutations], ["reverse", mutations.reverse]]
begin
  orders.each do |order_name, ordered|
    ordered.each do |id, path, scope, before, after, occurrence|
      absolute = File.join(ROOT, path)
      File.binwrite(absolute, mutate(originals.fetch(path), scope, before, after, occurrence))
      started = Time.now.utc
      command = command_for(id)
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
  step: 1438,
  regime: "executed_without_isolation_attestation",
  orders: orders.map(&:first),
  executions: results.length,
  killed: killed,
  mutation_score: killed.fdiv(results.length),
  unknown: 0,
  results: results
}
receipt_dir = File.join(ROOT, "00_nucleo/diagnosticos/p1438-mutations/receipts")
FileUtils.mkdir_p(receipt_dir)
File.write(File.join(receipt_dir, "campaign.json"), JSON.pretty_generate(report) + "\n")
puts JSON.pretty_generate(report.reject { |key, _| key == :results })
exit(killed == results.length ? 0 : 3)
