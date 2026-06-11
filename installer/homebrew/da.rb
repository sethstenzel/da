class Da < Formula
  desc "Directory alias manager — map short names to paths and open them with any tool"
  homepage "https://github.com/sethstenzel/da"  # TODO: update with real repo URL
  url "https://github.com/sethstenzel/da/archive/v0.4.5.tar.gz"
  sha256 "PLACEHOLDER"  # Run: curl -L <url> | shasum -a 256
  license "MIT"

  depends_on "rust" => :build

  def install
    cd "da" do
      system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    end
  end

  def caveats
    <<~EOS
      To enable 'dacd <alias>' for changing directories, run:
        da shell-init
      and follow the instructions to add the function to your shell profile.
    EOS
  end

  test do
    assert_match "Usage:", shell_output("#{bin}/da 2>&1", 0)
  end
end
