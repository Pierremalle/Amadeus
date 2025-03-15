DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$(dirname "$DIR")"

export ESP_IDF_SYS_ROOT_CRATE="$PROJECT_DIR/amadeus_embedded"
echo $ESP_IDF_SYS_ROOT_CRATE