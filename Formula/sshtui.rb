# typed: false
# frozen_string_literal: true

class Sshtui < Formula
  desc "TUI to pick an SSH host from config and connect"
  homepage "https://github.com/stoneburner/sshtui"
  version "1.0.2"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.2/sshtui-aarch64-apple-darwin.zip"
      sha256 "6aebc8d122a11631170e9bde14f198be71f4f97316c1844ed7d4181dee89ff11"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.2/sshtui-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_X86_64_LINUX_SHA"
    end
    on_arm do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.2/sshtui-aarch64-unknown-linux-gnu.tar.gz"
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
