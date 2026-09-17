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
QUERY = "03_infra/src/query_helpers.rs"

mutations = [
  ["M01", CORE, "    dict.insert(\n        \"lr\".into(),\n        Value::Func(crate::entities::func::Func::native(\"lr\", native_math_lr)),\n    );\n", ""],
  ["M02", CORE, "    Ok(Content::math_lr(body, size, size_explicit))", "    Ok(body)"],
  ["M03", CORE, "        0 => Content::Empty,", "        0 => Content::text(\"invalid-zero\"),", 2],
  ["M04", CORE, "            body.push(Content::text(\",\"));", "            body.push(Content::text(\";\"));"],
  ["M05", CORE, "    Ok(Content::math_lr(body, size, size_explicit))", "    Ok(Content::math_lr(body, size, false))"],
  ["M06", CORE, "        if name != \"size\" {", "        if false && name != \"size\" {"],
  ["M07", LAYOUT, "    if elem.size_explicit {", "    if false && elem.size_explicit {"],
  ["M08", SPACING, "        Content::MathLr(e) => node_math_class(&e.body),", "        Content::MathLr(_) => (MathClass::Normal, MathClass::Normal),"],
  ["M09", LAYOUT, "    (requested - 0.1 * font_size).max(0.0)", "    (requested - 0.0 * font_size).max(0.0)"],
  ["M10", CORE, "        Value::Content(content) => Ok(content.clone()),", "        Value::Content(Content::MathLr(elem)) => Ok(elem.body.clone()),\n        Value::Content(content) => Ok(content.clone()),", 4],
  ["M11", EVAL, "            .and(self.size.observation_relation(&other.size))\n            .and(self.size_explicit.observation_relation(&other.size_explicit))", "            .and(self.size_explicit.observation_relation(&other.size_explicit))"],
  ["M12", QUERY, "        | Content::MathDelimited(_) | Content::MathLr(_) | Content::MathAlignPoint(_)", "        | Content::MathDelimited(_) | Content::MathAlignPoint(_)", 1, "        Content::Heading(_)", "        Content::MathLr(elem) => has_any_text(&elem.body),\n        Content::Heading(_)"],
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

def command_for(id)
  if id == "M12"
    "cargo test -q -p typst-infra p1436"
  else
    "cargo test -q -p typst-core p1436"
  end
end

originals = mutations.map { |entry| entry[1] }.uniq.to_h do |path|
  [path, File.binread(File.join(ROOT, path))]
end

results = []
orders = [["forward", mutations], ["forward_repeat", mutations], ["reverse", mutations.reverse]]
begin
  orders.each do |order_name, ordered|
    ordered.each do |entry|
      id, path, before, after, occurrence, anchor, anchored = entry
      occurrence ||= 1
      absolute = File.join(ROOT, path)
      mutated = replace_nth(originals.fetch(path), before, after, occurrence)
      mutated = replace_nth(mutated, anchor, anchored, 1) if anchor
      File.binwrite(absolute, mutated)
      started = Time.now.utc
      output, status = Open3.capture2e(command_for(id), chdir: ROOT)
      results << {
        order: order_name,
        mutant: id,
        classification: status.success? ? "Preserved" : "Violated",
        exit_code: status.exitstatus,
        started_at: started.iso8601(6),
        finished_at: Time.now.utc.iso8601(6),
        command: command_for(id),
        tail: output.lines.last(12).join,
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
  step: 1436,
  regime: "executed_without_isolation_attestation",
  orders: orders.map(&:first),
  executions: results.length,
  killed: killed,
  mutation_score: killed.fdiv(results.length),
  results: results,
}
receipt_name = ENV["MUTANT"] ? "campaign-#{ENV['MUTANT'].downcase}-r2.json" : "campaign.json"
receipt = File.join(ROOT, "00_nucleo/diagnosticos/p1436-mutations/receipts", receipt_name)
File.write(receipt, JSON.pretty_generate(report) + "\n")
puts JSON.pretty_generate(report.reject { |key, _| key == :results })
exit(killed == results.length ? 0 : 3)
