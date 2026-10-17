# CI e release Tekt

O toolchain do workspace está em `rust-toolchain.toml`. Os auxiliares usam apenas
a biblioteca padrão do Python 3.11 ou posterior.

## Linhagem

O CI instala `crystalline-lint` do commit publicado
`12797073972e375d9ab078cf6750c8b30b71e2b9` de
<https://github.com/Dikluwe/tekt-linter>. A instalação local pode ter outra versão;
use esse commit para reproduzir o gate do CI.

```sh
cargo install --locked --git https://github.com/Dikluwe/tekt-linter --rev 12797073972e375d9ab078cf6750c8b30b71e2b9 crystalline-lint
python3 -m unittest discover -s tools/ci
crystalline-lint . --checks v1,v5,v15,v26 --fail-on warning
python3 tools/ci/check_reverse_lineage.py
```

O linter verifica headers, selos diretos, ownership e núcleos. O auxiliar verifica
o selo reverso de cada consumer nas quatro camadas, inclusive quando o selo direto
ainda coincide. Essa checagem adicional é necessária para a versão publicada do
linter. Ela não reescreve arquivos nem certifica conformidade semântica com o L0.

O relatório completo do linter é armazenado como `lineage.sarif`. Warnings das
demais regras continuam visíveis; os quatro códigos de linhagem acima bloqueiam
inclusive quando o linter os classifica como warning. Diagnósticos históricos são
excluídos da descoberta de consumers; código e prompts produtivos permanecem no
gate.

## Validação

Em Linux, instale `poppler-utils`, `qpdf`, `librsvg2-bin` e `imagemagick`.
Os testes de paridade exigem `TYPST_VANILLA` apontando para um oráculo cuja
proveniência tenha sido verificada. Não use a instalação antiga de
`/usr/local/bin/typst`. Para o binário local ratificado em P1489:

```sh
echo '7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea  lab/typst-original/target/release/typst' | sha256sum --check
export TYPST_VANILLA="$PWD/lab/typst-original/target/release/typst"
```

O CI faz checkout independente de `typst/typst` no pin
`9f2b6e8715237cb086899a42873660fe744622e8`; `build_oracle.py` verifica commit,
árvore upstream e ausência de modificações antes/depois do build. Compila com
lockfile e Rust do workspace, e guarda `oracle-build.json` como artefato com a
receita e SHA-256 do executável. O SHA de um rebuild pode diferir do binário
local P1489; ele é registrado, nunca substituído por uma string de versão.

```sh
cargo build --workspace --locked
TMPDIR="$(mktemp -d /dev/shm/tekt-tests.XXXXXX)" cargo testit --no-fail-fast -- --test-threads=1
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo fmt --check --all
RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --no-deps
```

`cargo testit` é um alias para `cargo test --workspace --locked`. O CI executa a
suíte completa, serialmente, em Linux. Os L0 dos testes recentes de exportação
exigem fixtures em `/dev/shm`, sem fallback ao disco. Em Windows, compila todos
os alvos, inclusive testes, e executa a CLI com `--version`; a suíte completa
nesse sistema depende de contratos de fixtures portáveis. Também há uma
checagem de compilação Linux i686. O check agregado
`Tests` exige sucesso de todos os jobs; falha, cancelamento ou job pulado bloqueiam
a aprovação. Os workflows de release e Docker reutilizam esse mesmo CI antes de
publicar. Falhas funcionais e de qualidade existentes precisam ser corrigidas
para liberar a publicação.

## Empacotamento

O workflow compila `typst-wiring`, binário `typst`, com `--locked`, na matriz de
targets declarada em `.github/workflows/release.yml`. Ele define
`TYPST_COMMIT_SHA` com o commit do checkout; `Cross.toml` repassa essa variável
para o container de compilação, conforme a
[configuração de ambiente do cross](https://github.com/cross-rs/cross/wiki/Configuration#buildenv).

Após uma compilação com `--target`, em um checkout limpo:

```sh
python3 tools/ci/package_release.py --target x86_64-unknown-linux-musl --builder cross
```

O auxiliar produz `dist/typst-<target>.tar.xz` (ou `.zip` para Windows) e seu
arquivo `.sha256`. O pacote contém o binário, README, licenças e `BUILD.json` com
commit, árvore Git, hash do binário, hashes do lockfile e toolchain, versão do
compilador e comando de build. Recusa arquivos modificados, arquivos não
rastreados que não sejam ignorados pelo Git e identidade de commit divergente.
Ele registra a receita declarada; não executa o build nem
prova sozinho que um binário arbitrário foi produzido por ela. O workflow faz o
build e o empacotamento em sequência no mesmo checkout.

O Dockerfile usa imagens-base fixadas por digest, Rust alinhado ao workspace e o
mesmo pacote/binário. O workflow fornece `REVISION` e `CREATED`. Um build manual
deve fornecer esses argumentos para preservar a proveniência da imagem.

As verificações locais de 2026-10-13, suas limitações e os bloqueios restantes estão
em `00_nucleo/diagnosticos/linhagem-ci-release-2026-10-13.md`, com a rodada
posterior em `00_nucleo/diagnosticos/correcoes-producao-2026-10-13.md`.
