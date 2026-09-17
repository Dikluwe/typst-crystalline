#!/usr/bin/env ruby
# frozen_string_literal: true

# P1431 revision-7 design verifier and execution shim.
# The public policy oracle freezes runner.rb at b31a7a...; changing that file
# would make M27 fail on identity before exercising policy.  This shim verifies
# the 30-family design, including the P1433 M26 repair and byte-exact R6
# public oracle identity,
# and delegates physical execution unchanged.

require "digest"
require "json"
require "rbconfig"

ROOT = File.expand_path("../../..", __dir__)
SUITE_PATH = File.join(ROOT, "00_nucleo/diagnosticos/p1431-mutants.json")
PUBLIC_RUNNER = File.join(__dir__, "runner.rb")
ORACLES_PATH = File.join(ROOT, "00_nucleo/diagnosticos/p1431-oracles-r6.json")
IDENTITY_ORACLES_PATH = File.join(ROOT, "00_nucleo/diagnosticos/p1431-oracles.json")

unless ARGV == ["--mode", "verify-design"]
  exec RbConfig.ruby, PUBLIC_RUNNER, *ARGV
end

errors = []
suite = JSON.parse(File.read(SUITE_PATH))
oracles = JSON.parse(File.read(ORACLES_PATH))
identity_oracles = JSON.parse(File.read(IDENTITY_ORACLES_PATH))
mutations = suite.fetch("mutations")
expected_ids = (1..30).map { |n| format("M%02d", n) }
identity = identity_oracles.fetch("identity_serialization")
expected_enumeration = "run cargo test -q -p typst-core p1431 -- --list, then typst-infra; retain lines ending exactly ': test'; strip that suffix; prefix the exact package name"
expected_record = "UTF-8 bytes <package>\t<fully-qualified-test-id>\n; no BOM; LF after every record including the last"

errors << "suite revision is not 7" unless suite.fetch("revision") == 7
errors << "mutation count is not 30" unless mutations.length == 30
errors << "mutation ids are not exhaustive and ordered" unless mutations.map { |m| m.fetch("id") } == expected_ids
errors << "oracle revision is not 6" unless oracles.fetch("revision") == 6
errors << "R6 oracle identity hash mismatch" unless oracles.dig("identity_and_hash_gate", "forward_sha256") == identity.fetch("forward_sha256")
errors << "oracle count is not 34" unless suite.dig("oracles", "test_count") == 34
errors << "public runner identity mismatch" unless Digest::SHA256.file(PUBLIC_RUNNER).hexdigest == suite.dig("runner", "public_policy_sha256")
errors << "identity serialization version mismatch" unless identity.fetch("version") == 1
errors << "identity enumeration algorithm mismatch" unless identity.fetch("enumeration") == expected_enumeration
errors << "identity record algorithm mismatch" unless identity.fetch("record") == expected_record

forward_records = identity.fetch("forward_records")
errors << "identity record count is not 34" unless forward_records.length == 34
forward_digest = Digest::SHA256.hexdigest(forward_records.map { |record| "#{record}\n" }.join)
reverse_digest = Digest::SHA256.hexdigest(forward_records.reverse.map { |record| "#{record}\n" }.join)
errors << "oracle forward identity hash mismatch" unless forward_digest == identity.fetch("forward_sha256")
errors << "oracle reverse identity hash mismatch" unless reverse_digest == identity.fetch("reverse_sha256")
errors << "suite forward identity hash mismatch" unless forward_digest == suite.dig("oracles", "forward_ids_sha256")
errors << "suite reverse identity hash mismatch" unless reverse_digest == suite.dig("oracles", "reverse_ids_sha256")

oracle_digest = Digest::SHA256.file(ORACLES_PATH).hexdigest
errors << "oracle hash mismatch" unless oracle_digest == suite.dig("oracles", "sha256")

orders = suite.fetch("execution_orders")
errors << "execution orders differ from contract" unless orders.map { |o| o.fetch("id") } == %w[forward forward_repeat reverse]
orders.each do |order|
  errors << "#{order.fetch('id')}: oracle count mismatch" unless order.fetch("oracle_test_count") == 34
  errors << "#{order.fetch('id')}: mutation vector mismatch" unless order.fetch("mutations").sort == expected_ids.sort
  expected_order_digest = order.fetch("id") == "reverse" ? reverse_digest : forward_digest
  errors << "#{order.fetch('id')}: oracle identity hash mismatch" unless order.fetch("oracle_ids_sha256") == expected_order_digest
end

mutations.each do |mutation|
  recipe_path = File.join(ROOT, mutation.fetch("recipe"))
  unless File.file?(recipe_path)
    errors << "#{mutation.fetch('id')}: missing recipe"
    next
  end
  digest = Digest::SHA256.file(recipe_path).hexdigest
  errors << "#{mutation.fetch('id')}: recipe hash mismatch" unless digest == mutation.fetch("recipe_sha256")
  recipe = JSON.parse(File.read(recipe_path))
  errors << "#{mutation.fetch('id')}: recipe id mismatch" unless recipe.fetch("id") == mutation.fetch("id")
  errors << "#{mutation.fetch('id')}: target allowlist mismatch" unless recipe.fetch("target_files").sort == mutation.fetch("target_files").sort
  expected_phase = mutation.fetch("id") == "M26" ? "post_candidate_equivalent_mutant_repair_p1433" : "design_without_candidate_then_mechanical_materialization"
  errors << "#{mutation.fetch('id')}: phase rule mismatch" unless recipe.fetch("phase_rule") == expected_phase
end

if errors.empty?
  puts JSON.pretty_generate(
    classification: "Preserved",
    reason_code: "PRESERVED_MUTATION_DESIGN",
    revision: 7,
    mutations: 30,
    oracle_tests: 34,
  )
  exit 0
end

puts JSON.pretty_generate(
  classification: "Unknown",
  reason_code: "UNKNOWN_IDENTITY_MISMATCH",
  detail: errors.join("; "),
)
exit 3
