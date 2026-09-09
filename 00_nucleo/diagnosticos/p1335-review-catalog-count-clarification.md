# União estrutural versus catálogo completo

Uma mensagem preliminar do revisor confundiu a união estrutural de 4708 paths
com o número de paths únicos do catálogo. A reconstrução direta do catálogo
mostrou 4718 IDs e também 4718 paths distintos, sem duplicatas. Os dez paths
fora da união estrutural são controles históricos: hsl, hsv, linear_rgb,
pdf.data-cell, pdf.header-cell, pdf.table-summary, std, csv.encode, xml.encode
e read.encode. A mensagem foi retificada antes do veredito final.

O denominador 4718 × 4 = 18872 nunca mudou. O sucessor do aggregate que apenas
rotula o bloco como cobertura de probes é igualmente válido e não requer
reexecução. O limite essencial é não chamar a união estrutural de catálogo
completo, nem interpretar esses probes de lookup como funções completas.
