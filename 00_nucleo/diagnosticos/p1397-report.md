# P1397 — relatório final de verificação R2

## Veredito

**Preserved — PASS_LIMITED_FRAGMENT.** A candidata preserva os 11 observáveis
congelados: `math.dif`, `math.Dif`, duas classes genéricas, seis wrappers de
estilo e o controle `math.lim`. O resultado foi **11 Preserved / 0 Difference /
0 Unknown**, com repetição e ordem inversa idênticas.

O protocolo completo Tekt foi **executado sem atestação de isolamento**. O
workspace e o contexto herdado eram compartilhados; a segregação efetiva foi
por autoridade de escrita, ordem causal e hashes protegidos.

Este veredito cobre somente a morfologia pública de `repr` congelada em P1397 e
o refinamento geral dos braços `MathClassOverride` e `MathStyled`. Não prova
paridade matemática geral nem paridade geral com Typst.

## Pré-selo e identidades

O pré-selo R2 foi aceito com SHA-256
`be1c476158b84dfff131ae569b6a8233c0005637d8cb38702d2cb1ea23c7083a`.
As identidades protegidas conferiram:

- L0 final: `cf13a3d23c7159125732b6dddeb3f8883a08c6b5cfa2ab9a877908b2761f8e24`;
- passo: `23ea09186ea561457685dc594b8e5276301a2378f8a76d5d2103d119e889af44`;
- contrato: `2a381fbf2c9e69e8d26526c174b7252a796d8fe0778b535555588d8ed405e0ef`;
- oráculo: `70357b440857f1f6b0f50b8cd1c3d7c2fdeef653291495dd13b5955d1ef99f9e`;
- recibo dos oráculos: `3d4f42165ce35f5f6621e5e3bd16ef95922909bd0ebb6f9b2fe993c9b0fa23bc`;
- catálogo adversarial: `8e91a1fd58a537f1c71b85c025acf283eb263938e591d6f86330650ac2466fe6`;
- fonte candidata: `c31e786562cbbcd7d874c91ef3b8884fa802cb64ca7c84f421b761bb5ee9cc52`;
- binário candidato: `50c137a42d8a0da4ba43213c2a04e0f8896478a8fc05e25da7e9fee9247a873c`.

A fonte vanila ratificada confirma a intenção: `ClassElem` declara `class` e
`body` nessa ordem; a representação genérica usa campos nomeados; `StyledElem`
usa `styled(child: ..., ..)`; e `dif`/`Dif` combinam espaço fino fraco, classe
unária e estilo upright. As quatro fontes conservaram os hashes selados.

## Refinamento e teste estático

O diff produtivo altera somente os dois braços autorizados, além de metadata de
linhagem e do teste local P1397:

- `MathClassOverride` deriva o nome da classe real, representa o corpo
  recursivamente e entrega os campos `class`/`body` ao formatter canônico;
- `MathStyled` testa de forma geral a presença de `kind`, `bold`, `italic` ou
  `cramped`, sem lista de styles, nome de binding, glifo ou classe especial;
- com todos os quatro eixos em `None`, a condição é falsa e o wrapper vazio
  continua achatando para `repr_content(&body)`;
- o teste estático P1397 cobre `kind`, `bold`, `italic`, `cramped` e composição
  de classe com estilo. Ele foi inspecionado, mas não compilado nem executado,
  conforme a proibição do lib-test monolítico. A fronteira do wrapper vazio é
  comprovada pela auditoria direta do braço produtivo, não por assertion própria.

Não há `dif`, `Dif`, `unary`, `relation`, letras ou kinds nominais nos dois
braços produtivos alterados. O formatter de sequência e `MathOp` não mudaram.

## Gates funcionais e adversariais

O verificador reexecutou o oráculo completo sob `RLIMIT_AS` de 512 MiB e timeout
externo de 60 s. O recibo `/tmp/p1397-final-verifier.json`, SHA-256
`60eb72ad8c1470cc3bb2f1b57c9bfda206ee93515bc43452ef137a3b352c4569`,
terminou exit 0 em 3,86 s, RSS máximo de 33.476 KiB e swap zero. A identidade
do binário foi aceita e as três ordens produziram exatamente os mesmos bytes.

O gate adversarial reconciliado rejeita 16 de 16 mutantes aplicáveis, score
`1.0`, sem sobrevivente e sem `Unknown`. É um gate composto
estático/contrafactual com testemunhas dinâmicas; não significa que 16 binários
mutantes tenham sido compilados. A política continua fail-closed.

## Memória e gates leves

Nenhum `cargo test`, lib-test monolítico ou teste amplo de workspace foi
executado. O build release fornecido pelo implementador rodou serialmente, uma
job, sob `RLIMIT_DATA` de 2 GiB: exit 0, 2m26s, RSS máximo de 1.135.180 KiB e
swap zero. O oráculo final anterior do implementador também terminou exit 0,
RSS 33.624 KiB e swap zero.

Na verificação independente:

- `crystalline-lint .`: exit 0, zero violações, 17,99 s, RSS 70.768 KiB, swap
  zero; warnings/infos históricos permaneceram não bloqueantes e nenhuma
  violação apontou os caminhos P1397;
- `cargo fmt --check`: exit 0;
- `git diff --check`: exit 0;
- parse JSON dos artefatos selados: exit 0.

O maior RSS válido do passo foi 1.135.180 KiB, abaixo do teto de interrupção de
1.782.579 KiB. Não houve swap nem OOM.
