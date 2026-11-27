#!/usr/bin/env bash
# Publica o UTSim Ensino pelo launcher, como produto próprio (`ensino`): monta a pasta do
# programa, cifra, assina e grava o pacote e o manifesto na pasta de publicação (local).
# Não envia nada à internet: depois, 'utsim-publicar github' sobe o pacote e o
# manifesto para o GitHub.
# O launcher já instalado passa a mostrar o produto; ele não precisa mudar.
#
# Uso: bash scripts/publicar-ensino.sh <pasta_de_publicação> [canal] [versão]
#   canal   padrão: estavel (o launcher 0.4.0 mostra todos; o canal só conta para launchers antigos)
#   versão  padrão: a de crates/utsim-ensino-app/Cargo.toml (versão própria, tag ensino-v<versão>)
#
# Antes: cargo build --release -p utsim-ensino-app -p utsim-publicar
# Notas da versão: a seção "## [ensino-<versão>]" do CHANGELOG.md, se existir; senão, os itens
# da seção "## [Não lançado]" que citam o UTSim Ensino.

set -eu

cd "$(dirname "$0")/.." || exit 1

[ $# -ge 1 ] || { echo "uso: $0 <pasta_de_publicação> [canal] [versão]" >&2; exit 2; }
SAIDA=$1
CANAL=${2:-estavel}
VERSAO=${3:-$(awk -F'"' '/^version = /{print $2; exit}' crates/utsim-ensino-app/Cargo.toml)}
VERSAO=${VERSAO#ensino-}
VERSAO=${VERSAO#v}

PUBLICAR=target/release/utsim-publicar
[ -x "$PUBLICAR" ] || PUBLICAR=target/release/utsim-publicar.exe
[ -x "$PUBLICAR" ] || { echo "erro: rode antes cargo build --release -p utsim-publicar" >&2; exit 1; }

case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*) PLATAFORMA=windows-x64; NOME=utsim-ensino.exe ;;
    Linux) PLATAFORMA=linux-x64; NOME=utsim-ensino ;;
    *) echo "erro: plataforma não suportada" >&2; exit 1 ;;
esac
BIN=target/release/$NOME
[ -f "$BIN" ] || { echo "erro: $BIN não encontrado; rode antes cargo build --release -p utsim-ensino-app" >&2; exit 1; }

# Os pacotes de missões e os elementos 3D moram na família (utsim-familia), na tag do
# Cargo.toml: vêm do checkout que o cargo baixou (ou de UTSIM_FAMILIA, uma pasta local).
FAMILIA=${UTSIM_FAMILIA:-$(cargo metadata --format-version 1 --offline 2>/dev/null \
    | grep -o '"manifest_path":"[^"]*utsim-ensino-core[^"]*"' | head -1 \
    | sed -e 's/^"manifest_path":"//' -e 's/"$//' -e 's#\\\\*#/#g' -e 's#/crates/utsim-ensino-core/Cargo.toml$##')}
[ -d "$FAMILIA/ensino/pacotes" ] || { echo "erro: pacotes da família não encontrados (rode antes o build ou defina UTSIM_FAMILIA)" >&2; exit 1; }

PASTA=$(mktemp -d)
trap 'rm -rf "$PASTA"' EXIT
mkdir -p "$PASTA/ensino/bin" "$PASTA/ensino/manual"
cp "$BIN" "$PASTA/ensino/bin/"
cp docs/manual/src/ensino.md "$PASTA/ensino/manual/"
# Os pacotes de missões fora da campanha: o app os lista sozinho (bin/../pacotes).
cp -r "$FAMILIA/ensino/pacotes" "$PASTA/ensino/pacotes"
# Os elementos 3D, para o editor de missões gerar a imagem do enunciado com o Blender do
# professor (o app os procura em bin/../assets/modelos3d): os modelos e os clipes das pessoas,
# com a licença deles.
mkdir -p "$PASTA/ensino/assets/terceiros"
cp -r "$FAMILIA/assets/modelos3d" "$PASTA/ensino/assets/modelos3d"
cp -r "$FAMILIA/assets/terceiros/quaternius_ual" "$PASTA/ensino/assets/terceiros/quaternius_ual"

if grep -q "^## \[ensino-$VERSAO\]" CHANGELOG.md; then
    bash scripts/notas-versao.sh "ensino-$VERSAO" > "$PASTA/notas.md"
else
    awk '/^## \[/ { n++ } n == 1 && /^- / && /UTSim Ensino/ { print }' CHANGELOG.md > "$PASTA/notas.md"
fi
[ -s "$PASTA/notas.md" ] || echo "(sem notas)" > "$PASTA/notas.md"

"$PUBLICAR" publicar \
    --saida "$SAIDA" \
    --produto ensino \
    --nome "UTSim Ensino" \
    --descricao "Ensino de engenharia de produção pela simulação: o jogo de problemas da indústria e a sala de aula" \
    --subtitulo "Ensino pela simulação" \
    --autor "UTSim" \
    --cor "#8A5CE6" \
    --icone assets/icone/familia/ensino_256.png \
    --versao "$VERSAO" \
    --canal "$CANAL" \
    --plataforma "$PLATAFORMA" \
    --pasta "$PASTA/ensino" \
    --executavel "bin/$NOME" \
    --notas "$PASTA/notas.md"
