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
      sha256 "86f63ace90462a2e490c48cabd9b4fadeaae49fdb18eb751d30680cd3a918ebb"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.2/sshtui-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "c12998f6b518f6e4c53d75e9251c837a3606434cafdae0facbbd3450e8db306a"
    end
    on_arm do
      url "https://github.com/stoneburner/sshtui/releases/download/v1.0.2/sshtui-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "eb5965a05cf57c003a608a034115b8e7f1155a4a2781eebb290d6f0c9f6ecb78"
    end
  end

  def install
    bin.install "sshtui"
  end

  test do
    assert_match "sshtui", shell_output("#{bin}/sshtui --help")
  end
end
