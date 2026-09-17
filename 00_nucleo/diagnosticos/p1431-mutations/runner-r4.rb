#!/usr/bin/env ruby
# frozen_string_literal: true

# P1431 revision-4 design verifier and execution shim.
# The public policy oracle freezes runner.rb at b31a7a...; changing that file
# would make M27 fail on identity before exercising policy.  This shim verifies
# the expanded 30-family design and delegates physical execution unchanged.

require "digest"
require "json"
require "rbconfig"

ROOT = File.expand_path("../../..", __dir__)
SUITE_PATH = File.join(ROOT, "00_nucleo/diagnosticos/p1431-mutants.json")
PUBLIC_RUNNER = File.join(__dir__, "runner.rb")
ORACLES_PATH = File.join(ROOT, "00_nucleo/diagnosticos/p1431-oracles.json")

unless ARGV == ["--mode", "verify-design"]
  exec RbConfig.ruby, PUBLIC_RUNNER, *ARGV
end

errors = []
suite = JSON.parse(File.read(SUITE_PATH))
oracles = JSON.parse(File.read(ORACLES_PATH))
mutations = suite.fetch("mutations")
expected_ids = (1..30).map { |n| format("M%02d", n) }

errors << "suite revision is not 4" unless suite.fetch("revision") == 4
errors << "mutation count is not 30" unless mutations.length == 30
errors << "mutation ids are not exhaustive and ordered" unless mutations.map { |m| m.fetch("id") } == expected_ids
errors << "oracle revision is not 4" unless oracles.fetch("revision") == 4
errors << "oracle count is not 34" unless suite.dig("oracles", "test_count") == 34
errors << "public runner identity mismatch" unless Digest::SHA256.file(PUBLIC_RUNNER).hexdigest == suite.dig("runner", "public_policy_sha256")

oracle_digest = Digest::SHA256.file(ORACLES_PATH).hexdigest
errors << "oracle hash mismatch" unless oracle_digest == suite.dig("oracles", "sha256")

orders = suite.fetch("execution_orders")
errors << "execution orders differ from contract" unless orders.map { |o| o.fetch("id") } == %w[forward forward_repeat reverse]
orders.each do |order|
  errors << "#{order.fetch('id')}: oracle count mismatch" unless order.fetch("oracle_test_count") == 34
  errors << "#{order.fetch('id')}: mutation vector mismatch" unless order.fetch("mutations").sort == expected_ids.sort
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
  errors << "#{mutation.fetch('id')}: candidate read authorized before freeze" unless recipe.fetch("phase_rule") == "design_without_candidate_then_mechanical_materialization"
end

if errors.empty?
  puts JSON.pretty_generate(
    classification: "Preserved",
    reason_code: "PRESERVED_MUTATION_DESIGN",
    revision: 4,
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
