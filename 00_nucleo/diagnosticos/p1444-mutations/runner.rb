#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"
require "time"

ROOT = File.expand_path("../../..", __dir__)
CORE = "01_core/src/compiler/stdlib/structural/math.rs"
CONTENT = "01_core/src/entities/content.rs"
EVAL = "01_core/src/compiler/eval/math.rs"
REPR = "01_core/src/compiler/eval/repr.rs"
ATTACH = "01_core/src/compiler/math/layout/attach.rs"
SPACING = "01_core/src/compiler/math/layout/spacing.rs"

mutations = [
  ["M01", CORE,
   "    dict.insert(\n        \"primes\".into(),\n        Value::Func(crate::entities::func::Func::native(\"primes\", native_math_primes)),\n    );\n", ""],
  ["M02", EVAL,
   "        Expr::MathPrimes(primes) => Ok(Content::math_primes(primes.count())),",
   "        Expr::MathPrimes(primes) => Ok(Content::MathText(\"′\".repeat(primes.count()).into())),"],
  ["M03", CONTENT,
   "        Self::MathPrimes(Arc::new(MathPrimesElem { count }))",
   "        Self::MathPrimes(Arc::new(MathPrimesElem { count: count + 1 }))"],
  ["M04", REPR,
   "        Content::MathPrimes(p) => format!(\"primes(count: {})\", p.count),",
   "        Content::MathPrimes(_) => \"primes(count: 0)\".to_string(),"],
  ["M05", CORE,
   "    if args.named.contains_key(\"count\") {",
   "    if false && args.named.contains_key(\"count\") {"],
  ["M06", CORE,
   "        [Value::Int(count)] if *count < 0 => {",
   "        [Value::Int(count)] if false && *count < 0 => {"],
  ["M07", CORE,
   "        [_, _, ..] => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"unexpected argument\".to_string(),\n            )])\n        }\n    };\n    Ok(Value::Content(Content::math_primes(count)))",
   "        [Value::Int(count), _, ..] => usize::try_from(*count).unwrap(),\n        [_, _, ..] => {\n            return Err(vec![SourceDiagnostic::error(\n                args.span,\n                \"unexpected argument\".to_string(),\n            )])\n        }\n    };\n    Ok(Value::Content(Content::math_primes(count)))"],
  ["M08", ATTACH, "            glyphs: vec!['‴'],", "            glyphs: vec!['″'],"],
  ["M09", ATTACH, "            half_width_extent: count + 1,", "            half_width_extent: count,"],
  ["M10", EVAL,
   "                sup.map_or(MathAttachSlot::Omitted, MathAttachSlot::Present),",
   "                MathAttachSlot::Omitted,"],
  ["M11", SPACING,
   "        Content::MathPrimes(_) => MathClass::Normal,",
   "        Content::MathPrimes(_) => MathClass::Alphabetic,"],
  ["M12", EVAL,
   "            let primes = (prime_count > 0).then(|| Content::math_primes(prime_count));",
   "            let primes = (prime_count > 0).then(|| Content::MathText(\"′\".repeat(prime_count).into()));"]
]

def replace_exact(text, before, after)
  count = text.scan(before).length
  raise "precondition count #{count}, expected 1: #{before.inspect}" unless count == 1
  text.sub(before, after)
end

originals = mutations.map { |entry| entry[1] }.uniq.to_h do |path|
  [path, File.binread(File.join(ROOT, path)).force_encoding(Encoding::UTF_8)]
end
mutations.each { |_, path, before, _| replace_exact(originals.fetch(path), before, before) }

results = []
orders = [["forward", mutations], ["forward_repeat", mutations], ["reverse", mutations.reverse]]
begin
  orders.each do |order_name, ordered|
    ordered.each do |id, path, before, after|
      absolute = File.join(ROOT, path)
      File.binwrite(absolute, replace_exact(originals.fetch(path), before, after))
      started = Time.now.utc
      command = "cargo test -q -p typst-core p1444_"
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
  step: 1444,
  regime: "executed_without_isolation_attestation",
  orders: orders.map(&:first),
  executions: results.length,
  killed: killed,
  mutation_score: killed.fdiv(results.length),
  unknown: 0,
  results: results
}
receipt_dir = File.join(ROOT, "00_nucleo/diagnosticos/p1444-mutations/receipts")
FileUtils.mkdir_p(receipt_dir)
File.write(File.join(receipt_dir, "campaign-final.json"), JSON.pretty_generate(report) + "\n")
puts JSON.pretty_generate(report.reject { |key, _| key == :results })
exit(killed == results.length ? 0 : 3)
