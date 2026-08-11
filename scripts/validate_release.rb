# frozen_string_literal: true

require "English"
require "json"
require "optparse"
require "pathname"

options = {
  root: File.expand_path("..", __dir__),
  changelog: "CHANGELOG.md"
}
OptionParser.new do |parser|
  parser.on("--root PATH") { |value| options[:root] = value }
  parser.on("--tag TAG") { |value| options[:tag] = value }
  parser.on("--notes PATH") { |value| options[:notes] = value }
  parser.on("--changelog PATH") { |value| options[:changelog] = value }
end.parse!

root = Pathname.new(options.fetch(:root)).realpath
tag = options.fetch(:tag)
notes_path = Pathname.new(options.fetch(:notes)).expand_path
changelog_path = Pathname.new(options.fetch(:changelog))
changelog_path = root.join(changelog_path) unless changelog_path.absolute?

abort("tag must use the vX.Y.Z form") unless tag.match?(/\Av(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)\z/)

version = tag.delete_prefix("v")
metadata_output = IO.popen(
  ["cargo", "metadata", "--format-version", "1", "--no-deps", "--locked"],
  chdir: root,
  &:read
)
abort("cargo metadata failed") unless $CHILD_STATUS.success?
metadata = JSON.parse(metadata_output)
workspace_members = metadata.fetch("workspace_members")
workspace_packages = metadata.fetch("packages").select do |package|
  workspace_members.include?(package.fetch("id"))
end
abort("workspace has no packages") if workspace_packages.empty?
unless workspace_packages.all? { |package| package.fetch("version") == version }
  abort("workspace package versions do not match #{version}")
end

changelog = changelog_path.read(encoding: "UTF-8")
header = /^## \[#{Regexp.escape(version)}\] - \d{4}-\d{2}-\d{2}$/
match = changelog.match(header)
abort("CHANGELOG.md has no #{version} release section") unless match
next_header = changelog.match(/^## /, match.end(0))
next_link = changelog.match(/^\[[^\]]+\]:\s+\S+$/, match.end(0))
boundary = [next_header, next_link].compact.map { |item| item.begin(0) }.min || changelog.length
notes = changelog[match.end(0)...boundary].strip
abort("CHANGELOG.md #{version} section is empty") if notes.empty?

notes_path.dirname.mkpath
notes_path.write("#{notes}\n", encoding: "UTF-8")
puts "Validated release #{tag}."
