RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

print_log() {
    message="$1"

    printf "${RED}[SCRIPT]${NC} ${GREEN}${message}${NC}\n"
}