#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"
require "time"

ROOT = File.expand_path("../../..", __dir__)
CORE = "01_core/src/compiler/stdlib/structural/math.rs"
REPR = "01_core/src/compiler/eval/repr.rs"
ELEM = "01_core/src/entities/elements/math_stretch.rs"

mutations = [
  ["M01", CORE, :global, "    dict.insert(\n        \"stretch\".into(),\n        Value::Func(crate::entities::func::Func::native(\"stretch\", native_math_stretch)),\n    );\n", ""],
  ["M02", CORE, :stretch, "    Ok(Content::math_stretch(body, size, size_explicit))", "    Ok(body)"],
  ["M03", CORE, :stretch, "    Ok(Content::math_stretch(body, size, size_explicit))", "    Ok(Content::math_stretch(body, size, false))"],
  ["M04", CORE, :stretch, "    if args.named.contains_key(\"body\") {", "    if false && args.named.contains_key(\"body\") {"],
  ["M05", CORE, :stretch, "        [] => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"missing argument: body\".to_string(),\n            )])\n        }", "        [] => Content::Empty,"],
  ["M06", CORE, :stretch, "        _ => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"unexpected argument\".to_string(),\n            )])\n        }", "        [value, ..] => math_lr_content_value(value, args.span)?,"],
  ["M07", CORE, :content, "        Value::None => Ok(Content::Empty),\n        other => Err(vec![SourceDiagnostic::error(", "        Value::None | Value::Int(_) => Ok(Content::Empty),\n        other => Err(vec![SourceDiagnostic::error("],
  ["M08", REPR, :global, "            if stretch.size_explicit {", "            if false && stretch.size_explicit {"],
  ["M09", REPR, :global, "            let mut fields = vec![format!(\"body: {}\", repr_math_lr_body(&stretch.body))];", "            let mut fields = Vec::new();"],
  ["M10", ELEM, :global, "            \"size\" if self.size_explicit => Some(Value::Relative(self.size)),", "            \"size\" if false => Some(Value::Relative(self.size)),"]
]

def mutate(original, scope, before, after)
  region = original
  prefix = ""
  suffix = ""
  if scope == :stretch
    start = original.index("pub(crate) fn math_stretch_from_args") or raise "stretch start"
    finish = original.index("pub fn native_math_stretch", start) or raise "stretch end"
    prefix = original[0...start]
    region = original[start...finish]
    suffix = original[finish..]
  elsif scope == :content
    start = original.index("fn math_lr_content_value") or raise "content start"
    finish = original.index("/// Validação e construção compartilhada de `math.lr`", start) or raise "content end"
    prefix = original[0...start]
    region = original[start...finish]
    suffix = original[finish..]
  end
  count = region.scan(before).length
  raise "precondition count #{count}" unless count == 1
  prefix + region.sub(before, after) + suffix
end

originals = mutations.map { |m| m[1] }.uniq.to_h { |path| [path, File.binread(File.join(ROOT, path)).force_encoding(Encoding::UTF_8)] }
results = []
orders = [["forward", mutations], ["forward_repeat", mutations], ["reverse", mutations.reverse]]
begin
  orders.each do |order_name, ordered|
    ordered.each do |id, path, scope, before, after|
      original = originals.fetch(path)
      File.binwrite(File.join(ROOT, path), mutate(original, scope, before, after))
      started = Time.now.utc
      command = "cargo test -q -p typst-core p1446_"
      output, status = Open3.capture2e("bash", "-lc", command, chdir: ROOT)
      results << { order: order_name, mutant: id, classification: status.success? ? "Preserved" : "Violated", exit_code: status.exitstatus, started_at: started.iso8601(6), finished_at: Time.now.utc.iso8601(6), command: command, tail: output.lines.last(12).join }
      File.binwrite(File.join(ROOT, path), original)
    end
  end
ensure
  originals.each { |path, content| File.binwrite(File.join(ROOT, path), content) }
end

killed = results.count { |r| r[:classification] == "Violated" }
report = { schema: "tekt.materialization.mutation-report/v1", step: 1446, regime: "executed_without_isolation_attestation", orders: orders.map(&:first), executions: results.length, killed: killed, mutation_score: killed.fdiv(results.length), unknown: 0, results: results }
receipt_dir = File.join(ROOT, "00_nucleo/diagnosticos/p1446-mutations/receipts")
FileUtils.mkdir_p(receipt_dir)
File.write(File.join(receipt_dir, "campaign.json"), JSON.pretty_generate(report) + "\n")
puts JSON.pretty_generate(report.reject { |key, _| key == :results })
exit(killed == results.length ? 0 : 3)
