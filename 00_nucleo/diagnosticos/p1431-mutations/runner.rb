#!/usr/bin/env ruby
# frozen_string_literal: true

# P1431 disposable mutation runner.
#
# This runner is intentionally candidate-agnostic at design-freeze time.  The
# adversary materializes one unified diff from a frozen recipe only after a
# candidate exists.  A materialized diff is executable only when it touches
# the recipe allowlist and applies uniquely to the exact candidate commit.

require "digest"
require "json"
require "open3"
require "optparse"
require "timeout"

ROOT = File.expand_path("../../..", __dir__)
MUTANTS = File.join(ROOT, "00_nucleo/diagnosticos/p1431-mutants.json")

def capture(*argv)
  Open3.capture3(*argv, chdir: ROOT)
end

def fail_unknown(code, detail)
  puts JSON.pretty_generate(
    classification: "Unknown",
    reason_code: code,
    detail: detail,
  )
  exit 3
end

def git_clean?
  _out, _err, status = capture("git", "diff", "--quiet", "--exit-code")
  status.success?
end

def patch_paths(path)
  out, err, status = capture("git", "apply", "--numstat", path)
  fail_unknown("UNKNOWN_EXECUTION_OR_BUDGET", "cannot inspect patch: #{err}") unless status.success?
  out.lines.filter_map do |line|
    fields = line.chomp.split("\t", 3)
    fields[2] if fields.length == 3
  end
end

def verify_design(document)
  errors = []
  errors << "mutation count is not 27" unless document.fetch("mutations").length == 27
  expected = (1..27).map { |n| format("M%02d", n) }
  actual = document.fetch("mutations").map { |m| m.fetch("id") }
  errors << "mutation ids are not exhaustive and ordered" unless actual == expected
  errors << "execution orders differ from frozen contract" unless document.fetch("execution_orders").map { |o| o.fetch("id") } == %w[forward forward_repeat reverse]
  document.fetch("mutations").each do |mutation|
    recipe = File.join(ROOT, mutation.fetch("recipe"))
    unless File.file?(recipe)
      errors << "#{mutation.fetch('id')}: missing recipe"
      next
    end
    digest = Digest::SHA256.file(recipe).hexdigest
    errors << "#{mutation.fetch('id')}: recipe hash mismatch" unless digest == mutation.fetch("recipe_sha256")
    parsed = JSON.parse(File.read(recipe))
    errors << "#{mutation.fetch('id')}: recipe id mismatch" unless parsed.fetch("id") == mutation.fetch("id")
    errors << "#{mutation.fetch('id')}: candidate read unexpectedly authorized before freeze" unless parsed.fetch("phase_rule") == "design_without_candidate_then_mechanical_materialization"
  end
  if errors.empty?
    puts JSON.pretty_generate(classification: "Preserved", reason_code: "PRESERVED_MUTATION_DESIGN", mutations: 27)
    exit 0
  end
  fail_unknown("UNKNOWN_IDENTITY_MISMATCH", errors.join("; "))
end

options = { timeout: 240 }
parser = OptionParser.new do |opts|
  opts.on("--mode MODE") { |value| options[:mode] = value }
  opts.on("--mutation ID") { |value| options[:mutation] = value }
  opts.on("--candidate SHA") { |value| options[:candidate] = value }
  opts.on("--patch PATH") { |value| options[:patch] = value }
  opts.on("--command COMMAND") { |value| options[:command] = value }
  opts.on("--witness REGEX") { |value| options[:witness] = value }
  opts.on("--timeout SECONDS", Integer) { |value| options[:timeout] = value }
end
parser.parse!

document = JSON.parse(File.read(MUTANTS))
verify_design(document) if options[:mode] == "verify-design"

unless options.values_at(:mode, :mutation, :candidate, :patch, :command, :witness).all?
  fail_unknown("UNKNOWN_EXECUTION_OR_BUDGET", "execute mode requires mutation, candidate, patch, command and witness")
end
fail_unknown("UNKNOWN_EXECUTION_OR_BUDGET", "unsupported mode") unless options[:mode] == "execute"

mutation = document.fetch("mutations").find { |item| item.fetch("id") == options[:mutation] }
fail_unknown("UNKNOWN_IDENTITY_MISMATCH", "unknown mutation id") unless mutation

head, head_err, head_status = capture("git", "rev-parse", "HEAD")
fail_unknown("UNKNOWN_IDENTITY_MISMATCH", head_err) unless head_status.success?
fail_unknown("UNKNOWN_IDENTITY_MISMATCH", "candidate HEAD mismatch") unless head.strip == options[:candidate]
fail_unknown("UNKNOWN_IDENTITY_MISMATCH", "tracked worktree is not clean before mutation") unless git_clean?

patch = File.expand_path(options[:patch], ROOT)
fail_unknown("UNKNOWN_EXECUTION_OR_BUDGET", "materialized patch is missing") unless File.file?(patch)
actual_paths = patch_paths(patch).sort
allowed_paths = mutation.fetch("target_files").sort
unless !actual_paths.empty? && (actual_paths - allowed_paths).empty?
  fail_unknown("UNKNOWN_IDENTITY_MISMATCH", "patch paths #{actual_paths.inspect} exceed #{allowed_paths.inspect}")
end

_check_out, check_err, check_status = capture("git", "apply", "--check", patch)
fail_unknown("UNKNOWN_EXECUTION_OR_BUDGET", "patch does not apply uniquely: #{check_err}") unless check_status.success?

applied = false
begin
  _apply_out, apply_err, apply_status = capture("git", "apply", patch)
  fail_unknown("UNKNOWN_EXECUTION_OR_BUDGET", "patch application failed: #{apply_err}") unless apply_status.success?
  applied = true

  stdout = stderr = ""
  status = nil
  begin
    Timeout.timeout(options[:timeout]) do
      stdout, stderr, status = Open3.capture3("bash", "-lc", options[:command], chdir: ROOT)
    end
  rescue Timeout::Error
    fail_unknown("UNKNOWN_EXECUTION_OR_BUDGET", "mutation command timed out")
  end

  stream = stdout + stderr
  if status.success?
    puts JSON.pretty_generate(
      mutation: mutation.fetch("id"),
      classification: "Preserved",
      reason_code: "MUTATION_SURVIVED",
      candidate: options[:candidate],
      patch_sha256: Digest::SHA256.file(patch).hexdigest,
    )
    exit_code = 2
  elsif Regexp.new(options[:witness], Regexp::MULTILINE).match?(stream)
    puts JSON.pretty_generate(
      mutation: mutation.fetch("id"),
      classification: "Violated",
      reason_code: mutation.fetch("expected_reason_code"),
      candidate: options[:candidate],
      patch_sha256: Digest::SHA256.file(patch).hexdigest,
    )
    exit_code = 0
  else
    fail_unknown("UNKNOWN_EXECUTION_OR_BUDGET", "non-zero command lacked attributable killer witness")
  end
ensure
  if applied
    _reverse_out, reverse_err, reverse_status = capture("git", "apply", "--reverse", patch)
    fail_unknown("UNKNOWN_EXECUTION_OR_BUDGET", "cannot reverse mutation: #{reverse_err}") unless reverse_status.success?
    fail_unknown("UNKNOWN_IDENTITY_MISMATCH", "tracked worktree differs after reversal") unless git_clean?
  end
end

exit exit_code
