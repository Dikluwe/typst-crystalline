# P1307-R4 — oráculo independente consolidado

Estado: `FROZEN_FOR_INDEPENDENT_DISCRIMINATOR`, não certificado de implementação.
Autor oracle/adversário: `/root/p1307_oracle`; nenhum candidato foi lido e
nenhum Rust foi escrito. Regime da skill `tekt-materializacao-segregada`:
**executado sem atestação de isolamento técnico**. A autoridade independente
de verificação é `/root/p1306_oracle`.

## Conteúdo congelado

O oráculo executável contém **455 casos**, cada um nos quatro perfis
default/html/a11y/html+a11y: **1820 células de expectativa**, sem Unknown.
São 332 casos não contextuais P1307, 48 casos R2 e 75 casos novos R4.
Os oito adaptadores antigos `eval --in` são explicitamente substituídos pelos
casos contextuais R2 de type/valor/serialização/representação. A ausência do
adaptador antigo no baseline não vira ausência de Location/LocatedContent.

Políticas por caso e perfil: 368 casos têm expectativa vanilla ratificada;
87 preservam baseline como controle ou dívida explicitamente identificada.
Isso não significa paridade geral. Entre as dívidas preservadas estão repr
geral de Content, classificação CBOR de Symbol/Content e binding positional
de sink não terminal. O caso desse sink é construído bilateralmente: não é
Unknown nem valor fictício. As correções declaradas de repr propagadas ao
CBOR têm novas expectativas de payload integral.

Cada caso traz source SHA-256, expressão/documento literal, origem da medição
em `measurement_ref`, lado escolhido em `expected_side` e observações vanilla,
baseline e `future_expected`. A comparação preserva integralmente a string
pública codificada; não usa sorting, round-trip, tamanho, substring ou pretty
normalization. Nos diagnósticos compara stdout, exit e stderr integral,
incluindo source exibida, carets, hints e traces. Só a identidade exata do
path temporário, absoluto ou relativo, é substituída pelo hash da source.

## Medições e refinamentos anteriores ao candidato

Proveniência: `p1307-r4-baseline.json` SHA-256
`52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`,
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` com working tree P1306
e os L0 R3 aprovados; lista exata, diff/stat e hashes no baseline. Vanilla
upstream ratificado `a51e02804`, executável SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
baseline cristalino SHA-256
`945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3`.
Os executáveis foram verificados antes das sondas; todos os runs conservam
argv, cwd, fonte, ambiente, horários, saídas e custo.

- Focal inicial: 32 runs observáveis. Fonte e medição distinguiram constructor
  `arguments(1,key:2)` de closure sink: o primeiro conserva positional-first;
  o sink retorna named remanescente antes dos posicionais capturados. Named
  repetidos e suas origens continuam necessários. Esse fato refinou o L0
  antes de candidato, em vez de congelar a hipótese causal errada.
- Extensão focal: 100 runs, com uma fixture pretendida como função que era
  Type (`str.with`). A tentativa fica preservada. Foi substituída pela nativa
  `repr.with`, com quatro runs de revisão afetada + controle; nenhuma matriz
  global foi executada enquanto a pré-condição estava inválida.
- Matriz nova: 66 casos × quatro perfis × dois binários × três ordens =
  1584 runs. Zero Unknown e zero instabilidade normal/repeat/reverse.
- Missing em aliases/With: 18 runs focais e 216 em matriz adicional. Para
  `{ let f = json.encode; f() }`, o vanilla ancora `f()` em 23..26; com
  `.with()` na definição, em 30..33; chamada direta de With em 0..20.
  TOML/YAML confirmam o mesmo contrato de chamada final real. Isso motivou
  o refinamento de transporte de span, sem inferir âncora do texto do erro.
- Integração dos adaptadores predecessores: 104 runs focais, todos Preserved
  contra o respectivo lado congelado, sem reexecutar suas matrizes completas.

O custo de processos efetivamente novos deste conjunto é 2058 runs;
reutilizações de rows na revisão não são contadas como execução nova.
O measurement conserva cada etapa, inclusive as duas validações estáticas
com a mesma falha de path: substituir `/tmp/...` antes de `../../../tmp/...`
deixava prefixo espúrio. A skill impôs pausa para revisão de desenho; o
coordenador aprovou substituições exatas por comprimento decrescente.
Os controles absoluto/relativo/não-path passaram antes do recorte de integração.
Não houve mudança de payload nem terceira execução completa para essa causa.

O baseline também refutou a suficiência isolada de join.rs: `Args + Args`
ainda não chegava ao braço existente. O owner arithmetic foi refinado pelo
coordenador antes do manifesto. A suíte distingue Add de With e preserva
argumentos/origens sobreviventes. A medição R4 não ajustou expected a candidato.

## API e uso posterior

`classify(expected, observed)` retorna Preserved, Violated ou Unknown.
Input incompleto, Source não resolvida, CLI inválida e contexto não executado
permanecem Unknown. A suíte não tem controle artificial de opacidade que possa
ser confundido com falha real obrigatória.

```text
python3 -B 00_nucleo/diagnosticos/p1307-r4-oracle.py replay --binary PATH
python3 -B 00_nucleo/diagnosticos/p1307-r4-oracle.py replay --binary PATH --reverse
```

`--case REGEX` e `--profile PERFIL` restringem explicitamente um recorte.
O replay escreve seu recibo em stdout; somente fixtures temporárias são
criadas. Não altera oráculo, medição ou código. A execução integral candidata
ainda não ocorreu e é obrigação posterior ao selo e RED independente.

O plano preserva as vinte famílias predecessoras e acrescenta quatorze planos
de ataques por owner/âncora/testemunha: perda de named antigo, fusão With
incorreta, ordem de sink, consumidos reintroduzidos, map/filter com spans
errados, callbacks reordenados, Add não conectado, repr With/State/Counter/
Location incorreta, CBOR sem delta ou com classificação indevidamente nova,
e carrier/views stale. **Nenhum mutante Rust foi executado; mutation_score
permanece null.** Aplicabilidade, compilação, witness e restauração serão
avaliadas por autoridade independente. Casos públicos não substituem a
auditoria de writers nem testes internos da mutabilidade pública de Args.

## Pins

- `p1307-r4-oracle.py`:
  `ef102f3a800475855b0cb21f40db666312cdb2f96fd9c867ea625b13c22b8e68`.
- `p1307-r4-oracle.json`:
  `d796d39ab7af2fc17cef0bf84440293f8cec51df5b35c547351141182c85f6ef`.
- `p1307-r4-measurement.json`:
  `e89a644b95248b4f2e03f47cb74bdbeef17d0e8644e260194c6724bbcd4c933f`.

Os 21 L0 efetivos e recibos de refinamento estão pinados no oracle. Números
acima derivam exclusivamente dos artefatos com estes hashes e dos horários
de suas etapas. Os predecessores P1307/R2/R3 permanecem byte-idênticos.

A revisão do verificador encontrou ainda cobertura faltante das duas rotas
math do refinamento de missing. Por decisão aditiva pré-selo do coordenador,
essa cobertura será um suplemento separado (`p1307-r4-math-*`), não edição
dos três arquivos congelados acima. O selo deverá exigir ambas as suítes.
