# formatter

純 Rust 的多格式文件 formatter CLI。

## 安裝

```bash
cargo install --path .
```

## 使用方式

```bash
# 單一檔案就地格式化
formatter path/to/file.json

# 目錄遞迴格式化，並行使用預設 CPU 核心數
formatter path/to/project

# 僅檢查是否需要格式化（不寫檔），有差異會以非 0 code 結束
formatter --check path/to/project

# 乾跑顯示會改哪些檔案
formatter --dry-run path/to/project

# 輸出到指定目錄並鏡像目錄結構
formatter path/to/project --output /tmp/formatted

# 指定工作執行緒
formatter path/to/project --jobs 8

# 只處理特定語言，或跳過特定語言
formatter path/to/project --only json,ts,go
formatter path/to/project --skip proto,r

# 額外忽略 glob
formatter path/to/project --ignore \"**/generated/**\"
```

## 支援格式

JSON, YAML, TOML, XML, Markdown, Bash、Dockerfile、Makefile、INI、Nginx conf、HTML、CSS、TypeScript、JavaScript、Golang、Rust、Python、Protobuf、GraphQL、HCL、Lua、R、SQL。
若偵測到不支援的格式會提示並跳過。

## 行為與規則

- 遵循 `.gitignore` 與 `.dockerignore`，並內建忽略：`.git`, `node_modules`, `vendor`, `target`, `dist`, `.cache`, `.idea`, `.vscode`, `.DS_Store`。
- 預設覆寫原檔；指定 `--output` 時鏡像輸出。
- 預設並行度為 CPU 核心數，可用 `--jobs` 調整。
- 失敗或無法解析的檔案會報錯但不中斷其他檔案。

## 測試

```bash
make test
```
