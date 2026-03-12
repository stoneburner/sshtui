# typed: false
# frozen_string_literal: true

class Sshtui < Formula
  desc "TUI to pick an SSH host from config and connect"
  homepage "https://github.com/yourusername/sshtui"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_intel do
      url "https://github.com/yourusername/sshtui/releases/download/v0.1.0/sshtui-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_X86_64_MACOS_SHA"
    end
    on_arm do
      url "https://github.com/yourusername/sshtui/releases/download/v0.1.0/sshtui-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ARM64_MACOS_SHA"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/yourusername/sshtui/releases/download/v0.1.0/sshtui-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_X86_64_LINUX_SHA"
    end
    on_arm do
      url "https://github.com/yourusername/sshtui/releases/download/v0.1.0/sshtui-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_ARM64_LINUX_SHA"
    end
  end

  def install
    bin.install "sshtui"
  end

  test do
    assert_match "sshtui", shell_output("#{bin}/sshtui --help")
  end
end
