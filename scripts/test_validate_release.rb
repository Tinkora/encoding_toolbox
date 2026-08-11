# frozen_string_literal: true

require "open3"
require "tempfile"

root = File.expand_path("..", __dir__)
validator = File.join(__dir__, "validate_release.rb")

Tempfile.create(["encoding-toolbox-changelog", ".md"]) do |changelog|
  changelog.write(<<~MARKDOWN)
    # Changelog

    ## [0.1.0] - 2026-08-11

    ### Added

    - A tested release fixture.

    [0.1.0]: https://github.com/Tinkora/encoding_toolbox/releases/tag/v0.1.0
  MARKDOWN
  changelog.flush

  Tempfile.create(["encoding-toolbox-notes", ".md"]) do |notes|
    stdout, stderr, status = Open3.capture3(
      "ruby", validator,
      "--root", root,
      "--tag", "v0.1.0",
      "--changelog", changelog.path,
      "--notes", notes.path
    )
    abort("validator failed: #{stdout}#{stderr}") unless status.success?
    abort("release notes were not extracted") unless File.read(notes.path).include?("tested release fixture")
  end
end

_, _, invalid_status = Open3.capture3(
  "ruby", validator,
  "--root", root,
  "--tag", "release-1",
  "--notes", File::NULL
)
abort("validator accepted an invalid tag") if invalid_status.success?

puts "Release validator tests passed."
