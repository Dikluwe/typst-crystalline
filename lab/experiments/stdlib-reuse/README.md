# Reúso de stdlib por avaliação

Este código é experimental. `apply.py` altera somente uma cópia separada em
`/dev/shm/tekt-stdlib-reuse-20261105`; o baseline e o produto ativo são recusados.
Use `--mutation share-identities`, `ignore-inputs` ou `ignore-features` apenas
nas cópias adversárias. O façade `__lab_resolve_stdlib` existe só no overlay de
laboratório e segue o mesmo resolver dos imports.

O snapshot guarda o template construído no root. Imports consultam inputs na
mesma posição e usam o template somente com pares na mesma ordem e features
iguais. Cada arquivo recebe um Scope e identidades de módulos novos, preservando
aliases internos. O helper é limitado à stdlib pinada; não é clonagem geral de
funções ou módulos produzidos por usuários.

O namespace `raw` contém o elemento `raw.line`, construído por
`registry/structural.rs` com o construtor estático `RawLineElem::construct`.
Sua função recebe identidade nova por arquivo, com memo local para aliases.
Bases/argumentos de `With` são copiados recursivamente; closures e plugins
avaliados não são aceitos como template.

Após compilar baseline/candidato em RAM, a bancada pareada é:

```sh
python3 measure.py --baseline /dev/shm/tekt-stdlib-reuse-20261105/base-target/release/typst \
  --candidate /dev/shm/tekt-stdlib-reuse-20261105/candidate-target/release/typst \
  --fixtures /CAMINHO/ABSOLUTO/fixtures --fixture plain.typ --fixture imports.typ \
  --ram-dir /dev/shm/tekt-stdlib-reuse-20261105/measurements --rounds 10 --repeats 2
```

Cada processo recebe uma cópia inteira da árvore de fixtures e um marcador de
epoch no main. O script aquece antes das amostras e alterna os pares. CLI usa
wait4; watch mede startup, reescrita com os mesmos bytes e edição do main, com
CPU por clock do processo e wall até nova publicação estável com EOF. Wall do
watch inclui eventos/debounce. Não há profilers ou extratores dentro do relógio.

`measurements.csv` é atualizado por processo concluído. Os totais de CPU/RSS de
wait4 se repetem nas três linhas de um watch: não os some como custos por ciclo.
Os PDFs de cada ciclo e logs ficam só na RAM. Valide texto, epochs, fontes,
geometria e pixels separadamente, depois calcule pares/intervalos; o script não
conclui ganho automaticamente. A árvore de fixtures deve ser autocontida e sem
symlinks; a área de saída deve ser nova, para recusar colisões e recursão de cópia.


Contagens fora dos relógios: `TEKT_STDLIB_RAM=/dev/shm/tekt-stdlib-reuse-20261105 python3 count.py`.
O script exige `base-target/release/typst` e `candidate-target/release/typst`,
cria projetos privados e recusa colisões com saídas anteriores. GDB mede entradas
do builder, Source::new_with_parser e coordenadores de layout; não mede duração
de fase. Funções inline sem símbolo não entram na contagem.
