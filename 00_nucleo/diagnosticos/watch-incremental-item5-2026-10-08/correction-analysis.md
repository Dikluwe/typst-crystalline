# Item 5 — escolha da próxima correção (8 de outubro de 2026)

## Medição antes da decisão

A comparação de produto em `measurements.json` fixou o release Tekt SHA-256
`3d6b4b21b85518c82433d48bfc64fb61dab8a1afda43708a3d3793568161201f`
contra o vanilla pinado; `README.md` documenta 36 comparações de texto/caixas
iguais, CPU/RSS e a proveniência do working tree. O contador em `counts.json`
mostra que `Source::new` não repete em main/import inalterados, mas as entradas
de avaliação repetem. Isso **não** atribui os ~54 ms à avaliação de Typst.

`correction_probe.py` mediu, separadamente do benchmark de produto, três ciclos
quentes de escrita atômica dos mesmos bytes para cada controle. O recibo
`correction-probe.json` conserva fontes completas por gerador/hash, comandos,
CPU, timestamps UTC `2026-10-09T02:51:13.105384`–`02:51:20.072301`, HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado
(`correction-git-diff-stat.txt` e `correction-git-status.txt`), hash do script e
do mesmo release. Medianas descritivas, sem intervalo de confiança:

| Controle Tekt | `eval_ms` | `total_ms` |
|---|---:|---:|
| Só `#let x = 1` | 1,33 | 1,51 |
| Texto simples | 1,52 | 50,45 |
| Texto com `#set text(size: 10pt)` | 1,54 | 50,31 |
| Texto com `#set text(font: "Libertinus Serif")` | 47,48 | 48,43 |
| Loop textual, sem import | 1,62 | 51,69 |
| Loop + import mínimo | 2,88 | 52,16 |
| Loop + 2.000 declarações no import | 4,34 | 53,16 |

O custo maior aparece com texto mesmo sem import. O estilo `font` move a
primeira demanda do catálogo para `eval_ms`, sem tornar a avaliação do módulo
a causa comprovada dos ~50 ms. Não subtrair medianas de casos como decomposição
exata. Em `03_infra/src/world/font_session.rs:23-52`, toda demanda da sessão
chama `generation::prepare`; em `03_infra/src/fontdb/generation.rs:27-29`, esse
preparo chama `Database::load_system_fonts` e descobre fontes de projeto.
O diagnóstico anterior `watch-validacao-2026-10-07.md` mediu ~49 ms de
validação pela descoberta completa em seu ambiente e estado, convergente com
a magnitude atual, mas não é uma medição da mesma chamada em cada amostra
acima. **Inferência principal:** redescoberta/validação do catálogo domina a
fixture atual. Uma medição direta do tempo de `prepare` no release atual, fora
de GDB e sem confundir exportação, pode refutar a atribuição fina.

## Escolha e ordem

1. **Evento só de metadados.** O vanilla ignora o evento; Tekt recompila.
   Corrigir separadamente a classificação do evento em L3 para não publicar
   por um toque exclusivo de mtime, preservando edição de conteúdo de mesmo
   tamanho, substituição atômica, criação/remoção, erros e recursos observados.
   Isto elimina trabalho espúrio, mas não afeta o caso de escrita dos mesmos
   bytes: o vanilla também recompila/publica nesse caso. O L0 vigente
   `00_nucleo/prompts/shell/watch.md` exige comparar fingerprint com mtime;
   atualizá-lo antes de qualquer código e testar RED→GREEN.
2. **Validação de geração de fontes.** Este é o primeiro candidato para reduzir
   a CPU de toda recompilação com texto. `FontCatalogSession` já retém o par
   book/slots; falta evitar a *redescoberta completa* quando se pode provar que
   nada relevante mudou. Não usar apenas mtime de fontes conhecidas: novas
   fontes, subdiretórios, symlinks e configuração fontconfig também alteram a
   descoberta. O fast path precisa de um manifesto completo das entradas
   observadas pela **mesma** descoberta usada por fontdb, incluindo diretórios,
   configuração e identidades dos arquivos. Se o adaptador atual não puder
   fornecer esse manifesto sem duplicar/inferir a travessia, conservar o
   preparo completo. Qualquer incerteza/erro/limite deve cair no caminho
   atual. Ao mudar a geração, reconstruir o par book/slots junto; não reusar
   índices ou resultados negativos da geração anterior.
3. **Reavaliar B/C depois disso.** A sonda dá 1,62 ms em `eval_ms` sem import e
   4,34 ms com 2.000 declarações. Ainda há trabalho de avaliação repetido,
   mas cachear Module/Frame agora ataca um resíduo menor e exige rastrear
   dependências, efeitos, warnings, rotas, identidades, fontes e contexto.
   Não escolher essa representação antes de medir o custo residual depois da
   correção de fontes. Layout/exportação permanecem eixos separados.

## Contratos e prova de aceitação

O L0 vigente `00_nucleo/prompts/infra/fontdb/generation.md` **manda repetir**
`Database::load_system_fonts` a cada demanda e proíbe watchers novos e
snapshots apenas de diretórios. O caminho rápido proposto é incompatível com
essa cláusula. Portanto não existe autorização L0 para escrevê-lo como
simples mudança local hoje. Primeiro é necessário demonstrar uma validação
causal completa, redigir a emenda L0 de `generation`, `font_session` e os
owners consumidores, com pins do núcleo correspondentes, e classificar o
gate ADR-0127. Uma assinatura pública nova ou mudança deliberada da garantia
de frescor exigiria ratificação humana antes de L1–L4. Esta análise não altera
L0 ou código.

Experimentos de refutação antes da implementação: validar em fixture de fontes
a mesma geração sem custo de descoberta, adicionar/remover/substituir fonte,
mtime restaurado, link retarget, diretório novo, fontconfig alterado, fonte
inválida tornando-se válida e falha/overflow do mecanismo de observação. Em
cada caso, comparar catálogo, seleção e render ao caminho de descoberta fresco.
Depois, medir três rodadas de `watch` com os mesmos cenários do item 5:
CPU, latência, RSS retido/startup, contagem de preparo/reuso, texto/caixas e
recuperação após erro. Se a prova de frescor ou o ganho repetido falhar, não
ligar o fast path de produção.
