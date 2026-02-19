# Enable sccache for all cargo builds
export CARGO_BUILD_RUSTC_WRAPPER := "sccache"

# Default: Show help menu
default:
    @just help

# ============================================================================
# Help Command
# ============================================================================

help:
    @echo ""
    @echo "\033[1;36m======================================\033[0m"
    @echo "\033[1;36m       Project Commands               \033[0m"
    @echo "\033[1;36m======================================\033[0m"
    @echo ""
    @echo "\033[1;35m  Most Common Commands:\033[0m"
    @echo "  just \033[0;33mdev\033[0m                      \033[0;32mStart showcase demo\033[0m"
    @echo "  just \033[0;33mbuild\033[0m                     \033[0;32mBuild the project\033[0m"
    @echo "  just \033[0;33mtest\033[0m                      \033[0;32mRun all tests\033[0m"
    @echo "  just \033[0;33mcheck\033[0m                     \033[0;32mRun all checks\033[0m"
    @echo ""
    @echo "\033[1;35m  Development:\033[0m"
    @echo "  just \033[0;33mdev\033[0m                       \033[0;32mStart showcase demo\033[0m"
    @echo "  just \033[0;33mexample NAME\033[0m              \033[0;32mRun a specific example\033[0m"
    @echo ""
    @echo "\033[1;35m  Building:\033[0m"
    @echo "  just \033[0;33mbuild\033[0m                     \033[0;32mBuild the project\033[0m"
    @echo ""
    @echo "\033[1;35m  Verification:\033[0m"
    @echo "  just \033[0;33mlint\033[0m                      \033[0;32mRun linter\033[0m"
    @echo "  just \033[0;33mfmt\033[0m                       \033[0;32mFormat code\033[0m"
    @echo "  just \033[0;33mfmt-check\033[0m                 \033[0;32mCheck formatting\033[0m"
    @echo "  just \033[0;33mcheck\033[0m                     \033[0;32mRun all verification\033[0m"
    @echo ""
    @echo "\033[1;35m  Testing:\033[0m"
    @echo "  just \033[0;33mtest\033[0m                      \033[0;32mRun all tests\033[0m"
    @echo ""
    @echo "\033[1;35m  Utilities:\033[0m"
    @echo "  just \033[0;33mclean\033[0m                     \033[0;32mClean build artifacts\033[0m"
    @echo "  just \033[0;33mdocs\033[0m                      \033[0;32mBuild & serve documentation\033[0m"
    @echo "  just \033[0;33mdocs-gh-pub\033[0m               \033[0;32mBuild & publish docs to GitHub Pages\033[0m"
    @echo ""
    @echo "\033[1;35m  Demos:\033[0m"
    @echo "  just \033[0;33mdemo\033[0m                      \033[0;32mPick and run an example\033[0m"
    @echo "  just \033[0;33mdemo-md\033[0m                   \033[0;32mMarkdown viewer demo\033[0m"
    @echo "  just \033[0;33mdemo-term\033[0m                 \033[0;32mTerminal pane demo\033[0m"
    @echo "  just \033[0;33mdemo-split\033[0m                \033[0;32mSplit layout demo\033[0m"
    @echo "  just \033[0;33mdemo-codediff\033[0m             \033[0;32mCode diff demo\033[0m"
    @echo "  just \033[0;33mdemo-aichat\033[0m               \033[0;32mAI chat demo\033[0m"
    @echo ""

# ============================================================================
# Development Commands
# ============================================================================
import 'justfiles/development/dev.just'
import 'justfiles/development/example.just'
import 'justfiles/development/demo.just'

# ============================================================================
# Building Commands
# ============================================================================
import 'justfiles/building/build.just'

# ============================================================================
# Verification Commands
# ============================================================================
import 'justfiles/verification/lint.just'
import 'justfiles/verification/fmt-check.just'
import 'justfiles/verification/check.just'

# ============================================================================
# Testing Commands
# ============================================================================
import 'justfiles/testing/test.just'

# ============================================================================
# Utilities Commands
# ============================================================================
import 'justfiles/utilities/fmt.just'
import 'justfiles/utilities/doc.just'
import 'justfiles/utilities/docs-gh-pub.just'
import 'justfiles/utilities/clean.just'
import 'justfiles/utilities/pub.just'
import 'justfiles/utilities/cast-record.just'
import 'justfiles/utilities/cast-interactive.just'
import 'justfiles/utilities/cast-replay.just'
import 'justfiles/utilities/cast-upload.just'
import 'justfiles/utilities/cast-gif.just'
import 'justfiles/utilities/demo-md.just'
import 'justfiles/utilities/demo-term.just'
import 'justfiles/utilities/demo-codediff.just'
import 'justfiles/utilities/demo-aichat.just'
import 'justfiles/utilities/demo-split.just'
import 'justfiles/utilities/cast-clean.just'
