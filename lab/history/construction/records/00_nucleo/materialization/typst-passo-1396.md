# Passo 1396 — espaço fino em `math.liminf` e `math.limsup`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo e regime

Corrigir a morfologia pública dos operadores compostos `math.liminf` e
`math.limsup`: entre `lim` e `inf`/`sup` deve existir U+2009 THIN SPACE, não
U+0020 SPACE.

Aplica-se o protocolo completo `tekt-materializacao-segregada`: intenção,
contrato, oráculos, ataques, implementação e veredito pertencem a autoridades
distintas. Como o workspace é compartilhado, a entrega final deverá dizer
**executado sem atestação de isolamento**. Este documento não emite selo nem
veredito.

## Medição anterior à decisão

Estado congelado antes de P1396, observado em `2026-09-14T15:54:52-03:00`:

- HEAD limpo `3f081c97f7dba59549a70e89d6bfb93afcc82b1a`;
- L0 anterior `compiler/stdlib/structural/math.md`, identificador administrativo
  [dado administrativo removido];
- owner produtivo `01_core/src/compiler/stdlib/structural/math.rs`, identificador administrativo
  [dado administrativo removido];
- fonte ratificada `lab/typst-original/crates/typst-library/src/math/op.rs`,
  identificador administrativo
  [dado administrativo removido];
- binário vanila ratificado, identificador administrativo
  [dado administrativo removido],
  52.149.176 bytes;
- binário cristalino baseline, identificador administrativo
  [dado administrativo removido],
  49.136.680 bytes.

No vanila ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`, `op.rs:88-89` contém
`liminf: "lim inf"` e `limsup: "lim sup"`; inspeção dos bytes confirma U+2009.
No cristalino, `math.rs:1163-1164` contém `"lim inf"` e `"lim sup"`, com
U+0020.

A sonda válida `eval repr(math.liminf)` produziu no vanila
`"op(text: [lim inf], limits: true)"`, exit 0, RSS máximo 18.260 KiB e zero
swap. O mesmo observável no cristalino produziu a variante com U+0020, exit 0,
RSS máximo 32.884 KiB e zero swap. Uma tentativa PDF com flag mal posicionada
terminou exit 2 e uma fixture que tratava incorretamente `math` como namespace
terminou exit 1; ambas são descartadas e não sustentam decisão.

## Classificação e decisão

O conteúdo textual do `MathOp` é morfologia da língua (ADR-0107), não detalhe
da estrutura Rust nem byte de PDF. A declaração explícita dos dois literais na
fonte ratificada é evidência de intenção; a observação isolada do render não é
usada para inferi-la. A hipótese seria refutada se a mesma revisão pinada
declarasse outro codepoint ou se uma sonda bilateral válida produzisse
sequências idênticas.

Trata-se de correção interna de tabela/paridade: não muda API pública,
comportamento de produto deliberadamente novo, fase do pipeline ou
compatibilidade. Portanto segue fluxo contínuo ADR-0127: L0 atualizado
primeiro, RED, implementação, GREEN, resselo e revalidação, sem paragem humana
intermediária. O L0 atualizado antes do código tem identificador administrativo
[dado administrativo removido].

## Delta autorizado

No owner `01_core/src/compiler/stdlib/structural/math.rs`, trocar somente:

- o texto de `liminf`, de `lim inf` para `lim\u{2009}inf`;
- o texto de `limsup`, de `lim sup` para `lim\u{2009}sup`.

É permitido atualizar apenas a metadata de linhagem da relação 1:1 após a
mudança. São proibidos: assinatura/API, layout, serializer, parser, eval,
diagnósticos, flags/defaults, outros operadores, refactor da tabela e qualquer
outro owner. `limits: true` permanece intacto.

## Contrato observável e ataques

O oráculo independente deve observar `repr(math.liminf)` e
`repr(math.limsup)`, confirmar U+2009 exatamente uma vez no separador e manter
controles adjacentes como `math.lim` e `math.max`. Deve rejeitar pelo menos:
U+0020, U+00A0, U+202F, ausência/espaço zero-width, correção de apenas um nome,
troca de `limits: true` e alteração de outro operador. Todas as mutações
aplicáveis precisam ser rejeitadas (`mutation_score = 1.0`) antes do selo.

Falha de identidade, parse, build, execução, timeout, OOM, swap, saída ausente
ou ilegível é `Unknown`; `Unknown` obrigatório bloqueia e nunca recebe crédito.
Repetição e ordem inversa devem produzir a mesma classificação. O contrato
mede morfologia/repr pública; não compara bytes PDF ou mecânica interna.

## Contenção de memória e gates

Não executar `cargo test` amplo nem compilar o lib-test monolítico. Sondas CLI
e oráculo rodam um processo por vez sob `RLIMIT_AS` de 512 MiB e timeout de 60
s. Build/check/lint rodam serialmente, uma job, sob `RLIMIT_DATA` de 2 GiB:

```bash
bash -lc 'ulimit -v 524288; exec timeout 60s /usr/bin/time -v <oraculo-cli-p1396>'
bash -lc 'ulimit -d 2097152; export CARGO_BUILD_JOBS=1; exec timeout 360s /usr/bin/time -v cargo build --release --bin typst'
bash -lc 'ulimit -d 2097152; export CARGO_BUILD_JOBS=1; exec timeout 240s /usr/bin/time -v cargo check -p typst-core'
bash -lc 'ulimit -d 2097152; export RAYON_NUM_THREADS=1; exec timeout 180s /usr/bin/time -v crystalline-lint .'
```

Cada recibo registra comando, instante, HEAD, `git diff HEAD --stat`, exit,
duração, RSS máximo e swap. Swap diferente de zero ou RSS acima de 1.782.579
KiB interrompe a sequência. Duas revisões sem ganho discriminatório na mesma
causa reabrem o desenho; o teto não é elevado para transformar falha em verde.

## Capacidades segregadas

- autor de intenção/contrato `/root/contrato_p1396`: lê L0, baseline, fonte e
  medições; escreve somente este passo, o L0 proprietário e o contrato JSON;
  não lê candidato futuro nem emite selo/veredito;
- autor de oráculos: recebe L0 e contrato congelados; escreve somente suíte e
  recibo, sem adaptar expected ao patch;
- adversário: recebe L0, contrato e baseline; escreve somente catálogo de
  mutações, sem corrigir solução;
- implementador: recebe contrato selado; escreve somente os dois literais e a
  linhagem autorizada;
- verificador: lê artefatos protegidos e saídas, escreve recibo/certificado e
  não altera o material julgado.

Hashes de oráculo, mutantes e selo ficam deliberadamente pendentes às
autoridades posteriores. Este passo não alega paridade matemática geral.

## R1 — alinhar expectativas test-only da fachada antes do candidato

Uma inspeção adicional do baseline, ainda antes de ler qualquer candidata,
encontrou uma contradição protegida fora do owner produtivo: o consumer
`01_core/src/compiler/stdlib/mod.rs`, identificador administrativo
[dado administrativo removido],
possui em `:12759-12777` dois testes P299 que ainda exigem U+0020 para
`liminf` e `limsup`. Seu header pertence ao L0 da fachada
`compiler/stdlib/_comum.md`, cujo baseline tinha identificador administrativo
[dado administrativo removido].

O L0 da fachada foi atualizado primeiro, sem tocar código, para obrigar os
dois expected a usar U+2009 e o comentário a nomear o espaço fino. Seu novo
identificador administrativo é
[dado administrativo removido].
R1 autoriza em `stdlib/mod.rs` somente os dois expected e comentários locais
correspondentes, mais a atualização de linhagem desse hub. Nenhum helper,
reexport, outro teste ou código produtivo da fachada pode mudar.

O pré-selo R0
[dado administrativo removido]
fica invalidado porque passo, contrato e escopo protegido mudaram. Oráculo e
catálogo adversarial não são alterados: continuam sendo o gate dinâmico da
morfologia produtiva, mas precisam ser novamente avaliados por um selador
contra este contrato R1. Nenhum `cargo test` ou lib-test monolítico será
compilado; a coerência das duas expectativas test-only é verificada por diff
estrutural e o comportamento por oráculo CLI sob o limite já congelado.

Durante R1, o L0 estrutural passou do hash semântico pré-candidato
[dado administrativo removido]
para o hash de linhagem
[dado administrativo removido].
O coordenador confirmou que a única diferença foi o resselo automático
autorizado de `Hash do Código: 4a4d07ab` para `e702b9c5`; a obrigação semântica
P1396 não mudou. O autor R1 não leu o diff nem o código candidato.

## R2 — comentário local do owner produtivo

Antes do veredito, o verificador encontrou no baseline do owner estrutural,
em `structural/math.rs:1155-1156`, um comentário que ainda exemplifica
`liminf → "lim inf"` com U+0020. Ele contradiz a obrigação U+2009 já explícita
no L0, embora não altere comportamento.

R2 autoriza somente atualizar esse comentário local para nomear espaço fino
U+2009 e representar `liminf` com esse codepoint. O restante do delta R1
permanece fechado: dois literais produtivos, dois expected/comentários P299 e
linhagem dos dois owners. O conteúdo semântico do L0 estrutural não precisa de
nova edição; somente seu resselo eventual continua autorizado.

O pré-selo R1
[dado administrativo removido]
fica superseded pela ampliação do delta. Oráculo e observáveis não mudam; não
se reexecuta corpus, lib-test, build ou lint nesta revisão contratual. Um novo
pré-selo R2 deve proteger o comentário e provar que o catálogo adversarial
rejeita sua permanência em U+0020. O autor R2 não leu a candidata.

Durante R2, `_comum.md` passou do hash semântico pré-lineage
[dado administrativo removido]
para o hash atual de linhagem
[dado administrativo removido].
O coordenador confirmou que a única mudança foi o resselo automático
autorizado de `Hash do Código`; o conteúdo semântico R1 não mudou. O autor R2
não leu esse diff nem a candidata.
