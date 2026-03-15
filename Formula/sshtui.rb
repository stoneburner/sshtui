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
      sha256 "4aee01e3d248af310d771840112a69a25a5b47d54fcef01517552672cfe3694b"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.2/sshtui-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "64361e213819ac76495e14cb91425ce49cbb5a1e80fd9b620a90ffb62dcd9ed9"
    end
    on_arm do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.2/sshtui-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "3b6de27119269ad09e4fe558c44a131c3bfdb59ffa416ddc53fec1aac61556ba"
    end
  end

  def install
    bin.install "sshtui"
  end

  test do
    assert_match "sshtui", shell_output("#{bin}/sshtui --help")
  end
end
