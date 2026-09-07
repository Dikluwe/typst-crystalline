# P1307-R6 — oracle de aceitação independente

**Congelado antes do candidato; não é selo nem certificado.** Regime: executado sem atestação de isolamento técnico. Autor `/root/p1307_oracle`, somente expectativas e medições; não leu candidato, não escreveu Rust, L0, testes produtivos, manifesto raiz ou veredito. A skill `tekt-materializacao-segregada` manteve expectativas originadas no vanilla/baseline e separou dívida histórica da obrigação aprovada.

Baseline R6 aprovado pelo humano (`autorizo`), SHA-256 `8cc0eae00457d2e7d54b420024eae49344032b34b295b4eda536faeb1ec3c4a3`. HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não commitado; `provenance` da medição guarda cada horário e `git diff HEAD --stat`. Vanilla ratificado upstream `a51e02804`, binário SHA `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`; baseline RAM SHA `945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3`. Não existe observação candidata neste oracle.

## Composição e política

515 casos, 1.982 células de expectativa:

- 455 casos R4: todos preservados. **Única supersessão:** `r2.construct-LocatedContent` passa a exigir repr realizado vanilla aprovado em R5, usando a fonte predecessor exata SHA `78fd17fa61567bfd8107820e9daa9de588d9fcd08c993c24261fd2c6a3e304a9`. As antigas observações permanecem em `supersession.prior_observations`.
- 11 casos math R4 intactos, com suas políticas explícitas.
- 17 casos R5 repr/CBOR. CBOR mantém **Text(repr)**, não vira o mapa vanilla. Os bytes futuros integrais vêm dos controles bilaterais de String igual ao repr vanilla medido, com origem própria por célula.
- 14 casos R5 Content selecionados: JSON de query padrão/numbering "1"/"I"/none/en/pt; seis pares de igualdade; dois erros de acesso exatos.
- 18 witnesses novos R6: JSON completo de fields, at/has/func/location, label presente/ausente, field access, métodos estáticos com aliases, escape array/closure antes/depois de mudança de numbering/idioma, duas ocorrências de body idêntico com estilos suportados distintos, igualdade entre idiomas e context colocado antes do Heading.

`r5_content_selection` classifica individualmente os 64 casos predecessores. Não impõe `repr(Dict)` multiline vanilla: a dívida desse formatter é distinta dos campos. Os witnesses novos usam `json.encode(x.fields())` ou valores escalares/tuplas, sem normalizar a string antiga. Casos originais que combinavam escape com supplement/offset não modelados continuam evidência histórica; os novos witnesses usam apenas numbering e idioma suportados. Callbacks, supplement custom/none, set level/depth/offset/outlined/bookmarked/hanging-indent e outras dívidas do produtor não foram promovidos a obrigação nem contados como paridade.

Perfis herdados permanecem os efetivamente medidos. Casos R5 default-only não recebem expectativas inventadas para outros perfis; o replay itera apenas `observations` presentes. Os 18 casos novos foram medidos nos quatro perfis de features, sempre target PDF, sem alegação de target HTML.

## Medição focal e resultados

A primeira tentativa focal passou: seis witnesses, 12 execuções bilaterais, zero Unknown e zero falhas de pré-condição vanilla. Só depois foram executados os demais witnesses e perfis. `bounded.rows` reúne **144 execuções únicas** (18 × quatro perfis × dois binários), incluindo as 12 focais reutilizadas; `extension_focal` é registro intermediário dos mesmos resultados, não custo adicional. Zero Unknown e nenhuma divergência entre perfis no recorte. Horários e durações integrais constam das rows e de `bounded.finished`.

Não houve repetição da matriz global herdada nem leitura de candidato. Os predecessores preservam suas medições, ordens e tentativas. Os dois Unknown de nested context R5 permanecem históricos; não foram substituídos por um falso resultado bilateral.

Achados discriminantes: valor capturado sob numbering "1"/en continua "1"/Section após mudar o bloco para "I"/pt; as duas ocorrências de mesmo corpo preservam "1"/Section e "I"/Seção separadamente; igualdade entre esses idiomas é false no vanilla e true no baseline. O controle estático independente do encoder mede `(10, <probe>, true, "function")` no vanilla versus `(2, "absent", false, "function")` no baseline, demonstrando que a rota estática/alias existe e a lacuna é observável sem depender da ausência de `json.encode`.

## API, origem e limites

`p1307-r6-oracle.py::classify(expected, observed)` delega o comparador R4 congelado: envelope integral, diagnóstico completo, incompletude → Unknown; sem sorting, truncamento ou normalização de payload. `cases[].observations[profile].future_expected` contém a obrigação. Cada célula possui `expected_measurement_ref` com artifact, collection(s), case_key/case_id/source_case_id, profile e side. Controles CBOR apontam para a fonte literal de origem, não para o documento contextual candidato. Os `measurement_ref` predecessores permanecem disponíveis.

Replay público executável após o selo:

```sh
python3 00_nucleo/diagnosticos/p1307-r6-oracle.py replay --binary /caminho/do/binario
```

Aceita `--case REGEX`, `--profile` e `--reverse`; emite recibo JSON em stdout e não altera os artefatos congelados. Todos os inputs são conferidos por SHA. O runner não declara sucesso geral: informa Preserved/Violated/Unknown por célula.

Os ataques propostos no JSON incluem perda/ordem de campos, label sintético, reconstrução pelo estilo de consumo, aliasing entre corpos iguais, perda em array/closure/métodos, primeiro context, igualdade e troca indevida do fallback CBOR. **Nenhum mutante Rust foi executado, mutation_score é null, nenhum selo foi autoatribuído.** Calibração e veredito pertencem ao verificador distinto `/root/p1306_oracle`.

## Pins finais

| Arquivo em `00_nucleo/diagnosticos/` | SHA-256 |
|---|---|
| `p1307-r6-oracle.py` | `62c4decfb6f07088ff2fb2aa1513ef1f62e2681edbb1431b42cd781eac56ef1a` |
| `p1307-r6-oracle.json` | `99d2a67984a468c8e86e20010ecc1bd76a364f1d1adebbb5b873fc341f7790a6` |
| `p1307-r6-oracle-measurement.json` | `ee6e8cd834d26b857ffbf4311af0422ce20008338c01a98545452d641c9686cb` |

Os inputs protegidos e seus hashes estão no oracle. Nenhum artefato predecessor P1305/P1306/P1307 foi alterado. O hash desta nota será registrado externamente no manifesto, evitando referência circular.
