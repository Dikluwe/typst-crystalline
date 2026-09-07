# P1303 — revisão adversarial operacional R1 de M10

## Identidade e regime

- `step`: `P1303`
- `role`: `ADVERSARIO`
- `revision`: `R1`
- `scope`: somente a materialização operacional do mutante semântico M10
- `regime`: protocolo completo de materialização segregada, **executado sem
  atestação de isolamento técnico**
- `artifact_kind`: addendum detached; o SHA-256 deste arquivo é publicado
  externamente, sem ciclo de auto-hash
- `baseline_commit`: `5b4a0d0438a535c54fdb5e74b28903c1313f5bc2`
- `contract_id`: `C-P1303-v1`
- `contract_sha256`: `837cb4bd1f38428d93cdadf9216c0182c69e3d8bfa8eb489ed52a710e2bbdf8a`
- `contract_size_bytes`: `17189`
- `parent_adversarial_plan_sha256`:
  `ebaaad08d475a53fc2dc6fe41ea69c901069f302633b7e41ba0f5293031057b8`
- `parent_adversarial_plan_size_bytes`: `20907`
- `required_mutation_score`: `1.0`
- `mutation_denominator`: `14`, invariável
- `revision_index`: `1`
- `adversarial_revision_budget`: `3`
- `revisions_remaining_after_R1`: `2`

Este addendum não modifica, reabre nem reinterpreta C-P1303-v1 ou os outros
treze mutantes. Ele corrige somente o operador usado para materializar a
semântica já congelada de M10: “expor o trio sem `a11y-extras`”. O assassino e
a testemunha contratual permanecem independentes do mecanismo do patch.

## Entradas, capacidades e limites

Inputs protegidos já congelados pelo plano pai:

| Input | SHA-256 | Bytes |
|---|---|---:|
| `00_nucleo/diagnosticos/p1303-contract.md` | `837cb4bd1f38428d93cdadf9216c0182c69e3d8bfa8eb489ed52a710e2bbdf8a` | 17189 |
| `00_nucleo/diagnosticos/p1303-adversarial-plan.md` | `ebaaad08d475a53fc2dc6fe41ea69c901069f302633b7e41ba0f5293031057b8` | 20907 |
| baseline `01_core/src/compiler/eval/bindings/field_access.rs` | `29abd27cc01b347da9fbc12a93f05265882c043d36cd9a03cff5e6487a96e024` | 35871 |
| baseline `01_core/src/compiler/stdlib/pdf.rs` | `b1d7b325eace8f6d819a16c6ff7c135f1ca42781bacaec71888045dbd977ce4d` | 17833 |

A entrada factual nova é exclusivamente a observação de execução M10-A
fornecida no handoff ao adversário. Não foi lido o estado transitório da
working tree nem qualquer diff transitório. A identidade exata dos dois blobs
do candidato congelado não foi fornecida ao autor deste addendum e não é
inferida: antes de executar M10-B, o executor deve copiá-la do recibo P6
congelado para o ledger e verificar ambos os hashes. Ausência ou divergência de
um desses hashes bloqueia a ativação; não autoriza consultar ou adaptar-se a um
hunk transitório.

Capacidades deste papel:

- leitura: skill `tekt-materializacao-segregada` e suas duas referências;
  hashes/identidades congelados acima; observação M10-A recebida;
- escrita: somente
  `00_nucleo/diagnosticos/p1303-adversarial-revision-1.md`;
- não exercidas: leitura da working tree transitória, alteração de contrato,
  plano pai, L0, testes, oracle ou produto; execução de M10-B; staging; commit;
  selo; veredito final.

O ambiente e o contexto conversacional continuam compartilhados. Assim, esta
revisão declara `executado sem atestação de isolamento técnico`; hashes
identificam inputs e restauração, mas não provam isolamento.

## Medição M10-A recebida — antes da decisão

`measurement_id`: `P1303-M10-A`

Transformação executada: o registro de `pdf.table-summary`,
`pdf.header-cell` e `pdf.data-cell` em `compiler/stdlib/pdf.rs` foi tornado
incondicional, sem mudança simultânea em `field_access.rs`.

Observações recebidas:

- a variante compilou;
- o assassino
  `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y` permaneceu PASS;
- os três fields não ficaram observavelmente expostos em `default` ou `html`;
- o ramo especial feature-gated de `field_access` interceptou cada acesso
  antes do lookup no módulo;
- portanto não houve exit/sucesso nem retorno de kind `function` nos seis casos
  negativos que deveriam testemunhar a semântica M10;
- nenhum crédito de mutação foi produzido.

O handoff não incluiu comando integral, timestamp, duração, output bruto ou
hash do recibo M10-A. Este addendum registra somente a medição comunicada e não
a promove a recibo reproduzível. Esses campos ausentes não são preenchidos por
inferência.

Vetor observado relevante:

```text
registro stdlib incondicional: aplicado e compilável
default × 3 fields expostos:   não
html × 3 fields expostos:      não
assassino negativo:            PASS
testemunha exit/sucesso:       ausente
crédito no numerador:          0
```

## Decisão posterior à medição

Classificação obrigatória de M10-A:

```text
INVALID_MUTANT_EXECUTION
```

M10-A não é mutante sobrevivente, morto nem equivalente: a transformação
compilável não materializou a semântica negativa que pretendia testar. Logo:

- `killed_credit(M10-A) = 0`;
- M10-A fica fora do numerador;
- o denominador permanece exatamente `14` porque M10 continua aplicável e
  obrigatório;
- PASS do assassino sob M10-A não é evidência de fraqueza nem de suficiência;
- M10 só pode receber crédito quando uma execução válida materializar a
  exposição e o assassino a rejeitar com testemunha.

Esta decisão mede antes de redesenhar. Não altera expectativa, oracle, teste,
score exigido ou política de `Unknown`.

## Hipótese refutável R1

`hypothesis_id`: `P1303-M10-B-H1`

O registro incondicional em `pdf.rs` é necessário, mas insuficiente: enquanto
o interceptor especial para os três nomes e a feature desligada retornar o
diagnóstico antes do lookup, os bindings não são observáveis. Se, como uma
única ativação semântica, o trio for registrado incondicionalmente **e** somente
esse interceptor especial for desativado, o fluxo geral de `Value::Module`
alcançará o scope e devolverá as três `function` em `default` e `html`.

Refutam a hipótese:

- qualquer um dos seis acessos ainda produzir erro após ambos os hunks ativos;
- algum acesso chegar a `Unknown`, crash, timeout ou span irresolvível em vez
  de sucesso;
- ser necessário mudar catálogo, feature flags, API, target, CLI, oracle ou
  assassino além dos dois hunks descritos;
- os dois hunks não compilarem juntos sobre os blobs candidatos congelados.

## M10-B — um mutante semântico, dois hunks sincronizados

`mutant_id`: `M10-B`

M10-B substitui operacionalmente M10-A, mas continua sendo o único item M10 no
denominador. Os dois hunks abaixo formam uma única transação de ataque. Nenhum
resultado obtido com apenas um hunk ativo é execução do mutante.

### Hunk A — registrar o trio incondicionalmente

Arquivo temporariamente mutado:
`01_core/src/compiler/stdlib/pdf.rs`.

No guard que envolve exclusivamente as três definições
`table-summary`/`header-cell`/`data-cell`, trocar a condição
`features.contains(Feature::A11yExtras)` por uma condição sempre verdadeira,
preservando os três nomes, funções, ordem e todos os bindings vizinhos. Forma
operacional aceitável:

```diff
-    if features.contains(Feature::A11yExtras) {
+    if true {
```

Não mover `attach`/`artifact`, não alterar seus nomes e não modificar o corpo
das três funções.

### Hunk B — desativar somente o interceptor especial

Arquivo temporariamente mutado:
`01_core/src/compiler/eval/bindings/field_access.rs`.

No único `if` especial cujo target é o módulo `pdf`, cujos fields são
`table-summary | header-cell | data-cell` e cuja condição exige ausência de
`A11yExtras`, anexar uma conjunção falsa ao predicado. Isso impede somente o
`return Err` especial e deixa o fluxo cair no lookup geral de `Value::Module`.
Forma operacional aceitável:

```diff
             && !ctx
                 .features
                 .contains(crate::entities::compiler_features::Feature::A11yExtras)
+            && false
         {
```

Não desativar o lookup geral, não mudar a seleção geral de span, projeção
`std`/`global`, mensagem/hints ou interceptores de outros nomes/targets.

### Atomicidade da ativação

O executor deve:

1. pinar SHA-256 e bytes dos dois blobs candidatos limpos;
2. aplicar Hunk A com `apply_patch`;
3. sem compilar ou testar o estado parcial, aplicar Hunk B com `apply_patch`;
4. registrar os dois diffs e hashes mutados como uma única ativação M10-B;
5. se qualquer aplicação falhar, aplicar imediatamente o inverso de todo hunk
   já aplicado e classificar `INVALID_MUTANT_EXECUTION`, sem crédito;
6. só então compilar e executar o assassino focal.

Uma compilação falha não mata M10-B. Ela produz
`INVALID_MUTANT_EXECUTION`, mantém crédito zero e consome a tentativa de
calibração correspondente.

## Assassino e testemunha invariáveis

O assassino permanece exatamente:

```text
p1303_negativos_pdf_exatos_nos_perfis_sem_a11y
```

Comando focal esperado, sem adaptar filtro ou expectativa ao mutante:

```bash
cargo test -p typst-core p1303_negativos_pdf_exatos_nos_perfis_sem_a11y -- --test-threads=1
```

M10-B é validamente materializado somente se os seis casos
`default/html × data-cell/header-cell/table-summary` avaliarem com sucesso e
devolverem `function` no lugar do erro obrigatório. A testemunha assassina é:

```text
default: 3 sucessos inesperados, exit de avaliação 0, kind function
html:    3 sucessos inesperados, exit de avaliação 0, kind function
teste:   FAIL porque esperava erro único, mensagem/hints/span exatos
```

O processo `cargo test` deve sair não zero exclusivamente pela rejeição desses
sucessos inesperados. O ledger preserva output integral e hash, lista os seis
casos e demonstra que a falha vem do assassino congelado. Se a semântica de
sucesso for observada e o teste permanecer PASS, M10-B é um mutante válido
sobrevivente e o resultado obrigatório é
`P1303_BLOCKED_MUTANT_SURVIVED`. Se a semântica não for observada, a execução é
inválida; se houver `Unknown`, o gate fica bloqueado/inconclusivo. Nenhum desses
casos recebe crédito.

Depois do foco, a execução válida/morta de M10-B entra no gate completo de
ordem normal, repetição e ordem invertida definido no plano pai. R1 não reduz
essa obrigação.

## Restauração inversa e prova byte-idêntica

Após a execução focal, restaurar os dois arquivos exclusivamente por hunks
inversos com `apply_patch`, em ordem reversa: primeiro Hunk B, depois Hunk A.
São proibidos `git reset`, `git checkout`, `git restore`, staging e commit.

A restauração só é aceita quando:

- SHA-256 e tamanho de `field_access.rs` coincidem byte por byte com o blob
  candidato limpo pinado antes de M10-B;
- SHA-256 e tamanho de `pdf.rs` coincidem byte por byte com o blob candidato
  limpo pinado antes de M10-B;
- o diff literal dos dois paths em relação ao candidato fica vazio;
- o mesmo assassino volta a PASS no candidato restaurado;
- nenhum outro path mudou durante ativação/restauração.

Qualquer falha de restauração bloqueia o mutante seguinte e o gate completo.
Não se tenta “corrigir” manualmente o candidato.

## Budget, score e rendimento da revisão

R1 é a revisão `1/3` do operador adversarial M10 e deixa duas revisões de
calibração disponíveis. Ela não amplia o budget causal do produto nem autoriza
uma nova implementação. O delta discriminatório pretendido é tornar
observáveis seis sucessos onde M10-A produziu zero, mantendo o mesmo
assassino.

O score continua, sem afrouxamento:

```text
mutation_score = mutantes válidos mortos / 14
score exigido  = 14 / 14 = 1.0
crédito M10-A  = 0
crédito M10    = 1 somente após M10-B válido e morto
```

Se R1 não produzir novo vetor observável, registrar causa pública e custo. Duas
revisões consecutivas com o mesmo vetor e a mesma causa exigem revisão do
desenho antes da terceira. Budget esgotado nunca converte execução inválida,
`Unknown` ou sobrevivente em sucesso e nunca remove M10 do denominador.

## Veredito deste addendum

`P1303_M10_A_INVALID_MUTANT_EXECUTION_M10_B_READY`.

Este veredito aprova somente a especificação operacional revisada do ataque.
Não afirma que M10-B foi executado, morto ou restaurado; não sela contrato,
oracle, testes ou candidato; e não constitui certificação P1303.
