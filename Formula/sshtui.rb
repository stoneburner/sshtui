# typed: false
# frozen_string_literal: true

class Sshtui < Formula
  desc "TUI to pick an SSH host from config and connect"
  homepage "https://github.com/stoneburner/sshtui"
  version "1.0.4"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.4/sshtui-aarch64-apple-darwin.zip"
      sha256 "f57a8096a9d4884172738ba0a4ba187b69db95c546e81ea8275d90788738a3f2"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.4/sshtui-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "40e7163a92a737740a3d514a3f5c1fc9ffdbd2c3eb509819498bcdf8807e2648"
    end
    on_arm do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.4/sshtui-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "45050204eabbe078df5b93b40b025cffa5fd42b7e63a27fed4cd0d38ee939d5d"
    end
  end

  def install
    bin.install "sshtui"
  end

  test do
    assert_match "sshtui", shell_output("#{bin}/sshtui --help")
  end
end
