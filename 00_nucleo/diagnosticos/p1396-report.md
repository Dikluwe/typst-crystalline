# P1396 — relatório final de verificação R2

## Veredito

**PASS_LIMITED_FRAGMENT** — a candidata preserva o contrato observável
congelado para `math.liminf`, `math.limsup`, `math.lim` e `math.max`.
`liminf` e `limsup` carregam exatamente um U+2009 THIN SPACE, continuam com
`limits: true` e os controles adjacentes permanecem inalterados.

O protocolo completo Tekt foi **executado sem atestação de isolamento**. O
workspace e o contexto herdado eram compartilhados; a independência foi
tratada por papéis, ordem causal, allowlists de escrita e artefatos selados.

Este veredito é restrito à morfologia textual desses dois operadores, aos dois
controles e à coerência dos comentários/expectativas abrangidos. Não prova
paridade matemática geral nem paridade geral com Typst.

## Entradas R2 confirmadas

- passo: `3a46521a603db79d6c3f1ee7724892fdc6ac7086f95f0f0bd1be2c7f0f96ff95`;
- contrato: `867d199b21265418221df49990d12b0b1cead85376ba5ebd0946f05330c043b0`;
- oráculo: `19f8df92ccb866893f876d40fe1463773a086411fae123c4ee60c83976ee00ae`;
- recibo dos oráculos: `38fe80374cf9803c79f22af47f43b8fe79dde2debc9f77fc9b12678612338b40`;
- catálogo adversarial: `c91cf1548d69a19dd22d2ee55892bffe5e446c06e921c74c0cc8c524c8bf4acd`;
- pré-selo R2: `06d1d6d93b273cbc83566d5530b61a8466e42a1be3b9bddee40d2447a5b93ef3`.

R0 e R1 estão superseded e inválidos; nenhum hash dessas revisões foi usado
como selo final.

## Candidata e delta

Hashes finais confirmados:

- L0 da fachada `_comum.md`:
  `422c678d197ec48d0299059d90629bd3a9db852734394c32d0907b96bceb2dd6`;
- L0 estrutural `math.md`:
  `e58334e2be9db99a59ad35b251316e29d0064383ace335a1c9419eddd8b9f42e`;
- fachada `stdlib/mod.rs`:
  `7b25817f43926b9fdbba6ecadf6ae33a78e0ab166d2fef5c97249323d7bd6b36`;
- candidata `structural/math.rs`:
  `14ba94bf6de39bf9a8d905499181d5c9e7ac7d3063f994816391449ed2de6578`;
- binário cristalino observado:
  `f61dc66b40c76ec796b3ae569f4cdafdded5e38b20056054825aa3d1fbb88474`;
- GREEN `/tmp/p1396-green.json`:
  `4a959c8a10dc1351c864ce103114147bc6bd6d9572e079e18ec4c184a1af8c46`.

O diff contra `3f081c97f7dba59549a70e89d6bfb93afcc82b1a` ficou dentro do escopo R2:
somente os dois literais produtivos, o comentário local da tabela, as duas
expectativas P299 e seus comentários, os dois L0s e a metadata de linhagem.
Nenhuma outra entrada da tabela, API ou owner produtivo mudou. A inspeção dos
recortes não encontrou alegação residual de U+0020 como separador correto.

## Evidência funcional e adversarial

O oráculo CLI exato terminou com `4 Preserved / 0 Difference / 0 Unknown`.
As ordens direta, repetida e inversa concordaram; a identidade do binário foi
aceita. O recibo externo registra exit 0, 1,20 s, RSS máximo de 32.948 KiB e
zero swap sob `RLIMIT_AS` de 512 MiB.

O gate adversarial R2 rejeitou 19 de 19 mutantes aplicáveis, score `1.0`, em
regime composto estático/contrafactual com testemunhas executadas. Os três
casos opacos ficaram fora do denominador e `Unknown` continuou fail-closed.
Esse score não significa que dezenove mutantes tenham sido compilados.

## Memória e gates arquiteturais

Nenhum `cargo test`, lib-test monolítico ou teste amplo de workspace foi
executado. Os gates pesados foram seriais, uma job, sob `RLIMIT_DATA` de 2
GiB. O maior RSS observado foi 1.134.628 KiB no build release, abaixo do teto
de interrupção de 1.782.579 KiB; todos os recibos registram zero swap e nenhum
OOM.

- build release: exit 0, 2m28,49s, RSS 1.134.628 KiB;
- `cargo check -p typst-core` após R1: exit 0, RSS 526.208 KiB;
- resselo após R1: exit 0, RSS 115.356 KiB;
- lint após R1: exit 0, zero violações, RSS 70.840 KiB;
- resselo após o comentário R2: exit 0, `Nothing to fix`, RSS 112.992 KiB;
- lint final R2: exit 0, zero violações, RSS 70.292 KiB;
- `cargo fmt --check` e `git diff --check`: exit 0.

O comentário R2 não altera o binário; por isso os recibos funcionais e de
compilação anteriores continuam aplicáveis à candidata comportamental. O
verificador final executou apenas hashes, diff e inspeções textuais leves.
