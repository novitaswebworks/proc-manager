#!/usr/bin/env bash
set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: ./update-brew.sh v1.0.2"
    exit 1
fi

echo "Fetching checksums for $VERSION..."
cd /tmp

MAC_ARM="proc-manager-macos-arm64.tar.gz"
MAC_X86="proc-manager-macos-x86_64.tar.gz"
LINUX_X86="proc-manager-linux-x86_64.tar.gz"

curl -sL -O "https://github.com/novitaswebworks/proc-manager/releases/download/$VERSION/$MAC_ARM"
curl -sL -O "https://github.com/novitaswebworks/proc-manager/releases/download/$VERSION/$MAC_X86"
curl -sL -O "https://github.com/novitaswebworks/proc-manager/releases/download/$VERSION/$LINUX_X86"

MAC_ARM_SHA=$(shasum -a 256 $MAC_ARM | cut -d ' ' -f 1)
MAC_X86_SHA=$(shasum -a 256 $MAC_X86 | cut -d ' ' -f 1)
LINUX_X86_SHA=$(shasum -a 256 $LINUX_X86 | cut -d ' ' -f 1)

echo "Mac ARM: $MAC_ARM_SHA"
echo "Mac x86: $MAC_X86_SHA"
echo "Linux x86: $LINUX_X86_SHA"

cd - > /dev/null

echo "Updating Formula..."
cat > ../homebrew-tap/Formula/proc-manager.rb <<EOF
class ProcManager < Formula
  desc "Modern TUI Process, Docker, and Service Manager"
  homepage "https://github.com/novitaswebworks/proc-manager"
  version "${VERSION#v}"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/novitaswebworks/proc-manager/releases/download/$VERSION/$MAC_ARM"
      sha256 "$MAC_ARM_SHA"
    else
      url "https://github.com/novitaswebworks/proc-manager/releases/download/$VERSION/$MAC_X86"
      sha256 "$MAC_X86_SHA"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/novitaswebworks/proc-manager/releases/download/$VERSION/$LINUX_X86"
      sha256 "$LINUX_X86_SHA"
    end
  end

  def install
    bin.install "proc-manager"
    bin.install_symlink "proc-manager" => "nova"
  end

  test do
    system "#{bin}/proc-manager", "--version"
  end
end
EOF

echo "Pushing to homebrew-tap..."
cd ../homebrew-tap
git add Formula/proc-manager.rb
git commit -m "Update proc-manager to $VERSION"
git push origin main
cd - > /dev/null

echo "✅ Homebrew tap updated successfully!"
