default:
  just --list

# Generate licenses/licenses.html
generate-license-html:
  cargo about generate licenses/about.hbs --config licenses/about.toml --output-file licenses/licenses.html

# Download Japanese dictionary
download-japanese-dictionary:
  #!/usr/bin/env bash
  set -euxo pipefail
  workdir=$(mktemp -d)
  cd ${workdir}
  curl -f -L -O https://github.com/daac-tools/vibrato/releases/download/v0.3.1/ipadic-mecab-2_7_0.tar.gz
  tar xf ipadic-mecab-2_7_0.tar.gz
  ls -R
  mv ipadic-mecab-2_7_0/system.dic {{justfile_directory()}}/core/assets/
  rm -rf ${workdir}
