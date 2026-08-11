# frozen_string_literal: true

require "pathname"
require "open3"

ROOT = Pathname.new(File.expand_path("..", __dir__))
REQUIRED = %w[
  README.md
  README.zh-CN.md
  CHANGELOG.md
  CODE_OF_CONDUCT.md
  CONTRIBUTING.md
  LICENSE
  SECURITY.md
  SUPPORT.md
].freeze

errors = []

REQUIRED.each do |relative|
  path = ROOT.join(relative)
  errors << "missing required file: #{relative}" unless path.file?
end

tracked_output, tracked_status = Open3.capture2("git", "-C", ROOT.to_s, "ls-files", "-z")
abort("git ls-files failed") unless tracked_status.success?
tracked = tracked_output.split("\0")

tracked.each do |relative|
  path = ROOT.join(relative)
  next unless path.file?
  next if path.binread.include?("\0")

  bytes = path.binread
  errors << "UTF-8 BOM is forbidden: #{relative}" if bytes.start_with?("\xEF\xBB\xBF".b)
  text = bytes.force_encoding(Encoding::UTF_8)
  errors << "invalid UTF-8: #{relative}" unless text.valid_encoding?
end

tracked.select { |relative| relative.end_with?(".md") }.sort.each do |relative|
  path = ROOT.join(relative)
  text = path.read(encoding: "UTF-8")
  text.scan(/\[[^\]]*\]\(([^)]+)\)/).flatten.each do |target|
    target = target.split(/\s+/, 2).first.delete_prefix("<").delete_suffix(">")
    next if target.empty? || target.start_with?("#", "https://", "http://", "mailto:")

    local = target.split("#", 2).first
    next if local.empty?

    resolved = path.dirname.join(local).cleanpath
    errors << "broken local link in #{path.relative_path_from(ROOT)}: #{target}" unless resolved.exist?
  end
end

readme = ROOT.join("README.md").read(encoding: "UTF-8")
chinese = ROOT.join("README.zh-CN.md").read(encoding: "UTF-8")
errors << "README.md must link to README.zh-CN.md" unless readme.include?("README.zh-CN.md")
errors << "README.zh-CN.md must link to README.md" unless chinese.include?("README.md")

if errors.any?
  warn errors.join("\n")
  exit 1
end

puts "Documentation contracts passed (#{tracked.length} tracked files scanned)."
