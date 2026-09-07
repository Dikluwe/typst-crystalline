# P1307 — o contrato dos encoders exige reabrir o escopo

**Resultado: `P1307_REDESIGN_REQUIRED`. Não implementado; não está pronto para o gate de aprovação do L0.**

O problema encontrado não é a ausência de autorização genérica. O passo exige resultados e diagnósticos que os quatro módulos autorizados não conseguem garantir com as informações atualmente disponíveis. Escrever os encoders agora deixaria defeitos conhecidos no contrato. A §3 do próprio passo manda parar quando outro owner se torna necessário.

## Evidências que mudam a decisão

### O fallback textual já perde informação antes do serializer

O mesmo valor construído nos dois binários produz:

| Expressão | Vanilla ratificado | Cristalino do baseline |
|---|---|---|
| `repr(state("probe", 0))` | `state("probe", 0)` | `state(...)` |
| `repr(counter(heading))` | `counter(heading)` | `counter(...)` |

Testemunhas: `construct-State` e `construct-Counter` em `p1307-contract-measurement.json`. O vanilla serializa essas representações completas; usar o helper compartilhado atual propagaria a perda para JSON, TOML e YAML.

Fonte: `01_core/src/compiler/eval/repr.rs:122` e `:123`; o helper em `01_core/src/compiler/eval/mod.rs:83` delega a esse owner. O fallback vanilla está em `lab/typst-original/crates/typst-library/src/foundations/value.rs:343`. `repr.rs` não integra o allowlist do P1307. Não foi duplicada sua lógica dentro dos novos encoders.

Errata de localização no contrato congelado: sua seção R1 cita `repr.rs:124-125`; as linhas corretas no baseline pinado são `122-123`, verificadas com `nl -ba`. O artefato do autor foi preservado; esta correção não altera a testemunha nem o contrato.

Há também um indício de fonte para `Location`: o cristalino escreve `location(...)` em `repr.rs:87`, enquanto o vanilla medido escreve `location(..)`. **Não o apresentamos como construção bilateral comprovada**, porque a observação contextual do cristalino ficou bloqueada, conforme abaixo.

### `.with(...)` perde a origem necessária para apontar o erro

No vanilla, a expressão:

```typst
yaml.encode.with(pretty: false)((z: (3, 2), a: (b: 1)))
```

produz `unexpected argument: pretty` e destaca `pretty: false`, intervalo UTF-8 half-open **[17, 30)** da expressão. A origem é a pré-ligação, não a chamada final. Testemunha: `yaml-encoder-with-named` na medição do contrato, com stderr integral e fonte.

Em `01_core/src/compiler/eval/call_dispatch.rs:1016`, `merge_with_args` combina os valores, mas guarda apenas `new.span`. Em `01_core/src/entities/args.rs:18`, `Args` contém valores posicionais, mapa de nomeados e um único span agregado, sem origem individual. Uma nativa que recebe esses argumentos não pode recuperar genericamente o local original já descartado. Reconstruir spans por busca textual não resolve aliases, colisões ou pré-ligações vindas de outro arquivo.

Isso exige auditar o transporte da origem em `call_dispatch` e seus contratos relacionados. **Não está decidido que alterar `Args` ou `Func` seja inevitável**: a solução e qualquer ampliação de entidade precisam de L0 próprio e avaliação antes de autorização. O relatório não autoriza essas mudanças.

### Parte obrigatória do corpus ainda não é observável bilateralmente

Os testes contextuais de `Location` e `LocatedContent` usam `eval --in`, aceito pelo vanilla, mas rejeitado pelo cristalino fresco com exit code 2 (`unexpected argument '--in' found`). Isso é falha do caminho de medição, não demonstração de ausência dessas variantes no produto.

A medição do contrato preserva 96 células `Unknown` nessa classe: 8 casos contextuais × 4 perfis × 3 ordens. O oracle independente mantém 32 células `Unknown` na ordem normal: os mesmos 8 tipos de caso × 4 perfis. Não foram convertidos em diagnóstico de linguagem equivalente nem excluídos do universo. As ordens repetida e invertida do oracle foram interrompidas após a última revisão focal; executá-las sobre o mesmo bloqueio não acrescentaria prova.

## O que ficou medido e preservado

O contrato mediu 350 casos, bilateralmente, em `default`, `html`, `a11y` e `html+a11y`, nas ordens normal, repetida e invertida: 8.400 invocações. Inclui assinaturas, defaults, strings, mapas, `Symbol`, `Content`, `Bytes`, floats, chamadas `.with`, decoders, CBOR e negativos. **Quantidade de invocações não é quantidade de testes aprovados.** As ausências dos novos encoders são baseline, não RED de uma implementação autorizada.

O oracle tem autoria distinta e conserva expectativas literais vanilla; controles de preservação dos decoders e CBOR identificam separadamente dívidas do baseline. O plano das 20 famílias de mutantes é somente documental: nenhum mutante produtivo foi executado, não existe mutation score nem certificado de implementação.

Limite adicional do contrato: a saída CLI `bytes(N)` mede o comprimento, não o payload CBOR integral. Comprimento mais round-trip não bastam para alegar preservação integral de Bytes; a reabertura deve incluir uma projeção pública completa desse controle, sem comparar a mecânica interna Rust.

O recibo do oracle registra também que algumas rotas finas de `.with` — override e origem de named inválido pré-ligado — foram medidas pelo autor do contrato, mas ainda precisam de testemunhas próprias no oracle independente. A aplicabilidade da instrumentação da família de mutação de namespace via `.with` tampouco foi demonstrada. São pendências explícitas da nova cadeia, não cobertura implicitamente aprovada.

L0, Rust, headers e Cargo permanecem iguais ao P0. Os artefatos anteriores do P1306 também foram preservados. Não houve resselo, teste RED produtivo, stage, commit, push ou exclusão de arquivos. A proposta inicial de resselo em cópia descartável não foi executada: a parada ocorreu antes de editar L0.

## Proveniência e checks

Todos os números deste relatório referem-se ao HEAD `b303f1f15b610e09872b567027e0d806387fde8c` **com working tree P1306 não commitado**. O `git diff HEAD --stat` congelado em `p1307-baseline.json` é:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  94 +++++++++-
 00_nucleo/prompts/compiler/eval/tests.md           |  76 +++++++-
 01_core/src/compiler/eval/bindings/field_access.rs |   4 +-
 01_core/src/compiler/eval/tests.rs                 | 198 ++++++++++++++++++++-
 4 files changed, 360 insertions(+), 12 deletions(-)
```

O baseline guarda diff integral, status, inventário de fontes e fecho P1306. SHA-256 do artefato: `418443cb00f8733ae034cbf098662869835a1235f2112ef423f900cf9b56e48e`.

- Vanilla: upstream ratificado `a51e02804`; `/usr/local/bin/typst`, SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Cristalino: build fresco em RAM, `cargo build --release -p typst-wiring --bin typst`, entre `2026-09-07T16:02:26.155147+00:00` e `16:04:05.969967+00:00`, exit 0. Binário `/dev/shm/p1307-baseline-target.hqbv0ooo/release/typst`, SHA-256 `945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3`.
- Medições: timestamps por chamada, scripts, fixtures, hashes dos binários, argv e saídas integrais nos arquivos `p1307-contract-measurement.json` e `p1307-oracle-measurement.json`. Tentativas anteriores e revisões focais permanecem registradas.
- `crystalline-lint .`, às `16:08:25 UTC`: exit 0; nenhum erro, 236 warnings e 1.133 infos. Não é um relatório geral sem avisos.
- Lint estrito V3/V4/V5/V13/V14/V15/V26, às `16:08:52 UTC`: zero violations; `git diff --check`, às `16:08:44 UTC`: exit 0. Saídas integrais em `p1307-gates.json`.

Não foram repetidos os gates P8 de implementação nem a matriz histórica de 627 probes: não há candidato P1307. A revisão documental verifica a preservação dos bytes, não declara paridade dos encoders.

## Próxima decisão necessária

Reabrir o P1307 para auditar os owners de `repr` e `call_dispatch`, definir como conservar os spans e resolver a observação contextual. Depois disso, atualizar os L0 necessários e apresentar um novo gate explícito para a superfície pública e `pretty: true`. Não reduzir silenciosamente o contrato, contornar a origem descartada ou afirmar que uma simples confirmação já habilita implementar o plano atual.

Regime: **executado sem atestação de isolamento técnico**. Contrato e oracle tiveram autores distintos. O limite de agentes impediu um verificador final distinto nesta fase; o coordenador fez somente revisão documental. `p1307-pre-gate-seal.json` registra **recusa de selo**, não um selo pronto, e não existe certificado P1307.
