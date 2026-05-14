class AiPrReviewer < Formula
  desc "AI PR Reviewer (AI-Debt Governor) intercepts low-quality AI-generated code diffs"
  homepage "https://github.com/dhia-bechattaoui/ai-pr-reviewer"
  url "https://github.com/dhia-bechattaoui/ai-pr-reviewer/archive/refs/tags/v0.0.1.tar.gz"
  sha256 "0000000000000000000000000000000000000000000000000000000000000000" # Placeholder: update with release archive hash
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Verify binary parsing capability without active Git dependencies
    output = shell_output("#{bin}/ai-pr-reviewer --raw-diff \"diff --git a/test.rs b/test.rs\"")
    assert_match "old_path", output
  end
end
