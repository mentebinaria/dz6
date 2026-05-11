class Dz6 < Formula
  desc "Fast Vim-inspired TUI hex editor"
  homepage "https://menteb.in/dz6"
  version "0.7.0"
  license "GPL-3.0-or-later"

  on_macos do
    on_intel do
      url "https://github.com/mentebinaria/dz6/releases/download/v#{version}/dz6-x86_64-apple-darwin.tar.gz"
      sha256 "__SHA256_INTEL_MACOS__"
    end
    on_arm do
      url "https://github.com/mentebinaria/dz6/releases/download/v#{version}/dz6-aarch64-apple-darwin.tar.gz"
      sha256 "__SHA256_ARM_MACOS__"
    end
  end

  on_linux do
    on_intel do
      if Hardware::CPU.is_64_bit?
        url "https://github.com/mentebinaria/dz6/releases/download/v#{version}/dz6-x86_64-unknown-linux-musl.tar.gz"
        sha256 "__SHA256_LINUX_X64__"
      end
    end
  end

  def install
    bin.install "dz6"
  end

  test do
    assert_match "dz6 #{version}", shell_output("#{bin}/dz6 --version")
  end
end
