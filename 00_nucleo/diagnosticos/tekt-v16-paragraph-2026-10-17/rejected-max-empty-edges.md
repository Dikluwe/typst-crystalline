### Refutador de métricas locais em Text vazio — 2026-10-17

Medição anterior: empty-fonts-before-measurement.json.gz e descents-before-
measurement.json.gz no diagnóstico deste lote medem 16 casos adicionais com
estado/UTC/hashes completos. Text vazio local18pt produz altura25.1172pt no
Tekt versus29.675781pt no pin; local8pt exige22.078125pt. top-edge:0pt ou
-5pt exigem altura16pt sem tinta, mas Tekt mantém25.1172pt. Texto seguinte
com top-edge:20pt deve começar42.400003pt em vez31.517213pt. A fonte pinada
lab/typst-original/crates/typst-layout/src/inline/line.rs:559-568 acumula top e
bottom separadamente a partir de zero. O booleano contains_empty_text perde
o estilo local; esse dado não pode ser substituído pelo estilo externo do Par.

Substituir esse predicado privado por empty_text_edges, que devolve
Option<(top,bottom)> geométrico. ParagraphBody::Styled preserva referência ao
StyledContent inteiro para aplicar sua cadeia lexical na medição; os demais
predicados continuam consultando somente seu body. Source permanece transparente.
Text vazio resolve TextStyle da cadeia corrente e consulta FontMetrics.text_edges;
top/bottom são contribuições não negativas, incluindo baseline_offset do estilo.
Sequence acumula máximos de top e bottom separadamente, não o máximo das somas.
Styled empurra sua própria Styles antes de descer; Empty/Space/Par/Opaque e
Text material não fornecem strut vazio. Não consultar fontes para corpo que não
contém Text vazio. A altura virtual de Par vazio é a soma das duas contribuições,
uma única vez. O frame virtual não emite tinta nem novos callbacks.

Esta cláusula supersede somente o booleano e o uso da métrica exterior. A sonda
futura da projeção deve rejeitar a variante nova em is_paragraph, is_empty_body
e empty_text_edges. Manter os seis grupos V19 previstos, atualizando apenas o
nome/resultado do terceiro consumidor. RED adicional de fonte8/18, top-edge
zero/negativo/20, descida10 e máximos de arestas em siblings; repetir os16 casos
sem alterar os esperados anteriores. Não certificar campos do constructor de
texto ainda aceitos sem efeito nem geometria universal de wrappers opacos.
