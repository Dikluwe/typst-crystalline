#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "time"

ROOT = File.expand_path("../../..", __dir__)
CORE = "01_core/src/compiler/stdlib/structural/math.rs"
LAYOUT = "01_core/src/compiler/math/layout/delimited.rs"
SPACING = "01_core/src/compiler/math/layout/spacing.rs"
EVAL = "01_core/src/compiler/eval/mod.rs"

mutations = [
  ["M01", CORE, "    dict.insert(\n        \"mid\".into(),\n        Value::Func(crate::entities::func::Func::native(\"mid\", native_math_mid)),\n    );\n", ""],
  ["M02", CORE, "    Ok(Content::math_mid(body))", "    Ok(body)"],
  ["M03", CORE, "    if args.named.contains_key(\"body\") {", "    if false && args.named.contains_key(\"body\") {"],
  ["M04", CORE, "        [] => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"missing argument: body\".to_string(),\n            )])\n        }", "        [] => Content::Empty,", 4],
  ["M05", CORE, "        _ => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"unexpected argument\".to_string(),\n            )])\n        }", "        [value, ..] => math_lr_content_value(value, args.span)?,", 2],
  ["M06", CORE, "        other => Err(vec![SourceDiagnostic::error(\n            span,\n            format!(\"expected content, found {}\", vanilla_type_name_class(other)),\n        )]),", "        Value::Int(_) => Ok(Content::Empty),\n        other => Err(vec![SourceDiagnostic::error(\n            span,\n            format!(\"expected content, found {}\", vanilla_type_name_class(other)),\n        )]),", 4],
  ["M07", SPACING, "        Content::MathMid(_) => (MathClass::Relation, MathClass::Relation),", "        Content::MathMid(_) => (MathClass::Normal, MathClass::Normal),"],
  ["M08", LAYOUT, "                self.mid_target_pt.set(Some(target));", "                self.mid_target_pt.set(None);"],
  ["M09", LAYOUT, "                let inherited = self.mid_target_pt.replace(None);", "                let inherited = self.mid_target_pt.get();"],
  ["M10", EVAL, "        ObservationRelation::Same.and(self.body.observation_relation(&other.body))", "        ObservationRelation::Same", 2]
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

originals = mutations.map { |entry| entry[1] }.uniq.to_h do |path|
  [path, File.binread(File.join(ROOT, path))]
end

results = []
orders = [["forward", mutations], ["forward_repeat", mutations], ["reverse", mutations.reverse]]
begin
  orders.each do |order_name, ordered|
    ordered.each do |id, path, before, after, occurrence|
      absolute = File.join(ROOT, path)
      File.binwrite(absolute, replace_nth(originals.fetch(path), before, after, occurrence || 1))
      started = Time.now.utc
      output, status = Open3.capture2e("cargo test -q -p typst-core p1437_", chdir: ROOT)
      results << {
        order: order_name,
        mutant: id,
        classification: status.success? ? "Preserved" : "Violated",
        exit_code: status.exitstatus,
        started_at: started.iso8601(6),
        finished_at: Time.now.utc.iso8601(6),
        command: "cargo test -q -p typst-core p1437_",
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
  step: 1437,
  regime: "executed_without_isolation_attestation",
  orders: orders.map(&:first),
  executions: results.length,
  killed: killed,
  mutation_score: killed.fdiv(results.length),
  results: results
}
receipt_dir = File.join(ROOT, "00_nucleo/diagnosticos/p1437-mutations/receipts")
Dir.mkdir(receipt_dir) unless Dir.exist?(receipt_dir)
receipt = ENV["MUTANT"] ? "focal-#{ENV['MUTANT'].downcase}.json" : "campaign.json"
File.write(File.join(receipt_dir, receipt), JSON.pretty_generate(report) + "\n")
puts JSON.pretty_generate(report.reject { |key, _| key == :results })
exit(killed == results.length ? 0 : 3)
