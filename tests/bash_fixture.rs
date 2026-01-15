use std::path::Path;

use formatter::formats::bash;

fn fmt(input: &str) -> String {
    bash::format(Path::new("sample.sh"), input)
        .unwrap()
        .unwrap_or_else(|| input.to_string())
}

#[test]
fn formats_realistic_protoc_script() {
    let input = r#"#!/bin/bash

NEED_PROTOC_VERSION="libprotoc 32.1"
NEED_PROTOC_GEN_GO_VERSION="protoc-gen-go v1.36.6"
NEED_PROTOC_GEN_GO_GRPC_VERSION="protoc-gen-go-grpc 1.5.1"

PROTOC_VERSION=$(protoc --version)
PROTOC_GEN_GO_VERSION=$(protoc-gen-go --version)
PROTOC_GEN_GO_GRPC_VERSION=$(protoc-gen-go-grpc --version 2 >& 1)

CUR_VER="${PROTOC_VERSION##* }"  # "33.0.0" 或 "32.1"
NEED_VER="${NEED_PROTOC_VERSION##* }"  # "32.1"

version_ge() {
  local cur="$1" need="$2"

  IFS='.' read -r c1 c2 c3 <<< "$cur"
  IFS='.' read -r n1 n2 n3 <<< "$need"
 c2=${c2:-0}; c3=${c3:-0}
 n2=${n2:-0}; n3=${n3:-0}

# 逐段比較
if (( c1 > n1 )); then return 0; fi
if (( c1 < n1 )); then return 1; fi
if (( c2 > n2 )); then return 0; fi
if (( c2 < n2 )); then return 1; fi
if (( c3 > = n3 )); then return 0; fi
  return 1
}

if ! version_ge "$CUR_VER" "$NEED_VER"; then
echo -e "${RED}protoc 版本太低 (目前: ${CUR_VER}，至少需要: ${NEED_VER})${RESET}"
exit 1
fi

if [[ "${PROTOC_GEN_GO_VERSION##* }" != "${NEED_PROTOC_GEN_GO_VERSION##* }" ]]; then
echo -e "${RED}protoc-gen-go 版本不一致 (目前: ${PROTOC_GEN_GO_VERSION##* }，需要: ${NEED_PROTOC_GEN_GO_VERSION##* })${RESET}"
exit 1
fi

if [[ "${PROTOC_GEN_GO_GRPC_VERSION##* }" != "${NEED_PROTOC_GEN_GO_GRPC_VERSION##* }" ]]; then
echo -e "${RED}protoc-gen-go-grpc 版本不一致 (目前: ${PROTOC_GEN_GO_GRPC_VERSION##* }，需要: ${NEED_PROTOC_GEN_GO_GRPC_VERSION##* })${RESET}"
exit 1
fi

set -o errexit

readonly RESET='\033[0m'
readonly RED='\033[1;31m'
readonly GREEN='\033[1;32m'
readonly YELLOW='\033[1;33m'
readonly protoDir="proto"

# 擷取輸入參數
TARGET_SERVICES="$1"

if [[ -z "$TARGET_SERVICES" ]]; then
echo -e "${RED}未指定 proto 資料夾${RESET}"
ls -1 ${protoDir}
exit 1
fi

# 執行編譯指令
protoc --proto_path=. \
  --go_out=./ \
  --go_opt=paths=source_relative \
  --go-grpc_out=require_unimplemented_servers=false:./ \
  --go-grpc_opt=paths=source_relative \
  ./${protoDir}/${TARGET_SERVICES}/*.proto

# 刪除版本號
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS 的 sed
sed -i '' '2,4d' ./proto/${TARGET_SERVICES}/*.pb.go
else
  # Linux 的 sed（例如 WSL、Git Bash）
sed -i '2,4d' ./proto/${TARGET_SERVICES}/*.pb.go
fi

echo -e "${GREEN}Done!${RESET}"
"#;

    let expected = r#"#!/bin/bash

NEED_PROTOC_VERSION="libprotoc 32.1"
NEED_PROTOC_GEN_GO_VERSION="protoc-gen-go v1.36.6"
NEED_PROTOC_GEN_GO_GRPC_VERSION="protoc-gen-go-grpc 1.5.1"

PROTOC_VERSION=$(protoc --version)
PROTOC_GEN_GO_VERSION=$(protoc-gen-go --version)
PROTOC_GEN_GO_GRPC_VERSION=$(protoc-gen-go-grpc --version 2 >& 1)

CUR_VER="${PROTOC_VERSION##* }"  # "33.0.0" 或 "32.1"
NEED_VER="${NEED_PROTOC_VERSION##* }"  # "32.1"

version_ge() {
  local cur="$1" need="$2"

  IFS='.' read -r c1 c2 c3 <<< "$cur"
  IFS='.' read -r n1 n2 n3 <<< "$need"
  c2=${c2:-0}; c3=${c3:-0}
  n2=${n2:-0}; n3=${n3:-0}

  # 逐段比較
  if (( c1 > n1 )); then return 0; fi
  if (( c1 < n1 )); then return 1; fi
  if (( c2 > n2 )); then return 0; fi
  if (( c2 < n2 )); then return 1; fi
  if (( c3 >= n3 )); then return 0; fi
  return 1
}

if ! version_ge "$CUR_VER" "$NEED_VER"; then
  echo -e "${RED}protoc 版本太低 (目前: ${CUR_VER}，至少需要: ${NEED_VER})${RESET}"
  exit 1
fi

if [[ "${PROTOC_GEN_GO_VERSION##* }" != "${NEED_PROTOC_GEN_GO_VERSION##* }" ]]; then
  echo -e "${RED}protoc-gen-go 版本不一致 (目前: ${PROTOC_GEN_GO_VERSION##* }，需要: ${NEED_PROTOC_GEN_GO_VERSION##* })${RESET}"
  exit 1
fi

if [[ "${PROTOC_GEN_GO_GRPC_VERSION##* }" != "${NEED_PROTOC_GEN_GO_GRPC_VERSION##* }" ]]; then
  echo -e "${RED}protoc-gen-go-grpc 版本不一致 (目前: ${PROTOC_GEN_GO_GRPC_VERSION##* }，需要: ${NEED_PROTOC_GEN_GO_GRPC_VERSION##* })${RESET}"
  exit 1
fi

set -o errexit

readonly RESET='\033[0m'
readonly RED='\033[1;31m'
readonly GREEN='\033[1;32m'
readonly YELLOW='\033[1;33m'
readonly protoDir="proto"

# 擷取輸入參數
TARGET_SERVICES="$1"

if [[ -z "$TARGET_SERVICES" ]]; then
  echo -e "${RED}未指定 proto 資料夾${RESET}"
  ls -1 ${protoDir}
  exit 1
fi

# 執行編譯指令
protoc --proto_path=. \
  --go_out=./ \
  --go_opt=paths=source_relative \
  --go-grpc_out=require_unimplemented_servers=false:./ \
  --go-grpc_opt=paths=source_relative \
  ./${protoDir}/${TARGET_SERVICES}/*.proto

# 刪除版本號
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS 的 sed
  sed -i '' '2,4d' ./proto/${TARGET_SERVICES}/*.pb.go
else
  # Linux 的 sed（例如 WSL、Git Bash）
  sed -i '2,4d' ./proto/${TARGET_SERVICES}/*.pb.go
fi

echo -e "${GREEN}Done!${RESET}"
"#;

    assert_eq!(fmt(input), expected);
}
