# frozen_string_literal: true

require "pathname"
require "yaml"

root = Pathname.new(File.expand_path("..", __dir__))
workflow = YAML.safe_load(root.join(".github/workflows/release.yml").read(encoding: "UTF-8"), aliases: true)
package_step = workflow.fetch("jobs").fetch("cli").fetch("steps").find do |step|
  step["name"] == "Package CLI"
end
abort("release workflow has no Package CLI step") unless package_step

script = package_step.fetch("run")
required = [
  'runner_temp="$RUNNER_TEMP"',
  '[[ "$RUNNER_OS" == "Windows" ]]',
  'runner_temp="$(cygpath -u "$RUNNER_TEMP")"',
  'tar -czf "$runner_temp/${package}.tar.gz" -C "$runner_temp" "$package"'
]

missing = required.reject { |snippet| script.include?(snippet) }
abort("Package CLI does not normalize Windows paths: #{missing.join(", ")}") unless missing.empty?
abort("Package CLI still passes RUNNER_TEMP directly to tar") if script.include?('tar -czf "$RUNNER_TEMP/')

puts "Release workflow tests passed."
