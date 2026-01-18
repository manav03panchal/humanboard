cask "humanboard" do
  version "0.1.0"
  sha256 "REPLACE_WITH_SHA256_FROM_RELEASE"

  url "https://github.com/manav03panchal/humanboard/releases/download/v#{version}/Humanboard.dmg"
  name "Humanboard"
  desc "Desktop productivity app"
  homepage "https://github.com/manav03panchal/humanboard"

  depends_on macos: ">= :big_sur"

  app "Humanboard.app"

  zap trash: [
    "~/Library/Application Support/Humanboard",
    "~/Library/Caches/Humanboard",
    "~/Library/Preferences/com.manavpanchal.humanboard.plist",
  ]
end
