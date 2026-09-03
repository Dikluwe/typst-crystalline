# Passo 1294 — sanitização aditiva da selagem terminal do P1293

## Estado e natureza

Este passo é exclusivamente de processo e evidência. Ele não implementa semântica,
não altera comportamento público e não reabre os lotes A, B, C ou D do P1293.

- Regime: sanitização segregada por capacidades e artefatos, sem alegação de
  isolamento técnico de leitura.
- Classificação ADR-0127: correção documental/criptográfica; não há gate humano de
  L0 porque não existe mudança de contrato público, comportamento por defeito,
  fase de pipeline ou compatibilidade.
- L0: não aplicável. É proibido alterar qualquer Prompt L0 ou Núcleo Tekt.
- Mutation testing novo: não aplicável. O produto e o contrato observável estão
  congelados; a campanha existente é verificada por hash e conteúdo, não refeita.
- Política de `Unknown`: `Unknown` nunca é sucesso e bloqueia o certificado.

**Retificação autorizada pelo dono em `2026-09-02T21:36:22-03:00`:** a
premissa original de conflito no digest terminal foi refutada antes de S1
escrever qualquer artefato. Este passo passa a registrar o digest terminal como
válido e a sanitizar aditivamente a anomalia histórica realmente medida: uma
chave JSON duplicada no manifesto P1293. Os bytes P1293 continuam congelados.

## Motivação medida antes da decisão

Auditoria inicial em `2026-09-02T20:52:38-03:00`, sobre o HEAD anterior
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, mediu corretamente o predecessor
e o inventário, mas alegou um conflito no digest terminal. S1 parou sem escrita
ao não reproduzi-lo. A revalidação independente no baseline commitado, com
Python `3.12.3` e o algoritmo canônico literal deste passo, encontrou:

1. O bloco
   `$.updated_definitive_final_candidate_read_only` de
   `00_nucleo/diagnosticos/p1293-contract-seal.json` recalcula corretamente para
   `04460e7920e9f87269b2594829aeb7e9cb878bd4373d1c94cb85ded45f2e8fbc`.
2. O inventário efetivo recalcula para
   `33fe3dcd9cd287b24a82e255f7dffd2728cc76aab969ad5b049d35bbcf7b9c42`,
   com `74/74` caminhos presentes e zero divergências.
3. O bloco terminal `$.final_complete_read_only` possui `4.588` bytes canônicos
   e recalcula para
   `077559cc1632cdd684532c2dec1638a9f57277b806b8263e519cadad7e50f045`,
   exatamente o digest declarado pelo metadado irmão. A alegação anterior
   `dcb5a0911cb9e9f7c1ecd8cb89a1697b8a058972b92bfd052be95ef24d5ff394`
   está refutada e não pode ser promovida a digest real.
4. O manifesto P1293 bruto, cujo SHA-256 permanece
   `6c8571c03437749598d736025296b190a4e3839730eec68be6368e4b5479b03c`,
   contém duas ocorrências da chave
   `$.contract_reopening_attach_ic_b.replacement_serial_seal`, com valores
   distintos. Em ordem textual, os valores canônicos medem `864` e `581` bytes
   e têm SHA-256, respectivamente,
   `181f102cb6c57b4a3d33e0ec3891f6e53e5683eee5a9f9d13140ca8a458c6679`
   e `029fcf801a8ff169b2aba3024873fed28d5b45949247f104ba87135c39e50fb2`.
   Um parser que aplica silently last-wins perde a primeira ocorrência; por
   isso a detecção usa pares de objeto e rejeição explícita de duplicatas.
5. A primeira repetição de `cargo test --workspace -q` expirou no teste temporal
   preexistente `p1137_watch_dependencias_recuperacao_e_filtro`; o teste isolado
   passou em `1,86 s` e a segunda suíte completa passou. Isto é flake observado,
   não equivalência nem sucesso determinístico.
6. Ao preparar o commit, `git diff --cached --check` detectou whitespace Markdown
   em recibos históricos antes não rastreados. Esses bytes foram preservados porque
   reformatá-los retroativamente quebraria hashes e linhagem.

A decisão é preservar os artefatos P1293 byte a byte e construir uma cadeia
P1294 aditiva que ateste a validade do digest terminal, registre a alegação
refutada sem tratá-la como verdade e exponha as duas ocorrências da chave
duplicada sem normalizar retroativamente o manifesto histórico.

## Baseline congelado

- Commit: `5079a0cdfaedc406b7c036ac61e40d7f6a1264d6`
- Data do commit: `2026-09-02T20:55:50-03:00`
- Assunto: `feat: close parity work through step 1293`
- Estado inicial exigido: árvore limpa.

Hashes P1293 congelados:

| Artefato | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1293-manifest.json` | `6c8571c03437749598d736025296b190a4e3839730eec68be6368e4b5479b03c` |
| `00_nucleo/diagnosticos/p1293-contract-seal.json` | `02701249dfeb7eaf0130c71b87393345bf93dfdefd87a72b489f897f5aba723c` |
| `00_nucleo/diagnosticos/p1293-final-certificate.json` | `52d9f7f2d0aef71b697453c46c55aefdb695754470f0dd45f5f7ce85df378660` |
| `00_nucleo/diagnosticos/p1293-final-verification-receipt.json` | `2ade14a5fc97b240e7c419925c6c3361b05c45e0315bc2f9ffadb244eefe87a6` |
| `00_nucleo/diagnosticos/p1293-preservation-verification-receipt.json` | `396c05a5319df8549e934a3764f3a0c42183fb3fa2174b346783bf430f7aa1b5` |
| `00_nucleo/diagnosticos/p1293-final-report.md` | `698362737ed9f1ffa2169a3d1ad519ad43a2bfa0513b2c39ebe59a7a2c092206` |
| `04_wiring/tests/p1292_contract.rs` | `5104b48ff89766e9910cd3a39eef3706fe0c8539a0fa6cb7dafcb3cf66a7484a` |
| `04_wiring/tests/p1293_contract.rs` | `2505aa34538067c743530f4a15feaf0a1b937c24bb51e2abdd64c1bc4151c7cb` |
| `target/release/typst` | `c527b4111493f444d66e44e6d38d4823b2d09515c73d92eda1b9c4a2a86bff90` |

O binário em `target/` não pertence ao Git; seu hash serve como testemunha de
reprodução e deve ser medido novamente depois de `cargo build --release`.

## Objetivo vinculante

Produzir uma âncora terminal P1294 verificável que:

1. preserve todos os artefatos P1293 byte a byte;
2. registre que o digest terminal declarado e o digest canônico real são o
   mesmo `077559cc...`, e que `dcb5a091...` é uma alegação refutada;
3. registre a chave duplicada histórica, as duas ocorrências em ordem textual e
   os hashes distintos de seus valores, sem editar o manifesto P1293;
4. valide novamente o predecessor canônico e o inventário de 74 caminhos;
5. prenda manifesto, selo, recibo e certificado sem autorreferência;
6. não transforme o incidente operacional anterior nem o flake de watch em sucesso;
7. não alegue equivalência funcional geral, isolamento técnico ou cumprimento
   operacional perfeito.

## Allowlist total de escrita

Após este passo existir, somente estes novos arquivos podem ser escritos:

1. `00_nucleo/diagnosticos/p1294-sanitization-manifest.json`
2. `00_nucleo/diagnosticos/p1294-terminal-seal.json`
3. `00_nucleo/diagnosticos/p1294-verification-receipt.json`
4. `00_nucleo/diagnosticos/p1294-certificate.json`
5. `00_nucleo/diagnosticos/p1294-final-report.md`

Este próprio documento é a sexta diferença permitida contra o baseline.

É proibida qualquer escrita em:

- `00_nucleo/prompts/` e `00_nucleo/prompts/_nuclei/`;
- `01_core/`, `02_shell/`, `03_infra/` e `04_wiring/`;
- qualquer artefato cujo nome comece por `p1293-`;
- oráculos, testes, superfícies ou arquivos em `lab/`;
- `00_nucleo/context/`;
- qualquer outro passo de materialização.

## Papéis e ordem causal

### Papel S1 — autor da obrigação e do manifesto

Entradas permitidas:

- este passo;
- commit baseline e os nove hashes congelados;
- leitura dos artefatos P1293 necessários para construir o inventário.

Escrita permitida somente em
`p1294-sanitization-manifest.json`.

O manifesto deve registrar:

- versão do protocolo e regime limitado;
- baseline completo e árvore inicialmente limpa;
- hashes brutos dos artefatos congelados;
- caminho JSON, tamanho canônico e igualdade entre digest declarado e real do
  bloco terminal, além da alegação `dcb5a091...` marcada como refutada;
- caminho da chave duplicada no manifesto P1293, contagem `2`, ordem textual,
  conteúdo materializado dos dois valores e seus hashes canônicos distintos;
- algoritmo canônico;
- método de detecção de duplicatas por pares de objeto, sem semântica last-wins;
- inventário de 74 caminhos materializado, não apenas uma referência narrativa;
- allowlists por papel;
- política de `Unknown`;
- incidente `PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED`;
- flake de watch como risco observado, ainda não absolvido.

Depois de publicado e hasheado, o manifesto fica somente leitura.

### Papel S2 — selador documental

Entradas permitidas:

- este passo;
- manifesto S1 e seu SHA-256;
- artefatos P1293 congelados, somente leitura.

Escrita permitida somente em `p1294-terminal-seal.json`.

S2 não pode escrever manifesto, produto, L0, teste, oráculo, artefato P1293,
recibo de verificação ou certificado.

O selo deve possuir dois objetos irmãos:

- `sanitized_terminal_metadata`, com versão, estado, escopo canônico e digest;
- `sanitized_terminal`, objeto canônico que contém baseline, hashes brutos,
  predecessor e terminal válidos, alegação de digest refutada, anomalia de chave
  duplicada, inventário e limites da claim.

O digest em `sanitized_terminal_metadata.canonical_sha256` deve ser calculado
somente sobre `sanitized_terminal`. Não incluir o metadado, o arquivo inteiro ou
um futuro certificado no objeto hasheado.

Depois de escrito e hasheado, o selo fica somente leitura.

### Papel S3 — verificador independente e certificador

Entradas permitidas:

- este passo;
- manifesto e selo já congelados;
- artefatos P1293 e árvore do baseline somente leitura;
- execução dos gates listados abaixo.

Escrita permitida somente em:

- `p1294-verification-receipt.json`;
- depois do recibo congelado, `p1294-certificate.json`;
- depois do certificado congelado, `p1294-final-report.md`.

S3 não corrige entradas que verifica. Qualquer erro devolve o fluxo a S1 ou S2
com novo hash e recibo de invalidação; não existe correção silenciosa.

## Algoritmo canônico obrigatório

Usar JSON UTF-8 semanticamente equivalente a:

```text
json.dumps(
    objeto,
    sort_keys=True,
    separators=(",", ":"),
    ensure_ascii=False,
).encode("utf-8")
```

As chaves são ordenadas recursivamente; arrays preservam ordem. O digest é SHA-256
dos bytes resultantes. A implementação usada deve ser registrada por caminho,
versão da runtime, comando e SHA-256 quando houver script auxiliar.

## Gates de S1 — congelamento

1. Confirmar HEAD exatamente
   `5079a0cdfaedc406b7c036ac61e40d7f6a1264d6` antes das novas escritas.
2. Confirmar árvore limpa antes da criação deste passo; depois, distinguir este
   passo das cinco futuras saídas.
3. Recalcular os nove hashes congelados.
4. Recalcular o bloco predecessor como `04460e7920e9f87269b2594829aeb7e9cb878bd4373d1c94cb85ded45f2e8fbc`.
5. Recalcular os `4.588` bytes canônicos do terminal como
   `077559cc1632cdd684532c2dec1638a9f57277b806b8263e519cadad7e50f045`.
6. Confirmar que o metadado histórico declara exatamente o mesmo digest.
7. Confirmar que `dcb5a0911cb9e9f7c1ecd8cb89a1697b8a058972b92bfd052be95ef24d5ff394`
   não é produzido pelo algoritmo obrigatório e registrá-lo somente como
   alegação refutada.
8. Detectar exatamente duas ocorrências de
   `$.contract_reopening_attach_ic_b.replacement_serial_seal`, preservar a
   ordem textual e recalcular os hashes dos valores como `181f102c...` e
   `029fcf80...`; qualquer parser last-wins usado como prova bloqueia S2.
9. Materializar e verificar o inventário: `74/74`, zero ausente, zero divergente,
   digest `33fe3dcd9cd287b24a82e255f7dffd2728cc76aab969ad5b049d35bbcf7b9c42`.

Qualquer valor inesperado bloqueia S2.

## Gates de S2 — selo

1. Validar JSON estrito e sem chaves duplicadas do manifesto e do selo. O
   manifesto P1293 histórico é validado como bytes congelados mais detecção da
   duplicata; não é silenciosamente normalizado nem aceito como entrada unívoca.
2. Confirmar o SHA-256 bruto do manifesto recebido.
3. Recalcular `sanitized_terminal` duas vezes em processos novos.
4. Recalcular em ordem de leitura inversa; o digest deve permanecer igual.
5. Confirmar que o digest do selo aponta para o bloco correto e não para o
   documento inteiro.
6. Confirmar que o selo contém zero autoridade de escrita futura e não declara
   aprovação funcional nova.
7. Calcular o SHA-256 bruto do selo somente após sua última escrita.

Qualquer divergência bloqueia S3.

## Gates de S3 — preservação e fechamento

### Integridade e arquitetura

- Recalcular manifesto, selo, bloco sanitizado e inventário.
- Recalcular a igualdade do digest terminal e detectar novamente as duas
  ocorrências distintas da chave histórica duplicada.
- Recalcular os hashes dos seis artefatos finais P1293.
- Confirmar os oráculos P1292 e P1293 byte a byte.
- `cargo fmt --all -- --check`
- `crystalline-lint .`
- Para cada check `V3`, `V4`, `V5`, `V7`, `V13`, `V14`, `V15` e `V26`, executar
  `crystalline-lint --checks <check> --fail-on warning .`.
- `crystalline-lint --fix-hashes --dry-run .` deve imprimir `Nothing to fix`.

### Preservação funcional

Executar duas ordens independentes:

```text
cargo test -p typst-wiring --test p1292_contract -- --nocapture
cargo test -p typst-wiring --test p1293_contract -- --nocapture
```

e depois a ordem inversa. Cada suíte deve produzir `11/11` em ambas as ordens.

Executar `cargo test --workspace -q` duas vezes. Qualquer falha, inclusive timeout
de `p1137_watch_dependencias_recuperacao_e_filtro`, bloqueia o certificado naquela
execução. Uma repetição verde não apaga uma anterior vermelha: o recibo deve
registrar ambas e classificar o flake sem atribuí-lo ao P1293 sem prova causal.

Executar `cargo build --release` e confirmar o SHA-256 do binário como
`c527b4111493f444d66e44e6d38d4823b2d09515c73d92eda1b9c4a2a86bff90`.

### Superfícies

Regenerar em `/tmp` os perfis default e HTML com os probes congelados e comparar
byte a byte com:

- `00_nucleo/diagnosticos/p1293-surface-default.json`, esperado `111/99/12`;
- `00_nucleo/diagnosticos/p1293-surface-html.json`, esperado `115/104/11`.

Ambos exigem zero missing, zero unverified e zero `Unknown`.

### Evidência de mutação preservada

Não executar mutantes contra a árvore produtiva. Verificar:

- runner `/tmp/p1293-final-observers-runner-31.py` com SHA-256
  `300b6e5cf5f65cdc3727bfa9cf781f324d75d7ba93420eb0beeb73074168d70f`;
- resultado `/tmp/p1293-final-74-results-31.json` com SHA-256
  `a2ae8c75b7792201c902e8c8d38e6ec2071328ebbcb10a6e09f6d7372a051e55`;
- dois baselines verdes, 31 mutações forward, 31 reverse, score `1.0`, zero
  sobreviventes, zero `Unknown` e estabilidade de ordem.

Se os arquivos temporários não existirem ou divergirem, classificar a evidência
como não reproduzida; não recriá-la dentro deste passo nem convertê-la em sucesso.

### Higiene integral do novo delta

Antes do certificado, o conjunto exato de diferenças contra o baseline deve ser
somente este passo e os cinco artefatos P1294. Nenhuma renomeação ou deleção.

Como arquivos novos não rastreados não aparecem em `git diff --check`, o
integrador deve adicionar ao índice somente a allowlist exata e executar:

```text
git diff --cached --check
```

Também verificar mecanicamente zero whitespace final e exatamente um newline EOF
nos cinco artefatos P1294. Whitespace já commitado em recibos P1292/P1293 é
histórico congelado e fica fora deste gate; não deve ser reformatado.

## Estrutura do recibo e certificado

O recibo S3 deve conter:

- timestamps de início e fim com timezone;
- HEAD e estado exato da árvore;
- comandos, códigos de saída e hashes de logs;
- hashes brutos do manifesto e selo;
- digest canônico recalculado;
- resultado `74/74` do inventário;
- todas as repetições de testes, inclusive falhas;
- resultado das superfícies e dos gates arquiteturais;
- `Unknown`, survivors e mismatches explicitamente numéricos;
- veredito `PASS_READY_FOR_CERTIFICATE` ou `BLOCKED`.

O certificado só pode ser emitido depois de congelar o recibo. Ele deve piná-lo
por SHA-256, além do manifesto, selo, baseline e bloco sanitizado. O certificado
é a âncora terminal e não pode ser posteriormente inserido no manifesto ou no
selo. O commit Git futuro será uma âncora externa adicional, não autorreferência.

Claim máxima permitida:

```text
Selagem terminal P1293 auditada e anomalia histórica sanitizada aditivamente
para os artefatos, versões e fragmento observável registrados, sem isolamento
técnico de leitura.
```

O certificado deve repetir literalmente:

```text
PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED
```

e negar explicitamente equivalência funcional geral, isolamento técnico de
leitura e cumprimento operacional perfeito.

## Condições de bloqueio

Bloquear sem certificado se ocorrer qualquer um dos seguintes:

- alteração de arquivo P1293, produto, L0, núcleo, teste, oráculo ou `lab/`;
- diferença fora da allowlist de seis caminhos;
- hash congelado, bloco canônico ou inventário divergente;
- digest terminal válido tratado como defeituoso, alegação refutada
  `dcb5a091...` promovida a digest real ou digest histórico sobrescrito;
- chave duplicada histórica omitida, reduzida por last-wins, normalizada ou
  alterada no manifesto P1293;
- qualquer `Unknown`, survivor ou caminho ausente;
- verificador alterando manifesto ou selo;
- teste protegido ou gate arquitetural falhando;
- qualquer uma das duas suítes workspace falhando;
- certificado incorporado retroativamente em entrada já selada;
- tentativa de absolver o incidente operacional ou o flake sem nova prova.

## Critério de conclusão

O P1294 conclui somente quando:

1. os cinco artefatos P1294 existem na ordem causal definida;
2. o delta inteiro respeita a allowlist;
3. o digest canônico do novo bloco recalcula exatamente em repetição e ordem
   inversa;
4. o inventário permanece `74/74` e os artefatos P1293 permanecem byte a byte;
5. todos os gates obrigatórios passam sem `Unknown`;
6. o certificado terminal pina manifesto, selo e recibo sem autorreferência;
7. o relatório final distingue aprovação funcional preservada, sanitização
   documental e limitações de atestação.

Até lá, o estado correto é:

```text
P1293_FUNCTIONALLY_APPROVED_HISTORICAL_MANIFEST_DUPLICATE_UNSANITIZED
```
