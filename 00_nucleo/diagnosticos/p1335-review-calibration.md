# P1335 — revisão focal, correções de leitores e limites

Plano: `p1335-review-plan.md`, congelado antes dos dados. Todos os números abaixo
pertencem aos recibos P1335 nomeados, com baseline/manifesto/hash do checker e UTC;
não são score histórico nem mutação de produto.

O inventário atual passou pela reconstrução de união finita a partir do stdout
estrutural cru e por todos os comandos de observação. Ataques R01–R03 em cópias
foram rejeitados; reordenação preservou o resultado. O adapter cristalino pode
descartar sentinel de profundidade ao falhar lookup; a revisão registrou esse limite
e não inferiu ausência de truncamento a partir da ausência do sentinel.

A primeira revisão do preparo funcional identificou fonte compilada diferente da
fonte declarada nos dois controles closure e políticas históricas descartadas por
dedup. O operador conservou R1/focal e produziu R2; `review-supplement-focal-r2`
conferiu o recorte corrigido. O classificador depois identificou dois formatos raw
não enviados no argv; o operador produziu R3, ainda antes do full. A revisão
`review-supplement-focal-r3` confirmou raw explícito e as fronteiras json/repr/
compile, com 24 células. São ganhos em causas distintas, sem repetir corpus completo
durante o preparo. O checker próprio R3 conserva o leitor R2 anterior em arquivo.

As expectativas históricas P1334 precisaram de metadados sucessores: baseline usa
`.cases` e transporta argv por produto; seu mapa expected tem somente ID/perfil/
saídas. A revisão apontou a perda da expressão/argv, e o classificador preservou
os bytes esperados ao corrigir o vínculo. Os offsets de argv por perfil também
foram corrigidos por ele em sucessor metadata-only. Isso não é mudança do oráculo.

Na primeira revisão transversal (`review-transversal-focal-r0.json`) houve 13
violações: oito pins PDF de R0 já não correspondiam aos arquivos, e cinco eram
erros do próprio leitor. A repetição R1 do operador havia reutilizado o destino
focal e substituído oito PDFs; os JSONs e canais de R0 continuaram disponíveis,
mas sua evidência gráfica original não foi recuperada. O método foi reaberto e
R3 usa diretórios exclusivos recusando reuso, com recibo do incidente do operador.

Os cinco falsos alarmes do leitor foram quatro contagens de headline usando HTML
maiúsculo em vez de html e uma imposição do contrato histórico cristalino ao
vanilla. O leitor sucessor deriva a headline dos bytes congelados e julga o
contrato cristalino nessa projeção; conserva a divergência bilateral de ordem
error/warning. `review-transversal-focal-r3.json` passou com os pins atuais íntegros.
Os seis controles de query aceitam somente a rejeição pública exata e mantêm
truncamento, ausência, erro genérico, crash e outro comando em Unknown. Esses
controles não aumentam os 14 ataques planejados.

Uma sugestão de controle genérico math com nome single-letter `f` foi refutada:
a gramática o trata como MathText, conforme a própria fonte math.rs:385–392.
A sugestão foi retirada; nome multigrapheme `custom` testa a hipótese correta de
MathIdent. Nenhuma conclusão sobre owner/prioridade pode usar a sugestão refutada.

Estado de calibração: R01–R12 pertinentes já têm resultados atuais nos recibos
de catálogo/runtime/principal parcial; R13/R14 aguardam seleção genuína. Não houve
duas revisões sem ganho na mesma causa. O veredito final exige as três ordens,
classificação e seleção finais, preservação e relato explícito da perda gráfica R0.
