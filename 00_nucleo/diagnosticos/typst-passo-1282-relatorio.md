# Relatório do Passo 1282 — inventário bilateral da superfície pública

## Veredito

**Inventário P1282 materializado.** Foram produzidos catálogos novos e bilaterais
para os perfis `default` e `html`, sem reutilizar contagens históricas como resultado.
As contagens abaixo são contagens de paths públicos inventariados; não são percentagem
de paridade da linguagem nem autorização para implementar lacunas.

Não houve alteração de binding, contrato público, comportamento padrão ou fase do
pipeline. As únicas mudanças executáveis pertencem ao harness em `lab/`.

## Regime e entradas congeladas

Foi usado o protocolo completo `tekt-materializacao-segregada`:

- contrato: agente `contrato_p1282`;
- adversário: agente `ataques_p1282`;
- implementação do harness: agente raiz;
- testes A/B: agente `testes_p1282`;
- verificação final: agente independente, após congelamento dos artefatos.

A segregação é procedural; o filesystem compartilhado impede atestar isolamento
técnico de leitura.

- Início: `2026-08-30T08:35:59-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Vanilla normativo: upstream/main `a51e02804`.
- SHA-256 `/usr/local/bin/typst`:
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- SHA-256 `target/release/typst`:
  `0a4d71415b4e3ba47ed9c50f08b612a61fd14e1e538790ea4e51245d3019ce3f`.
- SHA-256 do enumerador vanilla:
  `5bf12daca44288341fa0f121362872c5444c805a76d8498cea4ec60ea0bb7ff7`.
- SHA-256 do enumerador cristalino:
  `c17769724cd6ef9484833d0a531b6ec4cc87d0f211478863614de70c7453f70e`.

A working tree inicial já continha o P1281 e os passos P1281–P1287 não rastreados do
utilizador. Nenhum outro passo de materialização nem `00_nucleo/context/` foi lido.

## Contrato e RED

O contrato `C-P1282-v1` congelou:

- perfis simétricos `default` (sem feature) e `html` (`--features html` nos dois lados);
- owner, slot e access form explícitos, sem inferir membro pela presença de ponto;
- precedência de classificação presença → kind/access form → metadados;
- falha bilateral nunca equivalente a `MATCH`;
- `params: null` e símbolo sem identidade/variantes nunca equivalentes a metadados iguais;
- preservação de `Unknown` e de descendentes bloqueados pelo owner;
- adjudicação atual das 45 extensões históricas sem crédito ou remoção automática.

O adversário rejeitou as 18 mutações obrigatórias: mutation score **18/18 = 1,0**.

Ledger canônico do gate mutacional (janela adversarial
`2026-08-30T08:42:19-03:00`–`08:43:54-03:00`, nos binários congelados acima):

| # | Mutação atacada | Testemunha discriminatória | Resultado |
|---:|---|---|---|
| 1 | ponto no path implica membro | fixture `foo.bar` sem owner explícito | rejeitada: fica `Unknown`, não `MISSING_MEMBER` |
| 2 | `Type.method` implica instância | `array.all` no namespace e chamada com receiver | rejeitada: formas de acesso permanecem separadas |
| 3 | função implica método | `array.range(3)` e `(1,2).range()` | rejeitada: chamada estática e receiver não se fundem |
| 4 | feature desligada implica lacuna | `repr(type(html))` no perfil default | rejeitada: `disabled_by_profile` bilateral |
| 5 | HTML unilateral | comandos dos dois lados no perfil HTML | rejeitada: feature set deve ser idêntico |
| 6 | falha bilateral implica `MATCH` | binding inexistente nos dois binários | rejeitada: sucesso bilateral é precondição |
| 7 | `params: null` implica igualdade | fixture bilateral com metadado nulo | rejeitada: `UNVERIFIED_METADATA` |
| 8 | scopes podem ser omitidos | limite/profundidade do enumerador | rejeitada: truncamento explícito vira `Unknown` |
| 9 | aliases/variantes podem ser omitidos | `sym.arrow.r` | rejeitada: identidade, aliases e variantes são obrigatórios |
| 10 | ausência do owner propaga lacunas | fixture `owner.child` sem owner | rejeitada: descendente vai a `blocked_by_ancestor` |
| 11 | kinds diferentes podem ser normalizados | `kind_key` com owner/slot/access form | rejeitada: kind literal é preservado |
| 12 | SHA ou features divergentes são aceitáveis | proveniência congelada por lado e perfil | rejeitada: entrada divergente fica `Unknown` |
| 13 | timestamp volátil é aceitável | duas execuções equivalentes por bytes | rejeitada: payload canônico não contém tempo volátil |
| 14 | contagem permite percentagem de paridade | tabelas de paths do inventário | rejeitada: somente contagens, sem alegação linguística |
| 15 | as 45 extras recebem crédito automático | adjudicação bilateral das 45 sementes | rejeitada: cada seed recebe estado atual explícito |
| 16 | `Unknown` pode ser ignorado | conjunto e precedência de `Unknown` | rejeitada: fechamento exige conjunto vazio |
| 17 | kind `symbol` basta para `MATCH` | identidade e variantes de `sym.arrow` | rejeitada: permanece `UNVERIFIED_METADATA` |
| 18 | resultado pode ser transportado entre perfis | `html` no default e no perfil HTML | rejeitada: perfil integra a identidade da medição |

O adversário executou fixtures de merge e sondas runtime próprias; não leu nem avaliou
o patch candidato. Este ledger registra o recibo do papel adversarial, enquanto os testes
do harness abaixo permanecem uma evidência independente e não uma substituição do gate.

Após o primeiro passe do verificador, dois recibos narrativos foram recusados e
materializados no fluxo executável: ambos os enumeradores agora emitem uma entrada
`availability: unknown` se o limite de profundidade for atingido; cada catálogo bruto
carrega side, perfil, feature set, SHA-256 do binário de produto e revisão vanilla; o
entrypoint do merge valida esses pins e produz `UNKNOWN/invalid_provenance` diante de
qualquer divergência. Três testes de proveniência confirmaram RED antes da correção e
GREEN depois dela. Para a mutação de truncamento, dois testes Rust — um por enumerador —
foram executados com a chamada a `record_truncation` removida: ambos falharam com exit
`101`; restaurada a chamada, ambos passaram. Assim o teste mata especificamente o retorno
silencioso, em vez de apenas injetar um sentinel no classificador. A segunda medição
encerrou em `2026-08-30T09:18:36-03:00`.

O RED do harness histórico foi confirmado:

- enumerador vanilla usava `Features::all()`, misturando HTML, Bundle e A11y;
- enumerador cristalino percorria o scope bruto e incluía HTML mesmo no default;
- `merge.py` inferia membro pelo ponto e promovia `params: null` a `MATCH`;
- `run_probes.py` ativava HTML só no vanilla e tratava falha bilateral como igualdade;
- o payload histórico incluía timestamp volátil e não era byte-determinístico.

## Medição bilateral

| Perfil | Vanilla bruto | Cristalino bruto | Features em ambos |
|---|---:|---:|---|
| `default` | 2.015 | 1.086 | nenhuma |
| `html` | 2.130 | 1.150 | `html` |

Classificação canônica:

| Classe | `default` | `html` |
|---|---:|---:|
| `MATCH` | 637 | 638 |
| `MISSING_BINDING` | 0 | 0 |
| `MISSING_MEMBER` | 879 | 930 |
| `WRONG_KIND` | 0 | 0 |
| `UNVERIFIED_METADATA` | 404 | 467 |
| `EXTRA_BINDING` | 45 | 45 |
| `UNKNOWN` | 0 | 0 |
| descendentes `blocked_by_ancestor` | 95 | 95 |

Os 95 descendentes bloqueados ficam num ledger separado: a ausência do owner já é a
lacuna observável e não é multiplicada pelo fan-out dos filhos.

O ledger de features registra `html` como `disabled_by_profile` bilateral no perfil
default e `active_bilateral` no perfil HTML. Há 64 paths HTML ativos bilateralmente
somente quando a feature está ligada. Os demais membros HTML que só o vanilla expõe no
perfil HTML são lacunas reais desse perfil, não assimetria de flags.

## Probes e exemplos RED

Foram executadas 118 sondas no default e 120 no HTML. A amostra é determinística por
perfil e classe (`min(N, max(10, ceil(sqrt(N))))`), complementada por canários focais.

- `(true, false).all(x => x)`: sucesso bilateral e valor `false`; confirma método de
  instância.
- `array.range(3)`: vanilla produz `[0,1,2]`; cristalino falha; confirma que presença do
  path `array.range` não prova a mesma chamada estática.
- `repr(type(html))`: falha bilateral no default e retorna `"module"` bilateralmente no
  perfil HTML.
- `math.YY`, `sym.KK` e `emoji.books`: resolvem no vanilla e falham no cristalino.
- `sym.arrow`: resolve bilateralmente, mas identidade/variantes diferem; permanece
  `UNVERIFIED_METADATA`, não `MATCH` por kind.

Falhas do binário, feature desativada e diferença de produto ficam distintas nos recibos
por exit code, stdout, stderr e comando completo.

## Adjudicação das 45 extensões históricas

As 45 entradas foram usadas somente como sementes de sonda atual:

- 44 continuam `EXTRA_BINDING` nos dois perfis;
- `context` ficou `BILATERALLY_ABSENT` pela forma de acesso runtime usada;
- nenhuma recebeu crédito de paridade;
- nenhuma foi removida ou alterada no produto.

Essa medição não decide se cada extensão é compatível; fornece a fila atual para uma
adjudicação posterior.

## Entrega a P1283 — `math`, `sym` e `emoji`

O catálogo HTML, que é o superset de features medido, separa:

| Família | módulos | funções | símbolos | outros | entradas com variantes |
|---|---:|---:|---:|---:|---:|
| `math` | 3 | 47 | 298 | 49 | 298 |
| `sym` | 3 | 0 | 298 | 0 | 298 |
| `emoji` | 1 | 0 | 772 | 0 | 772 |

Aliases são agrupados por identidade completa do símbolo, separadamente por lado;
variantes preservam modifiers e valores de cada lado. As listas completas vivem em
`p1282-summary.json`, sob `p1283_families`.

## Reprodutibilidade

Comandos principais:

```sh
lab/typst-original/target/release/p1140-inventory \
  /tmp/p1282-vanilla-default.json default /usr/local/bin/typst
lab/typst-original/target/release/p1140-inventory \
  /tmp/p1282-vanilla-html.json html /usr/local/bin/typst

lab/surface-inventory/target/release/surface-inventory-p1140 \
  /tmp/p1282-crystalline-default.json \
  /tmp/p1282-vanilla-default.json default \
  lab/surface-inventory/extra_seeds.json target/release/typst
lab/surface-inventory/target/release/surface-inventory-p1140 \
  /tmp/p1282-crystalline-html.json \
  /tmp/p1282-vanilla-html.json html \
  lab/surface-inventory/extra_seeds.json target/release/typst
```

```sh
python3 lab/surface-inventory/merge.py \
  /tmp/p1282-vanilla-default.json \
  /tmp/p1282-crystalline-default.json \
  /tmp/p1282-inventory-default.json --profile default

python3 lab/surface-inventory/run_probes.py \
  --profile default \
  --vanilla-bin /usr/local/bin/typst \
  --crystalline-bin target/release/typst \
  --inventory /tmp/p1282-inventory-default.json \
  --output /tmp/p1282-probes-default.json
```

As quatro enumerações e os dois inventários fundidos foram repetidos e comparados por
bytes. Todos foram idênticos:

| Artefato | SHA-256 |
|---|---|
| vanilla default | `2f70d832f2b24f8c8eae8ffb15dd1e2770c667b9bfb24fcadefdcad43b4cfb27` |
| cristalino default | `f0d9ebc401b5310bd646ae8c01b7ac184adbdb29d9a787a76b93e2be68883a55` |
| vanilla HTML | `2ef941beab9ef3e2859de83c8bcfadc7a4482af417db23029b1bec0997f22c61` |
| cristalino HTML | `e5d307f86af1a58515bad450c51fbe218e94db32ccbd095b5b49287ebd87e844` |
| inventário default | `17923ca1aedb5dfb94e9e64d5eeae481ae33e3bb3c2917ceda4a00ea20a349c2` |
| inventário HTML | `3e389363f019c9c00f21fe47f23342a624b27c76d9c59002b870c5e1195659ed` |
| probes default | `ff66dfe25da4a4780171f5e05b5d640a88d96c3360079b5006d727864a46d728` |
| probes HTML | `de33ef5b1c724d022e569471b48caebe5a65752933d19490151a748121615f44` |
| resumo/entrega P1283 | `0e606416af0d039caba5d89980dfe737ac7ea077dd03e1eb21e115dec259f714` |

Os hashes dos probes e do resumo acima correspondem à execução anterior ao
versionamento e são reconfirmados nos gates finais.

## Verificação final segregada

Às `2026-08-30T09:29:55-03:00`, o verificador independente aprovou o candidato
recongelado. O mutation score contratual ficou sustentado em **18/18 = 1,0**.
Os gates finais foram:

- 22/22 testes Python do classificador e probes;
- 1/1 teste Rust de truncamento no enumerador vanilla e 1/1 no cristalino;
- ataque explícito sem `record_truncation`: RED bilateral com exit `101`; restauração:
  GREEN bilateral;
- `cargo test --workspace --quiet`: zero falhas;
- `cargo fmt --check` nos dois manifests do harness;
- `cargo build --workspace --bin typst`;
- `git diff --check` e `git diff --cached --check`;
- `crystalline-lint --checks v5,v15,v26 --fail-on warning .`: zero violações;
- `crystalline-lint .`: exit `0`, preservando apenas avisos preexistentes.

O veredito atesta segregação procedural, não isolamento técnico de leitura, pois os
papéis compartilham o mesmo filesystem.

## Artefatos perenes

- `p1282-vanilla-default.json`
- `p1282-crystalline-default.json`
- `p1282-vanilla-html.json`
- `p1282-crystalline-html.json`
- `p1282-inventory-default.json`
- `p1282-inventory-html.json`
- `p1282-probes-default.json`
- `p1282-probes-html.json`
- `p1282-summary.json`

Todos estão em `00_nucleo/diagnosticos/`.
