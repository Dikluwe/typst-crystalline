# P1462 — oráculos independentes de `format.svg.pretty`

**Estado:** INCONCLUSIVO — `Unknown` preservado; **SEM SELO / SEM TESTE RUST**

**Papel:** autor independente dos oráculos e testador A/B pré-candidato

**Regime:** protocolo completo da skill `tekt-materializacao-segregada`

**Atestação proporcional:** executado sem atestação de isolamento

**Baseline causal:** `514b2ae8f35b661127e38620dcbdaefa9e158624`

**Tree causal:** `c12c3dac7ba85e0bf9f89b21c5e88203ff84b48e`

Este artefato registra uma execução pré-candidata. O autor não recebeu nem leu
implementação P1462 e não adaptou expected ao resultado. O checkout e o
filesystem eram compartilhados; a separação é de papel, entradas, ordem causal
e escrita permitida, não isolamento técnico. Nenhum consumer produtivo, Prompt
L0, Núcleo, passo, manifesto ou contrato foi editado.

## 1. Entradas congeladas e capacidades

Entradas efetivamente lidas:

- `AGENTS.md` e instruções do repositório;
- skill `tekt-materializacao-segregada` e as duas referências diretas;
- manifesto e contrato congelado P1462;
- passo causal, ADR-0107, ADR-0108, ADR-0127 e ADR-0129;
- os dez Prompts L0 e o Núcleo enumerados pelo manifesto;
- padrões de testes e interfaces públicas existentes no baseline.

Hashes conferidos antes da execução:

| Entrada | SHA-256 |
|---|---|
| `p1462-manifesto.json` | `de5d15c28576da4436fdaf05ee1c92416e001577ca07d4d5761f7a6bfb7885b4` |
| `p1462-contrato-congelado.md` | `96052223adf9ed4ff49332ee72ee2c4096f1a9e04cb653077f4499b42fbd8503` |
| `wiring/tests/p1462-format-svg.md` | `c37dc1f71f8e383de33abb667c7c36656e652f1f149d584d51e1b597b89a8e57` |
| skill | `66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48` |
| `papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `artefatos-e-gates.md` | `16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d` |

Escrita persistente autorizada nesta fase: somente este diagnóstico. O arquivo
planejado `04_wiring/tests/p1462_format_svg.rs` permanece ausente, porque o
contrato §9 só permite seu nascimento depois de selo discriminatório. Artefatos
temporários ficaram em `/tmp`:

| Temporário | SHA-256 |
|---|---|
| `/tmp/p1462_oracles.py` | `e05d7eb748ce530039660daad827ea231d7a6ebda9c75fa1b545b8aac8d6337c` |
| `/tmp/p1462-carrier-probe/Cargo.toml` | `79bde8043b0161bf75692f2d93a43fe3e87498121fc02946a9c07b142d039858` |
| `/tmp/p1462-carrier-probe/src/main.rs` | `876077cfc49cba78db6589a5df183fa675f22a13772faee6f870b8f291bc9c71` |
| `/tmp/p1462-oracle-run.json` | `a495589f7c6d497901410341f505ce36bf230ae59c598bb60ff1f2b1b2cc98ec` |

## 2. Estado medido e proveniência

O binário cristalino foi reconstruído antes dos probes:

```text
cargo build -p typst-wiring --bin typst
exit: 0
elapsed reportado pelo Cargo: 4.88 s
```

Identidades executadas:

| Produto | Binário | SHA-256 | Identidade auxiliar |
|---|---|---|---|
| vanilla ratificado | `/usr/local/bin/typst` | `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f` | `typst 0.15.1 (586e1bd4)` |
| baseline cristalino | `target/debug/typst` | `67a3df0937e681d4bc9957b2467079d989d14e1dda381f4301516f7a379aa47d` | `typst 0.15.1 (514b2ae8)` |

Execução principal:

```text
início: 2026-09-19T12:53:33-03:00
fim da inspeção do recibo: 2026-09-19T13:00:34-03:00
HEAD: 514b2ae8f35b661127e38620dcbdaefa9e158624
tree: c12c3dac7ba85e0bf9f89b21c5e88203ff84b48e

python3 /tmp/p1462_oracles.py \
  --vanilla /usr/local/bin/typst \
  --candidate /repos/Antigravity/typst-crystalline/target/debug/typst \
  --carrier-manifest /tmp/p1462-carrier-probe/Cargo.toml \
  --output /tmp/p1462-oracle-run.json

exit: 3
significado: Unknown acidental presente; não selável
```

No início do comando, `git diff HEAD --stat` estava vazio e `git status
--short` listava somente artefatos não rastreados:

```text
?? 00_nucleo/diagnosticos/p1462-contrato-congelado.md
?? 00_nucleo/diagnosticos/p1462-manifesto.json
?? 00_nucleo/diagnosticos/p1462-mutantes.md
```

O terceiro arquivo apareceu concorrentemente no checkout compartilhado. Não
foi lido nem usado por este papel. Os dois primeiros eram as entradas
congeladas recebidas. Assim, o estado exato não era uma working tree limpa e
não se alega isolamento.

## 3. Corpus e expected imutável

Cada compilação usou diretório temporário novo, timeout explícito, exit code,
stdout/stderr, presença do artefato e observador próprio. Os expected abaixo
vieram do contrato congelado, não da implementação candidata.

| Caso | Cobertura | Expected congelado |
|---|---|---|
| `O1-identity-profiles` | O1, M01–M02 | direto, `std` e alias observam `(function, svg, false)` em default, a11y, html e html+a11y |
| `O2-manual-construction` | O2, M03 | falha com `cannot be constructed manually` |
| `O3-set-validation` | O3, M04 | unknown named e inteiro usam os diagnósticos exatos; posicional e duplicado falham no canal normal; nunca warning ignorado |
| `O4-container-boundary` | O4, M08, M14 | root aceita; `block` e `box` falham com a mensagem exata e sem artefato |
| `O5-O6-O7-order-matrix-xml` | O5–O7, M05–M07, M10–M11, M18 | sete linhas de precedência, três ordens root, wrapper styled, XML válido, forma compacta/pretty e morfologia XML normalizada |
| `O8-target-isolation` | O8, M12 | pares `false/true` conservam semântica de PDF, pixels PNG, DOM HTML e DOM/SVG de `html.frame` |
| `O9-sibling-absence` | O9, M13 | `format.png`, `format.pdf` e `format.html` continuam ausentes no cristalino P1462 |
| `O10-carriers-fail-closed` | O10, M02, M09 | structs públicas `Default + Clone + PartialEq`, independência de clones, campo público no documento, bool canônico aceito e inteiro na chave canônica rejeitado |
| `Unknown-controls` | política de opacidade | hash divergente, binário ausente, timeout e XML inválido são `Unknown` sem crédito |

As contagens P1357 não foram incorporadas ao runner nem ao futuro teste
mantido. Elas pertencem ao diagnóstico de inventário separado. Aqui o controle
público não frágil é a ausência dos três filhos irmãos; os totais exatos do
inventário continuam obrigação externa antes de fechamento.

M15–M17 não recebem falso crédito black-box: localização da precedência,
proibição de travessia L3, ownership 1:1 e integridade do Núcleo exigem inspeção
estática e os gates V15/V26 pelo adversário/verificador. O corpus público apenas
fornece testemunhas para M01–M14 e M18.

## 4. Resultado nas três ordens

O corpus rodou como `forward`, `reverse` e `forward-repeat` em ambos os
produtos. Os vetores por id foram iguais nas três ordens, mas estabilidade não
converte `Unknown` nem contrato refutado em sucesso.

### 4.1 Baseline cristalino — RED reproduzível

| Ordem | Duração | `Violated` | `Preserved` | `Unknown` |
|---|---:|---:|---:|---:|
| forward | 104.640 s | 6 | 3 | 0 |
| reverse | 102.180 s | 6 | 3 | 0 |
| forward-repeat | 101.120 s | 6 | 3 | 0 |

Vetor estável:

- `Violated`: O1, O2, O3, O4, O5–O7 e O10;
- `Preserved`: O8, O9 e os controles deliberados de `Unknown`;
- nenhum `Unknown` acidental no baseline.

O RED é discriminatório, não mera falha agregada:

- O1 para em `module format does not contain svg` nos quatro perfis;
- O2 não chega ao diagnóstico de construção manual;
- O3 cai no fallback/warning vigente em vez da validação contratada;
- O4 não distingue root dos dois containers pelo contrato novo;
- O5–O7 não observa documento/ordem/precedência P1462;
- O10 falha ao compilar exatamente porque
  `document_format_options`, `collect_document_format_options` e
  `PagedDocument.format_options` ainda não existem.

O `Preserved` de O8 no baseline é somente um controle de neutralidade: uma
regra ainda ignorada pode produzir outputs iguais e não demonstra transporte
P1462. O3 impede que esse silêncio receba crédito de implementação.

### 4.2 Vanilla ratificado — refutador e `Unknown`

| Ordem | Duração | `Preserved` | `Violated` | `Unknown` | N/A |
|---|---:|---:|---:|---:|---:|
| forward | 23.830 s | 5 | 2 | 1 | 1 |
| reverse | 24.239 s | 5 | 2 | 1 | 1 |
| forward-repeat | 24.638 s | 5 | 2 | 1 | 1 |

O1, O2, O3 e O4 foram `Preserved` nas três ordens. O10 é `NotApplicable` ao
Rust interno do vanilla. O9 aparece como `Violated` no runner porque o vanilla
já possui os filhos dos passos futuros; isto é um controle de escopo
cristalino deliberadamente assimétrico e não um refutador.

## 5. Refutador contratual 3/3 — regra sem named

O contrato O3/O5 exige que uma set rule sem named materialize `false` e que:

```text
true → #set format.svg() → compacta
```

O vanilla ratificado observado produziu forma **pretty** nas três ordens. A
morfologia XML normalizada foi a mesma das demais linhas da fixture, mas a
decisão serializadora permaneceu `true`:

| Ordem | expected congelado | observado |
|---|---|---|
| forward | compacta / `false` | pretty / `true` |
| reverse | compacta / `false` | pretty / `true` |
| forward-repeat | compacta / `false` | pretty / `true` |

As duas inversões adjacentes não refutadas funcionaram:

- `true → false` resultou compacta;
- `false → true` resultou pretty.

Portanto o problema não é uma inversão global da ordem. O witness específico
refuta a obrigação congelada de que named omitido sobrescreve o valor anterior
com o default. Não se alterou o expected para fazer o vanilla passar. Pela
ADR-0108, a intenção agora é insuficiente: é preciso decidir se P1462 pretende
paridade com a semântica medida (omissão não escreve) ou uma mudança deliberada
de produto (omissão escreve `false`). Essa decisão invalida a cadeia a partir
do contrato se mudar a obrigação.

## 6. `Unknown` O8 preservado

O8 ficou `Unknown` no vanilla nas três ordens por timeout. Antes dele, o par
PDF `false/true` foi conclusivo e igual semanticamente em texto e metadata
pública selecionada:

```text
text: x
Tagged: yes
Pages: 1
Encrypted: no
Page size: 595.276 x 841.89 pts (A4)
Page rot: 0
PDF version: 1.7
```

O runner retornou ao primeiro timeout posterior sem persistir o rótulo do
subcaso em andamento. Consequentemente não é possível distinguir pelo recibo
se o timeout foi na compilação/observação PNG ou no primeiro controle seguinte.
Essa extração ambígua é ela própria `Unknown`; não é convertida em falha do
produto nem em sucesso. O runner terminou com exit 3 e o protocolo manda parar.

Não houve segunda execução completa. Uma futura cadeia deve primeiro executar
focalmente o par imediatamente posterior ao PDF, com rótulo persistido antes
do subprocesso e timeout proporcional, seguido apenas dos controles de
fronteira. Só depois de esse recorte deixar de ser opaco se avalia repetir o
corpus completo.

## 7. Erro independente do oráculo `styled-root`

O bloco O5–O7 também foi marcado `Violated` por uma segunda causa que não deve
ser confundida com o refutador acima. O runner comparou a morfologia do caso
`styled-root`, cujo documento contém texto `x`, com a morfologia das fixtures
de página vazia. Os hashes normalizados diferiram legitimamente:

```text
página vazia: b5347fca0d6d006332ff7ebaa6382d6af244c4c594ee3c015852c1dc19ab90bc
styled-root:  ca2fbe4c7171d78648cacaeb089db3eaff37113184a7e79f9ec3f107bfc19509
```

O próprio `styled-root` compilou, gerou XML válido e observou forma pretty como
esperado. A comparação cruzada entre conteúdos distintos era inválida. Uma
revisão futura deve comparar compact/pretty da **mesma** fonte e manter o caso
styled apenas como controle de transparência/ordem. Este erro não explica nem
apaga o witness `true → empty`, que usou a mesma página vazia e repetiu 3/3.

## 8. Poder discriminatório e mutantes

Não se calcula `mutation_score`: houve `Unknown`, o contrato foi refutado e
nenhuma campanha de mutação deste papel foi autorizada. A cobertura desenhada
permanece registrada para a reabertura:

| Mutante | Witness planejado | Estado desta fase |
|---|---|---|
| M01–M02 | O1 + default O6/O10 | discrimina baseline; não selado |
| M03 | O2 | discrimina baseline; vanilla preserved |
| M04 | O3 quatro negativos | discrimina baseline; vanilla preserved |
| M05–M06 | documento true + regra vazia | M05 testemunhado; M06 refutado pela fonte |
| M07 | duas inversões O5 | witness conclusivo |
| M08/M14 | root + block + box + ausência de artefato | witness conclusivo no vanilla |
| M09 | probe Rust de valor inteiro na chave canônica | RED público congelado; aguarda candidato |
| M10–M11 | sete linhas de precedência O6 | linhas válidas medidas; sem score |
| M12 | pares PDF/PNG/HTML/frame | PDF conclusivo; restante `Unknown` |
| M13 | ausência pública dos três irmãos | controle cristalino conclusivo |
| M15–M17 | ownership estático, V15 e V26 | fora da prova black-box; pendente verificador |
| M18 | parser XML + normalização da mesma fonte | linhas vazias conclusivas; styled exige correção focal |

Os controles deliberados de hash divergente, binário ausente, timeout e XML
inválido classificaram `Unknown` e não deram crédito de produto, como exigido.

## 9. Veredito limitado

**INCONCLUSIVO — SEM SELO / SEM TESTE / SEM IMPLEMENTAÇÃO AUTORIZADA POR ESTE
ARTEFATO.**

Razões independentes:

1. O8 contém `Unknown` repetido 3/3 por timeout e extração ambígua;
2. a regra sem named do contrato O3/O5 foi refutada pelo vanilla ratificado
   3/3;
3. o runner possui uma comparação morfológica inválida no controle
   `styled-root` que precisa de revisão focal.

O baseline RED foi reproduzido e é informativo, mas não satisfaz o gate
discriminatório. Não existe `04_wiring/tests/p1462_format_svg.rs` nesta fase.
Qualquer revisão deve reabrir o desenho a partir da primeira entrada afetada,
preservar `Unknown`, medir o recorte focal antes do corpus e nunca adaptar o
contrato a uma implementação candidata.
