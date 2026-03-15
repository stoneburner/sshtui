# typed: false
# frozen_string_literal: true

class Sshtui < Formula
  desc "TUI to pick an SSH host from config and connect"
  homepage "https://github.com/stoneburner/sshtui"
  version "1.0.3"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.3/sshtui-aarch64-apple-darwin.zip"
      sha256 "977c2e1d9095c9811e9bcc6a42ba558dff98eeea2ea3743bb81f61dbf859ade2"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.3/sshtui-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "254078526f73ee10ab7b0cd1238de1e64e2d3e9e572ab783f509c2297d0df253"
    end
    on_arm do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.3/sshtui-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "bd4a635b906ba0069b1947f7c8050821908dea4d680641bc88e9a77893b4490a"
    end
  end

  def install
    bin.install "sshtui"
  end

  test do
    assert_match "sshtui", shell_output("#{bin}/sshtui --help")
  end
end
