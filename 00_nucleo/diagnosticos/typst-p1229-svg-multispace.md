# P1229 — Linear/Radial multi-space no SVG

Veredito: `ACCEPTED_WITH_CONTRACTUAL_UNKNOWN`.

O vanilla ratificado subdivide intervalos por erro no midpoint, medido em sRGB
codificado premultiplicado, com limiar `0.001` e cap `64`. O cristalino agora
usa esse método sem reutilizar nem alterar o helper histórico do PDF.

Passaram o envelope congelado e foram promovidos: Linear/Oklab,
Linear/LinearRgb e Radial/Hsv. Linear/Radial sRGB continuam no caminho nativo
anterior, sem sampling adicional. As outras 11 combinações permanecem
explicitamente `Unknown`, com fallback `gradient-color-space` em fill e stroke.

CMYK foi a divergência dominante na tentativa inicial (erro máximo aproximado
de `0.31936`). A causa é coerente com o scope-out vigente: o vanilla converte
CMYK por perfil ICC CGATS TR 001-1995, enquanto o owner cristalino de cor ainda
declara conversão ingénua. Portanto P1229 não mascara nem promove esse caso.

Evidência final: 8/8 fixtures compilam, quatro ordens são determinísticas,
3/3 promoções novas passam o budget, 11/11 combinações restantes conservam
fallback e 18/18 mutantes são rejeitados.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`
