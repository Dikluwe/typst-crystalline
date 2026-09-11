# Passo 1347 — autenticar workers, retirar autojulgamento e atestar a imagem executada

## Regime e objetivo

Regime: **executado sem atestacao de isolamento**.

P1347 é uma cadeia nova de protocolo completo. Não reabre nem altera artefatos
pinados de P1346. Seu único objetivo é eliminar as três causas que produziram os
cinco sobreviventes adversariais finais de P1346:

1. stdout fabricado aceito como prova de execução do worker;
2. worker DAG capaz de declarar a rejeição da própria mutação;
3. troca do descritor depois da verificação no filho e antes do `execve` real.

Somente um focal independente com score `1.0`, zero regressões canônicas e zero
`Unknown` de transporte autoriza um pré-selo novo. Este passo não autoriza RED,
implementação das 38 cápsulas, full corpus nem certificado.

## Estado medido e proveniência

Medição em `2026-09-11T11:47:11-03:00`:

- `HEAD = 496c45ac6279d4298079f848b24b5e97ba88edc1`;
- working tree limpo antes da autoria deste passo;
- P1346: `typst-passo-1346.md`, SHA-256
  `33dc3b0f1f4772e9021a7fbe58803f52fa6c789337d662e2c85ce408031aee41`;
- relatório adversarial P1346 R2 JSON, SHA-256
  `60de39bf2588fab84953f3a2fefea4cfd842dac205f1b23b669830106ea6bdc6`;
- relatório adversarial P1346 R2 Markdown, SHA-256
  `bd0ada205586718106e2e4851dfb8cfb7f50475967bd272567ef10d92512b466`;
- contrato P1346 R1, SHA-256
  `baa9065e14d9a67cd3f804c8fcc4063e3fc6a176156409a36a03f630181e026f`;
- checker P1346 R2, SHA-256
  `c6c8e425b9387afd2d02d20d148d407c4f2b252cef3428ccb1eb1eec91894ec1`;
- case runner P1346 R2, SHA-256
  `09bfbd15cd7aec9fb600dbea85ed84c6e7059cd252c3c42ceb296ee7c6a67c96`;
- caller P1346 R2, SHA-256
  `ba4fd42247eadd04ee7a8af8519043a77113205340cb60edbe79128e29ceb120`.

Nota de reprodução: o autor do manifesto deve conferir todos os hashes antes do
freeze; qualquer divergência é `PROTECTED_INPUT` e interrompe a cadeia. Neste
mesmo estado, `os.execve in os.supports_fd`, `hasattr(os, "pidfd_open")` e
`hasattr(os, "posix_spawn")` resultaram `True True True`. Disponibilidade não é
prova de correção do protocolo.

## Bloqueio herdado

O focal adversarial P1346 R2 obteve `181/186`, score
`0.9731182795698925`, com quatro controles canônicos verdes e cinco
sobreviventes válidos:

- `P1346-R2A01-forged-worker-all-violated`;
- `P1346-R2A02-forged-worker-duplicate-ids`;
- `P1346-R2A03-forged-worker-pid-string`;
- `P1346-R2A07-forged-dag-worker`;
- `P1346-R2A16-postcheck-execve-fd-swap`.

P1346 declarou stop definitivo. Nenhum checker, caller, corpus, receipt ou
relatório `p1346-*` pode ser editado, reclassificado ou usado como selo. P1347
compõe P1346 como baseline falhado imutável.

## Auditoria L0 e ADR-0127

Nenhuma mudança L0 é necessária. O escopo é exclusivamente o protocolo de
diagnóstico que mede uma materialização futura já legitimada pelos L0 vigentes.
Não há contrato público do produto, comportamento por defeito, mudança de fase
do pipeline ou quebra de compatibilidade. O gate de paragem ADR-0127 não é
acionado.

## Contrato de execução observado pelo pai

O pai deixa de aceitar `subprocess.CompletedProcess` ou qualquer campo do JSON
como prova suficiente de execução. A nova primitiva de worker deve:

1. criar pipes e um desafio fresco de 32 bytes no pai;
2. iniciar o filho por uma operação que devolva ao pai a identidade do processo;
3. obter PID, término e status por observação do pai (`pidfd`/`waitpid`), nunca
   do stdout;
4. ligar a resposta ao desafio e ao SHA-256 da invocação esperada;
5. exigir envelope JSON canônico, schema fechado e tipos exatos;
6. rejeitar stdout fabricado, ausência de filho real, PID textual, PID
   autoalegado divergente, linha extra, timeout ou status inesperado;
7. classificar toda falha desse transporte como `Violated`, nunca `Unknown`.

O worker não é fonte de identidade processual. Se mantiver um PID redundante no
envelope, ele deve ser inteiro estrito, positivo e igual ao PID observado pelo
pai; a forma preferida é omiti-lo do payload e registrá-lo somente na evidência
parent-owned.

## Identidade exata dos 122 casos

O pai congela a sequência canônica dos 122 IDs antes da execução. A resposta do
worker deve conter exatamente uma ocorrência de cada ID, na mesma ordem, sem
duplicatas, omissões, extras ou aliases. Cardinalidade isolada não é prova.

Cada linha possui schema fechado e valores tipados. A resposta carrega o digest
da sequência esperada e do desafio, mas esses campos são replay protection, não
substituem PID/status observados. Qualquer diferença é
`Violated/PROTECTED_INPUT` antes de importar classificações.

## DAG sem autojulgamento

O worker DAG não pode emitir `classification`, `reason_code`, testemunha aceita
ou qualquer equivalente. Ele pode apenas construir bytes mutados e registrar a
receita determinística da mutação.

Para T08–T11, o dado autorizado é:

```text
baseline pinado + mutação sem veredito
        -> raiz temporária em /dev/shm
        -> validador real recebe o anchor externo original
        -> reabre e percorre a cadeia causal inteira
        -> pai observa exit/stdout/stderr
        -> pai deriva a classificação do contrato
```

Diferença de SHA entre original e mutado prova somente que os bytes mudaram.
Ela nunca prova que o DAG rejeitou o mutante. O resultado só é `Violated` quando
o validador real falha fechado com a razão pública e a testemunha previstas.

## Execução atestada do probe

O caminho textual `/proc/self/fd/N` deixa de ser autoridade de execução. Neste
host, `os.execve` aceita um descritor; a implementação deve executar o memfd
selado diretamente pelo FD, sem nova resolução de pathname.

Para detectar uma troca feita dentro do filho imediatamente antes do syscall,
o pai deve atestar a imagem carregada:

1. o filho solicita tracing antes do exec e chama `execve` pelo descritor;
2. o pai recebe a paragem de exec do mesmo PID observado;
3. enquanto o processo está parado, o pai abre a imagem real exposta pelo
   kernel em `/proc/<pid>/exe`, mede stat, tamanho e SHA-256 e compara com o
   executável pinado e com o memfd selado;
4. somente após igualdade exata o pai permite continuar e envia a requisição;
5. PID reutilizado, ausência de exec-stop, imagem ilegível, digest divergente,
   segundo exec, ptrace/pidfd indisponível ou falha de qualquer syscall é
   `Violated/PROBE_AUTHORITY`;
6. após o término, o pai confirma status, stdout/stderr ligados ao mesmo PID e
   imutabilidade do memfd.

O `Unknown` opaco continua autorizado somente depois desta autenticação. Um
binário adversarial que produza JSON semanticamente plausível deve ser rejeitado
pela identidade da imagem antes da interpretação do payload.

## Autoridades e capacidades

1. **Autor da intenção/passo**: escreve somente este passo; não aprova o próprio
   contrato ou veredito.
2. **Autor do contrato P1347**: recebe este passo, P1346 pinado e o relatório dos
   cinco sobreviventes; escreve somente spec, binding e recibo de contrato; não
   lê implementação P1347.
3. **Autor do oráculo P1347**: recebe o contrato fechado e baseline pinado;
   escreve somente checker, caller, runner, corpus e recibos `p1347-oracle-*`;
   não lê mutações privadas P1347.
4. **Adversário P1347**: recebe passo, contrato e baseline P1346; escreve somente
   ataques e relatório `p1347-adversary-*`; não lê implementação P1347 antes de
   congelar as mutações.
5. **Verificador**: recebe artefatos fechados, executa focais e emite veredito;
   não corrige contrato, oráculo ou ataques durante a decisão.

O workspace compartilhado e o contexto herdado são registrados. Nenhum nome de
agente, hash ou recibo isoladamente autoriza alegar isolamento forte.

## Ataques obrigatórios P1347

Reexecutar os cinco sobreviventes P1346 e acrescentar pelo menos:

- stdout com IDs corretos, nonce copiado e nenhum processo real;
- ID omitido, extra, duplicado, reordenado e Unicode/alias equivalente;
- PID `bool`, zero, negativo, textual, PID de sibling e PID correto sem
  `waitpid` correspondente;
- wait status, stderr ou stdout pertencente a outro processo;
- worker DAG que devolve rótulos corretos sem chamar o validador;
- mutação com bytes diferentes que o DAG real ainda aceita;
- validador chamado com anchor fornecido pelo mutante;
- troca do FD antes do exec, dentro do wrapper de exec e depois do exec-stop;
- binário adversarial que imita challenge, nonce, schema e saída canônica;
- ausência/falha de `pidfd`, tracing, exec-stop ou `/proc/<pid>/exe`.

Controles obrigatórios: worker real em ordem canônica, DAG canônico aceito,
quatro mutações T08–T11 rejeitadas pelo validador real e probe canônico
autenticado que permanece `Unknown`.

## Ordem, gates e orçamento

1. congelar este passo, P1346 e o estado do repositório;
2. autor independente escreve contrato P1347 e receipt;
3. oráculo e adversário recebem somente as entradas permitidas e trabalham sem
   ler a saída privada um do outro;
4. executar primeiro os cinco sobreviventes e os novos ataques diretamente
   relacionados;
5. somente se o recorte focal passar, reexecutar os 181 negativos antes mortos e
   os quatro controles P1346;
6. exigir score `1.0`, zero regressões e determinismo normal/repeat/reverse;
7. validar JSON/schemas/hashes, `python -m py_compile`,
   `crystalline-lint --checks v5,v15,v26 .` e `git diff --check`;
8. emitir pré-selo P1347 limitado ao transporte, ou blocker reproduzível.

Budget: uma revisão contratual, até duas revisões focais do oráculo e duas do
adversário; autores executam full=0. Duas revisões sem ganho causal, novo
survivor válido, regressão canônica, entrada protegida alterada ou mecanismo de
atestação indisponível interrompe P1347 sem waiver.

## Aceitação e limites

- os cinco sobreviventes P1346 são `Violated` pela causa correta;
- todas as variantes P1347 são `Violated`;
- worker canônico, DAG canônico e demais controles preservam seu resultado;
- só o probe opaco autenticado pode ser `Unknown`;
- nenhum resultado é aceito porque o próprio worker/mutante forneceu o rótulo;
- nenhum artefato P1346 é alterado;
- nenhum L0 ou consumer produtivo é alterado;
- full corpus, RED, 38 cápsulas e certificado permanecem fora deste passo.

O pré-selo P1347, se emitido, prova somente que o transporte corrigido discrimina
o corpus declarado. Não fecha P1339/P1340, não prova equivalência funcional
geral e não autoriza implementação produtiva sem passo subsequente explícito.
